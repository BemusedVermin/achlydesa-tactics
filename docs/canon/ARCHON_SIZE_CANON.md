Status: PROPOSED

# Archon size canon

Data: `content/archons/size_canon.ron`. Every derived number below is printed by `python3 tools/scripts/angular_size.py content/archons/size_canon.ron`; the tables in section 3 are that output pasted verbatim. Starting values come from Execution Plan §2.2 ("The table below holds proposed values for Phase 0 to ratify"). Anything not in that table is **PROPOSED** and marked.

## 1. Method

- Angular size: θ = 2·atan(s / 2d). s is the largest body dimension; d = √(ground² + altitude²), with altitude the cruise altitude for airborne bodies. Mneme uses its width, because its length fills the horizon.
- Geometric first sighting: d_h = 3.57·(√2 + √h_target) km, h in meters; h_target is cruise altitude plus body height (airborne) or body height (ground). Effective sighting = min(d_h, `haze_visibility_km`). PROPOSED: haze 120 km for all archons, tunable per archon.
- Chunk = 1.92 km (docs/tasks/P1-06.md). An airborne body's atlas footprint is its ground projection (shadow).
- Schema additions to the spec (PROPOSED): `length_range_m` (Mneme) and footprint given as `(length_m, width_m)`.

## 2. The archons

Distances to a squad are ground distances unless stated.

### Heliarch

- **Image:** a mirror-plated whale, belly up, swimming under the sun with oil hanging beneath it.
- **Numbers:** 2400 × 900 × 500 m; altitude 1000 / 3000 / 6000 m (min / cruise / max); speed 20,000 m/h typical, 60,000 max (PROPOSED speeds, from the spec example). Overhead at cruise 43.6°; at 20 km 6.8°. Effective sighting 120 km (haze-limited; geometric 216.3 km).
- **Canon:** "An upside-down whale plated in mirrors and the ruins of solar towers" (World Bible §The Heliarch of the Second Sky). Only the Heliarch behaves like weather (World Bible §Distribution across the desert). Execution Plan §2.2 table: 2.4 km, 1–6 km altitude.
- **Awe beat:** at 20 km (6.8°) it is a whale-shaped hole in the haze; the same body at 1000 m overhead covers 100.4° of sky.

### Autophagos

- **Image:** a mountain that walks, with a city on its back.
- **Numbers:** footprint 4000 × 3500 m (about 11 km²); shell summit 1800 m; speed 500 m/h typical, 2000 max (PROPOSED). Summit height 20.4° at 5 km, 10.3° at 10 km, 2.6° at 40 km. Effective sighting 120 km (haze; geometric 156.5 km).
- **Canon:** "A mountain-sized hermit crab carrying the experimental city as its shell" (World Bible §Autophagos, the Sovereign Ecology). Plan §2.2: summit 1.8 km, footprint ~4 km.
- **Awe beat:** at 10 km it is 22.6° wide and 10.3° tall, a range of hills that has an address.

### Anodyne

- **Image:** a hospital lamp the width of a town, hovering with its drip-lines hanging.
- **Numbers:** bell 1000 m across and 600 m tall (PROPOSED; plan has 900 m, see Deviations); altitude 800 / 1500 / 2500 m; speed 3000 m/h typical, 10,000 max (PROPOSED). At 3 km: 17.0°. Overhead at cruise 36.9°. Effective sighting 120 km (haze; geometric 168.6 km).
- **Canon:** "A vast luminous hospital lamp resembling a jellyfish" (World Bible §Anodyne, the Painless King). Plan §2.2: hovering at 1.5 km.
- **Awe beat:** at 3 km (17.0°) the warm light falls on the squad's own wounds first.

### Strategos

