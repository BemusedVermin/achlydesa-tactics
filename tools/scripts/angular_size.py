#!/usr/bin/env python3
"""Compute every derived number in docs/canon/ARCHON_SIZE_CANON.md.

Usage: python3 tools/scripts/angular_size.py content/archons/size_canon.ron

Formulas (P0-04):
  theta = 2*atan(s / 2d), degrees, one decimal.
    s = largest body dimension (for a river with a length range: its width, because
        the along-bank extent fills the horizon); d = sqrt(ground^2 + altitude^2),
        altitude = cruise altitude for airborne bodies, 0 for ground bodies.
  d_h = 3.57*(sqrt(h_obs) + sqrt(h_target)) km, h in meters, h_obs = 2 m.
    h_target = cruise altitude + body height (airborne) or body height (ground).
  effective = min(d_h, haze_visibility_km).
  chunk = 1.92 km (see docs/tasks/P1-06.md). Footprint = footprint_m, else the
  ground projection (length x width) of an airborne body.
Offline analysis only; floats are allowed here (never in simulation crates).
"""
import math
import re
import sys

CHUNK_KM = 1.92
H_OBS = 2.0
DISTANCES_KM = [2, 5, 10, 20, 40]


def tokenize(text):
    text = re.sub(r"//[^\n]*", "", text)
    return re.findall(r'"(?:[^"\\]|\\.)*"|-?\d+|[A-Za-z_]\w*|[()\[\]:,]', text)


def parse(tokens):
    pos = 0

    def value():
        nonlocal pos
        t = tokens[pos]
        if t.startswith('"'):
            pos += 1
            return t[1:-1]
        if re.fullmatch(r"-?\d+", t):
            pos += 1
            return int(t)
        if t == "[":
            pos += 1
            out = []
            while tokens[pos] != "]":
                out.append(value())
                if tokens[pos] == ",":
                    pos += 1
            pos += 1
            return out
        if t == "(":
            pos += 1
            out = {}
            while tokens[pos] != ")":
                key = tokens[pos]
                assert tokens[pos + 1] == ":", f"expected ':' after {key}"
                pos += 2
                out[key] = value()
                if tokens[pos] == ",":
                    pos += 1
            pos += 1
            return out
        if t == "None":
            pos += 1
            return None
        if t == "Some":
            pos += 1
            assert tokens[pos] == "("
            pos += 1
            v = value()
            assert tokens[pos] == ")"
            pos += 1
            return v
        raise ValueError(f"unexpected token {t}")

    return value()


def theta(s, d):
    return 2 * math.degrees(math.atan(s / (2 * d)))


def fmt(x):
    return f"{x:.1f}"


def dist(ground_m, alt_m):
    return math.hypot(ground_m, alt_m)


def main_dim(a):
    b = a["body"]
    if a.get("length_range_m"):
        return b["width_m"]
    return max(b["length_m"], b["width_m"], b["height_m"])


def cruise(a):
    return a["altitude_m"]["cruise"] if a["altitude_m"] else 0


def md_table(header, rows):
    out = ["| " + " | ".join(header) + " |", "|" + "|".join("---" for _ in header) + "|"]
    out += ["| " + " | ".join(str(c) for c in r) + " |" for r in rows]
    return "\n".join(out)


