//! Typed identifiers and a sequential allocator.
//! Implements Execution Plan §3.2 and Technical Design §22.1 (deterministic contract).

use crate::error::CoreError;
use serde::{Deserialize, Serialize};

/// Declares a typed `u64` identifier.
///
/// The type serializes transparently as its inner `u64` and displays as `<name>#<n>`.
#[macro_export]
macro_rules! define_id {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
            $crate::__serde::Serialize, $crate::__serde::Deserialize,
        )]
        #[serde(crate = "::ach_core::__serde", transparent)]
        pub struct $name(pub u64);

        impl ::core::convert::From<u64> for $name {
            fn from(n: u64) -> Self {
                Self(n)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}#{}", stringify!($name), self.0)
            }
        }
    };
}

/// Sequential identifier allocator; part of simulation state.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdAllocator {
    next: u64,
}

impl IdAllocator {
    /// Creates an allocator whose first identifier is `first`.
    pub fn new(first: u64) -> Self {
        Self { next: first }
    }

    /// Allocates the next identifier.
    pub fn alloc<T: From<u64>>(&mut self) -> T {
        self.try_alloc().expect("invariant: u64 id space exhausted")
    }

    /// Returns the current counter value as an id, then increments.
    ///
    /// `u64::MAX` is never issued: reaching it reports [`CoreError::IdExhausted`].
    pub fn try_alloc<T: From<u64>>(&mut self) -> Result<T, CoreError> {
        let id = self.next;
        self.next = id.checked_add(1).ok_or(CoreError::IdExhausted)?;
        Ok(T::from(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    define_id!(
        /// Test id.
        ThingId
    );

    #[test]
    fn allocates_sequentially() {
        let mut a = IdAllocator::new(5);
        assert_eq!(a.alloc::<ThingId>(), ThingId(5));
        assert_eq!(a.alloc::<ThingId>(), ThingId(6));
    }

    #[test]
    fn exhaustion_is_an_error() {
        let mut a = IdAllocator::new(u64::MAX);
        assert_eq!(a.try_alloc::<ThingId>(), Err(CoreError::IdExhausted));
    }

    #[test]
    fn display_format() {
        assert_eq!(ThingId(7).to_string(), "ThingId#7");
    }
}