- **Image:** two wings of weapons holding a soft brain between them.
- **Numbers:** wingspan 6000 m, 800 m deep, wings 2500 m tall (PROPOSED depth and height); altitude 500 / 2000 / 4000 m; speed 15,000 m/h typical, 80,000 max (PROPOSED). At 15 km: wings 22.4°, brain 0.2°. Effective sighting 120 km (haze; geometric 244.5 km).
- **Canon:** "Two immense angelic wings constructed from drones, rifles, missiles, blades, and targeting arrays" (World Bible §Strategos, the Coward's War). Plan §2.2: wingspan 6 km, brain 40 m.
- **Awe beat:** at 15 km the wings span 22.4° of sky around a speck the eye cannot resolve.

### Pylaios

- **Image:** a black doorframe taller than any wall, standing in daylight that belongs elsewhere.
- **Numbers:** 300 m tall, 200 m wide, 60 m deep (width and depth PROPOSED); never moves, speed 0. At 2 km: 8.6°. Effective sighting 66.9 km (horizon-limited, below the haze).
- **Canon:** "rises higher than a city wall" and "never moves" (World Bible §Pylaios, the Gate That Arrives). Plan §2.2: gate 300 m high.
- **Awe beat:** at 2 km (8.6°) a road ahead folds flat and goes through it; at 66.9 km it first shows as a 0.3° tooth on the horizon that takes a day on foot to reach.

### Mneme

- **Image:** a level black road of ribbon that does not end, on which the dead look up.
- **Numbers:** width 200 m; length 200 to 900 km (PROPOSED range; Plan says "hundreds of km"), which is 104 to 469 chunks; thickness 3 m (PROPOSED); speed 200 m/h typical, 2000 max (PROPOSED reach-shifting rate). Width at 2 km 5.7°, at 5 km 2.3°. Effective sighting 11.2 km (horizon-limited, because it is flat).
- **Canon:** "A river of black magnetic ribbon flows across the desert without a bed" and "remaining perfectly level to itself" (World Bible §Mneme, the River That Remembers). Plan §2.2: 200 m wide.
- **Awe beat:** at its bank it runs horizon to horizon in both directions; at 11.2 km, the first sighting, it is a 1.0° black line across the dunes.

### Aletheia

- **Image:** a ring of white gloves the size of houses, all pointing inward at a cloth they never touch.
- **Numbers (all PROPOSED; the plan gives none):** ring 800 m across, hands 250 m tall, central cloth 90 m; speed 1000 m/h typical, 3000 max. At 5 km the ring is 9.1° and the cloth 1.0°. Effective sighting 61.5 km (horizon-limited).
- **Canon:** "A ring of enormous white-gloved hands surrounds a piece of black cloth shaped like a standing person" (World Bible §Aletheia, the Eye with a Sealed Chamber).
- **Awe beat:** at 5 km the hands (9.1°) are still pointing at the squad before the cloth (1.0°) can be told from a person.

## 3. Derived tables

Script output, verbatim.

<!-- BEGIN DERIVED -->
#### Table 1: angular size of the largest dimension (degrees)

Ground distance in km; airborne bodies use slant distance at cruise altitude. Mneme uses width.

| archon | s (m) | 2 km | 5 km | 10 km | 20 km | 40 km |
|---|---|---|---|---|---|---|
| heliarch | 2400 | 36.8 | 23.3 | 13.1 | 6.8 | 3.4 |
| autophagos | 4000 | 90.0 | 43.6 | 22.6 | 11.4 | 5.7 |
| anodyne | 1000 | 22.6 | 10.9 | 5.7 | 2.9 | 1.4 |
| strategos | 6000 | 93.4 | 58.2 | 32.8 | 17.0 | 8.6 |
| pylaios | 300 | 8.6 | 3.4 | 1.7 | 0.9 | 0.4 |
| mneme | 200 | 5.7 | 2.3 | 1.1 | 0.6 | 0.3 |
| aletheia | 800 | 22.6 | 9.1 | 4.6 | 2.3 | 1.1 |

#### Table 2: angular height (degrees)

Same distances; s = body height. For Autophagos this is the shell summit.

| archon | height (m) | 2 km | 5 km | 10 km | 20 km | 40 km |
|---|---|---|---|---|---|---|
| heliarch | 500 | 7.9 | 4.9 | 2.7 | 1.4 | 0.7 |
| autophagos | 1800 | 48.5 | 20.4 | 10.3 | 5.2 | 2.6 |
| anodyne | 600 | 13.7 | 6.6 | 3.4 | 1.7 | 0.9 |
| strategos | 2500 | 47.7 | 26.1 | 14.0 | 7.1 | 3.6 |
| pylaios | 300 | 8.6 | 3.4 | 1.7 | 0.9 | 0.4 |
| mneme | 3 | 0.1 | 0.0 | 0.0 | 0.0 | 0.0 |
| aletheia | 250 | 7.2 | 2.9 | 1.4 | 0.7 | 0.4 |

#### Table 3: directly overhead (airborne bodies, s = largest dimension)

| archon | s (m) | at min alt | at cruise alt | at max alt |
|---|---|---|---|---|
| heliarch | 2400 | 1000 m: 100.4 | 3000 m: 43.6 | 6000 m: 22.6 |
| anodyne | 1000 | 800 m: 64.0 | 1500 m: 36.9 | 2500 m: 22.6 |
| strategos | 6000 | 500 m: 161.1 | 2000 m: 112.6 | 4000 m: 73.7 |

#### Table 4: first-sighting distance (km)

d_h = 3.57*(sqrt(2) + sqrt(h_target)); effective = min(d_h, haze).

| archon | h_target (m) | d_h | haze | effective | limited by | angle there (deg) |
|---|---|---|---|---|---|---|
| heliarch | 3500 | 216.3 | 120 | 120.0 | haze | 1.1 |
| autophagos | 1800 | 156.5 | 120 | 120.0 | haze | 1.9 |
| anodyne | 2100 | 168.6 | 120 | 120.0 | haze | 0.5 |
| strategos | 4500 | 244.5 | 120 | 120.0 | haze | 2.9 |
| pylaios | 300 | 66.9 | 120 | 66.9 | horizon | 0.3 |
| mneme | 3 | 11.2 | 120 | 11.2 | horizon | 1.0 |
| aletheia | 250 | 61.5 | 120 | 61.5 | horizon | 0.7 |

#### Table 5: atlas footprint in chunks (1 chunk = 1.92 km)

| archon | footprint kind | length (chunks) | width (chunks) |
|---|---|---|---|
| heliarch | shadow projection | 1.25 | 0.47 |
| autophagos | ground contact | 2.08 | 1.82 |
| anodyne | shadow projection | 0.52 | 0.52 |
| strategos | shadow projection | 3.12 | 0.42 |
| pylaios | ground contact | 0.10 | 0.03 |
| mneme | ground contact | 104 to 469 | 0.10 |
| aletheia | ground contact | 0.42 | 0.42 |

#### Table 6: scale contrasts

| archon | small | large | ratio | ground dist | small (deg) | large (deg) |
|---|---|---|---|---|---|---|
| strategos | brain 40 m | wingspan 6000 m | 150:1 | 5 km | 0.4 | 58.2 |
| strategos | brain 40 m | wingspan 6000 m | 150:1 | 15 km | 0.2 | 22.4 |
| aletheia | black cloth 90 m | ring of hands 800 m | 8.9:1 | 5 km | 1.0 | 9.1 |
| aletheia | black cloth 90 m | ring of hands 800 m | 8.9:1 | 15 km | 0.3 | 3.1 |

#### Table 7: Execution Plan §2.2 reproduction

| archon | case | plan (deg) | computed (deg) | delta | within 1 deg |
|---|---|---|---|---|---|
| heliarch | overhead at 3 km | 44 | 43.6 | -0.4 | ok |
| heliarch | 20 km | 7 | 6.8 | -0.2 | ok |
| autophagos | 5 km, summit height | 20 | 20.4 | +0.4 | ok |
| autophagos | 10 km, summit height | 10 | 10.3 | +0.3 | ok |
| autophagos | 40 km, summit height | 2.6 | 2.6 | +0.0 | ok |
| anodyne | 3 km | 17 | 17.0 | +0.0 | ok |
| strategos | 15 km | 23 | 22.4 | -0.6 | ok |
| pylaios | 2 km | 8.5 | 8.6 | +0.1 | ok |
<!-- END DERIVED -->

## 4. Scale contrasts

- **Strategos:** a 40 m brain inside 6000 m of wings, a ratio of 150:1 (Table 6). The weapons are visible from 120 km; the thing they guard is not resolvable even at 5 km (0.4°). That gap is the coward's war: "The brain wants only to surrender" (World Bible §Strategos, the Coward's War). The interface may show wing angle and brain angle as two numbers, never the brain as a large icon (Field Atlas §6.2 asks for "restrained scale cues").
- **Aletheia:** the 90 m cloth inside an 800 m ring is 8.9:1; the center is small, but never the subject of the hands.
- **The Receiver:** the grammar inverts. After bodies of 300 to 6000 m, the answering presence is "a person-sized interface" (Campaign Bible §10), the smallest visible thing through which the largest reaches. PROPOSED: it gets no entry in the size data; its size is the absence of a number, a seated figure of about 1.7 m, and the horizon plate should show nothing else in frame at scale.

