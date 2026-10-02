//! Authoritative simulation time: integer milliseconds since the campaign epoch.
//! Implements Technical Design §16.1 (authoritative time) and §22.1 (no wall clock).

use crate::error::CoreError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

const MS_PER_SEC: i64 = 1_000;
const MS_PER_MIN: i64 = 60 * MS_PER_SEC;
const MS_PER_HOUR: i64 = 60 * MS_PER_MIN;
const MS_PER_DAY: i64 = 24 * MS_PER_HOUR;

/// Milliseconds since campaign epoch (Day 1, 00:00).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SimTime(pub i64);

/// Signed span in milliseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SimDuration(pub i64);

impl SimDuration {
    /// The empty span.
    pub const ZERO: Self = Self(0);

    /// A span of `s` seconds. Panics (const overflow) if out of `i64` range.
    pub const fn from_secs(s: i64) -> Self {
        Self(s * MS_PER_SEC)
    }
    /// A span of `m` minutes. Panics (const overflow) if out of `i64` range.
    pub const fn from_mins(m: i64) -> Self {
        Self(m * MS_PER_MIN)
    }
    /// A span of `h` hours. Panics (const overflow) if out of `i64` range.
    pub const fn from_hours(h: i64) -> Self {
        Self(h * MS_PER_HOUR)
    }
    /// A span of `d` days. Panics (const overflow) if out of `i64` range.
    pub const fn from_days(d: i64) -> Self {
        Self(d * MS_PER_DAY)
    }
    /// The span in milliseconds.
    pub const fn as_millis(self) -> i64 {
        self.0
    }
    /// Sum, or `None` on overflow.
    pub fn checked_add(self, o: Self) -> Option<Self> {
        self.0.checked_add(o.0).map(Self)
    }
    /// Difference, or `None` on overflow.
    pub fn checked_sub(self, o: Self) -> Option<Self> {
        self.0.checked_sub(o.0).map(Self)
    }
}

impl SimTime {
    /// Day 1, 00:00.
    pub const EPOCH: Self = Self(0);

    /// One-based day number, by floor division: negative times give day <= 0.
    pub fn day(self) -> i64 {
        self.0.div_euclid(MS_PER_DAY) + 1
    }

    /// Offset into the current day, always in `[0, 24h)`.
    pub fn time_of_day(self) -> SimDuration {
        SimDuration(self.0.rem_euclid(MS_PER_DAY))
    }

    /// Time at `hour:minute` on one-based `day`. Saturates at the `i64` range.
    pub fn at(day: i64, hour: i64, minute: i64) -> Self {
        let ms = (day - 1)
            .saturating_mul(MS_PER_DAY)
            .saturating_add(hour.saturating_mul(MS_PER_HOUR))
            .saturating_add(minute.saturating_mul(MS_PER_MIN));
        Self(ms)
    }

    /// `self + d`, or `None` on overflow.
    pub fn checked_add(self, d: SimDuration) -> Option<Self> {
        self.0.checked_add(d.0).map(Self)
    }

    /// `self - earlier` (negative if `earlier` is later). Saturates at the `i64` range.
    pub fn since(self, earlier: SimTime) -> SimDuration {
        SimDuration(self.0.saturating_sub(earlier.0))
    }
}

/// Formats as `D2 06:40`, or `D2 06:40:15` when seconds are nonzero.
/// Sub-second precision is not shown.
impl fmt::Display for SimTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tod = self.time_of_day().0;
        let (h, m, s) = (
            tod / MS_PER_HOUR,
            tod / MS_PER_MIN % 60,
            tod / MS_PER_SEC % 60,
        );
        write!(f, "D{} {h:02}:{m:02}", self.day())?;
        if s != 0 {
            write!(f, ":{s:02}")?;
        }
        Ok(())
    }
}

impl FromStr for SimTime {
    type Err = CoreError;

    /// Parses the [`Display`](fmt::Display) format.
    fn from_str(s: &str) -> Result<Self, CoreError> {
        let bad = || CoreError::InvalidTime(s.to_owned());
        let (day, clock) = s
            .strip_prefix('D')
            .and_then(|r| r.split_once(' '))
            .ok_or_else(bad)?;
        let day: i64 = day.parse().map_err(|_| bad())?;
        let field = |p: &str, max: i64| -> Option<i64> {
            if p.len() != 2 || !p.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            p.parse().ok().filter(|v| *v <= max)
        };
        let (hour, minute, second) = match clock.split(':').collect::<Vec<_>>()[..] {
            [h, m] => (field(h, 23), field(m, 59), Some(0)),
            [h, m, s] => (field(h, 23), field(m, 59), field(s, 59)),
            _ => return Err(bad()),
        };
        let (Some(hour), Some(minute), Some(second)) = (hour, minute, second) else {
            return Err(bad());
        };
        let base = SimTime::at(day, hour, minute);
        base.checked_add(SimDuration::from_secs(second))
            .ok_or(CoreError::TimeOverflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn display_examples() {
        assert_eq!(SimTime::at(2, 6, 40).to_string(), "D2 06:40");
        let t = SimTime::at(2, 6, 40)
            .checked_add(SimDuration::from_secs(15))
            .unwrap();
        assert_eq!(t.to_string(), "D2 06:40:15");
        assert_eq!(t.to_string().parse::<SimTime>().unwrap(), t);
    }

    #[test]
    fn negative_times_floor() {
        assert_eq!(SimTime(-1).day(), 0);
        assert_eq!(SimTime(-1).time_of_day(), SimDuration(MS_PER_DAY - 1));
        assert_eq!(SimTime::EPOCH.day(), 1);
    }

    #[test]
    fn rejects_garbage() {
        for s in [
            "",
            "2 06:40",
            "D2",
            "D2 6:40",
            "D2 24:00",
            "D2 06:60",
            "D2 06:40:15:01",
            "D2 06:40:",
        ] {
            assert!(s.parse::<SimTime>().is_err(), "{s}");
        }
    }

    proptest! {
        #[test]
        fn display_round_trips(d in -10i64..=10_000, h in 0i64..=23, m in 0i64..=59) {
            let t = SimTime::at(d, h, m);
            prop_assert_eq!(t.to_string().parse::<SimTime>().unwrap(), t);
        }

        #[test]
        fn time_of_day_in_range(ms in -1_000_000_000_000_000i64..=1_000_000_000_000_000) {
            let tod = SimTime(ms).time_of_day();
            prop_assert!(tod >= SimDuration::ZERO && tod < SimDuration::from_hours(24));
        }
    }
}
