#!/usr/bin/env python3
"""Reproduce every derived number in docs/design/ACHLYDESA_EXPERIENCE_BIBLE.md.

Usage: python3 docs/artifacts/P0-03/affect_constants.py > docs/artifacts/P0-03/affect_constants.txt

Offline analysis only. Floats are used here to *derive* integer constants; the
simulation uses only the integers this script prints (ADR-0009).
"""
import math


# ---------------------------------------------------------------- helpers
def lut(knots, x):
    """Integer piecewise-linear interpolation, truncating toward zero, clamped at both ends."""
    if x <= knots[0][0]:
        return knots[0][1]
    for (x0, y0), (x1, y1) in zip(knots, knots[1:]):
        if x <= x1:
            return y0 + int((y1 - y0) * (x - x0) / (x1 - x0))
    return knots[-1][1]


def half_life_ppm(minutes):
    """Decay per one-minute tick, in parts per million, for a given half-life."""
    return round(1e6 * (1 - 2 ** (-1 / minutes)))


def trace_decay(t, ppm):
    """One tick of trace decay: t -= max(1, t*ppm/1e6)."""
    return t - max(1, t * ppm // 1_000_000) if t > 0 else 0


print("== 1. Trace decay constants (ppm per simulated minute) ==")
HL = {"awe_flinch_trace": 6 * 60, "grief_trace": 21 * 24 * 60, "hope_trace": 3 * 24 * 60}
for k, m in HL.items():
    print(f"{k}: half-life {m} min -> {half_life_ppm(m)} ppm/min")

# ---------------------------------------------------------------- atan LUT
print("\n== 2. Angular-size lookup: x_pm = 500*s/d ; theta_cdeg = 2*atan(x_pm/1000) in 0.01 deg ==")
X_KNOTS = [0, 50, 100, 200, 300, 400, 500, 600, 800, 1000, 1250, 1500, 1750, 2000, 2500, 3000, 3500, 4000, 5000, 6000, 7000, 8000]
ATAN = [(x, round(2 * math.degrees(math.atan(x / 1000)) * 100)) for x in X_KNOTS]
print("knots (x_pm, theta_cdeg):", ATAN)
worst = (0.0, 0)
for x in range(0, 8001):
    exact = 2 * math.degrees(math.atan(x / 1000)) * 100
    err = abs(lut(ATAN, x) - exact)
    if err > worst[0]:
        worst = (err, x)
print(f"max interpolation error over x_pm 0..8000: {worst[0]:.1f} cdeg at x_pm={worst[1]} (limit 50 cdeg = 0.5 deg)")


def theta_cdeg(s_m, ground_m, alt_m):
    d = math.isqrt(ground_m ** 2 + alt_m ** 2)
    return lut(ATAN, 500 * s_m // d)


print("\n== 3. Awe ladder: theta (cdeg) -> awe (0..900) ==")
AWE = [(0, 0), (30, 0), (100, 120), (300, 260), (700, 420), (1500, 600), (2500, 720), (4500, 820), (10000, 900)]
print("AWE_ANGLE knots:", AWE)
AGE = [(0, 1000), (6, 800), (48, 450), (240, 200), (720, 100)]
print("AWE_AGE knots (hours -> permille):", AGE)
cases = [
    ("heliarch overhead 3 km cruise", 2400, 0, 3000),
    ("heliarch 20 km ground, cruise 3 km", 2400, 20000, 3000),
    ("heliarch overhead 1 km (min alt)", 2400, 0, 1000),
    ("autophagos 10 km (largest dim 4000)", 4000, 10000, 0),
    ("anodyne 3 km ground, 1.5 km alt (bell 1000)", 1000, 3000, 1500),
    ("strategos wings 15 km ground, 2 km alt", 6000, 15000, 2000),
    ("pylaios 2 km", 300, 2000, 0),
    ("heliarch first sighting 120 km", 2400, 120000, 3000),
]
print("case | theta_cdeg (LUT) | exact deg | awe at age 0 h | awe at age 48 h")
for name, s, g, a in cases:
    th = theta_cdeg(s, g, a)
    exact = 2 * math.degrees(math.atan(s / (2 * math.sqrt(g * g + a * a))))
    base = lut(AWE, th)
    print(f"{name} | {th} | {exact:.2f} | {base} | {base * lut(AGE, 48) // 1000}")

print("\n== 4. Dread / tension / hope tables ==")
DREAD_OVERDUE = [(1000, 0), (2000, 150), (4000, 400), (8000, 650), (20000, 800)]
DREAD_DEADLINE = [(0, 350), (60, 250), (360, 100), (1440, 0)]
TENSION_URGENCY = [(0, 150), (60, 75), (240, 0)]
HOPE_KEPT_DELTA = [(0, 0), (1, 40), (5, 110), (20, 220), (100, 350)]
HOPE_KEPT_LEVEL = [(0, 0), (10, 60), (50, 130), (200, 220), (1000, 300)]
print("squad overdue ratio r (permille):", {r: lut(DREAD_OVERDUE, r) for r in (500, 1000, 3000, 6000, 20000)})
print("deadline minutes:", {m: lut(DREAD_DEADLINE, max(m, 0)) for m in (-5, 0, 30, 180, 720, 5000)})
print("tension urgency minutes:", {m: lut(TENSION_URGENCY, m) for m in (0, 30, 120, 500)})
print("hope kept delta:", {n: lut(HOPE_KEPT_DELTA, n) for n in (1, 3, 12, 60, 400)})
print("hope kept level:", {n: lut(HOPE_KEPT_LEVEL, n) for n in (0, 5, 31, 412, 2000)})
print("max sums: dread", 800 + 300 + 350 + 150, "-> clipped to 1000; tension", 400 + 250 + 300 + 150, "-> clipped to 1000")

print("\n== 5. Grief trace example (PROPOSED weights) ==")
BASE = {"soldier": 80, "named": 200, "founder": 300}
AFF = {"none": 500, "acquaintance": 800, "close": 1200, "kin": 1500}


def w_death(kind, aff):
    return BASE[kind] * AFF[aff] // 1000


print("w(soldier, close) =", w_death("soldier", "close"), "; w(founder, close) =", w_death("founder", "close"),
      "; w(named, acquaintance) =", w_death("named", "acquaintance"))
ppm = half_life_ppm(HL["grief_trace"])
t = min(1_000_000, 1000 * (3 * w_death("soldier", "close") + w_death("founder", "close")))
print("Roll: 3 soldiers (close) + 1 founder (close): trace hit =", t // 1000, "output units")
out = {}
for minute in range(0, 70 * 24 * 60 + 1):
    if minute % (24 * 60) == 0 and (minute // (24 * 60)) in (0, 7, 21, 42, 70):
        out[minute // (24 * 60)] = t // 1000
    t = trace_decay(t, ppm)
print("grief trace output by day after acknowledgement:", out, "(output = max(trace, floor); floor with 1 founder dead = 25)")

print("\n== 6. Flinch awe trace ==")
ppm = half_life_ppm(HL["awe_flinch_trace"])
f = 1_000_000
rows = {}
for minute in range(0, 7 * 24 * 60 + 1):
    if minute in (0, 360, 720, 1440, 2880, 10080):
        rows[minute / 60] = 900 + f // 10_000
    f = trace_decay(f, ppm)
print("awe floor (900 + trace/10000) by hours after coincidence:", rows)

print("\n== 7. Ink fade: contrast of quantized steps ==")


def hex2rgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i:i + 2], 16) for i in (0, 2, 4))


def lum(rgb):
    def c(v):
        v /= 255
        return v / 12.92 if v <= 0.03928 else ((v + 0.055) / 1.055) ** 2.4
    r, g_, b = (c(v) for v in rgb)
    return 0.2126 * r + 0.7152 * g_ + 0.0722 * b


def contrast(a, b):
    la, lb = sorted((lum(a), lum(b)), reverse=True)
    return (la + 0.05) / (lb + 0.05)


for label, ink, sec, paper in (("day", "#352F3F", "#655B68", "#E9DFC9"), ("night", "#E4DDC6", "#AEB5A8", "#222B29")):
    i, s, p = hex2rgb(ink), hex2rgb(sec), hex2rgb(paper)
    steps = []
    for k in range(8):
        rgb = tuple(i[c] + (s[c] - i[c]) * k // 7 for c in range(3))
        steps.append(round(contrast(rgb, p), 2))
    print(f"{label}: contrast vs Paper for steps 0..7:", steps)


def fade_step(age_min, expected_min):
    grace = max(2 * expected_min, 30)
    horizon = 96 * 60
    pm = max(0, min(1000, 1000 * (age_min - grace) // (horizon - grace)))
    return min(7, pm * 8 // 1000)


print("fade_step by age in hours (expected interval 6 h):", {h: fade_step(h * 60, 360) for h in (1, 12, 24, 48, 72, 96, 264)})

print("\n== 8. Sound arithmetic ==")
for c in (5, 12, 40):
    print(f"anodyne 880 Hz detuned +{c} cents -> {880 * 2 ** (c / 1200):.2f} Hz, beat {880 * (2 ** (c / 1200) - 1):.2f} Hz")
print("anodyne cents = 5 + 35*dread/1000:", {d: 5 + 35 * d // 1000 for d in (0, 500, 1000)})
print("autophagos bpm = 40 + tension/50:", {t_: 40 + t_ // 50 for t_ in (0, 500, 1000)})
print("strategos stations = 3 + awe/200:", {a: 3 + a // 200 for a in (0, 400, 900, 1000)})
slot = 0.36 + 0.40 + 1.5
print(f"strategos call slot = 0.36 s checks + 0.40 s tail + 1.5 s quiet = {slot:.2f} s; round of 8 = {8 * slot:.1f} s")
print("heliarch sweep 220 -> 55 Hz:", math.log2(220 / 55), "octaves in 14 s =", round(14 / math.log2(220 / 55), 1), "s per octave")
print("sub-bass gain dB = -48 + 36*awe/1000:", {a: -48 + 36 * a // 1000 for a in (0, 500, 900, 1000)})

print("\n== 9. Plate and palette arithmetic ==")
w, h = 480, 120
print("plate pixels", w * h, "; 2-bit bytes", w * h * 2 // 8, "; px per degree at 120 deg FOV", w / 120,
      "; vertical FOV deg", h / (w / 120))
for name, deg in (("pylaios first sighting 0.3 deg", 0.3), ("heliarch 20 km 6.8 deg", 6.8),
                  ("heliarch overhead 43.6 deg (fits inside the 120 deg FOV)", 43.6)):
    print(f"{name}: {deg * w / 120:.1f} px wide")
print("hue shift limit 30 per mille of the hue circle =", 30 * 360 / 1000, "degrees")
print("palette shift dh_pm = clamp((hope - max(dread, grief)) * 30 // 1000, -30, 30):",
      {(hp, d): max(-30, min(30, (hp - d) * 30 // 1000)) for hp, d in ((0, 0), (800, 0), (800, 1000), (0, 1000), (300, 400))})
