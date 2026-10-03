//! `ach terrain info`: summary of a terrain directory.
//! Implements Technical Design §21.1 (storage model) for the `ach` CLI.

use super::import::VOID_FILLED_KEY;
use ach_world::{BASE_SPACING_CM, CHUNK_SIZE_CM, Terrain};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Arguments of `ach terrain info`.
#[derive(clap::Args, Debug)]
pub struct InfoArgs {
    /// Terrain directory written by `ach terrain import`.
    dir: PathBuf,
}

/// Base-elevation statistics over every sample point, metres.
struct Stats {
    min: f64,
    max: f64,
    mean: f64,
}

fn stats(terrain: &Terrain) -> Stats {
    let ((x0, y0), (x1, y1)) = terrain.bounds_cm();
    let (mut min, mut max, mut sum, mut n) = (i32::MAX, i32::MIN, 0_i128, 0_i128);
    for y in (y0..=y1).step_by(BASE_SPACING_CM as usize) {
        for x in (x0..=x1).step_by(BASE_SPACING_CM as usize) {
            if let Some(h) = terrain.base_height_cm(x, y) {
                (min, max, sum, n) = (min.min(h), max.max(h), sum + i128::from(h), n + 1);
            }
        }
    }
    Stats {
        min: f64::from(min) / 100.0,
        max: f64::from(max) / 100.0,
        mean: sum as f64 / n.max(1) as f64 / 100.0,
    }
}

/// The void-fill count recorded by the importer in the transform text.
fn void_filled(transform: &str) -> Option<&str> {
    let rest = &transform[transform.find(VOID_FILLED_KEY)? + VOID_FILLED_KEY.len()..];
    rest.split(|c: char| !c.is_ascii_digit()).next()
}

/// Formats the report for the terrain in `dir`.
pub fn report(dir: &Path) -> Result<String> {
    let terrain = Terrain::load(dir).with_context(|| format!("loading {}", dir.display()))?;
    let m = terrain.manifest();
    let (lo, hi) = (m.min_chunk, m.max_chunk);
    let chunks = (i64::from(hi.cx - lo.cx) + 1, i64::from(hi.cy - lo.cy) + 1);
    let km = |n: i64| (n * CHUNK_SIZE_CM) as f64 / 100_000.0;
    let s = stats(&terrain);
    let mut out = format!(
        "name: {}\nchunks: x {}..={}, y {}..={} ({} x {})\nsize: {:.2} x {:.2} km\n\
         base elevation m: min {:.2}, max {:.2}, mean {:.2}\nvoid samples filled: {}\n\
         dataset: {}\ntool version: {}\ntransform: {}\ntiles:\n",
        m.name,
        lo.cx,
        hi.cx,
        lo.cy,
        hi.cy,
        chunks.0,
        chunks.1,
        km(chunks.0),
        km(chunks.1),
        s.min,
        s.max,
        s.mean,
        void_filled(&m.source.transform).unwrap_or("unknown"),
        m.source.dataset,
        m.source.tool_version,
        m.source.transform,
    );
    for (name, hash) in &m.source.tiles {
        out.push_str(&format!("  {name} fnv1a64 {hash}\n"));
    }
    Ok(out)
}

/// Runs `ach terrain info`.
pub fn run(args: &InfoArgs) -> Result<()> {
    print!("{}", report(&args.dir)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn void_count_is_read_from_transform_text() {
        assert_eq!(void_filled("a; void_filled=12"), Some("12"));
        assert_eq!(void_filled("void_filled=0; x"), Some("0"));
        assert_eq!(void_filled("nothing"), None);
    }
}
