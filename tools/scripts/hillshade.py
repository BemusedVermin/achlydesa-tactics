#!/usr/bin/env python3
"""Render an annotated hillshade of a square window cut from Copernicus DEM tiles.

Offline inspection tool for task P0-05 (docs/canon/TERRAIN_CANDIDATES.md).
Never used in the build. See docs/decisions/ADR-0100-terrain-source.md.

Projection: local equirectangular about the window's SW corner.
  x_km (east)  = R * cos(lat_mid) * dlon_rad
  y_km (north) = R * dlat_rad
with R = 6371.0088 km and lat_mid = the window's central latitude.
Resampling: bilinear from the source tiles onto a px x px grid.
Shading: azimuth 315 deg, altitude 45 deg (Field Atlas day palette tints,
Field Atlas section 3.1: Low #DDD2B8, Middle #BBBBA4, High #9C979B).
Tint bands are window-relative: Low = 5th percentile elevation,
Middle = median, High = 95th percentile (PROPOSED).

Marks file (--marks): JSON list of objects
  {"n": 1, "label": "Dry Meridian road", "points": [[x_km, y_km], ...]}
One point draws a labelled disc, several draw a labelled polyline.
Coordinates are window-local km from the SW corner (x east, y north).
"""

import argparse
import json
import math
import sys

R_KM = 6371.0088
LOW = (0xDD, 0xD2, 0xB8)
MID = (0xBB, 0xBB, 0xA4)
HIGH = (0x9C, 0x97, 0x9B)


def parse_args(argv):
    p = argparse.ArgumentParser(
        description="Render an annotated hillshade PNG of a square window from DEM GeoTIFF tiles.",
    )
    p.add_argument("--tiles", nargs="+", help="DEM GeoTIFF paths covering the window")
    p.add_argument("--sw-lat", type=float, required=True, help="window SW corner latitude (deg N)")
    p.add_argument("--sw-lon", type=float, required=True, help="window SW corner longitude (deg E)")
    p.add_argument("--size-km", type=float, default=150.0, help="window edge length in km (default 150)")
    p.add_argument("--px", type=int, default=1500, help="output edge length in pixels (default 1500)")
    p.add_argument("--grid-km", type=float, default=10.0, help="grid spacing in km (default 10)")
    p.add_argument("--exag", type=float, default=1.0, help="vertical exaggeration (default 1)")
    p.add_argument("--marks", help="JSON file of annotation marks (see module docstring)")
    p.add_argument("--out", help="output PNG path")
    p.add_argument(
        "--list-tiles",
        action="store_true",
        help="print the 1-degree GLO-90 tile URLs the window needs, then exit",
    )
    args = p.parse_args(argv)
    if not args.list_tiles and not (args.tiles and args.out):
        p.error("--tiles and --out are required unless --list-tiles is given")
    return args


def window_bounds(sw_lat, sw_lon, size_km):
    """Return (lat_min, lat_max, lon_min, lon_max) in degrees for the window."""
    lat_max = sw_lat + math.degrees(size_km / R_KM)
    lat_mid = (sw_lat + lat_max) / 2
    lon_max = sw_lon + math.degrees(size_km / (R_KM * math.cos(math.radians(lat_mid))))
    return sw_lat, lat_max, sw_lon, lon_max


def list_tiles(sw_lat, sw_lon, size_km):
    """Print the GLO-90 tile URLs covering the window."""
    lat0, lat1, lon0, lon1 = window_bounds(sw_lat, sw_lon, size_km)
    print(f"window: lat {lat0:.4f}..{lat1:.4f}, lon {lon0:.4f}..{lon1:.4f}")
    base = "https://copernicus-dem-90m.s3.eu-central-1.amazonaws.com"
    for la in range(math.floor(lat0), math.floor(lat1 - 1e-9) + 1):
        for lo in range(math.floor(lon0), math.floor(lon1 - 1e-9) + 1):
            ns, ew = ("N" if la >= 0 else "S"), ("E" if lo >= 0 else "W")
            name = f"Copernicus_DSM_COG_30_{ns}{abs(la):02d}_00_{ew}{abs(lo):03d}_00_DEM"
            print(f"{base}/{name}/{name}.tif")


