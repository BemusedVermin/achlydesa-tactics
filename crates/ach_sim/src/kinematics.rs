//! Leg kinematics: per-segment lengths, speeds and march times, precomputed once.
//! Implements High-Level Design §4.1b (march tempo) and §5.2 (march postures), and
//! Technical Design §21.2 (advancement classes: a column advances analytically along its leg).

use crate::content::Content;
use crate::error::SimError;
use crate::scenario::{ColumnSpec, LegSpec};
use ach_core::WorldPos;
use ach_world::{MovementProfile, NavQuery, Terrain, nearest_vertex, route_path};
use serde::{Deserialize, Serialize};

const MM_PER_CM: i64 = 10;

/// What a leg is walked over: the inputs that fix every segment's speed.
#[derive(Clone, Copy, Debug)]
pub struct LegGround<'a> {
    /// Terrain whose heights give the slopes.
    pub terrain: &'a Terrain,
    /// Who is walking.
    pub profile: &'a MovementProfile,
    /// Road factor (route legs) or off-road factor (cross-country legs), per-mille.
    pub surface_factor_permille: i32,
    /// Posture speed multiplier, per-mille.
    pub posture_permille: i32,
}

/// Where a failed leg build went wrong; the caller adds column and leg context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegFault {
    /// The segment's slope is steeper than the profile allows, or its speed rounds to zero.
    Impassable(usize),
    /// The segment's endpoint has no height.
    OffTerrain(usize),
}

impl LegFault {
    /// Attaches the column name and leg index.
    pub fn into_error(self, column: &str, leg: usize) -> SimError {
        let column = column.to_owned();
        match self {
            Self::Impassable(segment) => SimError::ImpassableLeg {
                column,
                leg,
                segment,
            },
            Self::OffTerrain(segment) => SimError::OffTerrain {
                column,
                leg,
                segment,
            },
        }
    }
}

/// A polyline walked at fixed per-segment speeds. A leg with one point has no segments.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leg {
    /// Destination site id.
    pub to_site: String,
    points: Vec<WorldPos>,
    /// `cum_len_mm[i]` is the length of segments `0..i`; one entry more than segments.
    cum_len_mm: Vec<i64>,
    /// `cum_march_ms[i]` is the march time of segments `0..i`.
    cum_march_ms: Vec<i64>,
}

impl Leg {
    /// Precomputes the leg over `points`, dropping consecutive duplicates.
    ///
    /// Segment speed is `base_speed(slope) * surface_factor * posture / 10^6` mm/s and the
    /// segment's march time is `len_mm * 1000 / speed` ms, at least 1.
    pub fn build(
        to_site: &str,
        points: Vec<WorldPos>,
        ground: &LegGround<'_>,
    ) -> Result<Self, LegFault> {
        let mut points = points;
        points.dedup();
        let mut cum_len_mm = vec![0];
        let mut cum_march_ms = vec![0];
        for (i, pair) in points.windows(2).enumerate() {
            let len_mm = pair[0].horizontal_distance_cm(&pair[1]) * MM_PER_CM;
            let ms = segment_march_ms(ground, &pair[0], &pair[1], len_mm, i)?;
            cum_len_mm.push(cum_len_mm[i] + len_mm);
            cum_march_ms.push(cum_march_ms[i] + ms);
        }
        Ok(Self {
            to_site: to_site.to_owned(),
            points,
            cum_len_mm,
            cum_march_ms,
        })
    }

    /// Time to walk the whole leg, ms of marching.
    pub fn total_march_ms(&self) -> i64 {
        self.cum_march_ms.last().copied().unwrap_or(0)
    }

    /// Length of the whole leg, mm.
    pub fn total_len_mm(&self) -> i64 {
        self.cum_len_mm.last().copied().unwrap_or(0)
    }

    /// The exact end of the leg.
    pub fn end(&self) -> WorldPos {
        *self
            .points
            .last()
            .expect("invariant: a leg has at least one point")
    }

    /// The exact start of the leg.
    pub fn start(&self) -> WorldPos {
        self.points[0]
    }

