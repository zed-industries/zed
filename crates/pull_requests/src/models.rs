use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PullRequest {
    pub number: u32,
    pub id: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: PullRequestState,
    pub html_url: Url,
    pub user: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub merged_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub head: GitReference,
    pub base: GitReference,
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default)]
    pub assignees: Vec<User>,
    #[serde(default)]
    pub requested_reviewers: Vec<User>,
    #[serde(default)]
    pub draft: bool,
    pub mergeable: Option<bool>,
    pub mergeable_state: Option<String>,
    #[serde(default)]
    pub commits: u32,
    #[serde(default)]
    pub additions: u32,
    #[serde(default)]
    pub deletions: u32,
    #[serde(default)]
    pub changed_files: u32,
    #[serde(default)]
    pub comments: u32,
    #[serde(default)]
    pub review_comments: u32,
    #[serde(default)]
    pub reviews: Vec<PullRequestReview>,
    #[serde(default)]
    pub checks: CheckStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestState {
    Open,
    Closed,
    Merged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub avatar_url: Option<Url>,
    pub html_url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitReference {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
    pub user: Option<User>,
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub html_url: Url,
    pub clone_url: Option<Url>,
    pub ssh_url: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Label {
    pub id: u64,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PullRequestReview {
    pub id: u64,
    pub user: User,
    pub body: Option<String>,
    pub state: ReviewState,
    pub submitted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub comments: Vec<ReviewComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewState {
    Pending,
    Commented,
    Approved,
    ChangesRequested,
    Dismissed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewComment {
    pub id: u64,
    pub body: Option<String>,
    pub path: Option<String>,
    pub line: Option<u32>,
    pub start_line: Option<u32>,
    pub side: Option<CommentSide>,
    pub user: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub in_reply_to_id: Option<u64>,
    pub pull_request_review_id: Option<u64>,
    pub diff_hunk: Option<String>,
    pub original_line: Option<u32>,
    pub original_start_line: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CommentSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PullRequestComment {
    pub id: u64,
    pub body: Option<String>,
    pub user: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub reactions: ReactionSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReactionSummary {
    #[serde(default)]
    pub total_count: u32,
    #[serde(rename = "+1", default)]
    pub plus_one: u32,
    #[serde(rename = "-1", default)]
    pub minus_one: u32,
    #[serde(default)]
    pub laugh: u32,
    #[serde(default)]
    pub hooray: u32,
    #[serde(default)]
    pub confused: u32,
    #[serde(default)]
    pub heart: u32,
    #[serde(default)]
    pub rocket: u32,
    #[serde(default)]
    pub eyes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CheckStatus {
    #[serde(default)]
    pub state: CheckState,
    #[serde(default)]
    pub total_count: u32,
    #[serde(default)]
    pub statuses: Vec<Status>,
    #[serde(default)]
    pub check_runs: Vec<CheckRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum CheckState {
    #[default]
    Pending,
    Success,
    Failure,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Status {
    pub id: u64,
    pub state: CheckState,
    pub description: Option<String>,
    pub target_url: Option<Url>,
    pub context: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckRun {
    pub id: u64,
    pub name: String,
    pub status: CheckRunStatus,
    pub conclusion: Option<CheckRunConclusion>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub html_url: Url,
    pub details_url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CheckRunStatus {
    Queued,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunConclusion {
    Success,
    Failure,
    Neutral,
    Cancelled,
    Skipped,
    TimedOut,
    ActionRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullRequest {
    pub title: String,
    pub body: String,
    pub head: String,
    pub base: String,
    pub draft: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePullRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<PullRequestState>,
    pub base: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReview {
    pub body: String,
    pub event: ReviewEvent,
    pub comments: Vec<CreateReviewComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewEvent {
    Approve,
    RequestChanges,
    Comment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReviewComment {
    pub path: String,
    pub line: Option<u32>,
    pub side: Option<CommentSide>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestDiff {
    pub files: Vec<DiffFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffFile {
    pub filename: String,
    pub status: FileStatus,
    pub additions: u32,
    pub deletions: u32,
    pub changes: u32,
    pub patch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    Added,
    Removed,
    Modified,
    Renamed,
    Copied,
    Changed,
    Unchanged,
}
