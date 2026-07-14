use crate::{
    ChangedFile, CheckConclusion, CheckState, CheckSummary, CommentId, DiffSide,
    GITHUB_API_VERSION, Mergeability, PullRequestDetails, PullRequestId, PullRequestList,
    PullRequestReview, PullRequestState, PullRequestSummary, ReviewAnchor, ReviewComment,
    ReviewDecision, ReviewEvent, ReviewId, ReviewThread, ThreadId, User,
};
use anyhow::{Context as _, Result};
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClient, Method, Request, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{fmt, sync::Arc};

const API_BASE_URL: &str = "https://api.github.com";

#[derive(Debug)]
pub struct GitHubApiError {
    pub status: StatusCode,
    pub message: String,
    pub retry_after_seconds: Option<u64>,
}

impl fmt::Display for GitHubApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "GitHub API request failed with HTTP {}: {}",
            self.status, self.message
        )
    }
}

impl std::error::Error for GitHubApiError {}

pub struct GitHubClient {
    http_client: Arc<dyn HttpClient>,
    access_token: Arc<str>,
}

impl GitHubClient {
    pub fn new(http_client: Arc<dyn HttpClient>, access_token: impl Into<Arc<str>>) -> Self {
        Self {
            http_client,
            access_token: access_token.into(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.send_json::<T, ()>(Method::GET, path, None).await
    }

    pub async fn viewer(&self) -> Result<User> {
        let viewer: ApiUser = self.get("/user").await?;
        Ok(viewer.into())
    }

    pub async fn list_pull_requests(
        &self,
        owner: &str,
        repository: &str,
    ) -> Result<PullRequestList> {
        let viewer = self.viewer().await?;
        let mut pulls = Vec::new();
        for page in 1.. {
            let page_items: Vec<ApiPullRequest> = self
                .get(&format!(
                    "/repos/{owner}/{repository}/pulls?state=open&sort=updated&direction=desc&per_page=100&page={page}"
                ))
                .await?;
            let is_last_page = page_items.len() < 100;
            pulls.extend(page_items);
            if is_last_page {
                break;
            }
        }
        let mut waiting_for_review = Vec::new();
        let mut authored_by_viewer = Vec::new();
        for pull in pulls {
            let summary = pull.into_summary(owner, repository)?;
            if summary.author.login == viewer.login {
                authored_by_viewer.push(summary.clone());
            }
            if summary
                .requested_reviewers
                .iter()
                .any(|reviewer| reviewer.login == viewer.login)
            {
                waiting_for_review.push(summary);
            }
        }
        Ok(PullRequestList {
            waiting_for_review,
            authored_by_viewer,
        })
    }

    pub async fn pull_request_details(
        &self,
        pull_request: &PullRequestId,
    ) -> Result<PullRequestDetails> {
        let path = format!(
            "/repos/{}/{}/pulls/{}",
            pull_request.owner, pull_request.repository, pull_request.number
        );
        let details: ApiPullRequestDetails = self.get(&path).await?;
        let summary = details
            .pull
            .into_summary(&pull_request.owner, &pull_request.repository)?;
        let files = self.pull_request_files(pull_request).await?;
        let checks = self.checks(pull_request, &summary.head_sha).await?;
        let review_data = self.review_data(pull_request).await?;
        Ok(PullRequestDetails {
            summary,
            body: details.body.unwrap_or_default().into(),
            additions: details.additions,
            deletions: details.deletions,
            changed_files: details.changed_files,
            mergeability: match details.mergeable {
                Some(true) => Mergeability::Mergeable,
                Some(false) => Mergeability::Conflicting,
                None => Mergeability::Unknown,
            },
            review_decision: review_data.review_decision,
            files,
            threads: review_data.threads,
            checks,
            pending_review: review_data.pending_review,
        })
    }

    async fn pull_request_files(&self, pull_request: &PullRequestId) -> Result<Vec<ChangedFile>> {
        let mut files = Vec::new();
        for page in 1.. {
            let page_items: Vec<ApiChangedFile> = self
                .get(&format!(
                    "/repos/{}/{}/pulls/{}/files?per_page=100&page={page}",
                    pull_request.owner, pull_request.repository, pull_request.number
                ))
                .await?;
            let is_last_page = page_items.len() < 100;
            files.extend(page_items.into_iter().map(Into::into));
            if is_last_page {
                break;
            }
        }
        Ok(files)
    }

    async fn checks(
        &self,
        pull_request: &PullRequestId,
        head_sha: &str,
    ) -> Result<Vec<CheckSummary>> {
        let response: ApiCheckRuns = self
            .get(&format!(
                "/repos/{}/{}/commits/{head_sha}/check-runs?per_page=100",
                pull_request.owner, pull_request.repository
            ))
            .await?;
        Ok(response.check_runs.into_iter().map(Into::into).collect())
    }

    async fn review_data(&self, pull_request: &PullRequestId) -> Result<ReviewData> {
        const QUERY: &str = r#"
            query PullRequestReviewData($owner: String!, $repository: String!, $number: Int!) {
              repository(owner: $owner, name: $repository) {
                pullRequest(number: $number) {
                  reviewDecision
                  reviews(last: 20, states: PENDING) {
                    nodes { id body commit { oid } author { login avatarUrl url } }
                  }
                  reviewThreads(first: 100) {
                    nodes {
                      id isResolved isOutdated viewerCanResolve viewerCanReply
                      comments(first: 100) {
                        nodes {
                          databaseId body createdAt url path line originalLine startLine
                          diffSide originalCommit { oid }
                          author { login avatarUrl url }
                        }
                      }
                    }
                  }
                }
              }
            }
        "#;
        let response: ReviewDataResponse = self
            .graphql(
                QUERY,
                serde_json::json!({
                    "owner": pull_request.owner.as_ref(),
                    "repository": pull_request.repository.as_ref(),
                    "number": pull_request.number,
                }),
            )
            .await?;
        response
            .repository
            .and_then(|repository| repository.pull_request)
            .map(Into::into)
            .context("GitHub pull request review data was missing")
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        self.send_json(Method::POST, path, Some(body)).await
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let _: Option<serde_json::Value> = self
            .send_json::<Option<serde_json::Value>, ()>(Method::DELETE, path, None)
            .await?;
        Ok(())
    }

    pub async fn create_pending_review(
        &self,
        pull_request: &PullRequestId,
        commit_sha: &str,
    ) -> Result<ReviewId> {
        #[derive(Serialize)]
        struct Body<'a> {
            commit_id: &'a str,
        }
        #[derive(Deserialize)]
        struct Response {
            id: u64,
        }
        let response: Response = self
            .post(
                &format!(
                    "/repos/{}/{}/pulls/{}/reviews",
                    pull_request.owner, pull_request.repository, pull_request.number
                ),
                &Body {
                    commit_id: commit_sha,
                },
            )
            .await?;
        Ok(ReviewId(response.id.to_string().into()))
    }

    pub async fn add_pending_comment(
        &self,
        pull_request: &PullRequestId,
        review_id: &ReviewId,
        anchor: &ReviewAnchor,
        body: &str,
    ) -> Result<CommentId> {
        #[derive(Serialize)]
        struct Body<'a> {
            body: &'a str,
            path: &'a str,
            line: u32,
            side: &'static str,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_line: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_side: Option<&'static str>,
        }
        #[derive(Deserialize)]
        struct Response {
            id: u64,
        }
        let response: Response = self
            .post(
                &format!(
                    "/repos/{}/{}/pulls/{}/reviews/{}/comments",
                    pull_request.owner, pull_request.repository, pull_request.number, review_id.0
                ),
                &Body {
                    body,
                    path: &anchor.path,
                    line: anchor.line,
                    side: anchor.side.as_github_str(),
                    start_line: anchor.start_line,
                    start_side: anchor.start_line.map(|_| anchor.side.as_github_str()),
                },
            )
            .await?;
        Ok(CommentId(response.id))
    }

