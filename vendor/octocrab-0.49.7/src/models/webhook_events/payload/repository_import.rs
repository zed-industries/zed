use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryImportWebhookEventPayload {
    pub enterprise: Option<serde_json::Value>,
    pub status: RepositoryImportWebhookEventStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RepositoryImportWebhookEventStatus {
    Success,
    Cancelled,
    Failure,
}
