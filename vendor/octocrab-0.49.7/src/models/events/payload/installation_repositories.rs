//! This event occurs when there is activity relating to which repositories a
//! GitHub App installation can access. All GitHub Apps receive this event by
//! default. You cannot manually subscribe to this event.

use serde::{Deserialize, Serialize};

use super::InstallationEventRepository;
use crate::models::Author;

/// The payload in a webhook installation_repositories event type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationRepositoriesEventPayload {
    /// The action this event represents.
    pub action: InstallationRepositoriesEventAction,
    /// An enterprise on GitHub
    pub enterprise: Option<serde_json::Value>,
    /// An array of repositories, which were added to the installation
    pub repositories_added: Vec<InstallationEventRepository>,
    /// An array of repositories, which were removed from the installation
    pub repositories_removed: Vec<InstallationEventRepository>,
    /// Describe whether all repositories have been selected or there's a selection involved
    pub repository_selection: InstallationRepositoriesEventSelection,
    /// The initiator of the request, mainly for the [`created`](crate::models::events::payload::InstallationEventAction::Created) action
    pub requester: Option<Author>,
}

/// The action on an installation this event corresponds to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InstallationRepositoriesEventAction {
    /// A GitHub App installation was granted access to one or more repositories.
    Added,
    /// Access to one or more repositories was revoked for a GitHub App installation.
    Removed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InstallationRepositoriesEventSelection {
    All,
    Selected,
}
