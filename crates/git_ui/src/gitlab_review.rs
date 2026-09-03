use crate::{
    github_review::{
        ApiMethod, ApiRequest, CommentKind, CommentTarget, DiffSide, DiscussionAction,
        DiscussionComment, RemoteComment, ReviewProviderFailure, ReviewRepository, ReviewRequest,
        ReviewRequestRef, ReviewRequestSummaryData, ReviewThread, ReviewUser, ThreadComment,
        validate_sha,
    },
    review_provider::{ReviewProviderKind, ReviewRepositoryChoice},
};
use anyhow::{Context as _, Result, bail, ensure};
use futures::{AsyncWriteExt as _, FutureExt as _, future::BoxFuture};
use serde::Deserialize;
use serde_json::{Value, json};
use sha1::{Digest as _, Sha1};
use std::{path::PathBuf, process::Stdio, sync::Arc, time::Duration};

pub(crate) trait GitLabTransport: Send + Sync {
    fn request(&self, request: ApiRequest) -> BoxFuture<'static, Result<Value>>;
}

struct GlabCli {
    executor: gpui::BackgroundExecutor,
    host: String,
    root: PathBuf,
}

impl GitLabTransport for GlabCli {
    fn request(&self, request: ApiRequest) -> BoxFuture<'static, Result<Value>> {
        let executor = self.executor.clone();
        let host = self.host.clone();
        let root = self.root.clone();
        async move {
            ensure!(
                (request.endpoint == "user"
                    || request.endpoint == "version"
                    || request.endpoint.starts_with("projects/"))
                    && !request.endpoint.contains(['\n', '\r', '#']),
                "Invalid GitLab endpoint"
            );
            let writing = request.writing;
            let has_body = request.body.is_some();
            let mut command = smol::process::Command::new("glab");
            command
                .current_dir(root)
                .args([
                    "api",
                    "--hostname",
                    &host,
                    "--method",
                    request.method.as_str(),
                    &request.endpoint,
                ])
                .env("GLAB_NO_PROMPT", "1")
                .env("GLAB_DEBUG", "")
                .kill_on_drop(true)
                .stdin(if has_body { Stdio::piped() } else { Stdio::null() })
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            if has_body {
                command.args(["--input", "-"]);
            }
            let mut child = command.spawn().map_err(|_| ReviewProviderFailure {
                message: format!(
                    "GitLab CLI could not start. Install glab and run glab auth login --hostname {host}."
                ),
                outcome_unknown: false,
            })?;
            let operation = async move {
                if let Some(body) = request.body {
                    let mut stdin = child.stdin.take().context("GitLab CLI input is unavailable")?;
                    stdin
                        .write_all(&serde_json::to_vec(&body)?)
                        .await
                        .map_err(|_| ReviewProviderFailure {
                            message: "GitLab request was interrupted. Refresh before retrying."
                                .into(),
                            outcome_unknown: writing,
                        })?;
                    drop(stdin);
                }
                let output = child.output().await.map_err(|_| ReviewProviderFailure {
                    message: "GitLab request was interrupted. Refresh before retrying.".into(),
                    outcome_unknown: writing,
                })?;
                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    let (message, known) = if error.contains("401")
                        || error.contains("authentication")
                        || error.contains("auth login")
                    {
                        (
                            format!(
                                "GitLab authentication is required. Run glab auth login --hostname {host}."
                            ),
                            true,
                        )
                    } else if error.contains("403") {
                        (
                            "GitLab denied this request. Check project permissions or rate limits."
                                .into(),
                            true,
                        )
                    } else if error.contains("404") {
                        (
                            "The GitLab project or merge request was not found, or this account cannot access it."
                                .into(),
                            true,
                        )
                    } else if error.contains("409") || error.contains("422") {
                        (
                            "GitLab rejected this discussion target. Refresh the merge request and select the lines again."
                                .into(),
                            true,
                        )
                    } else {
                        (
                            "GitLab request failed. Refresh to check its outcome before retrying."
                                .into(),
                            false,
                        )
                    };
                    return Err(ReviewProviderFailure {
                        message,
                        outcome_unknown: writing && !known,
                    }
                    .into());
                }
                if request.method == ApiMethod::Delete && output.stdout.is_empty() {
                    return Ok(Value::Null);
                }
                serde_json::from_slice(&output.stdout).map_err(|_| {
                    ReviewProviderFailure {
                        message: "GitLab returned an unreadable response. Refresh before retrying a write."
                            .into(),
                        outcome_unknown: writing,
                    }
                    .into()
                })
            }
            .boxed();
            match futures::future::select(
                operation,
                executor.timer(Duration::from_secs(45)).boxed(),
            )
            .await
            {
                futures::future::Either::Left((result, _)) => result,
                futures::future::Either::Right(_) => Err(ReviewProviderFailure {
                    message:
                        "GitLab request timed out. Refresh to check whether the operation succeeded."
                            .into(),
                    outcome_unknown: writing,
                }
                .into()),
            }
        }
        .boxed()
    }
}

