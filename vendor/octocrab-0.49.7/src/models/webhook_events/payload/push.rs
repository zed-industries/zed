use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::repos::GitUserTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PushWebhookEventPayload {
    pub enterprise: Option<serde_json::Value>,
    pub after: String,
    pub base_ref: Option<String>,
    pub before: String,
    pub commits: Vec<PushWebhookEventCommit>,
    pub compare: Url,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub head_commit: Option<PushWebhookEventCommit>,
    pub pusher: GitUserTime,
    pub r#ref: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PushWebhookEventCommit {
    #[serde(default)]
    pub added: Vec<String>,
    pub author: GitUserTime,
    pub committer: GitUserTime,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    #[serde(default)]
    pub modified: Vec<String>,
    #[serde(default)]
    pub removed: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub tree_id: String,
    pub url: Url,
}
