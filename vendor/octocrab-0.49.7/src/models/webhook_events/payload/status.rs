use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::StatusId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusWebhookEventPayload {
    pub avatar_url: Option<Url>,
    pub branches: Vec<serde_json::Value>,
    pub commit: serde_json::Value,
    pub context: String,
    pub created_at: String,
    pub description: Option<String>,
    pub enterprise: Option<serde_json::Value>,
    pub id: StatusId,
    pub name: String,
    pub sha: String,
    pub state: CommitState,
    pub target_url: Option<Url>,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CommitState {
    Pending,
    Success,
    Failure,
    Error,
}