    /// Index of the segment containing `t_ms` (`cum[i] <= t < cum[i + 1]`), if `t_ms` is
    /// strictly inside the leg.
    fn segment_at(&self, t_ms: i64) -> Option<usize> {
        if t_ms <= 0 || t_ms >= self.total_march_ms() {
            return None;
        }
        Some(self.cum_march_ms.partition_point(|&c| c <= t_ms) - 1)
    }

    /// Position after `t_ms` of marching, clamped to the leg.
    pub fn position_at_march_ms(&self, t_ms: i64) -> WorldPos {
        let Some(i) = self.segment_at(t_ms) else {
            return if t_ms <= 0 { self.start() } else { self.end() };
        };
        let span = self.cum_march_ms[i + 1] - self.cum_march_ms[i];
        let permille = (t_ms - self.cum_march_ms[i]) * 1000 / span;
        self.points[i].lerp_permille(&self.points[i + 1], permille)
    }

    /// Distance walked after `t_ms` of marching, mm, clamped to the leg.
    pub fn distance_at_march_ms(&self, t_ms: i64) -> i64 {
        let Some(i) = self.segment_at(t_ms) else {
            return if t_ms <= 0 { 0 } else { self.total_len_mm() };
        };
        let span = self.cum_march_ms[i + 1] - self.cum_march_ms[i];
        let len = self.cum_len_mm[i + 1] - self.cum_len_mm[i];
        self.cum_len_mm[i] + (t_ms - self.cum_march_ms[i]) * len / span
    }
}

/// Pathfinding budget for cross-country legs.
const CROSS_COUNTRY_MAX_EXPANSIONS: u32 = 4_000_000;

/// Precomputes every leg of a column's plan, failing fast on impassable ground.
///
/// `FollowRoute` legs run along the route between the vertices nearest the two sites, with
/// the exact site positions prepended and appended; `CrossCountry` legs use fastest-time
/// pathfinding. Each leg starts at the previous leg's destination.
pub fn plan_legs(
    spec: &ColumnSpec,
    terrain: &Terrain,
    content: &Content,
) -> Result<Vec<Leg>, SimError> {
    let unknown = |what, key: &str| SimError::Unknown {
        what,
        key: key.to_owned(),
    };
    if spec.legs.is_empty() {
        return Err(SimError::Invalid(format!(
            "column {:?} has no legs",
            spec.name
        )));
    }
    if spec.posture_permille <= 0 {
        return Err(SimError::Invalid(format!(
            "column {:?} has a non-positive posture",
            spec.name
        )));
    }
    let profile = content
        .profiles
        .profiles
        .get(&spec.profile)
        .ok_or_else(|| unknown("profile", &spec.profile))?;
    let mut from = content.site(&spec.start_site)?;
    let mut legs = Vec::with_capacity(spec.legs.len());
    for (index, leg) in spec.legs.iter().enumerate() {
        let to = content.site(leg.to_site())?;
        let (points, surface_factor_permille) = match leg {
            LegSpec::FollowRoute { route, .. } => {
                let route = content
                    .routes
                    .routes
                    .get(route)
                    .ok_or_else(|| unknown("route", route))?;
                let factor = profile
                    .road_factor_permille
                    .get(&route.condition)
                    .ok_or_else(|| unknown("road condition", &route.condition))?;
                let (a, b) = (
                    nearest_vertex(route, &from.pos),
                    nearest_vertex(route, &to.pos),
                );
                let mut points = route_path(route, a, b);
                if points.first() != Some(&from.pos) {
                    points.insert(0, from.pos);
                }
                if points.last() != Some(&to.pos) {
                    points.push(to.pos);
                }
                (points, *factor)
            }
            LegSpec::CrossCountry { .. } => {
                let found = NavQuery { terrain, profile }
                    .find_path(&from.pos, &to.pos, CROSS_COUNTRY_MAX_EXPANSIONS)
                    .map_err(|source| SimError::NoPath {
                        column: spec.name.clone(),
                        leg: index,
                        source,
                    })?;
                (found.points, profile.offroad_factor_permille)
            }
        };
        let ground = LegGround {
            terrain,
            profile,
            surface_factor_permille,
            posture_permille: spec.posture_permille,
        };
        legs.push(
            Leg::build(&to.id, points, &ground).map_err(|f| f.into_error(&spec.name, index))?,
        );
        from = to;
    }
    Ok(legs)
}

