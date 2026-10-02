//! Terrain: chunked base heights plus procedural detail, and the queries built on them.
//! Implements High-Level Design §4.1a (grid and layers), Technical Design §11.1 (spatial
//! representation) and §21.1 (storage model), and Execution Plan §4.5 (terrain source).
//!
//! On disk a terrain directory holds `manifest.ron` and one `chunk_{cx}_{cy}.bin` per chunk.

use crate::chunk::{
    BASE_SPACING_CM, CHUNK_INTERVALS, CHUNK_SIZE_CM, ChunkCoord, HeightChunk, SAMPLES_PER_SIDE,
};
use crate::error::WorldError;
use crate::manifest::TerrainManifest;
use ach_core::{
    FileHeader, LevelId, MAGIC_SNAPSHOT, MAGIC_TERRAIN, Seed, WorldPos, read_header, read_snapshot,
    write_snapshot,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const MANIFEST_FILE: &str = "manifest.ron";
/// Chunk file schema version.
const CHUNK_SCHEMA: u32 = 1;
/// Half the central-difference baseline for gradients, cm.
const GRADIENT_HALF_CM: i64 = 1_500;

/// Walkability class of a surface, by gradient (per-mille):
/// flat below 30, gentle below 120, steep below 400, cliff beyond.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SurfaceClass {
    /// Gradient under 30 ‰.
    Flat,
    /// Gradient from 30 ‰ up to 120 ‰.
    Gentle,
    /// Gradient from 120 ‰ up to 400 ‰.
    Steep,
    /// Gradient of 400 ‰ or more.
    Cliff,
}

/// The theater's ground.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Terrain {
    manifest: TerrainManifest,
    chunks: BTreeMap<ChunkCoord, HeightChunk>,
}

/// Cell location along one axis: chunk index, cell within the chunk, Q16 fraction.
struct AxisCell {
    chunk: i32,
    cell: usize,
    frac: i64,
}

impl Terrain {
    /// Builds terrain by sampling `f(x_cm, y_cm)` at every sample point of every chunk
    /// in the manifest range. Shared chunk edges are sampled at identical positions.
    pub fn from_fn(manifest: TerrainManifest, f: impl Fn(i64, i64) -> i32) -> Self {
        let chunks = manifest
            .coords()
            .map(|coord| {
                let (ox, oy) = coord.origin_cm();
                let heights_cm = (0..SAMPLES_PER_SIDE)
                    .flat_map(|row| (0..SAMPLES_PER_SIDE).map(move |col| (col, row)))
                    .map(|(col, row)| {
                        f(
                            ox + col as i64 * BASE_SPACING_CM,
                            oy + row as i64 * BASE_SPACING_CM,
                        )
                    })
                    .collect();
                (coord, HeightChunk { coord, heights_cm })
            })
            .collect();
        Self { manifest, chunks }
    }

    /// Loads `manifest.ron` and every chunk file in its range from `dir`.
    ///
    /// Fails if a chunk is missing, has a bad header, or disagrees with the manifest
    /// (coordinate, seed, label, or sample count).
    pub fn load(dir: &Path) -> Result<Self, WorldError> {
        let path = dir.join(MANIFEST_FILE);
        let text = std::fs::read_to_string(&path).map_err(|e| io_err(&path, &e))?;
        let manifest: TerrainManifest =
            ron::from_str(&text).map_err(|e| WorldError::Manifest(e.to_string()))?;
        if !manifest.has_chunks() {
            return Err(WorldError::EmptyRange);
        }
        let mut chunks = BTreeMap::new();
        for coord in manifest.coords() {
            chunks.insert(coord, read_chunk(dir, &manifest, coord)?);
        }
        Ok(Self { manifest, chunks })
    }

    /// Writes `manifest.ron` and one file per chunk into `dir`, creating it if needed.
    /// Output is byte-deterministic.
    pub fn save(&self, dir: &Path) -> Result<(), WorldError> {
        std::fs::create_dir_all(dir).map_err(|e| io_err(dir, &e))?;
        let text = ron::ser::to_string_pretty(&self.manifest, ron::ser::PrettyConfig::default())
            .map_err(|e| WorldError::Manifest(e.to_string()))?;
        let path = dir.join(MANIFEST_FILE);
        std::fs::write(&path, text).map_err(|e| io_err(&path, &e))?;
        let header = FileHeader::new(
            MAGIC_TERRAIN,
            CHUNK_SCHEMA,
            Seed(self.manifest.detail.seed),
            self.manifest.name.clone(),
        );
        for (coord, chunk) in &self.chunks {
            let mut bytes = Vec::new();
            write_snapshot(&mut bytes, &header, chunk)?;
            let path = chunk_path(dir, *coord);
            std::fs::write(&path, bytes).map_err(|e| io_err(&path, &e))?;
        }
        Ok(())
    }

