//! Grid pathfinding: fastest-time A* over the 30 m base samples for a movement profile.
//! Implements Technical Design §11.2 (paths and formations, first two paragraphs) and
//! High-Level Design §5.2 (continuous terrain, leaders adapt exact local paths).
//!
//! Cells are the base height samples, so detail noise never affects routing. Search is
//! 8-connected with integer costs in milliseconds; ties break by cell index, so the same
//! query always returns the same path.

use crate::chunk::BASE_SPACING_CM;
use crate::profile::MovementProfile;
use crate::terrain::Terrain;
use ach_core::{LevelId, WorldPos};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Orthogonal step length, cm.
const ORTHO_CM: i64 = BASE_SPACING_CM;
/// Diagonal step length, cm (30 m × √2, rounded down).
const DIAG_CM: i64 = 4_242;
/// Cell index meaning "no parent".
const NO_PARENT: u32 = u32::MAX;

/// A pathfinding query against one terrain for one movement profile.
#[derive(Clone, Copy, Debug)]
pub struct NavQuery<'a> {
    /// Ground to cross.
    pub terrain: &'a Terrain,
    /// Who is crossing it.
    pub profile: &'a MovementProfile,
}

/// A found path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    /// Waypoints: `from`, the sample points walked, then `to`.
    pub points: Vec<WorldPos>,
    /// Travel time between the snapped start and goal samples, ms. The short legs from
    /// the exact endpoints to their samples are not timed.
    pub total_ms: i64,
}

/// Why a path was not found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PathError {
    /// An endpoint is off the surface or outside the terrain.
    #[error("endpoint is outside the terrain")]
    OutOfBounds,
    /// No passable chain of cells joins the endpoints.
    #[error("no passable path between the endpoints")]
    NoPath,
    /// The expansion budget ran out before the goal was reached.
    #[error("search expansion limit reached")]
    SearchLimit,
}

/// Row-major sample lattice covering a terrain.
struct Grid {
    min_x: i64,
    min_y: i64,
    cols: i64,
    rows: i64,
}

impl Grid {
    fn new(terrain: &Terrain) -> Self {
        let ((min_x, min_y), (max_x, max_y)) = terrain.bounds_cm();
        Self {
            min_x,
            min_y,
            cols: (max_x - min_x) / BASE_SPACING_CM + 1,
            rows: (max_y - min_y) / BASE_SPACING_CM + 1,
        }
    }

    fn len(&self) -> usize {
        usize::try_from(self.cols * self.rows).unwrap_or(usize::MAX)
    }

    /// Nearest sample to a surface position, `None` if it lies outside the terrain.
    fn snap(&self, pos: &WorldPos) -> Option<(i64, i64)> {
        if pos.level != LevelId::SURFACE {
            return None;
        }
        let near = |v: i64, min: i64, n: i64| {
            let c = (v - min + BASE_SPACING_CM / 2).div_euclid(BASE_SPACING_CM);
            (v >= min && c < n + 1 && v <= min + (n - 1) * BASE_SPACING_CM).then_some(c.min(n - 1))
        };
        Some((
            near(pos.x_cm, self.min_x, self.cols)?,
            near(pos.y_cm, self.min_y, self.rows)?,
        ))
    }

    fn index(&self, (col, row): (i64, i64)) -> u32 {
        u32::try_from(row * self.cols + col).unwrap_or(NO_PARENT - 1)
    }

    fn cell(&self, index: u32) -> (i64, i64) {
        let i = i64::from(index);
        (i % self.cols, i / self.cols)
    }

    fn pos(&self, (col, row): (i64, i64)) -> WorldPos {
        WorldPos {
            x_cm: self.min_x + col * BASE_SPACING_CM,
            y_cm: self.min_y + row * BASE_SPACING_CM,
            level: LevelId::SURFACE,
        }
    }

    fn contains(&self, (col, row): (i64, i64)) -> bool {
        (0..self.cols).contains(&col) && (0..self.rows).contains(&row)
    }
}

