use anyhow::{Context as _, Result, bail, ensure};
use futures::{AsyncWriteExt as _, FutureExt as _, future::BoxFuture};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Duration,
};
use util::ResultExt as _;

use crate::review_provider::{ReviewProviderKind, ReviewRepositoryChoice};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ReviewRepository {
    pub id: u64,
    pub full_name: String,
    #[serde(default)]
    pub provider: ReviewProviderKind,
    #[serde(default = "default_github_host")]
    pub host: String,
    #[serde(default)]
    pub web_url: Option<String>,
}

fn default_github_host() -> String {
    "github.com".into()
}

impl ReviewRepository {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.id != 0,
            "The review provider returned an invalid repository identity"
        );
        match self.provider {
            ReviewProviderKind::GitHub => validate_repository_name(&self.full_name),
            ReviewProviderKind::GitLab => validate_gitlab_repository_name(&self.full_name),
        }
    }
    pub fn endpoint(&self, suffix: &str) -> String {
        format!("repos/{}/{}", self.full_name, suffix)
    }

    pub fn provider_name(&self) -> &'static str {
        self.provider.name()
    }
}

pub(crate) fn validate_repository_name(name: &str) -> Result<()> {
    let parts: Vec<_> = name.split('/').collect();
    ensure!(
        parts.len() == 2
            && parts.iter().all(|part| !part.is_empty()
                && *part != "."
                && *part != ".."
                && part
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))),
        "Invalid GitHub repository name"
    );
    Ok(())
}

fn validate_gitlab_repository_name(name: &str) -> Result<()> {
    let parts: Vec<_> = name.split('/').collect();
    ensure!(
        parts.len() >= 2
            && parts.iter().all(|part| !part.is_empty()
                && *part != "."
                && *part != ".."
                && part
                    .bytes()
                    .all(|character| character.is_ascii_alphanumeric()
                        || b"-_.".contains(&character))),
        "Invalid GitLab project path"
    );
    Ok(())
}

#[cfg(test)]
pub(crate) fn repository_from_remote(remote: &str) -> Option<String> {
    let remote: git::RemoteUrl = remote.parse().ok()?;
    if remote.host_str()? != "github.com" {
        return None;
    }
    let name = remote
        .path()
        .trim_start_matches('/')
        .trim_end_matches(".git");
    validate_repository_name(name).ok()?;
    Some(name.to_string())
}

