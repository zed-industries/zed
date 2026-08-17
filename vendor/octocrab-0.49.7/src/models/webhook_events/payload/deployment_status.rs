use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentStatusWebhookEventPayload {
    pub action: DeploymentStatusWebhookEventAction,
    pub check_run: Option<serde_json::Value>,
    pub deployment: serde_json::Value,
    pub deployment_status: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
    pub workflow: Option<serde_json::Value>,
    pub workflow_run: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeploymentStatusWebhookEventAction {
    Created,
}
