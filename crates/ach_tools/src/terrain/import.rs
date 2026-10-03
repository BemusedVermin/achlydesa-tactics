//! `ach terrain import`: real-world elevation tiles to the game's chunked terrain.
//! Implements Execution Plan §4.5 (terrain source) and Technical Design §21.1 (storage model),
//! with the window transform of `content/terrain/red_ledger/window.ron`.
//!
//! Floating point is allowed here: `ach_tools` is not a simulation crate. The output is still
//! deterministic because [`ach_world::Terrain::save`] owns the byte format.

use super::source::{Mosaic, Tile};
use ach_core::fnv1a64;
use ach_world::{
    BASE_SPACING_CM, CHUNK_INTERVALS, CHUNK_SIZE_CM, ChunkCoord, DetailParams, SourceInfo, Terrain,
    TerrainManifest,
};
use anyhow::{Context, Result, bail, ensure};
use clap::ValueEnum;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Mean Earth radius of the equirectangular projection shared with `tools/scripts/hillshade.py`,
/// metres. The sites and routes of `content/world/red_ledger_*.ron` were placed on that projection.
const EARTH_RADIUS_M: f64 = 6_371_008.8;
/// Void-fill neighbourhood radius, samples.
const FILL_RADIUS: i64 = 3;
/// Void-fill passes at most.
const FILL_PASSES: usize = 5;
/// Unfilled voids listed in the error message at most.
const MAX_LISTED_VOIDS: usize = 20;
/// Prefix of the void count recorded in [`SourceInfo::transform`]; read back by `ach terrain info`.
pub const VOID_FILLED_KEY: &str = "void_filled=";

/// Which rectangle of `window.ron` to import.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Region {
    /// The 96 x 24 km Phase 1 strip.
    #[value(name = "strip_p1")]
    StripP1,
    /// The 30 x 30 km Phase 3 corridor.
    #[value(name = "corridor_p3")]
    CorridorP3,
}

impl Region {
    fn name(self) -> &'static str {
        match self {
            Self::StripP1 => "strip_p1",
            Self::CorridorP3 => "corridor_p3",
        }
    }
}

/// Axis-aligned rectangle in the local frame, metres.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Rect {
    /// South-west corner.
    pub min: (i64, i64),
    /// North-east corner.
    pub max: (i64, i64),
}

/// Mirror then rotate, as written in `window.ron`.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Transform {
    /// Counter-clockwise rotation in degrees; a multiple of 90.
    pub rotate_deg: u32,
    /// Flip the real window west-east before rotating.
    pub mirror_x: bool,
}

/// The subset of `window.ron` the importer reads.
#[derive(Clone, Debug, Deserialize)]
pub struct Window {
    /// Dataset name.
    pub source: String,
    /// Source tile file names.
    pub tiles: Vec<String>,
    /// South-west corner of the real window, `(lat, lon)` degrees.
    pub sw_lat_lon: (f64, f64),
    /// Real window extent, `(east, north)` metres.
    pub extent_m: (u32, u32),
    /// Applied to the real window to give the local frame.
    pub transform: Transform,
    /// Phase 1 strip.
    pub strip_p1: Rect,
    /// Phase 3 corridor.
    pub corridor_p3: Rect,
}

impl Window {
    /// Parses `window.ron`.
    pub fn load(path: &Path) -> Result<Self> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        ron::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    fn rect(&self, region: Region) -> Rect {
        match region {
            Region::StripP1 => self.strip_p1,
            Region::CorridorP3 => self.corridor_p3,
        }
    }
}

/// Maps local-frame metres back to window-local metres (origin at the real south-west corner).
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    /// Real window extent `(east, north)`, metres.
    window: (f64, f64),
    rotate_quarters: u32,
    mirror_x: bool,
}

