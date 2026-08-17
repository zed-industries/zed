//!
//! Artifact types for GitHub workflow job outputs and inputs.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Represents artifacts produced by jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Artifacts {
    /// Artifacts to upload after the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Vec<Artifact>>,

    /// Artifacts to download before the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<Vec<Artifact>>,
}

/// Represents an artifact produced by a job.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Artifact {
    /// The name of the artifact.
    pub name: String,

    /// The path to the artifact.
    pub path: String,

    /// The number of days to retain the artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}
