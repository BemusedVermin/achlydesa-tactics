//! Source reader for Copernicus GLO-30 GeoTIFF tiles (ADR-0100, Accepted option).
//! Implements Execution Plan §4.5 (terrain source) for the `ach terrain import` tool.
//!
//! A tile is a single-band float32 GeoTIFF covering one degree square. Tiles are mosaicked by
//! latitude and longitude; [`Mosaic::sample`] interpolates bilinearly across tile borders.
//! Floating point is allowed here: `ach_tools` is not a simulation crate.

use anyhow::{Context, Result, bail, ensure};
use std::collections::BTreeMap;
use std::io::{Read, Seek};
use tiff::decoder::{Decoder, DecodingResult};
use tiff::tags::Tag;

/// `GTRasterTypeGeoKey` in the GeoKey directory.
const RASTER_TYPE_KEY: u16 = 1025;
/// `RasterPixelIsPoint`: the tie point addresses the pixel centre rather than its corner.
const RASTER_PIXEL_IS_POINT: u16 = 2;
/// Tolerance when comparing tile registration offsets, degrees.
const OFFSET_TOLERANCE_DEG: f64 = 1e-9;

/// One decoded one-degree tile.
#[derive(Debug, Clone)]
pub struct Tile {
    width: usize,
    height: usize,
    /// Western edge of pixel column 0, degrees.
    west: f64,
    /// Northern edge of pixel row 0, degrees.
    north: f64,
    /// Row-major, row 0 northmost.
    heights: Vec<f32>,
    /// Value that marks a void, if the file declares one.
    nodata: Option<f32>,
}

impl Tile {
    /// Decodes a GeoTIFF: float32, single band, with pixel scale and tie point tags.
    ///
    /// The nodata value comes from the `GDAL_NODATA` tag when present. NaN always counts as void.
    pub fn read(reader: impl Read + Seek) -> Result<Self> {
        let mut dec = Decoder::new(reader).context("not a readable TIFF")?;
        let (w, h) = dec.dimensions()?;
        let scale = dec.get_tag_f64_vec(Tag::ModelPixelScaleTag)?;
        let tie = dec.get_tag_f64_vec(Tag::ModelTiepointTag)?;
        ensure!(scale.len() >= 2 && tie.len() >= 6, "malformed geo tags");
        let (dx, dy) = (scale[0], scale[1]);
        let point = dec
            .find_tag(Tag::GeoKeyDirectoryTag)?
            .map(|v| v.into_u16_vec())
            .transpose()?
            .is_some_and(|keys| raster_is_point(&keys));
        let shift = if point { 0.5 } else { 0.0 };
        let nodata = dec
            .find_tag(Tag::GdalNodata)?
            .map(|v| v.into_string())
            .transpose()?
            .and_then(|s| s.trim_end_matches('\0').trim().parse::<f32>().ok());
        let DecodingResult::F32(heights) = dec.read_image()? else {
            bail!("tile is not float32");
        };
        let (width, height) = (w as usize, h as usize);
        ensure!(heights.len() == width * height, "tile is not single band");
        Ok(Self {
            width,
            height,
            west: tie[3] - (tie[0] + shift) * dx,
            north: tie[4] + (tie[1] + shift) * dy,
            heights,
            nodata,
        })
    }

    /// Height at a pixel, or `None` for a void.
    fn pixel(&self, col: usize, row: usize) -> Option<f32> {
        let v = self.heights[row * self.width + col];
        (!v.is_nan() && Some(v) != self.nodata).then_some(v)
    }
}

/// True if the GeoKey directory declares point registration.
fn raster_is_point(keys: &[u16]) -> bool {
    keys.get(4..).is_some_and(|rest| {
        rest.as_chunks::<4>()
            .0
            .iter()
            .any(|k| k[0] == RASTER_TYPE_KEY && k[3] == RASTER_PIXEL_IS_POINT)
    })
}

