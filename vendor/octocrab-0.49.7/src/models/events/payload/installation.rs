//! This event occurs when there is activity relating to a GitHub App
//! installation. All GitHub Apps receive this event by default. You cannot
//! manually subscribe to this event.

use serde::{Deserialize, Serialize};

use super::InstallationEventRepository;
use crate::models::Author;

/// The payload in a webhook installation event type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationEventPayload {
    /// The action this event represents.
    pub action: InstallationEventAction,
    /// An enterprise on GitHub
    pub enterprise: Option<serde_json::Value>,
    /// An array of repositories that the installation can access
    pub repositories: Vec<InstallationEventRepository>,
    /// The initiator of the request, mainly for the [`created`](crate::models::events::payload::InstallationEventAction::Created) action
    pub requester: Option<Author>,
}

/// The action on an installation this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InstallationEventAction {
    /// Someone installed a GitHub App on a user or organization account.
    Created,
    /// Someone uninstalled a GitHub App on a user or organization account.
    Deleted,
    /// Someone granted new permissions to a GitHub App.
    NewPermissionsAccepted,
    /// Someone blocked access by a GitHub App to their user or organization account.
    Suspend,
    /// A GitHub App that was blocked from accessing a user or organization account was given access the account again.
    Unsuspend,
}
