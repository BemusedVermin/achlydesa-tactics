//! Column state, exact accounting, and planning of a column's next event.
//! Implements Simulation §3.1-§3.3 (event-driven clock) and Technical Design §21.2
//! (advancement classes: a column on a leg is advanced analytically between events).
//!
//! A column has at most one pending progress event: the earliest of its next schedule
//! boundary, the end of its current leg, and its next stock threshold. Between events nothing
//! is simulated; [`Column::advance_to`] settles marching progress and consumption exactly, so
//! the result does not depend on how many times it is called on the way.

use crate::error::SimError;
use crate::events::{SimEvent, phase};
use crate::kinematics::Leg;
use crate::scenario::ColumnSpec;
use crate::schedule::{Boundary, MarchSchedule};
use ach_core::{Milli, PerMille, SimDuration, SimTime, WorldPos};
use ach_logistics::{
    Activity, ConsumptionTable, StockKind, StockSet, consumed_between, cumulative, time_to_consume,
};
use ach_people::CharacterId;
use ach_sched::EventHandle;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

ach_core::define_id!(
    /// Identifier of a marching column.
    ColumnId
);

/// Stock thresholds as per-mille of the initial amount, in the order they fire.
/// Exhaustion (zero) follows the last one.
pub const THRESHOLDS_PERMILLE: [i32; 3] = [500, 250, 100];

const MM_PER_M: i64 = 1000;

/// Threshold bookkeeping for one stock kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockWatch {
    /// Amount at departure; thresholds are fractions of this.
    pub initial: Milli,
    /// Index of the next threshold to fire: `THRESHOLDS_PERMILLE.len()` means exhaustion,
    /// one more means everything has fired.
    pub next: usize,
}

impl StockWatch {
    /// The remaining amount at or below which the next threshold fires, if any is left.
    fn next_level(&self) -> Option<Milli> {
        match self.next.cmp(&THRESHOLDS_PERMILLE.len()) {
            std::cmp::Ordering::Less => {
                Some(self.initial.scale(PerMille(THRESHOLDS_PERMILLE[self.next])))
            }
            std::cmp::Ordering::Equal => Some(Milli(0)),
            std::cmp::Ordering::Greater => None,
        }
    }
}

/// A column event chosen by [`Column::plan_next`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Planned {
    /// When it fires.
    pub time: SimTime,
    /// Phase within that time.
    pub phase: u8,
    /// The event to journal.
    pub event: SimEvent,
}

/// Everything needed to start a column.
pub struct ColumnInit<'a> {
    /// Assigned identifier.
    pub id: ColumnId,
    /// The scenario entry.
    pub spec: &'a ColumnSpec,
    /// Resolved leader.
    pub leader: CharacterId,
    /// Resolved reporters.
    pub reporters: Vec<CharacterId>,
    /// Precomputed legs, in order; at least one.
    pub legs: Vec<Leg>,
    /// Validated schedule.
    pub schedule: MarchSchedule,
    /// Departure time.
    pub start: SimTime,
}

/// A marching column. All of it is simulation state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Column {
    /// Identifier.
    pub id: ColumnId,
    /// Display name.
    pub name: String,
    /// Leader.
    pub leader: CharacterId,
    /// People who report on the column.
    pub reporters: Vec<CharacterId>,
    /// Head count.
    pub persons: u32,
    /// Stocks on hand.
    pub stocks: StockSet,
    /// When the column marches.
    pub schedule: MarchSchedule,
    /// Departure site id.
    pub start_site: String,
    /// Legs of the plan.
    pub legs: Vec<Leg>,
    /// Index of the current leg; equals `legs.len()` once arrived.
    pub leg_index: usize,
    /// Marching ms done in the current leg.
    pub leg_march_ms: i64,
    /// Length of the completed legs, mm.
    pub completed_len_mm: i64,
    /// Length already covered when the current day began, mm.
    pub day_start_len_mm: i64,
    /// Departure time.
    pub departed_at: SimTime,
    /// Time up to which marching and consumption are settled.
    pub last_update: SimTime,
    /// Activity since `segment_start`.
    pub activity: Activity,
    /// Start of the current activity segment, which anchors consumption rounding.
    pub segment_start: SimTime,
    /// Last schedule boundary already handled; the next one is searched after it.
    pub boundary_cursor: SimTime,
    /// Threshold bookkeeping per stock kind.
    pub watches: BTreeMap<StockKind, StockWatch>,
    /// When the column reached its final site.
    pub arrived_at: Option<SimTime>,
    /// The pending progress event.
    pub pending: Option<EventHandle>,
    /// The pending end-of-day event.
    pub day_end: Option<EventHandle>,
}

