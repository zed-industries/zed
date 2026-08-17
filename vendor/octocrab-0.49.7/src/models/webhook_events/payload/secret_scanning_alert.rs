use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningAlertWebhookEventPayload {
    pub action: SecretScanningAlertWebhookEventAction,
    pub alert: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SecretScanningAlertWebhookEventAction {
    Created,
    Reopened,
    Resolved,
    Revoked,
}
