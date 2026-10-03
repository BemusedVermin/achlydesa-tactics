//! March schedules: daily windows with march/halt cycles, and exact time arithmetic over them.
//! Implements High-Level Design §4.1b (desert march windows and short halts) and
//! Technical Design §21.2 (analytic advancement needs exact inverse time queries).
//!
//! Inside each window, cycles of `march` then `halt` begin at the window start and the final
//! partial cycle is cut at the window end. Outside windows a column rests. All queries are
//! integer arithmetic on a cumulative "marching milliseconds since the epoch" function, so
//! splitting an interval never changes its total.

use crate::error::SimError;
use crate::scenario::ScheduleSpec;
use ach_core::{SimDuration, SimTime};
use ach_logistics::Activity;
use serde::{Deserialize, Serialize};

const DAY_MS: i64 = SimDuration::from_days(1).0;

/// Why the next schedule boundary matters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Boundary {
    /// A march window opens.
    WindowStart,
    /// A march window closes (from marching or from a halt).
    WindowEnd,
    /// A march stretch ends and a short halt begins.
    HaltStart,
    /// A short halt ends and marching resumes.
    HaltEnd,
}

/// Daily march windows and the march/halt cycle inside them.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarchSchedule {
    windows: Vec<(SimDuration, SimDuration)>,
    march: SimDuration,
    halt: SimDuration,
}

/// Parses `"HH:MM"` into an offset within the day.
fn parse_clock(s: &str) -> Result<SimDuration, SimError> {
    let bad = || SimError::Invalid(format!("bad clock time {s:?}, expected \"HH:MM\""));
    let (h, m) = s.split_once(':').ok_or_else(bad)?;
    let field = |p: &str, max: i64| {
        (p.len() == 2 && p.bytes().all(|b| b.is_ascii_digit()))
            .then(|| p.parse::<i64>().ok())
            .flatten()
            .filter(|v| *v <= max)
    };
    let (h, m) = (field(h, 23).ok_or_else(bad)?, field(m, 59).ok_or_else(bad)?);
    Ok(SimDuration(
        SimDuration::from_hours(h).0 + SimDuration::from_mins(m).0,
    ))
}

impl MarchSchedule {
    /// Validates and builds a schedule. Windows are sorted; they must be non-empty, lie
    /// within one day, and be separated by a gap. Both cycle lengths must be positive.
    pub fn new(spec: &ScheduleSpec) -> Result<Self, SimError> {
        let invalid = |why: &str| SimError::Invalid(format!("schedule: {why}"));
        if spec.march_minutes == 0 || spec.halt_minutes == 0 {
            return Err(invalid("march and halt minutes must be positive"));
        }
        let mut windows = spec
            .windows
            .iter()
            .map(|(a, b)| Ok((parse_clock(a)?, parse_clock(b)?)))
            .collect::<Result<Vec<_>, SimError>>()?;
        if windows.is_empty() {
            return Err(invalid("at least one window is required"));
        }
        if windows.iter().any(|(a, b)| a >= b) {
            return Err(invalid("a window must end after it starts"));
        }
        windows.sort();
        if windows.windows(2).any(|w| w[0].1 >= w[1].0) {
            return Err(invalid("windows must not overlap or touch"));
        }
        Ok(Self {
            windows,
            march: SimDuration::from_mins(i64::from(spec.march_minutes)),
            halt: SimDuration::from_mins(i64::from(spec.halt_minutes)),
        })
    }

    fn cycle_ms(&self) -> i64 {
        self.march.0 + self.halt.0
    }

    /// Marching ms in the first `elapsed` ms of a window.
    fn marching_in_first(&self, elapsed: i64) -> i64 {
        let (cycles, rest) = (elapsed / self.cycle_ms(), elapsed % self.cycle_ms());
        cycles * self.march.0 + rest.min(self.march.0)
    }

    fn window_marching(&self, (start, end): (SimDuration, SimDuration)) -> i64 {
        self.marching_in_first(end.0 - start.0)
    }

    fn marching_per_day(&self) -> i64 {
        self.windows.iter().map(|&w| self.window_marching(w)).sum()
    }

