//! Grammar files: parsed symbols and alternatives. Implements Execution Plan §4.6.

use crate::condition::Condition;
use crate::error::TextError;
use crate::template::Template;
use serde::Deserialize;
use std::collections::BTreeMap;

/// One weighted, conditional way to say a symbol.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alternative {
    /// Parsed template.
    pub template: Template,
    /// Base weight; 0 disables the alternative.
    pub weight: u32,
    /// Conditions that must all hold.
    pub when: Vec<Condition>,
    /// Voice tags this alternative carries.
    pub tags: Vec<String>,
}

/// A parsed grammar: symbol name to alternatives, in file order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Grammar {
    pub(crate) symbols: BTreeMap<String, Vec<Alternative>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAlternative {
    text: String,
    weight: u32,
    #[serde(default)]
    when: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawGrammar {
    symbols: BTreeMap<String, Vec<RawAlternative>>,
}

impl RawAlternative {
    fn parse(self) -> Result<Alternative, TextError> {
        Ok(Alternative {
            template: Template::parse(&self.text)?,
            weight: self.weight,
            when: self
                .when
                .iter()
                .map(|c| Condition::parse(c))
                .collect::<Result<_, _>>()?,
            tags: self.tags,
        })
    }
}

impl Grammar {
    /// Parses a RON grammar file, validating every template and condition.
    pub fn from_ron(src: &str) -> Result<Grammar, TextError> {
        let raw: RawGrammar = ron::from_str(src).map_err(|e| TextError::Ron(e.to_string()))?;
        let mut symbols = BTreeMap::new();
        for (name, alts) in raw.symbols {
            if u16::try_from(alts.len()).is_err() {
                return Err(TextError::TooManyAlternatives { symbol: name });
            }
            let alts = alts
                .into_iter()
                .map(RawAlternative::parse)
                .collect::<Result<Vec<_>, _>>()?;
            symbols.insert(name, alts);
        }
        Ok(Grammar { symbols })
    }

    /// Adds every symbol of `other`. A symbol defined in both is
    /// [`TextError::DuplicateSymbol`], and `self` is left unchanged.
    pub fn merge(&mut self, other: Grammar) -> Result<(), TextError> {
        if let Some(dup) = other.symbols.keys().find(|k| self.symbols.contains_key(*k)) {
            return Err(TextError::DuplicateSymbol(dup.clone()));
        }
        self.symbols.extend(other.symbols);
        Ok(())
    }

    /// The alternatives of `symbol`, if defined.
    pub fn alternatives(&self, symbol: &str) -> Option<&[Alternative]> {
        self.symbols.get(symbol).map(Vec::as_slice)
    }

    /// All symbol names, sorted.
    pub fn symbol_names(&self) -> impl Iterator<Item = &str> {
        self.symbols.keys().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = r#"(
      symbols: {
        "march.departed": [
          (text: "{time:hm}. Moving.", weight: 1000, when: [], tags: ["terse"]),
          (text: "Out of #place.camp#.", weight: 800, when: ["day == 1"]),
        ],
      },
    )"#;

    #[test]
    fn loads_spec_shape() {
        let g = Grammar::from_ron(SRC).unwrap();
        let alts = g.alternatives("march.departed").unwrap();
        assert_eq!(alts.len(), 2);
        assert_eq!(alts[0].tags, vec!["terse".to_owned()]);
        assert_eq!(alts[1].when.len(), 1);
    }

    #[test]
    fn bad_condition_and_template_fail_at_load() {
        let cond = r#"(symbols: {"a": [(text: "x", weight: 1, when: ["day ~ 1"])]})"#;
        assert!(matches!(
            Grammar::from_ron(cond),
            Err(TextError::BadCondition { .. })
        ));
        let tpl = r##"(symbols: {"a": [(text: "#oops", weight: 1)]})"##;
        assert!(matches!(
            Grammar::from_ron(tpl),
            Err(TextError::BadTemplate { .. })
        ));
        assert!(matches!(Grammar::from_ron("("), Err(TextError::Ron(_))));
    }

    #[test]
    fn merge_rejects_duplicates_and_keeps_self() {
        let mut a = Grammar::from_ron(SRC).unwrap();
        let before = a.clone();
        let dup = Grammar::from_ron(SRC).unwrap();
        assert_eq!(
            a.merge(dup),
            Err(TextError::DuplicateSymbol("march.departed".into()))
        );
        assert_eq!(a, before);
        let other = Grammar::from_ron(r#"(symbols: {"x": [(text: "y", weight: 1)]})"#).unwrap();
        a.merge(other).unwrap();
        assert_eq!(a.symbol_names().count(), 2);
    }
}