/// The eight neighbor offsets, with their step lengths in cm.
const STEPS: [(i64, i64, i64); 8] = [
    (-1, -1, DIAG_CM),
    (0, -1, ORTHO_CM),
    (1, -1, DIAG_CM),
    (-1, 0, ORTHO_CM),
    (1, 0, ORTHO_CM),
    (-1, 1, DIAG_CM),
    (0, 1, ORTHO_CM),
    (1, 1, DIAG_CM),
];

impl NavQuery<'_> {
    /// Fastest path from `from` to `to`.
    ///
    /// Both endpoints snap to the nearest base sample; the returned path starts at `from`
    /// and ends at `to` exactly. Fails with [`PathError::SearchLimit`] after
    /// `max_expansions` cells have been expanded.
    pub fn find_path(
        &self,
        from: &WorldPos,
        to: &WorldPos,
        max_expansions: u32,
    ) -> Result<Path, PathError> {
        let grid = Grid::new(self.terrain);
        let start = grid.snap(from).ok_or(PathError::OutOfBounds)?;
        let goal = grid.snap(to).ok_or(PathError::OutOfBounds)?;
        let (cells, total_ms) = self.search(&grid, start, goal, max_expansions)?;
        let mut points = vec![*from];
        points.extend(cells.into_iter().map(|c| grid.pos(c)));
        points.push(*to);
        Ok(Path { points, total_ms })
    }

    /// Ground speed on a step with the given slope, mm/s; `None` if impassable.
    fn step_speed_mm_s(&self, slope_permille: i64) -> Option<i64> {
        let slope = i32::try_from(slope_permille).ok()?;
        let base = i64::from(self.profile.base_speed_mm_s(slope)?);
        let speed = base * i64::from(self.profile.offroad_factor_permille) / 1000;
        (speed > 0).then_some(speed)
    }

    /// Time to step between two samples, ms; `None` if impassable or off the terrain.
    fn edge_ms(&self, grid: &Grid, a: (i64, i64), b: (i64, i64), d_cm: i64) -> Option<i64> {
        let height = |c| {
            let p = grid.pos(c);
            self.terrain.base_height_cm(p.x_cm, p.y_cm).map(i64::from)
        };
        let slope = (height(b)? - height(a)?) * 1000 / d_cm;
        let speed = self.step_speed_mm_s(slope)?;
        Some(d_cm * 10 * 1000 / speed)
    }

    /// Octile distance to the goal at the best possible speed, ms. Admissible up to
    /// integer rounding of each edge cost.
    fn heuristic_ms(&self, a: (i64, i64), goal: (i64, i64), best_speed_mm_s: i64) -> i64 {
        if best_speed_mm_s <= 0 {
            return 0;
        }
        let (dx, dy) = ((a.0 - goal.0).abs(), (a.1 - goal.1).abs());
        let (short, long) = (dx.min(dy), dx.max(dy));
        let cm = (long - short) * ORTHO_CM + short * DIAG_CM;
        cm * 10 * 1000 / best_speed_mm_s
    }

    /// A* over the grid. Returns the cells from `start` to `goal` inclusive and the cost.
    fn search(
        &self,
        grid: &Grid,
        start: (i64, i64),
        goal: (i64, i64),
        max_expansions: u32,
    ) -> Result<(Vec<(i64, i64)>, i64), PathError> {
        let best = i64::from(self.profile.max_speed_mm_s())
            * i64::from(self.profile.offroad_factor_permille)
            / 1000;
        let (start_i, goal_i) = (grid.index(start), grid.index(goal));
        let mut g = vec![i64::MAX; grid.len()];
        let mut parent = vec![NO_PARENT; grid.len()];
        let mut closed = vec![false; grid.len()];
        let mut open = BinaryHeap::new();
        let h0 = self.heuristic_ms(start, goal, best);
        g[start_i as usize] = 0;
        open.push(Reverse((h0, h0, start_i)));
        let mut expansions = 0u32;
        while let Some(Reverse((_, _, i))) = open.pop() {
            let slot = i as usize;
            if closed[slot] {
                continue;
            }
            closed[slot] = true;
            if i == goal_i {
                return Ok((unwind(grid, &parent, goal_i), g[slot]));
            }
            if expansions >= max_expansions {
                return Err(PathError::SearchLimit);
            }
            expansions += 1;
            let here = grid.cell(i);
            for (dx, dy, d_cm) in STEPS {
                let next = (here.0 + dx, here.1 + dy);
                if !grid.contains(next) {
                    continue;
                }
                let n = grid.index(next);
                if closed[n as usize] {
                    continue;
                }
                let Some(ms) = self.edge_ms(grid, here, next, d_cm) else {
                    continue;
                };
                let cost = g[slot] + ms;
                if cost < g[n as usize] {
                    g[n as usize] = cost;
                    parent[n as usize] = i;
                    let h = self.heuristic_ms(next, goal, best);
                    open.push(Reverse((cost + h, h, n)));
                }
            }
        }
        Err(PathError::NoPath)
    }
}

