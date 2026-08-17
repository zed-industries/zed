use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentWebhookEventPayload {
    pub action: DeploymentWebhookEventAction,
    pub deployment: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
    pub workflow: serde_json::Value,
    pub workflow_run: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeploymentWebhookEventAction {
    Created,
}
