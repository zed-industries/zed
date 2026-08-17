use serde::{Deserialize, Serialize};

use crate::models::pulls::{Comment, PullRequest};

use super::OldValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestReviewCommentWebhookEventPayload {
    pub action: PullRequestReviewCommentWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub comment: Comment,
    pub changes: Option<PullRequestReviewCommentWebhookEventChanges>,
    pub pull_request: PullRequest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestReviewCommentWebhookEventAction {
    Created,
    Deleted,
    Edited,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestReviewCommentWebhookEventChanges {
    pub body: Option<OldValue<String>>,
}