    pub async fn submit_review(
        &self,
        pull_request: &PullRequestId,
        review_id: &ReviewId,
        event: ReviewEvent,
        body: &str,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Body<'a> {
            event: &'static str,
            body: &'a str,
        }
        let _: serde_json::Value = self
            .post(
                &format!(
                    "/repos/{}/{}/pulls/{}/reviews/{}/events",
                    pull_request.owner, pull_request.repository, pull_request.number, review_id.0
                ),
                &Body {
                    event: event.as_github_str(),
                    body,
                },
            )
            .await?;
        Ok(())
    }

    pub async fn discard_review(
        &self,
        pull_request: &PullRequestId,
        review_id: &ReviewId,
    ) -> Result<()> {
        self.delete(&format!(
            "/repos/{}/{}/pulls/{}/reviews/{}",
            pull_request.owner, pull_request.repository, pull_request.number, review_id.0
        ))
        .await
    }

    pub async fn reply_to_comment(
        &self,
        pull_request: &PullRequestId,
        comment_id: CommentId,
        body: &str,
    ) -> Result<CommentId> {
        #[derive(Serialize)]
        struct Body<'a> {
            body: &'a str,
            in_reply_to: u64,
        }
        #[derive(Deserialize)]
        struct Response {
            id: u64,
        }
        let response: Response = self
            .post(
                &format!(
                    "/repos/{}/{}/pulls/{}/comments",
                    pull_request.owner, pull_request.repository, pull_request.number
                ),
                &Body {
                    body,
                    in_reply_to: comment_id.0,
                },
            )
            .await?;
        Ok(CommentId(response.id))
    }

    pub async fn set_thread_resolved(&self, thread_id: &ThreadId, resolved: bool) -> Result<()> {
        let mutation = if resolved {
            "mutation($threadId: ID!) { resolveReviewThread(input: {threadId: $threadId}) { thread { id } } }"
        } else {
            "mutation($threadId: ID!) { unresolveReviewThread(input: {threadId: $threadId}) { thread { id } } }"
        };
        let _: serde_json::Value = self
            .graphql(
                mutation,
                serde_json::json!({ "threadId": thread_id.0.as_ref() }),
            )
            .await?;
        Ok(())
    }

    pub async fn graphql<T: DeserializeOwned, V: Serialize>(
        &self,
        query: &str,
        variables: V,
    ) -> Result<T> {
        #[derive(Serialize)]
        struct GraphQlRequest<'a, V> {
            query: &'a str,
            variables: V,
        }
        let response: GraphQlResponse<T> = self
            .post("/graphql", &GraphQlRequest { query, variables })
            .await?;
        if let Some(errors) = response.errors
            && let Some(error) = errors.into_iter().next()
        {
            anyhow::bail!("GitHub GraphQL request failed: {}", error.message);
        }
        response.data.context("GitHub GraphQL response had no data")
    }

    async fn send_json<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let url = format!("{API_BASE_URL}{path}");
        let body = match body {
            Some(body) => AsyncBody::from(serde_json::to_vec(body)?),
            None => AsyncBody::default(),
        };
        let request = Request::builder()
            .method(method)
            .uri(url)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
            .body(body)?;
        let mut response = self.http_client.send(request).await?;
        let status = response.status();
        let retry_after_seconds = response
            .headers()
            .get("retry-after")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse().ok());
        let mut bytes = Vec::new();
        response.body_mut().read_to_end(&mut bytes).await?;
        if !status.is_success() {
            let message = serde_json::from_slice::<ApiErrorBody>(&bytes)
                .map(|body| body.message)
                .unwrap_or_else(|_| status.canonical_reason().unwrap_or("unknown error").into());
            return Err(GitHubApiError {
                status,
                message,
                retry_after_seconds,
            }
            .into());
        }
        if bytes.is_empty() {
            return serde_json::from_slice(b"null")
                .context("failed to decode empty GitHub API response");
        }
        serde_json::from_slice(&bytes).context("failed to decode GitHub API response")
    }
}

