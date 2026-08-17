use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectWebhookEventPayload {
    pub action: ProjectWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub project: serde_json::Value,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ProjectWebhookEventAction {
    Closed,
    Created,
    Deleted,
    Edited,
    Reopened,
}
