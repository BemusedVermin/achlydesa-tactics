//! Stock sets with exact integer accounting.
//! Implements High-Level Design §11.3 (water tracked separately) and Simulation §16.1.

use crate::error::LogisticsError;
use ach_core::Milli;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A kind of stock. Categories are not interchangeable currencies (Simulation §16.1).
///
/// Base units: Water = liter, Food = ration (one person-day), Fuel = liter,
/// Ammunition = round, Parts = kit, Medicine = treatment. `Milli(1000)` is one base unit.
/// Water is separate from food per High-Level Design §11.3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StockKind {
    /// Drinking and working water, in liters.
    Water,
    /// Provisions, in person-day rations.
    Food,
    /// Fuel, in liters.
    Fuel,
    /// Ammunition, in rounds.
    Ammunition,
    /// Spare parts, in kits.
    Parts,
    /// Medicine, in treatments.
    Medicine,
}

/// A request that exceeded what a stock held.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("shortfall of {kind:?}: requested {requested:?}, available {available:?}")]
pub struct Shortfall {
    /// The stock that fell short.
    pub kind: StockKind,
    /// Amount asked for.
    pub requested: Milli,
    /// Amount actually held.
    pub available: Milli,
}

/// Quantities by kind. Amounts are never negative; zero amounts are not stored,
/// so equal contents compare equal.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockSet {
    amounts: BTreeMap<StockKind, Milli>,
}

impl StockSet {
    /// Amount held of `k` (zero if none).
    pub fn get(&self, k: StockKind) -> Milli {
        self.amounts.get(&k).copied().unwrap_or(Milli(0))
    }

    /// Adds `q`. Fails on a negative `q` or if the amount would overflow `i64`.
    pub fn add(&mut self, k: StockKind, q: Milli) -> Result<(), LogisticsError> {
        if q.0 < 0 {
            return Err(LogisticsError::NegativeQuantity(q.0));
        }
        let sum = self
            .get(k)
            .checked_add(q)
            .ok_or(LogisticsError::Overflow(k))?;
        self.set(k, sum);
        Ok(())
    }

    /// Removes exactly `q`, or fails leaving the set unchanged.
    ///
    /// A negative `q` can never be satisfied and is reported as a shortfall.
    pub fn remove(&mut self, k: StockKind, q: Milli) -> Result<(), Shortfall> {
        let available = self.get(k);
        if q.0 < 0 || q > available {
            return Err(Shortfall {
                kind: k,
                requested: q,
                available,
            });
        }
        self.set(k, Milli(available.0 - q.0));
        Ok(())
    }

    /// Removes up to `q` and returns the amount actually removed (zero for negative `q`).
    pub fn remove_up_to(&mut self, k: StockKind, q: Milli) -> Milli {
        let held = self.get(k).0;
        let taken = Milli(q.0.clamp(0, held));
        self.set(k, Milli(held - taken.0));
        taken
    }

    /// Moves `q` of `k` from `from` to `to`. On any error neither set changes.
    ///
    /// Fails with [`LogisticsError::Shortfall`] if `from` holds too little, or
    /// [`LogisticsError::Overflow`] if `to` could not hold the result.
    pub fn transfer(
        from: &mut StockSet,
        to: &mut StockSet,
        k: StockKind,
        q: Milli,
    ) -> Result<(), LogisticsError> {
        to.get(k)
            .checked_add(q)
            .ok_or(LogisticsError::Overflow(k))?;
        from.remove(k, q)?;
        to.add(k, q)
    }

    /// Iterates non-zero amounts in kind order.
    pub fn iter(&self) -> impl Iterator<Item = (StockKind, Milli)> {
        self.amounts.iter().map(|(k, q)| (*k, *q))
    }

