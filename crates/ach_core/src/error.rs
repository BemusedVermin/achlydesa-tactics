//! Error type for `ach_core`. Implements Execution Plan §3.2 (public fallible functions return `Result`).

/// Failures raised by core primitives.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CoreError {
    /// The `u64` identifier space ran out.
    #[error("identifier space exhausted")]
    IdExhausted,
    /// Time arithmetic left the `i64` millisecond range.
    #[error("time overflow")]
    TimeOverflow,
    /// A string was not a valid `D<day> HH:MM[:SS]` time.
    #[error("invalid time {0:?}: expected \"D<day> HH:MM[:SS]\"")]
    InvalidTime(String),
}
