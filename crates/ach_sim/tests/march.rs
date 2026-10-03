//! Acceptance tests for the march simulation (P1-11), on synthetic flat terrain.
//!
//! One flat world is written to the target directory once: a 90 km road for the tempo
//! tests and a 6 km road with a waypoint for the logbook snapshot. Content that exists in the
//! repository (roster, profiles, consumption) is read from there.
#![allow(clippy::unwrap_used)]

use ach_core::{JournalEntry, JournalReader, JournalWriter, Milli, PerMille, SimTime};
use ach_logistics::{ConsumptionTable, StockKind, cumulative};
use ach_sim::{
    ColumnSpec, EndReason, LegSpec, MarchSchedule, Scenario, ScheduleSpec, Sim, SimEvent,
};
use ach_world::{ChunkCoord, DetailParams, SourceInfo, Terrain, TerrainManifest};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const REPO_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

const ROUTES: &str = r#"(routes: [
    (id: "long_road", name: "Long Road", kind: road, condition: worn,
     vertices: [(1000, 960), (91000, 960)]),
    (id: "short_road", name: "Short Road", kind: road, condition: worn,
     vertices: [(1000, 1500), (4000, 1500), (7000, 1500)]),
])"#;

const SITES: &str = r#"(status: "TEST", sites: [
    (id: "far_a", name: "Far A", kind: "camp", pos: (1000, 960), canon: "synthetic"),
    (id: "far_b", name: "Far B", kind: "camp", pos: (91000, 960), canon: "synthetic"),
    (id: "near_a", name: "Near A", kind: "camp", pos: (1000, 1500), canon: "synthetic"),
    (id: "near_mid", name: "Near Mid", kind: "ford", pos: (4000, 1500), canon: "synthetic"),
    (id: "near_b", name: "Near B", kind: "camp", pos: (7000, 1500), canon: "synthetic"),
])"#;

