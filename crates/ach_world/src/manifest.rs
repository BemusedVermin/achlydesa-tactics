//! Terrain manifest: extent, detail parameters, and data provenance.
//! Implements Technical Design §21.1 (storage model) and Execution Plan §4.5 (terrain source).

use crate::chunk::ChunkCoord;
use crate::detail::DetailParams;
use serde::{Deserialize, Serialize};

/// Where the heights came from, so a build can be audited and reproduced.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Dataset name.
    pub dataset: String,
    /// `(filename, fnv1a64 hex)` of each source tile.
    pub tiles: Vec<(String, String)>,
    /// Description of the projection and resampling applied.
    pub transform: String,
    /// Version of the tool that produced the chunks.
    pub tool_version: String,
}

/// Contents of `manifest.ron`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrainManifest {
    /// Theater or window name; also the label in chunk file headers.
    pub name: String,
    /// South-west chunk, inclusive.
    pub min_chunk: ChunkCoord,
    /// North-east chunk, inclusive.
    pub max_chunk: ChunkCoord,
    /// Sub-resolution detail parameters.
    pub detail: DetailParams,
    /// Source provenance.
    pub source: SourceInfo,
}

impl TerrainManifest {
    /// True if the chunk range contains at least one chunk.
    pub fn has_chunks(&self) -> bool {
        self.min_chunk.cx <= self.max_chunk.cx && self.min_chunk.cy <= self.max_chunk.cy
    }

    /// Every chunk coordinate in range, row by row (south to north, west to east).
    pub fn coords(&self) -> impl Iterator<Item = ChunkCoord> + use<> {
        let (x0, x1) = (self.min_chunk.cx, self.max_chunk.cx);
        (self.min_chunk.cy..=self.max_chunk.cy)
            .flat_map(move |cy| (x0..=x1).map(move |cx| ChunkCoord { cx, cy }))
    }
}