impl Column {
    /// Builds a column standing at the start of its first leg.
    pub fn new(init: ColumnInit<'_>) -> Result<Self, SimError> {
        let ColumnInit {
            id,
            spec,
            leader,
            reporters,
            legs,
            schedule,
            start,
        } = init;
        let mut stocks = StockSet::default();
        let mut watches = BTreeMap::new();
        for (&kind, &amount) in &spec.stocks {
            stocks.add(kind, Milli(amount))?;
            if amount > 0 {
                watches.insert(
                    kind,
                    StockWatch {
                        initial: Milli(amount),
                        next: 0,
                    },
                );
            }
        }
        Ok(Self {
            id,
            name: spec.name.clone(),
            leader,
            reporters,
            persons: spec.persons,
            stocks,
            activity: schedule.activity_at(start),
            schedule,
            start_site: spec.start_site.clone(),
            legs,
            leg_index: 0,
            leg_march_ms: 0,
            completed_len_mm: 0,
            day_start_len_mm: 0,
            departed_at: start,
            last_update: start,
            segment_start: start,
            boundary_cursor: SimTime(start.0 - 1),
            watches,
            arrived_at: None,
            pending: None,
            day_end: None,
        })
    }

    /// Whether the column has reached its final site.
    pub fn arrived(&self) -> bool {
        self.arrived_at.is_some()
    }

    /// Marching ms done in the current leg at `t >= last_update`.
    fn progress_at(&self, t: SimTime) -> i64 {
        self.leg_march_ms + self.schedule.marching_between(self.last_update, t).0
    }

    /// Position at `t >= last_update`, assuming no event intervenes.
    pub fn position_at(&self, t: SimTime) -> WorldPos {
        match self.legs.get(self.leg_index) {
            Some(leg) if !self.arrived() => leg.position_at_march_ms(self.progress_at(t)),
            _ => self.final_pos(),
        }
    }

    fn final_pos(&self) -> WorldPos {
        self.legs
            .last()
            .expect("invariant: a column has at least one leg")
            .end()
    }

    /// Distance walked since departure at `t >= last_update`, mm.
    fn distance_mm_at(&self, t: SimTime) -> i64 {
        match self.legs.get(self.leg_index) {
            Some(leg) if !self.arrived() => {
                self.completed_len_mm + leg.distance_at_march_ms(self.progress_at(t))
            }
            _ => self.completed_len_mm,
        }
    }

    /// Settles marching progress and consumption up to `t`.
    ///
    /// The span `[last_update, t)` is split at schedule boundaries; each piece adds its
    /// marching time to the current leg and consumes stocks at the activity's rate. Does
    /// nothing if `t <= last_update`.
    pub fn advance_to(&mut self, t: SimTime, table: &ConsumptionTable) {
        let mut cur = self.last_update;
        while cur < t {
            let boundary = (!self.arrived()).then(|| self.schedule.next_boundary(cur).0);
            let piece_end = boundary.map_or(t, |b| b.min(t));
            self.consume(table, cur, piece_end);
            if self.activity == Activity::Marching {
                self.leg_march_ms += piece_end.since(cur).0;
            }
            cur = piece_end;
            if boundary == Some(piece_end) {
                self.activity = self.schedule.activity_at(piece_end);
                self.segment_start = piece_end;
            }
        }
        self.last_update = self.last_update.max(t);
    }