impl Frame {
    /// Builds the frame of a window; rejects rotations that are not multiples of 90 degrees.
    pub fn new(window: &Window) -> Result<Self> {
        let deg = window.transform.rotate_deg;
        ensure!(
            deg.is_multiple_of(90),
            "rotate_deg {deg} is not a multiple of 90"
        );
        Ok(Self {
            window: (f64::from(window.extent_m.0), f64::from(window.extent_m.1)),
            rotate_quarters: (deg / 90) % 4,
            mirror_x: window.transform.mirror_x,
        })
    }

    /// Extent of the local frame `(east, north)`, metres: the window, swapped by odd quarter turns.
    pub fn local_extent(&self) -> (f64, f64) {
        if self.rotate_quarters % 2 == 1 {
            (self.window.1, self.window.0)
        } else {
            self.window
        }
    }

    /// Inverts the transform: undo the rotation about the window centre, then the mirror.
    pub fn to_window(&self, x: f64, y: f64) -> (f64, f64) {
        let (lw, lh) = self.local_extent();
        let (mut dx, mut dy) = (x - lw / 2.0, y - lh / 2.0);
        for _ in 0..self.rotate_quarters {
            (dx, dy) = (dy, -dx); // one quarter turn clockwise
        }
        if self.mirror_x {
            dx = -dx;
        }
        (dx + self.window.0 / 2.0, dy + self.window.1 / 2.0)
    }

    /// Applies the transform: mirror, then rotate counter-clockwise about the window centre.
    #[cfg(test)]
    fn forward(&self, u: f64, v: f64) -> (f64, f64) {
        let (lw, lh) = self.local_extent();
        let (mut dx, dy0) = (u - self.window.0 / 2.0, v - self.window.1 / 2.0);
        let mut dy = dy0;
        if self.mirror_x {
            dx = -dx;
        }
        for _ in 0..self.rotate_quarters {
            (dx, dy) = (-dy, dx); // one quarter turn counter-clockwise
        }
        (dx + lw / 2.0, dy + lh / 2.0)
    }
}

/// Equirectangular projection about the real window's central latitude (as `hillshade.py`).
#[derive(Clone, Copy, Debug)]
struct Projection {
    sw_lat: f64,
    sw_lon: f64,
    cos_mid: f64,
}

impl Projection {
    fn new(window: &Window) -> Self {
        let (sw_lat, sw_lon) = window.sw_lat_lon;
        let lat_max = sw_lat + (f64::from(window.extent_m.1) / EARTH_RADIUS_M).to_degrees();
        Self {
            sw_lat,
            sw_lon,
            cos_mid: ((sw_lat + lat_max) / 2.0).to_radians().cos(),
        }
    }

    /// `(lat, lon)` in degrees of window-local metres.
    fn lat_lon(&self, u: f64, v: f64) -> (f64, f64) {
        (
            self.sw_lat + (v / EARTH_RADIUS_M).to_degrees(),
            self.sw_lon + (u / (EARTH_RADIUS_M * self.cos_mid)).to_degrees(),
        )
    }
}

/// Chunk range covering `rect` rounded outward to whole chunks. A far edge that lies exactly
/// on a chunk boundary does not pull in the next chunk.
fn chunk_range(rect: Rect) -> Result<(ChunkCoord, ChunkCoord)> {
    let size_m = CHUNK_SIZE_CM / 100;
    let lo = |m: i64| i32::try_from(m.div_euclid(size_m)).context("region out of range");
    let hi = |m: i64| {
        i32::try_from((m + size_m - 1).div_euclid(size_m) - 1).context("region out of range")
    };
    ensure!(
        rect.min.0 < rect.max.0 && rect.min.1 < rect.max.1,
        "region is empty"
    );
    Ok((
        ChunkCoord {
            cx: lo(rect.min.0)?,
            cy: lo(rect.min.1)?,
        },
        ChunkCoord {
            cx: hi(rect.max.0)?,
            cy: hi(rect.max.1)?,
        },
    ))
}

/// Heights in metres over the sample lattice of a chunk range; `None` marks a void.
struct Grid {
    /// Lattice index of the south-west sample.
    origin: (i64, i64),
    nx: usize,
    ny: usize,
    cells: Vec<Option<f64>>,
}