#[derive(Clone)]
pub(crate) struct GitLabClient {
    transport: Arc<dyn GitLabTransport>,
    choice: ReviewRepositoryChoice,
}

#[derive(Deserialize)]
struct GitLabProject {
    id: u64,
    path_with_namespace: String,
    web_url: String,
}

#[derive(Deserialize)]
struct GitLabMergeRequest {
    iid: u64,
    title: String,
    description: Option<String>,
    state: String,
    merged_at: Option<String>,
    author: GitLabAuthor,
    source_branch: String,
    target_branch: String,
    sha: String,
    source_project_id: Option<u64>,
    target_project_id: u64,
    diff_refs: Option<GitLabDiffRefs>,
    web_url: String,
}

#[derive(Deserialize)]
struct GitLabMergeRequestSummary {
    iid: u64,
    title: String,
}

#[derive(Deserialize)]
struct GitLabDiffRefs {
    base_sha: String,
    head_sha: String,
    start_sha: String,
}

#[derive(Clone, Deserialize)]
struct GitLabAuthor {
    username: String,
}

#[derive(Clone, Deserialize)]
struct GitLabDiscussion {
    id: String,
    #[serde(default)]
    notes: Vec<GitLabNote>,
}

#[derive(Clone, Deserialize)]
struct GitLabNote {
    id: u64,
    body: String,
    author: GitLabAuthor,
    created_at: Option<String>,
    #[serde(default)]
    system: bool,
    #[serde(default)]
    resolvable: bool,
    #[serde(default)]
    resolved: bool,
    position: Option<GitLabPosition>,
}

#[derive(Clone, Deserialize)]
struct GitLabPosition {
    old_path: Option<String>,
    new_path: Option<String>,
    old_line: Option<u32>,
    new_line: Option<u32>,
    head_sha: Option<String>,
}

#[derive(Deserialize)]
struct GitLabDiffPath {
    old_path: String,
    new_path: String,
}

impl GitLabClient {
    pub(crate) fn new(
        choice: ReviewRepositoryChoice,
        root: PathBuf,
        executor: gpui::BackgroundExecutor,
    ) -> Self {
        let transport = Arc::new(GlabCli {
            executor,
            host: choice.host.clone(),
            root,
        });
        Self { transport, choice }
    }

    #[cfg(test)]
    fn with_transport(choice: ReviewRepositoryChoice, transport: Arc<dyn GitLabTransport>) -> Self {
        Self { transport, choice }
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
        .context("Invalid GitLab response")
    }

    fn project_endpoint(&self, suffix: &str) -> String {
        let project = url::form_urlencoded::byte_serialize(self.choice.full_name.as_bytes())
            .collect::<String>();
        if suffix.is_empty() {
            format!("projects/{project}")
        } else {
            format!("projects/{project}/{suffix}")
        }
    }

    pub(crate) async fn repository(&self) -> Result<ReviewRepository> {
        let project: GitLabProject = self.get(self.project_endpoint("")).await?;
        ensure!(
            project.path_with_namespace == self.choice.full_name,
            "GitLab returned a different project"
        );
        Ok(ReviewRepository {
            id: project.id,
            full_name: project.path_with_namespace,
            provider: ReviewProviderKind::GitLab,
            host: self.choice.host.clone(),
            web_url: Some(project.web_url),
        })
    }

    async fn project_by_id(&self, id: u64) -> Result<GitLabProject> {
        self.get(format!("projects/{id}")).await
    }