#[derive(Deserialize)]
struct ApiUser {
    login: String,
    avatar_url: Option<String>,
    html_url: Option<String>,
}

impl From<ApiUser> for User {
    fn from(user: ApiUser) -> Self {
        Self {
            login: user.login.into(),
            avatar_url: user.avatar_url.map(Into::into),
            html_url: user.html_url.map(Into::into),
        }
    }
}

#[derive(Deserialize)]
struct ApiPullRequest {
    number: u32,
    title: String,
    user: ApiUser,
    state: PullRequestState,
    draft: bool,
    head: ApiBranch,
    base: ApiBranch,
    updated_at: String,
    html_url: String,
    #[serde(default)]
    requested_reviewers: Vec<ApiUser>,
}

impl ApiPullRequest {
    fn into_summary(self, owner: &str, repository: &str) -> Result<PullRequestSummary> {
        if self.head.sha.is_empty() {
            anyhow::bail!("GitHub pull request #{} had an empty head SHA", self.number);
        }
        Ok(PullRequestSummary {
            id: PullRequestId::new(owner, repository, self.number),
            title: self.title.into(),
            author: self.user.into(),
            state: self.state,
            is_draft: self.draft,
            head_ref: self.head.reference.into(),
            head_sha: self.head.sha.into(),
            base_ref: self.base.reference.into(),
            updated_at: self.updated_at.into(),
            html_url: self.html_url.into(),
            requested_reviewers: self
                .requested_reviewers
                .into_iter()
                .map(Into::into)
                .collect(),
        })
    }
}

