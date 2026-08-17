//! Provides utilities for managing execution counts in Jupyter sessions.
//!
//! This module defines the `ExecutionCount` type, which represents a monotonically
//! increasing counter for tracking the number of code executions in a Jupyter session.
//! This count is not tied to individual cells but represents the overall execution history
//! of the session, including code run via `execute_request` in terminals.
//!
//! # Examples
//!
//! ```
//! use jupyter_protocol::ExecutionCount;
//!
//! // Start a new session
//! let mut count = ExecutionCount::new(1);
//! assert_eq!(count.value(), 1);
//!
//! // After executing some code
//! count.increment();
//! assert_eq!(count.value(), 2);
//!
//! // Creating from a known execution count
//! let count_from_usize: ExecutionCount = 3.into();
//! assert_eq!(count_from_usize.value(), 3);
//!
//! // Converting back to usize
//! let usize_from_count: usize = count.into();
//! assert_eq!(usize_from_count, 2);
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a monotonically increasing counter for tracking the number of code executions
/// in a Jupyter session. This count is maintained across all executions, including those in
/// notebook cells and via terminal `execute_request`s.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ExecutionCount(pub usize);

impl ExecutionCount {
    /// Creates a new `ExecutionCount` with the given count.
    ///
    /// # Arguments
    ///
    /// * `count` - The initial execution count value.
    pub fn new(count: usize) -> Self {
        Self(count)
    }
}

impl From<usize> for ExecutionCount {
    fn from(count: usize) -> Self {
        Self(count)
    }
}

impl From<ExecutionCount> for usize {
    fn from(count: ExecutionCount) -> Self {
        count.0
    }
}

impl From<ExecutionCount> for Value {
    fn from(count: ExecutionCount) -> Self {
        Value::Number(count.0.into())
    }
}

impl ExecutionCount {
    /// Increments the execution count by 1.
    ///
    /// Primarily for use by kernel authors.
    ///
    /// This should be called after each successful code execution in the session.
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// Returns the current value of the execution count.
    ///
    /// The current execution count for the session as a `usize`.
    pub fn value(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for ExecutionCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
