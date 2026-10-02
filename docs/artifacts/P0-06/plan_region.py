#!/usr/bin/env python3
"""Planning evidence for task P0-06 (docs/canon/RED_LEDGER_REGION.md).

Scratch tool, not part of the build. It snaps hand-chosen waypoints to least-cost
paths on the transformed window and writes the three RON files. The verified,
reproducible checks live in tools/scripts/region_overlay.py; this file only
documents how the coordinates were found.

Method (same family as docs/artifacts/P0-05/analysis):
  * window A, GLO-30, sampled at 300 m for routing and 100 m for ridge/slope work;
  * game frame = real window rotated 270 deg counter-clockwise (= 90 deg clockwise),
    so real north points game east, real east points game south;
  * least-cost path, cost 1 + (slope/4 deg)^2, cells over 7.5 deg forbidden,
    +3 on cells whose D8 catchment exceeds 150 cells (13.5 km2) so roads cross
    drainage lines rather than run down them;
  * D8 flow on a priority-flood-filled grid gives the two watercourses.
Run from the repo root:  python docs/artifacts/P0-06/plan_region.py
"""

import glob
import heapq
import math
import os
import sys

import numpy as np
import rasterio

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
sys.path.insert(0, os.path.join(ROOT, "tools", "scripts"))
import hillshade as h  # noqa: E402

SW = (30.0, 34.6)
D8 = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
STEP_KM = 0.4

tiles = sorted(glob.glob(os.path.join(ROOT, "data", "raw", "terrain", "*.tif")))


def game_grid(px):
    e = h.sample_window(np, rasterio, tiles, SW[0], SW[1], 150.0, px)  # row0 north (real)
    return np.rot90(e, 3)  # rotate 270 ccw; row0 = game north


N = 500
G = game_grid(N)
CELL = 150000.0 / N
G100 = game_grid(1500)


def cell(x, y):
    return int(N - 1 - y / 0.3), int(x / 0.3)


def km(r, q):
    return (q + 0.5) * 0.3, (N - r - 0.5) * 0.3


def flow(e):
    f = e.copy()
    closed = np.zeros(e.shape, bool)
    pq = []
    for r in range(N):
        for q in (0, N - 1):
            heapq.heappush(pq, (f[r, q], r, q))
            closed[r, q] = True
    for q in range(N):
        for r in (0, N - 1):
            if not closed[r, q]:
                heapq.heappush(pq, (f[r, q], r, q))
                closed[r, q] = True
    order = []
    while pq:
        z, r, q = heapq.heappop(pq)
        order.append((r, q))
        for dr, dc in D8:
            r2, q2 = r + dr, q + dc
            if 0 <= r2 < N and 0 <= q2 < N and not closed[r2, q2]:
                closed[r2, q2] = True
                f[r2, q2] = max(f[r2, q2], z + 1e-4)
                heapq.heappush(pq, (f[r2, q2], r2, q2))
    recv = -np.ones((N, N, 2), int)
    acc = np.ones((N, N))
    for r, q in reversed(order):
        best, bd = None, 0
        for dr, dc in D8:
            r2, q2 = r + dr, q + dc
            if 0 <= r2 < N and 0 <= q2 < N:
                d = (f[r, q] - f[r2, q2]) / math.hypot(dr, dc)
                if d > bd:
                    bd, best = d, (r2, q2)
        if best:
            acc[best] += acc[r, q]
            recv[r, q] = best
    return recv, acc


recv, acc = flow(G)
gy, gx = np.gradient(G, CELL)
slope = np.degrees(np.arctan(np.hypot(gx, gy)))
cost = 1 + (slope / 4.0) ** 2
cost[slope > 7.5] = np.inf
cost = cost + 3.0 * (acc > 150)


def dijkstra(src):
    dist = np.full(cost.shape, np.inf)
    prev = -np.ones(cost.shape, dtype=np.int64)
    dist[src] = 0
    pq = [(0.0, src[0], src[1])]
    while pq:
        d, r, q = heapq.heappop(pq)
        if d > dist[r, q]:
            continue
        for dr, dc in D8:
            r2, q2 = r + dr, q + dc
            if 0 <= r2 < N and 0 <= q2 < N and np.isfinite(cost[r2, q2]):
                nd = d + math.hypot(dr, dc) * 0.5 * (cost[r, q] + cost[r2, q2])
                if nd < dist[r2, q2]:
                    dist[r2, q2] = nd
                    prev[r2, q2] = r * N + q
                    heapq.heappush(pq, (nd, r2, q2))
    return prev