    fn consume(&mut self, table: &ConsumptionTable, from: SimTime, to: SimTime) {
        let kinds: Vec<StockKind> = self.stocks.iter().map(|(kind, _)| kind).collect();
        for kind in kinds {
            let used = consumed_between(
                table,
                kind,
                self.activity,
                self.persons,
                self.segment_start,
                from,
                to,
            );
            self.stocks.remove_up_to(kind, used);
        }
    }

    /// The earliest of the column's next boundary, leg end and stock threshold that falls
    /// at or before `end`. Ties go to the lower phase. Call only on a settled column.
    pub fn plan_next(&self, table: &ConsumptionTable, end: SimTime) -> Option<Planned> {
        [
            self.plan_boundary(),
            self.plan_site(),
            self.plan_stock(table, end),
        ]
        .into_iter()
        .flatten()
        .filter(|p| p.time <= end)
        .min_by_key(|p| (p.time, p.phase))
    }

    fn plan_boundary(&self) -> Option<Planned> {
        if self.arrived() {
            return None;
        }
        let (time, kind) = self.schedule.next_boundary(self.boundary_cursor);
        let (column, pos) = (self.id, self.position_at(time));
        let event = match kind {
            Boundary::WindowStart => SimEvent::MarchWindowStarted { column, pos },
            Boundary::WindowEnd => SimEvent::MarchWindowEnded {
                column,
                pos,
                marched_today_m: (self.distance_mm_at(time) - self.day_start_len_mm) / MM_PER_M,
            },
            Boundary::HaltStart => SimEvent::ShortHaltBegan { column, pos },
            Boundary::HaltEnd => SimEvent::ShortHaltEnded { column, pos },
        };
        Some(Planned {
            time,
            phase: phase::SCHEDULE,
            event,
        })
    }

    fn plan_site(&self) -> Option<Planned> {
        let leg = self.legs.get(self.leg_index).filter(|_| !self.arrived())?;
        let left = SimDuration(leg.total_march_ms() - self.leg_march_ms);
        Some(Planned {
            time: self
                .schedule
                .wall_time_after_marching(self.last_update, left),
            phase: phase::ARRIVAL,
            event: SimEvent::SiteReached {
                column: self.id,
                site: leg.to_site.clone(),
                pos: leg.end(),
                distance_from_start_m: (self.completed_len_mm + leg.total_len_mm()) / MM_PER_M,
            },
        })
    }

    fn plan_stock(&self, table: &ConsumptionTable, end: SimTime) -> Option<Planned> {
        self.watches
            .iter()
            .filter_map(|(&kind, watch)| {
                let level = watch.next_level()?;
                let (time, remaining) = self.stock_crossing(table, kind, level, end)?;
                let event = if watch.next < THRESHOLDS_PERMILLE.len() {
                    SimEvent::StockThreshold {
                        column: self.id,
                        kind,
                        remaining,
                        remaining_permille: remaining_permille(remaining, watch.initial),
                    }
                } else {
                    SimEvent::StockExhausted {
                        column: self.id,
                        kind,
                    }
                };
                Some(Planned {
                    time,
                    phase: phase::STOCK,
                    event,
                })
            })
            .min_by_key(|p| p.time)
    }

