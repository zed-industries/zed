use crate::models::commits::Comment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitCommentWebhookEventPayload {
    pub action: CommitCommentWebhookEventAction,
    pub comment: Comment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CommitCommentWebhookEventAction {
    Created,
}
