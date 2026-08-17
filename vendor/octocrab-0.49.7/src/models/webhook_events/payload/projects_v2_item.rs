use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectsV2ItemWebhookEventPayload {
    pub action: ProjectsV2ItemWebhookEventAction,
    pub changes: Option<serde_json::Value>,
    pub projects_v2_item: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ProjectsV2ItemWebhookEventAction {
    Archived,
    Converted,
    Created,
    Deleted,
    Edited,
    Reordered,
    Restored,
}