    async fn convert_merge_request(
        &self,
        project: &ReviewRepository,
        merge_request: GitLabMergeRequest,
    ) -> Result<ReviewRequest> {
        ensure!(
            merge_request.target_project_id == project.id,
            "Merge request belongs to a different GitLab project"
        );
        let refs = merge_request
            .diff_refs
            .context("GitLab is still preparing this merge request diff. Refresh shortly.")?;
        validate_sha(&refs.base_sha)?;
        validate_sha(&refs.start_sha)?;
        validate_sha(&refs.head_sha)?;
        ensure!(
            merge_request.sha == refs.head_sha,
            "GitLab returned inconsistent merge request revisions"
        );
        let source = if merge_request.source_project_id == Some(project.id) {
            None
        } else if let Some(source_project_id) = merge_request.source_project_id {
            Some(self.project_by_id(source_project_id).await?)
        } else {
            None
        };
        Ok(ReviewRequest {
            number: merge_request.iid,
            title: merge_request.title,
            body: merge_request.description,
            user: ReviewUser {
                login: merge_request.author.username,
            },
            state: merge_request.state,
            merged_at: merge_request.merged_at,
            head: ReviewRequestRef {
                branch: merge_request.source_branch,
                sha: refs.head_sha,
                repo: source
                    .map(|source| ReviewRepository {
                        id: source.id,
                        full_name: source.path_with_namespace,
                        provider: ReviewProviderKind::GitLab,
                        host: self.choice.host.clone(),
                        web_url: Some(source.web_url),
                    })
                    .or_else(|| {
                        (merge_request.source_project_id == Some(project.id))
                            .then(|| project.clone())
                    }),
            },
            base: ReviewRequestRef {
                branch: merge_request.target_branch,
                sha: refs.base_sha,
                repo: Some(project.clone()),
            },
            start_sha: Some(refs.start_sha),
            web_url: Some(merge_request.web_url),
        })
    }

    pub(crate) async fn merge_request(
        &self,
        project: &ReviewRepository,
        number: u64,
    ) -> Result<ReviewRequest> {
        let value: GitLabMergeRequest = self
            .get(self.project_endpoint(&format!("merge_requests/{number}")))
            .await?;
        ensure!(
            value.iid == number,
            "GitLab returned a different merge request"
        );
        self.convert_merge_request(project, value).await
    }

    pub(crate) async fn merge_request_summaries(
        &self,
        state: &str,
        page: u32,
        search: Option<&str>,
    ) -> Result<Vec<ReviewRequestSummaryData>> {
        ensure!(
            matches!(state, "open" | "closed" | "all") && page > 0,
            "Invalid MR filter"
        );
        let state = match state {
            "open" => "opened",
            value => value,
        };
        let mut parameters = url::form_urlencoded::Serializer::new(String::new());
        parameters
            .append_pair("state", state)
            .append_pair("order_by", "updated_at")
            .append_pair("sort", "desc")
            .append_pair("per_page", "100")
            .append_pair("page", &page.to_string());
        if let Some(search) = search.filter(|search| !search.trim().is_empty()) {
            parameters.append_pair("search", search.trim());
        }
        let values: Vec<GitLabMergeRequestSummary> = self
            .get(self.project_endpoint(&format!("merge_requests?{}", parameters.finish())))
            .await?;
        Ok(values
            .into_iter()
            .map(|request| ReviewRequestSummaryData {
                number: request.iid,
                title: request.title,
            })
            .collect())
    }

    pub(crate) async fn viewer(&self) -> Result<ReviewUser> {
        #[derive(Deserialize)]
        struct Viewer {
            username: String,
        }
        let viewer: Viewer = self.get("user".into()).await?;
        Ok(ReviewUser {
            login: viewer.username,
        })
    }

    async fn discussions(&self, number: u64) -> Result<Vec<GitLabDiscussion>> {
        let mut all = Vec::new();
        for page in 1..=100 {
            let values: Vec<GitLabDiscussion> = self
                .get(self.project_endpoint(&format!(
                    "merge_requests/{number}/discussions?per_page=100&page={page}"
                )))
                .await?;
            let done = values.len() < 100;
            all.extend(values);
            if done {
                return Ok(all);
            }
        }
        bail!("GitLab returned too many discussion pages")
    }

