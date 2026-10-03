//! The event loop: scenario setup, event dispatch, journaling and rescheduling.
//! Implements Technical Design §16.1-§16.3 (authoritative time, event ordering, scheduler
//! outline) and Simulation §3.1-§3.3 (event-driven clock, one pending event per actor).

use crate::column::{Column, ColumnId, ColumnInit};
use crate::content::Content;
use crate::error::SimError;
use crate::events::{EndReason, SCHEMA_VERSION, SimEvent, phase};
use crate::kinematics::plan_legs;
use crate::scenario::Scenario;
use crate::schedule::MarchSchedule;
use ach_core::{FileHeader, IdAllocator, JournalWriter, MAGIC_JOURNAL, Seed, SimTime, WorldPos};
use ach_logistics::StockSet;
use ach_people::Roster;
use ach_sched::Scheduler;
use ach_world::Terrain;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

/// The serializable part of a running simulation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimState {
    /// Id of the scenario being run.
    pub scenario_id: String,
    /// Campaign seed.
    pub seed: Seed,
    /// Identifier allocator (characters first, then columns).
    pub ids: IdAllocator,
    /// Known characters.
    pub roster: Roster,
    /// The columns, by id.
    pub columns: BTreeMap<ColumnId, Column>,
    /// Pending events and the clock.
    pub scheduler: Scheduler<SimEvent>,
    /// Sequence number the journal's next entry will get.
    pub journal_next_seq: u64,
    /// Journal fingerprint after the last entry.
    pub journal_fingerprint: u64,
    /// Time limit.
    pub end: SimTime,
}

/// A running scenario: serializable state plus content reloaded from the scenario's paths.
pub struct Sim {
    state: SimState,
    terrain: Arc<Terrain>,
    content: Arc<Content>,
    #[cfg(any(test, feature = "test-hooks"))]
    checkpoints: std::collections::BTreeSet<SimTime>,
}

/// What a call to [`Sim::run_until`] achieved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunSummary {
    /// The clock when the run stopped.
    pub end_time: SimTime,
    /// Journal entries written by this call.
    pub events: u64,
    /// Columns that have arrived, in column id order, with their arrival times.
    pub arrived: Vec<(ColumnId, SimTime)>,
    /// Journal fingerprint after the last entry.
    pub fingerprint: u64,
}

/// The first midnight at or after `t`, as a day number.
fn first_midnight_day(t: SimTime) -> i64 {
    if t.time_of_day().0 == 0 {
        t.day()
    } else {
        t.day() + 1
    }
}

/// The last millisecond of `day`.
fn last_ms_of_day(day: i64) -> SimTime {
    SimTime(SimTime::at(day + 1, 0, 0).0 - 1)
}

impl Sim {
    /// Loads the scenario's content under `repo_root`, plans every column's legs (failing
    /// fast on impassable ground) and queues the opening events.
    pub fn new(scenario: &Scenario, repo_root: &Path) -> Result<Self, SimError> {
        let start: SimTime = scenario.start.parse()?;
        let end: SimTime = scenario.end.parse()?;
        if end <= start {
            return Err(SimError::Invalid("end must be after start".into()));
        }
        if scenario.columns.is_empty() {
            return Err(SimError::Invalid("a scenario needs a column".into()));
        }
        let terrain = Terrain::load(&repo_root.join(&scenario.terrain_dir))?;
        let content = Content::load(scenario, repo_root)?;
        let mut ids = IdAllocator::new(1);
        let roster = Roster::load(&repo_root.join(&scenario.roster), &mut ids)?;
        let mut columns = BTreeMap::new();
        for spec in &scenario.columns {
            let character = |key: &str| {
                roster
                    .by_key(key)
                    .map(|c| c.id)
                    .ok_or_else(|| SimError::Unknown {
                        what: "character",
                        key: key.to_owned(),
                    })
            };
            let column = Column::new(ColumnInit {
                id: ids.try_alloc()?,
                leader: character(&spec.leader)?,
                reporters: spec
                    .reporters
                    .iter()
                    .map(|k| character(k))
                    .collect::<Result<_, _>>()?,
                legs: plan_legs(spec, &terrain, &content)?,
                schedule: MarchSchedule::new(&spec.schedule)?,
                spec,
                start,
            })?;
            columns.insert(column.id, column);
        }
        let state = SimState {
            scenario_id: scenario.id.clone(),
            seed: Seed(scenario.seed),
            ids,
            roster,
            columns,
            scheduler: Scheduler::new(start),
            journal_next_seq: 0,
            journal_fingerprint: 0,
            end,
        };
        let mut sim = Self {
            state,
            terrain: Arc::new(terrain),
            content: Arc::new(content),
            #[cfg(any(test, feature = "test-hooks"))]
            checkpoints: std::collections::BTreeSet::new(),
        };
        sim.queue_opening_events(start)?;
        Ok(sim)
    }

