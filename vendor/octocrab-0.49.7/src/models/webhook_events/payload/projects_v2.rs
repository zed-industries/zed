use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectsV2WebhookEventPayload {
    pub action: ProjectsV2WebhookEventAction,
    pub projects_v2: serde_json::Value,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ProjectsV2WebhookEventAction {
    Closed,
    Created,
    Deleted,
    Edited,
    Reopened,
}
