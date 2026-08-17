//!
//! Strategy types for GitHub workflow job execution.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the strategy for running jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Strategy {
    /// The matrix for job execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Value>,

    /// Whether to fail fast on errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,

    /// The maximum number of jobs to run in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<u32>,
}
