//! Template parsing. Implements Execution Plan §4.6.
//!
//! Syntax: `#symbol#` expands a symbol, `#symbol.cap#` capitalizes its first
//! letter, `{path}` / `{path:fmt}` inserts a binding, `##` is a literal `#`,
//! and `{{` / `}}` are literal braces.

use crate::error::TextError;

/// One piece of a parsed template.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    /// Literal text.
    Literal(String),
    /// A nested symbol, optionally capitalized.
    Symbol {
        /// Symbol name.
        name: String,
        /// Whether to capitalize the first letter of the expansion.
        cap: bool,
    },
    /// A binding insertion.
    Binding {
        /// Binding path.
        path: String,
        /// Format name, if any.
        fmt: Option<String>,
    },
}

/// A parsed template string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    /// The segments, in order.
    pub segments: Vec<Segment>,
}

impl Template {
    /// Parses `src`; malformed markup is [`TextError::BadTemplate`].
    pub fn parse(src: &str) -> Result<Template, TextError> {
        let bad = |reason: &str| TextError::BadTemplate {
            template: src.to_owned(),
            reason: reason.to_owned(),
        };
        let mut segments = Vec::new();
        let mut lit = String::new();
        let mut chars = src.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '#' if chars.peek() == Some(&'#') => {
                    chars.next();
                    lit.push('#');
                }
                '{' if chars.peek() == Some(&'{') => {
                    chars.next();
                    lit.push('{');
                }
                '}' if chars.peek() == Some(&'}') => {
                    chars.next();
                    lit.push('}');
                }
                '#' | '{' => {
                    let close = if c == '#' { '#' } else { '}' };
                    let mut inner = String::new();
                    let mut closed = false;
                    for x in chars.by_ref() {
                        if x == close {
                            closed = true;
                            break;
                        }
                        inner.push(x);
                    }
                    if !closed || inner.contains(['#', '{', '}']) {
                        return Err(bad("unterminated or nested markup"));
                    }
                    if !lit.is_empty() {
                        segments.push(Segment::Literal(std::mem::take(&mut lit)));
                    }
                    segments.push(if c == '#' {
                        symbol_segment(&inner).ok_or_else(|| bad("empty symbol name"))?
                    } else {
                        binding_segment(&inner).ok_or_else(|| bad("empty binding path"))?
                    });
                }
                '}' => return Err(bad("unmatched `}`")),
                _ => lit.push(c),
            }
        }
        if !lit.is_empty() {
            segments.push(Segment::Literal(lit));
        }
        Ok(Template { segments })
    }
}

fn symbol_segment(inner: &str) -> Option<Segment> {
    let (name, cap) = inner
        .strip_suffix(".cap")
        .map_or((inner, false), |n| (n, true));
    (!name.is_empty()).then(|| Segment::Symbol {
        name: name.to_owned(),
        cap,
    })
}

fn binding_segment(inner: &str) -> Option<Segment> {
    let (path, fmt) = match inner.split_once(':') {
        Some((p, f)) => (p, Some(f.to_owned())),
        None => (inner, None),
    };
    (!path.is_empty()).then(|| Segment::Binding {
        path: path.to_owned(),
        fmt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lit(s: &str) -> Segment {
        Segment::Literal(s.to_owned())
    }

    #[test]
    fn parses_all_segment_kinds() {
        let t =
            Template::parse("Out of #place.camp# at {time:hm}. #departed.detail.cap#{n}").unwrap();
        assert_eq!(
            t.segments,
            vec![
                lit("Out of "),
                Segment::Symbol {
                    name: "place.camp".into(),
                    cap: false
                },
                lit(" at "),
                Segment::Binding {
                    path: "time".into(),
                    fmt: Some("hm".into())
                },
                lit(". "),
                Segment::Symbol {
                    name: "departed.detail".into(),
                    cap: true
                },
                Segment::Binding {
                    path: "n".into(),
                    fmt: None
                },
            ]
        );
    }

    #[test]
    fn escapes() {
        let t = Template::parse("## {{x}} ##1").unwrap();
        assert_eq!(t.segments, vec![lit("# {x} #1")]);
    }

    #[test]
    fn escape_next_to_markup() {
        let t = Template::parse("{{{a}}}").unwrap();
        assert_eq!(
            t.segments,
            vec![
                lit("{"),
                Segment::Binding {
                    path: "a".into(),
                    fmt: None
                },
                lit("}"),
            ]
        );
    }

    #[test]
    fn malformed_is_rejected() {
        for s in [
            "#open", "{open", "close}", "##x#", "{}", "#.cap#", "{a:b", "#a{b}#",
        ] {
            assert!(
                matches!(Template::parse(s), Err(TextError::BadTemplate { .. })),
                "{s:?}"
            );
        }
    }
}
