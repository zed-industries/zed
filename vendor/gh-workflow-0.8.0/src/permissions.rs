//! Permission types for GitHub Workflow tokens.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Represents permissions for the `GITHUB_TOKEN`.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Permissions {
    /// Permissions for actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Level>,

    /// Permissions for repository contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Level>,

    /// Permissions for issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Level>,

    /// Permissions for pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<Level>,

    /// Permissions for deployments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Level>,

    /// Permissions for checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Level>,

    /// Permissions for statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Level>,

    /// Permissions for packages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Level>,

    /// Permissions for pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Level>,

    /// Permissions for ID tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<Level>,
}

/// Represents the level of permissions.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Level {
    Read,
    Write,
    #[default]
    None,
}
