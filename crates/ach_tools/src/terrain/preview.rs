//! `ach terrain preview`: hillshaded PNG of a terrain directory with sites and routes on top.
//! Implements Field Atlas §3.1 (day palette tokens) for the terrain fills; hillshade follows
//! `tools/scripts/hillshade.py` (azimuth 315 degrees, altitude 45 degrees), so the output can be
//! compared with `docs/canon/red_ledger_region.png`.

use ach_world::Terrain;
use anyhow::{Context, Result, ensure};
use image::{Rgb, RgbImage};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Hillshade light azimuth, degrees clockwise from north.
const AZIMUTH_DEG: f64 = 315.0;
/// Hillshade light altitude, degrees.
const ALTITUDE_DEG: f64 = 45.0;
/// Lower bound and upper bound of the shading factor applied to the tint.
const SHADE_RANGE: (f64, f64) = (0.2, 1.15);
/// Strength of the shading before clamping.
const SHADE_GAIN: f64 = 0.85;
/// Half the side of a site marker, pixels.
const MARKER_HALF_PX: i64 = 3;

// Field Atlas §3.1 day palette.
const LOW_GROUND: [f64; 3] = [221.0, 210.0, 184.0];
const MID_GROUND: [f64; 3] = [187.0, 187.0, 164.0];
const HIGH_GROUND: [f64; 3] = [156.0, 151.0, 155.0];
const INK: Rgb<u8> = Rgb([0x35, 0x2F, 0x3F]);
const SECONDARY: Rgb<u8> = Rgb([0x65, 0x5B, 0x68]);
const FRIENDLY: Rgb<u8> = Rgb([0x23, 0x69, 0x65]);
const CAUTION: Rgb<u8> = Rgb([0x73, 0x50, 0x08]);

/// Arguments of `ach terrain preview`.
#[derive(clap::Args, Debug)]
pub struct PreviewArgs {
    /// Terrain directory written by `ach terrain import`.
    dir: PathBuf,
    /// Output PNG.
    #[arg(long)]
    out: PathBuf,
    /// Ground metres covered by one pixel.
    #[arg(long, default_value_t = 60)]
    scale_m_per_px: u32,
    /// Sites file drawn on top.
    #[arg(long, default_value = "content/world/red_ledger_sites.ron")]
    sites: PathBuf,
    /// Routes file drawn on top.
    #[arg(long, default_value = "content/world/red_ledger_routes.ron")]
    routes: PathBuf,
}

/// The part of `red_ledger_sites.ron` the preview draws.
#[derive(Deserialize)]
struct SitesFile {
    sites: Vec<Site>,
}

#[derive(Deserialize)]
struct Site {
    pos: (i64, i64),
}

/// The part of `red_ledger_routes.ron` the preview draws.
#[derive(Deserialize)]
struct RoutesFile {
    routes: Vec<Route>,
}

#[derive(Deserialize)]
struct Route {
    kind: RouteKind,
    vertices: Vec<(i64, i64)>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum RouteKind {
    Road,
    Track,
    Approach,
}

fn load_ron<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    ron::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

/// Row-major heights in metres, row 0 northmost, one per pixel.
struct Raster {
    w: usize,
    h: usize,
    heights: Vec<f64>,
}

impl Raster {
    fn sample(terrain: &Terrain, scale_m: i64) -> Result<Self> {
        let ((x0, y0), (x1, y1)) = terrain.bounds_cm();
        let step = scale_m * 100;
        let (w, h) = (((x1 - x0) / step) as usize, ((y1 - y0) / step) as usize);
        ensure!(
            w > 1 && h > 1,
            "scale of {scale_m} m per pixel is too coarse"
        );
        let mut heights = Vec::with_capacity(w * h);
        for row in 0..h {
            let y = y1 - (2 * row as i64 + 1) * step / 2;
            for col in 0..w {
                let x = x0 + (2 * col as i64 + 1) * step / 2;
                let cm = terrain
                    .base_height_cm(x, y)
                    .context("invariant: pixel centre lies inside the bounds")?;
                heights.push(f64::from(cm) / 100.0);
            }
        }
        Ok(Self { w, h, heights })
    }

