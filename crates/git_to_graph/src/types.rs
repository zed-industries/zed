//! Core type definitions and utility functions for git2graph.

use serde::{Deserialize, Serialize};

/// Point type constants to understand the graph structure.
/// Read the graph from top to bottom:
/// - Fork: when a node forks into two paths (top -> bottom)
/// - MergeBack: when a branch merges back into a branch on its right
/// - MergeTo: when a branch on the right merges into a branch on its left
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PointType {
    /// Pipe: |
    Pipe = 0,
    /// MergeBack: ┘
    MergeBack = 1,
    /// Fork: ┐
    Fork = 2,
    /// MergeTo: ┌
    MergeTo = 3,
}

impl From<u8> for PointType {
    fn from(val: u8) -> Self {
        match val {
            0 => PointType::Pipe,
            1 => PointType::MergeBack,
            2 => PointType::Fork,
            3 => PointType::MergeTo,
            _ => PointType::Pipe,
        }
    }
}

impl PointType {
    /// Check if this point type is MergeTo.
    pub fn is_merge_to(self) -> bool {
        self == PointType::MergeTo
    }

    /// Check if this point type is Fork.
    pub fn is_fork(self) -> bool {
        self == PointType::Fork
    }
}

/// Git log ordering options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Order {
    /// Default git ordering
    Default,
    /// Date order
    Date,
    /// Topological order
    Topological,
}

// Line type constants for row-based rendering
#[allow(dead_code)]
pub const BOTTOM_HALF_LINE: i32 = 0;
#[allow(dead_code)]
pub const TOP_HALF_LINE: i32 = 1;
#[allow(dead_code)]
pub const FULL_LINE: i32 = 2;
#[allow(dead_code)]
pub const FORK_LINE: i32 = 3;
#[allow(dead_code)]
pub const MERGE_BACK_LINE: i32 = 4;

// Node property keys
pub const ID_KEY: &str = "id";
#[allow(dead_code)]
pub const AUTHOR_NAME_KEY: &str = "name";
#[allow(dead_code)]
pub const AUTHOR_EMAIL_KEY: &str = "email";
#[allow(dead_code)]
pub const TIMESTAMP_KEY: &str = "timestamp";
#[allow(dead_code)]
pub const DATE_ISO_KEY: &str = "date";
pub const PARENTS_KEY: &str = "parents";
#[allow(dead_code)]
pub const DECORATE_KEY: &str = "decorate";
#[allow(dead_code)]
pub const SUBJECT_KEY: &str = "subject";
pub const G_KEY: &str = "g";
pub const PARENTS_PATHS_TEST_KEY: &str = "parentsPaths";

/// Create a boxed value (helper for creating owned references).
#[allow(dead_code)]
pub fn ptr<T>(value: T) -> Box<T> {
    Box::new(value)
}

/// Ternary conditional helper.
pub fn ternary<T>(predicate: bool, a: T, b: T) -> T {
    if predicate { a } else { b }
}

/// Rotate negative indices to positive (Python-style indexing).
pub fn rotate_idx(idx: i32, length: usize) -> usize {
    let mut result = idx;
    if result < 0 {
        result += length as i32;
        if result < 0 {
            panic!("Index out of bounds: idx={}, length={}", idx, length);
        }
    }
    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_type_methods() {
        assert!(PointType::MergeTo.is_merge_to());
        assert!(!PointType::Fork.is_merge_to());
        assert!(PointType::Fork.is_fork());
        assert!(!PointType::MergeTo.is_fork());
    }

    #[test]
    fn test_rotate_idx() {
        assert_eq!(rotate_idx(0, 5), 0);
        assert_eq!(rotate_idx(2, 5), 2);
        assert_eq!(rotate_idx(-1, 5), 4);
        assert_eq!(rotate_idx(-2, 5), 3);
    }

    #[test]
    #[should_panic]
    fn test_rotate_idx_panic() {
        rotate_idx(-10, 5);
    }
}
