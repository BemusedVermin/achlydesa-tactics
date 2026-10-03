//! Speaker voice profiles. Implements Execution Plan §4.6 (per-character voice).

use crate::error::TextError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Neutral tag multiplier, in thousandths.
const NEUTRAL: u32 = 1000;

/// How one character sounds: tag affinities and how long they avoid repeating themselves.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProfile {
    /// Voice identifier (the key in the voice file).
    pub id: String,
    /// Tag affinity in thousandths; 1000 is neutral, absent tags count as 1000.
    pub tags: BTreeMap<String, u32>,
    /// How many of this speaker's recent choices per symbol are avoided.
    pub avoid_repeat: u8,
}

impl VoiceProfile {
    /// Effective weight of an alternative with `base` weight and `tags`:
    /// `base × Π tags[t] / 1000`, in `u64` with a division after each factor.
    /// A positive base weight never drops below 1; the result saturates at `u32::MAX`.
    pub fn effective_weight(&self, base: u32, tags: &[String]) -> u32 {
        let w = tags.iter().fold(u64::from(base), |w, t| {
            let f = u64::from(self.tags.get(t).copied().unwrap_or(NEUTRAL));
            w.saturating_mul(f) / u64::from(NEUTRAL)
        });
        let w = if base > 0 { w.max(1) } else { 0 };
        u32::try_from(w).unwrap_or(u32::MAX)
    }
}

/// A collection of voices, keyed by id.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceSet {
    /// Voices by id.
    pub voices: BTreeMap<String, VoiceProfile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawVoice {
    #[serde(default)]
    tags: BTreeMap<String, u32>,
    #[serde(default)]
    avoid_repeat: u8,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawVoices {
    voices: BTreeMap<String, RawVoice>,
}

impl VoiceSet {
    /// Parses a voice file: `( voices: { "ione": (tags: {..}, avoid_repeat: 3) } )`.
    pub fn from_ron(src: &str) -> Result<VoiceSet, TextError> {
        let raw: RawVoices = ron::from_str(src).map_err(|e| TextError::Ron(e.to_string()))?;
        let voices = raw
            .voices
            .into_iter()
            .map(|(id, v)| {
                let profile = VoiceProfile {
                    id: id.clone(),
                    tags: v.tags,
                    avoid_repeat: v.avoid_repeat,
                };
                (id, profile)
            })
            .collect();
        Ok(VoiceSet { voices })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ione() -> VoiceProfile {
        VoiceSet::from_ron(
            r#"( voices: { "ione": (tags: { "terse": 2000, "dry": 1500, "formal": 500 }, avoid_repeat: 3) } )"#,
        )
        .unwrap()
        .voices["ione"]
            .clone()
    }

    fn tags(t: &[&str]) -> Vec<String> {
        t.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn parses_voice_file() {
        let v = ione();
        assert_eq!(v.id, "ione");
        assert_eq!(v.avoid_repeat, 3);
        assert_eq!(v.tags["dry"], 1500);
    }

    #[test]
    fn effective_weight_known_answers() {
        let v = ione();
        assert_eq!(v.effective_weight(1000, &tags(&["terse"])), 2000);
        assert_eq!(v.effective_weight(1000, &tags(&["terse", "dry"])), 3000);
        assert_eq!(v.effective_weight(1000, &tags(&["formal", "formal"])), 250);
        assert_eq!(v.effective_weight(800, &tags(&["unknown"])), 800);
        assert_eq!(v.effective_weight(800, &[]), 800);
        // division after each factor: 7 * 500 / 1000 = 3, then 3 * 500 / 1000 = 1
        assert_eq!(v.effective_weight(7, &tags(&["formal", "formal"])), 1);
    }

    #[test]
    fn minimum_one_unless_base_zero() {
        let v = ione();
        assert_eq!(v.effective_weight(1, &tags(&["formal"])), 1);
        assert_eq!(v.effective_weight(0, &tags(&["terse"])), 0);
    }

    #[test]
    fn bad_voice_file_is_error() {
        assert!(matches!(
            VoiceSet::from_ron("(nope)"),
            Err(TextError::Ron(_))
        ));
    }
}