    fn at(&self, col: usize, row: usize) -> f64 {
        self.heights[row * self.w + col]
    }

    /// Central-difference (one-sided at the edge) gradient per metre: (east, north).
    fn gradient(&self, col: usize, row: usize, cell_m: f64) -> (f64, f64) {
        let (c0, c1) = (col.saturating_sub(1), (col + 1).min(self.w - 1));
        let (r0, r1) = (row.saturating_sub(1), (row + 1).min(self.h - 1));
        let dx = (self.at(c1, row) - self.at(c0, row)) / ((c1 - c0) as f64 * cell_m);
        // Rows run north to south.
        let dy = (self.at(col, r0) - self.at(col, r1)) / ((r1 - r0) as f64 * cell_m);
        (dx, dy)
    }

    /// Lambertian illumination in 0..=1 from the fixed light.
    fn shade(&self, col: usize, row: usize, cell_m: f64) -> f64 {
        let (dx, dy) = self.gradient(col, row, cell_m);
        let nz = 1.0 / (dx * dx + dy * dy + 1.0).sqrt();
        let (nx, ny) = (-dx * nz, -dy * nz);
        let (az, alt) = (AZIMUTH_DEG.to_radians(), ALTITUDE_DEG.to_radians());
        let (lx, ly, lz) = (az.sin() * alt.cos(), az.cos() * alt.cos(), alt.sin());
        (nx * lx + ny * ly + nz * lz).clamp(0.0, 1.0)
    }