    fn queue_opening_events(&mut self, start: SimTime) -> Result<(), SimError> {
        let end = self.state.end;
        let scenario = self.state.scenario_id.clone();
        let scheduler = &mut self.state.scheduler;
        scheduler.schedule(start, phase::DAY, SimEvent::ScenarioStarted { scenario })?;
        let day = first_midnight_day(start);
        if SimTime::at(day, 0, 0) <= end {
            scheduler.schedule(
                SimTime::at(day, 0, 0),
                phase::DAY,
                SimEvent::DayBegan { day },
            )?;
        }
        for column in self.state.columns.values() {
            let departed = SimEvent::ColumnDeparted {
                column: column.id,
                site: column.start_site.clone(),
                pos: column.legs[0].start(),
            };
            scheduler.schedule(start, phase::SCHEDULE, departed)?;
        }
        let reason = EndReason::TimeLimit;
        scheduler.schedule(end, phase::END, SimEvent::ScenarioEnded { reason })?;
        let ids: Vec<ColumnId> = self.state.columns.keys().copied().collect();
        for id in ids {
            self.schedule_day_end(id, start.day())?;
        }
        Ok(())
    }

    /// The current simulation time.
    pub fn now(&self) -> SimTime {
        self.state.scheduler.now()
    }

    /// The serializable state.
    pub fn state(&self) -> &SimState {
        &self.state
    }

    /// The terrain the columns march over.
    pub fn terrain(&self) -> &Terrain {
        &self.terrain
    }

    /// The loaded content.
    pub fn content(&self) -> &Content {
        &self.content
    }

    /// The journal header for this run: magic `ACHJ`, [`SCHEMA_VERSION`], seed, scenario id.
    pub fn journal_header(&self) -> FileHeader {
        FileHeader::new(
            MAGIC_JOURNAL,
            SCHEMA_VERSION,
            self.state.seed,
            self.state.scenario_id.clone(),
        )
    }

    /// Where a column will be at `t`, assuming no event intervenes before then.
    /// Valid for `t` at or after the column's last settled time; `None` for unknown columns.
    pub fn column_position_at(&self, column: ColumnId, t: SimTime) -> Option<WorldPos> {
        self.state.columns.get(&column).map(|c| c.position_at(t))
    }

    /// Processes the next event: settles every affected column's accounting up to the event
    /// time, applies the event, journals it, and schedules its consequences.
    ///
    /// Returns the event's time, or `None` when the queue is empty.
    pub fn step<W: Write>(
        &mut self,
        journal: &mut JournalWriter<W>,
    ) -> Result<Option<SimTime>, SimError> {
        self.step_until(SimTime(i64::MAX), journal)
    }

    /// Steps until the next event is later than `limit` or the scenario has ended.
    pub fn run_until<W: Write>(
        &mut self,
        limit: SimTime,
        journal: &mut JournalWriter<W>,
    ) -> Result<RunSummary, SimError> {
        let first_seq = journal.next_seq();
        while self.step_until(limit, journal)?.is_some() {}
        Ok(RunSummary {
            end_time: self.now(),
            events: journal.next_seq() - first_seq,
            arrived: self
                .state
                .columns
                .values()
                .filter_map(|c| c.arrived_at.map(|t| (c.id, t)))
                .collect(),
            fingerprint: journal.fingerprint(),
        })
    }

    fn step_until<W: Write>(
        &mut self,
        limit: SimTime,
        journal: &mut JournalWriter<W>,
    ) -> Result<Option<SimTime>, SimError> {
        #[cfg(any(test, feature = "test-hooks"))]
        if let Some(t) = self.take_due_checkpoint(limit) {
            self.advance_all(t);
            return Ok(Some(t));
        }
        let Some((key, event)) = self.state.scheduler.pop_until(limit) else {
            return Ok(None);
        };
        for out in self.handle(event, key.time)? {
            journal.append(key.time, &out)?;
        }
        self.state.journal_next_seq = journal.next_seq();
        self.state.journal_fingerprint = journal.fingerprint();
        Ok(Some(key.time))
    }