    fn set(&mut self, k: StockKind, q: Milli) {
        if q.0 == 0 {
            self.amounts.remove(&k);
        } else {
            self.amounts.insert(k, q);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const KINDS: [StockKind; 6] = [
        StockKind::Water,
        StockKind::Food,
        StockKind::Fuel,
        StockKind::Ammunition,
        StockKind::Parts,
        StockKind::Medicine,
    ];

    #[test]
    fn add_overflow_is_error_and_leaves_set_unchanged() {
        let mut s = StockSet::default();
        s.add(StockKind::Water, Milli(i64::MAX)).unwrap();
        assert_eq!(
            s.add(StockKind::Water, Milli(1)),
            Err(LogisticsError::Overflow(StockKind::Water))
        );
        assert_eq!(s.get(StockKind::Water), Milli(i64::MAX));
    }

    #[test]
    fn remove_reports_shortfall() {
        let mut s = StockSet::default();
        s.add(StockKind::Food, Milli(500)).unwrap();
        let err = s.remove(StockKind::Food, Milli(501)).unwrap_err();
        assert_eq!(err.available, Milli(500));
        assert_eq!(err.requested, Milli(501));
        assert_eq!(s.get(StockKind::Food), Milli(500));
        assert_eq!(s.remove_up_to(StockKind::Food, Milli(900)), Milli(500));
        assert_eq!(s, StockSet::default());
    }

    #[test]
    fn transfer_overflow_changes_nothing() {
        let (mut a, mut b) = (StockSet::default(), StockSet::default());
        a.add(StockKind::Fuel, Milli(10)).unwrap();
        b.add(StockKind::Fuel, Milli(i64::MAX)).unwrap();
        assert_eq!(
            StockSet::transfer(&mut a, &mut b, StockKind::Fuel, Milli(10)),
            Err(LogisticsError::Overflow(StockKind::Fuel))
        );
        assert_eq!(a.get(StockKind::Fuel), Milli(10));
    }

    #[derive(Clone, Debug)]
    enum Op {
        Add(bool, usize, i64),
        Remove(bool, usize, i64),
        RemoveUpTo(bool, usize, i64),
        Transfer(bool, usize, i64),
    }

    fn op() -> impl Strategy<Value = Op> {
        (any::<bool>(), 0usize..6, -1_000i64..100_000, 0u8..4).prop_map(|(side, k, q, which)| {
            match which {
                0 => Op::Add(side, k, q),
                1 => Op::Remove(side, k, q),
                2 => Op::RemoveUpTo(side, k, q),
                _ => Op::Transfer(side, k, q),
            }
        })
    }

    proptest! {
        #[test]
        fn conservation(ops in proptest::collection::vec(op(), 0..60)) {
            let mut sets = [StockSet::default(), StockSet::default()];
            // Expected total per kind: what entered from outside minus what left.
            let mut flow = [0i128; 6];
            for op in ops {
                let [a, b] = &mut sets;
                match op {
                    Op::Add(s, k, q) => {
                        let set = if s { a } else { b };
                        if set.add(KINDS[k], Milli(q)).is_ok() { flow[k] += i128::from(q); }
                    }
                    Op::Remove(s, k, q) => {
                        let set = if s { a } else { b };
                        if set.remove(KINDS[k], Milli(q)).is_ok() { flow[k] -= i128::from(q); }
                    }
                    Op::RemoveUpTo(s, k, q) => {
                        let set = if s { a } else { b };
                        flow[k] -= i128::from(set.remove_up_to(KINDS[k], Milli(q)).0);
                    }
                    Op::Transfer(s, k, q) => {
                        let _ = if s {
                            StockSet::transfer(a, b, KINDS[k], Milli(q))
                        } else {
                            StockSet::transfer(b, a, KINDS[k], Milli(q))
                        };
                    }
                }
                for (i, k) in KINDS.iter().enumerate() {
                    let (x, y) = (sets[0].get(*k).0, sets[1].get(*k).0);
                    prop_assert_eq!(i128::from(x) + i128::from(y), flow[i]);
                    prop_assert!(x >= 0 && y >= 0);
                }
            }
        }
    }
}