def main():
    data = parse(tokenize(open(sys.argv[1], encoding="utf-8").read()))
    A = data["archons"]
    out = []

    out.append("#### Table 1: angular size of the largest dimension (degrees)")
    out.append("")
    out.append("Ground distance in km; airborne bodies use slant distance at cruise altitude. Mneme uses width.")
    out.append("")
    rows = []
    for a in A:
        s, alt = main_dim(a), cruise(a)
        rows.append([a["id"], s] + [fmt(theta(s, dist(k * 1000, alt))) for k in DISTANCES_KM])
    out.append(md_table(["archon", "s (m)"] + [f"{k} km" for k in DISTANCES_KM], rows))
    out.append("")

    out.append("#### Table 2: angular height (degrees)")
    out.append("")
    out.append("Same distances; s = body height. For Autophagos this is the shell summit.")
    out.append("")
    rows = []
    for a in A:
        s, alt = a["body"]["height_m"], cruise(a)
        rows.append([a["id"], s] + [fmt(theta(s, dist(k * 1000, alt))) for k in DISTANCES_KM])
    out.append(md_table(["archon", "height (m)"] + [f"{k} km" for k in DISTANCES_KM], rows))
    out.append("")

    out.append("#### Table 3: directly overhead (airborne bodies, s = largest dimension)")
    out.append("")
    rows = []
    for a in A:
        if a["altitude_m"]:
            s = main_dim(a)
            al = a["altitude_m"]
            rows.append([a["id"], s] + [f"{al[k]} m: {fmt(theta(s, al[k]))}" for k in ("min", "cruise", "max")])
    out.append(md_table(["archon", "s (m)", "at min alt", "at cruise alt", "at max alt"], rows))
    out.append("")

    out.append("#### Table 4: first-sighting distance (km)")
    out.append("")
    out.append("d_h = 3.57*(sqrt(2) + sqrt(h_target)); effective = min(d_h, haze).")
    out.append("")
    rows = []
    for a in A:
        ht = cruise(a) + a["body"]["height_m"]
        dh = 3.57 * (math.sqrt(H_OBS) + math.sqrt(ht))
        eff = min(dh, a["haze_visibility_km"])
        limit = "haze" if dh > a["haze_visibility_km"] else "horizon"
        # angular size of the largest dimension at the effective distance, ground distance
        ang = theta(main_dim(a), dist(eff * 1000, cruise(a)))
        rows.append([a["id"], ht, f"{dh:.1f}", a["haze_visibility_km"], f"{eff:.1f}", limit, fmt(ang)])
    out.append(md_table(["archon", "h_target (m)", "d_h", "haze", "effective", "limited by", "angle there (deg)"], rows))
    out.append("")

    out.append("#### Table 5: atlas footprint in chunks (1 chunk = 1.92 km)")
    out.append("")
    rows = []
    for a in A:
        fp = a["footprint_m"] or {"length_m": a["body"]["length_m"], "width_m": a["body"]["width_m"]}
        kind = "ground contact" if a["footprint_m"] else "shadow projection"
        l, w = fp["length_m"] / 1000 / CHUNK_KM, fp["width_m"] / 1000 / CHUNK_KM
        rng = a.get("length_range_m")
        lt = f"{l:.2f}" if not rng else f"{rng['min'] / 1000 / CHUNK_KM:.0f} to {rng['max'] / 1000 / CHUNK_KM:.0f}"
        rows.append([a["id"], kind, lt, f"{w:.2f}"])
    out.append(md_table(["archon", "footprint kind", "length (chunks)", "width (chunks)"], rows))
    out.append("")

    out.append("#### Table 6: scale contrasts")
    out.append("")
    rows = []
    for a in A:
        sc = a["scale_contrast"]
        if sc:
            alt = cruise(a)
            for k in (5, 15):
                d = dist(k * 1000, alt)
                rows.append([a["id"], f"{sc['small_part']} {sc['small_m']} m", f"{sc['large_part']} {sc['large_m']} m",
                             f"{sc['large_m'] // sc['small_m']}:1" if sc['large_m'] % sc['small_m'] == 0 else f"{sc['large_m'] / sc['small_m']:.1f}:1",
                             f"{k} km", fmt(theta(sc['small_m'], d)), fmt(theta(sc['large_m'], d))])
    out.append(md_table(["archon", "small", "large", "ratio", "ground dist", "small (deg)", "large (deg)"], rows))
    out.append("")

    out.append("#### Table 7: Execution Plan §2.2 reproduction")
    out.append("")
    by = {a["id"]: a for a in A}

    def ang(i, s, g_km):
        return theta(s, dist(g_km * 1000, cruise(by[i])))

    checks = [
        ("heliarch", "overhead at 3 km", 44, theta(2400, by["heliarch"]["altitude_m"]["cruise"])),
        ("heliarch", "20 km", 7, ang("heliarch", 2400, 20)),
        ("autophagos", "5 km, summit height", 20, ang("autophagos", 1800, 5)),
        ("autophagos", "10 km, summit height", 10, ang("autophagos", 1800, 10)),
        ("autophagos", "40 km, summit height", 2.6, ang("autophagos", 1800, 40)),
        ("anodyne", "3 km", 17, ang("anodyne", 1000, 3)),
        ("strategos", "15 km", 23, ang("strategos", 6000, 15)),
        ("pylaios", "2 km", 8.5, ang("pylaios", 300, 2)),
    ]
    rows = [[i, w, p, fmt(v), f"{round(v - p, 1) + 0.0:+.1f}", "ok" if abs(v - p) <= 1 else "FAIL"] for i, w, p, v in checks]
    out.append(md_table(["archon", "case", "plan (deg)", "computed (deg)", "delta", "within 1 deg"], rows))
    print("\n".join(out))


if __name__ == "__main__":
    main()