def sample_window(np, rasterio, tiles, sw_lat, sw_lon, size_km, px):
    """Return elevation (px x px, row 0 = north) in metres, NaN where no tile or nodata."""
    step = size_km / px
    xs = (np.arange(px) + 0.5) * step
    ys = (np.arange(px) + 0.5) * step
    lat_mid = sw_lat + math.degrees(size_km / 2 / R_KM)
    lat = sw_lat + np.degrees(ys / R_KM)
    lon = sw_lon + np.degrees(xs / (R_KM * math.cos(math.radians(lat_mid))))
    lon_g, lat_g = np.meshgrid(lon, lat)
    out = np.full((px, px), np.nan, dtype=np.float64)
    for path in tiles:
        with rasterio.open(path) as src:
            data = src.read(1).astype(np.float64)
            if src.nodata is not None:
                data[data == src.nodata] = np.nan
            inv = ~src.transform
            col = inv.a * lon_g + inv.b * lat_g + inv.c - 0.5  # pixel centres
            row = inv.d * lon_g + inv.e * lat_g + inv.f - 0.5
            h, w = data.shape
            inside = (col >= 0) & (col <= w - 1) & (row >= 0) & (row <= h - 1)
            if not inside.any():
                continue
            c = np.clip(col, 0, w - 1)
            r = np.clip(row, 0, h - 1)
            c0 = np.floor(c).astype(int)
            r0 = np.floor(r).astype(int)
            c1 = np.minimum(c0 + 1, w - 1)
            r1 = np.minimum(r0 + 1, h - 1)
            fc = c - c0
            fr = r - r0
            v = (
                data[r0, c0] * (1 - fc) * (1 - fr)
                + data[r0, c1] * fc * (1 - fr)
                + data[r1, c0] * (1 - fc) * fr
                + data[r1, c1] * fc * fr
            )
            take = inside & np.isnan(out)
            out[take] = v[take]
    return out[::-1, :]  # flip so row 0 is north


def hillshade(np, elev, cell_m, exag, az_deg=315.0, alt_deg=45.0):
    z = np.nan_to_num(elev, nan=float(np.nanmedian(elev))) * exag
    dzdy_south, dzdx = np.gradient(z, cell_m)  # rows run north->south
    dzdy = -dzdy_south  # per metre northward
    nz = 1.0 / np.sqrt(dzdx**2 + dzdy**2 + 1.0)
    nx, ny = -dzdx * nz, -dzdy * nz
    az, alt = math.radians(az_deg), math.radians(alt_deg)
    lx, ly, lz = math.sin(az) * math.cos(alt), math.cos(az) * math.cos(alt), math.sin(alt)
    return np.clip(nx * lx + ny * ly + nz * lz, 0.0, 1.0)


def tint(np, elev):
    valid = elev[~np.isnan(elev)]
    lo, mid, hi = np.percentile(valid, [5, 50, 95])
    z = np.nan_to_num(elev, nan=mid)
    stops = np.array([lo, mid, hi])
    rgb = np.stack(
        [np.interp(z, stops, [LOW[i], MID[i], HIGH[i]]) for i in range(3)], axis=-1
    )
    return rgb, (lo, mid, hi)


