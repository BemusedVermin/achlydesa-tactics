//! Static checks over a grammar. Implements Execution Plan §4.6.
//!
//! Roots are the symbols the simulation expands directly; each root has a
//! binding set. Nested symbols inherit the binding set of every root that
//! reaches them, so a shared symbol must be valid for each.

use crate::format::FORMATS;
use crate::grammar::Grammar;
use crate::template::Segment;
use crate::voice::VoiceSet;
use std::collections::{BTreeMap, BTreeSet};

/// Which bindings each root symbol makes available.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BindingSchema {
    /// Root symbol to the binding paths available when it is expanded.
    pub roots: BTreeMap<String, BTreeSet<String>>,
}

/// A problem found by [`lint`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LintIssue {
    /// `symbol` references `missing`, which is not defined (error).
    UnknownSymbol {
        /// Symbol containing the reference (or the root itself, if the root is undefined).
        symbol: String,
        /// The undefined symbol.
        missing: String,
    },
    /// `symbol`, reached from `root`, uses a binding the root does not provide (error).
    UnknownBinding {
        /// Symbol using the binding.
        symbol: String,
        /// Root whose binding set lacks it.
        root: String,
        /// The binding path.
        path: String,
    },
    /// No alternative of `symbol` is unconditional (error).
    NoUnconditionalFallback {
        /// The symbol.
        symbol: String,
    },
    /// `symbol` is not reachable from any root (error).
    UnreachableSymbol {
        /// The symbol.
        symbol: String,
    },
    /// `symbol` uses a format name that does not exist (error).
    BadFormat {
        /// Symbol using the format.
        symbol: String,
        /// The format name.
        fmt: String,
    },
    /// A grammar tag that no voice defines (warning).
    UnknownVoiceTag {
        /// The tag.
        tag: String,
    },
}

impl LintIssue {
    /// Whether this issue is an error; only [`LintIssue::UnknownVoiceTag`] is a warning.
    pub fn is_error(&self) -> bool {
        !matches!(self, LintIssue::UnknownVoiceTag { .. })
    }
}