    fn handle(&mut self, event: SimEvent, t: SimTime) -> Result<Vec<SimEvent>, SimError> {
        match event {
            SimEvent::DayBegan { day } => {
                let next = SimTime::at(day + 1, 0, 0);
                if next <= self.state.end {
                    self.state.scheduler.schedule(
                        next,
                        phase::DAY,
                        SimEvent::DayBegan { day: day + 1 },
                    )?;
                }
                Ok(vec![event])
            }
            SimEvent::ScenarioEnded { .. } => {
                self.advance_all(t);
                self.cancel_all_pending();
                let reason = if self.all_arrived() {
                    EndReason::AllArrived
                } else {
                    EndReason::TimeLimit
                };
                Ok(vec![SimEvent::ScenarioEnded { reason }])
            }
            _ => match event.column() {
                Some(id) => self.handle_column_event(id, event, t),
                None => Ok(vec![event]),
            },
        }
    }

    fn handle_column_event(
        &mut self,
        id: ColumnId,
        event: SimEvent,
        t: SimTime,
    ) -> Result<Vec<SimEvent>, SimError> {
        let is_day_end = matches!(event, SimEvent::ColumnDayEnded { .. });
        let table = &self.content.consumption;
        let column = self
            .state
            .columns
            .get_mut(&id)
            .expect("invariant: queued events name existing columns");
        column.advance_to(t, table);
        let out = column.apply(event, t);
        if is_day_end {
            column.day_end = None;
            if !column.arrived() {
                self.schedule_day_end(id, t.day() + 1)?;
            }
            return Ok(out);
        }
        column.pending = None;
        if column.arrived()
            && let Some(handle) = column.day_end.take()
        {
            self.state.scheduler.cancel(handle);
        }
        self.schedule_progress(id)?;
        if self.all_arrived() && column_just_arrived(&out) {
            let reason = EndReason::AllArrived;
            self.state
                .scheduler
                .schedule(t, phase::END, SimEvent::ScenarioEnded { reason })?;
        }
        Ok(out)
    }

    /// Queues the column's next progress event, if any falls before the time limit.
    fn schedule_progress(&mut self, id: ColumnId) -> Result<(), SimError> {
        let SimState {
            columns,
            scheduler,
            end,
            ..
        } = &mut self.state;
        let column = columns
            .get_mut(&id)
            .expect("invariant: scheduling names existing columns");
        column.pending = column
            .plan_next(&self.content.consumption, *end)
            .map(|p| scheduler.schedule(p.time, p.phase, p.event))
            .transpose()?;
        Ok(())
    }

    /// Queues the end of `day` for a column. The queued event is a wake-up: its journaled
    /// form is built from the settled state when it fires.
    fn schedule_day_end(&mut self, id: ColumnId, day: i64) -> Result<(), SimError> {
        let SimState {
            columns,
            scheduler,
            end,
            ..
        } = &mut self.state;
        let at = last_ms_of_day(day);
        if at > *end {
            return Ok(());
        }
        let wake = SimEvent::ColumnDayEnded {
            column: id,
            day,
            distance_today_m: 0,
            stocks: StockSet::default(),
            pos: WorldPos::surface_m(0, 0),
        };
        let handle = scheduler.schedule(at, phase::STOCK, wake)?;
        if let Some(column) = columns.get_mut(&id) {
            column.day_end = Some(handle);
        }
        Ok(())
    }

    fn all_arrived(&self) -> bool {
        self.state.columns.values().all(Column::arrived)
    }

    fn advance_all(&mut self, t: SimTime) {
        let table = &self.content.consumption;
        for column in self.state.columns.values_mut() {
            column.advance_to(t, table);
        }
    }

    /// Drops every queued event; the clock stays where it is.
    fn cancel_all_pending(&mut self) {
        let scheduler = &mut self.state.scheduler;
        let keys: Vec<_> = scheduler.iter().map(|(key, _)| *key).collect();
        for key in keys {
            scheduler.cancel(key);
        }
    }
}

/// Whether the events just applied include an arrival.
fn column_just_arrived(events: &[SimEvent]) -> bool {
    events
        .iter()
        .any(|e| matches!(e, SimEvent::ColumnArrived { .. }))
}

#[cfg(any(test, feature = "test-hooks"))]
impl Sim {
    /// Test hook: settles every column's accounting at `t` without journaling anything.
    /// Accounting must not depend on how often it is settled; tests use this to check that.
    pub fn insert_accounting_checkpoint(&mut self, t: SimTime) {
        self.checkpoints.insert(t);
    }

