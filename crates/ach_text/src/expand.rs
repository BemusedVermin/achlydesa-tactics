//! Symbol expansion with voice weighting and repeat avoidance.
//! Implements Execution Plan §4.6.

use crate::error::TextError;
use crate::format::{Bindings, render};
use crate::grammar::Grammar;
use crate::template::Segment;
use crate::voice::VoiceProfile;
use ach_core::RngCursor;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

/// Maximum symbol nesting depth.
pub const MAX_DEPTH: usize = 16;
/// Maximum output length in characters.
pub const MAX_CHARS: usize = 2_000;

/// Which alternatives each speaker used recently, per symbol.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpansionHistory {
    /// (speaker, symbol) to recent alternative indices, oldest first.
    recent: BTreeMap<(String, String), VecDeque<u16>>,
}

impl ExpansionHistory {
    fn recent(&self, speaker: &str, symbol: &str) -> Vec<u16> {
        self.recent
            .get(&(speaker.to_owned(), symbol.to_owned()))
            .map(|q| q.iter().copied().collect())
            .unwrap_or_default()
    }

    fn record(&mut self, speaker: &VoiceProfile, symbol: &str, index: u16) {
        let cap = usize::from(speaker.avoid_repeat);
        let q = self
            .recent
            .entry((speaker.id.clone(), symbol.to_owned()))
            .or_default();
        q.push_back(index);
        while q.len() > cap {
            q.pop_front();
        }
    }
}

/// Expands symbols of a [`Grammar`], updating a speaker history.
pub struct Expander<'g> {
    /// The grammar to expand from.
    pub grammar: &'g Grammar,
    /// Repeat-avoidance state, updated by every choice.
    pub history: &'g mut ExpansionHistory,
}

struct Ctx<'a> {
    speaker: &'a VoiceProfile,
    bindings: &'a dyn Bindings,
}

