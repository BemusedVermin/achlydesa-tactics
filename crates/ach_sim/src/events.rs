//! Journaled simulation events and their ordering phases.
//! Implements Technical Design §16.2 (event ordering) and Simulation §3.3 (journal of
//! everything that happens).

use crate::column::ColumnId;
use ach_core::{Milli, PerMille, SimDuration, WorldPos};
use ach_logistics::{StockKind, StockSet};
use serde::{Deserialize, Serialize};

/// Journal schema version, written to the journal header.
pub const SCHEMA_VERSION: u32 = 1;

/// Phases within one timestamp; lower runs first (Technical Design §16.2).
pub mod phase {
    /// Calendar events.
    pub const DAY: u8 = 0;
    /// Departures and march-schedule boundaries.
    pub const SCHEDULE: u8 = 10;
    /// Reaching sites.
    pub const ARRIVAL: u8 = 20;
    /// Stock thresholds and end-of-day bookkeeping.
    pub const STOCK: u8 = 30;
    /// End of the scenario.
    pub const END: u8 = 250;
}

/// Why a scenario ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndReason {
    /// Every column reached its final site.
    AllArrived,
    /// The scenario's time limit passed.
    TimeLimit,
}

/// Everything the Phase 1 simulation journals.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimEvent {
    /// The run began.
    ScenarioStarted {
        /// Scenario id.
        scenario: String,
    },
    /// A new day began at midnight.
    DayBegan {
        /// One-based day number.
        day: i64,
    },
    /// A column set out from its start site.
    ColumnDeparted {
        /// The column.
        column: ColumnId,
        /// Site id it left.
        site: String,
        /// Where it stands.
        pos: WorldPos,
    },
    /// A march window opened.
    MarchWindowStarted {
        /// The column.
        column: ColumnId,
        /// Where it stands.
        pos: WorldPos,
    },
    /// A march window closed.
    MarchWindowEnded {
        /// The column.
        column: ColumnId,
        /// Where it stands.
        pos: WorldPos,
        /// Distance covered since the day began, m.
        marched_today_m: i64,
    },
    /// A short halt began.
    ShortHaltBegan {
        /// The column.
        column: ColumnId,
        /// Where it stands.
        pos: WorldPos,
    },
    /// A short halt ended.
    ShortHaltEnded {
        /// The column.
        column: ColumnId,
        /// Where it stands.
        pos: WorldPos,
    },
    /// A column reached the end of a leg.
    SiteReached {
        /// The column.
        column: ColumnId,
        /// Site id.
        site: String,
        /// Where it stands.
        pos: WorldPos,
        /// Distance walked since departure, m.
        distance_from_start_m: i64,
    },
    /// A stock fell to a fraction of its initial amount.
    StockThreshold {
        /// The column.
        column: ColumnId,
        /// Which stock.
        kind: StockKind,
        /// Amount left.
        remaining: Milli,
        /// Amount left as a fraction of the initial amount.
        remaining_permille: PerMille,
    },
    /// A stock ran out.
    StockExhausted {
        /// The column.
        column: ColumnId,
        /// Which stock.
        kind: StockKind,
    },
    /// A day ended for a column.
    ColumnDayEnded {
        /// The column.
        column: ColumnId,
        /// One-based day number.
        day: i64,
        /// Distance covered that day, m.
        distance_today_m: i64,
        /// Stocks left.
        stocks: StockSet,
        /// Where it stands.
        pos: WorldPos,
    },
    /// A column reached its final site.
    ColumnArrived {
        /// The column.
        column: ColumnId,
        /// Final site id.
        site: String,
        /// Where it stands.
        pos: WorldPos,
        /// Distance walked since departure, m.
        total_distance_m: i64,
        /// Time since departure.
        elapsed: SimDuration,
    },
    /// The run ended.
    ScenarioEnded {
        /// Why.
        reason: EndReason,
    },
}

impl SimEvent {
    /// The column this event concerns, if it is a column event.
    pub fn column(&self) -> Option<ColumnId> {
        match self {
            Self::ColumnDeparted { column, .. }
            | Self::MarchWindowStarted { column, .. }
            | Self::MarchWindowEnded { column, .. }
            | Self::ShortHaltBegan { column, .. }
            | Self::ShortHaltEnded { column, .. }
            | Self::SiteReached { column, .. }
            | Self::StockThreshold { column, .. }
            | Self::StockExhausted { column, .. }
            | Self::ColumnDayEnded { column, .. }
            | Self::ColumnArrived { column, .. } => Some(*column),
            Self::ScenarioStarted { .. } | Self::DayBegan { .. } | Self::ScenarioEnded { .. } => {
                None
            }
        }
    }
}
