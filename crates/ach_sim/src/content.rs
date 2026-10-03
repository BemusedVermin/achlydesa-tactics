//! Content a scenario refers to: profiles, routes, consumption rates and sites.
//! Implements Simulation §11 (scenario setup); the site format is `red_ledger_sites.ron`.

use crate::error::SimError;
use crate::scenario::Scenario;
use ach_core::WorldPos;
use ach_logistics::ConsumptionTable;
use ach_world::{ProfileSet, RouteSet};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

/// A named place on the map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
    /// Stable identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Free-form kind (`camp`, `checkpoint`, ...).
    pub kind: String,
    /// Position on the surface.
    pub pos: WorldPos,
}

/// Everything loaded from the scenario's content paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Content {
    /// Movement profiles by name.
    pub profiles: ProfileSet,
    /// Authored routes by id.
    pub routes: RouteSet,
    /// Per-person consumption rates.
    pub consumption: ConsumptionTable,
    /// Sites by id.
    pub sites: BTreeMap<String, Site>,
}

#[derive(Deserialize)]
struct RawSite {
    id: String,
    name: String,
    kind: String,
    pos: (i64, i64),
}

#[derive(Deserialize)]
struct RawSites {
    sites: Vec<RawSite>,
}

impl Content {
    /// Loads the content files named by `scenario`, resolving paths against `repo_root`.
    pub fn load(scenario: &Scenario, repo_root: &Path) -> Result<Self, SimError> {
        Ok(Self {
            profiles: ProfileSet::load(&repo_root.join(&scenario.profiles))?,
            routes: RouteSet::load(&repo_root.join(&scenario.routes))?,
            consumption: ConsumptionTable::load(&repo_root.join(&scenario.consumption))?,
            sites: load_sites(&repo_root.join(&scenario.sites))?,
        })
    }

    /// The site with this id.
    pub fn site(&self, id: &str) -> Result<&Site, SimError> {
        self.sites.get(id).ok_or_else(|| SimError::Unknown {
            what: "site",
            key: id.to_owned(),
        })
    }
}

/// Parses a site file, ignoring fields this crate does not use. Duplicate ids are an error.
pub fn load_sites(path: &Path) -> Result<BTreeMap<String, Site>, SimError> {
    let shown = path.display().to_string();
    let text = std::fs::read_to_string(path).map_err(|e| SimError::Io {
        path: shown.clone(),
        message: e.to_string(),
    })?;
    let raw: RawSites = ron::from_str(&text).map_err(|e| SimError::Parse {
        path: shown,
        message: e.to_string(),
    })?;
    let mut sites = BTreeMap::new();
    for s in raw.sites {
        let site = Site {
            pos: WorldPos::surface_m(s.pos.0, s.pos.1),
            id: s.id,
            name: s.name,
            kind: s.kind,
        };
        let id = site.id.clone();
        if sites.insert(id.clone(), site).is_some() {
            return Err(SimError::Invalid(format!("duplicate site id {id:?}")));
        }
    }
    Ok(sites)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_ledger_sites_load() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/world/red_ledger_sites.ron");
        let sites = load_sites(&path).unwrap();
        let camp = &sites["cistern_camp"];
        assert_eq!(camp.name, "Cistern Camp");
        assert_eq!(camp.pos, WorldPos::surface_m(10_000, 96_500));
    }
}
