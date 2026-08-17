use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PublicWebhookEventPayload {
    pub enterprise: Option<serde_json::Value>,
}