/// Tiles of identical shape and registration, addressed by the integer-degree corner of each.
#[derive(Debug)]
pub struct Mosaic {
    /// Keyed by `(lon, lat)` of the south-west corner.
    tiles: BTreeMap<(i32, i32), Tile>,
    width: usize,
    height: usize,
    /// Registration offset of tile edges from whole degrees, longitude.
    off_lon: f64,
    /// Registration offset of tile edges from whole degrees, latitude.
    off_lat: f64,
}

impl Mosaic {
    /// Builds a mosaic. Every tile must have the same pixel dimensions and registration.
    pub fn new(tiles: Vec<Tile>) -> Result<Self> {
        let first = tiles.first().context("no source tiles")?;
        let (width, height) = (first.width, first.height);
        let off_lon = first.west - first.west.round();
        let off_lat = first.north - first.north.round();
        let mut map = BTreeMap::new();
        for t in tiles {
            ensure!(
                (t.width, t.height) == (width, height),
                "tiles differ in pixel dimensions"
            );
            ensure!(
                (t.west - t.west.round() - off_lon).abs() < OFFSET_TOLERANCE_DEG
                    && (t.north - t.north.round() - off_lat).abs() < OFFSET_TOLERANCE_DEG,
                "tiles differ in registration"
            );
            let key = (t.west.round() as i32, t.north.round() as i32 - 1);
            ensure!(map.insert(key, t).is_none(), "two tiles cover {key:?}");
        }
        Ok(Self {
            tiles: map,
            width,
            height,
            off_lon,
            off_lat,
        })
    }

    /// Height in metres by bilinear interpolation, or `None` if a contributing pixel is void.
    ///
    /// Fails if a contributing pixel lies in no tile.
    pub fn sample(&self, lat: f64, lon: f64) -> Result<Option<f64>> {
        let gx = (lon - self.off_lon) * self.width as f64 - 0.5;
        let gy = (lat - self.off_lat) * self.height as f64 - 0.5;
        let (x0, y0) = (gx.floor(), gy.floor());
        let (fx, fy) = (gx - x0, gy - y0);
        let (x0, y0) = (x0 as i64, y0 as i64);
        let mut sum = 0.0;
        for (dx, dy, weight) in [
            (0, 0, (1.0 - fx) * (1.0 - fy)),
            (1, 0, fx * (1.0 - fy)),
            (0, 1, (1.0 - fx) * fy),
            (1, 1, fx * fy),
        ] {
            if weight == 0.0 {
                continue;
            }
            match self.pixel(x0 + dx, y0 + dy)? {
                Some(v) => sum += weight * f64::from(v),
                None => return Ok(None),
            }
        }
        Ok(Some(sum))
    }

    /// Pixel at global column `gx` and row-from-south `gy`.
    fn pixel(&self, gx: i64, gy: i64) -> Result<Option<f32>> {
        let (w, h) = (self.width as i64, self.height as i64);
        let key = (gx.div_euclid(w) as i32, gy.div_euclid(h) as i32);
        let tile = self.tiles.get(&key).with_context(|| {
            format!(
                "no source tile covers the tile with south-west corner lon {} lat {}",
                key.0, key.1
            )
        })?;
        let col = gx.rem_euclid(w) as usize;
        let row = (h - 1 - gy.rem_euclid(h)) as usize;
        Ok(tile.pixel(col, row))
    }
}

