//! Composition facade and headless runner. See Execution Plan §4.1.
//!
//! This slice is the Phase 1 march simulation: columns marching over real terrain on the
//! event-driven clock (Simulation §3.1-§3.3, Technical Design §16.1-§16.3), journaling
//! everything that happens.
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod column;
pub mod content;
pub mod error;
pub mod events;
pub mod kinematics;
pub mod scenario;
pub mod schedule;
pub mod sim;

pub use column::{Column, ColumnId};
pub use content::{Content, Site};
pub use error::SimError;
pub use events::{EndReason, SCHEMA_VERSION, SimEvent, phase};
pub use scenario::{ColumnSpec, LegSpec, Scenario, ScheduleSpec};
pub use schedule::{Boundary, MarchSchedule};
pub use sim::{RunSummary, Sim, SimState};
