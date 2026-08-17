//!
//! Secret types for GitHub workflow secrets and security.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Represents a secret required for the workflow.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Secret {
    /// Indicates if the secret is required.
    pub required: bool,

    /// A description of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
