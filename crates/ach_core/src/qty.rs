//! Integer quantities and ratios.
//! Implements Execution Plan §3.2 and Technical Design §22.1 (no floats).

use serde::{Deserialize, Serialize};

/// Thousandths of a stock's base unit (1 L water => `Milli(1000)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Milli(pub i64);

/// Ratio in thousandths. 1000 = 100 %.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PerMille(pub i32);

impl Milli {
    /// Sum, or `None` on overflow.
    pub fn checked_add(self, o: Milli) -> Option<Milli> {
        self.0.checked_add(o.0).map(Milli)
    }

    /// Difference, or `None` on overflow.
    pub fn checked_sub(self, o: Milli) -> Option<Milli> {
        self.0.checked_sub(o.0).map(Milli)
    }

    /// `floor(self * p / 1000)`, computed exactly in `i128`.
    ///
    /// Rounds toward negative infinity, so negative quantities round away from zero.
    /// Saturates at the `i64` range when `p` exceeds 100 % enough to overflow.
    pub fn scale(self, p: PerMille) -> Milli {
        let v = (i128::from(self.0) * i128::from(p.0)).div_euclid(1000);
        let clamped = v.clamp(i128::from(i64::MIN), i128::from(i64::MAX));
        Milli(i64::try_from(clamped).expect("invariant: value clamped into i64 range"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_scale() {
        assert_eq!(Milli(1000).scale(PerMille(333)), Milli(333));
    }

    #[test]
    fn scale_floors_toward_negative_infinity() {
        assert_eq!(Milli(1).scale(PerMille(500)), Milli(0));
        assert_eq!(Milli(-1).scale(PerMille(500)), Milli(-1));
    }

    #[test]
    fn scale_saturates() {
        assert_eq!(Milli(i64::MAX).scale(PerMille(i32::MAX)), Milli(i64::MAX));
        assert_eq!(Milli(i64::MIN).scale(PerMille(i32::MAX)), Milli(i64::MIN));
    }

    #[test]
    fn checked_ops() {
        assert_eq!(Milli(i64::MAX).checked_add(Milli(1)), None);
        assert_eq!(Milli(i64::MIN).checked_sub(Milli(1)), None);
        assert_eq!(Milli(5).checked_sub(Milli(7)), Some(Milli(-2)));
    }
}