    /// Marching ms accrued since the epoch up to `t`.
    fn marching_before(&self, t: SimTime) -> i64 {
        let tod = t.time_of_day().0;
        let today: i64 = self
            .windows
            .iter()
            .map(|&(s, e)| self.marching_in_first((tod.min(e.0) - s.0).max(0)))
            .sum();
        t.0.div_euclid(DAY_MS) * self.marching_per_day() + today
    }

    /// Activity at wall time `t` for a column that is under way.
    pub fn activity_at(&self, t: SimTime) -> Activity {
        let tod = t.time_of_day();
        match self.windows.iter().find(|&&(s, e)| s <= tod && tod < e) {
            None => Activity::Resting,
            Some(&(s, _)) if (tod.0 - s.0) % self.cycle_ms() < self.march.0 => Activity::Marching,
            Some(_) => Activity::Halted,
        }
    }

    /// Marching milliseconds accrued in `[a, b)`. Zero if `b <= a`.
    pub fn marching_between(&self, a: SimTime, b: SimTime) -> SimDuration {
        SimDuration((self.marching_before(b) - self.marching_before(a)).max(0))
    }

    /// Earliest wall time `t >= from` at which `marching_between(from, t) == need`.
    /// A `need` of zero or less gives `from`.
    pub fn wall_time_after_marching(&self, from: SimTime, need: SimDuration) -> SimTime {
        if need.0 <= 0 {
            return from;
        }
        let target = self.marching_before(from) + need.0;
        let per_day = self.marching_per_day();
        let day = (target - 1).div_euclid(per_day);
        let mut left = target - day * per_day;
        for &window in &self.windows {
            let in_window = self.window_marching(window);
            if left <= in_window {
                let cycles = (left - 1) / self.march.0;
                let offset = window.0.0 + cycles * self.cycle_ms() + (left - cycles * self.march.0);
                return SimTime(day * DAY_MS + offset);
            }
            left -= in_window;
        }
        unreachable!("invariant: the remainder is within the day's marching total")
    }

    /// Boundaries within one day as `(offset ms, kind)`, ascending.
    fn boundaries_in_day(&self) -> Vec<(i64, Boundary)> {
        let mut out = Vec::new();
        for &(start, end) in &self.windows {
            out.push((start.0, Boundary::WindowStart));
            let mut cycle_start = start.0;
            loop {
                let halt_start = cycle_start + self.march.0;
                let next_march = cycle_start + self.cycle_ms();
                if halt_start >= end.0 {
                    break;
                }
                out.push((halt_start, Boundary::HaltStart));
                if next_march >= end.0 {
                    break;
                }
                out.push((next_march, Boundary::HaltEnd));
                cycle_start = next_march;
            }
            out.push((end.0, Boundary::WindowEnd));
        }
        out
    }

