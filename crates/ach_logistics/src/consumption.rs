//! Exact integer consumption of stocks per person-hour.
//! Implements Simulation §16.2 (demand and endurance by activity) with the water
//! priority of High-Level Design §11.3. Thresholds can be scheduled from
//! [`time_to_consume`] instead of polled.

use crate::error::LogisticsError;
use crate::stock::StockKind;
use ach_core::{Milli, SimDuration, SimTime};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

const MS_PER_HOUR: i128 = 3_600_000;

/// What a group is doing, which sets how fast it consumes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Activity {
    /// Moving along a route.
    Marching,
    /// Stopped but alert.
    Halted,
    /// Stopped and resting.
    Resting,
}

/// Per-person, per-hour rates by (kind, activity), in milli-units. Missing entries are zero.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumptionTable {
    /// Milli-units consumed per person-hour.
    pub rates: BTreeMap<(StockKind, Activity), Milli>,
}

/// On-disk form: the table plus a status marker (`"TUNING"`), which is not kept.
#[derive(Deserialize)]
struct File {
    status: String,
    rates: BTreeMap<(StockKind, Activity), Milli>,
}

impl ConsumptionTable {
    /// Loads a table from a RON file. Negative rates are rejected.
    pub fn load(path: &Path) -> Result<Self, LogisticsError> {
        let shown = path.display().to_string();
        let text = std::fs::read_to_string(path).map_err(|e| LogisticsError::Io {
            path: shown.clone(),
            message: e.to_string(),
        })?;
        let file: File = ron::from_str(&text).map_err(|e| LogisticsError::Parse {
            path: shown,
            message: e.to_string(),
        })?;
        if let Some((key, _)) = file.rates.iter().find(|(_, r)| r.0 < 0) {
            return Err(LogisticsError::NegativeRate(format!(
                "{key:?} in {} table",
                file.status
            )));
        }
        Ok(Self { rates: file.rates })
    }

    fn rate(&self, kind: StockKind, activity: Activity) -> i128 {
        self.rates
            .get(&(kind, activity))
            .map_or(0, |r| i128::from(r.0))
    }
}

/// Exact cumulative consumption: `floor(rate_per_hour * persons * elapsed_ms / 3_600_000)`,
/// computed in `i128` and saturated to the `i64` range. Negative `elapsed` counts as zero.
pub fn cumulative(
    table: &ConsumptionTable,
    kind: StockKind,
    activity: Activity,
    persons: u32,
    elapsed: SimDuration,
) -> Milli {
    let ms = i128::from(elapsed.0.max(0));
    let v = table.rate(kind, activity) * i128::from(persons) * ms / MS_PER_HOUR;
    Milli(i64::try_from(v).unwrap_or(i64::MAX))
}

/// Consumption over `[t0, t1]` within ONE activity segment that began at `seg_start`,
/// computed as `cumulative(t1 - seg_start) - cumulative(t0 - seg_start)`, so splitting
/// an interval never changes the total. Times before the segment start count as the
/// start, and `t1 < t0` yields zero.
pub fn consumed_between(
    table: &ConsumptionTable,
    kind: StockKind,
    activity: Activity,
    persons: u32,
    seg_start: SimTime,
    t0: SimTime,
    t1: SimTime,
) -> Milli {
    let at = |t: SimTime| cumulative(table, kind, activity, persons, t.since(seg_start));
    Milli(at(t1).0.saturating_sub(at(t0).0).max(0))
}

/// Smallest elapsed duration since segment start at which cumulative consumption reaches
/// `target`. `None` if the rate for `persons` is zero (never) or the answer exceeds the
/// `i64` millisecond range. A target of zero or less is reached immediately.
pub fn time_to_consume(
    table: &ConsumptionTable,
    kind: StockKind,
    activity: Activity,
    persons: u32,
    target: Milli,
) -> Option<SimDuration> {
    let per_hour = table.rate(kind, activity) * i128::from(persons);
    if per_hour == 0 {
        return None;
    }
    let need = i128::from(target.0.max(0)) * MS_PER_HOUR;
    let ms = (need + per_hour - 1) / per_hour;
    i64::try_from(ms).ok().map(SimDuration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn table() -> ConsumptionTable {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/logistics/consumption.ron");
        ConsumptionTable::load(&path).unwrap()
    }

    const ACTIVITIES: [Activity; 3] = [Activity::Marching, Activity::Halted, Activity::Resting];

    #[test]
    fn content_values() {
        let t = table();
        let w = |a| t.rates[&(StockKind::Water, a)];
        assert_eq!(w(Activity::Marching), Milli(450));
        assert_eq!(w(Activity::Halted), Milli(200));
        assert_eq!(w(Activity::Resting), Milli(150));
        assert_eq!(t.rates[&(StockKind::Food, Activity::Resting)], Milli(42));
    }

    #[test]
    fn three_day_march_known_answer() {
        // 30 people, 72 hours at 450 mL: 30 * 72 * 0.45 L = 972 L.
        let got = cumulative(
            &table(),
            StockKind::Water,
            Activity::Marching,
            30,
            SimDuration::from_mins(72 * 60),
        );
        assert_eq!(got, Milli(972_000));
    }

    #[test]
    fn unlisted_kind_never_consumes() {
        let t = table();
        assert_eq!(
            time_to_consume(&t, StockKind::Fuel, Activity::Marching, 10, Milli(1)),
            None
        );
    }

    #[test]
    fn missing_file_is_io_error() {
        let err = ConsumptionTable::load(Path::new("/nonexistent/x.ron"));
        assert!(matches!(err, Err(LogisticsError::Io { .. })));
    }

    #[test]
    fn negative_rate_rejected() {
        let path = std::env::temp_dir().join("ach_logistics_neg_rate.ron");
        std::fs::write(
            &path,
            "(status: \"TUNING\", rates: {(Water, Marching): Milli(-1)})",
        )
        .unwrap();
        assert!(matches!(
            ConsumptionTable::load(&path),
            Err(LogisticsError::NegativeRate(_))
        ));
    }

    proptest! {
        #[test]
        fn splitting_invariance(
            persons in 0u32..5_000,
            act in 0usize..3,
            seg in -1_000_000i64..1_000_000_000,
            mut cuts in proptest::collection::vec(0i64..500_000_000, 2..8),
        ) {
            let t = table();
            let seg_start = SimTime(seg);
            cuts.sort_unstable();
            let times: Vec<SimTime> = cuts.iter().map(|c| SimTime(seg + c)).collect();
            let between = |a: SimTime, b: SimTime|
                consumed_between(&t, StockKind::Water, ACTIVITIES[act], persons, seg_start, a, b).0;
            let sum: i64 = times.windows(2).map(|w| between(w[0], w[1])).sum();
            prop_assert_eq!(sum, between(times[0], times[times.len() - 1]));
        }

        #[test]
        fn time_to_consume_is_exact(
            persons in 1u32..5_000,
            act in 0usize..3,
            target in 1i64..10_000_000,
        ) {
            let t = table();
            let a = ACTIVITIES[act];
            let d = time_to_consume(&t, StockKind::Food, a, persons, Milli(target)).unwrap();
            prop_assert!(cumulative(&t, StockKind::Food, a, persons, d).0 >= target);
            let before = SimDuration(d.0 - 1);
            prop_assert!(cumulative(&t, StockKind::Food, a, persons, before).0 < target);
        }
    }
}