    /// Removes and returns the earliest checkpoint if it is due before the next event.
    fn take_due_checkpoint(&mut self, limit: SimTime) -> Option<SimTime> {
        let next_event = self.state.scheduler.peek()?.0.time;
        let t = *self.checkpoints.first()?;
        (t <= next_event && t <= limit).then(|| {
            self.checkpoints.pop_first();
            t
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::{ColumnSpec, LegSpec, ScheduleSpec};
    use ach_logistics::StockKind;
    use ach_world::{ChunkCoord, DetailParams, SourceInfo, TerrainManifest};
    use proptest::prelude::*;
    use std::sync::OnceLock;

    const ROUTES: &str = r#"(routes: [
        (id: "road", name: "Road", kind: road, condition: worn,
         vertices: [(1000, 960), (13000, 960)]),
    ])"#;

    const SITES: &str = r#"(sites: [
        (id: "a", name: "A", kind: "camp", pos: (1000, 960)),
        (id: "b", name: "B", kind: "camp", pos: (13000, 960)),
    ])"#;

    /// A flat 14 km world, written once.
    fn world() -> &'static Path {
        static WORLD: OnceLock<std::path::PathBuf> = OnceLock::new();
        WORLD.get_or_init(|| {
            let dir = std::env::temp_dir().join("ach_sim_accounting_world");
            let manifest = TerrainManifest {
                name: "flat".into(),
                min_chunk: ChunkCoord { cx: 0, cy: 0 },
                max_chunk: ChunkCoord { cx: 6, cy: 0 },
                detail: DetailParams {
                    amplitude_cm: 0,
                    ..DetailParams::default()
                },
                source: SourceInfo {
                    dataset: "synthetic".into(),
                    tiles: vec![],
                    transform: "none".into(),
                    tool_version: "0".into(),
                },
            };
            Terrain::from_fn(manifest, |_, _| 0)
                .save(&dir.join("terrain"))
                .unwrap();
            std::fs::write(dir.join("routes.ron"), ROUTES).unwrap();
            std::fs::write(dir.join("sites.ron"), SITES).unwrap();
            dir
        })
    }

    /// A slow multi-day march whose water crosses every threshold on the way.
    fn scenario() -> Scenario {
        let path = |name: &str| world().join(name).display().to_string();
        Scenario {
            id: "accounting".into(),
            seed: 3,
            start: "D1 05:00".into(),
            end: "D6 05:00".into(),
            terrain_dir: path("terrain"),
            roster: "content/people/red_ledger_founders.ron".into(),
            profiles: "content/movement/profiles.ron".into(),
            consumption: "content/logistics/consumption.ron".into(),
            routes: path("routes.ron"),
            sites: path("sites.ron"),
            columns: vec![ColumnSpec {
                name: "Slow column".into(),
                leader: "ione_var".into(),
                reporters: vec![],
                persons: 12,
                profile: "foot".into(),
                posture_permille: 160,
                stocks: [(StockKind::Water, 200_000), (StockKind::Food, 100_000)].into(),
                start_site: "a".into(),
                legs: vec![LegSpec::FollowRoute {
                    route: "road".into(),
                    to_site: "b".into(),
                }],
                schedule: ScheduleSpec {
                    windows: vec![
                        ("05:00".into(), "11:00".into()),
                        ("15:00".into(), "19:00".into()),
                    ],
                    march_minutes: 50,
                    halt_minutes: 10,
                },
            }],
        }
    }

    /// Runs to the end with no-op checkpoints at `checkpoints_ms`; returns state and journal
    /// fingerprint.
    fn run(checkpoints_ms: &[i64]) -> (SimState, u64) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut sim = Sim::new(&scenario(), &root).unwrap();
        for &ms in checkpoints_ms {
            sim.insert_accounting_checkpoint(SimTime(ms));
        }
        let mut journal = JournalWriter::create(Vec::new(), sim.journal_header()).unwrap();
        let summary = sim.run_until(SimTime(i64::MAX), &mut journal).unwrap();
        (sim.state().clone(), summary.fingerprint)
    }

    #[test]
    fn baseline_arrives_and_crosses_thresholds() {
        let (state, _) = run(&[]);
        let column = state.columns.values().next().unwrap();
        assert!(column.arrived());
        assert!(column.watches[&StockKind::Water].next > 0);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(24))]

        /// Settling accounting at extra times changes neither stocks, nor arrival, nor journal.
        #[test]
        fn accounting_is_independent_of_extra_wakeups(
            checkpoints in prop::collection::vec(0i64..400_000_000, 0..40),
        ) {
            let (base_state, base_fp) = run(&[]);
            let (state, fp) = run(&checkpoints);
            prop_assert_eq!(fp, base_fp);
            prop_assert_eq!(state, base_state);
        }
    }
}