    /// The manifest this terrain was built from.
    pub fn manifest(&self) -> &TerrainManifest {
        &self.manifest
    }

    /// Covered extent `(min, max)` in cm, both inclusive. The maximum is the far edge of
    /// the last chunk, which is a sample point.
    pub fn bounds_cm(&self) -> ((i64, i64), (i64, i64)) {
        let min = self.manifest.min_chunk.origin_cm();
        let (mx, my) = self.manifest.max_chunk.origin_cm();
        (min, (mx + CHUNK_SIZE_CM, my + CHUNK_SIZE_CM))
    }

    /// Bilinear height of the source data at a point, cm; `None` outside the bounds.
    pub fn base_height_cm(&self, x_cm: i64, y_cm: i64) -> Option<i32> {
        let ((min_x, min_y), (max_x, max_y)) = self.bounds_cm();
        let ax = axis_cell(x_cm, min_x, max_x)?;
        let ay = axis_cell(y_cm, min_y, max_y)?;
        let chunk = self.chunks.get(&ChunkCoord {
            cx: ax.chunk,
            cy: ay.chunk,
        })?;
        let h = |dc: usize, dr: usize| i128::from(chunk.sample(ax.cell + dc, ay.cell + dr));
        let (fx, fy) = (ax.frac, ay.frac);
        let w = |a: i64, b: i64| i128::from(a * b);
        let one = 1 << 16;
        let sum = h(0, 0) * w(one - fx, one - fy)
            + h(1, 0) * w(fx, one - fy)
            + h(0, 1) * w(one - fx, fy)
            + h(1, 1) * w(fx, fy);
        i32::try_from(sum >> 32).ok()
    }

    /// Procedural detail at a point, cm. Defined everywhere; callers combine it with
    /// [`Terrain::base_height_cm`] via [`Terrain::height_cm`].
    pub fn detail_cm(&self, x_cm: i64, y_cm: i64) -> i32 {
        self.manifest.detail.eval_cm(x_cm, y_cm)
    }

    /// Ground height (base plus detail) at a surface position, cm.
    ///
    /// `None` outside the bounds and on any level other than the surface (P1 has no others).
    pub fn height_cm(&self, pos: &WorldPos) -> Option<i32> {
        if pos.level != LevelId::SURFACE {
            return None;
        }
        let base = self.base_height_cm(pos.x_cm, pos.y_cm)?;
        Some(base.saturating_add(self.detail_cm(pos.x_cm, pos.y_cm)))
    }

    /// Rise over run from `a` to `b` in per-mille, truncating toward zero.
    /// Zero horizontal distance gives `Some(0)`; `None` if either end has no height.
    pub fn slope_permille(&self, a: &WorldPos, b: &WorldPos) -> Option<i32> {
        let (ha, hb) = (self.height_cm(a)?, self.height_cm(b)?);
        let dist = a.horizontal_distance_cm(b);
        if dist == 0 {
            return Some(0);
        }
        let slope = (i64::from(hb) - i64::from(ha)) * 1000 / dist;
        Some(i32::try_from(slope).unwrap_or(if slope < 0 { i32::MIN } else { i32::MAX }))
    }

    /// Gradient magnitude at `pos` in per-mille, by central differences over ±1,500 cm.
    /// `None` if any of the four probe points lies outside the terrain.
    pub fn gradient_permille(&self, pos: &WorldPos) -> Option<i32> {
        let at = |dx: i64, dy: i64| {
            self.height_cm(&WorldPos {
                x_cm: pos.x_cm + dx,
                y_cm: pos.y_cm + dy,
                level: pos.level,
            })
            .map(i64::from)
        };
        let span = 2 * GRADIENT_HALF_CM;
        let gx = (at(GRADIENT_HALF_CM, 0)? - at(-GRADIENT_HALF_CM, 0)?) * 1000 / span;
        let gy = (at(0, GRADIENT_HALF_CM)? - at(0, -GRADIENT_HALF_CM)?) * 1000 / span;
        i32::try_from((gx * gx + gy * gy).isqrt()).ok()
    }

    /// Walkability class at `pos`, from [`Terrain::gradient_permille`].
    pub fn surface_class(&self, pos: &WorldPos) -> Option<SurfaceClass> {
        Some(match self.gradient_permille(pos)? {
            ..30 => SurfaceClass::Flat,
            30..120 => SurfaceClass::Gentle,
            120..400 => SurfaceClass::Steep,
            _ => SurfaceClass::Cliff,
        })
    }
}

