# Terrain candidates

**Status: PROPOSED — INCOMPLETE.** Not authoritative. Source decision: `docs/decisions/ADR-0100-terrain-source.md` (Proposed).

## What this document is not yet
The sandbox that produced it had no network path for DEM downloads and could not install Python packages. Therefore:
- **No candidate PNG exists.** `docs/canon/terrain_candidates/` is empty. Liam cannot judge by eye until the commands below are run.
- **Window-local feature coordinates (features 1–4) are not given.** Placing them requires looking at the data; inventing them would be fabrication. The tables say `TBD (render)`.
- **Elevation ranges are not given** for the same reason.
- `tools/scripts/hillshade.py` was written but **never run end to end** (only `--help` and `--list-tiles` were run).
- The regions below are chosen from general geographic knowledge, not from inspection of the data. Treat every "expected" statement as a hypothesis to confirm on the render.

## How to produce the renders
```
pip install -r tools/scripts/requirements.txt
mkdir -p data/raw/terrain docs/canon/terrain_candidates
# download tiles: commands in ADR-0100 §Download commands
python3 tools/scripts/hillshade.py --sw-lat 30.0 --sw-lon 34.6 --tiles data/raw/terrain/*.tif --out docs/canon/terrain_candidates/A-negev-arava.png
```
`--tiles` accepts any paths, so download one window at a time or pass only that window's tiles (names from `--list-tiles`). After first inspection, write a marks file (format in the script docstring) with features 1–4 in window-local km and re-run with `--marks <file>`; commit the marks files next to the PNGs. Output is 1500 × 1500 px (100 m per pixel), 10 km grid, 20 km scale bar, north arrow, tints Low `#DDD2B8` / Middle `#BBBBA4` / High `#9C979B` (Field Atlas §3.1, verified by grep: Low ground `#DDD2B8` at line 99; Middle and High were taken from the task spec).

Real-region names appear only so Liam can find them; none is for in-game use.

## Candidates (all PROPOSED; SW corner in WGS84 degrees, extent 150 × 150 km)

Window lat/lon extents are computed by `hillshade.py --list-tiles` (equirectangular about the window centre, R = 6371.0088 km).

| ID | Real region (reference only) | SW corner | NE corner (computed) | GLO-90 tiles |
|---|---|---|---|---|
| A | Central Negev, Arava rift, Edom escarpment | 30.0 N, 34.6 E | 31.349 N, 36.168 E | 6 |
| B | Hisma basin, Rum sandstone massifs, Ras en-Naqab | 29.0 N, 35.0 E | 30.349 N, 36.553 E | 4 |
| C | Southern Sinai: Feiran–Tih plateau margin | 28.3 N, 33.5 E | 29.649 N, 35.042 E | 6 |
| D | Harrat ash-Shaam basalt field, Azraq depression | 31.8 N, 36.5 E | 33.149 N, 38.099 E | 9 |

Expected character (hypotheses, from general geography, **unverified against the data**):

- **A.** Rift floor (graben) running north–south beside a high rim on the east, with erosion craters inside the Negev block. Hypothesis: road along the rift, rim as escarpment, a crater as contrasting high ground, several wadis breaching the rim. Feel: "a long pale graben under a stepped limestone rim".
- **B.** Flat sandy basin broken by isolated sandstone massifs, with a scarp on the north-west. Hypothesis: passable corridor across the basin; massifs as contrasting high ground; few perennial-style watercourses, so features 3–4 are the doubtful part. Feel: "a pale floor with black-shadowed towers standing in it".
- **C.** Rugged granite massif edge to a limestone plateau, wadi systems draining a high interior. Hypothesis: many wadis (good for 3–4), but the long mostly passable corridor (feature 1) is the doubtful part. Feel: "a high red-grey wall with dry rivers cut through it".
- **D.** Black basalt plateau around a flat evaporite basin. Hypothesis: very long passable corridor, basalt rim as escarpment, shallow wadis radiating into the basin; strong contrast between flat pale basin and dark rough field. Hypothesis weak on massifs and craters. Feel: "a pale flat under a black basalt rim".

### Per-candidate feature table (to be completed from the render)

For each of A–D:

| Feature | Placement (window-local km, x east, y north) |
|---|---|
| 1. Dry Meridian road corridor, >= 100 km | TBD (render) |
| 2. Escarpment or ridge near corridor | TBD (render) |
| 3. Secondary wadi the road crosses (Reedbank) | TBD (render) |
| 4. Three Crossings: three separable crossings within about 20 km | TBD (render) |
| 5. Contrasting high ground | TBD (render) |
| Elevation range (min / p5 / median / p95 / max) | printed by `hillshade.py`, TBD |

Reference for the features: Campaign Bible §Movements I–II. The route is "Cistern Camp to Lamp Ward, via the Dry Meridian road. Reedbank lies on a secondary river crossing." Three Crossings: "Keeping two communities connected and forcing the offensive to culminate" (both verified by grep, Campaign Bible lines 71 and 114).

## Provisional ranking (not evidence-based; replace after rendering)
This ordering reflects only how many required features each region is expected to supply, as hypothesized above.
1. **A** — most likely to supply all five features, including a crater.
2. **D** — best road and rim; weakest on features 3–5.
3. **C** — best wadi systems; weakest corridor.
4. **B** — strong high ground; weakest watercourses.

**Recommendation: none yet.** Provisionally A, pending Liam viewing the renders. Placement (P0-06) must not begin until a window is picked.

## Attribution (for the eventual credits)
Notice text and its source page are in ADR-0100; the full licence has not been read.
