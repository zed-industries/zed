use serde::{Deserialize, Serialize};

use crate::models::pulls::{PullRequest, Review};

use super::OldValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestReviewWebhookEventPayload {
    pub action: PullRequestReviewWebhookEventAction,
    pub pull_request: PullRequest,
    pub review: Review,
    pub changes: Option<PullRequestReviewWebhookEventChanges>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestReviewWebhookEventAction {
    Dismissed,
    Edited,
    Submitted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestReviewWebhookEventChanges {
    pub body: Option<OldValue<String>>,
}
