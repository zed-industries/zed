use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PullRequestId {
    pub owner: Arc<str>,
    pub repository: Arc<str>,
    pub number: u32,
}

impl PullRequestId {
    pub fn new(owner: impl Into<Arc<str>>, repository: impl Into<Arc<str>>, number: u32) -> Self {
        Self {
            owner: owner.into(),
            repository: repository.into(),
            number,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub login: Arc<str>,
    pub avatar_url: Option<Arc<str>>,
    pub html_url: Option<Arc<str>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestState {
    Open,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewDecision {
    Approved,
    ChangesRequested,
    ReviewRequired,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mergeability {
    Mergeable,
    Conflicting,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PullRequestSummary {
    pub id: PullRequestId,
    pub title: Arc<str>,
    pub author: User,
    pub state: PullRequestState,
    pub is_draft: bool,
    pub head_ref: Arc<str>,
    pub head_sha: Arc<str>,
    pub base_ref: Arc<str>,
    pub updated_at: Arc<str>,
    pub html_url: Arc<str>,
    pub requested_reviewers: Vec<User>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PullRequestList {
    pub waiting_for_review: Vec<PullRequestSummary>,
    pub authored_by_viewer: Vec<PullRequestSummary>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PullRequestDetails {
    pub summary: PullRequestSummary,
    pub body: Arc<str>,
    pub additions: u32,
    pub deletions: u32,
    pub changed_files: u32,
    pub mergeability: Mergeability,
    pub review_decision: ReviewDecision,
    pub files: Vec<ChangedFile>,
    pub threads: Vec<ReviewThread>,
    pub checks: Vec<CheckSummary>,
    pub pending_review: Option<PullRequestReview>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangedFile {
    pub path: Arc<str>,
    pub previous_path: Option<Arc<str>>,
    pub additions: u32,
    pub deletions: u32,
    pub status: Arc<str>,
    pub patch: Option<Arc<str>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffSide {
    Left,
    Right,
}

impl DiffSide {
    pub fn as_github_str(self) -> &'static str {
        match self {
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAnchor {
    pub path: Arc<str>,
    pub commit_sha: Arc<str>,
    pub side: DiffSide,
    pub start_line: Option<u32>,
    pub line: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReviewId(pub Arc<str>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThreadId(pub Arc<str>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommentId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComment {
    pub id: CommentId,
    pub author: User,
    pub body: Arc<str>,
    pub created_at: Arc<str>,
    pub html_url: Arc<str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewThread {
    pub id: ThreadId,
    pub anchor: Option<ReviewAnchor>,
    pub is_resolved: bool,
    pub is_outdated: bool,
    pub viewer_can_reply: bool,
    pub viewer_can_resolve: bool,
    pub comments: Vec<ReviewComment>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewEvent {
    Comment,
    Approve,
    RequestChanges,
}

impl ReviewEvent {
    pub fn as_github_str(self) -> &'static str {
        match self {
            Self::Comment => "COMMENT",
            Self::Approve => "APPROVE",
            Self::RequestChanges => "REQUEST_CHANGES",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PullRequestReview {
    pub id: ReviewId,
    pub commit_sha: Arc<str>,
    pub author: User,
    pub body: Arc<str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckState {
    Queued,
    InProgress,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckConclusion {
    Success,
    Failure,
    Cancelled,
    Neutral,
    Skipped,
    TimedOut,
    ActionRequired,
    Stale,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckSummary {
    pub name: Arc<str>,
    pub state: CheckState,
    pub conclusion: Option<CheckConclusion>,
    pub details_url: Option<Arc<str>>,
}