#[derive(Deserialize)]
struct ApiBranch {
    #[serde(rename = "ref")]
    reference: String,
    sha: String,
}

#[derive(Deserialize)]
struct ApiPullRequestDetails {
    #[serde(flatten)]
    pull: ApiPullRequest,
    body: Option<String>,
    additions: u32,
    deletions: u32,
    changed_files: u32,
    mergeable: Option<bool>,
}

#[derive(Deserialize)]
struct ApiChangedFile {
    filename: String,
    previous_filename: Option<String>,
    additions: u32,
    deletions: u32,
    status: String,
    patch: Option<String>,
}

impl From<ApiChangedFile> for ChangedFile {
    fn from(file: ApiChangedFile) -> Self {
        Self {
            path: file.filename.into(),
            previous_path: file.previous_filename.map(Into::into),
            additions: file.additions,
            deletions: file.deletions,
            status: file.status.into(),
            patch: file.patch.map(Into::into),
        }
    }
}

#[derive(Deserialize)]
struct ApiCheckRuns {
    check_runs: Vec<ApiCheckRun>,
}

#[derive(Deserialize)]
struct ApiCheckRun {
    name: String,
    status: String,
    conclusion: Option<String>,
    details_url: Option<String>,
}

impl From<ApiCheckRun> for CheckSummary {
    fn from(check: ApiCheckRun) -> Self {
        let state = match check.status.as_str() {
            "queued" => CheckState::Queued,
            "in_progress" => CheckState::InProgress,
            _ => CheckState::Completed,
        };
        let conclusion = check
            .conclusion
            .as_deref()
            .map(|conclusion| match conclusion {
                "success" => CheckConclusion::Success,
                "failure" => CheckConclusion::Failure,
                "cancelled" => CheckConclusion::Cancelled,
                "neutral" => CheckConclusion::Neutral,
                "skipped" => CheckConclusion::Skipped,
                "timed_out" => CheckConclusion::TimedOut,
                "action_required" => CheckConclusion::ActionRequired,
                "stale" => CheckConclusion::Stale,
                _ => CheckConclusion::Unknown,
            });
        Self {
            name: check.name.into(),
            state,
            conclusion,
            details_url: check.details_url.map(Into::into),
        }
    }
}

struct ReviewData {
    review_decision: ReviewDecision,
    threads: Vec<ReviewThread>,
    pending_review: Option<PullRequestReview>,
}