def trace(prev, r, q):
    p = []
    while r >= 0:
        p.append((r, q))
        v = prev[r, q]
        r, q = (v // N, v % N) if v >= 0 else (-1, -1)
    return p[::-1]


def smooth(pk, k=4):
    pad = np.pad(pk, ((k, k), (0, 0)), mode="edge")
    ker = np.ones(2 * k + 1) / (2 * k + 1)
    s = np.stack([np.convolve(pad[:, i], ker, mode="valid") for i in range(2)], 1)
    s[0], s[-1] = pk[0], pk[-1]
    return s


def resample(pk, step=STEP_KM):
    L = np.r_[0, np.cumsum(np.hypot(*np.diff(pk, axis=0).T))]
    n = max(2, int(np.ceil(L[-1] / step)) + 1)
    t = np.linspace(0, L[-1], n)
    return np.stack([np.interp(t, L, pk[:, 0]), np.interp(t, L, pk[:, 1])], 1)


def route(wps):
    out = []
    for a, b in zip(wps, wps[1:]):
        seg = trace(dijkstra(cell(*a)), *cell(*b))
        out += seg if not out else seg[1:]
    pk = smooth(np.array([km(*t) for t in out]))
    pk[0], pk[-1] = wps[0], wps[-1]  # end exactly on the requested points, not on cell centres
    return resample(pk)


def line(a, b):
    a, b = np.array(a, float), np.array(b, float)
    n = max(2, int(np.ceil(np.hypot(*(b - a)) / STEP_KM)) + 1)
    return np.stack([np.linspace(a[0], b[0], n), np.linspace(a[1], b[1], n)], 1)


def trace_down(x, y):
    r, q = cell(x, y)
    out = [(r, q)]
    while True:
        r2, q2 = recv[r, q]
        if r2 < 0:
            break
        r, q = r2, q2
        out.append((r, q))
    return np.array([km(*t) for t in out]), np.array([acc[t] for t in out])


def arclen(pk):
    return np.r_[0, np.cumsum(np.hypot(*np.diff(pk, axis=0).T))]


def nearest(pk, x, y):
    return int(np.hypot(pk[:, 0] - x, pk[:, 1] - y).argmin())


def at_km(pk, s):
    return int(np.abs(arclen(pk) - s).argmin())


# ---- Dry Meridian ---------------------------------------------------------
dm = route([(10, 96.5), (39.5, 94.2), (71.5, 83.3), (86, 82.5), (106, 86)])
L = arclen(dm)
i_camp = 0
i_station = nearest(dm, 40.8, 94.2)
i_term = at_km(dm, L[i_station] + 6.0)
i_j = nearest(dm, 72.0, 83.3)
# trunk ford: first vertex east of J where the cell catchment exceeds 2000 cells
i_ford = next(i for i in range(i_j + 5, len(dm)) if acc[cell(*dm[i])] > 2000)
i_garr = nearest(dm, 84.6, 83.0)
i_lamp = len(dm) - 1

# ---- Watercourses ---------------------------------------------------------
trunk_xy, trunk_acc = trace_down(40.0, 93.1)
sT = arclen(trunk_xy)
reed_xy, reed_acc = trace_down(70.8, 64.0)
cut = int(np.argmax(reed_acc > 4500))  # first cell after the channel joins the trunk wadi (catchment > ~405 km2)
reed_xy, reed_acc = reed_xy[:cut], reed_acc[:cut]
sR = arclen(reed_xy)


def on_water(xy, s_arr, s):
    i = int(np.abs(s_arr - s).argmin())
    return xy[i]


# Three Crossings, along-channel positions on the trunk (km from trace start)
cross_s = [9.5, 17.0, 24.5]
crossings = [on_water(trunk_xy, sT, s) for s in cross_s]

# Reedbank: on the secondary channel, 15.3 km down from the rim spring
reedbank = on_water(reed_xy, sR, 15.3)

# ---- Spur, approach, crossing tracks --------------------------------------
spur_a = route([tuple(dm[i_j]), (72.6, 76.5), tuple(reedbank)])
# the spur arrives from the west and ends 1.6 km beyond Reedbank, on the far bank of the channel
spur = np.vstack([spur_a, line(spur_a[-1], (reedbank[0] + 1.6, reedbank[1] + 0.1))[1:]])

sera = route([tuple(dm[nearest(dm, 46.0, 92.4)]), (58.0, 77.0), tuple(reedbank)])
excluded = tuple(sera[nearest(sera, 58.0, 77.0)])  # on the approach itself

tracks = {}
for k, cp in zip("abc", crossings):
    # Dry Meridian vertex level with the crossing in x
    j = int(np.abs(dm[:, 0] - cp[0]).argmin())
    start = dm[j]
    d = cp - start
    d = d / np.hypot(*d)
    tracks[k] = np.vstack([line(start, cp), line(cp, cp + 1.2 * d)[1:]])

depot_i = int(np.abs(dm[:, 0] - 77.5).argmin())
depot = dm[depot_i] + np.array([0.0, -1.8])
depot_track = line(dm[depot_i], depot)

# ---- Ridge near the station (100 m grid) ----------------------------------
sx, sy = dm[i_station]


def relief(x, y, r_in=1.0, r_out=2.0):
    """Height of the 100 m cell at (x, y) above the mean of the 1-2 km ring around it."""
    P = 1500
    c, r = int(x / 0.1), int(P - 1 - y / 0.1)
    rr = np.arange(max(0, r - 20), min(P, r + 21))
    cc = np.arange(max(0, c - 20), min(P, c + 21))
    R, C = np.meshgrid(rr, cc, indexing="ij")
    d = np.hypot((R - r) * 0.1, (C - c) * 0.1)
    ring = (d >= r_in) & (d <= r_out)
    return float(G100[r, c] - G100[R[ring], C[ring]].mean())


best = None
for dx in np.arange(-4.5, 4.51, 0.3):
    for dy in np.arange(-4.5, 4.51, 0.3):
        if math.hypot(dx, dy) <= 4.5:
            v = relief(sx + dx, sy + dy)
            if best is None or v > best[0]:
                best = (v, sx + dx, sy + dy)
ridge = (best[1], best[2])
print("ridge relief", round(best[0], 1), "at", np.round(ridge, 2), "dist", round(math.hypot(ridge[0] - sx, ridge[1] - sy), 2))

# ---- Heliarch tour --------------------------------------------------------
tour = [
    (2.0, 146.0, 4000),
    (16.0, 128.0, 3000),
    (30.0, 112.0, 3000),
    (46.0, 100.0, 2000),
    (56.0, 95.0, 1000),
    (70.0, 90.0, 1500),
    (88.0, 80.0, 3000),
    (110.0, 66.0, 4000),
    (140.0, 50.0, 6000),
]


# ---- Write RON ------------------------------------------------------------
def m(v):
    return int(round(v * 1000 / 10.0) * 10)


def pt(p):
    return f"({m(p[0])}, {m(p[1])})"


def verts(pk, per_line=6):
    out = []
    for i in range(0, len(pk), per_line):
        out.append("      " + ", ".join(pt(p) for p in pk[i : i + per_line]) + ",")
    return "\n".join(out)


def rr(pk):
    return np.array([[m(a) / 1000, m(b) / 1000] for a, b in pk])


routes = [
    ("dry_meridian", "The Dry Meridian", "road", "intact", dm,
     "Cistern Camp to Lamp Ward; Basin freight road. Condition intact: it is the major freight road (World Bible States made from services)."),
    ("reedbank_spur", "Reedbank Relief Road", "road", "broken", spur,
     "Leaves the Dry Meridian at the junction and crosses the secondary channel at Reedbank. Broken at the start of Movement I: relief contracts are suspended (Campaign Bible section 4); reopening it is the inciting commitment."),
    ("sera_approach", "Sera's Rim Approach", "approach", "worn", sera,
     "Rim-foot approach to Reedbank through the excluded settlement; less exposed than the floor road, longer, and slower."),
    ("crossing_road_a", "Upper Crossing Track", "track", "worn", tracks["a"], "Dry Meridian to three_crossings_a and the north bank."),
    ("crossing_road_b", "Middle Crossing Road", "road", "intact", tracks["b"], "Dry Meridian to three_crossings_b and the north bank; the best-kept of the three."),
    ("crossing_road_c", "Lower Crossing Track", "track", "worn", tracks["c"], "Dry Meridian to three_crossings_c and the north bank."),
    ("depot_track", "Depot Track", "track", "worn", depot_track, "Dry Meridian to the reserve depot."),
]

os.makedirs(os.path.join(ROOT, "content", "world"), exist_ok=True)
with open(os.path.join(ROOT, "content", "world", "red_ledger_routes.ron"), "w", encoding="utf-8", newline="\n") as f:
    f.write("// Red Ledger routes. Status PROPOSED. Spec: docs/tasks/P0-06.md; narrative: docs/canon/RED_LEDGER_REGION.md.\n")
    f.write("// Frame: x east, y north, integer metres, origin at the SW corner of the transformed window (content/terrain/red_ledger/window.ron).\n")
    f.write("// Route vertices are at most 500 m apart. Generated by docs/artifacts/P0-06/plan_region.py, then checked by tools/scripts/region_overlay.py.\n")
    f.write("(\n  status: \"PROPOSED\",\n  routes: [\n")
    for rid, name, kind, cond, pk, note in routes:
        f.write(f"    (\n      id: \"{rid}\",\n      name: \"{name}\",\n      kind: {kind},\n      condition: {cond},\n      notes: \"{note}\",\n      vertices: [\n")
        f.write(verts(rr(pk)).replace("      ", "        ", 1) if False else "\n".join("  " + ln for ln in verts(rr(pk)).split("\n")))
        f.write("\n      ],\n    ),\n")
    f.write("  ],\n")
    f.write("  // Watercourses (PROPOSED): D8 thalwegs from the 300 m grid, used for the along-watercourse distances.\n")
    f.write("  watercourses: [\n")
    for wid, name, xy, a, note in (
        ("trunk_wadi", "The Trunk Wadi", trunk_xy, trunk_acc, "Main drainage of the floor; the Three Crossings sit on it. Trace starts at the confluence of the two western arms."),
        ("reed_channel", "The Reed Channel", reed_xy, reed_acc, "Secondary channel (about 300 km2) that rises at the rim and joins the trunk wadi; Reedbank stands on it."),
    ):
        # resample the trace to 0.4 km vertices, clipped to the window
        keep = [p for p in resample(xy, 0.4) if 0 <= p[0] <= 150 and 0 <= p[1] <= 150]
        f.write(f"    (\n      id: \"{wid}\",\n      name: \"{name}\",\n      catchment_km2_end: {int(round(a[-1] * 0.09))},\n      notes: \"{note}\",\n      vertices: [\n")
        f.write("\n".join("  " + ln for ln in verts(rr(np.array(keep))).split("\n")))
        f.write("\n      ],\n    ),\n")
    f.write("  ],\n")
    f.write("  heliarch_tour: (\n    id: \"heliarch_tour_1\",\n    name: \"The Announced Tour of the Second Sky\",\n")
    f.write("    altitude_basis: \"agl\",  // PROPOSED: metres above local ground; Size Canon bounds 1000 / 3000 / 6000 m\n")
    f.write("    speed_m_per_h: 20000,  // ARCHON_SIZE_CANON typical\n")
    f.write("    announced: \"PROPOSED: proclaimed before the caravan leaves Cistern Camp. The column covers the 30 km to the inspection point in about one to two days (High-Level Design section 4.1b: 90 km in three to five days), so the whale is announced to enter the window at the north-west corner at 08:00 on the day of the checkpoint. At 20,000 m/h it is nearest the inspection point 3.1 h later and over the Three Crossings 3.4 to 4.0 h later (tour hours printed by tools/scripts/region_overlay.py).\",\n")
    f.write("    canon: \"ACHLYDESA_CAMPAIGN_AND_CHARACTER_BIBLE.md section 5 Movement III (announced tour); path PROPOSED\",\n")
    f.write("    vertices: [\n")
    for x, y, a in tour:
        f.write(f"      (x_m: {m(x)}, y_m: {m(y)}, altitude_m: {a}),\n")
    f.write("    ],\n  ),\n)\n")

# sites
S = "ACHLYDESA_CAMPAIGN_AND_CHARACTER_BIBLE.md"
W = "achlydesa-world-bible.md"
sites = [
    ("cistern_camp", "Cistern Camp", "camp", dm[i_camp],
     f"{W} §Households and distance",
     "Start of the caravan's route; the seasonal arrival of the northbound convoy is a neighbourhood event. Position PROPOSED."),
    ("inspection_station", "The Meridian Inspection Point", "checkpoint", dm[i_station],
     f"{S} §4 The opening: the Red Ledger incident",
     "Captain Varo Kest's checkpoint on the Dry Meridian. Name and position PROPOSED. A low ridge stands within 5 km on the north side (station_ridge)."),
    ("station_ridge", "The North Bluff", "ridge", np.array(ridge),
     "PROPOSED",
     "Low ridge beside the checkpoint: cover for a covered withdrawal and a vantage over the road. Added by this task for the corridor_p3 ridge requirement."),
    ("roadside_terminal", "The Milestone Terminal", "terminal", dm[i_term],
     f"{S} §4 The opening: the Red Ledger incident",
     "The old roadside terminal that recognises a rejected travel claim. Name and position PROPOSED."),
    ("reedbank", "Reedbank", "town", np.array(reedbank),
     f"{S} §4 The opening: the Red Ledger incident",
     f"The town that sheltered the caravan, on a secondary river crossing (the Reed Channel, catchment about {int(round(reed_acc[int(np.abs(sR - 15.3).argmin())] * 0.09))} km2 at the town). Position PROPOSED."),
    ("excluded_settlement", "Ostrakon", "settlement", np.array(excluded),
     f"{S} §5 Movement I — People under protection",
     "Excluded settlement on Sera's less exposed approach. Name (a potsherd, the old voting token) and position PROPOSED."),
    ("reserve_depot", "Ninth Reserve Depot", "depot", depot,
     f"{S} §5 Movement I — People under protection",
     "Olan's supposedly empty reserve depot. Name and position PROPOSED."),
    ("marches_garrison", "The Ford Garrison", "garrison", dm[i_garr] + np.array([0.0, -1.2]),
     f"{S} §5 Movement II — A name for the army",
     "Marches garrison whose supply guarantees have failed; stands on the south bank 1.2 km off the Dry Meridian, above the two places (road km 75 and km 80) where the road meets the bed of the Trunk Wadi. Name and position PROPOSED."),
    ("three_crossings_a", "Upper Crossing", "crossing", crossings[0],
     f"{S} §5 Movement II — A name for the army",
     "Upstream-most of the Three Crossings on the Trunk Wadi. Names and positions PROPOSED."),
    ("three_crossings_b", "Middle Crossing", "crossing", crossings[1], f"{S} §5 Movement II — A name for the army", "Middle crossing."),
    ("three_crossings_c", "Lower Crossing", "crossing", crossings[2], f"{S} §5 Movement II — A name for the army", "Downstream-most crossing."),
    ("lamp_ward", "Lamp Ward", "ward", dm[i_lamp],
     f"{W} §The Lamp Concord",
     "Care settlement near the opening campaign's road. Position PROPOSED, at the east end of the Dry Meridian on the dry basin shore."),
]
with open(os.path.join(ROOT, "content", "world", "red_ledger_sites.ron"), "w", encoding="utf-8", newline="\n") as f:
    f.write("// Red Ledger sites. Status PROPOSED. Spec: docs/tasks/P0-06.md; narrative: docs/canon/RED_LEDGER_REGION.md.\n")
    f.write("// Frame: x east, y north, integer metres, origin at the SW corner of the transformed window.\n")
    f.write("// `canon` cites the document that establishes the site; positions and invented names are PROPOSED either way.\n")
    f.write("(\n  status: \"PROPOSED\",\n  sites: [\n")
    for sid, name, kind, p, canon, note in sites:
        f.write(f"    (id: \"{sid}\", name: \"{name}\", kind: \"{kind}\", pos: ({m(p[0])}, {m(p[1])}),\n     canon: \"{canon}\",\n     notes: \"{note}\"),\n")
    f.write("  ],\n)\n")

print("DM km", round(L[-1], 1), "station km", round(L[i_station], 1), "J km", round(L[i_j], 1), "ford km", round(L[i_ford], 1))
