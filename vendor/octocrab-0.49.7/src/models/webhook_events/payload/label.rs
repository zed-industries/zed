use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LabelWebhookEventPayload {
    pub action: LabelWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub label: serde_json::Value,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum LabelWebhookEventAction {
    Created,
    Deleted,
    Edited,
}
