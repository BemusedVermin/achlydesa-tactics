//! Height chunks and chunk coordinates.
//! Implements High-Level Design §4.1a (grid) and Technical Design §21.1 (storage model).

use serde::{Deserialize, Serialize};

/// Spacing of source height samples, cm (30 m).
pub const BASE_SPACING_CM: i64 = 3_000;
/// Cells per chunk side; chunks have one more sample per side so edges are shared.
pub const CHUNK_INTERVALS: i64 = 64;
/// Chunk side length, cm (1.92 km).
pub const CHUNK_SIZE_CM: i64 = BASE_SPACING_CM * CHUNK_INTERVALS;
/// Height samples per chunk side.
pub const SAMPLES_PER_SIDE: usize = 65;

/// Index of a chunk in the chunk lattice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    /// Column (east).
    pub cx: i32,
    /// Row (north).
    pub cy: i32,
}

impl ChunkCoord {
    /// The chunk containing a point, by floor division.
    ///
    /// Coordinates whose chunk index does not fit `i32` saturate.
    pub fn containing(x_cm: i64, y_cm: i64) -> Self {
        let idx = |v: i64| {
            let c = v.div_euclid(CHUNK_SIZE_CM);
            i32::try_from(c).unwrap_or(if c < 0 { i32::MIN } else { i32::MAX })
        };
        Self {
            cx: idx(x_cm),
            cy: idx(y_cm),
        }
    }

    /// Position of the chunk's south-west sample, cm.
    pub fn origin_cm(self) -> (i64, i64) {
        (
            i64::from(self.cx) * CHUNK_SIZE_CM,
            i64::from(self.cy) * CHUNK_SIZE_CM,
        )
    }
}

/// A 65x65 grid of heights in cm, row-major: `index = row * 65 + col`, row = y.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeightChunk {
    /// Which chunk this is.
    pub coord: ChunkCoord,
    /// `SAMPLES_PER_SIDE²` heights, cm.
    pub heights_cm: Vec<i32>,
}

impl HeightChunk {
    /// Height at sample (`col`, `row`), each in `0..65`.
    ///
    /// # Panics
    /// If the chunk is malformed or an index is out of range.
    pub fn sample(&self, col: usize, row: usize) -> i32 {
        self.heights_cm[row * SAMPLES_PER_SIDE + col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn containing_floors_negatives() {
        assert_eq!(ChunkCoord::containing(-1, 0), ChunkCoord { cx: -1, cy: 0 });
        assert_eq!(
            ChunkCoord::containing(CHUNK_SIZE_CM, -CHUNK_SIZE_CM - 1),
            ChunkCoord { cx: 1, cy: -2 }
        );
    }

    #[test]
    fn origin_round_trips() {
        let c = ChunkCoord { cx: -3, cy: 5 };
        let (x, y) = c.origin_cm();
        assert_eq!(ChunkCoord::containing(x, y), c);
        assert_eq!(
            ChunkCoord::containing(x - 1, y),
            ChunkCoord { cx: -4, cy: 5 }
        );
    }
}
