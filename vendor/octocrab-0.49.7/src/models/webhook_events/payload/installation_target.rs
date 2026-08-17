use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationTargetWebhookEventPayload {
    pub account: serde_json::Value,
    pub action: String,
    pub changes: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
    pub target_type: String,
}
