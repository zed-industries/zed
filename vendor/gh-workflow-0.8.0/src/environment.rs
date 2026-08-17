//!
//! Environment types for GitHub workflow deployment environments.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Represents an environment for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Environment {
    /// The name of the environment.
    pub name: String,

    /// The URL associated with the environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
