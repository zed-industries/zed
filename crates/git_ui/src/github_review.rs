use anyhow::{Context as _, Result, bail, ensure};
use futures::{AsyncWriteExt as _, FutureExt as _, future::BoxFuture};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{path::Path, process::Stdio, sync::Arc, time::Duration};
use util::ResultExt as _;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GitHubRepo {
    pub id: u64,
    pub full_name: String,
}

impl GitHubRepo {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.id != 0,
            "GitHub returned an invalid repository identity"
        );
        validate_repository_name(&self.full_name)
    }
    pub fn endpoint(&self, suffix: &str) -> String {
        format!("repos/{}/{}", self.full_name, suffix)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GitHubUser {
    pub login: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PullRequestRef {
    #[serde(rename = "ref")]
    pub branch: String,
    pub sha: String,
    pub repo: Option<GitHubRepo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PullRequest {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub user: GitHubUser,
    pub state: String,
    pub merged_at: Option<String>,
    pub head: PullRequestRef,
    pub base: PullRequestRef,
}

impl PullRequest {
    pub fn validate(&self, repo: &GitHubRepo) -> Result<()> {
        repo.validate()?;
        ensure!(
            self.number > 0
                && self
                    .base
                    .repo
                    .as_ref()
                    .is_some_and(|base| base.id == repo.id),
            "PR belongs to a different repository"
        );
        validate_sha(&self.head.sha)?;
        validate_sha(&self.base.sha)?;
        Ok(())
    }
    pub fn url(&self, repo: &GitHubRepo) -> String {
        format!("https://github.com/{}/pull/{}", repo.full_name, self.number)
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
pub(crate) struct PullRequestSummary {
    pub number: u64,
    pub title: String,
}
impl From<PullRequest> for PullRequestSummary {
    fn from(pr: PullRequest) -> Self {
        Self {
            number: pr.number,
            title: pr.title,
        }
    }
}

#[derive(Deserialize)]
struct PullRequestSearch {
    total_count: u32,
    incomplete_results: bool,
    items: Vec<PullRequestSummary>,
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
    pub id: u64,
    pub body: Option<String>,
    pub user: GitHubUser,
    pub created_at: Option<String>,
    pub submitted_at: Option<String>,
    pub path: Option<String>,
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

#[derive(Clone, Debug)]
pub(crate) struct ApiRequest {
    pub endpoint: String,
    pub body: Option<Value>,
}

pub(crate) trait GitHubTransport: Send + Sync {
    fn request(&self, request: ApiRequest) -> BoxFuture<'static, Result<Value>>;
}

#[derive(Debug)]
pub(crate) struct GitHubFailure {
    pub message: String,
    pub outcome_unknown: bool,
}
impl std::fmt::Display for GitHubFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}
impl std::error::Error for GitHubFailure {}

pub(crate) struct GhCli {
    executor: gpui::BackgroundExecutor,
}
impl GitHubTransport for GhCli {
    fn request(&self, request: ApiRequest) -> BoxFuture<'static, Result<Value>> {
        let executor = self.executor.clone();
        async move {
            ensure!((request.endpoint.starts_with("repos/") || (request.body.is_none() && request.endpoint.starts_with("search/issues?"))) && !request.endpoint.contains(['\n', '\r', '#']), "Invalid GitHub endpoint");
            let writing = request.body.is_some();
            let mut command = smol::process::Command::new("gh");
            command.args(["api", "--hostname", "github.com", "--method", if writing { "POST" } else { "GET" },
                "--header", "Accept: application/vnd.github+json", "--header", "X-GitHub-Api-Version: 2026-03-10", &request.endpoint])
                .env("GH_PROMPT_DISABLED", "1").env("GH_DEBUG", "").kill_on_drop(true)
                .stdin(if writing { Stdio::piped() } else { Stdio::null() }).stdout(Stdio::piped()).stderr(Stdio::piped());
            if writing { command.args(["--input", "-"]); }
            let mut child = command.spawn().map_err(|_| GitHubFailure { message: "GitHub CLI could not start. Install gh and run gh auth login.".into(), outcome_unknown: false })?;
            let operation = async move {
                if let Some(body) = request.body {
                    let mut stdin = child.stdin.take().context("GitHub CLI input is unavailable")?;
                    stdin.write_all(&serde_json::to_vec(&body)?).await.map_err(|_| GitHubFailure { message: "GitHub request was interrupted. Refresh before retrying.".into(), outcome_unknown: writing })?;
                    drop(stdin);
                }
                let output = child.output().await.map_err(|_| GitHubFailure { message: "GitHub request was interrupted. Refresh before retrying.".into(), outcome_unknown: writing })?;
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
                    return Err(GitHubFailure { message: message.into(), outcome_unknown: writing && !known }.into());
                }
                serde_json::from_slice(&output.stdout).map_err(|_| GitHubFailure { message: "GitHub returned an unreadable response. Refresh before retrying a post.".into(), outcome_unknown: writing }.into())
            }.boxed();
            match futures::future::select(operation, executor.timer(Duration::from_secs(45)).boxed()).await {
                futures::future::Either::Left((result, _)) => result,
                futures::future::Either::Right(_) => Err(GitHubFailure { message: "GitHub request timed out. Refresh to check whether a comment was posted.".into(), outcome_unknown: writing }.into()),
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

impl GitHubClient {
    pub async fn repository(&self, full_name: &str) -> Result<GitHubRepo> {
        validate_repository_name(full_name)?;
        let repo: GitHubRepo = self.get(format!("repos/{full_name}")).await?;
        repo.validate()?;
        Ok(repo)
    }
    async fn get<T: serde::de::DeserializeOwned>(&self, endpoint: String) -> Result<T> {
        serde_json::from_value(
            self.transport
                .request(ApiRequest {
                    endpoint,
                    body: None,
                })
                .await?,
        )
        .context("Invalid GitHub response")
    }
    pub async fn pull_requests(
        &self,
        repo: &GitHubRepo,
        state: &str,
        page: u32,
    ) -> Result<Vec<PullRequest>> {
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
        repo: &GitHubRepo,
        title: &str,
        state: &str,
        page: u32,
    ) -> Result<(Vec<PullRequestSummary>, bool)> {
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
        let result: PullRequestSearch = self.get(format!("search/issues?{parameters}")).await?;
        ensure!(
            !result.incomplete_results,
            "GitHub returned an incomplete search. Narrow the title and try again."
        );
        let next = page < 10 && page.saturating_mul(100) < result.total_count;
        Ok((result.items, next))
    }

    pub async fn pull_request(&self, repo: &GitHubRepo, number: u64) -> Result<PullRequest> {
        let pr: PullRequest = self.get(repo.endpoint(&format!("pulls/{number}"))).await?;
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
        repo: &GitHubRepo,
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
        repo: &GitHubRepo,
        pr: &PullRequest,
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
        repo: &GitHubRepo,
        pr: &PullRequest,
        target: &CommentTarget,
        body: &str,
    ) -> Result<RemoteComment> {
        pr.validate(repo)?;
        ensure!(!body.trim().is_empty(), "Write a comment before posting");
        let (suffix, body) = match target {
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
        serde_json::from_value(self.transport.request(ApiRequest { endpoint: repo.endpoint(&suffix), body: Some(body) }).await?).map_err(|_| GitHubFailure { message: "GitHub posted the comment but returned an unreadable response; refresh before retrying".into(), outcome_unknown: true }.into())
    }
}

pub(crate) fn pr_number(query: &str, repo: &GitHubRepo) -> Result<u64> {
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
    pub repository: GitHubRepo,
    pub pull_request: PullRequest,
    pub branch: String,
    pub base_ref: String,
}

impl Checkout {
    pub fn review_key(&self, worktree: &Path) -> Result<String> {
        Ok(format!(
            "github_review_v1:{}",
            crate::review_state::digest(&[&serde_json::to_vec(&(
                worktree,
                self.repository.id,
                self.pull_request.number,
                &self.pull_request.base.branch
            ))?])
        ))
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
        "Commit or move local changes, including untracked files, before opening or updating a PR. Nothing was stashed or discarded."
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
            Ok(_) => bail!("Finish the active Git operation before changing the PR checkout"),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

pub(crate) async fn checkout_pull_request(
    executor: &gpui::BackgroundExecutor,
    root: std::path::PathBuf,
    repo: GitHubRepo,
    pr: PullRequest,
    previous: Option<Checkout>,
    ready: impl FnMut() -> Result<()>,
) -> Result<Checkout> {
    let url = format!("https://github.com/{}.git", repo.full_name);
    checkout_from_remote(executor, root, repo, pr, previous, url, ready).await
}

async fn checkout_from_remote(
    executor: &gpui::BackgroundExecutor,
    root: std::path::PathBuf,
    repo: GitHubRepo,
    pr: PullRequest,
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
    let prefix = format!("refs/zed/reviews/{}/{}", repo.id, pr.number);
    let head_ref = format!("{prefix}/head-{}", pr.head.sha);
    let base_ref = format!("{prefix}/base-{}", pr.base.sha);
    git_output(
        executor,
        &root,
        &[
            "-c",
            "credential.helper=",
            "-c",
            "credential.https://github.com.helper=!gh auth git-credential",
            "fetch",
            "--no-tags",
            "--no-write-fetch-head",
            &url,
            &format!("refs/pull/{}/head:{head_ref}", pr.number),
            &format!("refs/heads/{}:{base_ref}", pr.base.branch),
        ],
    )
    .await?;
    ensure!(
        git_text(executor, &root, &["rev-parse", &head_ref]).await? == pr.head.sha
            && git_text(executor, &root, &["rev-parse", &base_ref]).await? == pr.base.sha,
        "The PR changed during fetch. Refresh its metadata before opening it."
    );
    let current = git_text(
        executor,
        &root,
        &["symbolic-ref", "--quiet", "--short", "HEAD"],
    )
    .await?;
    let head = git_text(executor, &root, &["rev-parse", "HEAD"]).await?;
    let previously_associated = previous.as_ref().is_some_and(|saved| {
        saved.repository.id == repo.id
            && saved.pull_request.number == pr.number
            && saved.branch == current
    });
    let configured_remote = git_text(
        executor,
        &root,
        &["config", "--get", &format!("branch.{current}.remote")],
    )
    .await
    .ok();
    let configured_merge = git_text(
        executor,
        &root,
        &["config", "--get", &format!("branch.{current}.merge")],
    )
    .await
    .ok();
    let remote_url = if let Some(remote) = configured_remote {
        git_text(executor, &root, &["remote", "get-url", &remote])
            .await
            .ok()
            .and_then(|url| repository_from_remote(&url))
    } else {
        None
    };
    let associated = previously_associated
        || (configured_merge.as_deref() == Some(&format!("refs/heads/{}", pr.head.branch))
            && pr
                .head
                .repo
                .as_ref()
                .is_some_and(|repo| remote_url.as_deref() == Some(&repo.full_name)));
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
    let branch = if associated && head == pr.head.sha {
        current
    } else {
        let existing = git_text(
            executor,
            &root,
            &["for-each-ref", "--format=%(refname:short)", "refs/heads/"],
        )
        .await?;
        let name = format!("review/pr-{}-{}", pr.number, &pr.head.sha[..8]);
        let mut branch = name.clone();
        let mut suffix = 2;
        while existing.lines().any(|existing| existing == branch) {
            branch = format!("{name}-{suffix}");
            suffix += 1;
        }
        ready()?;
        check_clean_checkout(executor, &root).await?;
        ready()?;
        // A fresh branch preserves local commits and prior PR revisions, including force pushes.
        git_output(
            executor,
            &root,
            &["switch", "--no-guess", "-c", &branch, &head_ref],
        )
        .await?;
        branch
    };
    Ok(Checkout {
        repository: repo,
        pull_request: pr,
        branch,
        base_ref,
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CommentDraft {
    pub target: CommentTarget,
    pub body: String,
    pub outcome_unknown: bool,
}

pub(crate) async fn validate_inline(
    executor: &gpui::BackgroundExecutor,
    root: &Path,
    pr: &PullRequest,
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
        "The PR revision changed. Keep this draft and select a new target."
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
        "The checkout no longer matches this PR revision"
    );
    let merge_base = git_text(executor, root, &["merge-base", &pr.base.sha, &pr.head.sha]).await?;
    let published_base = git_blob(executor, root, &merge_base, path).await?;
    ensure!(
        published_base.as_deref() == effective_base.map(str::as_bytes),
        "The editor base no longer matches the published PR comparison. Your draft is kept."
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
        "The selected lines are not in the published PR diff. Your draft is kept."
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
                let current = git_blob(executor, root, &pr.head.sha, path)
                    .await?
                    .map(String::from_utf8)
                    .transpose()
                    .context("Inline comments require UTF-8 text")?;
                let base = git_blob(executor, root, &merge_base, path)
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
            });
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::VecDeque, sync::Mutex};

    fn repo() -> GitHubRepo {
        GitHubRepo {
            id: 7,
            full_name: "owner/project".into(),
        }
    }
    fn pr() -> PullRequest {
        PullRequest {
            number: 12,
            title: "Review changes".into(),
            body: None,
            user: GitHubUser {
                login: "author".into(),
            },
            state: "open".into(),
            merged_at: None,
            head: PullRequestRef {
                branch: "feature".into(),
                sha: "a".repeat(40),
                repo: Some(GitHubRepo {
                    id: 8,
                    full_name: "contributor/project".into(),
                }),
            },
            base: PullRequestRef {
                branch: "main".into(),
                sha: "b".repeat(40),
                repo: Some(repo()),
            },
        }
    }
    fn comment() -> Value {
        json!({"id":1,"body":"Comment","user":{"login":"reviewer"}})
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
                    .downcast_ref::<GitHubFailure>()
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
        PullRequest,
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
            let collision = format!("review/pr-12-{}", &pr.head.sha[..8]);
            git_output(executor, &root, &["branch", &collision])
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
            assert_eq!(checkout.branch, format!("{collision}-2"));
            assert_eq!(
                git_text(executor, &root, &["rev-parse", &collision])
                    .await
                    .unwrap(),
                original
            );
            assert_eq!(
                git_text(executor, &root, &["rev-parse", "HEAD"])
                    .await
                    .unwrap(),
                pr.head.sha
            );
            let retained = checkout_from_remote(
                executor,
                root.clone(),
                repo(),
                pr.clone(),
                Some(checkout.clone()),
                remote.to_str().unwrap().into(),
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(retained.branch, checkout.branch);
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