    fn convert_discussions(
        discussions: Vec<GitLabDiscussion>,
        viewer: &str,
    ) -> Vec<DiscussionComment> {
        let mut result = Vec::new();
        for discussion in discussions {
            let root_id = discussion.notes.first().map(|note| note.id);
            let resolved = discussion.notes.iter().any(|note| note.resolved);
            let resolvable = discussion.notes.iter().any(|note| note.resolvable);
            let comments = discussion
                .notes
                .iter()
                .map(|note| ThreadComment {
                    database_id: note.id,
                    viewer_did_author: note.author.username.eq_ignore_ascii_case(viewer),
                    viewer_can_update: note.author.username.eq_ignore_ascii_case(viewer),
                    viewer_can_delete: note.author.username.eq_ignore_ascii_case(viewer),
                })
                .collect::<Vec<_>>();
            let thread = Arc::new(ReviewThread {
                id: discussion.id,
                is_resolved: resolved,
                is_outdated: discussion.notes.iter().any(|note| {
                    note.position.as_ref().is_some_and(|position| {
                        position.old_line.is_none() && position.new_line.is_none()
                    })
                }),
                viewer_can_resolve: resolvable && !resolved,
                viewer_can_unresolve: resolvable && resolved,
                viewer_can_reply: true,
                comments,
            });
            for note in discussion.notes.into_iter().filter(|note| !note.system) {
                let position = note.position.as_ref();
                let (path, line, original_line, side) = match position {
                    Some(position) if position.new_line.is_some() => (
                        position.new_path.clone(),
                        position.new_line,
                        position.old_line,
                        Some(DiffSide::Right),
                    ),
                    Some(position) => (
                        position.old_path.clone(),
                        position.old_line,
                        position.old_line,
                        Some(DiffSide::Left),
                    ),
                    None => (None, None, None, None),
                };
                result.push(DiscussionComment {
                    kind: if position.is_some() {
                        CommentKind::Inline
                    } else {
                        CommentKind::Conversation
                    },
                    comment: RemoteComment {
                        thread: Some(thread.clone()),
                        id: note.id,
                        body: Some(note.body),
                        user: ReviewUser {
                            login: note.author.username,
                        },
                        created_at: note.created_at,
                        submitted_at: None,
                        path,
                        old_path: position.and_then(|position| position.old_path.clone()),
                        new_path: position.and_then(|position| position.new_path.clone()),
                        line,
                        original_line,
                        start_line: None,
                        side,
                        commit_id: position.and_then(|position| position.head_sha.clone()),
                        original_commit_id: None,
                        in_reply_to_id: root_id.filter(|id| *id != note.id),
                        diff_hunk: None,
                    },
                });
            }
        }
        result
    }

    pub(crate) async fn discussion(&self, number: u64) -> Result<Vec<DiscussionComment>> {
        let viewer = self.viewer().await?;
        Ok(Self::convert_discussions(
            self.discussions(number).await?,
            &viewer.login,
        ))
    }

    pub(crate) async fn review_threads(&self, number: u64) -> Result<Vec<ReviewThread>> {
        let viewer = self.viewer().await?;
        let converted = Self::convert_discussions(self.discussions(number).await?, &viewer.login);
        let mut threads = std::collections::BTreeMap::new();
        for entry in converted {
            if let Some(thread) = entry.comment.thread {
                threads
                    .entry(thread.id.clone())
                    .or_insert_with(|| (*thread).clone());
            }
        }
        Ok(threads.into_values().collect())
    }

    async fn request(
        &self,
        method: ApiMethod,
        endpoint: String,
        body: Option<Value>,
    ) -> Result<Value> {
        self.transport
            .request(ApiRequest {
                method,
                endpoint,
                writing: method != ApiMethod::Get,
                body,
            })
            .await
    }

    async fn find_discussion(&self, number: u64, note_id: u64) -> Result<GitLabDiscussion> {
        self.discussions(number)
            .await?
            .into_iter()
            .find(|discussion| discussion.notes.iter().any(|note| note.id == note_id))
            .context("GitLab discussion no longer exists")
    }

    pub(crate) async fn update_comment(
        &self,
        number: u64,
        id: u64,
        original: &str,
        body: &str,
    ) -> Result<RemoteComment> {
        ensure!(!body.trim().is_empty(), "Write a comment before saving");
        let viewer = self.viewer().await?;
        let discussion = self.find_discussion(number, id).await?;
        let note = discussion
            .notes
            .iter()
            .find(|note| note.id == id)
            .context("GitLab comment no longer exists")?;
        ensure!(
            note.author.username.eq_ignore_ascii_case(&viewer.login),
            "Only your own comments can be edited"
        );
        ensure!(
            note.body == original,
            "This comment changed on GitLab. Your draft is kept; refresh before editing again."
        );
        let value = self
            .request(
                ApiMethod::Put,
                self.project_endpoint(&format!(
                    "merge_requests/{number}/discussions/{}/notes/{id}",
                    discussion.id
                )),
                Some(json!({"body": body})),
            )
            .await?;
        let note: GitLabNote =
            serde_json::from_value(value).map_err(|_| ReviewProviderFailure {
                message: "The edit may have succeeded; refresh before retrying".into(),
                outcome_unknown: true,
            })?;
        Ok(Self::convert_discussions(
            vec![GitLabDiscussion {
                id: discussion.id,
                notes: vec![note],
            }],
            &viewer.login,
        )
        .remove(0)
        .comment)
    }

