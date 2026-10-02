//! Error type for `ach_people`. Returned by public fallible functions per the project error rules.

use ach_core::CoreError;

/// Failures raised while loading a roster.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PeopleError {
    /// The content file could not be read.
    #[error("cannot read {path}: {message}")]
    Io {
        /// Path that failed to open.
        path: String,
        /// Underlying I/O error text.
        message: String,
    },
    /// The content file is not valid RON for the roster format.
    #[error("cannot parse {path}: {message}")]
    Parse {
        /// Path that failed to parse.
        path: String,
        /// Parser error text.
        message: String,
    },
    /// Two entries share a key.
    #[error("duplicate character key {0:?}")]
    DuplicateKey(String),
    /// The id allocator ran out of identifiers.
    #[error(transparent)]
    Id(#[from] CoreError),
}
