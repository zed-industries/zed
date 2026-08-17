use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MemberWebhookEventPayload {
    pub action: MemberWebhookEventAction,
    pub changes: Option<serde_json::Value>,
    pub enterprise: Option<serde_json::Value>,
    pub member: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MemberWebhookEventAction {
    Added,
    Edited,
    Removed,
}