#[derive(Deserialize)]
struct ReviewDataResponse {
    repository: Option<ReviewRepository>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReviewRepository {
    pull_request: Option<GraphQlPullRequest>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlPullRequest {
    review_decision: Option<String>,
    reviews: GraphQlConnection<GraphQlPendingReview>,
    review_threads: GraphQlConnection<GraphQlThread>,
}

#[derive(Deserialize)]
struct GraphQlConnection<T> {
    nodes: Vec<T>,
}

#[derive(Deserialize)]
struct GraphQlUser {
    login: String,
    #[serde(rename = "avatarUrl")]
    avatar_url: Option<String>,
    url: Option<String>,
}

impl From<GraphQlUser> for User {
    fn from(user: GraphQlUser) -> Self {
        Self {
            login: user.login.into(),
            avatar_url: user.avatar_url.map(Into::into),
            html_url: user.url.map(Into::into),
        }
    }
}

#[derive(Deserialize)]
struct GraphQlCommit {
    oid: String,
}

#[derive(Deserialize)]
struct GraphQlPendingReview {
    id: String,
    body: String,
    commit: GraphQlCommit,
    author: Option<GraphQlUser>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlThread {
    id: String,
    is_resolved: bool,
    is_outdated: bool,
    viewer_can_resolve: bool,
    viewer_can_reply: bool,
    comments: GraphQlConnection<GraphQlComment>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphQlComment {
    database_id: Option<u64>,
    body: String,
    created_at: String,
    url: String,
    path: String,
    line: Option<u32>,
    original_line: Option<u32>,
    start_line: Option<u32>,
    diff_side: Option<String>,
    original_commit: Option<GraphQlCommit>,
    author: Option<GraphQlUser>,
}

impl From<GraphQlPullRequest> for ReviewData {
    fn from(pull_request: GraphQlPullRequest) -> Self {
        let review_decision = match pull_request.review_decision.as_deref() {
            Some("APPROVED") => ReviewDecision::Approved,
            Some("CHANGES_REQUESTED") => ReviewDecision::ChangesRequested,
            Some("REVIEW_REQUIRED") => ReviewDecision::ReviewRequired,
            _ => ReviewDecision::Unknown,
        };
        let pending_review =
            pull_request
                .reviews
                .nodes
                .into_iter()
                .next()
                .map(|review| PullRequestReview {
                    id: ReviewId(review.id.into()),
                    commit_sha: review.commit.oid.into(),
                    author: review.author.map(Into::into).unwrap_or_else(unknown_user),
                    body: review.body.into(),
                });
        let threads = pull_request
            .review_threads
            .nodes
            .into_iter()
            .map(|thread| {
                let anchor = thread.comments.nodes.first().and_then(|comment| {
                    let line = comment.line.or(comment.original_line)?;
                    let commit_sha = comment.original_commit.as_ref()?.oid.clone();
                    Some(ReviewAnchor {
                        path: comment.path.clone().into(),
                        commit_sha: commit_sha.into(),
                        side: match comment.diff_side.as_deref() {
                            Some("LEFT") => DiffSide::Left,
                            _ => DiffSide::Right,
                        },
                        start_line: comment.start_line,
                        line,
                    })
                });
                ReviewThread {
                    id: ThreadId(thread.id.into()),
                    anchor,
                    is_resolved: thread.is_resolved,
                    is_outdated: thread.is_outdated,
                    viewer_can_reply: thread.viewer_can_reply,
                    viewer_can_resolve: thread.viewer_can_resolve,
                    comments: thread
                        .comments
                        .nodes
                        .into_iter()
                        .filter_map(|comment| {
                            Some(ReviewComment {
                                id: CommentId(comment.database_id?),
                                author: comment.author.map(Into::into).unwrap_or_else(unknown_user),
                                body: comment.body.into(),
                                created_at: comment.created_at.into(),
                                html_url: comment.url.into(),
                            })
                        })
                        .collect(),
                }
            })
            .collect();
        Self {
            review_decision,
            threads,
            pending_review,
        }
    }
}

fn unknown_user() -> User {
    User {
        login: "ghost".into(),
        avatar_url: None,
        html_url: None,
    }
}

#[derive(Deserialize)]
struct ApiErrorBody {
    message: String,
}

#[derive(Deserialize)]
struct GraphQlResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQlError>>,
}

#[derive(Deserialize)]
struct GraphQlError {
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiffSide;
    use http_client::{FakeHttpClient, Response};
    use pretty_assertions::assert_eq;

    #[test]
    fn serializes_review_anchor_with_current_line_api() {
        let anchor = ReviewAnchor {
            path: "src/main.rs".into(),
            commit_sha: "abc".into(),
            side: DiffSide::Right,
            start_line: Some(10),
            line: 12,
        };
        assert_eq!(anchor.side.as_github_str(), "RIGHT");
    }

    #[gpui::test]
    async fn reports_api_error_without_echoing_response(cx: &mut gpui::TestAppContext) {
        let client = FakeHttpClient::create(|request| async move {
            assert_eq!(request.headers()["authorization"], "Bearer test-token");
            Ok(Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(AsyncBody::from(r#"{"message":"Resource not accessible"}"#))?)
        });
        let client = GitHubClient::new(client, "test-token");
        let error = client
            .get::<serde_json::Value>("/user")
            .await
            .expect_err("request should fail");
        assert!(error.to_string().contains("Resource not accessible"));
        assert!(!error.to_string().contains("test-token"));
        cx.run_until_parked();
    }
}
