//! Error type for `ach_world`. Implements Execution Plan §3.2 (public fallible functions return `Result`).

use ach_core::CoreError;

/// Failures raised while loading, saving, or building terrain.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WorldError {
    /// A binary file header or frame was invalid.
    #[error("terrain file format: {0}")]
    Format(#[from] CoreError),
    /// A filesystem operation failed.
    #[error("io error on {path}: {message}")]
    Io {
        /// Path involved.
        path: String,
        /// Underlying error text.
        message: String,
    },
    /// `manifest.ron` could not be parsed or written.
    #[error("manifest error: {0}")]
    Manifest(String),
    /// The manifest chunk range is empty or inverted.
    #[error("manifest chunk range is empty")]
    EmptyRange,
    /// A chunk inside the manifest range has no file.
    #[error("missing chunk ({cx}, {cy})")]
    MissingChunk {
        /// Chunk column.
        cx: i32,
        /// Chunk row.
        cy: i32,
    },
    /// A chunk file disagrees with the manifest or has the wrong shape.
    #[error("chunk ({cx}, {cy}) is inconsistent: {reason}")]
    BadChunk {
        /// Chunk column.
        cx: i32,
        /// Chunk row.
        cy: i32,
        /// What disagreed.
        reason: &'static str,
    },
}