/// Follows parent links back from the goal and returns the cells start-first.
fn unwind(grid: &Grid, parent: &[u32], goal: u32) -> Vec<(i64, i64)> {
    let mut cells = vec![grid.cell(goal)];
    let mut at = goal;
    while parent[at as usize] != NO_PARENT {
        at = parent[at as usize];
        cells.push(grid.cell(at));
    }
    cells.reverse();
    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::ChunkCoord;
    use crate::detail::DetailParams;
    use crate::manifest::{SourceInfo, TerrainManifest};
    use crate::profile::tests::foot;

    fn terrain(max: (i32, i32), f: impl Fn(i64, i64) -> i32) -> Terrain {
        Terrain::from_fn(
            TerrainManifest {
                name: "test".into(),
                min_chunk: ChunkCoord { cx: 0, cy: 0 },
                max_chunk: ChunkCoord {
                    cx: max.0,
                    cy: max.1,
                },
                detail: DetailParams::default(),
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

    fn at(x: i64, y: i64) -> WorldPos {
        WorldPos {
            x_cm: x,
            y_cm: y,
            level: LevelId::SURFACE,
        }
    }

    #[test]
    fn flat_plane_cost_matches_octile_time() {
        let t = terrain((1, 1), |_, _| 0);
        let p = foot();
        let q = NavQuery {
            terrain: &t,
            profile: &p,
        };
        // 20 columns east and 10 rows north: 10 diagonal plus 10 orthogonal steps.
        let path = q
            .find_path(&at(30_000, 30_000), &at(90_000, 60_000), 100_000)
            .unwrap();
        let speed = 1399 * 900 / 1000;
        let analytic = (10 * DIAG_CM + 10 * ORTHO_CM) * 10 * 1000 / speed;
        assert!((path.total_ms - analytic).abs() <= 20, "{}", path.total_ms);
        assert_eq!(path.points.len(), 23);
        assert_eq!(path.points[0], at(30_000, 30_000));
        assert_eq!(path.points[22], at(90_000, 60_000));
    }

    #[test]
    fn endpoints_are_exact_and_samples_snap() {
        let t = terrain((0, 0), |_, _| 0);
        let p = foot();
        let q = NavQuery {
            terrain: &t,
            profile: &p,
        };
        let (from, to) = (at(31_000, 29_000), at(61_400, 28_600));
        let path = q.find_path(&from, &to, 1_000).unwrap();
        assert_eq!(path.points.first(), Some(&from));
        assert_eq!(path.points.last(), Some(&to));
        assert_eq!(path.points[1], at(30_000, 30_000));
        assert_eq!(path.points[path.points.len() - 2], at(60_000, 30_000));
    }

    #[test]
    fn path_threads_the_gap_in_a_wall() {
        // A 40 m thick wall, too steep to climb even diagonally, with one 100 m gap.
        let wall =
            |x: i64, y: i64| (180_000..=186_000).contains(&x) && !(96_000..=99_000).contains(&y);
        let t = terrain((1, 1), move |x, y| if wall(x, y) { 4_000 } else { 0 });
        let p = foot();
        let q = NavQuery {
            terrain: &t,
            profile: &p,
        };
        let path = q
            .find_path(&at(30_000, 30_000), &at(350_000, 30_000), 1_000_000)
            .unwrap();
        let crossing: Vec<_> = path
            .points
            .iter()
            .filter(|p| (180_000..=186_000).contains(&p.x_cm))
            .collect();
        assert!(!crossing.is_empty());
        assert!(crossing.iter().all(|p| (96_000..=99_000).contains(&p.y_cm)));
    }

    #[test]
    fn sealed_wall_has_no_path_and_limit_is_reported() {
        let t = terrain((1, 1), |x, _| {
            if (180_000..=186_000).contains(&x) {
                4_000
            } else {
                0
            }
        });
        let p = foot();
        let q = NavQuery {
            terrain: &t,
            profile: &p,
        };
        let (a, b) = (at(30_000, 30_000), at(350_000, 30_000));
        assert_eq!(q.find_path(&a, &b, 1_000_000), Err(PathError::NoPath));
        assert_eq!(q.find_path(&a, &b, 10), Err(PathError::SearchLimit));
    }

    #[test]
    fn out_of_bounds_endpoints_are_rejected() {
        let t = terrain((0, 0), |_, _| 0);
        let p = foot();
        let q = NavQuery {
            terrain: &t,
            profile: &p,
        };
        let inside = at(30_000, 30_000);
        assert_eq!(
            q.find_path(&inside, &at(-1_600, 0), 10),
            Err(PathError::OutOfBounds)
        );
        assert_eq!(
            q.find_path(&at(0, 192_001), &inside, 10),
            Err(PathError::OutOfBounds)
        );
    }

    #[test]
    fn uphill_costs_more_than_downhill() {
        // A 50 permille ramp rising east.
        let t = terrain((1, 0), |x, _| (x / 20) as i32);
        let p = foot();
        let q = NavQuery {
            terrain: &t,
            profile: &p,
        };
        let (a, b) = (at(30_000, 30_000), at(120_000, 30_000));
        let up = q.find_path(&a, &b, 100_000).unwrap().total_ms;
        let down = q.find_path(&b, &a, 100_000).unwrap().total_ms;
        assert!(up > down, "up {up} down {down}");
    }

    #[test]
    fn same_query_gives_identical_path() {
        let t = terrain((1, 1), |x, y| ((x * 7 + y * 13) % 900) as i32);
        let p = foot();
        let q = NavQuery {
            terrain: &t,
            profile: &p,
        };
        let (a, b) = (at(10_000, 20_000), at(300_000, 250_000));
        assert_eq!(
            q.find_path(&a, &b, 1_000_000),
            q.find_path(&a, &b, 1_000_000)
        );
    }

    /// Timing check for the spec (not a CI test): a 50 km query over 96 × 24 km of rough
    /// ground. Run with `cargo test -p ach_world --release -- --ignored --nocapture` under `time`.
    #[test]
    #[ignore = "timing measurement, run manually in release mode"]
    fn fifty_km_query_over_rough_terrain() {
        let rough = |x: i64, y: i64| {
            let n = ach_core::mix64(((x / 3000) as u64) << 32 ^ (y / 3000) as u64);
            (n % 400) as i32 + (x / 100 % 800) as i32
        };
        let t = terrain((49, 12), rough);
        let p = foot();
        let q = NavQuery {
            terrain: &t,
            profile: &p,
        };
        let path = q
            .find_path(&at(500_000, 1_200_000), &at(5_500_000, 1_200_000), u32::MAX)
            .unwrap();
        println!("{} points, {} ms", path.points.len(), path.total_ms);
    }
}
