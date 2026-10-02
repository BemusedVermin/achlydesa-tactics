//! World model: terrain, pathing, and spatial queries. See Execution Plan §4.1.
//!
//! This crate currently provides chunked integer heightfields and height queries
//! (High-Level Design §4.1a, Technical Design §11.1 and §21.1, Execution Plan §4.5).
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod chunk;
pub mod detail;
pub mod error;
pub mod manifest;
pub mod terrain;

pub use chunk::{
    BASE_SPACING_CM, CHUNK_INTERVALS, CHUNK_SIZE_CM, ChunkCoord, HeightChunk, SAMPLES_PER_SIDE,
};
pub use detail::DetailParams;
pub use error::WorldError;
pub use manifest::{SourceInfo, TerrainManifest};
pub use terrain::{SurfaceClass, Terrain};
