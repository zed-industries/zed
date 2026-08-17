use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectCardWebhookEventPayload {
    pub action: ProjectCardWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub project_card: serde_json::Value,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ProjectCardWebhookEventAction {
    Converted,
    Created,
    Deleted,
    Edited,
    Moved,
}
