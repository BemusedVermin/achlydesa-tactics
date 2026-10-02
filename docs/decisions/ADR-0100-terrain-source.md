# ADR-0100 — Terrain source

**Status:** Proposed
**Date:** 2026-10-01 · **Conflict:** none (new decision; task P0-05)

## Context
- The theater is built from real elevation data of an arid, Levant-like region, then remixed: "rotate, mirror, stitch". The plan names SRTM and Copernicus GLO-30 and says to check attribution terms (Execution Plan §4.5).
- The theater is about 1,000–1,500 km across at literal scale (High-Level Design §4.1; stale until P0-02, used here only because nothing above covers it). Source resolution needs only to seed the terrain; detail below the source resolution is procedural (Execution Plan §4.5).
- The game never uses real place names (task P0-05 spec). Attribution therefore belongs in credits and the data manifest, not in the world.

## Verification status of this ADR
Retrieval date for every provider page below: **2026-10-01**. The Copernicus and AWS pages were fetched with `curl` and their HTML text read directly, so strings quoted from them are verbatim (characters such as © arrive as `�` in the dump; the sign is shown here as ©). All 21 GLO-90 tiles needed for windows A-D were then downloaded from the public bucket (about 93 MB) and opened with `rasterio`: every tile is 1200 × 1200 float32 EPSG:4326, none has a nodata tag, none contains a NaN, and the lowest value across them is -427.1 m. The USGS SRTM page returned HTTP 403 to `curl` (CDN block), so **SRTM statements are still unverified against the provider's own text**; SRTM is not the recommended option.

## Options compared

| | SRTM 1-arc-second (USGS) | Copernicus GLO-30 | Copernicus GLO-90 (survey only) |
|---|---|---|---|
| Resolution | 1 arc-second (about 30 m) *(unverified: page blocked)* | 30 m ("GLO-30 offers global coverage at a resolution of 30 metres", CDSE page) | 90 m ("GLO-90 ... 90 metres", same page) |
| Format / tiling | DTED, BIL or GeoTIFF; 1° × 1° *(unverified)* | Cloud Optimized GeoTIFF, 1° × 1° tiles, 3600 px high; width 3600 px at 0-50° latitude, narrower further north (AWS readme) | COG, 1° × 1° tiles, 1200 px high; 1200 × 1200 px at these latitudes (measured); about 5.1 MB per tile |
| Pixel registration | not checked | pixel centres fall on whole degrees, so the GeoTIFF origin is half a pixel outside the tile (measured on GLO-90; the script uses the file transform) | same |
| Voids | not checked | not stated on the pages read; none found in the 21 GLO-90 tiles read (0 NaN, no nodata tag). GLO-30 tiles were not downloaded | none found (see left) |
| Water | not checked | "ocean areas do not have tiles", assume height 0 (AWS readme); the render of window C shows the resulting flat 0 m sea | same |
| Download, no account | USGS directs to EarthExplorer (login not verified) | **Yes.** Public S3 bucket `copernicus-dem-30m`, region eu-central-1, anonymous HTTPS and `--no-sign-request` | Public bucket `copernicus-dem-90m`; **downloaded successfully** (21 tiles) |
| URL pattern | none stable without login | `https://copernicus-dem-30m.s3.eu-central-1.amazonaws.com/<key>/<key>.tif` | `https://copernicus-dem-90m.s3.eu-central-1.amazonaws.com/<key>/<key>.tif`, key `Copernicus_DSM_COG_30_<N/S><lat>_00_<E/W><lon>_00_DEM` |
| Licence | public domain, cite DOI 10.5066/F7PR7TFT *(unverified: page blocked)* | free licence; mandatory credit notices (below) | same |
| Surface type | radar surface model | surface model (DSM), includes "buildings, infrastructure and vegetation" (CDSE page) | same |

