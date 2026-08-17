use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectColumnWebhookEventPayload {
    pub action: ProjectColumnWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub project_column: serde_json::Value,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ProjectColumnWebhookEventAction {
    Created,
    Deleted,
    Edited,
    Moved,
}
