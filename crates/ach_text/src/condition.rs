//! Alternative conditions: `<path> <op> <literal>`. Implements Execution Plan §4.6.

use crate::error::TextError;
use crate::format::{Bindings, Value};
use std::cmp::Ordering;

/// Comparison operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `==`
    Eq,
    /// `!=`
    Ne,
}

impl Op {
    fn parse(s: &str) -> Option<Op> {
        Some(match s {
            "<" => Op::Lt,
            "<=" => Op::Le,
            ">" => Op::Gt,
            ">=" => Op::Ge,
            "==" => Op::Eq,
            "!=" => Op::Ne,
            _ => return None,
        })
    }

    fn holds(self, ord: Ordering) -> bool {
        match self {
            Op::Lt => ord.is_lt(),
            Op::Le => ord.is_le(),
            Op::Gt => ord.is_gt(),
            Op::Ge => ord.is_ge(),
            Op::Eq => ord.is_eq(),
            Op::Ne => ord.is_ne(),
        }
    }
}

/// The right-hand side of a condition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    /// An integer.
    Int(i64),
    /// A double-quoted string.
    Text(String),
}

/// A parsed condition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Condition {
    /// Binding path tested.
    pub path: String,
    /// Operator.
    pub op: Op,
    /// Literal compared against.
    pub literal: Literal,
}

impl Condition {
    /// Parses `<path> <op> <literal>`.
    ///
    /// Text literals only combine with `==` and `!=`.
    pub fn parse(src: &str) -> Result<Condition, TextError> {
        let bad = |reason: &str| TextError::BadCondition {
            condition: src.to_owned(),
            reason: reason.to_owned(),
        };
        let mut parts = src.trim().splitn(3, char::is_whitespace);
        let (Some(path), Some(op), Some(lit)) = (parts.next(), parts.next(), parts.next()) else {
            return Err(bad("expected `<path> <op> <literal>`"));
        };
        let op = Op::parse(op).ok_or_else(|| bad("unknown operator"))?;
        let lit = lit.trim();
        let literal = if let Some(inner) = lit.strip_prefix('"') {
            let text = inner
                .strip_suffix('"')
                .filter(|t| !t.contains('"'))
                .ok_or_else(|| bad("unterminated or malformed string literal"))?;
            if !matches!(op, Op::Eq | Op::Ne) {
                return Err(bad("text literals support only == and !="));
            }
            Literal::Text(text.to_owned())
        } else {
            Literal::Int(lit.parse().map_err(|_| bad("literal is not an integer"))?)
        };
        Ok(Condition {
            path: path.to_owned(),
            op,
            literal,
        })
    }

    /// Whether the condition holds. A missing binding, or a value whose type
    /// does not match the literal, makes it false.
    pub fn eval(&self, bindings: &dyn Bindings) -> bool {
        let Some(value) = bindings.get(&self.path) else {
            return false;
        };
        match (&self.literal, &value) {
            (Literal::Text(lit), Value::Text(v)) => self.op.holds(v.as_str().cmp(lit.as_str())),
            (Literal::Int(lit), v) => v.as_i64().is_some_and(|n| self.op.holds(n.cmp(lit))),
            (Literal::Text(_), _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::MapBindings;
    use std::collections::BTreeMap;

    fn b() -> MapBindings {
        MapBindings(BTreeMap::from([
            ("day".to_owned(), Value::Int(2)),
            ("who".to_owned(), Value::Text("ione".into())),
            ("water".to_owned(), Value::Milli(-5)),
        ]))
    }

    fn ev(s: &str) -> bool {
        Condition::parse(s).unwrap().eval(&b())
    }

    #[test]
    fn parses_and_evaluates_every_op() {
        assert!(ev("day == 2") && !ev("day == 3"));
        assert!(ev("day != 3") && !ev("day != 2"));
        assert!(ev("day < 3") && !ev("day < 2"));
        assert!(ev("day <= 2") && !ev("day <= 1"));
        assert!(ev("day > 1") && !ev("day > 2"));
        assert!(ev("day >= 2") && !ev("day >= 3"));
        assert!(ev("water < 0") && ev("water == -5"));
        assert!(ev(r#"who == "ione""#) && ev(r#"who != "kest""#));
        assert!(!ev(r#"who == "kest""#));
    }

    #[test]
    fn mismatches_and_missing_are_false() {
        assert!(!ev("missing == 1"));
        assert!(!ev("who == 1"));
        assert!(!ev(r#"day == "2""#));
    }

    #[test]
    fn spaces_inside_strings_survive() {
        let c = Condition::parse(r#"who == "a b""#).unwrap();
        assert_eq!(c.literal, Literal::Text("a b".into()));
    }

    #[test]
    fn malformed_is_rejected() {
        for s in [
            "",
            "day",
            "day ==",
            "day = 2",
            "day == two",
            r#"who == "x"#,
            r#"who < "x""#,
            r#"who == "a"b""#,
            "day == 2.5",
        ] {
            assert!(
                matches!(Condition::parse(s), Err(TextError::BadCondition { .. })),
                "{s:?}"
            );
        }
    }
}
