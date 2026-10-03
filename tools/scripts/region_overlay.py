#!/usr/bin/env python3
"""Verify and draw the Red Ledger region (task P0-06).

Offline inspection tool, never used in the build. Reads
  content/terrain/red_ledger/window.ron
  content/world/red_ledger_sites.ron
  content/world/red_ledger_routes.ron
resamples the Copernicus GLO-30 tiles named in window.ron into the transformed
local frame (100 m cells, 1500 x 1500), prints every distance constraint of
docs/tasks/P0-06.md with PASS/FAIL, and writes docs/canon/red_ledger_region.png.

Frame: x east, y north, integer metres, origin at the SW corner after the window
transform (mirror_x first, then counter-clockwise rotation). Projection and
resampling are those of tools/scripts/hillshade.py (equirectangular about the
window centre latitude, bilinear). Colours: Field Atlas section 3.1 day palette.

Usage (repo root):  python tools/scripts/region_overlay.py [--out PATH] [--no-image]
Exit status 1 if any check fails.
"""

import argparse
import heapq
import math
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.path.insert(0, HERE)

PX = 1500  # raster edge in pixels; 100 m cells for the 150 km window
PAPER = (0xE9, 0xDF, 0xC9)
INK = (0x35, 0x2F, 0x3F)
SECONDARY = (0x65, 0x5B, 0x68)
FRIENDLY = (0x23, 0x69, 0x65)
CAUTION = (0x73, 0x50, 0x08)
ANOMALOUS = (0x88, 0x46, 0x53)
WATER = (0x4A, 0x6F, 0x8C)

REQUIRED_SITES = [
    "cistern_camp", "inspection_station", "roadside_terminal", "reedbank",
    "excluded_settlement", "reserve_depot", "marches_garrison",
    "three_crossings_a", "three_crossings_b", "three_crossings_c", "lamp_ward",
]
REQUIRED_ROUTES = ["dry_meridian", "reedbank_spur", "sera_approach"]
SLOPE_LIMIT_PERMILLE = 150
RIDGE_RELIEF_MIN_M = 40.0  # PROPOSED definition: crest cell minus mean of the 1-2 km ring around it


# ---------------------------------------------------------------- RON subset
_TOK = re.compile(
    r'\s*(?://[^\n]*\n?|/\*.*?\*/|(?P<str>"(?:[^"\\]|\\.)*")|(?P<num>-?\d+(?:\.\d+)?)|(?P<id>[A-Za-z_][A-Za-z0-9_]*)|(?P<p>[()\[\],:]))',
    re.S,
)


def _tokens(text):
    pos, out = 0, []
    while pos < len(text):
        m = _TOK.match(text, pos)
        if not m:
            if text[pos:].strip() == "":
                break
            raise ValueError(f"RON: cannot tokenize near {text[pos:pos + 30]!r}")
        pos = m.end()
        if m.group("str") is not None:
            out.append(("s", bytes(m.group("str")[1:-1], "utf-8").decode("unicode_escape") if "\\" in m.group("str") else m.group("str")[1:-1]))
        elif m.group("num") is not None:
            t = m.group("num")
            out.append(("n", float(t) if "." in t else int(t)))
        elif m.group("id") is not None:
            out.append(("i", m.group("id")))
        elif m.group("p") is not None:
            out.append(("p", m.group("p")))
    return out


def parse_ron(text):
    """Parse the RON subset used by the content files: structs, tuples, lists, strings, numbers, bare identifiers."""
    toks = _tokens(text)
    pos = 0

    def value():
        nonlocal pos
        kind, v = toks[pos]
        if kind in ("s", "n"):
            pos += 1
            return v
        if kind == "i":
            pos += 1
            if v == "true":
                return True
            if v == "false":
                return False
            if v == "None":
                return None
            if pos < len(toks) and toks[pos] == ("p", "("):  # Some(x) style wrapper
                return value()
            return v
        if (kind, v) == ("p", "["):
            pos += 1
            items = []
            while toks[pos] != ("p", "]"):
                items.append(value())
                if toks[pos] == ("p", ","):
                    pos += 1
            pos += 1
            return items
        if (kind, v) == ("p", "("):
            pos += 1
            if toks[pos][0] == "i" and toks[pos + 1] == ("p", ":"):
                d = {}
                while toks[pos] != ("p", ")"):
                    key = toks[pos][1]
                    pos += 2
                    d[key] = value()
                    if toks[pos] == ("p", ","):
                        pos += 1
                pos += 1
                return d
            items = []
            while toks[pos] != ("p", ")"):
                items.append(value())
                if toks[pos] == ("p", ","):
                    pos += 1
            pos += 1
            return items
        raise ValueError(f"RON: unexpected token {toks[pos]}")

    result = value()
    if pos != len(toks):
        raise ValueError("RON: trailing tokens")
    return result


def load_ron(rel):
    with open(os.path.join(ROOT, rel), encoding="utf-8") as f:
        return parse_ron(f.read())


# ------------------------------------------------------------------ checks
class Report:
    def __init__(self):
        self.failed = 0

    def check(self, ok, label, detail=""):
        if not ok:
            self.failed += 1
        print(f"{'PASS' if ok else 'FAIL'}  {label}" + (f"  [{detail}]" if detail else ""))

    def info(self, text):
        print(f"      {text}")


def plen(pts):
    return sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(pts, pts[1:]))


def seg_dist_pt(p, a, b):
    ax, ay, bx, by = a[0], a[1], b[0], b[1]
    dx, dy = bx - ax, by - ay
    t = 0.0 if dx == dy == 0 else max(0.0, min(1.0, ((p[0] - ax) * dx + (p[1] - ay) * dy) / (dx * dx + dy * dy)))
    return math.hypot(p[0] - (ax + t * dx), p[1] - (ay + t * dy))