impl Expander<'_> {
    /// Expands `symbol` for `speaker`.
    ///
    /// Per symbol occurrence: keep alternatives with positive weight whose
    /// conditions hold; drop the speaker's recent picks unless that leaves
    /// none; choose by effective voice weight with one RNG draw; record the
    /// pick; recurse. Limits: depth [`MAX_DEPTH`], length [`MAX_CHARS`].
    pub fn expand(
        &mut self,
        symbol: &str,
        speaker: &VoiceProfile,
        bindings: &dyn Bindings,
        rng: &mut RngCursor,
    ) -> Result<String, TextError> {
        let ctx = Ctx { speaker, bindings };
        self.expand_symbol(symbol, &ctx, rng, 0)
    }

    fn expand_symbol(
        &mut self,
        symbol: &str,
        ctx: &Ctx<'_>,
        rng: &mut RngCursor,
        depth: usize,
    ) -> Result<String, TextError> {
        if depth > MAX_DEPTH {
            return Err(TextError::TooDeep);
        }
        let grammar = self.grammar;
        let alts = grammar
            .alternatives(symbol)
            .ok_or_else(|| TextError::UnknownSymbol(symbol.to_owned()))?;
        let eligible: Vec<usize> = (0..alts.len())
            .filter(|&i| alts[i].weight > 0 && alts[i].when.iter().all(|c| c.eval(ctx.bindings)))
            .collect();
        let recent = self.history.recent(&ctx.speaker.id, symbol);
        let fresh: Vec<usize> = eligible
            .iter()
            .copied()
            .filter(|&i| !recent.iter().any(|&r| usize::from(r) == i))
            .collect();
        let candidates = if fresh.is_empty() { eligible } else { fresh };
        let weights: Vec<u32> = candidates
            .iter()
            .map(|&i| ctx.speaker.effective_weight(alts[i].weight, &alts[i].tags))
            .collect();
        let pick = rng
            .choose_weighted(&weights)
            .ok_or_else(|| TextError::NoAlternative {
                symbol: symbol.to_owned(),
            })?;
        let index = candidates[pick];
        self.history.record(
            ctx.speaker,
            symbol,
            u16::try_from(index).expect("invariant: alternative count checked at load"),
        );
        let mut out = String::new();
        for seg in &alts[index].template.segments {
            match seg {
                Segment::Literal(s) => out.push_str(s),
                Segment::Binding { path, fmt } => {
                    let v = ctx
                        .bindings
                        .get(path)
                        .ok_or_else(|| TextError::UnknownBinding(path.clone()))?;
                    out.push_str(&render(&v, fmt.as_deref())?);
                }
                Segment::Symbol { name, cap } => {
                    let inner = self.expand_symbol(name, ctx, rng, depth + 1)?;
                    out.push_str(&if *cap { capitalize(&inner) } else { inner });
                }
            }
            if out.chars().count() > MAX_CHARS {
                return Err(TextError::TooLong);
            }
        }
        Ok(out)
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars
        .next()
        .map(|c| c.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{MapBindings, Value};
    use ach_core::{Domain, Seed, StreamKey};
    use proptest::prelude::*;

    fn cursor(seed: u64) -> RngCursor {
        RngCursor::new(
            Seed(seed),
            StreamKey {
                domain: Domain::Text,
                subject: 1,
            },
            0,
        )
    }

    fn voice(avoid: u8) -> VoiceProfile {
        VoiceProfile {
            id: "ione".into(),
            avoid_repeat: avoid,
            ..VoiceProfile::default()
        }
    }

    fn six() -> Grammar {
        Grammar::from_ron(
            r#"(symbols: {"s": [
              (text: "a", weight: 1000), (text: "b", weight: 1000), (text: "c", weight: 1000),
              (text: "d", weight: 1000), (text: "e", weight: 1000), (text: "f", weight: 1000),
            ]})"#,
        )
        .unwrap()
    }

    fn run(
        g: &Grammar,
        sym: &str,
        v: &VoiceProfile,
        b: &MapBindings,
        seed: u64,
    ) -> Result<String, TextError> {
        let mut h = ExpansionHistory::default();
        Expander {
            grammar: g,
            history: &mut h,
        }
        .expand(sym, v, b, &mut cursor(seed))
    }

    #[test]
    fn deterministic_and_seed_sensitive() {
        let g = six();
        let b = MapBindings::default();
        let v = voice(0);
        assert_eq!(run(&g, "s", &v, &b, 9), run(&g, "s", &v, &b, 9));
        let distinct: std::collections::BTreeSet<String> =
            (0..100).map(|s| run(&g, "s", &v, &b, s).unwrap()).collect();
        assert!(distinct.len() >= 5, "{}", distinct.len());
    }

    #[test]
    fn nesting_bindings_cap_and_conditions() {
        let g = Grammar::from_ron(
            r##"(symbols: {
              "top": [(text: "#inner.cap# at {t:hm}, {n:words} {who}. ## {{ok}}", weight: 1)],
              "inner": [
                (text: "night", weight: 1, when: ["day == 2"]),
                (text: "day", weight: 1000, when: ["day == 1"]),
                (text: "never", weight: 0),
              ],
            })"##,
        )
        .unwrap();
        let b = MapBindings(BTreeMap::from([
            ("day".into(), Value::Int(2)),
            ("t".into(), Value::Time(ach_core::SimTime::at(1, 6, 40))),
            ("n".into(), Value::Int(12)),
            ("who".into(), Value::Text("walkers".into())),
        ]));
        assert_eq!(
            run(&g, "top", &voice(0), &b, 0).unwrap(),
            "Night at 06:40, twelve walkers. # {ok}"
        );
    }

    #[test]
    fn voice_tags_bias_choice() {
        let g = Grammar::from_ron(
            r#"(symbols: {"s": [
              (text: "terse", weight: 1000, tags: ["terse"]),
              (text: "long", weight: 1000, tags: ["long"]),
            ]})"#,
        )
        .unwrap();
        let mut v = voice(0);
        v.tags.insert("terse".into(), 20_000);
        v.tags.insert("long".into(), 1);
        let b = MapBindings::default();
        let terse = (0..200)
            .filter(|&s| run(&g, "s", &v, &b, s).unwrap() == "terse")
            .count();
        assert!(terse > 190, "{terse}");
    }

    #[test]
    fn errors() {
        let g = Grammar::from_ron(
            r##"(symbols: {
              "loop": [(text: "x#loop#", weight: 1)],
              "none": [(text: "x", weight: 1, when: ["a == 1"])],
              "bind": [(text: "{missing}", weight: 1)],
              "fmt": [(text: "{a:km}", weight: 1)],
              "ref": [(text: "#ghost#", weight: 1)],
              "wide": [(text: "#wide2##wide2##wide2##wide2##wide2#", weight: 1)],
              "wide2": [(text: "#w3##w3##w3##w3##w3##w3##w3#", weight: 1)],
              "w3": [(text: "#w4##w4##w4##w4##w4##w4##w4#", weight: 1)],
              "w4": [(text: "0123456789", weight: 1)],
            })"##,
        )
        .unwrap();
        let b = MapBindings(BTreeMap::from([("a".into(), Value::Int(0))]));
        let v = voice(0);
        let e = |s: &str| run(&g, s, &v, &b, 0).unwrap_err();
        assert_eq!(e("loop"), TextError::TooDeep);
        assert_eq!(
            e("none"),
            TextError::NoAlternative {
                symbol: "none".into()
            }
        );
        assert_eq!(e("bind"), TextError::UnknownBinding("missing".into()));
        assert!(matches!(e("fmt"), TextError::FormatMismatch { .. }));
        assert_eq!(e("ref"), TextError::UnknownSymbol("ghost".into()));
        assert_eq!(e("wide"), TextError::TooLong);
        assert_eq!(e("absent"), TextError::UnknownSymbol("absent".into()));
    }

    #[test]
    fn exhausted_history_falls_back_to_all() {
        let g = Grammar::from_ron(r#"(symbols: {"s": [(text: "only", weight: 1)]})"#).unwrap();
        let mut h = ExpansionHistory::default();
        let v = voice(3);
        let b = MapBindings::default();
        let mut rng = cursor(0);
        for _ in 0..5 {
            let out = Expander {
                grammar: &g,
                history: &mut h,
            }
            .expand("s", &v, &b, &mut rng)
            .unwrap();
            assert_eq!(out, "only");
        }
    }

    proptest! {
        #[test]
        fn no_repeat_within_window(seed: u64) {
            let g = Grammar::from_ron(
                r#"(symbols: {"s": [
                  (text: "a", weight: 1000), (text: "b", weight: 700),
                  (text: "c", weight: 300), (text: "d", weight: 50),
                ]})"#,
            ).unwrap();
            let mut h = ExpansionHistory::default();
            let v = voice(3);
            let b = MapBindings::default();
            let mut rng = cursor(seed);
            let outs: Vec<String> = (0..40)
                .map(|_| Expander { grammar: &g, history: &mut h }.expand("s", &v, &b, &mut rng).unwrap())
                .collect();
            for w in outs.windows(4) {
                let set: std::collections::BTreeSet<_> = w.iter().collect();
                prop_assert_eq!(set.len(), 4, "{:?}", w);
            }
        }
    }

    const SAMPLE: &str = r##"(symbols: {
      "report": [
        (text: "#opening# #detail#", weight: 1000),
        (text: "#detail#", weight: 500, tags: ["terse"]),
      ],
      "opening": [
        (text: "{time:hm}. Moving.", weight: 1000, tags: ["terse"]),
        (text: "Out of camp at {time:hm}.", weight: 800),
        (text: "We left before light.", weight: 600, when: ["day == 1"], tags: ["dry"]),
      ],
      "detail": [
        (text: "{persons:words} on the road, water at {water:L}.", weight: 1000),
        (text: "#headcount.cap#; {water:L} left.", weight: 700, tags: ["terse"]),
        (text: "Walked {dist:km1}.", weight: 400),
      ],
      "headcount": [
        (text: "{persons:n} walking", weight: 1000),
        (text: "{persons:n} fit", weight: 1000, tags: ["dry"]),
      ],
    })"##;

    #[test]
    fn sample_snapshot() {
        let g = Grammar::from_ron(SAMPLE).unwrap();
        let mut v = voice(2);
        v.tags.insert("terse".into(), 2000);
        v.tags.insert("dry".into(), 1500);
        let b = MapBindings(BTreeMap::from([
            ("day".into(), Value::Int(1)),
            ("time".into(), Value::Time(ach_core::SimTime::at(2, 6, 40))),
            ("persons".into(), Value::Int(12)),
            ("water".into(), Value::Milli(340_400)),
            ("dist".into(), Value::DistanceM(12_449)),
        ]));
        let mut h = ExpansionHistory::default();
        let mut rng = cursor(2026);
        let lines: Vec<String> = (0..20)
            .map(|_| {
                Expander {
                    grammar: &g,
                    history: &mut h,
                }
                .expand("report", &v, &b, &mut rng)
                .unwrap()
            })
            .collect();
        insta::assert_snapshot!(lines.join("\n"));
    }
}
