use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecurityAdvisoryWebhookEventPayload {
    pub action: SecurityAdvisoryWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub security_advisory: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SecurityAdvisoryWebhookEventAction {
    Published,
    Updated,
    Withdrawn,
}
