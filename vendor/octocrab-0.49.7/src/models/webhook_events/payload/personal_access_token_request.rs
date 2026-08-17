use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PersonalAccessTokenRequestWebhookEventPayload {
    pub action: PersonalAccessTokenRequestWebhookEventAction,
    pub personal_access_token_request: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PersonalAccessTokenRequestWebhookEventAction {
    Approved,
    Cancelled,
    Created,
    Denied,
}