    pub(crate) async fn discussion_action(
        &self,
        number: u64,
        action: &DiscussionAction,
    ) -> Result<()> {
        match action {
            DiscussionAction::Delete { comment_id, .. } => {
                let viewer = self.viewer().await?;
                let discussion = self.find_discussion(number, *comment_id).await?;
                let note = discussion
                    .notes
                    .iter()
                    .find(|note| note.id == *comment_id)
                    .context("GitLab comment no longer exists")?;
                ensure!(
                    note.author.username.eq_ignore_ascii_case(&viewer.login),
                    "Only your own comments can be deleted"
                );
                self.request(
                    ApiMethod::Delete,
                    self.project_endpoint(&format!(
                        "merge_requests/{number}/discussions/{}/notes/{comment_id}",
                        discussion.id
                    )),
                    None,
                )
                .await?;
            }
            DiscussionAction::Resolve {
                thread_id,
                resolved,
            } => {
                let discussion = self
                    .discussions(number)
                    .await?
                    .into_iter()
                    .find(|discussion| &discussion.id == thread_id)
                    .context("GitLab discussion no longer exists")?;
                ensure!(
                    discussion.notes.iter().any(|note| note.resolvable),
                    "GitLab does not permit this discussion action"
                );
                self.request(
                    ApiMethod::Put,
                    self.project_endpoint(&format!(
                        "merge_requests/{number}/discussions/{thread_id}"
                    )),
                    Some(json!({"resolved": resolved})),
                )
                .await?;
            }
        }
        Ok(())
    }

    pub(crate) async fn validate_target(
        &self,
        project: &ReviewRepository,
        request: &ReviewRequest,
        target: &CommentTarget,
    ) -> Result<()> {
        let current = self.merge_request(project, request.number).await?;
        ensure!(
            current.head.sha == request.head.sha
                && current.base.sha == request.base.sha
                && current.start_sha == request.start_sha,
            "The merge request revision changed. Refresh before posting."
        );
        if let CommentTarget::Inline { path, .. } = target {
            self.diff_paths(request.number, path).await?;
        }
        Ok(())
    }

    async fn diff_paths(&self, number: u64, path: &str) -> Result<(String, String)> {
        for page in 1..=100 {
            let diffs: Vec<GitLabDiffPath> = self
                .get(self.project_endpoint(&format!(
                    "merge_requests/{number}/diffs?per_page=100&page={page}"
                )))
                .await?;
            let done = diffs.len() < 100;
            if let Some(diff) = diffs
                .into_iter()
                .find(|diff| diff.old_path == path || diff.new_path == path)
            {
                return Ok((diff.old_path, diff.new_path));
            }
            if done {
                break;
            }
        }
        bail!("This path is not in GitLab's published merge request diff. Select the target again.")
    }

