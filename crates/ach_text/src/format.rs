//! Binding values and their display formats. Implements Execution Plan §4.6.
//!
//! All arithmetic is integer; rounding is half up (toward positive infinity
//! on exact ties).

use crate::error::TextError;
use ach_core::{SimDuration, SimTime};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A fact a template can insert or a condition can test.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Value {
    /// A plain integer.
    Int(i64),
    /// Free text.
    Text(String),
    /// A distance in metres.
    DistanceM(i64),
    /// A point in simulation time.
    Time(SimTime),
    /// A span of simulation time.
    Duration(SimDuration),
    /// A ratio in per-mille.
    PerMille(i32),
    /// A quantity in milli-units.
    Milli(i64),
}

impl Value {
    /// The numeric reading used by conditions; `None` for [`Value::Text`].
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(v) | Value::DistanceM(v) | Value::Milli(v) => Some(*v),
            Value::Time(t) => Some(t.0),
            Value::Duration(d) => Some(d.0),
            Value::PerMille(p) => Some(i64::from(*p)),
            Value::Text(_) => None,
        }
    }

    fn kind(&self) -> &'static str {
        match self {
            Value::Int(_) => "Int",
            Value::Text(_) => "Text",
            Value::DistanceM(_) => "DistanceM",
            Value::Time(_) => "Time",
            Value::Duration(_) => "Duration",
            Value::PerMille(_) => "PerMille",
            Value::Milli(_) => "Milli",
        }
    }
}

/// Source of binding values, looked up by dotted path.
pub trait Bindings {
    /// The value at `path`, if bound.
    fn get(&self, path: &str) -> Option<Value>;
}

/// A [`Bindings`] backed by a sorted map.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MapBindings(pub BTreeMap<String, Value>);

impl Bindings for MapBindings {
    fn get(&self, path: &str) -> Option<Value> {
        self.0.get(path).cloned()
    }
}

/// Every format name accepted in `{path:fmt}`.
pub const FORMATS: &[&str] = &[
    "km1", "km", "m", "hm", "day", "dt", "dur", "pct", "L", "n", "words",
];

const WORDS: [&str; 21] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
];

const MS_PER_MIN: i64 = 60_000;

/// `value / divisor` rounded half up. `divisor` must be at least 2.
fn round_div(value: i64, divisor: i64) -> i64 {
    let r = (i128::from(value) + i128::from(divisor / 2)).div_euclid(i128::from(divisor));
    i64::try_from(r).expect("invariant: quotient by a divisor >= 2 fits i64")
}

fn sign(v: i64) -> &'static str {
    if v < 0 { "-" } else { "" }
}