    /// First time at or before `end` when `kind` has fallen to `level` or below, with the
    /// amount left then. Walks forward over activity segments without changing state.
    fn stock_crossing(
        &self,
        table: &ConsumptionTable,
        kind: StockKind,
        level: Milli,
        end: SimTime,
    ) -> Option<(SimTime, Milli)> {
        let mut remaining = self.stocks.get(kind).0;
        if remaining <= level.0 {
            return Some((self.last_update, Milli(remaining)));
        }
        let (mut cur, mut activity, mut seg_start) =
            (self.last_update, self.activity, self.segment_start);
        let persons = self.persons;
        while cur < end {
            let boundary = (!self.arrived()).then(|| self.schedule.next_boundary(cur).0);
            let piece_end = boundary.map_or(end, |b| b.min(end));
            let done = cumulative(table, kind, activity, persons, cur.since(seg_start)).0;
            let target = Milli(remaining - level.0 + done);
            if let Some(d) = time_to_consume(table, kind, activity, persons, target) {
                let at = seg_start.checked_add(d)?;
                if at <= piece_end {
                    let used = cumulative(table, kind, activity, persons, d).0 - done;
                    return Some((at, Milli((remaining - used).max(0))));
                }
            }
            let used = consumed_between(table, kind, activity, persons, seg_start, cur, piece_end);
            remaining -= used.0;
            cur = piece_end;
            if boundary == Some(piece_end) {
                activity = self.schedule.activity_at(piece_end);
                seg_start = piece_end;
            }
        }
        None
    }

    /// Applies a column event that fired at `t` on a settled column and returns the events
    /// to journal, in order.
    ///
    /// A site reached at the end of the last leg also yields [`SimEvent::ColumnArrived`] and
    /// the arrival day's [`SimEvent::ColumnDayEnded`]. A queued `ColumnDayEnded` is only a
    /// wake-up: the journaled one is built here from the settled state.
    pub fn apply(&mut self, event: SimEvent, t: SimTime) -> Vec<SimEvent> {
        match &event {
            SimEvent::MarchWindowStarted { .. }
            | SimEvent::MarchWindowEnded { .. }
            | SimEvent::ShortHaltBegan { .. }
            | SimEvent::ShortHaltEnded { .. } => self.boundary_cursor = t,
            SimEvent::StockThreshold { kind, .. } | SimEvent::StockExhausted { kind, .. } => {
                if let Some(watch) = self.watches.get_mut(kind) {
                    watch.next += 1;
                }
            }
            SimEvent::SiteReached { .. } => return self.reach_site(event, t),
            SimEvent::ColumnDayEnded { .. } => return vec![self.end_day(t)],
            _ => {}
        }
        vec![event]
    }

    fn reach_site(&mut self, reached: SimEvent, t: SimTime) -> Vec<SimEvent> {
        let leg_len = self.legs[self.leg_index].total_len_mm();
        self.completed_len_mm += leg_len;
        self.leg_index += 1;
        self.leg_march_ms = 0;
        let mut out = vec![reached];
        if self.leg_index == self.legs.len() {
            self.arrived_at = Some(t);
            self.activity = Activity::Resting;
            self.segment_start = t;
            out.push(SimEvent::ColumnArrived {
                column: self.id,
                site: self.legs[self.leg_index - 1].to_site.clone(),
                pos: self.final_pos(),
                total_distance_m: self.completed_len_mm / MM_PER_M,
                elapsed: t.since(self.departed_at),
            });
            out.push(self.end_day(t));
        }
        out
    }

    /// The `ColumnDayEnded` event for the day containing `t`; starts the next day's tally.
    pub fn end_day(&mut self, t: SimTime) -> SimEvent {
        let total = self.distance_mm_at(t);
        let event = SimEvent::ColumnDayEnded {
            column: self.id,
            day: t.day(),
            distance_today_m: (total - self.day_start_len_mm) / MM_PER_M,
            stocks: self.stocks.clone(),
            pos: self.position_at(t),
        };
        self.day_start_len_mm = total;
        event
    }
}

/// `floor(remaining * 1000 / initial)`.
fn remaining_permille(remaining: Milli, initial: Milli) -> PerMille {
    let v = i128::from(remaining.0) * 1000 / i128::from(initial.0.max(1));
    PerMille(i32::try_from(v).unwrap_or(i32::MAX))
}
