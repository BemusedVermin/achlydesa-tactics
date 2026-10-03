//! World model: terrain, pathing, and spatial queries. See Execution Plan §4.1.
//!
//! This crate currently provides chunked integer heightfields and height queries
//! (High-Level Design §4.1a, Technical Design §11.1 and §21.1, Execution Plan §4.5),
//! plus movement profiles, grid pathfinding and route handling
//! (High-Level Design §4.1b and §5.2, Technical Design §11.2).
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod chunk;
pub mod detail;
pub mod error;
pub mod manifest;
pub mod nav;
pub mod profile;
pub mod route;
pub mod terrain;

pub use chunk::{
    BASE_SPACING_CM, CHUNK_INTERVALS, CHUNK_SIZE_CM, ChunkCoord, HeightChunk, SAMPLES_PER_SIDE,
};
pub use detail::DetailParams;
pub use error::WorldError;
pub use manifest::{SourceInfo, TerrainManifest};
pub use nav::{NavQuery, Path, PathError};
pub use profile::{MovementProfile, ProfileSet};
pub use route::{Route, RouteSet, nearest_vertex, route_path};
pub use terrain::{SurfaceClass, Terrain};
