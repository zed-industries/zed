//!
//! Expression types for GitHub workflow conditions and evaluations.

use serde::{Deserialize, Serialize};

/// Represents an expression used in conditions.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct Expression(pub String);

impl Expression {
    /// Creates a new `Expression` from a string.
    pub fn new<T: ToString>(expr: T) -> Self {
        Self(expr.to_string())
    }
}