def dist_to_polyline(p, pts):
    return min(seg_dist_pt(p, a, b) for a, b in zip(pts, pts[1:]))


def segments_cross(a, b, c, d):
    def o(p, q, r):
        return (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])

    return o(a, b, c) * o(a, b, d) < 0 and o(c, d, a) * o(c, d, b) < 0


def polylines_cross(p, q):
    return any(segments_cross(a, b, c, d) for a, b in zip(p, p[1:]) for c, d in zip(q, q[1:]))


# ------------------------------------------------------------------ terrain
def build_terrain(win):
    import numpy as np
    import rasterio
    import hillshade as h

    tiles = [os.path.join(ROOT, "data", "raw", "terrain", t) for t in win["tiles"]]
    missing = [t for t in tiles if not os.path.exists(t)]
    if missing:
        sys.exit(f"missing tiles (see ADR-0100 Download commands): {missing}")
    sw_lat, sw_lon = win["sw_lat_lon"]
    size_km = win["extent_m"][0] / 1000.0
    e = h.sample_window(np, rasterio, tiles, sw_lat, sw_lon, size_km, PX)  # row 0 = real north
    nan = int(np.isnan(e).sum())
    if win["transform"]["mirror_x"]:
        e = e[:, ::-1]
    k = (win["transform"]["rotate_deg"] // 90) % 4
    e = np.rot90(e, k)  # counter-clockwise; row 0 = game north afterwards
    return np, h, e, nan


class Dem:
    """Bilinear elevation lookup on the 100 m game-frame raster (row 0 = north)."""

    def __init__(self, np, e, cell_m):
        self.np, self.e, self.cell = np, e, cell_m
        self.n = e.shape[0]

    def z(self, x_m, y_m):
        c = x_m / self.cell - 0.5
        r = self.n - 1 - (y_m / self.cell - 0.5)
        c0 = int(max(0, min(self.n - 2, math.floor(c))))
        r0 = int(max(0, min(self.n - 2, math.floor(r))))
        fc, fr = min(max(c - c0, 0.0), 1.0), min(max(r - r0, 0.0), 1.0)
        e = self.e
        return float(
            e[r0, c0] * (1 - fc) * (1 - fr) + e[r0, c0 + 1] * fc * (1 - fr)
            + e[r0 + 1, c0] * (1 - fc) * fr + e[r0 + 1, c0 + 1] * fc * fr
        )

    def ring_relief(self, x_m, y_m, r_in=1000.0, r_out=2000.0):
        np = self.np
        c, r = int(x_m / self.cell), int(self.n - 1 - y_m / self.cell)
        span = int(r_out / self.cell) + 1
        rr = np.arange(max(0, r - span), min(self.n, r + span + 1))
        cc = np.arange(max(0, c - span), min(self.n, c + span + 1))
        R, C = np.meshgrid(rr, cc, indexing="ij")
        d = np.hypot((R - r) * self.cell, (C - c) * self.cell)
        ring = (d >= r_in) & (d <= r_out)
        return float(self.e[r, c] - self.e[R[ring], C[ring]].mean())


def road_graph(routes, kinds, snap_m=60):
    """Graph of route vertices (keyed by index) joined along each route and across routes where vertices lie within snap_m."""
    nodes, edges = [], {}
    for rt in routes:
        if rt["kind"] not in kinds:
            continue
        prev = None
        for v in rt["vertices"]:
            idx = len(nodes)
            nodes.append((v[0], v[1], rt["id"]))
            edges[idx] = []
            if prev is not None:
                d = math.hypot(v[0] - nodes[prev][0], v[1] - nodes[prev][1])
                edges[idx].append((prev, d))
                edges[prev].append((idx, d))
            prev = idx
    # join coincident vertices of different routes
    for i, a in enumerate(nodes):
        for j in range(i + 1, len(nodes)):
            b = nodes[j]
            if a[2] != b[2] and abs(a[0] - b[0]) <= snap_m and abs(a[1] - b[1]) <= snap_m:
                d = math.hypot(a[0] - b[0], a[1] - b[1])
                edges[i].append((j, d))
                edges[j].append((i, d))
    return nodes, edges


def graph_dist(nodes, edges, src_xy, dst_xy):
    def near(p):
        return min(range(len(nodes)), key=lambda i: math.hypot(nodes[i][0] - p[0], nodes[i][1] - p[1]))

    s, t = near(src_xy), near(dst_xy)
    dist = {s: 0.0}
    prev = {}
    pq = [(0.0, s)]
    while pq:
        d, u = heapq.heappop(pq)
        if u == t:
            break
        if d > dist.get(u, 1e18):
            continue
        for v, w in edges[u]:
            if d + w < dist.get(v, 1e18):
                dist[v] = d + w
                prev[v] = u
                heapq.heappush(pq, (d + w, v))
    if t not in dist:
        return None, []
    path, u = [t], t
    while u != s:
        u = prev[u]
        path.append(u)
    return dist[t], [(nodes[i][0], nodes[i][1]) for i in path[::-1]]


def cite_ok(cite):
    """A canon citation is `file §section` where the file exists and a heading matches, or PROPOSED."""
    if cite.strip() == "PROPOSED":
        return True
    m = re.match(r"^(\S+\.md) §(.+)$", cite.strip())
    if not m:
        return False
    fname, sec = m.groups()
    path = os.path.join(ROOT, "docs", "design", fname)
    if not os.path.exists(path):
        return False

    def norm(s):
        return re.sub(r"[^a-z0-9]+", "", s.lower())

    with open(path, encoding="utf-8") as f:
        heads = [norm(ln.lstrip("#")) for ln in f if ln.startswith("#")]
    want = [norm(sec), norm(re.sub(r"^\d+\s+", "", sec))]
    return any(w and any(w in hd for hd in heads) for w in want)


# ------------------------------------------------------------------ drawing
def draw(np, h, e, win, sites, routes, water, tour, args):
    from PIL import Image, ImageDraw, ImageFont

    cell = win["extent_m"][0] / PX
    sh = h.hillshade(np, e, cell, 1.0)
    low, mid, high = (0xDD, 0xD2, 0xB8), (0xBB, 0xBB, 0xA4), (0x9C, 0x97, 0x9B)
    p5, p50, p95 = np.percentile(e, [5, 50, 95])
    stops = np.array([p5, p50, p95])
    rgb = np.stack([np.interp(e, stops, [low[i], mid[i], high[i]]) for i in range(3)], axis=-1)
    f = np.clip(0.85 * sh / math.sin(math.radians(45)), 0.2, 1.15)
    base = np.clip(rgb * f[..., None], 0, 255).astype(np.uint8)
    img = Image.fromarray(base).convert("RGBA")
    ov = Image.new("RGBA", img.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(ov)
    big = ImageFont.load_default(size=18)
    font = ImageFont.load_default(size=14)
    small = ImageFont.load_default(size=12)
    s = 1.0 / cell

    def P(x, y):
        return (x * s, PX - y * s)

    # 10 km grid, labels every 20 km
    for k in range(0, 151, 10):
        a = 90 if k % 50 == 0 else 45
        d.line([P(k * 1000, 0), P(k * 1000, 150000)], fill=INK + (a,), width=1)
        d.line([P(0, k * 1000), P(150000, k * 1000)], fill=INK + (a,), width=1)
        if k % 20 == 0 and 0 < k < 150:
            d.text((P(k * 1000, 0)[0] + 3, PX - 15), f"{k}", fill=INK + (255,), font=small)
            d.text((3, P(0, k * 1000)[1] + 2), f"{k}", fill=INK + (255,), font=small)

    def dashed(pts, color, width, on, off):
        acc, draw_on = 0.0, True
        for a, b in zip(pts, pts[1:]):
            L = math.hypot(b[0] - a[0], b[1] - a[1])
            if L == 0:
                continue
            t = 0.0
            while t < L:
                run = (on if draw_on else off) - acc
                step = min(run, L - t)
                if draw_on:
                    p0 = (a[0] + (b[0] - a[0]) * t / L, a[1] + (b[1] - a[1]) * t / L)
                    p1 = (a[0] + (b[0] - a[0]) * (t + step) / L, a[1] + (b[1] - a[1]) * (t + step) / L)
                    d.line([p0, p1], fill=color, width=width)
                t += step
                acc += step
                if acc >= (on if draw_on else off) - 1e-9:
                    acc, draw_on = 0.0, not draw_on

    # watercourses (thin, behind roads)
    for w in water:
        d.line([P(*v) for v in w["vertices"]], fill=WATER + (200,), width=2)

    # boxes
    for key, color, label in (("strip_p1", INK, "strip_p1  96 x 24 km (Phase 1)"), ("corridor_p3", CAUTION, "corridor_p3  30 x 30 km (Phase 3)")):
        b = win[key]
        x0, y0, x1, y1 = b["min"][0], b["min"][1], b["max"][0], b["max"][1]
        pts = [P(x0, y0), P(x1, y0), P(x1, y1), P(x0, y1), P(x0, y0)]
        dashed(pts, color + (255,), 3, 18, 8) if key == "strip_p1" else d.line(pts, fill=color + (255,), width=3)

    # routes
    for rt in routes:
        pts = [P(*v) for v in rt["vertices"]]
        col = INK + (255,)
        wd = 5 if rt["id"] == "dry_meridian" else 3
        if rt["condition"] == "intact":
            d.line(pts, fill=col, width=wd, joint="curve")
        elif rt["condition"] == "worn":
            dashed(pts, col, wd, 14, 6)
        else:
            dashed(pts, col, wd, 3, 7)

    # heliarch tour: anomalous colour, long-short dash
    tp = [P(v["x_m"], v["y_m"]) for v in tour["vertices"]]
    dashed(tp, ANOMALOUS + (255,), 4, 22, 6)
    # label placement is explicit (km, top-left of the text) so the dense corridor stays legible
    SITE_LABEL_KM = {
        "cistern_camp": (7.0, 102.5), "station_ridge": (21.0, 104.5), "inspection_station": (22.0, 89.2),
        "roadside_terminal": (44.0, 86.2), "three_crossings_a": (37.0, 108.0), "three_crossings_b": (50.5, 110.8),
        "three_crossings_c": (64.0, 103.0), "excluded_settlement": (60.0, 73.2), "reedbank": (67.0, 70.0),
        "reserve_depot": (66.0, 86.6), "marches_garrison": (86.5, 77.2), "lamp_ward": (95.0, 91.5),
    }
    ROUTE_LABEL_KM = {
        "dry_meridian": (12.5, 98.2, "The Dry Meridian"), "reedbank_spur": (76.5, 74.0, "Reedbank Relief Road"),
        "sera_approach": (49.0, 80.2, "Sera's Rim Approach"), "crossing_road_a": (38.0, 98.6, ""),
        "crossing_road_b": (0, 0, ""), "crossing_road_c": (0, 0, ""), "depot_track": (0, 0, ""),
    }
    TOUR_LABEL_KM = [(5.0, 134.0), (19.0, 118.0), (33.0, 106.6), (43.0, 103.4), (57.5, 97.6), (72.0, 92.5), (91.0, 88.0), (112.0, 62.5), (136.0, 53.5)]
    labels = []  # (x, y, text, color, font, leader_to)

    def leader(frm, to, color):
        d.line([frm, to], fill=color + (200,), width=1)

    for i, (v, pp) in enumerate(zip(tour["vertices"], tp)):
        d.ellipse([pp[0] - 4, pp[1] - 4, pp[0] + 4, pp[1] + 4], fill=ANOMALOUS + (255,))
        lx, ly = P(*[c * 1000 for c in TOUR_LABEL_KM[i]]) if i < len(TOUR_LABEL_KM) else (pp[0] + 7, pp[1] - 8)
        alt = v["altitude_m"] / 1000
        labels.append((lx, ly, f"{alt:g} km", ANOMALOUS, small, pp))
    tx, ty = P(16000, 142000)
    labels.append((tx, ty, "Heliarch tour: " + tour["name"] + " (altitude above ground)", ANOMALOUS, font, None))

    for rt in routes:
        lx_, ly_, txt = ROUTE_LABEL_KM.get(rt["id"], (0, 0, ""))
        if txt:
            lx, ly = P(lx_ * 1000, ly_ * 1000)
            labels.append((lx, ly, txt, INK, font, None))

    for st in sites:
        x, y = P(*st["pos"])
        if st["id"] == "station_ridge":
            d.polygon([(x, y - 9), (x - 9, y + 7), (x + 9, y + 7)], fill=FRIENDLY + (255,), outline=PAPER + (255,))
        elif st["kind"] in ("checkpoint", "garrison"):
            d.polygon([(x, y - 10), (x + 10, y), (x, y + 10), (x - 10, y)], fill=FRIENDLY + (255,), outline=PAPER + (255,))
        else:
            d.rectangle([x - 8, y - 8, x + 8, y + 8], fill=FRIENDLY + (255,), outline=PAPER + (255,), width=2)
        lx_, ly_ = SITE_LABEL_KM.get(st["id"], (st["pos"][0] / 1000 + 1, st["pos"][1] / 1000 + 1))
        lx, ly = P(lx_ * 1000, ly_ * 1000)
        labels.append((lx, ly, st["name"], FRIENDLY, font, (x, y)))

    for kind_key, color, text in (
        ("strip_p1", INK, "strip_p1: 96 x 24 km, Phase 1 three-day march"),
        ("corridor_p3", CAUTION, "corridor_p3: 30 x 30 km, Phase 3"),
    ):
        b_ = win[kind_key]
        if kind_key == "strip_p1":
            lx, ly = P(b_["min"][0] + 300, b_["min"][1] - 400)
            labels.append((lx, ly + 2, text, color, font, None))
        else:
            lx, ly = P(b_["min"][0] + 300, b_["min"][1] - 400)
            labels.append((lx, ly + 2, text, color, font, None))

    # place labels on paper backings, with a thin leader to the marker where one exists
    for x, y, text, color, fnt, lead in labels:
        w = d.textlength(text, font=fnt)
        if lead is not None:
            cx, cy = x + w / 2, y + fnt.size / 2
            if math.hypot(cx - lead[0], cy - lead[1]) > 28:
                leader((min(max(lead[0], x), x + w), y + fnt.size + 2 if lead[1] > y else y - 2), lead, color)
        d.rectangle([x - 3, y - 2, x + w + 3, y + fnt.size + 2], fill=PAPER + (230,))
        d.text((x, y), text, fill=color + (255,), font=fnt)

    # titles, legend, scale, north
    def panel(x0, y0, lines, wide):
        h_ = 8 + 17 * len(lines)
        d.rectangle([x0, y0, x0 + wide, y0 + h_], fill=PAPER + (235,), outline=INK + (255,))
        for i, (txt, col) in enumerate(lines):
            d.text((x0 + 8, y0 + 6 + 17 * i), txt, fill=col + (255,), font=font)

    panel(
        14, 14,
        [
            ("Red Ledger region (Movements I-II) - local frame, x east, y north", INK),
            ("Status: PROPOSED.  GLO-30 hillshade, light from NW, 10 km grid.", SECONDARY),
            ("Window: real window rotated 270 deg ccw (real N = game E).", SECONDARY),
        ],
        520,
    )
    panel(
        14, PX - 150,
        [
            ("Legend", INK),
            ("square/diamond/triangle  site / Marches site / ridge", FRIENDLY),
            ("solid / dashed / dotted  route: intact / worn / broken", INK),
            ("long-short dash  Heliarch tour, altitude above ground", ANOMALOUS),
            ("blue line  watercourse (D8 thalweg)", WATER),
            ("dashed box strip_p1;  solid box corridor_p3", CAUTION),
            ("crossing roads and the depot track: ids in red_ledger_routes.ron", SECONDARY),
        ],
        420,
    )
    bx, by = PX - 270, PX - 40
    d.rectangle([bx - 10, by - 26, bx + 200 + 10, by + 14], fill=PAPER + (235,))
    d.line([(bx, by), (bx + 20000 * s, by)], fill=INK + (255,), width=4)
    for t in (0, 10000, 20000):
        d.line([(bx + t * s, by - 6), (bx + t * s, by + 6)], fill=INK + (255,), width=2)
    d.text((bx, by - 22), "0", fill=INK + (255,), font=small)
    d.text((bx + 20000 * s - 40, by - 22), "20 km", fill=INK + (255,), font=small)
    ax, ay = PX - 45, 75
    d.rectangle([ax - 25, ay - 55, ax + 25, ay + 25], fill=PAPER + (235,))
    d.polygon([(ax, ay - 45), (ax - 12, ay + 10), (ax, ay), (ax + 12, ay + 10)], fill=INK + (255,))
    d.text((ax - 6, ay + 4), "N", fill=INK + (255,), font=big)

    out = Image.alpha_composite(img, ov).convert("RGB")
    os.makedirs(os.path.dirname(args.out), exist_ok=True)
    out.save(args.out)
    print(f"wrote {os.path.relpath(args.out, ROOT)} ({out.size[0]} x {out.size[1]})")


# --------------------------------------------------------------------- main
def main(argv=None):
    ap = argparse.ArgumentParser(description="Verify and draw the Red Ledger region.")
    ap.add_argument("--out", default=os.path.join(ROOT, "docs", "canon", "red_ledger_region.png"))
    ap.add_argument("--no-image", action="store_true", help="run the checks only")
    args = ap.parse_args(argv)

    try:
        win = load_ron("content/terrain/red_ledger/window.ron")
        site_doc = load_ron("content/world/red_ledger_sites.ron")
        route_doc = load_ron("content/world/red_ledger_routes.ron")
    except (OSError, ValueError, IndexError) as exc:
        sys.exit(f"cannot load content: {exc}")

    R = Report()
    sites = site_doc["sites"]
    S = {s["id"]: s for s in sites}
    routes = route_doc["routes"]
    RT = {r["id"]: r for r in routes}
    water = route_doc["watercourses"]
    W = {w["id"]: w for w in water}
    tour = route_doc["heliarch_tour"]
    for s in sites:
        s["pos"] = tuple(s["pos"])
    for r in routes + water:
        r["vertices"] = [tuple(v) for v in r["vertices"]]

    print("== Content")
    R.check(not [i for i in REQUIRED_SITES if i not in S], "all required site ids present", ", ".join(i for i in REQUIRED_SITES if i not in S))
    R.check(not [i for i in REQUIRED_ROUTES if i not in RT], "all required route ids present", ", ".join(i for i in REQUIRED_ROUTES if i not in RT))
    bad = [s["id"] for s in sites if not cite_ok(s["canon"])]
    R.check(not bad, "every site has a verified citation or PROPOSED", ", ".join(bad))
    sp = max(max(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(r["vertices"], r["vertices"][1:])) for r in routes)
    R.check(sp <= 500, "route vertices at most 500 m apart", f"largest gap {sp:.0f} m")
    R.check(all(r["kind"] in ("road", "track", "approach") and r["condition"] in ("intact", "worn", "broken") for r in routes), "route kind/condition values valid")

    print("== Window and boxes")
    ext = win["extent_m"]
    R.check(win["transform"]["rotate_deg"] in (0, 90, 180, 270), "rotate_deg in {0,90,180,270}", str(win["transform"]["rotate_deg"]))
    for key, (w_, h_) in (("strip_p1", (96000, 24000)), ("corridor_p3", (30000, 30000))):
        b = win[key]
        dw, dh = b["max"][0] - b["min"][0], b["max"][1] - b["min"][1]
        inside = b["min"][0] >= 0 and b["min"][1] >= 0 and b["max"][0] <= ext[0] and b["max"][1] <= ext[1]
        R.check((dw, dh) == (w_, h_), f"{key} is exactly {w_} x {h_} m", f"{dw} x {dh}")
        R.check(inside, f"{key} inside the window")
    strip, cor = win["strip_p1"], win["corridor_p3"]
    ox = max(0, min(strip["max"][0], cor["max"][0]) - max(strip["min"][0], cor["min"][0]))
    oy = max(0, min(strip["max"][1], cor["max"][1]) - max(strip["min"][1], cor["min"][1]))
    R.info(f"corridor_p3 overlaps strip_p1 by {ox / 1000:g} x {oy / 1000:g} km = {ox * oy / 1e6:.0f} of 900 km2 ({ox * oy / 9e8:.0%}); the 30 km corridor is taller than the 24 km strip, so it cannot be wholly inside")

    def inside_win(p):
        return 0 <= p[0] <= ext[0] and 0 <= p[1] <= ext[1]

    R.check(all(inside_win(s["pos"]) for s in sites) and all(inside_win(v) for r in routes + water for v in r["vertices"]), "all sites and route vertices inside the window")

    print("== Terrain")
    np, h, e, nan = build_terrain(win)
    cell = ext[0] / PX
    dem = Dem(np, e, cell)
    R.check(nan == 0, "no void cells in the sampled window (GLO-30 void check)", f"{nan} NaN of {PX * PX}")
    R.info(f"elevation min {e.min():.0f} m, median {np.median(e):.0f} m, max {e.max():.0f} m")
    gy_, gx_ = np.gradient(e, cell)
    flat = (e < -380) & (np.hypot(gx_, gy_) < 0.002)
    ys_, xs_ = np.nonzero(flat)
    if flat.any():
        R.info(
            f"flat floor below -380 m (slope < 2 permille; the real lake surface reads as a level plain): {flat.sum() * (cell / 1000) ** 2:.0f} km2, "
            f"x {xs_.min() * cell / 1000:.0f}-{xs_.max() * cell / 1000:.0f} km, y {(PX - 1 - ys_.max()) * cell / 1000:.0f}-{(PX - 1 - ys_.min()) * cell / 1000:.0f} km"
        )
    for sid in REQUIRED_SITES + ["station_ridge"]:
        if sid in S:
            R.info(f"{sid:20s} {S[sid]['pos'][0]/1000:7.2f} {S[sid]['pos'][1]/1000:7.2f} km   z {dem.z(*S[sid]['pos']):7.1f} m")

    print("== Slope on routes (segment rise/run on the 100 m raster)")
    worst = {}
    for r in routes:
        v = r["vertices"]
        sl = [abs(dem.z(*b) - dem.z(*a)) / max(1.0, math.hypot(b[0] - a[0], b[1] - a[1])) * 1000 for a, b in zip(v, v[1:])]
        worst[r["id"]] = max(sl)
        ok = max(sl) <= SLOPE_LIMIT_PERMILLE
        R.check(ok, f"{r['id']} slope <= {SLOPE_LIMIT_PERMILLE} permille", f"max {max(sl):.0f}, mean {sum(sl) / len(sl):.0f}, length {plen(v) / 1000:.1f} km")

    print("== Distances along road-class routes (dry_meridian, reedbank_spur, crossing road b; excludes tracks and approaches)")
    nodes, edges = road_graph(routes, ("road",))
    camp, stn, reed = S["cistern_camp"]["pos"], S["inspection_station"]["pos"], S["reedbank"]["pos"]
    d_cs, _ = graph_dist(nodes, edges, camp, stn)
    d_sr, _ = graph_dist(nodes, edges, stn, reed)
    d_cr, p_cr = graph_dist(nodes, edges, camp, reed)
    R.check(d_cs is not None and 20000 <= d_cs <= 45000, "Cistern Camp to inspection station 20-45 km", f"{(d_cs or 0) / 1000:.1f} km")
    R.check(d_sr is not None and 20000 <= d_sr <= 45000, "inspection station to Reedbank 20-45 km", f"{(d_sr or 0) / 1000:.1f} km")
    R.check(d_cr is not None and 70000 <= d_cr <= 90000, "Cistern Camp to Reedbank 70-90 km", f"{(d_cr or 0) / 1000:.1f} km")
    sx0, sy0, sx1, sy1 = strip["min"][0], strip["min"][1], strip["max"][0], strip["max"][1]
    out = [p for p in p_cr if not (sx0 <= p[0] <= sx1 and sy0 <= p[1] <= sy1)]
    R.check(bool(p_cr) and not out, "Camp-to-Reedbank road lies inside strip_p1", f"{len(p_cr)} vertices, {len(out)} outside; x {min(p[0] for p in p_cr)/1000:.1f}-{max(p[0] for p in p_cr)/1000:.1f}, y {min(p[1] for p in p_cr)/1000:.1f}-{max(p[1] for p in p_cr)/1000:.1f} km")
    na, ea = road_graph(routes, ("road", "track", "approach"))
    d_sera, _ = graph_dist(na, ea, S["roadside_terminal"]["pos"], reed)
    R.info(f"for comparison, terminal to Reedbank by all routes (incl. the rim approach): {(d_sera or 0) / 1000:.1f} km; Sera's approach alone {plen(RT['sera_approach']['vertices']) / 1000:.1f} km")
    d_cl, _ = graph_dist(nodes, edges, camp, S["lamp_ward"]["pos"])
    R.info(f"Camp to Lamp Ward along the Dry Meridian {(d_cl or 0) / 1000:.1f} km; the whole Dry Meridian {plen(RT['dry_meridian']['vertices']) / 1000:.1f} km")
    d_st, _ = graph_dist(nodes, edges, stn, S["roadside_terminal"]["pos"])
    R.info(f"station to roadside terminal {(d_st or 0) / 1000:.1f} km")
    d_cg, _ = graph_dist(nodes, edges, camp, S["marches_garrison"]["pos"])
    d_rg, _ = graph_dist(nodes, edges, reed, S["marches_garrison"]["pos"])
    R.info(f"Camp to garrison {(d_cg or 0) / 1000:.1f} km; Reedbank to garrison {(d_rg or 0) / 1000:.1f} km (road-class routes)")

    dmv = RT["dry_meridian"]["vertices"]
    R.check(dist_to_polyline(stn, dmv) <= 50, "inspection_station on the Dry Meridian", f"{dist_to_polyline(stn, dmv):.0f} m")
    R.check(dist_to_polyline(S["roadside_terminal"]["pos"], dmv) <= 50, "roadside_terminal on the Dry Meridian", f"{dist_to_polyline(S['roadside_terminal']['pos'], dmv):.0f} m")
    # order along the road
    def arc(p):
        best = min(range(len(dmv)), key=lambda i: math.hypot(dmv[i][0] - p[0], dmv[i][1] - p[1]))
        return plen(dmv[: best + 1])

    seq = [arc(S[k]["pos"]) for k in ("cistern_camp", "inspection_station", "roadside_terminal")]
    R.check(seq == sorted(seq), "Dry Meridian order: Cistern Camp, inspection station, roadside terminal", ", ".join(f"{v / 1000:.1f}" for v in seq))
    R.check(arc(S["lamp_ward"]["pos"]) >= max(seq), "Lamp Ward lies beyond the terminal on the Dry Meridian", f"{arc(S['lamp_ward']['pos']) / 1000:.1f} km")

    print("== corridor_p3 contents")
    def in_cor(p):
        return cor["min"][0] <= p[0] <= cor["max"][0] and cor["min"][1] <= p[1] <= cor["max"][1]

    ridge = S["station_ridge"]["pos"]
    dr = math.hypot(ridge[0] - stn[0], ridge[1] - stn[1])
    rel = dem.ring_relief(*ridge)
    R.check(in_cor(stn), "corridor_p3 contains the inspection station")
    R.check(in_cor(ridge), "corridor_p3 contains the ridge")
    R.check(dr <= 5000, "ridge within 5 km of the inspection station", f"{dr / 1000:.2f} km")
    R.check(rel >= RIDGE_RELIEF_MIN_M, f"ridge crest stands >= {RIDGE_RELIEF_MIN_M:g} m above its 1-2 km ring (PROPOSED test)", f"{rel:.0f} m; crest z {dem.z(*ridge):.0f} m")
    cr_in = [k for k in ("three_crossings_a", "three_crossings_b", "three_crossings_c") if in_cor(S[k]["pos"])]
    R.check(len(cr_in) >= 1, "corridor_p3 contains a crossing", f"{len(cr_in)} of 3: {', '.join(cr_in)}")

    print("== Three Crossings along the watercourse")
    tw = W["trunk_wadi"]["vertices"]
    cum = [0.0]
    for a, b in zip(tw, tw[1:]):
        cum.append(cum[-1] + math.hypot(b[0] - a[0], b[1] - a[1]))

    def chain(p):
        i = min(range(len(tw)), key=lambda i: math.hypot(tw[i][0] - p[0], tw[i][1] - p[1]))
        return cum[i], math.hypot(tw[i][0] - p[0], tw[i][1] - p[1])

    cs = {k: chain(S[k]["pos"]) for k in ("three_crossings_a", "three_crossings_b", "three_crossings_c")}
    R.check(all(v[1] <= 300 for v in cs.values()), "each crossing lies on the trunk wadi (within 300 m)", ", ".join(f"{k[-1]} {v[1]:.0f} m" for k, v in cs.items()))
    ks = list(cs)
    for i in range(3):
        for j in range(i + 1, 3):
            dd = abs(cs[ks[i]][0] - cs[ks[j]][0])
            R.check(5000 <= dd <= 20000, f"{ks[i][-1]}-{ks[j][-1]} along the watercourse 5-20 km", f"{dd / 1000:.1f} km")
    for k in ks:
        rts = [r["id"] for r in routes if dist_to_polyline(S[k]["pos"], r["vertices"]) <= 200]
        R.check(any(RT[i]["kind"] in ("road", "track") for i in rts), f"a road or track reaches {k}", ", ".join(rts))
    R.check(dist_to_polyline(S["marches_garrison"]["pos"], dmv) <= 1500, "marches_garrison within 1.5 km of the Dry Meridian", f"{dist_to_polyline(S['marches_garrison']['pos'], dmv):.0f} m")
    dm_cum = [0.0]
    for a, b in zip(dmv, dmv[1:]):
        dm_cum.append(dm_cum[-1] + math.hypot(b[0] - a[0], b[1] - a[1]))
    near_bed = [i for i, v in enumerate(dmv) if dist_to_polyline(v, tw) <= 300]
    runs, cur = [], []
    for i in near_bed:
        if cur and i != cur[-1] + 1:
            runs.append(cur)
            cur = []
        cur.append(i)
    if cur:
        runs.append(cur)
    R.info("Dry Meridian within 300 m of the trunk wadi thalweg at road km " + ", ".join(f"{dm_cum[r[0]] / 1000:.1f}-{dm_cum[r[-1]] / 1000:.1f}" for r in runs))
    gch = chain(S["marches_garrison"]["pos"])[0]
    R.info(f"garrison is {(gch - max(v[0] for v in cs.values())) / 1000:.1f} km downstream of the lower crossing along the wadi")

    print("== Reedbank and the excluded settlement")
    rc = W["reed_channel"]["vertices"]
    R.check(dist_to_polyline(reed, rc) <= 300, "Reedbank stands on the secondary channel (within 300 m)", f"{dist_to_polyline(reed, rc):.0f} m; catchment about {W['reed_channel']['catchment_km2_end']} km2 at its mouth, trunk about {W['trunk_wadi']['catchment_km2_end']} km2")
    spv = RT["reedbank_spur"]["vertices"]
    i0 = min(range(len(spv)), key=lambda i: math.hypot(spv[i][0] - reed[0], spv[i][1] - reed[1]))
    j0 = min(range(1, len(rc) - 1), key=lambda j: math.hypot(rc[j][0] - reed[0], rc[j][1] - reed[1]))
    tx, ty = rc[j0 + 1][0] - rc[j0 - 1][0], rc[j0 + 1][1] - rc[j0 - 1][1]

    def side(p):
        return tx * (p[1] - rc[j0][1]) - ty * (p[0] - rc[j0][0])

    before, after = spv[max(0, i0 - 3)], spv[min(len(spv) - 1, i0 + 3)]
    R.check(0 < i0 < len(spv) - 1 and side(before) * side(after) < 0, "reedbank_spur crosses the secondary channel at Reedbank (road continues to the far bank)", f"ends {math.hypot(spv[-1][0] - reed[0], spv[-1][1] - reed[1]):.0f} m beyond Reedbank")
    R.check(dist_to_polyline(S["excluded_settlement"]["pos"], RT["sera_approach"]["vertices"]) <= 100, "excluded_settlement on sera_approach", f"{dist_to_polyline(S['excluded_settlement']['pos'], RT['sera_approach']['vertices']):.0f} m")
    R.check(RT["sera_approach"]["vertices"][-1] == reed or math.hypot(RT["sera_approach"]["vertices"][-1][0] - reed[0], RT["sera_approach"]["vertices"][-1][1] - reed[1]) <= 100, "sera_approach ends at Reedbank")
    # exposure: how far each approach runs within sight of the open floor is not computed here; report plain distances
    ex_d = [dist_to_polyline(p, dmv) for p in RT["sera_approach"]["vertices"]]
    R.info(f"sera_approach stays {min(ex_d) / 1000:.1f}-{max(ex_d) / 1000:.1f} km from the Dry Meridian, mean {sum(ex_d) / len(ex_d) / 1000:.1f} km")

    print("== Inspection station setting")
    R.info(f"the post sits {-dem.ring_relief(*stn, r_in=300.0, r_out=800.0):.0f} m below the mean of the ground 300-800 m around it (negative = on a rise)")
    R.info(f"the road falls from {dem.z(*dmv[min(range(len(dmv)), key=lambda i: abs(arc(dmv[i]) - (arc(stn) - 20000)))]):.0f} m twenty km west of the post to {dem.z(*stn):.0f} m at the post")

    def los(a_xy, za, b_xy, zb, skip=150.0):
        run = math.hypot(b_xy[0] - a_xy[0], b_xy[1] - a_xy[1])
        n = max(2, int(run / 50))
        worst_ = -1e9
        for t in range(1, n):
            dd_ = run * t / n
            if dd_ < skip:
                continue
            p_ = (a_xy[0] + (b_xy[0] - a_xy[0]) * t / n, a_xy[1] + (b_xy[1] - a_xy[1]) * t / n)
            worst_ = max(worst_, (dem.z(*p_) - za) / dd_)
        return (zb - za) / run >= worst_ - 1e-9

    ze = dem.z(*ridge) + 2.0
    seen = [v for v in dmv if math.hypot(v[0] - ridge[0], v[1] - ridge[1]) > 200 and los(ridge, ze, v, dem.z(*v) + 2.0)]
    seen_km = [arc(v) / 1000 for v in seen]
    R.info(
        f"from the North Bluff crest (2 m eye) {len(seen)} of {len(dmv)} Dry Meridian vertices are in line of sight; "
        f"road km {min(seen_km):.1f} to {max(seen_km):.1f} (post at km {arc(stn) / 1000:.1f}); station itself in sight: {los(ridge, ze, stn, dem.z(*stn) + 2.0)}"
        if seen else "from the North Bluff crest no Dry Meridian vertex is in line of sight"
    )
    R.info(f"rim: ground at Reedbank {dem.z(*reed):.0f} m; highest ground within 6 km south of Reedbank {max(dem.z(reed[0] + dx, reed[1] - dy) for dx in range(-6000, 6001, 500) for dy in range(0, 6001, 500)):.0f} m")

    print("== Heliarch tour")
    tv = [(v["x_m"], v["y_m"], v["altitude_m"]) for v in tour["vertices"]]
    R.check(all(1000 <= v[2] <= 6000 for v in tv), "tour altitudes within the Size Canon band 1000-6000 m")
    R.check(all(inside_win(v) for v in tv), "tour vertices inside the window")
    tl = [(v[0], v[1]) for v in tv]
    speed = tour["speed_m_per_h"]
    HELIARCH_LENGTH_M = 2400  # ARCHON_SIZE_CANON, largest body dimension

    # dense tour: (x, y, altitude, distance along the tour), 100 m steps, altitude interpolated linearly
    dense_t, run_s = [], 0.0
    for a, b in zip(tv, tv[1:]):
        L_ = math.hypot(b[0] - a[0], b[1] - a[1])
        n_ = max(1, int(L_ / 100))
        for k in range(n_):
            f_ = k / n_
            dense_t.append((a[0] + (b[0] - a[0]) * f_, a[1] + (b[1] - a[1]) * f_, a[2] + (b[2] - a[2]) * f_, run_s + L_ * f_))
        run_s += L_
    dense_t.append((tv[-1][0], tv[-1][1], tv[-1][2], run_s))

    def closest(p):
        best_ = min(dense_t, key=lambda q: (q[0] - p[0]) ** 2 + (q[1] - p[1]) ** 2)
        hd = math.hypot(best_[0] - p[0], best_[1] - p[1])
        sl = math.hypot(hd, best_[2])
        return hd, best_[2], sl, math.degrees(2 * math.atan(HELIARCH_LENGTH_M / (2 * sl))), best_[3] / speed, best_

    dm_dense = []
    for a, b in zip(dmv, dmv[1:]):
        L_ = math.hypot(b[0] - a[0], b[1] - a[1])
        n_ = max(1, int(L_ / 100))
        dm_dense += [(a[0] + (b[0] - a[0]) * k / n_, a[1] + (b[1] - a[1]) * k / n_) for k in range(n_)]
    dm_dense.append(dmv[-1])
    hmin, pc = min(
        ((math.hypot(q[0] - p[0], q[1] - p[1]), q) for q in dense_t for p in dm_dense[::5]),
        key=lambda t: t[0],
    )
    hd, alt, slant, theta, t_h, qbest = closest(S["inspection_station"]["pos"])
    R.check(
        hmin <= 20000,
        "tour passes within 20 km (horizontal) of the Dry Meridian",
        f"closest {hmin / 1000:.1f} km, at tour point ({pc[0] / 1000:.1f}, {pc[1] / 1000:.1f}) km",
    )
    tour_len = plen(tl)
    R.info(f"tour length {tour_len / 1000:.1f} km; {tour_len / speed:.1f} h at {speed} m/h typical")
    R.info("closest approach of the tour line to key sites (altitude above ground interpolated; angle = 2*atan(2400 m / 2 / slant), Size Canon method):")
    for sid in ("cistern_camp", "inspection_station", "roadside_terminal", "three_crossings_a", "three_crossings_b", "three_crossings_c", "reedbank", "reserve_depot", "marches_garrison", "lamp_ward"):
        hd, alt, slant, theta, t_h, q = closest(S[sid]["pos"])
        R.info(f"  {sid:20s} horizontal {hd / 1000:5.1f} km, altitude {alt:5.0f} m, slant {slant / 1000:5.1f} km, {theta:5.1f} deg, tour hour {t_h:4.1f}")
    cum_t = 0.0
    for k, (a, b) in enumerate(zip(tv, tv[1:])):
        cum_t += math.hypot(b[0] - a[0], b[1] - a[1])
        R.info(f"  vertex {k + 1} ({b[0] / 1000:g}, {b[1] / 1000:g}) km alt {b[2]} m: {cum_t / speed:.1f} h after entry")
    # where the tour crosses the Dry Meridian road
    cross_pts = [(x, y) for (x, y, _, _), (x2, y2, _, _) in zip(dense_t, dense_t[1:]) for a, b in zip(dmv, dmv[1:]) if segments_cross((x, y), (x2, y2), a, b)]
    R.info("tour line crosses the Dry Meridian at " + (", ".join(f"({x / 1000:.1f}, {y / 1000:.1f}) km" for x, y in cross_pts) or "no point"))
    over = [k for k in ("three_crossings_a", "three_crossings_b", "three_crossings_c") if dist_to_polyline(S[k]["pos"], tl) <= 6000]
    R.info(f"crossings within 6 km (horizontal) of the tour line: {', '.join(over) or 'none'}")

    if not args.no_image:
        draw(np, h, e, win, sites, routes, water, tour, args)

    print(f"\n{'ALL CHECKS PASS' if R.failed == 0 else str(R.failed) + ' CHECK(S) FAILED'}")
    return 1 if R.failed else 0


if __name__ == "__main__":
    sys.exit(main())
