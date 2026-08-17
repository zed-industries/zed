//! This event occurs when there is activity relating to the user or
//! organization account that a GitHub App is installed on.

use serde::{Deserialize, Serialize};

use crate::models::orgs::Organization;

/// The payload in a webhook installation_target event type.
///
/// Somebody renamed the user or organization account that a GitHub App is installed on.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationTargetEventPayload {
    pub account: Organization,
    pub changes: InstallationTargetChanges,
    pub target_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationTargetChanges {
    pub login: InstallationTargetLoginChanges,
    pub slug: InstallationTargetSlugChanges,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationTargetLoginChanges {
    pub from: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationTargetSlugChanges {
    pub from: String,
}