    /// The 5th, 50th and 95th percentile heights, which anchor the elevation tint.
    fn tint_stops(&self) -> [f64; 3] {
        let mut sorted = self.heights.clone();
        sorted.sort_by(f64::total_cmp);
        let at = |p: f64| sorted[((sorted.len() - 1) as f64 * p).round() as usize];
        [at(0.05), at(0.5), at(0.95)]
    }
}

/// Piecewise-linear ground tint between the three palette stops.
fn tint(z: f64, stops: [f64; 3]) -> [f64; 3] {
    let blend =
        |a: [f64; 3], b: [f64; 3], t: f64| std::array::from_fn(|i| a[i] + (b[i] - a[i]) * t);
    let frac = |lo: f64, hi: f64| {
        if hi > lo {
            ((z - lo) / (hi - lo)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    };
    if z < stops[1] {
        blend(LOW_GROUND, MID_GROUND, frac(stops[0], stops[1]))
    } else {
        blend(MID_GROUND, HIGH_GROUND, frac(stops[1], stops[2]))
    }
}

fn hillshade_image(raster: &Raster, cell_m: f64) -> RgbImage {
    let stops = raster.tint_stops();
    let light = ALTITUDE_DEG.to_radians().sin();
    RgbImage::from_fn(raster.w as u32, raster.h as u32, |c, r| {
        let (c, r) = (c as usize, r as usize);
        let f =
            (SHADE_GAIN * raster.shade(c, r, cell_m) / light).clamp(SHADE_RANGE.0, SHADE_RANGE.1);
        let rgb = tint(raster.at(c, r), stops);
        Rgb(rgb.map(|v| (v * f).clamp(0.0, 255.0) as u8))
    })
}

/// Maps local-frame metres to pixel coordinates, row 0 north.
struct Pixels {
    min_m: (f64, f64),
    height_m: f64,
    scale_m: f64,
}

impl Pixels {
    fn at(&self, x_m: i64, y_m: i64) -> (i64, i64) {
        (
            ((x_m as f64 - self.min_m.0) / self.scale_m).floor() as i64,
            ((self.height_m - (y_m as f64 - self.min_m.1)) / self.scale_m).floor() as i64,
        )
    }
}

fn put(img: &mut RgbImage, x: i64, y: i64, color: Rgb<u8>) {
    if x >= 0 && y >= 0 && x < i64::from(img.width()) && y < i64::from(img.height()) {
        img.put_pixel(x as u32, y as u32, color);
    }
}

/// Bresenham line, clipped by [`put`].
fn line(img: &mut RgbImage, from: (i64, i64), to: (i64, i64), color: Rgb<u8>) {
    let (mut x, mut y) = from;
    let (dx, dy) = ((to.0 - x).abs(), -(to.1 - y).abs());
    let (sx, sy) = ((to.0 - x).signum(), (to.1 - y).signum());
    let mut err = dx + dy;
    loop {
        put(img, x, y, color);
        if (x, y) == to {
            return;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn draw_routes(img: &mut RgbImage, px: &Pixels, routes: &RoutesFile) {
    for route in &routes.routes {
        let color = match route.kind {
            RouteKind::Road => INK,
            RouteKind::Track => SECONDARY,
            RouteKind::Approach => CAUTION,
        };
        for pair in route.vertices.windows(2) {
            line(
                img,
                px.at(pair[0].0, pair[0].1),
                px.at(pair[1].0, pair[1].1),
                color,
            );
        }
    }
}

fn draw_sites(img: &mut RgbImage, px: &Pixels, sites: &SitesFile) {
    for site in &sites.sites {
        let (cx, cy) = px.at(site.pos.0, site.pos.1);
        for dy in -MARKER_HALF_PX..=MARKER_HALF_PX {
            for dx in -MARKER_HALF_PX..=MARKER_HALF_PX {
                let edge = dx.abs() == MARKER_HALF_PX || dy.abs() == MARKER_HALF_PX;
                put(img, cx + dx, cy + dy, if edge { INK } else { FRIENDLY });
            }
        }
    }
}

/// Runs `ach terrain preview`.
pub fn run(args: &PreviewArgs) -> Result<()> {
    ensure!(args.scale_m_per_px > 0, "--scale-m-per-px must be positive");
    let terrain =
        Terrain::load(&args.dir).with_context(|| format!("loading {}", args.dir.display()))?;
    let scale = i64::from(args.scale_m_per_px);
    let raster = Raster::sample(&terrain, scale)?;
    let cell_m = scale as f64;
    let mut img = hillshade_image(&raster, cell_m);
    let ((x0, y0), (_, y1)) = terrain.bounds_cm();
    let px = Pixels {
        min_m: (x0 as f64 / 100.0, y0 as f64 / 100.0),
        height_m: (y1 - y0) as f64 / 100.0,
        scale_m: cell_m,
    };
    draw_routes(&mut img, &px, &load_ron(&args.routes)?);
    draw_sites(&mut img, &px, &load_ron(&args.sites)?);
    if let Some(parent) = args.out.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    img.save(&args.out)
        .with_context(|| format!("writing {}", args.out.display()))?;
    println!(
        "wrote {} x {} px to {}",
        img.width(),
        img.height(),
        args.out.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raster(f: impl Fn(usize, usize) -> f64) -> Raster {
        let (w, h) = (5, 5);
        Raster {
            w,
            h,
            heights: (0..w * h).map(|k| f(k % w, k / w)).collect(),
        }
    }

    #[test]
    fn flat_ground_is_lit_at_the_sun_altitude() {
        let r = raster(|_, _| 100.0);
        let lit = r.shade(2, 2, 60.0);
        assert!((lit - ALTITUDE_DEG.to_radians().sin()).abs() < 1e-12);
    }

    #[test]
    fn north_west_facing_slope_is_brighter_than_south_east_facing() {
        // Height rises toward the south-east, so the surface faces north-west, toward the light.
        let nw_facing = raster(|c, r| 10.0 * (c + r) as f64);
        let se_facing = raster(|c, r| -10.0 * (c + r) as f64);
        assert!(nw_facing.shade(2, 2, 60.0) > se_facing.shade(2, 2, 60.0));
    }

    #[test]
    fn tint_hits_the_palette_stops() {
        let stops = [0.0, 50.0, 100.0];
        assert_eq!(tint(-10.0, stops), LOW_GROUND);
        assert_eq!(tint(50.0, stops), MID_GROUND);
        assert_eq!(tint(500.0, stops), HIGH_GROUND);
    }

    #[test]
    fn line_draws_both_endpoints_and_clips() {
        let mut img = RgbImage::new(4, 4);
        line(&mut img, (-2, 0), (3, 3), INK);
        assert_eq!(*img.get_pixel(3, 3), INK);
        line(&mut img, (0, 0), (0, 9), CAUTION);
        assert_eq!(*img.get_pixel(0, 3), CAUTION);
    }
}
