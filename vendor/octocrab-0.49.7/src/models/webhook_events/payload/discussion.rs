use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DiscussionWebhookEventPayload {
    pub action: DiscussionWebhookEventAction,
    pub answer: Option<serde_json::Value>,
    pub discussion: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
    pub label: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiscussionWebhookEventAction {
    Answered,
    CategoryChanged,
    Closed,
    Created,
    Deleted,
    Edited,
    Labeled,
    Locked,
    Pinned,
    Reopened,
    Transferred,
    Unanswered,
    Unlabeled,
    Unlocked,
    Unpinned,
}
