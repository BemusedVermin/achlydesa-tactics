//! Error type for `ach_sim`. Implements Execution Plan §3.2 (public fallible functions return `Result`).

use ach_core::CoreError;
use ach_logistics::LogisticsError;
use ach_people::PeopleError;
use ach_sched::SchedError;
use ach_world::{PathError, WorldError};

/// Failures raised while loading a scenario or running the simulation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SimError {
    /// A file could not be read.
    #[error("cannot read {path}: {message}")]
    Io {
        /// Path that failed to open.
        path: String,
        /// Underlying I/O error text.
        message: String,
    },
    /// A file is not valid RON for its format.
    #[error("cannot parse {path}: {message}")]
    Parse {
        /// Path that failed to parse.
        path: String,
        /// Parser error text.
        message: String,
    },
    /// The scenario names something its content files do not define.
    #[error("unknown {what} {key:?}")]
    Unknown {
        /// Kind of thing looked up (`"site"`, `"route"`, ...).
        what: &'static str,
        /// The key that was not found.
        key: String,
    },
    /// The scenario is internally inconsistent.
    #[error("invalid scenario: {0}")]
    Invalid(String),
    /// A leg crosses ground the profile cannot walk (fail-fast; Phase 3 turns this into a
    /// decision request).
    #[error("column {column:?} leg {leg} segment {segment} is impassable")]
    ImpassableLeg {
        /// Column name.
        column: String,
        /// Index of the leg in the column's plan.
        leg: usize,
        /// Index of the segment within the leg.
        segment: usize,
    },
    /// A leg runs off the terrain.
    #[error("column {column:?} leg {leg} leaves the terrain at segment {segment}")]
    OffTerrain {
        /// Column name.
        column: String,
        /// Index of the leg in the column's plan.
        leg: usize,
        /// Index of the segment within the leg.
        segment: usize,
    },
    /// Cross-country pathfinding failed.
    #[error("column {column:?} leg {leg}: {source}")]
    NoPath {
        /// Column name.
        column: String,
        /// Index of the leg in the column's plan.
        leg: usize,
        /// Why the search failed.
        source: PathError,
    },
    /// A core primitive failed (time parsing, journal, ids).
    #[error(transparent)]
    Core(#[from] CoreError),
    /// The scheduler rejected an event.
    #[error(transparent)]
    Sched(#[from] SchedError),
    /// Terrain, profile or route loading failed.
    #[error(transparent)]
    World(#[from] WorldError),
    /// Roster loading failed.
    #[error(transparent)]
    People(#[from] PeopleError),
    /// Consumption loading or a stock operation failed.
    #[error(transparent)]
    Logistics(#[from] LogisticsError),
}