/// `1412` becomes `1,412`.
fn group_thousands(v: i64) -> String {
    let digits = v.unsigned_abs().to_string();
    let mut out = String::from(sign(v));
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

fn clock(t: SimTime) -> String {
    let minutes = t.time_of_day().0 / MS_PER_MIN;
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

fn dt_text(t: SimTime) -> String {
    format!("D{} {}", t.day(), clock(t))
}

fn duration_text(d: SimDuration) -> String {
    let total = round_div(d.0, MS_PER_MIN);
    let (h, m) = (total.unsigned_abs() / 60, total.unsigned_abs() % 60);
    let s = sign(total);
    match (h, m) {
        (0, m) => format!("{s}{m} min"),
        (h, 0) => format!("{s}{h} h"),
        (h, m) => format!("{s}{h} h {m} min"),
    }
}

fn km1_text(m: i64) -> String {
    let tenths = round_div(m, 100);
    let a = tenths.unsigned_abs();
    format!("{}{}.{} km", sign(tenths), a / 10, a % 10)
}

fn pct_text(p: i32) -> String {
    format!("{}%", round_div(i64::from(p), 10))
}

fn mismatch(fmt: &str, v: &Value) -> TextError {
    TextError::FormatMismatch {
        fmt: fmt.to_owned(),
        found: v.kind(),
    }
}

fn default_text(value: &Value) -> String {
    match value {
        Value::Int(v) => v.to_string(),
        Value::Text(s) => s.clone(),
        Value::DistanceM(m) if m.unsigned_abs() < 1000 => format!("{m} m"),
        Value::DistanceM(m) => km1_text(*m),
        Value::Time(t) => dt_text(*t),
        Value::Duration(d) => duration_text(*d),
        Value::PerMille(p) => pct_text(*p),
        Value::Milli(v) => round_div(*v, 1000).to_string(),
    }
}

/// Renders `value` with `fmt`, or with its default form when `fmt` is `None`.
///
/// Defaults: `Int` plain digits; `Text` as is; `DistanceM` `m` below 1000 m,
/// else `km1`; `Time` `dt`; `Duration` `dur`; `PerMille` `pct`; `Milli` whole
/// units, rounded.
pub fn render(value: &Value, fmt: Option<&str>) -> Result<String, TextError> {
    let Some(fmt) = fmt else {
        return Ok(default_text(value));
    };
    match (fmt, value) {
        ("km1", Value::DistanceM(m)) => Ok(km1_text(*m)),
        ("km", Value::DistanceM(m)) => Ok(format!("{} km", round_div(*m, 1000))),
        ("m", Value::DistanceM(m)) => Ok(format!("{m} m")),
        ("hm", Value::Time(t)) => Ok(clock(*t)),
        ("day", Value::Time(t)) => Ok(format!("Day {}", t.day())),
        ("dt", Value::Time(t)) => Ok(dt_text(*t)),
        ("dur", Value::Duration(d)) => Ok(duration_text(*d)),
        ("pct", Value::PerMille(p)) => Ok(pct_text(*p)),
        ("L", Value::Milli(v)) => Ok(format!("{} L", round_div(*v, 1000))),
        ("n", Value::Int(v)) => Ok(group_thousands(*v)),
        ("words", Value::Int(v)) => Ok(usize::try_from(*v)
            .ok()
            .and_then(|i| WORDS.get(i))
            .map_or_else(|| v.to_string(), |w| (*w).to_owned())),
        (f, _) if FORMATS.contains(&f) => Err(mismatch(f, value)),
        (f, _) => Err(TextError::UnknownFormat(f.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(v: Value, f: &str) -> String {
        render(&v, Some(f)).unwrap()
    }

    #[test]
    fn format_table_known_answers() {
        assert_eq!(r(Value::DistanceM(12_449), "km1"), "12.4 km");
        assert_eq!(r(Value::DistanceM(12_450), "km1"), "12.5 km");
        assert_eq!(r(Value::DistanceM(12_500), "km"), "13 km");
        assert_eq!(r(Value::DistanceM(12_499), "km"), "12 km");
        assert_eq!(r(Value::DistanceM(340), "m"), "340 m");
        let t = Value::Time(SimTime::at(2, 6, 40));
        assert_eq!(r(t.clone(), "hm"), "06:40");
        assert_eq!(r(t.clone(), "day"), "Day 2");
        assert_eq!(r(t, "dt"), "D2 06:40");
        let d = Value::Duration(SimDuration::from_mins(200));
        assert_eq!(r(d, "dur"), "3 h 20 min");
        assert_eq!(r(Value::PerMille(612), "pct"), "61%");
        assert_eq!(r(Value::PerMille(615), "pct"), "62%");
        assert_eq!(r(Value::Milli(340_400), "L"), "340 L");
        assert_eq!(r(Value::Milli(340_500), "L"), "341 L");
        assert_eq!(r(Value::Int(1412), "n"), "1,412");
        assert_eq!(r(Value::Int(1_234_567), "n"), "1,234,567");
        assert_eq!(r(Value::Int(999), "n"), "999");
        assert_eq!(r(Value::Int(-1412), "n"), "-1,412");
        assert_eq!(r(Value::Int(12), "words"), "twelve");
        assert_eq!(r(Value::Int(0), "words"), "zero");
        assert_eq!(r(Value::Int(20), "words"), "twenty");
        assert_eq!(r(Value::Int(21), "words"), "21");
        assert_eq!(r(Value::Int(-1), "words"), "-1");
    }

    #[test]
    fn duration_edges() {
        let d = |m| Value::Duration(SimDuration::from_mins(m));
        assert_eq!(r(d(180), "dur"), "3 h");
        assert_eq!(r(d(20), "dur"), "20 min");
        assert_eq!(r(d(0), "dur"), "0 min");
        assert_eq!(r(d(-80), "dur"), "-1 h 20 min");
        // 30 s rounds up to a whole minute
        assert_eq!(
            r(Value::Duration(SimDuration::from_secs(30)), "dur"),
            "1 min"
        );
    }

    #[test]
    fn small_distances_and_signs() {
        assert_eq!(r(Value::DistanceM(40), "km1"), "0.0 km");
        assert_eq!(r(Value::DistanceM(50), "km1"), "0.1 km");
        assert_eq!(r(Value::DistanceM(-12_449), "km1"), "-12.4 km");
    }

    #[test]
    fn defaults() {
        let d = |v: Value| render(&v, None).unwrap();
        assert_eq!(d(Value::Int(7)), "7");
        assert_eq!(d(Value::Text("Ione".into())), "Ione");
        assert_eq!(d(Value::DistanceM(340)), "340 m");
        assert_eq!(d(Value::DistanceM(12_449)), "12.4 km");
        assert_eq!(d(Value::Time(SimTime::at(2, 6, 40))), "D2 06:40");
        assert_eq!(
            d(Value::Duration(SimDuration::from_mins(200))),
            "3 h 20 min"
        );
        assert_eq!(d(Value::PerMille(612)), "61%");
        assert_eq!(d(Value::Milli(340_400)), "340");
    }

    #[test]
    fn mismatch_and_unknown() {
        assert!(matches!(
            render(&Value::Int(1), Some("km")),
            Err(TextError::FormatMismatch { .. })
        ));
        assert!(matches!(
            render(&Value::Text("x".into()), Some("n")),
            Err(TextError::FormatMismatch { .. })
        ));
        assert!(matches!(
            render(&Value::Int(1), Some("nope")),
            Err(TextError::UnknownFormat(_))
        ));
    }

    #[test]
    fn map_bindings_lookup() {
        let b = MapBindings(BTreeMap::from([("a".to_owned(), Value::Int(1))]));
        assert_eq!(b.get("a"), Some(Value::Int(1)));
        assert_eq!(b.get("b"), None);
    }
}
