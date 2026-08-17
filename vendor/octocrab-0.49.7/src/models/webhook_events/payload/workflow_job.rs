use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowJobWebhookEventPayload {
    pub action: WorkflowJobWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub workflow_job: serde_json::Value,
    pub deployment: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum WorkflowJobWebhookEventAction {
    Completed,
    InProgress,
    Queued,
    Waiting,
}
