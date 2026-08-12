//! GitHub pull request API and device-flow authentication.
//!
//! The GitHub App client ID is read from `ZED_GITHUB_APP_CLIENT_ID` at build time, with a runtime
//! environment-variable fallback for local development. The GitHub App must have Device Flow
//! enabled and request read access to Checks plus write access to Pull requests. Users must install
//! the app for private repositories before its user access token can access them.

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