    pub(crate) async fn post(
        &self,
        project: &ReviewRepository,
        request: &ReviewRequest,
        target: &CommentTarget,
        body: &str,
    ) -> Result<RemoteComment> {
        ensure!(!body.trim().is_empty(), "Write a comment before posting");
        self.validate_target(project, request, target).await?;
        let (endpoint, payload) = match target {
            CommentTarget::Edit { .. } => bail!("Use Save changes to edit a comment"),
            CommentTarget::General => (
                self.project_endpoint(&format!("merge_requests/{}/discussions", request.number)),
                json!({"body": body}),
            ),
            CommentTarget::Reply { comment_id } => {
                let discussion = self.find_discussion(request.number, *comment_id).await?;
                let value = self
                    .request(
                        ApiMethod::Post,
                        self.project_endpoint(&format!(
                            "merge_requests/{}/discussions/{}/notes",
                            request.number, discussion.id
                        )),
                        Some(json!({"body": body})),
                    )
                    .await?;
                let note: GitLabNote = serde_json::from_value(value).map_err(|_| {
                    ReviewProviderFailure {
                        message: "GitLab may have posted the reply but returned an unreadable response; refresh before retrying".into(),
                        outcome_unknown: true,
                    }
                })?;
                let viewer = self.viewer().await?;
                return Self::convert_discussions(
                    vec![GitLabDiscussion {
                        id: discussion.id,
                        notes: vec![note],
                    }],
                    &viewer.login,
                )
                .into_iter()
                .next()
                .map(|entry| entry.comment)
                .context("GitLab returned an empty reply");
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
                    head_sha == &request.head.sha && base_sha == &request.base.sha,
                    "The merge request revision changed. Refresh and select the target again."
                );
                let start_sha = request
                    .start_sha
                    .as_deref()
                    .context("GitLab merge request start revision is unavailable")?;
                let (old_path, new_path) = self.diff_paths(request.number, path).await?;
                let line_code_path = match side {
                    DiffSide::Left => old_path.as_str(),
                    DiffSide::Right => new_path.as_str(),
                };
                let mut position = json!({
                    "position_type": "text",
                    "base_sha": base_sha,
                    "start_sha": start_sha,
                    "head_sha": head_sha,
                    "old_path": old_path,
                    "new_path": new_path,
                });
                let line_type = match side {
                    DiffSide::Left => "old",
                    DiffSide::Right => "new",
                };
                let line_field = match side {
                    DiffSide::Left => "old_line",
                    DiffSide::Right => "new_line",
                };
                position[line_field] = json!(line);
                if start_line != line {
                    let line_code = |line: u32| {
                        let digest = format!("{:x}", Sha1::digest(line_code_path.as_bytes()));
                        match side {
                            DiffSide::Left => format!("{digest}_{line}_0"),
                            DiffSide::Right => format!("{digest}_0_{line}"),
                        }
                    };
                    let mut start = json!({
                        "line_code": line_code(*start_line),
                        "type": line_type,
                    });
                    start[line_field] = json!(start_line);
                    let mut end = json!({
                        "line_code": line_code(*line),
                        "type": line_type,
                    });
                    end[line_field] = json!(line);
                    position["line_range"] = json!({"start": start, "end": end});
                }
                (
                    self.project_endpoint(&format!(
                        "merge_requests/{}/discussions",
                        request.number
                    )),
                    json!({"body": body, "position": position}),
                )
            }
        };
        let value = self
            .request(ApiMethod::Post, endpoint, Some(payload))
            .await?;
        let discussion: GitLabDiscussion = serde_json::from_value(value).map_err(|_| ReviewProviderFailure {
            message: "GitLab may have posted the discussion but returned an unreadable response; refresh before retrying".into(),
            outcome_unknown: true,
        })?;
        let viewer = self.viewer().await?;
        Self::convert_discussions(vec![discussion], &viewer.login)
            .into_iter()
            .last()
            .map(|entry| entry.comment)
            .context("GitLab returned an empty discussion")
    }
}