def annotate(Image, ImageDraw, ImageFont, img, px, size_km, grid_km, marks):
    s = px / size_km
    img = img.convert("RGBA")
    ov = Image.new("RGBA", img.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    font = ImageFont.load_default(size=16)
    small = ImageFont.load_default(size=13)

    def to_px(x, y):
        return x * s, px - y * s

    k = 0.0
    while k <= size_km + 1e-9:
        major = abs(k % (grid_km * 5)) < 1e-9
        a = 110 if major else 60
        d.line([to_px(k, 0), to_px(k, size_km)], fill=(40, 40, 40, a), width=1)
        d.line([to_px(0, k), to_px(size_km, k)], fill=(40, 40, 40, a), width=1)
        if major and 0 < k < size_km:
            d.text((k * s + 3, px - 16), f"{k:g}", fill=(30, 30, 30, 230), font=small)
            d.text((3, px - k * s + 2), f"{k:g}", fill=(30, 30, 30, 230), font=small)
        k += grid_km

    for m in marks:
        pts = [to_px(*p) for p in m["points"]]
        col = (176, 40, 30, 255)
        if len(pts) > 1:
            d.line(pts, fill=col, width=4)
        for (x, y) in pts[:1] if len(pts) > 1 else pts:
            d.ellipse([x - 14, y - 14, x + 14, y + 14], fill=col, outline=(255, 255, 255, 255), width=2)
            d.text((x - 5, y - 9), str(m["n"]), fill=(255, 255, 255, 255), font=font)
        lx, ly = pts[0][0] + 18, pts[0][1] - 10
        w = d.textlength(m["label"], font=font)
        d.rectangle([lx - 3, ly - 2, lx + w + 3, ly + 20], fill=(255, 255, 255, 200))
        d.text((lx, ly), m["label"], fill=(120, 20, 15, 255), font=font)

    # scale bar (20 km) bottom-left, inset
    bx, by = 40, px - 50
    d.rectangle([bx - 6, by - 22, bx + 20 * s + 6, by + 14], fill=(255, 255, 255, 190))
    d.line([(bx, by), (bx + 20 * s, by)], fill=(0, 0, 0, 255), width=4)
    for t in (0, 10, 20):
        d.line([(bx + t * s, by - 6), (bx + t * s, by + 6)], fill=(0, 0, 0, 255), width=2)
    d.text((bx, by - 21), "0", fill=(0, 0, 0, 255), font=small)
    d.text((bx + 20 * s - 22, by - 21), "20 km", fill=(0, 0, 0, 255), font=small)

    # north arrow top-right
    ax, ay = px - 45, 70
    d.rectangle([ax - 25, ay - 55, ax + 25, ay + 25], fill=(255, 255, 255, 190))
    d.polygon([(ax, ay - 45), (ax - 12, ay + 10), (ax, ay), (ax + 12, ay + 10)], fill=(0, 0, 0, 255))
    d.text((ax - 6, ay + 4), "N", fill=(0, 0, 0, 255), font=font)

    return Image.alpha_composite(img, ov).convert("RGB")


def main(argv=None):
    args = parse_args(sys.argv[1:] if argv is None else argv)
    if args.list_tiles:
        list_tiles(args.sw_lat, args.sw_lon, args.size_km)
        return
    try:
        import numpy as np
        import rasterio
        from PIL import Image, ImageDraw, ImageFont
    except ImportError as e:  # pragma: no cover
        sys.exit(f"missing dependency ({e}); run: pip install -r tools/scripts/requirements.txt")

    elev = sample_window(np, rasterio, args.tiles, args.sw_lat, args.sw_lon, args.size_km, args.px)
    if np.isnan(elev).all():
        sys.exit("error: no tile covers the window")
    missing = float(np.isnan(elev).mean())
    if missing > 0:
        print(f"warning: {missing:.1%} of window has no data", file=sys.stderr)

    cell_m = args.size_km * 1000.0 / args.px
    shade = hillshade(np, elev, cell_m, args.exag)
    rgb, (lo, mid, hi) = tint(np, elev)
    factor = np.clip(0.85 * shade / math.sin(math.radians(45)), 0.2, 1.15)
    out = np.clip(rgb * factor[..., None], 0, 255).astype(np.uint8)
    out[np.isnan(elev)] = (255, 0, 255)  # magenta = no data

    marks = []
    if args.marks:
        with open(args.marks, encoding="utf-8") as f:
            marks = json.load(f)

    img = annotate(Image, ImageDraw, ImageFont, Image.fromarray(out), args.px, args.size_km, args.grid_km, marks)
    img.save(args.out)
    print(
        f"wrote {args.out}: min {np.nanmin(elev):.0f} m, p5 {lo:.0f}, median {mid:.0f}, "
        f"p95 {hi:.0f}, max {np.nanmax(elev):.0f} m"
    )


if __name__ == "__main__":
    main()