/// Locates `v` on one axis. The far bound belongs to the last cell with fraction 1.
fn axis_cell(v: i64, min: i64, max: i64) -> Option<AxisCell> {
    if v < min || v > max {
        return None;
    }
    let (v, last) = if v == max { (v - 1, true) } else { (v, false) };
    let cell = v.div_euclid(BASE_SPACING_CM);
    let frac = if last {
        1 << 16
    } else {
        (v.rem_euclid(BASE_SPACING_CM) << 16) / BASE_SPACING_CM
    };
    Some(AxisCell {
        chunk: i32::try_from(cell.div_euclid(CHUNK_INTERVALS)).ok()?,
        cell: usize::try_from(cell.rem_euclid(CHUNK_INTERVALS)).ok()?,
        frac,
    })
}

fn chunk_path(dir: &Path, c: ChunkCoord) -> PathBuf {
    dir.join(format!("chunk_{}_{}.bin", c.cx, c.cy))
}

fn io_err(path: &Path, e: &std::io::Error) -> WorldError {
    WorldError::Io {
        path: path.display().to_string(),
        message: e.to_string(),
    }
}

/// Reads one chunk file and checks it against the manifest.
fn read_chunk(
    dir: &Path,
    manifest: &TerrainManifest,
    coord: ChunkCoord,
) -> Result<HeightChunk, WorldError> {
    let bad = |reason| WorldError::BadChunk {
        cx: coord.cx,
        cy: coord.cy,
        reason,
    };
    let path = chunk_path(dir, coord);
    let mut bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(WorldError::MissingChunk {
                cx: coord.cx,
                cy: coord.cy,
            });
        }
        Err(e) => return Err(io_err(&path, &e)),
    };
    // `read_snapshot` is the only public framed reader and insists on the snapshot magic.
    // Validate the real terrain header first, then present the same bytes as a snapshot.
    read_header(&mut bytes.as_slice(), MAGIC_TERRAIN)?;
    bytes[..4].copy_from_slice(&MAGIC_SNAPSHOT);
    let (header, chunk): (FileHeader, HeightChunk) = read_snapshot(&mut bytes.as_slice())?;
    if header.seed != Seed(manifest.detail.seed) || header.label != manifest.name {
        return Err(bad("header seed or label differs from manifest"));
    }
    if chunk.coord != coord {
        return Err(bad("coordinate differs from file name"));
    }
    if chunk.heights_cm.len() != SAMPLES_PER_SIDE * SAMPLES_PER_SIDE {
        return Err(bad("wrong sample count"));
    }
    Ok(chunk)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detail::DetailParams;
    use crate::manifest::SourceInfo;
    use proptest::prelude::*;

    fn manifest(min: (i32, i32), max: (i32, i32), detail: DetailParams) -> TerrainManifest {
        TerrainManifest {
            name: "test".into(),
            min_chunk: ChunkCoord {
                cx: min.0,
                cy: min.1,
            },
            max_chunk: ChunkCoord {
                cx: max.0,
                cy: max.1,
            },
            detail,
            source: SourceInfo {
                dataset: "synthetic".into(),
                tiles: vec![("a.tif".into(), "00ff".into())],
                transform: "none".into(),
                tool_version: "0".into(),
            },
        }
    }

    fn no_detail() -> DetailParams {
        DetailParams {
            amplitude_cm: 0,
            ..DetailParams::default()
        }
    }

    fn plane(x: i64, y: i64) -> i32 {
        (2 * x / 1000 + 5 * y / 1000) as i32
    }

    fn plane_terrain() -> Terrain {
        Terrain::from_fn(manifest((-1, -1), (1, 1), no_detail()), plane)
    }

    fn pos(x: i64, y: i64) -> WorldPos {
        WorldPos {
            x_cm: x,
            y_cm: y,
            level: LevelId::SURFACE,
        }
    }

    #[test]
    fn plane_exact_at_samples() {
        let t = plane_terrain();
        for (x, y) in [
            (0, 0),
            (3_000, 6_000),
            (-192_000, -192_000),
            (-3_000, 189_000),
        ] {
            assert_eq!(t.base_height_cm(x, y), Some(plane(x, y)));
        }
    }

    #[test]
    fn plane_within_1cm_everywhere() {
        let t = plane_terrain();
        let ((x0, y0), (x1, y1)) = t.bounds_cm();
        for (x, y) in [
            (x0, y0),
            (x1, y1),
            (x1, y0),
            (x0, y1),
            (1, 1),
            (-4_321, 12_345),
            (190_001, -777),
        ] {
            let exact = (2 * x + 5 * y).div_euclid(1000);
            let got = i64::from(t.base_height_cm(x, y).unwrap());
            assert!((got - exact).abs() <= 1, "({x},{y}): {got} vs {exact}");
        }
    }

    #[test]
    fn bounds_and_outside() {
        let t = plane_terrain();
        assert_eq!(t.bounds_cm(), ((-192_000, -192_000), (384_000, 384_000)));
        assert!(t.base_height_cm(384_000, 384_000).is_some());
        assert_eq!(t.base_height_cm(384_001, 0), None);
        assert_eq!(t.base_height_cm(0, -192_001), None);
    }

    #[test]
    fn other_levels_have_no_height() {
        let t = plane_terrain();
        let p = WorldPos {
            level: LevelId(1),
            ..pos(0, 0)
        };
        assert_eq!(t.height_cm(&p), None);
    }

    #[test]
    fn height_is_base_plus_detail() {
        let t = Terrain::from_fn(manifest((0, 0), (0, 0), DetailParams::default()), plane);
        let (x, y) = (50_123, 77_456);
        let sum = t.base_height_cm(x, y).unwrap() + t.detail_cm(x, y);
        assert_eq!(t.height_cm(&pos(x, y)), Some(sum));
    }

    #[test]
    fn gradient_of_plane_is_five_permille() {
        let t = plane_terrain();
        let g = t.gradient_permille(&pos(10_000, 20_000)).unwrap();
        assert!((g - 5).abs() <= 1, "{g}");
        assert_eq!(
            t.surface_class(&pos(10_000, 20_000)),
            Some(SurfaceClass::Flat)
        );
        assert_eq!(t.gradient_permille(&pos(-192_000, 0)), None);
    }

    #[test]
    fn slope_cases() {
        let t = plane_terrain();
        assert_eq!(t.slope_permille(&pos(5, 5), &pos(5, 5)), Some(0));
        // East one cell on a 2 ‰ plane: +6 cm per 3000 cm.
        assert_eq!(t.slope_permille(&pos(0, 0), &pos(3000, 0)), Some(2));
        assert_eq!(t.slope_permille(&pos(3000, 0), &pos(0, 0)), Some(-2));
        assert_eq!(t.slope_permille(&pos(0, 0), &pos(10_000_000, 0)), None);
    }

    #[test]
    fn classes_by_slope() {
        let steep = |k: i64| {
            Terrain::from_fn(manifest((0, 0), (0, 0), no_detail()), move |x, _| {
                (x * k / 1000) as i32
            })
        };
        let class = |k| steep(k).surface_class(&pos(90_000, 90_000)).unwrap();
        assert_eq!(class(29), SurfaceClass::Flat);
        assert_eq!(class(30), SurfaceClass::Gentle);
        assert_eq!(class(120), SurfaceClass::Steep);
        assert_eq!(class(400), SurfaceClass::Cliff);
    }

    #[test]
    fn save_load_round_trip_is_byte_identical() {
        let dir = std::env::temp_dir().join(format!("ach_world_rt_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let t = Terrain::from_fn(
            manifest((0, -1), (1, 0), DetailParams::default()),
            |x, y| ((x ^ y) % 997) as i32,
        );
        t.save(&dir.join("a")).unwrap();
        let back = Terrain::load(&dir.join("a")).unwrap();
        assert_eq!(back, t);
        back.save(&dir.join("b")).unwrap();
        let mut names: Vec<_> = std::fs::read_dir(dir.join("a"))
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        names.sort();
        assert_eq!(names.len(), 5);
        for n in names {
            assert_eq!(
                std::fs::read(dir.join("a").join(&n)).unwrap(),
                std::fs::read(dir.join("b").join(&n)).unwrap(),
                "{n:?}"
            );
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_reports_missing_chunk_and_bad_magic() {
        let dir = std::env::temp_dir().join(format!("ach_world_bad_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        plane_terrain().save(&dir).unwrap();
        let victim = dir.join("chunk_0_0.bin");
        let good = std::fs::read(&victim).unwrap();
        std::fs::remove_file(&victim).unwrap();
        assert_eq!(
            Terrain::load(&dir).unwrap_err(),
            WorldError::MissingChunk { cx: 0, cy: 0 }
        );
        let mut bad = good;
        bad[0] = b'X';
        std::fs::write(&victim, bad).unwrap();
        assert!(matches!(Terrain::load(&dir), Err(WorldError::Format(_))));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    proptest! {
        #[test]
        fn base_continuous_across_chunk_edges(
            // Straddle the x = 0 and y = 0 chunk boundaries.
            dx in -200i64..200, y in -190_000i64..380_000, dy in -200i64..200, x in -190_000i64..380_000,
        ) {
            let t = Terrain::from_fn(manifest((-1, -1), (1, 1), no_detail()), |x, y| {
                // Smooth surface with slope well under 1 cm per cm.
                (x / 50 - y / 70 + (x / 1000) * (y / 1000) / 100) as i32
            });
            let h = |x, y| i64::from(t.base_height_cm(x, y).unwrap());
            prop_assert!((h(dx, y) - h(dx + 1, y)).abs() <= 2);
            prop_assert!((h(x, dy) - h(x, dy + 1)).abs() <= 2);
        }
    }
}
