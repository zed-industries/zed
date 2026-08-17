use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentProtectionRuleWebhookEventPayload {
    pub action: DeploymentProtectionRuleWebhookEventAction,
    pub environment: Option<String>,
    pub event: Option<String>,
    pub deployment_callback_url: Option<String>,
    pub deployment: Option<serde_json::Value>,
    pub pull_requests: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeploymentProtectionRuleWebhookEventAction {
    Requested,
}
