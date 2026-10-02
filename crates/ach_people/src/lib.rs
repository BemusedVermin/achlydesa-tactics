//! People model: characters, affinity, cohesion, succession. See Execution Plan §4.1.
//! This slice implements the minimal roster of Simulation §19.1 (people as persistent
//! participants), seeded with the Red Ledger founders of Campaign Bible §4 and §6.
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod error;

use ach_core::IdAllocator;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

pub use error::PeopleError;

ach_core::define_id!(
    /// Stable identifier of a character.
    CharacterId
);

/// A character's job in the caravan (Campaign Bible §6).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Role {
    /// Leads the escort.
    EscortCaptain,
    /// Heads recovery and maintenance crews.
    RecoveryChief,
    /// Medical orderly.
    Orderly,
    /// Leads transport.
    TransportLeader,
    /// Guard still in training.
    GuardTrainee,
    /// Keeps the traveling archive.
    ArchiveKeeper,
    /// Ordinary guard.
    Guard,
    /// Civilian traveler.
    Passenger,
    /// Any role not listed above.
    Other(String),
}

/// A named person. `voice` keys the text system's per-character register.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    /// Identifier assigned at load time.
    pub id: CharacterId,
    /// Stable content key, unique within a roster.
    pub key: String,
    /// Display name.
    pub name: String,
    /// Age in years at A238.
    pub age: u16,
    /// Job in the caravan.
    pub role: Role,
    /// Voice identifier used by `ach_text`.
    pub voice: String,
}

/// One content-file entry, before an id is assigned.
#[derive(Deserialize)]
struct Entry {
    key: String,
    name: String,
    age: u16,
    role: Role,
    voice: String,
}

/// All known characters, indexed by id and by content key.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Roster {
    by_id: BTreeMap<CharacterId, Character>,
    by_key: BTreeMap<String, CharacterId>,
}

impl Roster {
    /// Loads a content file and assigns ids with the allocator, in file order.
    ///
    /// Fails with [`PeopleError::DuplicateKey`] if two entries share a key.
    pub fn load(path: &Path, ids: &mut IdAllocator) -> Result<Self, PeopleError> {
        let shown = path.display().to_string();
        let text = std::fs::read_to_string(path).map_err(|e| PeopleError::Io {
            path: shown.clone(),
            message: e.to_string(),
        })?;
        let entries: Vec<Entry> = ron::from_str(&text).map_err(|e| PeopleError::Parse {
            path: shown,
            message: e.to_string(),
        })?;
        Self::from_entries(entries, ids)
    }

    fn from_entries(entries: Vec<Entry>, ids: &mut IdAllocator) -> Result<Self, PeopleError> {
        let mut roster = Self::default();
        for e in entries {
            if roster.by_key.contains_key(&e.key) {
                return Err(PeopleError::DuplicateKey(e.key));
            }
            let id: CharacterId = ids.try_alloc()?;
            roster.by_key.insert(e.key.clone(), id);
            roster.by_id.insert(
                id,
                Character {
                    id,
                    key: e.key,
                    name: e.name,
                    age: e.age,
                    role: e.role,
                    voice: e.voice,
                },
            );
        }
        Ok(roster)
    }

    /// Looks a character up by id.
    pub fn get(&self, id: CharacterId) -> Option<&Character> {
        self.by_id.get(&id)
    }

    /// Looks a character up by content key.
    pub fn by_key(&self, key: &str) -> Option<&Character> {
        self.by_key.get(key).and_then(|id| self.by_id.get(id))
    }

    /// Iterates characters in id order.
    pub fn iter(&self) -> impl Iterator<Item = &Character> {
        self.by_id.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn founders() -> Roster {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../content/people/red_ledger_founders.ron");
        Roster::load(&path, &mut IdAllocator::new(1)).unwrap()
    }

    #[test]
    fn founders_match_campaign_bible() {
        let r = founders();
        let got: Vec<_> = r
            .iter()
            .map(|c| (c.key.as_str(), c.age, c.role.clone()))
            .collect();
        let want = vec![
            ("ione_var", 41, Role::EscortCaptain),
            ("tessa_ruun", 35, Role::RecoveryChief),
            ("neris_vale", 29, Role::Orderly),
            ("petra_oss", 32, Role::TransportLeader),
            ("eren_tal", 19, Role::GuardTrainee),
            ("mara_den", 78, Role::ArchiveKeeper),
        ];
        assert_eq!(got, want);
        assert_eq!(r.by_key("ione_var").unwrap().name, "Ione Var");
        assert_eq!(r.by_key("ione_var").unwrap().voice, "ione");
        let id = r.by_key("mara_den").unwrap().id;
        assert_eq!(r.get(id).unwrap().key, "mara_den");
        assert_eq!(id, CharacterId(6));
    }

    #[test]
    fn duplicate_key_rejected() {
        let text = r#"[
            (key: "a", name: "A", age: 1, role: Guard, voice: "a"),
            (key: "a", name: "B", age: 2, role: Passenger, voice: "b"),
        ]"#;
        let entries: Vec<Entry> = ron::from_str(text).unwrap();
        let err = Roster::from_entries(entries, &mut IdAllocator::new(1)).unwrap_err();
        assert_eq!(err, PeopleError::DuplicateKey("a".into()));
    }

    #[test]
    fn missing_file_is_io_error() {
        let err = Roster::load(Path::new("/nonexistent/x.ron"), &mut IdAllocator::new(1));
        assert!(matches!(err, Err(PeopleError::Io { .. })));
    }
}
