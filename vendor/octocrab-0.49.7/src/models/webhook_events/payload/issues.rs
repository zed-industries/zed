use serde::{Deserialize, Serialize};

use crate::models::{issues::Issue, Author, Label, Milestone};

use super::OldValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssuesWebhookEventPayload {
    pub action: IssuesWebhookEventAction,
    pub assignee: Option<Author>,
    pub enterprise: Option<serde_json::Value>,
    pub issue: Issue,
    pub milestone: Option<Milestone>,
    pub label: Option<Label>,
    pub changes: Option<IssuesWebhookEventChanges>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum IssuesWebhookEventAction {
    Assigned,
    Closed,
    Deleted,
    Demilestoned,
    Edited,
    Labeled,
    Locked,
    Milestoned,
    Opened,
    Pinned,
    Reopened,
    Transferred,
    Unassigned,
    Unlabeled,
    Unlocked,
    Unpinned,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssuesWebhookEventChanges {
    pub body: Option<OldValue<String>>,
    pub title: Option<OldValue<String>>,
    pub assignee: Option<OldValue<Author>>,
}