impl Grid {
    fn new(
        min: ChunkCoord,
        max: ChunkCoord,
        mut sample: impl FnMut(f64, f64) -> Result<Option<f64>>,
    ) -> Result<Self> {
        let spacing_m = (BASE_SPACING_CM / 100) as f64;
        let origin = (
            i64::from(min.cx) * CHUNK_INTERVALS,
            i64::from(min.cy) * CHUNK_INTERVALS,
        );
        let nx = ((i64::from(max.cx - min.cx) + 1) * CHUNK_INTERVALS + 1) as usize;
        let ny = ((i64::from(max.cy - min.cy) + 1) * CHUNK_INTERVALS + 1) as usize;
        let mut cells = Vec::with_capacity(nx * ny);
        for j in 0..ny {
            for i in 0..nx {
                let x = (origin.0 + i as i64) as f64 * spacing_m;
                let y = (origin.1 + j as i64) as f64 * spacing_m;
                cells.push(sample(x, y)?);
            }
        }
        Ok(Self {
            origin,
            nx,
            ny,
            cells,
        })
    }

    fn at(&self, i: usize, j: usize) -> Option<f64> {
        self.cells[j * self.nx + i]
    }

    /// Mean of the valid cells within [`FILL_RADIUS`] of `(i, j)`, if any.
    fn neighbourhood_mean(&self, i: usize, j: usize) -> Option<f64> {
        let r = FILL_RADIUS;
        let (mut sum, mut n) = (0.0, 0_u32);
        for dj in -r..=r {
            for di in -r..=r {
                if di * di + dj * dj > r * r {
                    continue;
                }
                let (ii, jj) = (i as i64 + di, j as i64 + dj);
                if ii < 0 || jj < 0 || ii >= self.nx as i64 || jj >= self.ny as i64 {
                    continue;
                }
                if let Some(v) = self.at(ii as usize, jj as usize) {
                    sum += v;
                    n += 1;
                }
            }
        }
        (n > 0).then(|| sum / f64::from(n))
    }

    /// Fills voids from their valid neighbours, up to [`FILL_PASSES`] passes; each pass reads
    /// the state at its start. Returns the number of samples filled.
    fn fill_voids(&mut self) -> usize {
        let mut filled = 0;
        for _ in 0..FILL_PASSES {
            let updates: Vec<(usize, f64)> = (0..self.ny)
                .flat_map(|j| (0..self.nx).map(move |i| (i, j)))
                .filter(|&(i, j)| self.at(i, j).is_none())
                .filter_map(|(i, j)| Some((j * self.nx + i, self.neighbourhood_mean(i, j)?)))
                .collect();
            if updates.is_empty() {
                break;
            }
            filled += updates.len();
            for (idx, v) in updates {
                self.cells[idx] = Some(v);
            }
        }
        filled
    }

    /// Local-frame coordinates in metres of every remaining void, listing at most a few.
    fn void_report(&self) -> Option<String> {
        let spacing = BASE_SPACING_CM / 100;
        let voids: Vec<(i64, i64)> = (0..self.ny)
            .flat_map(|j| (0..self.nx).map(move |i| (i, j)))
            .filter(|&(i, j)| self.at(i, j).is_none())
            .map(|(i, j)| {
                (
                    (self.origin.0 + i as i64) * spacing,
                    (self.origin.1 + j as i64) * spacing,
                )
            })
            .collect();
        (!voids.is_empty()).then(|| {
            let shown: Vec<String> = voids
                .iter()
                .take(MAX_LISTED_VOIDS)
                .map(|(x, y)| format!("({x}, {y})"))
                .collect();
            format!(
                "{} unfilled voids (local metres): {}",
                voids.len(),
                shown.join(", ")
            )
        })
    }

    /// Height in cm at lattice index `(ix, iy)`; the caller guarantees it is filled.
    fn height_cm(&self, ix: i64, iy: i64) -> i32 {
        let (i, j) = ((ix - self.origin.0) as usize, (iy - self.origin.1) as usize);
        let m = self
            .at(i, j)
            .expect("invariant: voids were reported before lookup");
        (m * 100.0).round() as i32
    }
}

