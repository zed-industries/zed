use serde::{Deserialize, Serialize};

use crate::models::webhook_events::InstallationEventRepository;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationRepositoriesWebhookEventPayload {
    pub action: InstallationRepositoriesWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub repositories_added: Vec<InstallationEventRepository>,
    pub repositories_removed: Vec<InstallationEventRepository>,
    pub repository_selection: InstallationRepositoriesWebhookEventSelection,
    pub requester: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InstallationRepositoriesWebhookEventAction {
    Added,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InstallationRepositoriesWebhookEventSelection {
    All,
    Selected,
}
