use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryDispatchWebhookEventPayload {
    pub action: String,
    pub branch: String,
    pub client_payload: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
}