pub(crate) fn repository_choice_from_remote(remote: &str) -> Option<ReviewRepositoryChoice> {
    let parsed: git::RemoteUrl = remote.parse().ok()?;
    let host = parsed.host_str()?.to_string();
    let full_name = parsed
        .path()
        .trim_start_matches('/')
        .trim_end_matches(".git")
        .to_string();
    if full_name.is_empty()
        || full_name.split('/').any(|part| {
            part.is_empty()
                || matches!(part, "." | "..")
                || !part.bytes().all(|character| {
                    character.is_ascii_alphanumeric() || b"-_./".contains(&character)
                })
        })
    {
        return None;
    }
    Some(ReviewRepositoryChoice {
        provider: if host.eq_ignore_ascii_case("github.com") {
            ReviewProviderKind::GitHub
        } else {
            ReviewProviderKind::GitLab
        },
        host,
        full_name,
        remote_url: remote.to_string(),
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ReviewUser {
    pub login: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ReviewRequestRef {
    #[serde(rename = "ref")]
    pub branch: String,
    pub sha: String,
    pub repo: Option<ReviewRepository>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ReviewRequest {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub user: ReviewUser,
    pub state: String,
    pub merged_at: Option<String>,
    pub head: ReviewRequestRef,
    pub base: ReviewRequestRef,
    #[serde(default)]
    pub start_sha: Option<String>,
    #[serde(default)]
    pub web_url: Option<String>,
}

impl ReviewRequest {
    pub fn validate(&self, repo: &ReviewRepository) -> Result<()> {
        repo.validate()?;
        ensure!(
            self.number > 0
                && self
                    .base
                    .repo
                    .as_ref()
                    .is_some_and(|base| base.id == repo.id),
            "Review belongs to a different repository"
        );
        validate_sha(&self.head.sha)?;
        validate_sha(&self.base.sha)?;
        Ok(())
    }
    pub fn url(&self, repo: &ReviewRepository) -> String {
        self.web_url.clone().unwrap_or_else(|| match repo.provider {
            ReviewProviderKind::GitHub => {
                format!(
                    "https://{}/{}/pull/{}",
                    repo.host, repo.full_name, self.number
                )
            }
            ReviewProviderKind::GitLab => format!(
                "https://{}/{}/-/merge_requests/{}",
                repo.host, repo.full_name, self.number
            ),
        })
    }
}

pub(crate) fn validate_sha(sha: &str) -> Result<()> {
    ensure!(
        (sha.len() == 40 || sha.len() == 64) && sha.bytes().all(|c| c.is_ascii_hexdigit()),
        "Invalid Git commit identity"
    );
    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ReviewRequestSummaryData {
    pub number: u64,
    pub title: String,
}
impl From<ReviewRequest> for ReviewRequestSummaryData {
    fn from(pr: ReviewRequest) -> Self {
        Self {
            number: pr.number,
            title: pr.title,
        }
    }
}

#[derive(Deserialize)]
struct ReviewRequestSearch {
    total_count: u32,
    incomplete_results: bool,
    items: Vec<ReviewRequestSummaryData>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum DiffSide {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum CommentTarget {
    General,
    Edit {
        comment_id: u64,
        comment_kind: CommentKind,
    },
    Reply {
        comment_id: u64,
    },
    Inline {
        path: String,
        side: DiffSide,
        start_line: u32,
        line: u32,
        head_sha: String,
        base_sha: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CommentKind {
    Conversation,
    Review,
    Inline,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RemoteComment {
    #[serde(default)]
    pub thread: Option<Arc<ReviewThread>>,
    pub id: u64,
    pub body: Option<String>,
    pub user: ReviewUser,
    pub created_at: Option<String>,
    pub submitted_at: Option<String>,
    pub path: Option<String>,
    #[serde(default)]
    pub old_path: Option<String>,
    #[serde(default)]
    pub new_path: Option<String>,
    pub line: Option<u32>,
    pub original_line: Option<u32>,
    pub start_line: Option<u32>,
    pub side: Option<DiffSide>,
    pub commit_id: Option<String>,
    pub original_commit_id: Option<String>,
    pub in_reply_to_id: Option<u64>,
    pub diff_hunk: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct DiscussionComment {
    pub kind: CommentKind,
    pub comment: RemoteComment,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReviewThread {
    pub id: String,
    pub is_resolved: bool,
    pub is_outdated: bool,
    pub viewer_can_resolve: bool,
    pub viewer_can_unresolve: bool,
    pub viewer_can_reply: bool,
    #[serde(default)]
    pub comments: Vec<ThreadComment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ThreadComment {
    #[serde(
        rename = "fullDatabaseId",
        deserialize_with = "deserialize_database_id"
    )]
    pub database_id: u64,
    pub viewer_did_author: bool,
    pub viewer_can_update: bool,
    pub viewer_can_delete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum DiscussionAction {
    Delete {
        comment_id: u64,
        comment_kind: CommentKind,
    },
    Resolve {
        thread_id: String,
        resolved: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ApiMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}
impl ApiMethod {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Patch => "PATCH",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ApiRequest {
    pub method: ApiMethod,
    pub writing: bool,
    pub endpoint: String,
    pub body: Option<Value>,
}

pub(crate) trait GitHubTransport: Send + Sync {
    fn request(&self, request: ApiRequest) -> BoxFuture<'static, Result<Value>>;
}

#[derive(Debug)]
pub(crate) struct ReviewProviderFailure {
    pub message: String,
    pub outcome_unknown: bool,
}
impl std::fmt::Display for ReviewProviderFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}
impl std::error::Error for ReviewProviderFailure {}

pub(crate) struct GhCli {
    executor: gpui::BackgroundExecutor,
}
impl GitHubTransport for GhCli {
    fn request(&self, request: ApiRequest) -> BoxFuture<'static, Result<Value>> {
        let executor = self.executor.clone();
        async move {
            ensure!((request.endpoint.starts_with("repos/") || request.endpoint == "graphql" || (!request.writing && (request.endpoint == "user" || request.endpoint.starts_with("search/issues?")))) && !request.endpoint.contains(['\n', '\r', '#']), "Invalid GitHub endpoint");
            let writing = request.writing;
            let has_body = request.body.is_some();
            let mut command = smol::process::Command::new("gh");
            command.args(["api", "--hostname", "github.com", "--method", request.method.as_str(),
                "--header", "Accept: application/vnd.github+json", "--header", "X-GitHub-Api-Version: 2026-03-10", &request.endpoint])
                .env("GH_PROMPT_DISABLED", "1").env("GH_DEBUG", "").kill_on_drop(true)
                .stdin(if has_body { Stdio::piped() } else { Stdio::null() }).stdout(Stdio::piped()).stderr(Stdio::piped());
            if has_body { command.args(["--input", "-"]); }
            let mut child = command.spawn().map_err(|_| ReviewProviderFailure { message: "GitHub CLI could not start. Install gh and run gh auth login.".into(), outcome_unknown: false })?;
            let operation = async move {
                if let Some(body) = request.body {
                    let mut stdin = child.stdin.take().context("GitHub CLI input is unavailable")?;
                    stdin.write_all(&serde_json::to_vec(&body)?).await.map_err(|_| ReviewProviderFailure { message: "GitHub request was interrupted. Refresh before retrying.".into(), outcome_unknown: writing })?;
                    drop(stdin);
                }
                let output = child.output().await.map_err(|_| ReviewProviderFailure { message: "GitHub request was interrupted. Refresh before retrying.".into(), outcome_unknown: writing })?;
                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    let (message, known) = if error.contains("HTTP 401") || error.contains("gh auth login") {
                        ("GitHub authentication is required. Run gh auth login.", true)
                    } else if error.contains("HTTP 403") {
                        ("GitHub denied this request. Check repository permissions or rate limits.", true)
                    } else if error.contains("HTTP 404") {
                        ("The GitHub repository or PR was not found, or this account cannot access it.", true)
                    } else if error.contains("HTTP 422") {
                        ("GitHub rejected this comment target. Refresh the PR and check the selected lines.", true)
                    } else { ("GitHub request failed. Refresh to check its outcome before retrying.", false) };
                    return Err(ReviewProviderFailure { message: message.into(), outcome_unknown: writing && !known }.into());
                }
                if request.method == ApiMethod::Delete && output.stdout.is_empty() { return Ok(Value::Null); }
                serde_json::from_slice(&output.stdout).map_err(|_| ReviewProviderFailure { message: "GitHub returned an unreadable response. Refresh before retrying a post.".into(), outcome_unknown: writing }.into())
            }.boxed();
            match futures::future::select(operation, executor.timer(Duration::from_secs(45)).boxed()).await {
                futures::future::Either::Left((result, _)) => result,
                futures::future::Either::Right(_) => Err(ReviewProviderFailure { message: "GitHub request timed out. Refresh to check whether a comment was posted.".into(), outcome_unknown: writing }.into()),
            }
        }.boxed()
    }
}

#[derive(Clone)]
pub(crate) struct GitHubClient {
    transport: Arc<dyn GitHubTransport>,
}
impl GitHubClient {
    pub fn new(executor: gpui::BackgroundExecutor) -> Self {
        Self {
            transport: Arc::new(GhCli { executor }),
        }
    }
}

#[derive(Clone)]
pub(crate) enum ReviewClient {
    GitHub(GitHubClient),
    GitLab(crate::gitlab_review::GitLabClient),
}

impl ReviewClient {
    pub(crate) fn for_choice(
        choice: ReviewRepositoryChoice,
        root: PathBuf,
        executor: gpui::BackgroundExecutor,
    ) -> Self {
        match choice.provider {
            ReviewProviderKind::GitHub => Self::GitHub(GitHubClient::new(executor)),
            ReviewProviderKind::GitLab => Self::GitLab(crate::gitlab_review::GitLabClient::new(
                choice, root, executor,
            )),
        }
    }

    pub(crate) async fn repository(
        &self,
        choice: &ReviewRepositoryChoice,
    ) -> Result<ReviewRepository> {
        match self {
            Self::GitHub(client) => {
                let mut repository = client.repository(&choice.full_name).await?;
                repository.provider = ReviewProviderKind::GitHub;
                repository.host = choice.host.clone();
                repository.web_url = Some(format!("https://{}/{}", choice.host, choice.full_name));
                Ok(repository)
            }
            Self::GitLab(client) => client.repository().await,
        }
    }

    pub(crate) async fn viewer(&self) -> Result<ReviewUser> {
        match self {
            Self::GitHub(client) => client.viewer().await,
            Self::GitLab(client) => client.viewer().await,
        }
    }

    pub(crate) async fn review_threads(
        &self,
        repo: &ReviewRepository,
        number: u64,
    ) -> Result<Vec<ReviewThread>> {
        match self {
            Self::GitHub(client) => client.review_threads(repo, number).await,
            Self::GitLab(client) => client.review_threads(number).await,
        }
    }

    pub(crate) async fn update_comment(
        &self,
        repo: &ReviewRepository,
        number: u64,
        kind: CommentKind,
        id: u64,
        original: &str,
        body: &str,
    ) -> Result<RemoteComment> {
        match self {
            Self::GitHub(client) => client.update_comment(repo, kind, id, original, body).await,
            Self::GitLab(client) => client.update_comment(number, id, original, body).await,
        }
    }

    pub(crate) async fn discussion_action(
        &self,
        repo: &ReviewRepository,
        number: u64,
        action: &DiscussionAction,
    ) -> Result<()> {
        match self {
            Self::GitHub(client) => client.discussion_action(repo, number, action).await,
            Self::GitLab(client) => client.discussion_action(number, action).await,
        }
    }

    pub(crate) async fn request_summaries(
        &self,
        repo: &ReviewRepository,
        state: &str,
        page: u32,
        query: Option<&str>,
    ) -> Result<(Vec<ReviewRequestSummaryData>, bool)> {
        match self {
            Self::GitHub(client) => {
                if let Some(query) = query.filter(|query| !query.trim().is_empty()) {
                    client.search_pull_requests(repo, query, state, page).await
                } else {
                    let values = client.pull_requests(repo, state, page).await?;
                    let next = values.len() == 100;
                    Ok((values.into_iter().map(Into::into).collect(), next))
                }
            }
            Self::GitLab(client) => {
                let values = client.merge_request_summaries(state, page, query).await?;
                let next = values.len() == 100;
                Ok((values, next))
            }
        }
    }

    pub(crate) async fn review_request(
        &self,
        repo: &ReviewRepository,
        number: u64,
    ) -> Result<ReviewRequest> {
        match self {
            Self::GitHub(client) => client.pull_request(repo, number).await,
            Self::GitLab(client) => client.merge_request(repo, number).await,
        }
    }

    pub(crate) async fn discussion(
        &self,
        repo: &ReviewRepository,
        number: u64,
    ) -> Result<Vec<DiscussionComment>> {
        match self {
            Self::GitHub(client) => client.discussion(repo, number).await,
            Self::GitLab(client) => client.discussion(number).await,
        }
    }

    pub(crate) async fn validate_published_target(
        &self,
        repo: &ReviewRepository,
        pr: &ReviewRequest,
        target: &CommentTarget,
    ) -> Result<()> {
        match self {
            Self::GitHub(client) => client.validate_published_target(repo, pr, target).await,
            Self::GitLab(client) => client.validate_target(repo, pr, target).await,
        }
    }

    pub(crate) async fn post(
        &self,
        repo: &ReviewRepository,
        pr: &ReviewRequest,
        target: &CommentTarget,
        body: &str,
    ) -> Result<RemoteComment> {
        match self {
            Self::GitHub(client) => client.post(repo, pr, target, body).await,
            Self::GitLab(client) => client.post(repo, pr, target, body).await,
        }
    }
}

impl GitHubClient {
    async fn graphql(&self, query: &str, variables: Value, writing: bool) -> Result<Value> {
        let response = self
            .transport
            .request(ApiRequest {
                endpoint: "graphql".into(),
                method: ApiMethod::Post,
                writing,
                body: Some(json!({"query": query, "variables": variables})),
            })
            .await?;
        if response
            .get("errors")
            .is_some_and(|errors| errors.as_array().is_none_or(|errors| !errors.is_empty()))
        {
            return Err(ReviewProviderFailure {
                message:
                    "GitHub could not complete the thread request. Refresh and check permissions."
                        .into(),
                outcome_unknown: writing,
            }
            .into());
        }
        response
            .get("data")
            .filter(|data| !data.is_null())
            .cloned()
            .ok_or_else(|| {
                ReviewProviderFailure {
                    message: "GitHub returned incomplete thread data".into(),
                    outcome_unknown: writing,
                }
                .into()
            })
    }

    pub async fn viewer(&self) -> Result<ReviewUser> {
        self.get("user".into()).await
    }

    pub async fn review_threads(
        &self,
        repo: &ReviewRepository,
        number: u64,
    ) -> Result<Vec<ReviewThread>> {
        repo.validate()?;
        let (owner, name) = repo
            .full_name
            .split_once('/')
            .context("Invalid repository")?;
        let mut cursor = Value::Null;
        let mut threads = Vec::new();
        loop {
            let data = self
                .graphql(
                    THREADS_QUERY,
                    json!({"owner":owner,"name":name,"number":number,"cursor":cursor}),
                    false,
                )
                .await?;
            let connection = &data["repository"]["pullRequest"]["reviewThreads"];
            let nodes = connection["nodes"]
                .as_array()
                .context("GitHub thread list is unavailable")?;
            for node in nodes {
                let mut thread: ReviewThread = serde_json::from_value({
                    let mut node = node.clone();
                    node["comments"] = json!([]);
                    node
                })?;
                thread.comments = serde_json::from_value(node["comments"]["nodes"].clone())?;
                let mut replies = next_cursor(&node["comments"])?;
                while let Some(cursor) = replies {
                    let data = self
                        .graphql(
                            THREAD_COMMENTS_QUERY,
                            json!({"id":thread.id,"cursor":cursor}),
                            false,
                        )
                        .await?;
                    let connection = &data["node"]["comments"];
                    thread
                        .comments
                        .extend(serde_json::from_value::<Vec<ThreadComment>>(
                            connection["nodes"].clone(),
                        )?);
                    let next = next_cursor(connection)?;
                    ensure!(
                        next.as_deref() != Some(&cursor),
                        "GitHub repeated a comment page"
                    );
                    replies = next;
                }
                threads.push(thread);
            }
            match next_cursor(connection)? {
                Some(next) => {
                    ensure!(
                        cursor.as_str() != Some(&next),
                        "GitHub repeated a thread page"
                    );
                    cursor = json!(next);
                }
                None => return Ok(threads),
            }
        }
    }

    pub async fn update_comment(
        &self,
        repo: &ReviewRepository,
        kind: CommentKind,
        id: u64,
        original: &str,
        body: &str,
    ) -> Result<RemoteComment> {
        ensure!(!body.trim().is_empty(), "Write a comment before saving");
        let endpoint = comment_endpoint(repo, kind, id)?;
        let viewer = self.viewer().await?;
        let current: RemoteComment = self.get(endpoint.clone()).await?;
        ensure!(
            current.user.login.eq_ignore_ascii_case(&viewer.login),
            "Only your own comments can be edited"
        );
        ensure!(
            current.body.as_deref().unwrap_or_default() == original,
            "This comment changed on GitHub. Your draft is kept; refresh and compare before editing again."
        );
        let value = self
            .transport
            .request(ApiRequest {
                endpoint,
                method: ApiMethod::Patch,
                writing: true,
                body: Some(json!({"body":body})),
            })
            .await?;
        serde_json::from_value(value).map_err(|_| {
            ReviewProviderFailure {
                message: "The edit may have succeeded; refresh before retrying".into(),
                outcome_unknown: true,
            }
            .into()
        })
    }

    pub async fn discussion_action(
        &self,
        repo: &ReviewRepository,
        number: u64,
        action: &DiscussionAction,
    ) -> Result<()> {
        repo.validate()?;
        match action {
            DiscussionAction::Delete {
                comment_id,
                comment_kind,
            } => {
                let endpoint = comment_endpoint(repo, *comment_kind, *comment_id)?;
                let viewer = self.viewer().await?;
                let current: RemoteComment = self.get(endpoint.clone()).await?;
                ensure!(
                    current.user.login.eq_ignore_ascii_case(&viewer.login),
                    "Only your own comments can be deleted"
                );
                self.transport
                    .request(ApiRequest {
                        endpoint,
                        method: ApiMethod::Delete,
                        writing: true,
                        body: None,
                    })
                    .await?;
            }
            DiscussionAction::Resolve {
                thread_id,
                resolved,
            } => {
                let threads = self.review_threads(repo, number).await?;
                let thread = threads
                    .iter()
                    .find(|thread| &thread.id == thread_id)
                    .context("Thread no longer exists")?;
                if thread.is_resolved == *resolved {
                    return Ok(());
                }
                ensure!(
                    if *resolved {
                        thread.viewer_can_resolve
                    } else {
                        thread.viewer_can_unresolve
                    },
                    "GitHub does not permit this thread action"
                );
                let mutation = if *resolved {
                    "mutation($id:ID!){resolveReviewThread(input:{threadId:$id}){thread{id isResolved}}}"
                } else {
                    "mutation($id:ID!){unresolveReviewThread(input:{threadId:$id}){thread{id isResolved}}}"
                };
                let data = self
                    .graphql(mutation, json!({"id":thread_id}), true)
                    .await?;
                let operation = if *resolved {
                    "resolveReviewThread"
                } else {
                    "unresolveReviewThread"
                };
                if data[operation]["thread"]["id"].as_str() != Some(thread_id)
                    || data[operation]["thread"]["isResolved"].as_bool() != Some(*resolved)
                {
                    return Err(ReviewProviderFailure {
                        message: "GitHub did not confirm the thread state; refresh before retrying"
                            .into(),
                        outcome_unknown: true,
                    }
                    .into());
                }
            }
        }
        Ok(())
    }

    pub async fn repository(&self, full_name: &str) -> Result<ReviewRepository> {
        validate_repository_name(full_name)?;
        let repo: ReviewRepository = self.get(format!("repos/{full_name}")).await?;
        repo.validate()?;
        Ok(repo)
    }
    async fn get<T: serde::de::DeserializeOwned>(&self, endpoint: String) -> Result<T> {
        serde_json::from_value(
            self.transport
                .request(ApiRequest {
                    endpoint,
                    method: ApiMethod::Get,
                    writing: false,
                    body: None,
                })
                .await?,
        )
        .context("Invalid GitHub response")
    }
    pub async fn pull_requests(
        &self,
        repo: &ReviewRepository,
        state: &str,
        page: u32,
    ) -> Result<Vec<ReviewRequest>> {
        repo.validate()?;
        ensure!(
            matches!(state, "open" | "closed" | "all") && page > 0,
            "Invalid PR filter"
        );
        self.get(repo.endpoint(&format!(
            "pulls?state={state}&sort=updated&direction=desc&per_page=100&page={page}"
        )))
        .await
    }

    pub async fn search_pull_requests(
        &self,
        repo: &ReviewRepository,
        title: &str,
        state: &str,
        page: u32,
    ) -> Result<(Vec<ReviewRequestSummaryData>, bool)> {
        repo.validate()?;
        ensure!(
            matches!(state, "open" | "closed" | "all") && (1..=10).contains(&page),
            "Invalid PR search filter"
        );
        let title = title.replace(['"', '\\', '\n', '\r'], " ");
        let mut query = format!(
            "repo:{} is:pr in:title \"{}\"",
            repo.full_name,
            title.trim()
        );
        if state != "all" {
            query.push_str(&format!(" state:{state}"));
        }
        let parameters = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("q", &query)
            .append_pair("sort", "updated")
            .append_pair("order", "desc")
            .append_pair("per_page", "100")
            .append_pair("page", &page.to_string())
            .finish();
        let result: ReviewRequestSearch = self.get(format!("search/issues?{parameters}")).await?;
        ensure!(
            !result.incomplete_results,
            "GitHub returned an incomplete search. Narrow the title and try again."
        );
        let next = page < 10 && page.saturating_mul(100) < result.total_count;
        Ok((result.items, next))
    }

    pub async fn pull_request(
        &self,
        repo: &ReviewRepository,
        number: u64,
    ) -> Result<ReviewRequest> {
        let pr: ReviewRequest = self.get(repo.endpoint(&format!("pulls/{number}"))).await?;
        pr.validate(repo)?;
        ensure!(
            pr.number == number,
            "GitHub returned a different PR than requested"
        );
        Ok(pr)
    }
    async fn all_comments(&self, endpoint: String) -> Result<Vec<RemoteComment>> {
        let mut comments = Vec::new();
        let mut page = 1;
        loop {
            let values: Vec<RemoteComment> = self
                .get(format!("{endpoint}?per_page=100&page={page}"))
                .await?;
            let done = values.len() < 100;
            comments.extend(values);
            if done {
                return Ok(comments);
            }
            page += 1;
        }
    }
    pub async fn discussion(
        &self,
        repo: &ReviewRepository,
        number: u64,
    ) -> Result<Vec<DiscussionComment>> {
        repo.validate()?;
        let endpoints = [
            (
                CommentKind::Conversation,
                format!("issues/{number}/comments"),
            ),
            (CommentKind::Review, format!("pulls/{number}/reviews")),
            (CommentKind::Inline, format!("pulls/{number}/comments")),
        ];
        let mut discussion = Vec::new();
        for (kind, endpoint) in endpoints {
            discussion.extend(
                self.all_comments(repo.endpoint(&endpoint))
                    .await?
                    .into_iter()
                    .filter(|comment| comment.body.as_ref().is_some_and(|body| !body.is_empty()))
                    .map(|comment| DiscussionComment { kind, comment }),
            );
        }
        discussion.sort_by(|a, b| {
            a.comment
                .created_at
                .as_ref()
                .or(a.comment.submitted_at.as_ref())
                .cmp(
                    &b.comment
                        .created_at
                        .as_ref()
                        .or(b.comment.submitted_at.as_ref()),
                )
        });
        Ok(discussion)
    }
    pub async fn validate_published_target(
        &self,
        repo: &ReviewRepository,
        pr: &ReviewRequest,
        target: &CommentTarget,
    ) -> Result<()> {
        let CommentTarget::Inline {
            path,
            side,
            start_line,
            line,
            ..
        } = target
        else {
            return Ok(());
        };
        #[derive(Deserialize)]
        struct File {
            filename: String,
            patch: Option<String>,
        }
        for page in 1..=30 {
            let files: Vec<File> = self
                .get(repo.endpoint(&format!(
                    "pulls/{}/files?per_page=100&page={page}",
                    pr.number
                )))
                .await?;
            let count = files.len();
            if let Some(file) = files.into_iter().find(|file| file.filename == *path) {
                ensure!(
                    file.patch.as_ref().is_some_and(|patch| range_in_patch(
                        patch,
                        *side,
                        *start_line,
                        *line
                    )),
                    "GitHub cannot verify these lines in the published diff. Your draft is kept."
                );
                let current = self.pull_request(repo, pr.number).await?;
                ensure!(
                    current.head.sha == pr.head.sha && current.base.sha == pr.base.sha,
                    "The PR changed while validating this comment. Refresh before posting."
                );
                return Ok(());
            }
            if count < 100 {
                break;
            }
        }
        bail!(
            "This path is not in GitHub's published PR file list. Renamed or unavailable targets must be selected again."
        )
    }

    pub async fn post(
        &self,
        repo: &ReviewRepository,
        pr: &ReviewRequest,
        target: &CommentTarget,
        body: &str,
    ) -> Result<RemoteComment> {
        pr.validate(repo)?;
        ensure!(!body.trim().is_empty(), "Write a comment before posting");
        let (suffix, body) = match target {
            CommentTarget::Edit { .. } => bail!("Use Save changes to edit a comment"),
            CommentTarget::General => (
                format!("issues/{}/comments", pr.number),
                json!({"body":body}),
            ),
            CommentTarget::Reply { comment_id } => {
                ensure!(*comment_id > 0, "Invalid comment thread");
                (
                    format!("pulls/{}/comments/{comment_id}/replies", pr.number),
                    json!({"body":body}),
                )
            }
            CommentTarget::Inline {
                path,
                side,
                start_line,
                line,
                head_sha,
                base_sha,
            } => {
                ensure!(
                    head_sha == &pr.head.sha && base_sha == &pr.base.sha,
                    "The PR revision changed. Refresh and select the comment target again."
                );
                ensure!(
                    *start_line > 0 && start_line <= line,
                    "Invalid comment range"
                );
                ensure!(
                    !Path::new(path).is_absolute()
                        && !path.split('/').any(|part| part == "..")
                        && !path.is_empty(),
                    "Invalid comment path"
                );
                let mut body =
                    json!({"body":body,"commit_id":head_sha,"path":path,"line":line,"side":side});
                if start_line != line {
                    body["start_line"] = json!(start_line);
                    body["start_side"] = json!(side);
                }
                (format!("pulls/{}/comments", pr.number), body)
            }
        };
        serde_json::from_value(self.transport.request(ApiRequest { endpoint: repo.endpoint(&suffix), method: ApiMethod::Post, writing: true, body: Some(body) }).await?).map_err(|_| ReviewProviderFailure { message: "GitHub posted the comment but returned an unreadable response; refresh before retrying".into(), outcome_unknown: true }.into())
    }
}

const THREADS_QUERY: &str = "query($owner:String!,$name:String!,$number:Int!,$cursor:String){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100,after:$cursor){nodes{id isResolved isOutdated viewerCanResolve viewerCanUnresolve viewerCanReply comments(first:100){nodes{fullDatabaseId viewerDidAuthor viewerCanUpdate viewerCanDelete} pageInfo{hasNextPage endCursor}}} pageInfo{hasNextPage endCursor}}}}}";
const THREAD_COMMENTS_QUERY: &str = "query($id:ID!,$cursor:String){node(id:$id){... on PullRequestReviewThread{comments(first:100,after:$cursor){nodes{fullDatabaseId viewerDidAuthor viewerCanUpdate viewerCanDelete} pageInfo{hasNextPage endCursor}}}}}";

fn deserialize_database_id<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<u64, D::Error> {
    let value = Value::deserialize(deserializer)?;
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
        .ok_or_else(|| serde::de::Error::custom("Invalid GitHub comment ID"))
}

fn next_cursor(connection: &Value) -> Result<Option<String>> {
    if connection["pageInfo"]["hasNextPage"]
        .as_bool()
        .context("Missing GitHub pagination")?
    {
        Ok(Some(
            connection["pageInfo"]["endCursor"]
                .as_str()
                .filter(|cursor| !cursor.is_empty())
                .context("Missing GitHub page cursor")?
                .to_owned(),
        ))
    } else {
        Ok(None)
    }
}

fn comment_endpoint(repo: &ReviewRepository, kind: CommentKind, id: u64) -> Result<String> {
    repo.validate()?;
    ensure!(id > 0, "Invalid comment");
    Ok(repo.endpoint(&match kind {
        CommentKind::Conversation => format!("issues/comments/{id}"),
        CommentKind::Inline => format!("pulls/comments/{id}"),
        CommentKind::Review => bail!("Submitted review summaries are read-only"),
    }))
}

pub(crate) fn pr_number(query: &str, repo: &ReviewRepository) -> Result<u64> {
    let query = query.trim();
    let number = if let Some(url) = query.strip_prefix("https://github.com/") {
        let prefix = format!("{}/pull/", repo.full_name);
        url.strip_prefix(&prefix)
            .context("Choose a PR from the selected repository")?
            .split(['/', '?', '#'])
            .next()
            .unwrap_or_default()
    } else {
        query.trim_start_matches('#')
    };
    let number = number
        .parse::<u64>()
        .context("Enter a PR number or its GitHub URL")?;
    if number == 0 {
        bail!("PR numbers start at 1");
    }
    Ok(number)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Checkout {
    pub repository: ReviewRepository,
    pub pull_request: ReviewRequest,
    pub branch: String,
    pub base_ref: String,
    #[serde(default)]
    pub warning: Option<String>,
}

impl Checkout {
    pub fn review_key(&self, worktree: &Path) -> Result<String> {
        if self.repository.provider == ReviewProviderKind::GitHub {
            Ok(format!(
                "github_review_v1:{}",
                crate::review_state::digest(&[&serde_json::to_vec(&(
                    worktree,
                    self.repository.id,
                    self.pull_request.number,
                    &self.pull_request.base.branch
                ))?])
            ))
        } else {
            Ok(format!(
                "provider_review_v1:{}",
                crate::review_state::digest(&[&serde_json::to_vec(&(
                    worktree,
                    "gitlab",
                    &self.repository.host,
                    self.repository.id,
                    self.pull_request.number,
                    &self.pull_request.base.branch
                ))?])
            ))
        }
    }
}

pub(crate) async fn git_output(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    arguments: &[&str],
) -> Result<Vec<u8>> {
    let operation = smol::process::Command::new("git")
        .current_dir(root)
        .arg("--literal-pathspecs")
        .args(arguments)
        .env("GIT_TERMINAL_PROMPT", "0")
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .output()
        .boxed();
    match futures::future::select(operation, executor.timer(Duration::from_secs(120)).boxed()).await
    {
        futures::future::Either::Left((output, _)) => {
            let output = output.context("Git could not start")?;
            ensure!(
                output.status.success(),
                "Git {} failed. Check repository access and Git status.",
                arguments.first().unwrap_or(&"command")
            );
            Ok(output.stdout)
        }
        futures::future::Either::Right(_) => {
            bail!("Git timed out. Check repository access and retry.")
        }
    }
}

async fn git_text(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    arguments: &[&str],
) -> Result<String> {
    Ok(
        String::from_utf8(git_output(executor, root, arguments).await?)?
            .trim()
            .to_owned(),
    )
}

async fn git_blob(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    revision: &str,
    path: &str,
) -> Result<Option<Vec<u8>>> {
    let entry = git_output(executor, root, &["ls-tree", "-z", revision, "--", path]).await?;
    if entry.is_empty() {
        return Ok(None);
    }
    ensure!(
        entry.starts_with(b"100644 blob ") || entry.starts_with(b"100755 blob "),
        "Inline comments require an ordinary text file"
    );
    Ok(Some(
        git_output(executor, root, &["show", &format!("{revision}:{path}")]).await?,
    ))
}

pub(crate) async fn check_clean_checkout(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
) -> Result<()> {
    ensure!(
        git_output(
            executor,
            root,
            &[
                "status",
                "--porcelain=v1",
                "-z",
                "--untracked-files=normal",
                "--ignore-submodules=none"
            ]
        )
        .await?
        .is_empty(),
        "Commit or move local changes, including untracked files, before opening or updating a review. Nothing was stashed or discarded."
    );
    for marker in [
        "MERGE_HEAD",
        "CHERRY_PICK_HEAD",
        "REVERT_HEAD",
        "rebase-merge",
        "rebase-apply",
        "sequencer",
        "BISECT_LOG",
        "index.lock",
    ] {
        let path = git_text(executor, root, &["rev-parse", "--git-path", marker]).await?;
        match smol::fs::metadata(root.join(path)).await {
            Ok(_) => bail!("Finish the active Git operation before changing the review checkout"),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct LocalReviewBranch {
    name: String,
    tip: String,
    upstream_remote: Option<String>,
    upstream_ref: Option<String>,
    association: Option<crate::review_provider::ReviewCheckoutAssociation>,
}

fn checkout_identity(
    repo: &ReviewRepository,
    pr: &ReviewRequest,
) -> crate::review_provider::ReviewCheckoutIdentity {
    let source_repository = pr.head.repo.as_ref().unwrap_or(repo);
    crate::review_provider::ReviewCheckoutIdentity {
        provider: match repo.provider {
            ReviewProviderKind::GitHub => "github",
            ReviewProviderKind::GitLab => "gitlab",
        }
        .into(),
        repository_id: repo.id.to_string(),
        review_number: pr.number,
        source_host: repo.host.clone(),
        source_repository: source_repository.full_name.clone(),
        source_available: pr.head.repo.is_some(),
        source_branch: pr.head.branch.clone(),
        target_branch: pr.base.branch.clone(),
    }
}

fn association_key(branch: &str) -> String {
    format!("branch.{branch}.zed-review-association")
}

async fn local_review_branches(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
) -> Result<Vec<LocalReviewBranch>> {
    let config = git_output(executor, root, &["config", "--local", "--null", "--list"]).await?;
    let mut associations: HashMap<String, crate::review_provider::ReviewCheckoutAssociation> =
        HashMap::new();
    for entry in String::from_utf8(config)?.split('\0') {
        let Some((key, value)) = entry.split_once('\n') else {
            continue;
        };
        let Some(branch) = key
            .strip_prefix("branch.")
            .and_then(|key| key.strip_suffix(".zed-review-association"))
        else {
            continue;
        };
        if let Ok(association) = serde_json::from_str(value) {
            associations.insert(branch.to_string(), association);
        }
    }
    let refs = git_text(
        executor,
        root,
        &[
            "for-each-ref",
            "--format=%(refname:short)%09%(objectname)%09%(upstream:remotename)%09%(upstream:remoteref)",
            "refs/heads/",
        ],
    )
    .await?;
    Ok(refs
        .lines()
        .filter_map(|line| {
            let mut fields = line.splitn(4, '\t');
            let name = fields.next()?.to_string();
            let tip = fields.next()?.to_string();
            let upstream_remote = fields
                .next()
                .filter(|value| !value.is_empty())
                .map(str::to_string);
            let upstream_ref = fields
                .next()
                .filter(|value| !value.is_empty())
                .map(str::to_string);
            let association = associations.remove(&name);
            Some(LocalReviewBranch {
                name,
                tip,
                upstream_remote,
                upstream_ref,
                association,
            })
        })
        .collect())
}

async fn remote_repositories(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    branches: &[LocalReviewBranch],
) -> HashMap<String, String> {
    let mut result = HashMap::default();
    let remotes: HashSet<_> = branches
        .iter()
        .filter_map(|branch| branch.upstream_remote.clone())
        .collect();
    for remote in remotes {
        let repository = match git_text(executor, root, &["remote", "get-url", &remote]).await {
            Ok(url) => repository_choice_from_remote(&url).map(|choice| choice.full_name),
            Err(error) => {
                log::warn!("could not inspect review checkout remote {remote}: {error:#}");
                None
            }
        };
        if let Some(repository) = repository {
            result.insert(remote, repository);
        }
    }
    result
}

async fn branches_in_other_worktrees(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
) -> Result<HashSet<String>> {
    let output = git_text(executor, root, &["worktree", "list", "--porcelain"]).await?;
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let mut result = HashSet::default();
    let mut worktree = None;
    let mut branch = None;
    for line in output.lines().chain(std::iter::once("")) {
        if let Some(path) = line.strip_prefix("worktree ") {
            worktree = Some(path);
        } else if let Some(name) = line.strip_prefix("branch refs/heads/") {
            branch = Some(name);
        } else if line.is_empty() {
            if worktree.is_some_and(|path| {
                Path::new(path)
                    .canonicalize()
                    .unwrap_or_else(|_| Path::new(path).to_path_buf())
                    != root
            }) && let Some(branch) = branch.take()
            {
                result.insert(branch.to_string());
            }
            worktree = None;
            branch = None;
        }
    }
    Ok(result)
}

async fn write_checkout_association(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    branch: &str,
    identity: &crate::review_provider::ReviewCheckoutIdentity,
    head_sha: &str,
    managed: bool,
) -> Result<()> {
    let value = serde_json::to_string(&crate::review_provider::ReviewCheckoutAssociation {
        identity: identity.clone(),
        last_head_sha: head_sha.to_string(),
        managed,
    })?;
    git_output(
        executor,
        root,
        &[
            "config",
            "--local",
            "--replace-all",
            &association_key(branch),
            &value,
        ],
    )
    .await?;
    Ok(())
}

pub(crate) async fn checkout_review_request(
    executor: &gpui::BackgroundExecutor,
    root: std::path::PathBuf,
    repo: ReviewRepository,
    pr: ReviewRequest,
    previous: Option<Checkout>,
    ready: impl FnMut() -> Result<()>,
) -> Result<Checkout> {
    let url = if repo.provider == ReviewProviderKind::GitLab {
        review_remote_url(executor, &root, &repo)
            .await
            .unwrap_or_else(|| format!("https://{}/{}.git", repo.host, repo.full_name))
    } else {
        format!("https://{}/{}.git", repo.host, repo.full_name)
    };
    checkout_from_remote(executor, root, repo, pr, previous, url, ready).await
}

async fn review_remote_url(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    repo: &ReviewRepository,
) -> Option<String> {
    let remotes = git_text(executor, root, &["remote", "-v"]).await.ok()?;
    remotes.lines().find_map(|line| {
        let mut fields = line.split_whitespace();
        let _name = fields.next()?;
        let url = fields.next()?;
        let choice = repository_choice_from_remote(url)?;
        (choice.host.eq_ignore_ascii_case(&repo.host) && choice.full_name == repo.full_name)
            .then(|| url.to_string())
    })
}

async fn checkout_from_remote(
    executor: &gpui::BackgroundExecutor,
    root: std::path::PathBuf,
    repo: ReviewRepository,
    pr: ReviewRequest,
    previous: Option<Checkout>,
    url: String,
    mut ready: impl FnMut() -> Result<()>,
) -> Result<Checkout> {
    pr.validate(&repo)?;
    ready()?;
    check_clean_checkout(executor, &root).await?;
    git_output(
        executor,
        &root,
        &[
            "check-ref-format",
            &format!("refs/heads/{}", pr.base.branch),
        ],
    )
    .await?;
    git_output(
        executor,
        &root,
        &[
            "check-ref-format",
            &format!("refs/heads/{}", pr.head.branch),
        ],
    )
    .await?;
    let prefix = if repo.provider == ReviewProviderKind::GitHub {
        format!("refs/zed/reviews/{}/{}", repo.id, pr.number)
    } else {
        format!(
            "refs/zed/reviews/gitlab-{}/{}/{}",
            crate::review_state::digest(&[repo.host.as_bytes()]),
            repo.id,
            pr.number
        )
    };
    let head_ref = format!("{prefix}/head-{}", pr.head.sha);
    let base_ref = format!("{prefix}/base-{}", pr.base.sha);
    let head_source = match repo.provider {
        ReviewProviderKind::GitHub => format!("refs/pull/{}/head", pr.number),
        ReviewProviderKind::GitLab => format!("refs/merge-requests/{}/head", pr.number),
    };
    let fetched_base_ref = format!("{prefix}/target-{}", pr.base.sha);
    let mut fetch_arguments = Vec::new();
    if repo.provider == ReviewProviderKind::GitHub {
        fetch_arguments.extend([
            "-c".to_string(),
            "credential.helper=".to_string(),
            "-c".to_string(),
            format!(
                "credential.https://{}.helper=!gh auth git-credential",
                repo.host
            ),
        ]);
    }
    fetch_arguments.extend([
        "fetch".into(),
        "--no-tags".into(),
        "--no-write-fetch-head".into(),
        url,
        format!("{head_source}:{head_ref}"),
        format!("refs/heads/{}:{fetched_base_ref}", pr.base.branch),
    ]);
    let fetch_arguments = fetch_arguments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    git_output(executor, &root, &fetch_arguments).await?;
    ensure!(
        git_text(
            executor,
            &root,
            &["cat-file", "-e", &format!("{}^{{commit}}", pr.base.sha)]
        )
        .await
        .is_ok(),
        "The review base revision is no longer available from the target repository"
    );
    git_output(executor, &root, &["update-ref", &base_ref, &pr.base.sha]).await?;
    ensure!(
        git_text(executor, &root, &["rev-parse", &head_ref]).await? == pr.head.sha
            && git_text(executor, &root, &["rev-parse", &base_ref]).await? == pr.base.sha,
        "The review changed during fetch. Refresh its metadata before opening it."
    );
    let current = git_text(
        executor,
        &root,
        &["symbolic-ref", "--quiet", "--short", "HEAD"],
    )
    .await?;
    let head = git_text(executor, &root, &["rev-parse", "HEAD"]).await?;
    ready()?;
    check_clean_checkout(executor, &root).await?;
    ensure!(
        git_text(
            executor,
            &root,
            &["symbolic-ref", "--quiet", "--short", "HEAD"]
        )
        .await?
            == current
            && git_text(executor, &root, &["rev-parse", "HEAD"]).await? == head,
        "The checkout changed during fetch. Try again."
    );
    ready()?;
    let identity = checkout_identity(&repo, &pr);
    let branches = local_review_branches(executor, &root).await?;
    let remote_repositories = remote_repositories(executor, &root, &branches).await;
    let other_worktrees = branches_in_other_worktrees(executor, &root).await?;
    let previous_matches = previous.as_ref().is_some_and(|checkout| {
        checkout_identity(&checkout.repository, &checkout.pull_request) == identity
    });
    let source_ref = format!("refs/heads/{}", identity.source_branch);
    let mut candidate_names = Vec::new();
    let mut add_candidate = |name: &str| {
        if !candidate_names.iter().any(|candidate| candidate == name) {
            candidate_names.push(name.to_string());
        }
    };
    let qualifies = |branch: &LocalReviewBranch| {
        let previous = previous_matches
            && previous
                .as_ref()
                .is_some_and(|checkout| checkout.branch == branch.name);
        let associated = branch
            .association
            .as_ref()
            .is_some_and(|association| association.identity == identity);
        let upstream = branch.upstream_ref.as_deref() == Some(&source_ref)
            && branch.upstream_remote.as_ref().is_some_and(|remote| {
                remote_repositories.get(remote) == Some(&identity.source_repository)
            });
        let source_branch = identity.source_available
            && branch.name == identity.source_branch
            && branch
                .association
                .as_ref()
                .is_none_or(|association| association.identity == identity);
        previous || associated || upstream || source_branch
    };
    if branches
        .iter()
        .find(|branch| branch.name == current)
        .is_some_and(qualifies)
    {
        add_candidate(&current);
    }
    if previous_matches && let Some(previous) = &previous {
        add_candidate(&previous.branch);
    }
    for branch in branches.iter().filter(|branch| {
        branch.upstream_ref.as_deref() == Some(&source_ref)
            && branch.upstream_remote.as_ref().is_some_and(|remote| {
                remote_repositories.get(remote) == Some(&identity.source_repository)
            })
    }) {
        add_candidate(&branch.name);
    }
    for branch in branches.iter().filter(|branch| {
        branch
            .association
            .as_ref()
            .is_some_and(|association| association.identity == identity)
    }) {
        add_candidate(&branch.name);
    }
    if identity.source_available
        && branches.iter().any(|branch| {
            branch.name == identity.source_branch
                && branch
                    .association
                    .as_ref()
                    .is_none_or(|association| association.identity == identity)
        })
    {
        add_candidate(&identity.source_branch);
    }

    let mut warnings = Vec::new();
    let mut selected = None;
    for name in candidate_names {
        let Some(branch) = branches.iter().find(|branch| branch.name == name) else {
            continue;
        };
        if other_worktrees.contains(&branch.name) {
            warnings.push(format!(
                "Preserved {} because it is checked out in another worktree.",
                branch.name
            ));
            continue;
        }
        let previous_managed = previous_matches
            && previous.as_ref().is_some_and(|checkout| {
                checkout.branch == branch.name
                    && (branch.name.starts_with("review/pr-")
                        || branch.name.starts_with("review/mr-"))
            });
        let managed = branch
            .association
            .as_ref()
            .is_some_and(|association| association.identity == identity && association.managed)
            || previous_managed;
        let is_fast_forward = if branch.tip == pr.head.sha {
            false
        } else {
            git_text(executor, &root, &["merge-base", &branch.tip, &head_ref])
                .await
                .is_ok_and(|merge_base| merge_base == branch.tip)
        };
        let may_follow_force_push = (managed
            && branch.association.as_ref().is_some_and(|association| {
                association.identity == identity && association.last_head_sha == branch.tip
            }))
            || (previous_managed
                && previous
                    .as_ref()
                    .is_some_and(|checkout| checkout.pull_request.head.sha == branch.tip));
        if branch.tip != pr.head.sha && !is_fast_forward && !may_follow_force_push {
            warnings.push(format!(
                "Preserved {} because it contains commits outside the current review.",
                branch.name
            ));
            continue;
        }
        ready()?;
        check_clean_checkout(executor, &root).await?;
        if branch.tip != pr.head.sha && !is_fast_forward && may_follow_force_push {
            // Keep the last provider-confirmed head reachable before moving a
            // Zed-managed branch across a force push. This is intentionally a
            // hidden review ref, so recovery does not add another local branch.
            let recovery_ref = format!("{prefix}/recovery-{}", branch.tip);
            git_output(executor, &root, &["update-ref", &recovery_ref, &branch.tip]).await?;
        }
        if branch.name == current {
            if is_fast_forward {
                git_output(executor, &root, &["merge", "--ff-only", &head_ref]).await?;
            } else if branch.tip != pr.head.sha {
                git_output(executor, &root, &["reset", "--hard", &head_ref]).await?;
            }
        } else {
            if branch.tip != pr.head.sha {
                git_output(
                    executor,
                    &root,
                    &["branch", "--force", &branch.name, &head_ref],
                )
                .await?;
            }
            git_output(executor, &root, &["switch", "--no-guess", &branch.name]).await?;
        }
        write_checkout_association(
            executor,
            &root,
            &branch.name,
            &identity,
            &pr.head.sha,
            managed,
        )
        .await?;
        selected = Some(branch.name.clone());
        break;
    }

    let branch = if let Some(branch) = selected {
        branch
    } else {
        let existing: HashSet<_> = branches.iter().map(|branch| branch.name.as_str()).collect();
        let preferred = identity.source_branch.clone();
        let request_prefix = if repo.provider == ReviewProviderKind::GitLab {
            "mr"
        } else {
            "pr"
        };
        let base = format!(
            "review/{request_prefix}-{}-{}",
            pr.number, identity.source_branch
        );
        let mut branch = if !identity.source_available {
            warnings.push(
                "The review source repository is unavailable; using a managed review branch."
                    .into(),
            );
            base.clone()
        } else if existing.contains(preferred.as_str()) {
            warnings.push(format!(
                "Preserved unrelated local branch {preferred}; using a managed review branch."
            ));
            base.clone()
        } else {
            preferred
        };
        let mut suffix = 2;
        while existing.contains(branch.as_str()) || other_worktrees.contains(&branch) {
            branch = format!("{base}-{suffix}");
            suffix += 1;
        }
        ready()?;
        check_clean_checkout(executor, &root).await?;
        git_output(
            executor,
            &root,
            &["switch", "--no-guess", "-c", &branch, &head_ref],
        )
        .await?;
        write_checkout_association(executor, &root, &branch, &identity, &pr.head.sha, true).await?;
        branch
    };
    Ok(Checkout {
        repository: repo,
        pull_request: pr,
        branch,
        base_ref,
        warning: (!warnings.is_empty()).then(|| warnings.join(" ")),
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CommentDraft {
    #[serde(default)]
    pub original_body: Option<String>,
    pub target: CommentTarget,
    pub body: String,
    pub outcome_unknown: bool,
}

pub(crate) async fn validate_inline(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    pr: &ReviewRequest,
    target: &CommentTarget,
    effective: Option<&str>,
    effective_base: Option<&str>,
) -> Result<()> {
    let CommentTarget::Inline {
        path,
        side,
        start_line,
        line,
        head_sha,
        base_sha,
    } = target
    else {
        return Ok(());
    };
    ensure!(
        head_sha == &pr.head.sha && base_sha == &pr.base.sha,
        "The review revision changed. Keep this draft and select a new target."
    );
    ensure!(
        *start_line > 0 && start_line <= line,
        "Invalid comment range"
    );
    ensure!(
        !path.is_empty()
            && !Path::new(path).is_absolute()
            && !path.split('/').any(|part| part == ".."),
        "Invalid comment path"
    );
    ensure!(
        git_text(executor, root, &["rev-parse", "HEAD"]).await? == pr.head.sha,
        "The checkout no longer matches this review revision"
    );
    let merge_base = git_text(executor, root, &["merge-base", &pr.base.sha, &pr.head.sha]).await?;
    let published_base = git_blob(executor, root, &merge_base, path).await?;
    ensure!(
        published_base.as_deref() == effective_base.map(str::as_bytes),
        "The editor base no longer matches the published review comparison. Your draft is kept."
    );
    let published = git_blob(executor, root, &pr.head.sha, path).await?;
    let current = match effective {
        Some(text) => Some(text.as_bytes().to_vec()),
        None => smol::fs::read(root.join(path)).await.ok(),
    };
    ensure!(
        published.as_ref() == current.as_ref(),
        "Local edits make this inline location uncertain. Your draft is kept; restore the published file before posting."
    );
    let patch = git_text(
        executor,
        root,
        &[
            "diff",
            "--no-ext-diff",
            "--no-textconv",
            "--unified=3",
            "--find-renames",
            &merge_base,
            &pr.head.sha,
            "--",
            path,
        ],
    )
    .await?;
    ensure!(
        range_in_patch(&patch, *side, *start_line, *line),
        "The selected lines are not in the published review diff. Your draft is kept."
    );
    Ok(())
}

pub(crate) fn range_in_patch(patch: &str, side: DiffSide, start: u32, end: u32) -> bool {
    start > 0
        && start <= end
        && patch
            .lines()
            .filter(|line| line.starts_with("@@ "))
            .any(|header| {
                let Some(range) =
                    header
                        .split_whitespace()
                        .nth(if side == DiffSide::Left { 1 } else { 2 })
                else {
                    return false;
                };
                let mut numbers = range.get(1..).unwrap_or_default().split(',');
                let Ok(first) = numbers.next().unwrap_or_default().parse::<u32>() else {
                    return false;
                };
                let Ok(count) = numbers.next().unwrap_or("1").parse::<u32>() else {
                    return false;
                };
                count > 0 && start >= first && end < first.saturating_add(count)
            })
}

#[derive(Clone, Debug)]
pub(crate) struct PublishedComment {
    pub comment: RemoteComment,
    pub current: Option<String>,
    pub base: Option<String>,
    pub provider_name: String,
}

impl PublishedComment {
    pub(crate) fn into_placed(self) -> Option<crate::review_provider::PlacedReviewComment> {
        let path = self
            .comment
            .new_path
            .clone()
            .or_else(|| self.comment.path.clone())?;
        let line = self.comment.line?;
        let side = match self.comment.side? {
            DiffSide::Left => crate::review_provider::ReviewDiffSide::Left,
            DiffSide::Right => crate::review_provider::ReviewDiffSide::Right,
        };
        let root_comment_id = self
            .comment
            .in_reply_to_id
            .unwrap_or(self.comment.id)
            .to_string();
        let thread = self.comment.thread.as_ref();
        Some(crate::review_provider::PlacedReviewComment {
            id: self.comment.id.to_string(),
            thread: crate::review_provider::ReviewThreadState {
                id: crate::review_provider::ReviewThreadId(
                    thread
                        .map(|thread| thread.id.clone())
                        .unwrap_or_else(|| format!("comment-{root_comment_id}")),
                ),
                resolved: thread.is_some_and(|thread| thread.is_resolved),
                outdated: thread.is_some_and(|thread| thread.is_outdated),
                can_reply: thread.is_some_and(|thread| thread.viewer_can_reply),
                can_resolve: thread.is_some_and(|thread| thread.viewer_can_resolve),
                can_reopen: thread.is_some_and(|thread| thread.viewer_can_unresolve),
            },
            root_comment_id,
            author: self.comment.user.login,
            body: self.comment.body.unwrap_or_default(),
            provider_name: self.provider_name,
            url: None,
            path,
            side,
            line,
            current: self.current,
            base: self.base,
        })
    }
}

pub(crate) async fn published_comments(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    checkout: &Checkout,
    discussion: &[DiscussionComment],
) -> Result<Vec<PublishedComment>> {
    let pr = &checkout.pull_request;
    let merge_base = git_text(executor, root, &["merge-base", &pr.base.sha, &pr.head.sha]).await?;
    let mut files = std::collections::BTreeMap::new();
    let mut result = Vec::new();
    for entry in discussion {
        let comment = &entry.comment;
        if entry.kind != CommentKind::Inline
            || comment.line.is_none()
            || comment.commit_id.as_deref() != Some(&pr.head.sha)
        {
            continue;
        }
        let Some(path) = &comment.path else {
            continue;
        };
        if !files.contains_key(path) {
            let pair: Result<_> = async {
                let current_path = comment.new_path.as_ref().unwrap_or(path);
                let old_path = comment.old_path.as_ref().unwrap_or(path);
                let current = git_blob(executor, root, &pr.head.sha, current_path)
                    .await?
                    .map(String::from_utf8)
                    .transpose()
                    .context("Inline comments require UTF-8 text")?;
                let base = git_blob(executor, root, &merge_base, old_path)
                    .await?
                    .map(String::from_utf8)
                    .transpose()
                    .context("Inline comments require UTF-8 text")?;
                Ok((current, base))
            }
            .await;
            files.insert(path.clone(), pair.log_err());
        }
        if let Some(Some((current, base))) = files.get(path) {
            result.push(PublishedComment {
                comment: comment.clone(),
                current: current.clone(),
                base: base.clone(),
                provider_name: checkout.repository.provider_name().into(),
            });
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::VecDeque, sync::Mutex};

    fn repo() -> ReviewRepository {
        ReviewRepository {
            id: 7,
            full_name: "owner/project".into(),
            provider: ReviewProviderKind::GitHub,
            host: "github.com".into(),
            web_url: None,
        }
    }
    fn pr() -> ReviewRequest {
        ReviewRequest {
            number: 12,
            title: "Review changes".into(),
            body: None,
            user: ReviewUser {
                login: "author".into(),
            },
            state: "open".into(),
            merged_at: None,
            head: ReviewRequestRef {
                branch: "feature".into(),
                sha: "a".repeat(40),
                repo: Some(ReviewRepository {
                    id: 8,
                    full_name: "contributor/project".into(),
                    provider: ReviewProviderKind::GitHub,
                    host: "github.com".into(),
                    web_url: None,
                }),
            },
            base: ReviewRequestRef {
                branch: "main".into(),
                sha: "b".repeat(40),
                repo: Some(repo()),
            },
            start_sha: None,
            web_url: None,
        }
    }
    fn comment() -> Value {
        json!({"id":1,"body":"Comment","user":{"login":"reviewer"}})
    }

    fn thread_page(resolved: bool, can_resolve: bool) -> Value {
        json!({"data":{"repository":{"pullRequest":{"reviewThreads":{
            "nodes":[{"id":"thread-1","isResolved":resolved,"isOutdated":false,"viewerCanResolve":can_resolve,"viewerCanUnresolve":true,"viewerCanReply":true,
                "comments":{"nodes":[{"fullDatabaseId":"1","viewerDidAuthor":true,"viewerCanUpdate":true,"viewerCanDelete":true}],"pageInfo":{"hasNextPage":false}}}],
            "pageInfo":{"hasNextPage":false}
        }}}}})
    }

    #[test]
    fn thread_and_reply_pages_are_read_only_even_with_post_transport() {
        smol::block_on(async {
            let mut first = thread_page(false, true);
            first["data"]["repository"]["pullRequest"]["reviewThreads"]["pageInfo"] =
                json!({"hasNextPage":true,"endCursor":"thread-page"});
            first["data"]["repository"]["pullRequest"]["reviewThreads"]["nodes"][0]["comments"]["pageInfo"] =
                json!({"hasNextPage":true,"endCursor":"reply-page"});
            let replies = json!({"data":{"node":{"comments":{"nodes":[{"fullDatabaseId":"9007199254740993","viewerDidAuthor":false,"viewerCanUpdate":false,"viewerCanDelete":false}],"pageInfo":{"hasNextPage":false}}}}});
            let last = json!({"data":{"repository":{"pullRequest":{"reviewThreads":{"nodes":[],"pageInfo":{"hasNextPage":false}}}}}});
            let (client, transport) = client(vec![Ok(first), Ok(replies), Ok(last)]);
            let threads = client.review_threads(&repo(), 12).await.unwrap();
            assert_eq!(threads.len(), 1);
            assert_eq!(threads[0].comments.len(), 2);
            assert_eq!(threads[0].comments[1].database_id, 9007199254740993);
            let requests = transport.requests.lock().unwrap();
            assert!(
                requests
                    .iter()
                    .all(|request| request.method == ApiMethod::Post && !request.writing)
            );
            assert_eq!(
                requests[1].body.as_ref().unwrap()["variables"]["cursor"],
                "reply-page"
            );
            assert_eq!(
                requests[2].body.as_ref().unwrap()["variables"]["cursor"],
                "thread-page"
            );
        });
    }

    #[test]
    fn comment_edits_validate_author_and_original_before_patch() {
        smol::block_on(async {
            let (client, transport) = client(vec![
                Ok(json!({"login":"reviewer"})),
                Ok(comment()),
                Ok(comment()),
            ]);
            client
                .update_comment(&repo(), CommentKind::Inline, 1, "Comment", "Edited")
                .await
                .unwrap();
            let requests = transport.requests.lock().unwrap();
            assert_eq!(requests[2].method, ApiMethod::Patch);
            assert!(requests[2].writing);
            assert_eq!(requests[2].endpoint, "repos/owner/project/pulls/comments/1");
            assert_eq!(requests[2].body.as_ref().unwrap()["body"], "Edited");
        });
        for (viewer, original) in [("someone-else", "Comment"), ("reviewer", "old body")] {
            smol::block_on(async {
                let (client, transport) = client(vec![Ok(json!({"login":viewer})), Ok(comment())]);
                assert!(
                    client
                        .update_comment(&repo(), CommentKind::Conversation, 1, original, "Edited")
                        .await
                        .is_err()
                );
                assert!(
                    transport
                        .requests
                        .lock()
                        .unwrap()
                        .iter()
                        .all(|request| !request.writing)
                );
            });
        }
    }

    #[test]
    fn deletion_has_no_body_and_submitted_reviews_stay_read_only() {
        smol::block_on(async {
            let (client, transport) = client(vec![
                Ok(json!({"login":"reviewer"})),
                Ok(comment()),
                Ok(Value::Null),
            ]);
            client
                .discussion_action(
                    &repo(),
                    12,
                    &DiscussionAction::Delete {
                        comment_id: 1,
                        comment_kind: CommentKind::Conversation,
                    },
                )
                .await
                .unwrap();
            let requests = transport.requests.lock().unwrap();
            assert_eq!(requests[2].method, ApiMethod::Delete);
            assert!(requests[2].writing && requests[2].body.is_none());
            assert_eq!(
                requests[2].endpoint,
                "repos/owner/project/issues/comments/1"
            );
            assert!(comment_endpoint(&repo(), CommentKind::Review, 1).is_err());
        });
    }

    #[test]
    fn thread_actions_check_permissions_and_graphql_write_errors_are_uncertain() {
        smol::block_on(async {
            let (client, transport) = client(vec![Ok(thread_page(false, false))]);
            assert!(
                client
                    .discussion_action(
                        &repo(),
                        12,
                        &DiscussionAction::Resolve {
                            thread_id: "thread-1".into(),
                            resolved: true
                        }
                    )
                    .await
                    .is_err()
            );
            assert_eq!(transport.requests.lock().unwrap().len(), 1);
        });
        for resolved in [false, true] {
            smol::block_on(async {
                let (client, transport) = client(vec![
                    Ok(thread_page(!resolved, true)),
                    Ok(json!({"errors":[{"message":"interrupted"}]})),
                ]);
                let error = client
                    .discussion_action(
                        &repo(),
                        12,
                        &DiscussionAction::Resolve {
                            thread_id: "thread-1".into(),
                            resolved,
                        },
                    )
                    .await
                    .unwrap_err();
                assert!(
                    error
                        .downcast_ref::<ReviewProviderFailure>()
                        .unwrap()
                        .outcome_unknown
                );
                let requests = transport.requests.lock().unwrap();
                assert!(requests[1].writing);
                let query = requests[1].body.as_ref().unwrap()["query"]
                    .as_str()
                    .unwrap();
                assert!(query.contains(if resolved {
                    "{resolveReviewThread"
                } else {
                    "{unresolveReviewThread"
                }));
            });
        }
    }
    struct MockTransport {
        requests: Mutex<Vec<ApiRequest>>,
        results: Mutex<VecDeque<Result<Value>>>,
    }
    impl GitHubTransport for MockTransport {
        fn request(&self, request: ApiRequest) -> BoxFuture<'static, Result<Value>> {
            self.requests.lock().unwrap().push(request);
            let result = self
                .results
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected GitHub request");
            async move { result }.boxed()
        }
    }
    fn client(results: Vec<Result<Value>>) -> (GitHubClient, Arc<MockTransport>) {
        let transport = Arc::new(MockTransport {
            requests: Mutex::new(Vec::new()),
            results: Mutex::new(results.into()),
        });
        (
            GitHubClient {
                transport: transport.clone(),
            },
            transport,
        )
    }

    #[test]
    fn repository_and_pr_input_cannot_escape_selected_host_or_repository() {
        for remote in [
            "git@github.com:owner/project.git",
            "https://github.com/owner/project.git",
            "ssh://git@github.com/owner/project",
        ] {
            assert_eq!(
                repository_from_remote(remote).as_deref(),
                Some("owner/project")
            );
        }
        for remote in [
            "https://github.com.evil/owner/project",
            "https://gitlab.com/owner/project",
            "https://github.com/owner/project/extra",
        ] {
            assert!(repository_from_remote(remote).is_none());
        }
        for name in [
            "owner/project?x=y",
            "owner/..",
            "../project",
            "owner/a/b",
            "owner/project\n",
        ] {
            assert!(validate_repository_name(name).is_err());
        }
        for query in [
            "12",
            "#12",
            "https://github.com/owner/project/pull/12/files",
        ] {
            assert_eq!(pr_number(query, &repo()).unwrap(), 12);
        }
        for query in [
            "0",
            "https://github.com/other/project/pull/12",
            "https://evil/owner/project/pull/12",
        ] {
            assert!(pr_number(query, &repo()).is_err());
        }
    }

    #[test]
    fn review_repository_choices_preserve_provider_and_host() {
        let github = repository_choice_from_remote("git@github.com:owner/project.git").unwrap();
        assert_eq!(github.provider, ReviewProviderKind::GitHub);
        assert_eq!(github.host, "github.com");
        assert_eq!(github.full_name, "owner/project");

        let gitlab =
            repository_choice_from_remote("https://gitlab.com/YashedP/SpotiNotifs.git").unwrap();
        assert_eq!(gitlab.provider, ReviewProviderKind::GitLab);
        assert_eq!(gitlab.host, "gitlab.com");
        assert_eq!(gitlab.full_name, "YashedP/SpotiNotifs");
    }

    #[test]
    fn inline_ranges_are_checked_on_the_correct_side_without_overflow() {
        let patch = "@@ -3,2 +7,3 @@\n-old\n+new\n@@ -12,0 +20 @@\n+added";
        assert!(range_in_patch(patch, DiffSide::Left, 3, 4));
        assert!(range_in_patch(patch, DiffSide::Right, 7, 9));
        assert!(range_in_patch(patch, DiffSide::Right, 20, 20));
        for (side, start, end) in [
            (DiffSide::Left, 7, 7),
            (DiffSide::Left, 12, 12),
            (DiffSide::Right, 7, 20),
            (DiffSide::Right, 0, 1),
            (DiffSide::Right, 9, 7),
            (DiffSide::Right, u32::MAX, u32::MAX),
        ] {
            assert!(!range_in_patch(patch, side, start, end));
        }
    }

    #[test]
    fn post_routes_general_replies_and_both_inline_sides_exactly() {
        smol::block_on(async {
            let pr = pr();
            let (client, transport) = client((0..4).map(|_| Ok(comment())).collect());
            client
                .post(&repo(), &pr, &CommentTarget::General, "general")
                .await
                .unwrap();
            client
                .post(
                    &repo(),
                    &pr,
                    &CommentTarget::Reply { comment_id: 44 },
                    "reply",
                )
                .await
                .unwrap();
            for side in [DiffSide::Left, DiffSide::Right] {
                client
                    .post(
                        &repo(),
                        &pr,
                        &CommentTarget::Inline {
                            path: "src/a.rs".into(),
                            side,
                            start_line: 2,
                            line: 4,
                            head_sha: pr.head.sha.clone(),
                            base_sha: pr.base.sha.clone(),
                        },
                        "inline",
                    )
                    .await
                    .unwrap();
            }
            let requests = transport.requests.lock().unwrap();
            assert_eq!(
                requests[0].endpoint,
                "repos/owner/project/issues/12/comments"
            );
            assert_eq!(
                requests[1].endpoint,
                "repos/owner/project/pulls/12/comments/44/replies"
            );
            for (index, side) in [(2, "LEFT"), (3, "RIGHT")] {
                let body = requests[index].body.as_ref().unwrap();
                assert_eq!(body["side"], side);
                assert_eq!(body["start_side"], side);
                assert_eq!(body["start_line"], 2);
                assert_eq!(body["line"], 4);
                assert_eq!(body["commit_id"], pr.head.sha);
                assert!(body.get("position").is_none());
            }
        });
    }

    #[test]
    fn published_file_validation_checks_path_side_and_revision_again() {
        smol::block_on(async {
            let pr = pr();
            let target = CommentTarget::Inline {
                path: "src/a.rs".into(),
                side: DiffSide::Right,
                start_line: 7,
                line: 8,
                head_sha: pr.head.sha.clone(),
                base_sha: pr.base.sha.clone(),
            };
            let files = json!([{"filename":"src/a.rs", "patch":"@@ -1,2 +7,2 @@\n-old\n+new"}]);
            let (client, transport) = client(vec![
                Ok(files.clone()),
                Ok(serde_json::to_value(&pr).unwrap()),
            ]);
            client
                .validate_published_target(&repo(), &pr, &target)
                .await
                .unwrap();
            assert!(
                transport
                    .requests
                    .lock()
                    .unwrap()
                    .iter()
                    .all(|request| request.body.is_none())
            );
            let mut advanced = pr.clone();
            advanced.head.sha = "c".repeat(40);
            let (client, _) =
                self::client(vec![Ok(files), Ok(serde_json::to_value(advanced).unwrap())]);
            assert!(
                client
                    .validate_published_target(&repo(), &pr, &target)
                    .await
                    .is_err()
            );
            let (client, _) = self::client(vec![Ok(
                json!([{"filename":"renamed.rs", "patch":"@@ -1 +7 @@"}]),
            )]);
            assert!(
                client
                    .validate_published_target(&repo(), &pr, &target)
                    .await
                    .is_err()
            );
        });
    }

    #[test]
    fn title_search_is_encoded_scoped_and_paginated() {
        smol::block_on(async {
            let (client, transport) = client(vec![Ok(
                json!({"total_count":205,"incomplete_results":false,"items":[{"number":12,"title":"Some title"}]}),
            )]);
            let (items, more) = client
                .search_pull_requests(&repo(), "branch & review", "closed", 2)
                .await
                .unwrap();
            assert!(more);
            assert_eq!(items[0].number, 12);
            let requests = transport.requests.lock().unwrap();
            let request = &requests[0];
            let query =
                url::Url::parse(&format!("https://api.github.com/{}", request.endpoint)).unwrap();
            let parameters: std::collections::BTreeMap<_, _> =
                query.query_pairs().into_owned().collect();
            assert_eq!(
                parameters["q"],
                "repo:owner/project is:pr in:title \"branch & review\" state:closed"
            );
            assert_eq!(parameters["page"], "2");
            assert!(request.body.is_none());
        });
    }

    #[test]
    fn stale_revision_and_invalid_targets_never_reach_the_transport() {
        smol::block_on(async {
            let (client, transport) = client(Vec::new());
            let pr = pr();
            for target in [
                CommentTarget::Inline {
                    path: "a".into(),
                    side: DiffSide::Right,
                    start_line: 1,
                    line: 1,
                    head_sha: "c".repeat(40),
                    base_sha: pr.base.sha.clone(),
                },
                CommentTarget::Inline {
                    path: "../a".into(),
                    side: DiffSide::Left,
                    start_line: 1,
                    line: 1,
                    head_sha: pr.head.sha.clone(),
                    base_sha: pr.base.sha.clone(),
                },
                CommentTarget::Reply { comment_id: 0 },
            ] {
                assert!(client.post(&repo(), &pr, &target, "comment").await.is_err());
            }
            assert!(
                client
                    .post(&repo(), &pr, &CommentTarget::General, "  ")
                    .await
                    .is_err()
            );
            assert!(transport.requests.lock().unwrap().is_empty());
        });
    }

    #[test]
    fn successful_but_unreadable_post_is_not_safe_to_retry() {
        smol::block_on(async {
            let (client, transport) = client(vec![Ok(json!({"unexpected":true}))]);
            let error = client
                .post(&repo(), &pr(), &CommentTarget::General, "comment")
                .await
                .unwrap_err();
            assert!(
                error
                    .downcast_ref::<ReviewProviderFailure>()
                    .unwrap()
                    .outcome_unknown
            );
            assert_eq!(transport.requests.lock().unwrap().len(), 1);
        });
    }

    #[test]
    fn discussion_includes_paginated_conversation_reviews_and_inline_threads() {
        smol::block_on(async {
            let (client, transport) = client(vec![
                Ok(json!(vec![comment(); 100])),
                Ok(json!([comment()])),
                Ok(json!([comment()])),
                Ok(json!([comment()])),
            ]);
            let discussion = client.discussion(&repo(), 12).await.unwrap();
            assert_eq!(discussion.len(), 103);
            assert_eq!(
                discussion
                    .iter()
                    .filter(|entry| entry.kind == CommentKind::Conversation)
                    .count(),
                101
            );
            let requests = transport.requests.lock().unwrap();
            assert!(requests[1].endpoint.ends_with("page=2"));
            assert!(requests.iter().all(|request| request.body.is_none()));
        });
    }

    #[test]
    fn pr_review_identity_survives_head_updates_but_isolates_base_and_worktree() {
        let mut checkout = Checkout {
            repository: repo(),
            pull_request: pr(),
            branch: "review/old".into(),
            base_ref: "old-sha".into(),
            warning: None,
        };
        let original = checkout.review_key(Path::new("/worktree")).unwrap();
        checkout.branch = "review/new".into();
        checkout.base_ref = "new-sha".into();
        checkout.pull_request.head.sha = "c".repeat(40);
        assert_eq!(
            original,
            checkout.review_key(Path::new("/worktree")).unwrap()
        );
        assert_ne!(
            original,
            checkout.review_key(Path::new("/another-worktree")).unwrap()
        );
        checkout.pull_request.base.branch = "release".into();
        assert_ne!(
            original,
            checkout.review_key(Path::new("/worktree")).unwrap()
        );
    }

    async fn fixture(
        executor: &gpui::BackgroundExecutor,
    ) -> (
        tempfile::TempDir,
        std::path::PathBuf,
        std::path::PathBuf,
        ReviewRequest,
    ) {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("checkout");
        let remote = directory.path().join("remote.git");
        smol::fs::create_dir(&root).await.unwrap();
        git_output(executor, &root, &["init", "-b", "main"])
            .await
            .unwrap();
        git_output(executor, &root, &["config", "user.name", "Fixture"])
            .await
            .unwrap();
        git_output(
            executor,
            &root,
            &["config", "user.email", "fixture@example.invalid"],
        )
        .await
        .unwrap();
        smol::fs::write(root.join("a.txt"), "base\n").await.unwrap();
        git_output(executor, &root, &["add", "."]).await.unwrap();
        git_output(executor, &root, &["commit", "-m", "base"])
            .await
            .unwrap();
        let mut pr = pr();
        pr.base.sha = git_text(executor, &root, &["rev-parse", "HEAD"])
            .await
            .unwrap();
        git_output(executor, &root, &["switch", "-c", "feature"])
            .await
            .unwrap();
        smol::fs::write(root.join("a.txt"), "changed\n")
            .await
            .unwrap();
        git_output(executor, &root, &["commit", "-am", "feature"])
            .await
            .unwrap();
        pr.head.sha = git_text(executor, &root, &["rev-parse", "HEAD"])
            .await
            .unwrap();
        git_output(
            executor,
            &root,
            &["update-ref", "refs/pull/12/head", &pr.head.sha],
        )
        .await
        .unwrap();
        git_output(
            executor,
            &root,
            &[
                "clone",
                "--bare",
                root.to_str().unwrap(),
                remote.to_str().unwrap(),
            ],
        )
        .await
        .unwrap();
        git_output(
            executor,
            &root,
            &[
                "push",
                remote.to_str().unwrap(),
                "refs/pull/12/head:refs/pull/12/head",
            ],
        )
        .await
        .unwrap();
        git_output(executor, &root, &["switch", "main"])
            .await
            .unwrap();
        (directory, root, remote, pr)
    }

    async fn publish_review_head(
        executor: &gpui::BackgroundExecutor,
        root: &Path,
        remote: &Path,
        start: &str,
        contents: &str,
    ) -> String {
        let current = git_text(
            executor,
            root,
            &["symbolic-ref", "--quiet", "--short", "HEAD"],
        )
        .await
        .unwrap();
        git_output(executor, root, &["switch", "--detach", start])
            .await
            .unwrap();
        smol::fs::write(root.join("a.txt"), contents).await.unwrap();
        git_output(executor, root, &["commit", "-am", "updated review head"])
            .await
            .unwrap();
        let head = git_text(executor, root, &["rev-parse", "HEAD"])
            .await
            .unwrap();
        git_output(
            executor,
            root,
            &[
                "push",
                "--force",
                remote.to_str().unwrap(),
                &format!("{head}:refs/pull/12/head"),
            ],
        )
        .await
        .unwrap();
        git_output(executor, root, &["switch", &current])
            .await
            .unwrap();
        head
    }

    #[gpui::test]
    fn checkout_preserves_branches_and_rejects_dirty_state_and_fetch_races(
        cx: &mut gpui::TestAppContext,
    ) {
        let executor = &cx.background_executor;
        smol::block_on(async {
            let (_directory, root, remote, pr) = fixture(executor).await;
            let original = git_text(executor, &root, &["rev-parse", "HEAD"])
                .await
                .unwrap();
            smol::fs::write(root.join("untracked.txt"), "do not lose")
                .await
                .unwrap();
            assert!(
                checkout_from_remote(
                    executor,
                    root.clone(),
                    repo(),
                    pr.clone(),
                    None,
                    remote.to_str().unwrap().into(),
                    || Ok(())
                )
                .await
                .is_err()
            );
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "HEAD"])
                    .await
                    .unwrap(),
                original
            );
            smol::fs::remove_file(root.join("untracked.txt"))
                .await
                .unwrap();
            let checkout = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(checkout.branch, "feature");
            assert!(checkout.warning.is_none());
            assert_eq!(
                git_text(executor, &root, &["rev-parse", &checkout.base_ref])
                    .await
                    .unwrap(),
                pr.base.sha
            );
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "HEAD"])
                    .await
                    .unwrap(),
                pr.head.sha
            );
            git_output(executor, &root, &["switch", "main"])
                .await
                .unwrap();
            let retained = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(retained.branch, checkout.branch);
            assert_eq!(
                git_text(
                    executor,
                    &root,
                    &["for-each-ref", "--format=%(refname:short)", "refs/heads/"]
                )
                .await
                .unwrap()
                .lines()
                .filter(|branch| branch.starts_with("review/pr-12-"))
                .count(),
                0
            );
            let mut moved = pr.clone();
            moved.head.sha = "c".repeat(40);
            assert!(
                checkout_from_remote(
                    executor,
                    root.clone(),
                    repo(),
                    moved,
                    Some(checkout),
                    remote.to_str().unwrap().into(),
                    || Ok(())
                )
                .await
                .is_err()
            );
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "HEAD"])
                    .await
                    .unwrap(),
                pr.head.sha
            );
            let mut checks = 0;
            assert!(
                checkout_from_remote(
                    executor,
                    root.clone(),
                    repo(),
                    pr.clone(),
                    None,
                    remote.to_str().unwrap().into(),
                    || {
                        checks += 1;
                        ensure!(checks < 2, "buffer became dirty");
                        Ok(())
                    }
                )
                .await
                .is_err()
            );
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "HEAD"])
                    .await
                    .unwrap(),
                pr.head.sha
            );
        });
    }

    #[gpui::test]
    fn gitlab_checkout_fetches_merge_request_ref_and_reuses_source_branch(
        cx: &mut gpui::TestAppContext,
    ) {
        let executor = &cx.background_executor;
        smol::block_on(async {
            let (_directory, root, remote, mut request) = fixture(executor).await;
            git_output(
                executor,
                &root,
                &[
                    "update-ref",
                    "refs/merge-requests/12/head",
                    &request.head.sha,
                ],
            )
            .await
            .unwrap();
            git_output(
                executor,
                &root,
                &[
                    "push",
                    remote.to_str().unwrap(),
                    "refs/merge-requests/12/head:refs/merge-requests/12/head",
                ],
            )
            .await
            .unwrap();
            let repository = ReviewRepository {
                id: 7,
                full_name: "group/subgroup/project".into(),
                provider: ReviewProviderKind::GitLab,
                host: "code.internal.example".into(),
                web_url: Some("https://code.internal.example/group/subgroup/project".into()),
            };
            request.head.repo = Some(repository.clone());
            request.base.repo = Some(repository.clone());
            request.start_sha = Some(request.base.sha.clone());
            request.web_url = Some(
                "https://code.internal.example/group/subgroup/project/-/merge_requests/12".into(),
            );
            let checkout = checkout_from_remote(
                executor,
                root.clone(),
                repository,
                request.clone(),
                None,
                remote.to_string_lossy().into_owned(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(checkout.branch, "feature");
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "HEAD"])
                    .await
                    .unwrap(),
                request.head.sha
            );
            assert!(checkout.base_ref.contains("/gitlab-"));
            let association = git_text(
                executor,
                &root,
                &["config", "--get", &association_key("feature")],
            )
            .await
            .unwrap();
            let association: crate::review_provider::ReviewCheckoutAssociation =
                serde_json::from_str(&association).unwrap();
            assert_eq!(association.identity.provider, "gitlab");
            assert_eq!(association.identity.source_host, "code.internal.example");
        });
    }

    #[gpui::test]
    fn checkout_reuses_managed_and_upstream_source_branches(cx: &mut gpui::TestAppContext) {
        let executor = &cx.background_executor;
        smol::block_on(async {
            let (_directory, root, remote, pr) = fixture(executor).await;
            git_output(executor, &root, &["branch", "-D", "feature"])
                .await
                .unwrap();
            let created = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(created.branch, "feature");
            git_output(executor, &root, &["switch", "main"])
                .await
                .unwrap();
            let reopened = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(reopened.branch, created.branch);

            git_output(executor, &root, &["switch", "main"])
                .await
                .unwrap();
            git_output(
                executor,
                &root,
                &["branch", "-m", "feature", "local-feature"],
            )
            .await
            .unwrap();
            git_output(
                executor,
                &root,
                &[
                    "config",
                    "--unset-all",
                    "branch.local-feature.zed-review-association",
                ],
            )
            .await
            .unwrap();
            git_output(
                executor,
                &root,
                &[
                    "remote",
                    "add",
                    "contributor",
                    "https://github.com/contributor/project.git",
                ],
            )
            .await
            .unwrap();
            git_output(
                executor,
                &root,
                &["config", "branch.local-feature.remote", "contributor"],
            )
            .await
            .unwrap();
            git_output(
                executor,
                &root,
                &["config", "branch.local-feature.merge", "refs/heads/feature"],
            )
            .await
            .unwrap();
            git_output(
                executor,
                &root,
                &[
                    "update-ref",
                    "refs/remotes/contributor/feature",
                    &pr.head.sha,
                ],
            )
            .await
            .unwrap();
            let upstream = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr,
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(upstream.branch, "local-feature");
        });
    }

    #[gpui::test]
    fn checkout_updates_safe_branches_and_preserves_diverged_branches(
        cx: &mut gpui::TestAppContext,
    ) {
        let executor = &cx.background_executor;
        smol::block_on(async {
            let (_directory, root, remote, mut pr) = fixture(executor).await;
            let old_head = pr.head.sha.clone();
            pr.head.sha =
                publish_review_head(executor, &root, &remote, &old_head, "fast forward\n").await;
            let fast_forwarded = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(fast_forwarded.branch, "feature");
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "feature"])
                    .await
                    .unwrap(),
                pr.head.sha
            );

            let (_directory, root, remote, mut pr) = fixture(executor).await;
            let user_head = pr.head.sha.clone();
            pr.head.sha =
                publish_review_head(executor, &root, &remote, &pr.base.sha, "force pushed\n").await;
            let fallback = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(fallback.branch, "review/pr-12-feature");
            assert!(fallback.warning.is_some());
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "feature"])
                    .await
                    .unwrap(),
                user_head
            );

            let (_directory, root, remote, mut pr) = fixture(executor).await;
            git_output(executor, &root, &["branch", "-D", "feature"])
                .await
                .unwrap();
            let managed = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            let old_head = pr.head.sha.clone();
            git_output(executor, &root, &["switch", "main"])
                .await
                .unwrap();
            pr.head.sha = publish_review_head(
                executor,
                &root,
                &remote,
                &pr.base.sha,
                "managed force push\n",
            )
            .await;
            let updated = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(updated.branch, managed.branch);
            assert_ne!(old_head, pr.head.sha);
            assert_eq!(
                git_text(
                    executor,
                    &root,
                    &[
                        "rev-parse",
                        &format!("refs/zed/reviews/7/12/recovery-{old_head}")
                    ],
                )
                .await
                .unwrap(),
                old_head
            );
            assert_eq!(
                git_text(executor, &root, &["rev-parse", &updated.branch])
                    .await
                    .unwrap(),
                pr.head.sha
            );

            let (_directory, root, remote, pr) = fixture(executor).await;
            git_output(executor, &root, &["switch", "feature"])
                .await
                .unwrap();
            smol::fs::write(root.join("a.txt"), "local commit\n")
                .await
                .unwrap();
            git_output(executor, &root, &["commit", "-am", "local work"])
                .await
                .unwrap();
            let local_head = git_text(executor, &root, &["rev-parse", "HEAD"])
                .await
                .unwrap();
            git_output(executor, &root, &["switch", "main"])
                .await
                .unwrap();
            let ahead = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr,
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(ahead.branch, "review/pr-12-feature");
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "feature"])
                    .await
                    .unwrap(),
                local_head
            );
        });
    }

    #[gpui::test]
    fn checkout_isolates_forks_worktrees_and_deleted_sources(cx: &mut gpui::TestAppContext) {
        let executor = &cx.background_executor;
        smol::block_on(async {
            let (_directory, root, remote, pr) = fixture(executor).await;
            let mut other_identity = checkout_identity(&repo(), &pr);
            other_identity.source_repository = "other/project".into();
            write_checkout_association(
                executor,
                &root,
                "feature",
                &other_identity,
                &pr.head.sha,
                true,
            )
            .await
            .unwrap();
            let isolated = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(isolated.branch, "review/pr-12-feature");
            git_output(executor, &root, &["switch", "main"])
                .await
                .unwrap();
            let reopened = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(reopened.branch, isolated.branch);

            let (directory, root, remote, pr) = fixture(executor).await;
            let secondary = directory.path().join("secondary");
            git_output(
                executor,
                &root,
                &["worktree", "add", secondary.to_str().unwrap(), "feature"],
            )
            .await
            .unwrap();
            let worktree_checkout = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_ne!(worktree_checkout.branch, "feature");
            assert!(worktree_checkout.warning.is_some());

            let (_directory, root, remote, pr) = fixture(executor).await;
            let mut deleted = pr;
            deleted.number = 13;
            deleted.head.repo = None;
            git_output(
                executor,
                &root,
                &[
                    "push",
                    remote.to_str().unwrap(),
                    &format!("{}:refs/pull/13/head", deleted.head.sha),
                ],
            )
            .await
            .unwrap();
            let deleted_checkout = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                deleted,
                None,
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert!(deleted_checkout.branch.starts_with("review/pr-13-feature"));
            assert!(deleted_checkout.warning.is_some());
        });
    }

    #[gpui::test]
    fn local_edits_block_inline_posting_and_git_operations_block_checkout(
        cx: &mut gpui::TestAppContext,
    ) {
        let executor = &cx.background_executor;
        smol::block_on(async {
            let (_directory, root, _remote, pr) = fixture(executor).await;
            git_output(executor, &root, &["switch", "feature"])
                .await
                .unwrap();
            let target = CommentTarget::Inline {
                path: "a.txt".into(),
                side: DiffSide::Right,
                start_line: 1,
                line: 1,
                head_sha: pr.head.sha.clone(),
                base_sha: pr.base.sha.clone(),
            };
            validate_inline(
                executor,
                &root,
                &pr,
                &target,
                Some("changed\n"),
                Some("base\n"),
            )
            .await
            .unwrap();
            assert!(
                validate_inline(
                    executor,
                    &root,
                    &pr,
                    &target,
                    Some("changed\n"),
                    Some("wrong base\n")
                )
                .await
                .is_err()
            );
            assert!(
                validate_inline(
                    executor,
                    &root,
                    &pr,
                    &target,
                    Some("unsaved\n"),
                    Some("base\n")
                )
                .await
                .is_err()
            );
            smol::fs::write(root.join("a.txt"), "external edit\n")
                .await
                .unwrap();
            assert!(
                validate_inline(executor, &root, &pr, &target, None, Some("base\n"))
                    .await
                    .is_err()
            );
            assert!(check_clean_checkout(executor, &root).await.is_err());
            git_output(executor, &root, &["restore", "a.txt"])
                .await
                .unwrap();
            smol::fs::write(root.join(".git/MERGE_HEAD"), &pr.base.sha)
                .await
                .unwrap();
            assert!(check_clean_checkout(executor, &root).await.is_err());
        });
    }
}