fn load_mosaic(window: &Window, raw: &Path) -> Result<(Mosaic, Vec<(String, String)>)> {
    let mut tiles = Vec::new();
    let mut hashes = Vec::new();
    for name in &window.tiles {
        let path = raw.join(name);
        let bytes = std::fs::read(&path).with_context(|| {
            format!(
                "reading source tile {} (see ADR-0100 Download commands)",
                path.display()
            )
        })?;
        hashes.push((name.clone(), format!("{:016x}", fnv1a64(&bytes))));
        let file = File::open(&path).with_context(|| format!("opening {}", path.display()))?;
        tiles.push(Tile::read(BufReader::new(file)).with_context(|| format!("decoding {name}"))?);
    }
    Ok((Mosaic::new(tiles)?, hashes))
}

/// Arguments of `ach terrain import`.
#[derive(clap::Args, Debug)]
pub struct ImportArgs {
    /// `window.ron` describing tiles, window and regions.
    #[arg(long)]
    window: std::path::PathBuf,
    /// Region of the window to import.
    #[arg(long)]
    region: Region,
    /// Directory holding the raw source tiles.
    #[arg(long)]
    raw: std::path::PathBuf,
    /// Output terrain directory.
    #[arg(long)]
    out: std::path::PathBuf,
}

/// Runs the import and writes the terrain directory.
pub fn run(args: &ImportArgs) -> Result<()> {
    let window = Window::load(&args.window)?;
    let frame = Frame::new(&window)?;
    let projection = Projection::new(&window);
    let (mosaic, hashes) = load_mosaic(&window, &args.raw)?;
    let (min_chunk, max_chunk) = chunk_range(window.rect(args.region))?;
    let (lw, lh) = frame.local_extent();
    let mut grid = Grid::new(min_chunk, max_chunk, |x, y| {
        ensure!(
            (0.0..=lw).contains(&x) && (0.0..=lh).contains(&y),
            "sample ({x}, {y}) m lies outside the {lw} x {lh} m window"
        );
        let (u, v) = frame.to_window(x, y);
        let (lat, lon) = projection.lat_lon(u, v);
        mosaic.sample(lat, lon)
    })?;
    let filled = grid.fill_voids();
    if let Some(report) = grid.void_report() {
        bail!("{report}");
    }
    let manifest = TerrainManifest {
        name: format!("red_ledger_{}", args.region.name()),
        min_chunk,
        max_chunk,
        detail: DetailParams::default(),
        source: SourceInfo {
            dataset: window.source.clone(),
            tiles: hashes,
            transform: transform_text(&window, args.region, filled),
            tool_version: env!("CARGO_PKG_VERSION").to_owned(),
        },
    };
    let spacing = BASE_SPACING_CM;
    let terrain = Terrain::from_fn(manifest, |x_cm, y_cm| {
        grid.height_cm(x_cm / spacing, y_cm / spacing)
    });
    terrain.save(&args.out)?;
    println!(
        "wrote {} chunks to {} ({filled} void samples filled)",
        terrain.manifest().coords().count(),
        args.out.display()
    );
    Ok(())
}

