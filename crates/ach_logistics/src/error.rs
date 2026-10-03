//! Error type for `ach_logistics`. Returned by public fallible functions per the project error rules.

use crate::stock::{Shortfall, StockKind};

/// Failures raised by stock operations and content loading.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LogisticsError {
    /// The content file could not be read.
    #[error("cannot read {path}: {message}")]
    Io {
        /// Path that failed to open.
        path: String,
        /// Underlying I/O error text.
        message: String,
    },
    /// The content file is not valid RON for the consumption format.
    #[error("cannot parse {path}: {message}")]
    Parse {
        /// Path that failed to parse.
        path: String,
        /// Parser error text.
        message: String,
    },
    /// A stock amount would exceed the `i64` milli-unit range.
    #[error("{0:?} stock overflow")]
    Overflow(StockKind),
    /// A quantity that must be non-negative was negative.
    #[error("negative quantity {0} milli-units")]
    NegativeQuantity(i64),
    /// A consumption rate in the content file is negative.
    #[error("negative consumption rate for {0}")]
    NegativeRate(String),
    /// The source did not hold enough stock.
    #[error(transparent)]
    Shortfall(#[from] Shortfall),
}