/// Writes the synthetic world once and returns its directory.
fn world() -> &'static Path {
    static WORLD: OnceLock<PathBuf> = OnceLock::new();
    WORLD.get_or_init(|| {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("p1_11_world");
        let manifest = TerrainManifest {
            name: "p1_11_flat".into(),
            min_chunk: ChunkCoord { cx: 0, cy: 0 },
            max_chunk: ChunkCoord { cx: 47, cy: 0 },
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

fn windows() -> ScheduleSpec {
    ScheduleSpec {
        windows: vec![
            ("05:00".into(), "11:00".into()),
            ("15:00".into(), "19:00".into()),
        ],
        march_minutes: 50,
        halt_minutes: 10,
    }
}

fn scenario(id: &str, end: &str, column: ColumnSpec) -> Scenario {
    let path = |name: &str| world().join(name).display().to_string();
    Scenario {
        id: id.into(),
        seed: 11,
        start: "D1 05:00".into(),
        end: end.into(),
        terrain_dir: path("terrain"),
        roster: "content/people/red_ledger_founders.ron".into(),
        profiles: "content/movement/profiles.ron".into(),
        consumption: "content/logistics/consumption.ron".into(),
        routes: path("routes.ron"),
        sites: path("sites.ron"),
        columns: vec![column],
    }
}

fn column(
    start_site: &str,
    legs: Vec<LegSpec>,
    posture: i32,
    persons: u32,
    water_milli: i64,
) -> ColumnSpec {
    ColumnSpec {
        name: "Red Ledger caravan".into(),
        leader: "ione_var".into(),
        reporters: vec!["tessa_ruun".into()],
        persons,
        profile: "foot".into(),
        posture_permille: posture,
        stocks: BTreeMap::from([
            (StockKind::Water, water_milli),
            (StockKind::Food, i64::from(persons) * 10_000),
        ]),
        start_site: start_site.into(),
        legs,
        schedule: windows(),
    }
}

fn follow(route: &str, to_site: &str) -> LegSpec {
    LegSpec::FollowRoute {
        route: route.into(),
        to_site: to_site.into(),
    }
}

/// 90 km on a worn road at posture 620 (the tempo scenario).
fn long_march(water_milli: i64, end: &str) -> Scenario {
    let leg = vec![follow("long_road", "far_b")];
    scenario(
        "long_march",
        end,
        column("far_a", leg, 620, 20, water_milli),
    )
}

/// 6 km in two legs at a slow posture (a column with wounded), arriving the same day.
fn short_march() -> Scenario {
    let legs = vec![
        follow("short_road", "near_mid"),
        follow("short_road", "near_b"),
    ];
    scenario(
        "six_km_day",
        "D2 00:00",
        column("near_a", legs, 160, 6, 30_000),
    )
}

struct Outcome {
    sim: Sim,
    entries: Vec<JournalEntry<SimEvent>>,
    arrival: Option<SimTime>,
}

fn run(scenario: &Scenario) -> Outcome {
    let mut sim = Sim::new(scenario, Path::new(REPO_ROOT)).unwrap();
    let mut journal = JournalWriter::create(Vec::new(), sim.journal_header()).unwrap();
    let summary = sim.run_until(SimTime(i64::MAX), &mut journal).unwrap();
    assert_eq!(summary.fingerprint, journal.fingerprint());
    let bytes = journal.into_inner();
    let (_, reader) = JournalReader::<_, SimEvent>::open(&bytes[..]).unwrap();
    let entries: Vec<_> = reader.collect::<Result<_, _>>().unwrap();
    assert_eq!(entries.len() as u64, summary.events);
    Outcome {
        sim,
        entries,
        arrival: summary.arrived.first().map(|&(_, t)| t),
    }
}

fn only_column(outcome: &Outcome) -> &ach_sim::Column {
    outcome.sim.state().columns.values().next().unwrap()
}

#[test]
fn tempo_of_a_guarded_foot_column_over_90_km() {
    let outcome = run(&long_march(2_000_000, "D12 05:00"));
    let arrival = outcome.arrival.expect("the column arrives");
    println!("90 km arrival: {arrival}");
    assert!(arrival >= SimTime::at(4, 5, 0), "{arrival}");
    assert!(arrival <= SimTime::at(6, 5, 0), "{arrival}");
    let last = outcome.entries.last().unwrap();
    assert_eq!(last.time, arrival);
    assert_eq!(
        last.event,
        SimEvent::ScenarioEnded {
            reason: EndReason::AllArrived
        }
    );
}

#[test]
fn distance_never_decreases() {
    let outcome = run(&long_march(2_000_000, "D12 05:00"));
    let (mut covered_m, mut last_site_m, mut last_x_cm) = (0, 0, 0);
    for entry in &outcome.entries {
        match &entry.event {
            SimEvent::SiteReached {
                distance_from_start_m,
                ..
            } => {
                assert!(*distance_from_start_m >= last_site_m);
                last_site_m = *distance_from_start_m;
            }
            SimEvent::ColumnDayEnded {
                distance_today_m, ..
            } => {
                assert!(*distance_today_m >= 0);
                covered_m += distance_today_m;
            }
            _ => {}
        }
        // The road runs due east, so progress is the x coordinate.
        if let SimEvent::MarchWindowStarted { pos, .. }
        | SimEvent::MarchWindowEnded { pos, .. }
        | SimEvent::ShortHaltBegan { pos, .. }
        | SimEvent::ShortHaltEnded { pos, .. }
        | SimEvent::ColumnDayEnded { pos, .. } = &entry.event
        {
            assert!(pos.x_cm >= last_x_cm, "{:?}", entry.event);
            last_x_cm = pos.x_cm;
        }
    }
    // Daily distances are each floored, so their sum trails the total by under a metre a day.
    let days = outcome.arrival.unwrap().day();
    assert!((90_000 - covered_m).abs() <= days, "{covered_m}");
}

#[test]
fn consumption_is_conserved() {
    let scenario = long_march(2_000_000, "D12 05:00");
    let outcome = run(&scenario);
    let arrival = outcome.arrival.unwrap();
    let spec = &scenario.columns[0];
    let schedule = MarchSchedule::new(&spec.schedule).unwrap();
    let table = ConsumptionTable::load(&Path::new(REPO_ROOT).join(&scenario.consumption)).unwrap();
    let column = only_column(&outcome);
    for kind in [StockKind::Water, StockKind::Food] {
        // Walk the activity segments independently of the simulation's own accounting.
        let (mut cursor, mut used) = (SimTime::at(1, 5, 0), 0);
        while cursor < arrival {
            let next = schedule.next_boundary(cursor).0.min(arrival);
            let activity = schedule.activity_at(cursor);
            used += cumulative(&table, kind, activity, spec.persons, next.since(cursor)).0;
            cursor = next;
        }
        let initial = spec.stocks[&kind];
        assert_eq!(initial - column.stocks.get(kind).0, used, "{kind:?}");
    }
}

#[test]
fn water_thresholds_fire_in_order_once_at_the_crossing() {
    let initial = 450_000;
    let outcome = run(&long_march(initial, "D12 05:00"));
    let water: Vec<_> = outcome
        .entries
        .iter()
        .filter_map(|e| match &e.event {
            SimEvent::StockThreshold {
                kind: StockKind::Water,
                remaining,
                remaining_permille,
                ..
            } => Some((e.time, *remaining, remaining_permille.0)),
            _ => None,
        })
        .collect();
    let permilles: Vec<i32> = water.iter().map(|w| w.2).collect();
    assert_eq!(permilles, [500, 250, 100]);
    assert!(water.windows(2).all(|w| w[0].0 < w[1].0));
    // Consumption is under one milli-liter a millisecond, so at the first instant at or below
    // a level the remaining amount equals the level exactly.
    for (_, remaining, permille) in &water {
        assert_eq!(*remaining, Milli(initial).scale(PerMille(*permille)));
    }
    let exhausted = outcome.entries.iter().any(|e| {
        matches!(
            e.event,
            SimEvent::StockExhausted {
                kind: StockKind::Water,
                ..
            }
        )
    });
    assert_eq!(
        exhausted,
        only_column(&outcome).stocks.get(StockKind::Water) == Milli(0)
    );
}

#[test]
fn journal_is_ordered_and_ends_with_the_scenario() {
    let outcome = run(&long_march(450_000, "D12 05:00"));
    assert!(outcome.entries.windows(2).all(|w| w[0].time <= w[1].time));
    assert!(
        outcome
            .entries
            .windows(2)
            .enumerate()
            .all(|(i, w)| w[1].seq == w[0].seq + 1 && i as u64 == w[0].seq)
    );
    let ends = outcome
        .entries
        .iter()
        .filter(|e| matches!(e.event, SimEvent::ScenarioEnded { .. }))
        .count();
    assert_eq!(ends, 1);
    assert!(matches!(
        outcome.entries.last().unwrap().event,
        SimEvent::ScenarioEnded { .. }
    ));
}

#[test]
fn time_limit_ends_an_unfinished_march() {
    let outcome = run(&long_march(2_000_000, "D2 12:00"));
    assert_eq!(outcome.arrival, None);
    let last = outcome.entries.last().unwrap();
    assert_eq!(last.time, SimTime::at(2, 12, 0));
    assert_eq!(
        last.event,
        SimEvent::ScenarioEnded {
            reason: EndReason::TimeLimit
        }
    );
    // Consumption was settled up to the limit, not just to the last event.
    let water = only_column(&outcome).stocks.get(StockKind::Water);
    assert!(water.0 < 2_000_000 - 20 * 450 * 10);
}

#[test]
fn run_is_replayable() {
    let a = run(&short_march());
    let b = run(&short_march());
    assert_eq!(a.entries, b.entries);
    assert_eq!(a.sim.state(), b.sim.state());
}

#[test]
fn unknown_site_fails_at_load() {
    let mut s = short_march();
    s.columns[0].start_site = "nowhere".into();
    assert!(matches!(
        Sim::new(&s, Path::new(REPO_ROOT)),
        Err(ach_sim::SimError::Unknown { what: "site", .. })
    ));
}

/// One line per journal entry: sequence, time to the millisecond, then the event in RON.
fn logbook(entries: &[JournalEntry<SimEvent>]) -> String {
    entries
        .iter()
        .map(|e| {
            let tod = e.time.time_of_day().0;
            let (h, m, s, ms) = (
                tod / 3_600_000,
                tod / 60_000 % 60,
                tod / 1000 % 60,
                tod % 1000,
            );
            let event = ron::to_string(&e.event).unwrap();
            format!(
                "{:03} D{} {h:02}:{m:02}:{s:02}.{ms:03}  {event}",
                e.seq,
                e.time.day()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn six_km_day_logbook() {
    let outcome = run(&short_march());
    insta::assert_snapshot!(logbook(&outcome.entries));
}