/// Writes a synthetic square GeoTIFF for tests; `corner` is the (west, north) edge in degrees
/// and the tile spans one degree.
#[cfg(test)]
pub(crate) fn synthetic_tiff(
    size: usize,
    corner: (f64, f64),
    nodata: Option<&str>,
    f: impl Fn(usize, usize) -> f32,
) -> Vec<u8> {
    use tiff::encoder::{TiffEncoder, colortype::Gray32Float};
    let data: Vec<f32> = (0..size)
        .flat_map(|r| (0..size).map(move |c| (c, r)))
        .map(|(c, r)| f(c, r))
        .collect();
    let mut out = std::io::Cursor::new(Vec::new());
    let mut enc = TiffEncoder::new(&mut out).unwrap();
    let mut img = enc
        .new_image::<Gray32Float>(size as u32, size as u32)
        .unwrap();
    let px = 1.0 / size as f64;
    let dir = img.encoder();
    dir.write_tag(Tag::ModelPixelScaleTag, &[px, px, 0.0][..])
        .unwrap();
    dir.write_tag(
        Tag::ModelTiepointTag,
        &[0.0, 0.0, 0.0, corner.0, corner.1, 0.0][..],
    )
    .unwrap();
    if let Some(n) = nodata {
        dir.write_tag(Tag::GdalNodata, n).unwrap();
    }
    img.write_data(&data).unwrap();
    out.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// 4x4 tile whose west edge is 10, north edge is 21, so it spans lon 10..11, lat 20..21.
    fn tile(f: impl Fn(usize, usize) -> f32, nodata: Option<&str>) -> Tile {
        Tile::read(Cursor::new(synthetic_tiff(4, (10.0, 21.0), nodata, f))).unwrap()
    }

    #[test]
    fn reads_geometry_and_samples_pixel_centres() {
        // Height = 100 * column + row, row 0 north.
        let m = Mosaic::new(vec![tile(|c, r| (100 * c + r) as f32, None)]).unwrap();
        // Pixel centre of column 1, row 2: lon 10.375, lat 21 - 2.5/4 = 20.375.
        let v = m.sample(20.375, 10.375).unwrap().unwrap();
        assert!((v - 102.0).abs() < 1e-6, "{v}");
        // Halfway between columns 1 and 2 on that row.
        let v = m.sample(20.375, 10.5).unwrap().unwrap();
        assert!((v - 152.0).abs() < 1e-6, "{v}");
    }

    #[test]
    fn nodata_tag_and_nan_are_voids() {
        let f = |c: usize, r: usize| match (c, r) {
            (1, 1) => -32768.0,
            (2, 2) => f32::NAN,
            _ => 5.0,
        };
        let m = Mosaic::new(vec![tile(f, Some("-32768"))]).unwrap();
        // Centre of (1,1): lon 10.375, lat 20.625.
        assert_eq!(m.sample(20.625, 10.375).unwrap(), None);
        // Centre of (2,2): lon 10.625, lat 20.375.
        assert_eq!(m.sample(20.375, 10.625).unwrap(), None);
        // Centre of (0,0): untouched.
        assert_eq!(m.sample(20.875, 10.125).unwrap(), Some(5.0));
    }

    #[test]
    fn mosaic_interpolates_across_tile_border_and_rejects_gaps() {
        let read = |west: f64, v: f32| {
            Tile::read(Cursor::new(synthetic_tiff(
                4,
                (west, 21.0),
                None,
                move |_, _| v,
            )))
            .unwrap()
        };
        let m = Mosaic::new(vec![read(10.0, 10.0), read(11.0, 20.0)]).unwrap();
        // The border lon 11.0 is midway between the last and first pixel centres.
        let v = m.sample(20.5, 11.0).unwrap().unwrap();
        assert!((v - 15.0).abs() < 1e-6, "{v}");
        assert!(m.sample(20.5, 12.5).is_err());
    }

    #[test]
    fn point_registration_is_detected() {
        assert!(raster_is_point(&[1, 1, 0, 1, 1025, 0, 1, 2]));
        assert!(!raster_is_point(&[1, 1, 0, 1, 1025, 0, 1, 1]));
    }

    #[test]
    fn mismatched_tiles_are_rejected() {
        let a = tile(|_, _| 0.0, None);
        let b = Tile::read(Cursor::new(synthetic_tiff(
            8,
            (11.0, 21.0),
            None,
            |_, _| 0.0,
        )))
        .unwrap();
        assert!(Mosaic::new(vec![a, b]).is_err());
        assert!(Mosaic::new(vec![]).is_err());
    }
}
