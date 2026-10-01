//! Fixed-point theater positions in integer centimeters.
//! Implements Execution Plan §3.2, Technical Design §22.1 (no floats) and
//! High-Level Design §4.1a (grid and layers).

use serde::{Deserialize, Serialize};

/// Centimeters per meter.
pub const CM_PER_M: i64 = 100;

/// Traversable layer; 0 = terrain surface (High-Level Design §4.1a).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LevelId(pub u16);

impl LevelId {
    /// The terrain surface.
    pub const SURFACE: Self = Self(0);
}

/// Theater-frame position. x east, y north, centimeters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WorldPos {
    /// Eastward offset, cm.
    pub x_cm: i64,
    /// Northward offset, cm.
    pub y_cm: i64,
    /// Layer the position is on.
    pub level: LevelId,
}

/// Elevation in centimeters above datum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ElevationCm(pub i32);

impl WorldPos {
    /// A surface position from whole meters.
    pub const fn surface_m(x_m: i64, y_m: i64) -> Self {
        Self {
            x_cm: x_m * CM_PER_M,
            y_cm: y_m * CM_PER_M,
            level: LevelId::SURFACE,
        }
    }

    /// Exact `floor(sqrt(dx² + dy²))` in cm, ignoring level.
    ///
    /// Computed in `u128` with an integer square root. Saturates at `i64::MAX`
    /// for separations beyond the representable range.
    pub fn horizontal_distance_cm(&self, other: &WorldPos) -> i64 {
        let dx = u128::from(self.x_cm.abs_diff(other.x_cm));
        let dy = u128::from(self.y_cm.abs_diff(other.y_cm));
        // Each square is < 2^128; their sum can overflow only for >2^64.5 cm separations.
        let sum = (dx * dx).saturating_add(dy * dy);
        i64::try_from(sum.isqrt()).unwrap_or(i64::MAX)
    }

    /// Point `t_permille / 1000` of the way from `self` to `other`.
    ///
    /// `t_permille` is clamped to `[0, 1000]`. Each axis is `a + floor((b - a) * t / 1000)`:
    /// fractions round toward negative infinity (so half-centimeters round down), whatever
    /// the direction of travel. The level is `self`'s, except at `t = 1000`, which is
    /// exactly `other`.
    pub fn lerp_permille(&self, other: &WorldPos, t_permille: i64) -> WorldPos {
        let t = i128::from(t_permille.clamp(0, 1000));
        if t == 1000 {
            return *other;
        }
        let axis = |a: i64, b: i64| -> i64 {
            let step = (i128::from(b) - i128::from(a)) * t;
            let v = i128::from(a) + step.div_euclid(1000);
            i64::try_from(v).expect("invariant: interpolant lies between two i64 endpoints")
        };
        WorldPos {
            x_cm: axis(self.x_cm, other.x_cm),
            y_cm: axis(self.y_cm, other.y_cm),
            level: self.level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn p(x: i64, y: i64) -> WorldPos {
        WorldPos {
            x_cm: x,
            y_cm: y,
            level: LevelId::SURFACE,
        }
    }

    #[test]
    fn known_distance() {
        let a = WorldPos::surface_m(0, 0);
        assert_eq!(
            a.horizontal_distance_cm(&WorldPos::surface_m(300, 400)),
            50_000
        );
    }

    #[test]
    fn distance_floors() {
        assert_eq!(p(0, 0).horizontal_distance_cm(&p(1, 1)), 1);
    }

    #[test]
    fn distance_saturates() {
        assert_eq!(
            p(i64::MIN, i64::MIN).horizontal_distance_cm(&p(i64::MAX, i64::MAX)),
            i64::MAX
        );
    }

    #[test]
    fn lerp_rounds_toward_negative_infinity() {
        // 1 cm * 500 / 1000 = 0.5 -> 0; -1 cm * 500 / 1000 = -0.5 -> -1.
        assert_eq!(p(0, 0).lerp_permille(&p(1, -1), 500), p(0, -1));
    }

    #[test]
    fn lerp_clamps_and_hits_endpoints() {
        let (a, b) = (p(-7, 3), p(11, 90));
        assert_eq!(a.lerp_permille(&b, -5), a);
        assert_eq!(a.lerp_permille(&b, 0), a);
        assert_eq!(a.lerp_permille(&b, 1000), b);
        assert_eq!(a.lerp_permille(&b, 5000), b);
    }

    const R: i64 = 200_000_000;

    proptest! {
        #[test]
        fn distance_symmetric_and_zero(ax in -R..=R, ay in -R..=R, bx in -R..=R, by in -R..=R) {
            let (a, b) = (p(ax, ay), p(bx, by));
            prop_assert_eq!(a.horizontal_distance_cm(&b), b.horizontal_distance_cm(&a));
            prop_assert_eq!(a.horizontal_distance_cm(&a), 0);
        }

        #[test]
        fn triangle_inequality_within_1cm(
            ax in -R..=R, ay in -R..=R, bx in -R..=R, by in -R..=R, cx in -R..=R, cy in -R..=R,
        ) {
            let (a, b, c) = (p(ax, ay), p(bx, by), p(cx, cy));
            prop_assert!(a.horizontal_distance_cm(&c)
                <= a.horizontal_distance_cm(&b) + b.horizontal_distance_cm(&c) + 1);
        }
    }
}
