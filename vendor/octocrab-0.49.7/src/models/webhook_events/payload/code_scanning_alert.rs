use serde::{Deserialize, Serialize};

use crate::models::{code_scannings::CodeScanningAlert, orgs::Organization, Author, Repository};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeScanningAlertWebhookEventPayload {
    /// The action that was performed.
    pub action: CodeScanningAlertWebhookEventAction,
    /// The code scanning alert that was affected.
    pub alert: CodeScanningAlert,
    /// The commit SHA of the code scanning alert. When the action is reopened_by_user or closed_by_user, the event was triggered by the sender and this value will be empty.
    pub commit_oid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    /// The Git reference of the code scanning alert. When the action is reopened_by_user or closed_by_user, the event was triggered by the sender and this value will be empty.
    pub r#ref: String,
    /// The repository that the code scanning alert belongs to.
    pub repository: Repository,
    /// The user that triggered the code scanning alert.
    pub sender: Author,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CodeScanningAlertWebhookEventAction {
    AppearedInBranch,
    ClosedByUser,
    Created,
    Fixed,
    Reopened,
    ReopenedByUser,
}