fn transform_text(window: &Window, region: Region, filled: usize) -> String {
    let t = window.transform;
    format!(
        "region={}; window sw_lat_lon=({}, {}) extent_m=({}, {}); mirror_x={} then rotate_deg={} ccw about centre; \
         equirectangular R={EARTH_RADIUS_M} m at central latitude; bilinear; {VOID_FILLED_KEY}{filled}",
        region.name(),
        window.sw_lat_lon.0,
        window.sw_lat_lon.1,
        window.extent_m.0,
        window.extent_m.1,
        t.mirror_x,
        t.rotate_deg,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn window(rotate_deg: u32, mirror_x: bool, extent_m: (u32, u32)) -> Window {
        Window {
            source: "test".into(),
            tiles: vec![],
            sw_lat_lon: (30.0, 34.6),
            extent_m,
            transform: Transform {
                rotate_deg,
                mirror_x,
            },
            strip_p1: Rect {
                min: (0, 0),
                max: (1, 1),
            },
            corridor_p3: Rect {
                min: (0, 0),
                max: (1, 1),
            },
        }
    }

    #[test]
    fn transform_inverse_round_trips_for_all_eight_combinations() {
        for rotate_deg in [0, 90, 180, 270] {
            for mirror_x in [false, true] {
                let f = Frame::new(&window(rotate_deg, mirror_x, (150_000, 90_000))).unwrap();
                for (u, v) in [(0.0, 0.0), (150_000.0, 90_000.0), (12_345.0, 67_890.0)] {
                    let (x, y) = f.forward(u, v);
                    let (u2, v2) = f.to_window(x, y);
                    assert!(
                        (u - u2).abs() < 1e-6 && (v - v2).abs() < 1e-6,
                        "rot {rotate_deg} mirror {mirror_x}: ({u},{v}) -> ({u2},{v2})"
                    );
                    let (lw, lh) = f.local_extent();
                    assert!((-1e-6..=lw + 1e-6).contains(&x) && (-1e-6..=lh + 1e-6).contains(&y));
                }
            }
        }
    }

    #[test]
    fn rotate_270_turns_real_north_into_game_east() {
        let f = Frame::new(&window(270, false, (1000, 1000))).unwrap();
        // Real north becomes east: the real north-centre is the game east-centre.
        assert_eq!(f.forward(500.0, 1000.0), (1000.0, 500.0));
        // Real east becomes game south.
        assert_eq!(f.forward(1000.0, 500.0), (500.0, 0.0));
    }

    #[test]
    fn mirror_flips_west_east_before_rotation() {
        let f = Frame::new(&window(0, true, (1000, 1000))).unwrap();
        assert_eq!(f.forward(100.0, 200.0), (900.0, 200.0));
        assert!(Frame::new(&window(45, false, (1000, 1000))).is_err());
    }

    #[test]
    fn chunk_range_rounds_outward() {
        let (lo, hi) = chunk_range(Rect {
            min: (6_000, 74_000),
            max: (102_000, 98_000),
        })
        .unwrap();
        assert_eq!((lo.cx, lo.cy, hi.cx, hi.cy), (3, 38, 53, 51));
        // A far edge on a chunk boundary does not add a chunk.
        let (_, hi) = chunk_range(Rect {
            min: (0, 0),
            max: (3_840, 1_920),
        })
        .unwrap();
        assert_eq!((hi.cx, hi.cy), (1, 0));
    }

    fn grid_with_voids(voids: &[(usize, usize)]) -> Grid {
        let (nx, ny) = (9, 9);
        let cells = (0..nx * ny)
            .map(|k| (!voids.contains(&(k % nx, k / nx))).then_some(10.0))
            .collect();
        Grid {
            origin: (0, 0),
            nx,
            ny,
            cells,
        }
    }

    #[test]
    fn voids_fill_with_neighbour_mean_in_passes() {
        let mut g = grid_with_voids(&[(4, 4)]);
        assert_eq!(g.fill_voids(), 1);
        assert_eq!(g.at(4, 4), Some(10.0));
        // A 7x7 block fills in two passes: its centre is four samples from the valid rim.
        let block: Vec<_> = (1..8).flat_map(|j| (1..8).map(move |i| (i, j))).collect();
        let mut g = grid_with_voids(&block);
        assert_eq!(g.fill_voids(), 49);
        assert!(g.void_report().is_none());
    }

    #[test]
    fn unreachable_voids_are_reported_with_coordinates() {
        let all: Vec<_> = (0..9).flat_map(|j| (0..9).map(move |i| (i, j))).collect();
        let mut g = grid_with_voids(&all);
        assert_eq!(g.fill_voids(), 0);
        let report = g.void_report().unwrap();
        assert!(report.starts_with("81 unfilled voids"), "{report}");
        assert!(report.contains("(0, 0)"), "{report}");
    }
}
