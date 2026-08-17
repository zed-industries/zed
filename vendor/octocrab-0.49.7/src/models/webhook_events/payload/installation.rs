use serde::{Deserialize, Serialize};

use crate::models::webhook_events::InstallationEventRepository;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationWebhookEventPayload {
    pub action: InstallationWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    #[serde(default)]
    pub repositories: Option<Vec<InstallationEventRepository>>,
    pub requester: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InstallationWebhookEventAction {
    Created,
    Deleted,
    NewPermissionsAccepted,
    Suspend,
    Unsuspend,
}