    /// Next boundary strictly after `t`, with its kind.
    pub fn next_boundary(&self, t: SimTime) -> (SimTime, Boundary) {
        let day = t.0.div_euclid(DAY_MS);
        let in_day = self.boundaries_in_day();
        (day..=day + 1)
            .flat_map(|d| {
                in_day
                    .iter()
                    .map(move |&(off, kind)| (SimTime(d * DAY_MS + off), kind))
            })
            .find(|&(at, _)| at > t)
            .expect("invariant: every day has a boundary, so one follows within a day")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn spec(windows: &[(&str, &str)], march: u32, halt: u32) -> ScheduleSpec {
        ScheduleSpec {
            windows: windows
                .iter()
                .map(|&(a, b)| (a.to_owned(), b.to_owned()))
                .collect(),
            march_minutes: march,
            halt_minutes: halt,
        }
    }

    fn standard() -> MarchSchedule {
        MarchSchedule::new(&spec(&[("05:00", "11:00"), ("15:00", "19:00")], 50, 10)).unwrap()
    }

    fn at(h: i64, m: i64) -> SimTime {
        SimTime::at(1, h, m)
    }

    #[test]
    fn activity_follows_cycles_and_windows() {
        let s = standard();
        assert_eq!(s.activity_at(at(4, 59)), Activity::Resting);
        assert_eq!(s.activity_at(at(5, 0)), Activity::Marching);
        assert_eq!(s.activity_at(at(5, 49)), Activity::Marching);
        assert_eq!(s.activity_at(at(5, 50)), Activity::Halted);
        assert_eq!(s.activity_at(at(6, 0)), Activity::Marching);
        assert_eq!(s.activity_at(at(11, 0)), Activity::Resting);
        assert_eq!(s.activity_at(at(18, 55)), Activity::Halted);
    }

    #[test]
    fn marching_totals_per_day() {
        let s = standard();
        // 6 h window: 6 cycles of 50 min; 4 h window: 4 cycles of 50 min.
        let day = s.marching_between(SimTime::at(1, 0, 0), SimTime::at(2, 0, 0));
        assert_eq!(day, SimDuration::from_mins(500));
    }

    #[test]
    fn truncated_final_cycle_is_cut_at_window_end() {
        let s = MarchSchedule::new(&spec(&[("05:00", "06:30")], 50, 10)).unwrap();
        // 05:00-05:50 march, 05:50-06:00 halt, 06:00-06:30 march (truncated).
        assert_eq!(
            s.marching_between(at(5, 0), at(7, 0)),
            SimDuration::from_mins(80)
        );
        assert_eq!(s.activity_at(at(6, 29)), Activity::Marching);
        assert_eq!(s.next_boundary(at(6, 0)), (at(6, 30), Boundary::WindowEnd));
    }

    #[test]
    fn boundaries_in_order() {
        let s = standard();
        assert_eq!(s.next_boundary(at(4, 0)), (at(5, 0), Boundary::WindowStart));
        assert_eq!(s.next_boundary(at(5, 0)), (at(5, 50), Boundary::HaltStart));
        assert_eq!(s.next_boundary(at(5, 50)), (at(6, 0), Boundary::HaltEnd));
        assert_eq!(
            s.next_boundary(at(10, 55)),
            (at(11, 0), Boundary::WindowEnd)
        );
        assert_eq!(
            s.next_boundary(at(19, 0)),
            (SimTime::at(2, 5, 0), Boundary::WindowStart)
        );
    }

    #[test]
    fn wall_time_skips_halts_and_nights() {
        let s = standard();
        assert_eq!(
            s.wall_time_after_marching(at(5, 0), SimDuration::from_mins(50)),
            at(5, 50)
        );
        assert_eq!(
            s.wall_time_after_marching(at(5, 0), SimDuration::from_mins(51)),
            at(6, 1)
        );
        assert_eq!(
            s.wall_time_after_marching(at(5, 0), SimDuration::from_mins(500)),
            at(18, 50)
        );
        assert_eq!(
            s.wall_time_after_marching(at(5, 0), SimDuration::from_mins(501)),
            SimTime::at(2, 5, 1)
        );
        assert_eq!(
            s.wall_time_after_marching(at(7, 0), SimDuration::ZERO),
            at(7, 0)
        );
    }

    #[test]
    fn invalid_schedules_are_rejected() {
        let bad = [
            spec(&[], 50, 10),
            spec(&[("11:00", "05:00")], 50, 10),
            spec(&[("05:00", "11:00"), ("10:00", "12:00")], 50, 10),
            spec(&[("05:00", "11:00"), ("11:00", "12:00")], 50, 10),
            spec(&[("05:00", "24:00")], 50, 10),
            spec(&[("5:00", "11:00")], 50, 10),
            spec(&[("05:00", "11:00")], 0, 10),
            spec(&[("05:00", "11:00")], 50, 0),
        ];
        for s in &bad {
            assert!(MarchSchedule::new(s).is_err(), "{s:?}");
        }
    }

    proptest! {
        #[test]
        fn marching_between_is_additive(a in 0i64..400_000_000, b in 0i64..400_000_000, c in 0i64..400_000_000) {
            let s = standard();
            let mut v = [a, b, c];
            v.sort_unstable();
            let [a, b, c] = v.map(SimTime);
            prop_assert_eq!(
                s.marching_between(a, b).0 + s.marching_between(b, c).0,
                s.marching_between(a, c).0
            );
        }

        #[test]
        fn wall_time_is_the_earliest_exact_inverse(from in 0i64..200_000_000, need in 1i64..100_000_000) {
            let s = standard();
            let (from, need) = (SimTime(from), SimDuration(need));
            let t = s.wall_time_after_marching(from, need);
            prop_assert_eq!(s.marching_between(from, t), need);
            prop_assert!(s.marching_between(from, SimTime(t.0 - 1)) < need);
        }
    }
}
