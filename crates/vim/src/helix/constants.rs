//! Constants for Helix mode operations
//!
//! This module defines constants used throughout the Helix implementation
//! to improve code readability and maintainability.

/// Maximum iterations for boundary searches to prevent infinite loops
///
/// This limit ensures that boundary detection algorithms terminate even
/// in pathological cases (e.g., malformed buffers or edge cases in text analysis).
///
/// # Performance Note
/// This is a safety limit and should not be reached in normal operation.
/// If this limit is hit, it indicates a bug in the boundary detection logic.
pub const MAX_BOUNDARY_SEARCH_ITERATIONS: usize = 1000;

/// Number of characters to look ahead when detecting boundaries
///
/// Used in lookahead operations to determine if the next character
/// forms a boundary with the current character.
pub const BOUNDARY_LOOKAHEAD_CHARS: usize = 1;

/// Default number of times to repeat an operation when no count is given
///
/// When a user doesn't provide a repeat count (e.g., just `w` instead of `3w`),
/// this value is used as the default.
pub const DEFAULT_OPERATION_COUNT: usize = 1;

/// Maximum number of selections to maintain
///
/// This is a performance and memory limit to prevent excessive resource usage
/// when splitting selections (e.g., with regex split operations).
///
/// # User Impact
/// Users attempting to create more selections will receive a warning and
/// the operation will be capped at this limit.
pub const MAX_SELECTIONS: usize = 10_000;

/// Minimum number of selections for performance optimization path
///
/// When the number of selections is below this threshold, we use a simpler
/// algorithm that may allocate more freely. Above this threshold, we use
/// a more complex but memory-efficient algorithm.
pub const SELECTION_OPTIMIZATION_THRESHOLD: usize = 100;

/// Buffer size for clipboard operations
///
/// Size in bytes for the clipboard buffer when copying/pasting large selections.
pub const CLIPBOARD_BUFFER_SIZE: usize = 1024 * 1024; // 1 MB

/// Maximum distance for "nearby" operations
///
/// Used to determine if two positions are "close enough" for certain operations
/// like merging adjacent selections or detecting duplicate cursors.
pub const NEARBY_DISTANCE_THRESHOLD: usize = 3;

/// Default timeout for search operations in milliseconds
///
/// Regex searches that take longer than this will be cancelled to maintain
/// editor responsiveness.
pub const SEARCH_TIMEOUT_MS: u64 = 500;

/// Maximum length for undo/redo history per selection
///
/// Each selection maintains its own undo history. This limits the memory
/// used by the undo system.
pub const MAX_UNDO_HISTORY_LENGTH: usize = 1000;

/// Number of lines to scroll with page up/down
///
/// When using page navigation commands, this determines how many lines
/// to scroll. Set to 0 to use viewport height minus this value.
pub const PAGE_SCROLL_LINES: usize = 0;

/// Minimum width for aligned selections
///
/// When aligning selections, ensure they are at least this wide to
/// maintain readability.
pub const MIN_ALIGNED_SELECTION_WIDTH: usize = 1;

/// Maximum number of cached boundary positions
///
/// Caches boundary positions for performance. Larger values use more
/// memory but reduce computation.
pub const BOUNDARY_CACHE_SIZE: usize = 256;

/// Default incremental search delay in milliseconds
///
/// When typing in search mode, wait this long before performing the search
/// to allow for more input.
pub const INCREMENTAL_SEARCH_DELAY_MS: u64 = 100;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_are_sensible() {
        // Verify constants have reasonable values
        assert!(MAX_BOUNDARY_SEARCH_ITERATIONS > 0);
        assert!(DEFAULT_OPERATION_COUNT > 0);
        assert!(MAX_SELECTIONS > 0);
        assert!(MAX_SELECTIONS < usize::MAX / 2); // Prevent overflow
        assert!(SELECTION_OPTIMIZATION_THRESHOLD < MAX_SELECTIONS);
        assert!(NEARBY_DISTANCE_THRESHOLD > 0);
        assert!(SEARCH_TIMEOUT_MS > 0);
        assert!(CLIPBOARD_BUFFER_SIZE > 0);
    }

    #[test]
    fn test_selection_threshold_relationship() {
        // Optimization threshold should be much smaller than max
        assert!(SELECTION_OPTIMIZATION_THRESHOLD * 10 < MAX_SELECTIONS);
    }

    #[test]
    fn test_timeout_is_reasonable() {
        // Search timeout should be long enough for complex operations
        // but short enough to maintain responsiveness
        assert!(SEARCH_TIMEOUT_MS >= 100);
        assert!(SEARCH_TIMEOUT_MS <= 5000);
    }
}
