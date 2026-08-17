use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeployKeyWebhookEventPayload {
    pub action: DeployKeyWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub key: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeployKeyWebhookEventAction {
    Created,
    Deleted,
}
