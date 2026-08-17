use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReleaseWebhookEventPayload {
    pub action: ReleaseWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub release: serde_json::Value,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReleaseWebhookEventAction {
    Created,
    Deleted,
    Edited,
    Prereleased,
    Published,
    Released,
    Unpublished,
}
