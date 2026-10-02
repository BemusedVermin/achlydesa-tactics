//! Core primitives: ids, time, fixed-point coordinates, keyed RNG, journal. See Execution Plan §4.1.
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

// Lets `define_id!` expand to `::ach_core::...` paths inside this crate too.
extern crate self as ach_core;

pub mod error;
pub mod hash;
pub mod ids;
pub mod pos;
pub mod qty;
pub mod rng;
pub mod time;

/// Re-export used by `define_id!` so downstream crates need not name `serde` themselves.
#[doc(hidden)]
pub use serde as __serde;

pub use error::CoreError;
pub use hash::fnv1a64;
pub use ids::IdAllocator;
pub use pos::{CM_PER_M, ElevationCm, LevelId, WorldPos};
pub use qty::{Milli, PerMille};
pub use rng::{
    Domain, RngCursor, Seed, StreamKey, choose_weighted, draw_below, draw_permille, draw_u64, mix64,
};
pub use time::{SimDuration, SimTime};
