mod authentication;
mod client;
mod models;

pub use authentication::{
    DeviceAuthorization, DeviceFlowPoll, GitHubAuthentication, GitHubCredentials,
};
pub use client::{GitHubApiError, GitHubClient};
pub use models::{
    ChangedFile, CheckConclusion, CheckState, CheckSummary, CommentId, DiffSide, Mergeability,
    PullRequestDetails, PullRequestId, PullRequestList, PullRequestReview, PullRequestState,
    PullRequestSummary, ReviewAnchor, ReviewComment, ReviewDecision, ReviewEvent, ReviewId,
    ReviewThread, ThreadId, User,
};

pub const GITHUB_API_VERSION: &str = "2026-03-10";
pub const GITHUB_APP_CLIENT_ID: Option<&str> = option_env!("ZED_GITHUB_APP_CLIENT_ID");
