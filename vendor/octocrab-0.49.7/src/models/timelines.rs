use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TimelineEvent {
    /// Identifies the actual type of event that occurred.
    pub event: Event,
    /// The unique identifier of the event.
    pub id: Option<TimelineEventId>,
    /// The Global Node ID of the event.
    pub node_id: Option<String>,
    /// The REST API URL for fetching the event.
    pub url: Option<Url>,
    /// The person who generated the event.
    pub actor: Option<Author>,
    /// The SHA of the commit that referenced this issue.
    pub commit_id: Option<String>,
    /// The GitHub REST API link to the commit that referenced this issue.
    pub commit_url: Option<String>,
    /// The timestamp indicating when the event occurred.
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_card: Option<ProjectCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<ProjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_association: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree: Option<repos::CommitObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<repos::Verification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<repos::Commit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer: Option<CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>, // differs from other milestones the API returns. Has only a title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>, // differs from other labels the API returns. Has only a name and a color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_column_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<Rename>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<pulls::ReviewState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_review: Option<DismissedReview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_reviewer: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_requester: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Author>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DismissedReview {
    pub state: pulls::ReviewState,
    pub review_id: ReviewId,
    pub dismissal_message: Option<String>,
    pub dismissal_commit_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Source {
    pub issue: issues::Issue,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Rename {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Label {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Milestone {
    pub title: String,
}

/// The author of a commit, identified by its name and email.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub date: Option<chrono::DateTime<chrono::Utc>>,
}