/// Symbols directly referenced by `symbol`'s alternatives.
fn references(grammar: &Grammar, symbol: &str) -> BTreeSet<String> {
    grammar
        .alternatives(symbol)
        .into_iter()
        .flatten()
        .flat_map(|a| &a.template.segments)
        .filter_map(|s| match s {
            Segment::Symbol { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect()
}

/// Defined symbols reachable from `root`, including itself.
fn reachable(grammar: &Grammar, root: &str) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut stack = vec![root.to_owned()];
    while let Some(s) = stack.pop() {
        if grammar.alternatives(&s).is_some() && seen.insert(s.clone()) {
            stack.extend(references(grammar, &s));
        }
    }
    seen
}

/// Binding paths used by `symbol`, in templates and conditions.
fn bindings_used(grammar: &Grammar, symbol: &str) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    for alt in grammar.alternatives(symbol).into_iter().flatten() {
        paths.extend(alt.when.iter().map(|c| c.path.clone()));
        for seg in &alt.template.segments {
            if let Segment::Binding { path, .. } = seg {
                paths.insert(path.clone());
            }
        }
    }
    paths
}

fn lint_symbol_local(grammar: &Grammar, symbol: &str, out: &mut BTreeSet<LintIssue>) {
    let alts = grammar.alternatives(symbol).unwrap_or_default();
    if !alts.iter().any(|a| a.when.is_empty()) {
        out.insert(LintIssue::NoUnconditionalFallback {
            symbol: symbol.to_owned(),
        });
    }
    for r in references(grammar, symbol) {
        if grammar.alternatives(&r).is_none() {
            out.insert(LintIssue::UnknownSymbol {
                symbol: symbol.to_owned(),
                missing: r,
            });
        }
    }
    for seg in alts.iter().flat_map(|a| &a.template.segments) {
        if let Segment::Binding { fmt: Some(f), .. } = seg
            && !FORMATS.contains(&f.as_str())
        {
            out.insert(LintIssue::BadFormat {
                symbol: symbol.to_owned(),
                fmt: f.clone(),
            });
        }
    }
}

/// Checks `grammar` against `voices` and the root binding `schema`.
///
/// Returns issues sorted and deduplicated. Every issue is an error except
/// [`LintIssue::UnknownVoiceTag`] (see [`LintIssue::is_error`]).
pub fn lint(grammar: &Grammar, voices: &VoiceSet, schema: &BindingSchema) -> Vec<LintIssue> {
    let mut out = BTreeSet::new();
    for symbol in grammar.symbol_names() {
        lint_symbol_local(grammar, symbol, &mut out);
    }

    let mut covered = BTreeSet::new();
    for (root, available) in &schema.roots {
        if grammar.alternatives(root).is_none() {
            out.insert(LintIssue::UnknownSymbol {
                symbol: root.clone(),
                missing: root.clone(),
            });
            continue;
        }
        for symbol in reachable(grammar, root) {
            for path in bindings_used(grammar, &symbol).difference(available) {
                out.insert(LintIssue::UnknownBinding {
                    symbol: symbol.clone(),
                    root: root.clone(),
                    path: path.clone(),
                });
            }
            covered.insert(symbol);
        }
    }
    for symbol in grammar.symbol_names().filter(|s| !covered.contains(*s)) {
        out.insert(LintIssue::UnreachableSymbol {
            symbol: symbol.to_owned(),
        });
    }

    let known: BTreeSet<&String> = voices.voices.values().flat_map(|v| v.tags.keys()).collect();
    for symbol in grammar.symbol_names() {
        for tag in grammar
            .alternatives(symbol)
            .into_iter()
            .flatten()
            .flat_map(|a| &a.tags)
            .filter(|t| !known.contains(t))
        {
            out.insert(LintIssue::UnknownVoiceTag { tag: tag.clone() });
        }
    }
    out.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema(roots: &[(&str, &[&str])]) -> BindingSchema {
        BindingSchema {
            roots: roots
                .iter()
                .map(|(r, b)| ((*r).to_owned(), b.iter().map(|s| (*s).to_owned()).collect()))
                .collect(),
        }
    }

    fn voices() -> VoiceSet {
        VoiceSet::from_ron(r#"(voices: {"ione": (tags: {"terse": 2000})})"#).unwrap()
    }

    fn s(x: &str) -> String {
        x.to_owned()
    }

    #[test]
    fn clean_grammar_has_no_issues() {
        let g = Grammar::from_ron(
            r##"(symbols: {
              "a": [(text: "{t:hm} #b#", weight: 1, tags: ["terse"])],
              "b": [(text: "x", weight: 1), (text: "y", weight: 1, when: ["t > 1"])],
            })"##,
        )
        .unwrap();
        assert_eq!(lint(&g, &voices(), &schema(&[("a", &["t"])])), vec![]);
    }

    #[test]
    fn catches_each_issue_kind() {
        let g = Grammar::from_ron(
            r##"(symbols: {
              "a": [(text: "#b# #ghost# {nope} {t:zz}", weight: 1, tags: ["loud"])],
              "b": [(text: "x {t}", weight: 1, when: ["secret == 1"])],
              "orphan": [(text: "z", weight: 1)],
            })"##,
        )
        .unwrap();
        let issues = lint(&g, &voices(), &schema(&[("a", &["t"]), ("missing", &[])]));
        let expect = [
            LintIssue::UnknownSymbol {
                symbol: s("a"),
                missing: s("ghost"),
            },
            LintIssue::UnknownSymbol {
                symbol: s("missing"),
                missing: s("missing"),
            },
            LintIssue::UnknownBinding {
                symbol: s("a"),
                root: s("a"),
                path: s("nope"),
            },
            LintIssue::UnknownBinding {
                symbol: s("b"),
                root: s("a"),
                path: s("secret"),
            },
            LintIssue::NoUnconditionalFallback { symbol: s("b") },
            LintIssue::UnreachableSymbol {
                symbol: s("orphan"),
            },
            LintIssue::BadFormat {
                symbol: s("a"),
                fmt: s("zz"),
            },
            LintIssue::UnknownVoiceTag { tag: s("loud") },
        ];
        for e in &expect {
            assert!(issues.contains(e), "missing {e:?} in {issues:?}");
        }
        assert_eq!(issues.len(), expect.len());
        let errors = issues.iter().filter(|i| i.is_error()).count();
        assert_eq!(errors, expect.len() - 1);
    }

    #[test]
    fn shared_symbol_must_fit_every_root() {
        let g = Grammar::from_ron(
            r##"(symbols: {
              "r1": [(text: "#shared#", weight: 1)],
              "r2": [(text: "#shared#", weight: 1)],
              "shared": [(text: "{water}", weight: 1)],
            })"##,
        )
        .unwrap();
        let issues = lint(&g, &voices(), &schema(&[("r1", &["water"]), ("r2", &[])]));
        assert_eq!(
            issues,
            vec![LintIssue::UnknownBinding {
                symbol: s("shared"),
                root: s("r2"),
                path: s("water")
            }]
        );
    }

    #[test]
    fn cycles_terminate() {
        let g = Grammar::from_ron(r##"(symbols: {"a": [(text: "#a#", weight: 1)]})"##).unwrap();
        assert_eq!(lint(&g, &voices(), &schema(&[("a", &[])])), vec![]);
    }
}
