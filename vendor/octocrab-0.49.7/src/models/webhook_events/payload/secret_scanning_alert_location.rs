use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningAlertLocationWebhookEventPayload {
    pub action: SecretScanningAlertLocationWebhookEventAction,
    pub alert: serde_json::Value,
    pub location: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SecretScanningAlertLocationWebhookEventAction {
    Created,
}
