use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TeamWebhookEventPayload {
    pub action: TeamWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub team: serde_json::Value,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TeamWebhookEventAction {
    AddedToRepository,
    Created,
    Deleted,
    Edited,
    RemovedFromRepository,
}
