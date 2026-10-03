#!/usr/bin/env python3
"""P0-07 evidence: where is the caravan column when the Heliarch's tour passes?

Inputs (read, not copied): content/world/red_ledger_routes.ron (dry_meridian polyline,
heliarch_tour_1 vertices), content/world/red_ledger_sites.ron (inspection_station).
Model (PROPOSED): the column leaves the inspection point at 15:00 on the checkpoint day
(day 0) and walks the Dry Meridian at a constant pace; the tour enters the window at
08:00 on ENTRY_DAY. Angular size = 2*atan(2400 m / 2 / slant), the size canon method
(ARCHON_SIZE_CANON section 1). Offline analysis only: floats are allowed here, never in
the simulation.

Run: python3 docs/artifacts/P0-07/onboarding_timeline.py
"""
import math
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[3]
routes = (ROOT / "content/world/red_ledger_routes.ron").read_text()
sites = (ROOT / "content/world/red_ledger_sites.ron").read_text()


def route_vertices(rid):
    i = routes.index(f'id: "{rid}"')
    j = routes.index("vertices: [", i)
    k = routes.index("],\n", j)
    return [(int(a), int(b)) for a, b in re.findall(r"\((\d+), (\d+)\)", routes[j:k])]


dm = route_vertices("dry_meridian")
cum = [0.0]
for (x0, y0), (x1, y1) in zip(dm, dm[1:]):
    cum.append(cum[-1] + math.hypot(x1 - x0, y1 - y0))


def road_point(s):
    s = max(0.0, min(s, cum[-1]))
    for i in range(1, len(cum)):
        if s <= cum[i]:
            f = (s - cum[i - 1]) / (cum[i] - cum[i - 1])
            return (dm[i - 1][0] + f * (dm[i][0] - dm[i - 1][0]),
                    dm[i - 1][1] + f * (dm[i][1] - dm[i - 1][1]))
    return dm[-1]


def road_km_of(site_id):
    m = re.search(rf'id: "{site_id}".*?pos: \((\d+), (\d+)\)', sites, re.S)
    p = (int(m.group(1)), int(m.group(2)))
    i = min(range(len(dm)), key=lambda n: math.hypot(dm[n][0] - p[0], dm[n][1] - p[1]))
    return cum[i]


s_post = road_km_of("inspection_station")
s_term = road_km_of("roadside_terminal")

t = routes[routes.index('id: "heliarch_tour_1"'):]
tour = [(int(a), int(b), int(c)) for a, b, c in
        re.findall(r"x_m: (\d+), y_m: (\d+), altitude_m: (\d+)", t)]
tcum = [0.0]
for a, b in zip(tour, tour[1:]):
    tcum.append(tcum[-1] + math.hypot(b[0] - a[0], b[1] - a[1]))
SPEED = 20000.0  # m/h, size canon typical
BODY = 2400.0    # m, body length


def whale(h):
    s = max(0.0, min(h * SPEED, tcum[-1]))
    for i in range(1, len(tcum)):
        if s <= tcum[i]:
            f = (s - tcum[i - 1]) / (tcum[i] - tcum[i - 1])
            return tuple(tour[i - 1][k] + f * (tour[i][k] - tour[i - 1][k]) for k in range(3))
    return tour[-1]


def theta(h, xy):
    w = whale(h)
    d = math.sqrt((w[0] - xy[0]) ** 2 + (w[1] - xy[1]) ** 2 + w[2] ** 2)
    return math.degrees(2 * math.atan(BODY / 2 / d)), d


print(f"inspection point at road km {s_post/1000:.1f}; terminal at road km {s_term/1000:.1f}; "
      f"road length {cum[-1]/1000:.1f} km")
print(f"tour length {tcum[-1]/1000:.1f} km, {tcum[-1]/SPEED:.2f} h")
print()
s_reed_junction = 65100.0  # RED_LEDGER_REGION section 6: the column turns off at road km 65.1
SPUR = 10900.0             # reedbank_spur length, RED_LEDGER_REGION section 3
print("Beat 6 window: terminal to Reedbank = (junction - terminal) + spur, at each pace")
for pace in (18, 24, 30):
    d = (s_reed_junction - s_term) + SPUR
    print(f"  pace {pace} km/d: {d/1000:.1f} km / {pace} km/d = {d/1000/pace:.2f} d = {d/1000/pace*24:.0f} h")
print()
LEAVE_H = 15.0  # column leaves the post at 15:00 on day 0 (PROPOSED)
for pace in (18, 24, 30):  # km/day, High-Level Design 4.1b band
    for entry_day in (0, 1, 2, 3):
        entry_h = entry_day * 24 + 8.0
        best = None
        first = None
        for step in range(0, int(8.6 * 60) + 1):
            h = step / 60.0
            now = entry_h + h
            s = s_post + max(0.0, now - LEAVE_H) / 24.0 * pace * 1000
            th, d = theta(h, road_point(s))
            if first is None and th >= 1.0:
                first = (h, th, d, s)
            if best is None or th > best[1]:
                best = (h, th, d, s)
        fd = (f"1 deg at tour h {first[0]:.1f} (road km {first[3]/1000:.1f}, {first[2]/1000:.0f} km)"
              if first else "never 1 deg")
        print(f"pace {pace:2d} km/d  entry day {entry_day}: peak {best[1]:5.1f} deg at tour h "
              f"{best[0]:.1f}, road km {best[3]/1000:5.1f}, slant {best[2]/1000:5.1f} km; {fd}")
    print()
