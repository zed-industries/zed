use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowRunWebhookEventPayload {
    pub action: WorkflowRunWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub workflow: Option<serde_json::Value>,
    pub workflow_run: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum WorkflowRunWebhookEventAction {
    Completed,
    InProgress,
    Requested,
}