Sources, all retrieved 2026-10-01:
- Copernicus DEM on AWS: https://registry.opendata.aws/copernicus-dem/ . Licence line, verbatim: "GLO-30 Public and GLO-90 are available on a free basis for the general public under the terms and conditions of the Licence". The link behind "Licence" is the CDSE page below.
- Copernicus DEM, CDSE collection page, which carries the licence terms: https://dataspace.copernicus.eu/explore-data/data-collections/copernicus-contributing-missions/collections-description/COP-DEM . Verbatim: "The GLO-30 and GLO-90 datasets are available worldwide with a free license." Under *User Obligations*, for unmodified GLO-30 data: "the User shall inform the General Public of the source by using the following notice: © DLR e.V. 2010-2014 and © Airbus Defence and Space GmbH 2014-2018 provided under COPERNICUS by the European Union and ESA; all rights reserved". For adapted data the notice begins "produced using Copernicus WorldDEM-30 © DLR e.V. 2010-2014 and © Airbus Defence and Space GmbH 2014-2018 provided under COPERNICUS by the European Union and ESA; all rights reserved"; for GLO-90 the same with "WorldDEM-90". Research citation requested: https://doi.org/10.5270/ESA-c5d3d65 .
- Copernicus AWS readme (processing, tile size, ocean tiles): https://copernicus-dem-30m.s3.amazonaws.com/readme.html .
- SRTM: https://www.usgs.gov/centers/eros/science/usgs-eros-archive-digital-elevation-shuttle-radar-topography-mission-srtm-1 returned 403 to `curl`. **No quote.** Re-read in a browser before relying on any SRTM claim.
- The ESA mission annex (spacedata.copernicus.eu) was not reachable earlier and was not retried. The CDSE page above is what the AWS registry's "Licence" link points to. Whether game heightmaps derived from GLO-30 count as "adapted or modified" data, and whether shipping them needs the second notice, is a **question for Liam**: the safe reading is to ship the "produced using" notice.

## Decision
**PROPOSED:** use Copernicus GLO-30 as the production source and GLO-90 for surveying candidate windows.

Reasoning in one sentence each:
- It needs no account, and the public bucket gives exact, scriptable URLs, so the pipeline is reproducible by any collaborator.
- GLO-30 matches SRTM 1″ resolution; the 90 m sibling keeps candidate surveys at about 5 MB per tile.
- Its attribution duty is a fixed notice we can place in credits, which is cheaper than an unresolved void problem; SRTM remains the fallback if Liam rejects the Copernicus licence terms (public domain, but needs a login).

**Open before acceptance:** (1) Liam to confirm the "produced using" credit notice is acceptable for the shipped game. (2) GLO-30 tiles were not downloaded in this task, so GLO-30 void behaviour is unconfirmed (none found in GLO-90). (3) SRTM facts are unverified because USGS blocked the fetch.

## Raw data policy
Raw tiles live in `data/raw/terrain/` and are **never committed** (`/data/raw/` is listed in `.gitignore`). Generated heightfields go in `data/generated/`.

## Download commands
Prerequisite: `mkdir -p data/raw/terrain`. The tile list for a window comes from `hillshade.py --list-tiles`, which computes the covering 1° tiles (formula: lat span = 150 km / R, lon span = 150 km / (R cos lat_mid), R = 6371.0088 km; see `window_bounds` in the script). The commands below were run (Git Bash, Windows, Python 3.14) and fetched all 21 unique tiles. On Windows use `python` for `python3`, and `--list-tiles` prints LF line endings on every OS, so the URLs pipe cleanly into `xargs`.

```
cd data/raw/terrain
# A — Negev / Arava
python ../../../tools/scripts/hillshade.py --list-tiles --sw-lat 30.0 --sw-lon 34.6 | tail -n +2 | xargs -n1 curl -fLO
# B — Hisma / Rum
python ../../../tools/scripts/hillshade.py --list-tiles --sw-lat 29.0 --sw-lon 35.0 | tail -n +2 | xargs -n1 curl -fLO
# C — Sinai
python ../../../tools/scripts/hillshade.py --list-tiles --sw-lat 28.3 --sw-lon 33.5 | tail -n +2 | xargs -n1 curl -fLO
# D — Harrat / Azraq
python ../../../tools/scripts/hillshade.py --list-tiles --sw-lat 31.8 --sw-lon 36.5 | tail -n +2 | xargs -n1 curl -fLO
```
`curl -O` names each file by its URL basename, which is the tile key. Tile counts and sizes: A 6, B 4, C 6, D 9 tiles (21 unique, because windows share tiles); at about 5.1 MB per GLO-90 tile, 20–46 MB per window and about 93 MB for all four. Confirmed by completed download.

Equivalent with the AWS CLI: `aws s3 cp --no-sign-request s3://copernicus-dem-90m/<key>/<key>.tif data/raw/terrain/`.

For production (GLO-30), substitute bucket `copernicus-dem-30m` and key prefix `Copernicus_DSM_COG_10_`.

## Consequences
- P0-06 and Phase 1 terrain tasks read `data/raw/terrain/` and must record the attribution notice in the credits.
- `docs/canon/TERRAIN_CANDIDATES.md` depends on this ADR for tile names.
- If rejected, replace the bucket URLs with EarthExplorer exports; `hillshade.py` reads any GeoTIFF.
