use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MilestoneWebhookEventPayload {
    pub action: MilestoneWebhookEventAction,
    pub milestone: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MilestoneWebhookEventAction {
    Closed,
    Created,
    Deleted,
    Edited,
    Opened,
}