pub(crate) fn merge_request_number(query: &str, choice: &ReviewRepositoryChoice) -> Result<u64> {
    let number = if let Some(url) = query.strip_prefix(&format!("https://{}/", choice.host)) {
        let prefix = format!("{}/-/merge_requests/", choice.full_name);
        url.strip_prefix(&prefix)
            .context("Choose an MR from the selected GitLab project")?
            .split(['/', '?', '#'])
            .next()
            .unwrap_or_default()
    } else {
        query.trim_start_matches(['!', '#'])
    };
    let number = number
        .parse::<u64>()
        .context("Enter an MR IID or its GitLab URL")?;
    ensure!(number > 0, "MR IIDs start at 1");
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::VecDeque, sync::Mutex};

    struct MockTransport {
        results: Mutex<VecDeque<Result<Value>>>,
        requests: Mutex<Vec<ApiRequest>>,
    }

    impl GitLabTransport for MockTransport {
        fn request(&self, request: ApiRequest) -> BoxFuture<'static, Result<Value>> {
            self.requests.lock().unwrap().push(request);
            let result = self.results.lock().unwrap().pop_front().unwrap();
            async move { result }.boxed()
        }
    }

    fn choice() -> ReviewRepositoryChoice {
        ReviewRepositoryChoice {
            provider: ReviewProviderKind::GitLab,
            host: "code.internal.example".into(),
            full_name: "group/subgroup/project".into(),
            remote_url: "git@code.internal.example:group/subgroup/project.git".into(),
        }
    }

    fn merge_request() -> Value {
        json!({
            "iid": 42,
            "title": "Review this",
            "description": "Body",
            "state": "opened",
            "merged_at": null,
            "author": {"username": "author"},
            "source_branch": "feature",
            "target_branch": "main",
            "sha": "a".repeat(40),
            "source_project_id": 7,
            "target_project_id": 7,
            "diff_refs": {
                "base_sha": "b".repeat(40),
                "start_sha": "c".repeat(40),
                "head_sha": "a".repeat(40)
            },
            "web_url": "https://code.internal.example/group/subgroup/project/-/merge_requests/42"
        })
    }

    fn repository() -> ReviewRepository {
        ReviewRepository {
            id: 7,
            full_name: "group/subgroup/project".into(),
            provider: ReviewProviderKind::GitLab,
            host: "code.internal.example".into(),
            web_url: Some("https://code.internal.example/group/subgroup/project".into()),
        }
    }

    fn client(results: Vec<Result<Value>>) -> (GitLabClient, Arc<MockTransport>) {
        let transport = Arc::new(MockTransport {
            results: Mutex::new(results.into()),
            requests: Mutex::new(Vec::new()),
        });
        (
            GitLabClient::with_transport(choice(), transport.clone()),
            transport,
        )
    }

    #[test]
    fn parses_self_managed_merge_request_urls() {
        let choice = ReviewRepositoryChoice {
            provider: ReviewProviderKind::GitLab,
            host: "code.internal.example".into(),
            full_name: "group/subgroup/project".into(),
            remote_url: "git@code.internal.example:group/subgroup/project.git".into(),
        };
        assert_eq!(
            merge_request_number(
                "https://code.internal.example/group/subgroup/project/-/merge_requests/42",
                &choice,
            )
            .unwrap(),
            42
        );
        assert_eq!(merge_request_number("!7", &choice).unwrap(), 7);
    }

    #[test]
    fn lists_nested_self_managed_projects_without_hydrating_every_merge_request() {
        smol::block_on(async {
            let (client, transport) = client(vec![
                Ok(json!({
                    "id": 7,
                    "path_with_namespace": "group/subgroup/project",
                    "web_url": "https://code.internal.example/group/subgroup/project"
                })),
                Ok(json!([
                    {"iid": 42, "title": "First"},
                    {"iid": 41, "title": "Second"}
                ])),
            ]);
            let project = client.repository().await.unwrap();
            let requests = client
                .merge_request_summaries("open", 1, Some("First"))
                .await
                .unwrap();
            assert_eq!(project, repository());
            assert_eq!(requests[0].number, 42);
            let requests = transport.requests.lock().unwrap();
            assert_eq!(requests[0].endpoint, "projects/group%2Fsubgroup%2Fproject");
            assert!(requests[1].endpoint.contains("state=opened"));
            assert!(requests[1].endpoint.contains("search=First"));
            assert!(requests.iter().all(|request| !request.writing));
        });
    }

    #[test]
    fn posts_multiline_right_side_discussion_with_exact_gitlab_diff_revision() {
        smol::block_on(async {
            let posted = json!({
                "id": "discussion-1",
                "notes": [{
                    "id": 99,
                    "body": "Please revise",
                    "author": {"username": "reviewer"},
                    "created_at": "2026-09-02T00:00:00Z",
                    "system": false,
                    "resolvable": true,
                    "resolved": false,
                    "position": {
                        "old_path": "old/name.rs",
                        "new_path": "src/name.rs",
                        "new_line": 12,
                        "head_sha": "a".repeat(40)
                    }
                }]
            });
            let (client, transport) = client(vec![
                Ok(merge_request()),
                Ok(json!([{"old_path": "old/name.rs", "new_path": "src/name.rs"}])),
                Ok(json!([{"old_path": "old/name.rs", "new_path": "src/name.rs"}])),
                Ok(posted),
                Ok(json!({"username": "reviewer"})),
            ]);
            let request = client
                .convert_merge_request(
                    &repository(),
                    serde_json::from_value(merge_request()).unwrap(),
                )
                .await
                .unwrap();
            let comment = client
                .post(
                    &repository(),
                    &request,
                    &CommentTarget::Inline {
                        path: "src/name.rs".into(),
                        side: DiffSide::Right,
                        start_line: 10,
                        line: 12,
                        head_sha: "a".repeat(40),
                        base_sha: "b".repeat(40),
                    },
                    "Please revise",
                )
                .await
                .unwrap();
            assert_eq!(comment.id, 99);
            let requests = transport.requests.lock().unwrap();
            let post = requests
                .iter()
                .find(|request| request.method == ApiMethod::Post)
                .unwrap();
            let position = &post.body.as_ref().unwrap()["position"];
            assert_eq!(position["old_path"], "old/name.rs");
            assert_eq!(position["new_path"], "src/name.rs");
            assert_eq!(position["start_sha"], "c".repeat(40));
            assert_eq!(position["line_range"]["start"]["new_line"], 10);
            assert_eq!(position["line_range"]["end"]["new_line"], 12);
            assert!(
                position["line_range"]["start"]["line_code"]
                    .as_str()
                    .unwrap()
                    .ends_with("_0_10")
            );
        });
    }

    #[test]
    fn posts_multiline_left_side_discussion_using_the_old_path_line_code() {
        smol::block_on(async {
            let posted = json!({
                "id": "discussion-1",
                "notes": [{
                    "id": 99,
                    "body": "Please restore this",
                    "author": {"username": "reviewer"},
                    "created_at": "2026-09-02T00:00:00Z",
                    "system": false,
                    "resolvable": true,
                    "resolved": false,
                    "position": {
                        "old_path": "old/name.rs",
                        "new_path": "src/name.rs",
                        "old_line": 12,
                        "head_sha": "a".repeat(40)
                    }
                }]
            });
            let (client, transport) = client(vec![
                Ok(merge_request()),
                Ok(json!([{"old_path": "old/name.rs", "new_path": "src/name.rs"}])),
                Ok(json!([{"old_path": "old/name.rs", "new_path": "src/name.rs"}])),
                Ok(posted),
                Ok(json!({"username": "reviewer"})),
            ]);
            let request = client
                .convert_merge_request(
                    &repository(),
                    serde_json::from_value(merge_request()).unwrap(),
                )
                .await
                .unwrap();
            client
                .post(
                    &repository(),
                    &request,
                    &CommentTarget::Inline {
                        path: "src/name.rs".into(),
                        side: DiffSide::Left,
                        start_line: 10,
                        line: 12,
                        head_sha: "a".repeat(40),
                        base_sha: "b".repeat(40),
                    },
                    "Please restore this",
                )
                .await
                .unwrap();

            let requests = transport.requests.lock().unwrap();
            let post = requests
                .iter()
                .find(|request| request.method == ApiMethod::Post)
                .unwrap();
            let position = &post.body.as_ref().unwrap()["position"];
            let old_path_digest = format!("{:x}", Sha1::digest(b"old/name.rs"));
            assert_eq!(position["old_line"], 12);
            assert_eq!(position["line_range"]["start"]["old_line"], 10);
            assert_eq!(
                position["line_range"]["start"]["line_code"],
                format!("{old_path_digest}_10_0")
            );
        });
    }

    #[test]
    fn mutates_discussion_notes_through_their_parent_discussion() {
        smol::block_on(async {
            let discussion = json!([{
                "id": "discussion-1",
                "notes": [{
                    "id": 99,
                    "body": "Original",
                    "author": {"username": "reviewer"},
                    "created_at": "2026-09-02T00:00:00Z",
                    "system": false,
                    "resolvable": true,
                    "resolved": false,
                    "position": null
                }]
            }]);
            let edited_note = json!({
                "id": 99,
                "body": "Edited",
                "author": {"username": "reviewer"},
                "created_at": "2026-09-02T00:00:00Z",
                "system": false,
                "resolvable": true,
                "resolved": false,
                "position": null
            });
            let (client, transport) = client(vec![
                Ok(json!({"username": "reviewer"})),
                Ok(discussion),
                Ok(edited_note),
            ]);
            let comment = client
                .update_comment(42, 99, "Original", "Edited")
                .await
                .unwrap();
            assert_eq!(comment.body.as_deref(), Some("Edited"));
            let requests = transport.requests.lock().unwrap();
            let update = requests
                .iter()
                .find(|request| request.method == ApiMethod::Put)
                .unwrap();
            assert_eq!(
                update.endpoint,
                "projects/group%2Fsubgroup%2Fproject/merge_requests/42/discussions/discussion-1/notes/99"
            );
        });
    }
}
