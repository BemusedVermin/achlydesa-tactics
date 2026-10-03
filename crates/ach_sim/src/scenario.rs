//! Scenario files: what a run contains and where its content lives.
//! Implements Simulation §11 (scenario setup) for the Phase 1 march skeleton.

use crate::error::SimError;
use ach_logistics::StockKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// A complete, self-describing run: clock, content paths and the columns to march.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scenario {
    /// Scenario id; also the journal label.
    pub id: String,
    /// Campaign seed.
    pub seed: u64,
    /// Start time, `"D1 05:00"` format.
    pub start: String,
    /// Time limit, same format.
    pub end: String,
    /// Terrain chunk directory, relative to the repo root.
    pub terrain_dir: String,
    /// Roster file, relative to the repo root.
    pub roster: String,
    /// Movement profile file, relative to the repo root.
    pub profiles: String,
    /// Consumption table, relative to the repo root.
    pub consumption: String,
    /// Route file, relative to the repo root.
    pub routes: String,
    /// Site file (the `red_ledger_sites.ron` format), relative to the repo root.
    pub sites: String,
    /// The marching columns.
    pub columns: Vec<ColumnSpec>,
}

/// One marching column.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnSpec {
    /// Display name.
    pub name: String,
    /// Leader's character key.
    pub leader: String,
    /// Character keys of the people who report on the column.
    pub reporters: Vec<String>,
    /// Head count, which sets consumption.
    pub persons: u32,
    /// Movement profile name, e.g. `"foot"`.
    pub profile: String,
    /// Posture speed multiplier in per-mille (guarded foot column about 620).
    pub posture_permille: i32,
    /// Starting stocks, in milli-units.
    pub stocks: BTreeMap<StockKind, i64>,
    /// Id of the site the column starts at.
    pub start_site: String,
    /// The legs to march, in order.
    pub legs: Vec<LegSpec>,
    /// When the column is under way.
    pub schedule: ScheduleSpec,
}

/// One leg of a column's plan.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegSpec {
    /// Along an authored route, between the vertices nearest the two sites.
    FollowRoute {
        /// Route id.
        route: String,
        /// Destination site id.
        to_site: String,
    },
    /// Over open ground by fastest-time pathfinding.
    CrossCountry {
        /// Destination site id.
        to_site: String,
    },
}

impl LegSpec {
    /// The id of the site this leg ends at.
    pub fn to_site(&self) -> &str {
        match self {
            Self::FollowRoute { to_site, .. } | Self::CrossCountry { to_site } => to_site,
        }
    }
}

/// Daily march windows and the march/halt cycle inside them (High-Level Design §4.1b).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleSpec {
    /// `("05:00", "11:00")` start and end of each window.
    pub windows: Vec<(String, String)>,
    /// Minutes marched per cycle.
    pub march_minutes: u32,
    /// Minutes halted per cycle.
    pub halt_minutes: u32,
}

impl Scenario {
    /// Reads a scenario from a RON file.
    pub fn load(path: &Path) -> Result<Self, SimError> {
        let shown = path.display().to_string();
        let text = std::fs::read_to_string(path).map_err(|e| SimError::Io {
            path: shown.clone(),
            message: e.to_string(),
        })?;
        ron::from_str(&text).map_err(|e| SimError::Parse {
            path: shown,
            message: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ron_form() {
        let text = r#"(
            id: "t", seed: 7, start: "D1 05:00", end: "D2 05:00",
            terrain_dir: "a", roster: "b", profiles: "c", consumption: "d", routes: "e", sites: "f",
            columns: [(
                name: "N", leader: "l", reporters: [], persons: 3, profile: "foot",
                posture_permille: 620, stocks: { Water: 1000 }, start_site: "s",
                legs: [FollowRoute(route: "r", to_site: "x"), CrossCountry(to_site: "y")],
                schedule: (windows: [("05:00", "11:00")], march_minutes: 50, halt_minutes: 10),
            )],
        )"#;
        let s: Scenario = ron::from_str(text).unwrap();
        assert_eq!(s.columns[0].stocks[&StockKind::Water], 1000);
        assert_eq!(s.columns[0].legs[1].to_site(), "y");
    }
}