fn segment_march_ms(
    ground: &LegGround<'_>,
    a: &WorldPos,
    b: &WorldPos,
    len_mm: i64,
    segment: usize,
) -> Result<i64, LegFault> {
    let slope = ground
        .terrain
        .slope_permille(a, b)
        .ok_or(LegFault::OffTerrain(segment))?;
    let base = ground
        .profile
        .base_speed_mm_s(slope)
        .ok_or(LegFault::Impassable(segment))?;
    let speed = i64::from(base)
        * i64::from(ground.surface_factor_permille)
        * i64::from(ground.posture_permille)
        / 1_000_000;
    if speed <= 0 {
        return Err(LegFault::Impassable(segment));
    }
    Ok((len_mm * 1000 / speed).max(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ach_core::LevelId;
    use ach_world::{ChunkCoord, DetailParams, ProfileSet, SourceInfo, TerrainManifest};
    use std::path::Path;

    fn terrain(f: impl Fn(i64, i64) -> i32) -> Terrain {
        Terrain::from_fn(
            TerrainManifest {
                name: "t".into(),
                min_chunk: ChunkCoord { cx: 0, cy: 0 },
                max_chunk: ChunkCoord { cx: 0, cy: 0 },
                detail: DetailParams {
                    amplitude_cm: 0,
                    ..DetailParams::default()
                },
                source: SourceInfo {
                    dataset: "synthetic".into(),
                    tiles: vec![],
                    transform: "none".into(),
                    tool_version: "0".into(),
                },
            },
            f,
        )
    }

    fn foot() -> MovementProfile {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/movement/profiles.ron");
        ProfileSet::load(&path).unwrap().profiles["foot"].clone()
    }

    fn ground<'a>(t: &'a Terrain, p: &'a MovementProfile) -> LegGround<'a> {
        LegGround {
            terrain: t,
            profile: p,
            surface_factor_permille: 1000,
            posture_permille: 1000,
        }
    }

    #[test]
    fn flat_leg_times_match_speed() {
        let (t, p) = (terrain(|_, _| 0), foot());
        let pts = vec![WorldPos::surface_m(0, 0), WorldPos::surface_m(1_399, 0)];
        let leg = Leg::build("x", pts, &ground(&t, &p)).unwrap();
        // 1,399 m at 1,399 mm/s.
        assert_eq!(leg.total_len_mm(), 1_399_000);
        assert_eq!(leg.total_march_ms(), 1_000_000);
        let mid = WorldPos {
            x_cm: 69_950,
            y_cm: 0,
            level: LevelId::SURFACE,
        };
        assert_eq!(leg.position_at_march_ms(500_000), mid);
        assert_eq!(leg.distance_at_march_ms(500_000), 699_500);
        assert_eq!(leg.position_at_march_ms(-5), leg.start());
        assert_eq!(leg.position_at_march_ms(2_000_000), leg.end());
    }

    #[test]
    fn steep_ground_is_impassable_with_segment_index() {
        let (t, p) = (terrain(|x, _| i32::try_from(x).unwrap()), foot());
        let pts = vec![WorldPos::surface_m(0, 0), WorldPos::surface_m(100, 0)];
        let err = Leg::build("x", pts, &ground(&t, &p)).unwrap_err();
        assert_eq!(err, LegFault::Impassable(0));
    }

    #[test]
    fn duplicate_points_are_dropped_and_single_point_has_no_segments() {
        let (t, p) = (terrain(|_, _| 0), foot());
        let a = WorldPos::surface_m(10, 10);
        let leg = Leg::build("x", vec![a, a], &ground(&t, &p)).unwrap();
        assert_eq!((leg.total_march_ms(), leg.total_len_mm()), (0, 0));
        assert_eq!(leg.position_at_march_ms(5), a);
    }

    #[test]
    fn off_terrain_is_reported() {
        let (t, p) = (terrain(|_, _| 0), foot());
        let pts = vec![WorldPos::surface_m(0, 0), WorldPos::surface_m(-5, 0)];
        let err = Leg::build("x", pts, &ground(&t, &p)).unwrap_err();
        assert_eq!(err, LegFault::OffTerrain(0));
    }
}
