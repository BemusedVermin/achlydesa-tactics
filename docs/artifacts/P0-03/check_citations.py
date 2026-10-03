#!/usr/bin/env python3
"""Check that every `<Source> §<section>` citation in the Experience Bible resolves to a heading.

Usage: python3 docs/artifacts/P0-03/check_citations.py
A citation resolves when its section number matches a heading's number, or its section
name is contained in (or contains) a heading's text, case-insensitively.
"""
import re

D = "docs/design/"
FILES = {
    "World Bible": "achlydesa-world-bible.md",
    "Campaign Bible": "ACHLYDESA_CAMPAIGN_AND_CHARACTER_BIBLE.md",
    "Field Atlas": "ACHLYDESA_FIELD_ATLAS_STYLE_AND_UI_BIBLE.md",
    "Technical Design": "ACHLYDESA_AUTONOMOUS_SQUAD_TECHNICAL_DESIGN.md",
    "Simulation": "ACHLYDESA_SIMULATION_AND_PLAYER_INTERACTIONS.md",
    "Execution Plan": "ACHLYDESA_EXECUTION_PLAN.md",
}
heads = {}
for key, name in FILES.items():
    with open(D + name, encoding="utf-8") as fh:
        heads[key] = [ln.lstrip("#").strip() for ln in fh if ln.startswith("#")]

with open(D + "ACHLYDESA_EXPERIENCE_BIBLE.md", encoding="utf-8") as fh:
    doc = fh.read()

pat = re.compile(
    r"(World Bible|Campaign Bible|Field Atlas|Technical Design|Simulation|Execution Plan) §([^\n]*?)"
    r"(?=[;)]|\.\s|\.$|,\s(?:and|the|[a-z])| \(|\"|\s—|$)"
)
seen = set()
for m in pat.finditer(doc):
    seen.add((m.group(1), m.group(2).strip().rstrip(":")))

unresolved = []
for src, sec in sorted(seen):
    s = sec.lower()
    ok = False
    num = re.match(r"^(\d+(?:\.\d+)?)\b", sec)
    for h in heads[src]:
        hl = h.lower()
        if num and re.match(r"^" + re.escape(num.group(1)) + r"[\. ]", h):
            ok = True
        if s and (s in hl or hl in s):
            ok = True
    if not ok:
        unresolved.append((src, sec))

print(f"{len(seen)} unique citations; {len(unresolved)} unresolved")
for src, sec in unresolved:
    print(f"  UNRESOLVED: {src} §{sec}")