## 5. Deviations from Execution Plan §2.2

All plan angles are reproduced within 1° (Table 7). Interpretation note: the plan's Autophagos angles are for the 1.8 km summit height, not the 4 km footprint, so Table 2 carries them and Table 1 (largest dimension) gives larger values.

- Anodyne bell 900 m changed to 1000 m. The plan's 17° at 3 km only holds if 3 km is slant distance; with the bell hovering 1.5 km up and 3 km on the ground, 900 m gives 15.3° (1.7° off), and 1000 m gives 17.0°.
- Heliarch, Anodyne, Strategos altitude is measured as cruise altitude for slant distance; the plan's 20 km and 15 km cases move by 0.2° and 0.6°.

## 6. Open questions

World Bible scale cues not given a number here:

1. Heliarch "broken mechanical halo drags auroras behind it" (World Bible §The Heliarch of the Second Sky): halo and aurora trail sizes are unset.
2. Heliarch oil "falls from its blowholes in long black curtains": curtain length is assumed to reach the ground from 1000 to 6000 m altitude; not specified.
3. Anodyne tendrils "stretch into the bodies of distant worshippers" (World Bible §Anodyne, the Painless King): reach is unset; the 600 m body height excludes tendrils.
4. Mneme "pale vertebrae" below the surface (World Bible §Mneme, the River That Remembers): size and depth unset.
5. Autophagos "one of Achlydesa's largest functioning societies" (World Bible §Autophagos, the Sovereign Ecology) sits on about 11 km² of footprint, which may be small for it; a larger footprint would run it further off the sheet. Ratify or enlarge.
6. Pylaios "roads, settlements, and stretches of horizon periodically fold flat" (World Bible §Pylaios, the Gate That Arrives): the radius of the fold is unset.
7. Honored without question: "mountain-sized" (Autophagos, 1800 m), "higher than a city wall" (Pylaios, 300 m), "perfectly level to itself" (Mneme, 3 m thick), "never moves" (Pylaios, speed 0).
