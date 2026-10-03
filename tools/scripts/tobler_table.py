#!/usr/bin/env python3
"""Generate content/movement/profiles.ron, including the `foot` speed table.

Usage: python3 tools/scripts/tobler_table.py [output_path]

Tobler's hiking function: v(s) = 6 * exp(-3.5 * |s + 0.05|) km/h, s = slope as a fraction.
Sampled at s = -600 permille .. +600 permille in 20 permille steps (61 entries); each value is
converted to mm/s and rounded to nearest (halves up). Flat (s = 0) gives 1399 mm/s.
Spec: docs/tasks/P1-07.md. Offline tooling only; floats are allowed here (never in
simulation crates). Output is deterministic.
"""
import math
import sys

DEFAULT_OUT = "content/movement/profiles.ron"
MM_S_PER_KM_H = 1_000_000 / 3600


def tobler_mm_s(slope_permille):
    kmh = 6.0 * math.exp(-3.5 * abs(slope_permille / 1000 + 0.05))
    return math.floor(kmh * MM_S_PER_KM_H + 0.5)


def render():
    table = [(s, tobler_mm_s(s)) for s in range(-600, 601, 20)]
    assert dict(table)[0] == 1399
    rows = []
    for i in range(0, len(table), 6):
        rows.append("        " + ", ".join(f"({s}, {v})" for s, v in table[i : i + 6]) + ",")
    body = "\n".join(rows)
    return f"""// Movement profiles. Spec: docs/tasks/P1-07.md. The speed table is generated; do not hand-edit.
// speed_table entries are (slope permille, mm/s). Factors are permille.
(
  profiles: {{
    "foot": (
      speed_table: [
{body}
      ],
      max_abs_slope_permille: 600,
      offroad_factor_permille: 900,
      road_factor_permille: {{ "intact": 1100, "worn": 1000, "broken": 850 }},
    ),
  }},
  generated_by: "tools/scripts/tobler_table.py",
)
"""


def main():
    out = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_OUT
    with open(out, "w", newline="\n") as f:
        f.write(render())


if __name__ == "__main__":
    main()
