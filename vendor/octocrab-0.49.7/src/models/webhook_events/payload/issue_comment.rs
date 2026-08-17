use serde::{Deserialize, Serialize};

use crate::models::issues::{Comment, Issue};

use super::OldValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueCommentWebhookEventPayload {
    pub action: IssueCommentWebhookEventAction,
    pub changes: Option<IssueCommentWebhookEventChanges>,
    pub comment: Comment,
    pub enterprise: Option<serde_json::Value>,
    pub issue: Issue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum IssueCommentWebhookEventAction {
    Created,
    Deleted,
    Edited,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueCommentWebhookEventChanges {
    pub body: OldValue<String>,
}
