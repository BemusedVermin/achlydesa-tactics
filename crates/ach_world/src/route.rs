//! Routes: authored polylines that columns follow explicitly in Phase 1.
//! Implements Technical Design §11.2 (paths and formations) and reads the format of
//! `content/world/red_ledger_routes.ron`.

use crate::error::WorldError;
use ach_core::{CM_PER_M, LevelId, WorldPos};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

/// Densification step, cm. Slightly under the 3,000 cm segment limit so that flooring
/// each coordinate (error under 1.5 cm per segment) never pushes a segment past it.
const STEP_CM: i64 = 2_990;

/// An authored road, track, or approach.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Route {
    /// Stable identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// `road`, `track`, or `approach`.
    pub kind: String,
    /// `intact`, `worn`, or `broken`; keys of `MovementProfile::road_factor_permille`.
    pub condition: String,
    /// Vertices, whole meters `(x, y)`.
    pub points_m: Vec<(i64, i64)>,
}

/// All routes in a file, by id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteSet {
    /// Routes keyed by `Route::id`.
    pub routes: BTreeMap<String, Route>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawKind {
    Road,
    Track,
    Approach,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawCondition {
    Intact,
    Worn,
    Broken,
}

#[derive(Deserialize)]
struct RawRoute {
    id: String,
    name: String,
    kind: RawKind,
    condition: RawCondition,
    vertices: Vec<(i64, i64)>,
}

#[derive(Deserialize)]
struct RawFile {
    routes: Vec<RawRoute>,
}

impl From<RawRoute> for Route {
    fn from(raw: RawRoute) -> Self {
        let kind = match raw.kind {
            RawKind::Road => "road",
            RawKind::Track => "track",
            RawKind::Approach => "approach",
        };
        let condition = match raw.condition {
            RawCondition::Intact => "intact",
            RawCondition::Worn => "worn",
            RawCondition::Broken => "broken",
        };
        Self {
            id: raw.id,
            name: raw.name,
            kind: kind.to_owned(),
            condition: condition.to_owned(),
            points_m: raw.vertices,
        }
    }
}

impl RouteSet {
    /// Loads the `routes` list of a route file; other sections (watercourses, tours,
    /// notes) are ignored. Fails on a duplicate route id.
    pub fn load(path: &Path) -> Result<Self, WorldError> {
        let text = std::fs::read_to_string(path).map_err(|e| WorldError::Io {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;
        let raw: RawFile = ron::from_str(&text).map_err(|e| WorldError::Manifest(e.to_string()))?;
        let mut routes = BTreeMap::new();
        for route in raw.routes.into_iter().map(Route::from) {
            let id = route.id.clone();
            if routes.insert(id.clone(), route).is_some() {
                return Err(WorldError::Manifest(format!("duplicate route id {id}")));
            }
        }
        Ok(Self { routes })
    }
}

fn vertex_pos(&(x_m, y_m): &(i64, i64)) -> WorldPos {
    WorldPos::surface_m(x_m, y_m)
}

/// Nearest vertex index to a position, by horizontal distance (ties go to the lowest
/// index). An empty route gives 0.
pub fn nearest_vertex(route: &Route, pos: &WorldPos) -> usize {
    let dist2 = |p: &(i64, i64)| {
        let dx = i128::from(p.0 * CM_PER_M - pos.x_cm);
        let dy = i128::from(p.1 * CM_PER_M - pos.y_cm);
        dx * dx + dy * dy
    };
    route
        .points_m
        .iter()
        .enumerate()
        .min_by_key(|&(i, p)| (dist2(p), i))
        .map_or(0, |(i, _)| i)
}

/// Sub-route between two vertex indices (either direction), densified so no segment
/// exceeds 3,000 cm. The first and last points are the vertices themselves; a reversed
/// query returns exactly the reversed point list. Indices past the end clamp to the last
/// vertex; an empty route gives an empty list.
pub fn route_path(route: &Route, from_idx: usize, to_idx: usize) -> Vec<WorldPos> {
    let Some(last) = route.points_m.len().checked_sub(1) else {
        return Vec::new();
    };
    let lo = from_idx.min(to_idx).min(last);
    let hi = from_idx.max(to_idx).min(last);
    let mut points = vec![vertex_pos(&route.points_m[lo])];
    for pair in route.points_m[lo..=hi].windows(2) {
        densify_into(&mut points, &vertex_pos(&pair[0]), &vertex_pos(&pair[1]));
    }
    if from_idx > to_idx {
        points.reverse();
    }
    points
}

/// Appends the points after `a` up to and including `b`, spaced at most 3,000 cm apart.
fn densify_into(points: &mut Vec<WorldPos>, a: &WorldPos, b: &WorldPos) {
    let n = ((a.horizontal_distance_cm(b) + STEP_CM - 1) / STEP_CM).max(1);
    let at = |from: i64, to: i64, i: i64| from + ((to - from) * i).div_euclid(n);
    for i in 1..n {
        points.push(WorldPos {
            x_cm: at(a.x_cm, b.x_cm, i),
            y_cm: at(a.y_cm, b.y_cm, i),
            level: LevelId::SURFACE,
        });
    }
    points.push(*b);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The segment limit from the spec, cm.
    const MAX_SEGMENT_CM: i64 = 3_000;

    fn route(points_m: Vec<(i64, i64)>) -> Route {
        Route {
            id: "t".into(),
            name: "T".into(),
            kind: "road".into(),
            condition: "worn".into(),
            points_m,
        }
    }

    fn max_segment(points: &[WorldPos]) -> i64 {
        points
            .windows(2)
            .map(|w| w[0].horizontal_distance_cm(&w[1]))
            .max()
            .unwrap_or(0)
    }

    #[test]
    fn densified_segments_are_short_and_endpoints_exact() {
        let r = route(vec![(0, 0), (500, 120), (503, 700), (-40, 701)]);
        let p = route_path(&r, 0, 3);
        assert!(max_segment(&p) <= MAX_SEGMENT_CM, "{}", max_segment(&p));
        assert_eq!(p.first(), Some(&WorldPos::surface_m(0, 0)));
        assert_eq!(p.last(), Some(&WorldPos::surface_m(-40, 701)));
        assert!(p.contains(&WorldPos::surface_m(500, 120)));
    }

    #[test]
    fn reversal_reverses_the_point_list() {
        let r = route(vec![(0, 0), (500, 120), (503, 700)]);
        let mut fwd = route_path(&r, 0, 2);
        fwd.reverse();
        assert_eq!(route_path(&r, 2, 0), fwd);
    }

    #[test]
    fn sub_route_and_degenerate_cases() {
        let r = route(vec![(0, 0), (100, 0), (200, 0)]);
        let p = route_path(&r, 1, 2);
        assert_eq!(p.first(), Some(&WorldPos::surface_m(100, 0)));
        assert_eq!(p.last(), Some(&WorldPos::surface_m(200, 0)));
        assert_eq!(route_path(&r, 1, 1), vec![WorldPos::surface_m(100, 0)]);
        assert!(route_path(&route(vec![]), 0, 0).is_empty());
    }

    #[test]
    fn nearest_vertex_ties_go_low() {
        let r = route(vec![(0, 0), (100, 0), (200, 0)]);
        assert_eq!(nearest_vertex(&r, &WorldPos::surface_m(50, 0)), 0);
        assert_eq!(nearest_vertex(&r, &WorldPos::surface_m(140, 30)), 1);
        assert_eq!(nearest_vertex(&r, &WorldPos::surface_m(900, 0)), 2);
    }

    #[test]
    fn red_ledger_routes_load_and_densify() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/world/red_ledger_routes.ron");
        let set = RouteSet::load(&path).unwrap();
        assert_eq!(set.routes.len(), 7);
        assert_eq!(set.routes["dry_meridian"].condition, "intact");
        assert_eq!(set.routes["depot_track"].kind, "track");
        for r in set.routes.values() {
            let p = route_path(r, 0, r.points_m.len() - 1);
            assert!(max_segment(&p) <= MAX_SEGMENT_CM, "{}", r.id);
        }
    }
}
