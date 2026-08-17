use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryAdvisoryWebhookEventPayload {
    pub action: RepositoryAdvisoryWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub repository: serde_json::Value,
    pub repository_advisory: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RepositoryAdvisoryWebhookEventAction {
    Published,
    Reported,
}
