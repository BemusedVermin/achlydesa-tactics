# ADR-0100 — Terrain source

**Status:** Proposed
**Date:** 2026-10-01 · **Conflict:** none (new decision; task P0-05)

## Context
- The theater is built from real elevation data of an arid, Levant-like region, then remixed: "rotate, mirror, stitch". The plan names SRTM and Copernicus GLO-30 and says to check attribution terms (Execution Plan §4.5).
- The theater is about 1,000–1,500 km across at literal scale (High-Level Design §4.1; stale until P0-02, used here only because nothing above covers it). Source resolution needs only to seed the terrain; detail below the source resolution is procedural (Execution Plan §4.5).
- The game never uses real place names (task P0-05 spec). Attribution therefore belongs in credits and the data manifest, not in the world.

## Verification status of this ADR
Retrieval date for every provider page below: **2026-10-01**, fetched with the agent's WebFetch tool, which returns a model-condensed rendering of the page, not raw HTML. Strings in quotation marks are as returned by that tool; **re-read each provider page before Liam accepts this ADR**. The agent sandbox could not run `curl` against the buckets, so **no tile was downloaded, no file header inspected, and no render produced**. S3 listing results (key names and object sizes) were read through WebFetch of the bucket's public list endpoint.

## Options compared

| | SRTM 1-arc-second (USGS) | Copernicus GLO-30 | Copernicus GLO-90 (survey only) |
|---|---|---|---|
| Resolution | "1 arc-second (30 meters)" for global coverage | 30 m (Copernicus Dataspace collection page, 2026-10-01) | 90 m (same page) |
| Format / tiling | DTED, BIL, or GeoTIFF; 1°×1° files, ~25 MB GeoTIFF | GeoTIFF (COG), 1°×1° tiles; key `Copernicus_DSM_COG_10_<N/S><lat>_00_<E/W><lon>_00_DEM` | Same layout, key prefix `Copernicus_DSM_COG_30_`; observed size of tile N30 E034: 5,143,778 bytes |
| Voids | "worldwide coverage of void filled data", yet "Some tiles may still contain voids" (USGS) | **Not verified.** Page fetched did not state it. Tiles ship auxiliary editing masks (`AUXFILES/…_EDM.tif`, `…_FLM.tif`, `…_ACM.kml`, seen in the bucket listing); their meaning was not read | Same |
| Download, no account | **No.** USGS says "EarthExplorer can be used to search, preview, and download" (EarthExplorer normally needs a free login; not verified here) | **Yes.** Public S3 bucket `copernicus-dem-30m`, region eu-central-1, anonymous HTTPS and `--no-sign-request` | Public S3 bucket `copernicus-dem-90m` |
| URL pattern | none stable without login | `https://copernicus-dem-30m.s3.eu-central-1.amazonaws.com/<key>/<key>.tif` (listing seen for N30 E034) | `https://copernicus-dem-90m.s3.eu-central-1.amazonaws.com/<key>/<key>.tif` (object existence for N30 E034 confirmed by listing) |
| Licence | Public domain; cite DOI 10.5066/F7PR7TFT | Free for the general public; mandatory credit notices (below) | Same |
| Surface type | Radar surface model | Surface model (DSM), EGM2008 vertical datum | Same |

Sources:
- SRTM: https://www.usgs.gov/centers/eros/science/usgs-eros-archive-digital-elevation-shuttle-radar-topography-mission-srtm-1 (retrieved 2026-10-01). Quoted: "The dataset is in the public domain" was the tool's wording; the page text itself must be re-read.
- Copernicus on AWS: https://registry.opendata.aws/copernicus-dem/ (retrieved 2026-10-01). As returned: "GLO-30 Public and GLO-90 are available on a free basis for the general public under the terms and conditions of the Licence". Citation form given: "Copernicus Digital Elevation Model (DEM) was accessed on `DATE` from https://registry.opendata.aws/copernicus-dem."
- Copernicus licence and credit: https://dataspace.copernicus.eu/explore-data/data-collections/copernicus-contributing-missions/collections-description/COP-DEM (retrieved 2026-10-01). Required notice for adapted data, as returned: "produced using Copernicus WorldDEM-30 © DLR e.V. 2010-2014 and © Airbus Defence and Space GmbH 2014-2018 provided under COPERNICUS by the European Union and ESA; all rights reserved". Unmodified-data notice and DOI https://doi.org/10.5270/ESA-c5d3d65 were also returned. The full licence text was **not** read; terms on redistribution of derived heightmaps shipped inside the game are **unverified**.
- A request for the ESA mission annex (spacedata.copernicus.eu) was refused by the network (ECONNREFUSED); the full legal licence was not obtained.

## Decision
**PROPOSED:** use Copernicus GLO-30 as the production source and GLO-90 for surveying candidate windows.

Reasoning in one sentence each:
- It needs no account, and the public bucket gives exact, scriptable URLs, so the pipeline is reproducible by any collaborator.
- GLO-30 matches SRTM 1″ resolution; the 90 m sibling keeps candidate surveys at about 5 MB per tile.
- Its attribution duty is a fixed notice we can place in credits, which is cheaper than an unresolved void problem; SRTM remains the fallback if Liam rejects the Copernicus licence terms (public domain, but needs a login).

**Open before acceptance:** read the full Copernicus licence for derivative-work and redistribution terms, and confirm void behavior from the tile metadata once a tile is downloaded.

## Raw data policy
Raw tiles live in `data/raw/terrain/` and are **never committed** (`/data/raw/` is listed in `.gitignore`). Generated heightfields go in `data/generated/`.

## Download commands
Prerequisite: `mkdir -p data/raw/terrain`. The tile list for a window comes from `hillshade.py --list-tiles`, which computes the covering 1° tiles (formula: lat span = 150 km / R, lon span = 150 km / (R cos lat_mid), R = 6371.0088 km; see `window_bounds` in the script).

```
cd data/raw/terrain
# A — Negev / Arava
python3 ../../../tools/scripts/hillshade.py --list-tiles --sw-lat 30.0 --sw-lon 34.6 | tail -n +2 | xargs -n1 curl -fLO
# B — Hisma / Rum
python3 ../../../tools/scripts/hillshade.py --list-tiles --sw-lat 29.0 --sw-lon 35.0 | tail -n +2 | xargs -n1 curl -fLO
# C — Sinai
python3 ../../../tools/scripts/hillshade.py --list-tiles --sw-lat 28.3 --sw-lon 33.5 | tail -n +2 | xargs -n1 curl -fLO
# D — Harrat / Azraq
python3 ../../../tools/scripts/hillshade.py --list-tiles --sw-lat 31.8 --sw-lon 36.5 | tail -n +2 | xargs -n1 curl -fLO
```
`curl -O` names each file by its URL basename, which is the tile key. Tile counts and sizes: A 6, B 4, C 6, D 9 tiles; at the observed 5.1 MB per GLO-90 tile, about 20–46 MB per window. The URL pattern was confirmed by bucket listing, **not by a completed download**.

Equivalent with the AWS CLI: `aws s3 cp --no-sign-request s3://copernicus-dem-90m/<key>/<key>.tif data/raw/terrain/`.

For production (GLO-30), substitute bucket `copernicus-dem-30m` and key prefix `Copernicus_DSM_COG_10_`.

## Consequences
- P0-06 and Phase 1 terrain tasks read `data/raw/terrain/` and must record the attribution notice in the credits.
- `docs/canon/TERRAIN_CANDIDATES.md` depends on this ADR for tile names.
- If rejected, replace the bucket URLs with EarthExplorer exports; `hillshade.py` reads any GeoTIFF.
