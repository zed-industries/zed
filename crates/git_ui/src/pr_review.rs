//! GitHub pull request review, driven by the `gh` CLI.
//!
//! A PR is reviewed in a "review copy": a worktree under `~/.cache/pr-reviews`
//! whose HEAD sits at the merge base, so every change in the PR shows up in the
//! git panel. Line comments are kept as pending notes in a Markdown file and
//! posted together as one review. The folder, ref and notes layout matches the
//! `review` shell script in the Zed config, so both can be used side by side.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, anyhow};
use editor::Editor;
use gpui::{App, SharedString, WeakEntity, actions};
use language::Point;
use serde::{Deserialize, Serialize};
use ui::prelude::*;
use util::ResultExt as _;
use workspace::{
    OpenMode, OpenOptions, Toast, Workspace,
    notifications::{NotificationId, NotifyTaskExt as _},
};

use crate::picker_prompt;

mod inline_comments;
mod overview;

actions!(
    pr_review,
    [
        /// Picks an open pull request and opens it for review in a new window.
        StartReview,
        /// Opens the overview of the pull request this window is reviewing.
        OpenOverview,
        /// Adds a pending review comment on the line under the cursor.
        AddComment,
        /// Posts your pending comments as one review: approve, request changes or comment.
        SubmitReview,
        /// Opens the pull request this window is reviewing on GitHub.
        OpenOnGitHub,
    ]
);

const REVIEWS_DIR_ENV: &str = "PR_REVIEWS_DIR";
const SUMMARY_HEADING: &str = "## Summary";

pub(crate) fn reviews_dir() -> PathBuf {
    std::env::var_os(REVIEWS_DIR_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(default_reviews_dir)
}

fn default_reviews_dir() -> PathBuf {
    util::paths::home_dir().join(".cache").join("pr-reviews")
}

pub(crate) fn notes_dir() -> PathBuf {
    reviews_dir().join(".notes")
}

/// A worktree created to review one pull request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReviewCopy {
    pub dir: PathBuf,
    pub name: String,
    pub pr_number: u32,
}

impl ReviewCopy {
    pub fn new(repo_name: &str, pr_number: u32) -> Self {
        let name = format!("{repo_name}-pr-{pr_number}");
        Self {
            dir: reviews_dir().join(&name),
            name,
            pr_number,
        }
    }

    /// The review copy a worktree root belongs to, if it is one.
    pub fn for_worktree_root(root: &Path) -> Option<Self> {
        Self::for_worktree_root_in(root, &reviews_dir())
            .or_else(|| Self::for_worktree_root_in(root, &default_reviews_dir()))
    }

    fn for_worktree_root_in(root: &Path, reviews_dir: &Path) -> Option<Self> {
        if root.parent()? != reviews_dir {
            return None;
        }
        let name = root.file_name()?.to_str()?.to_string();
        let (_, number) = name.rsplit_once("-pr-")?;
        let pr_number = number.parse().ok()?;
        Some(Self {
            dir: root.to_path_buf(),
            name,
            pr_number,
        })
    }

    pub fn branch(&self) -> String {
        format!("review/pr-{}", self.pr_number)
    }

    /// The ref that remembers which PR commit this copy was loaded from.
    pub fn head_ref(&self) -> String {
        format!("refs/pr-review/pr-{}", self.pr_number)
    }

    pub fn notes_path(&self) -> PathBuf {
        notes_dir().join(format!("{}.review.md", self.name))
    }

    pub fn overview_cache_path(&self) -> PathBuf {
        notes_dir().join(format!("{}.overview.json", self.name))
    }

    pub fn submitted_notes_path(&self) -> PathBuf {
        notes_dir().join(format!("{}.submitted.md", self.name))
    }
}

/// A line comment waiting in the notes file to be posted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PendingComment {
    pub path: String,
    pub line: u32,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ReviewNotes {
    pub body: String,
    pub comments: Vec<PendingComment>,
}

impl ReviewNotes {
    /// Text under "## Summary" is the review body; each "### path:line"
    /// heading starts a line comment. Comments with no text are dropped.
    pub fn parse(notes: &str) -> Self {
        let mut body = Vec::new();
        let mut comments: Vec<(String, u32, Vec<&str>)> = Vec::new();
        for row in notes.lines() {
            if let Some((path, line)) = parse_comment_heading(row) {
                comments.push((path, line, Vec::new()));
            } else if let Some((_, _, text)) = comments.last_mut() {
                text.push(row);
            } else if row.trim() != SUMMARY_HEADING {
                body.push(row);
            }
        }
        Self {
            body: body.join("\n").trim().to_string(),
            comments: comments
                .into_iter()
                .map(|(path, line, text)| PendingComment {
                    path,
                    line,
                    text: text.join("\n").trim().to_string(),
                })
                .filter(|comment| !comment.text.is_empty())
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.body.is_empty() && self.comments.is_empty()
    }
}

fn parse_comment_heading(row: &str) -> Option<(String, u32)> {
    let heading = row.strip_prefix("###")?.trim();
    let (path, line) = heading.rsplit_once(':')?;
    let line = line.parse().ok()?;
    let path = path.trim();
    if path.is_empty() {
        return None;
    }
    Some((path.to_string(), line))
}

/// Adds a line comment to the notes file text, creating the template if needed.
pub(crate) fn append_pending_comment(
    existing: Option<&str>,
    path: &str,
    line: u32,
    text: &str,
) -> String {
    let mut notes = match existing {
        Some(existing) if !existing.trim().is_empty() => existing.to_string(),
        _ => format!("{SUMMARY_HEADING}\n\n"),
    };
    if !notes.ends_with('\n') {
        notes.push('\n');
    }
    notes.push_str(&format!("\n### {path}:{line}\n\n{}\n", text.trim()));
    notes
}

/// Removes the first pending comment on `path:line` with this text.
pub(crate) fn remove_pending_comment(existing: &str, path: &str, line: u32, text: &str) -> String {
    let mut notes = ReviewNotes::parse(existing);
    if let Some(index) = notes
        .comments
        .iter()
        .position(|comment| comment.path == path && comment.line == line && comment.text == text)
    {
        notes.comments.remove(index);
    }
    let mut rendered = format!("{SUMMARY_HEADING}\n\n");
    if !notes.body.is_empty() {
        rendered.push_str(&notes.body);
        rendered.push('\n');
    }
    for comment in &notes.comments {
        rendered.push_str(&format!(
            "\n### {}:{}\n\n{}\n",
            comment.path, comment.line, comment.text
        ));
    }
    rendered
}

/// Replaces the review body under "## Summary", keeping the line comments.
pub(crate) fn set_review_body(existing: Option<&str>, body: &str) -> String {
    let existing = existing.unwrap_or_default();
    let comments_start = existing
        .lines()
        .scan(0, |offset, row| {
            let start = *offset;
            *offset += row.len() + 1;
            Some((start, row))
        })
        .find(|(_, row)| parse_comment_heading(row).is_some())
        .map(|(start, _)| start)
        .unwrap_or(existing.len());
    let mut notes = format!("{SUMMARY_HEADING}\n\n");
    if !body.trim().is_empty() {
        notes.push_str(body.trim());
        notes.push('\n');
    }
    let comments = &existing[comments_start.min(existing.len())..];
    if !comments.is_empty() {
        notes.push('\n');
        notes.push_str(comments);
    }
    notes
}

/// The new-side line numbers inside each file's diff hunks. GitHub only
/// accepts line comments on these lines.
pub(crate) fn commentable_lines(diff: &str) -> HashMap<String, BTreeSet<u32>> {
    let mut lines: HashMap<String, BTreeSet<u32>> = HashMap::new();
    let mut path: Option<String> = None;
    for row in diff.lines() {
        if let Some(new_path) = row.strip_prefix("+++ ") {
            path = new_path.strip_prefix("b/").map(str::to_string);
        } else if row.starts_with("@@")
            && let Some(path) = &path
            && let Some((start, count)) = parse_hunk_new_range(row)
        {
            lines
                .entry(path.clone())
                .or_default()
                .extend(start..start + count);
        }
    }
    lines
}

fn parse_hunk_new_range(row: &str) -> Option<(u32, u32)> {
    let new_range = row
        .split_whitespace()
        .find(|part| part.starts_with('+'))?
        .trim_start_matches('+');
    let (start, count) = match new_range.split_once(',') {
        Some((start, count)) => (start.parse().ok()?, count.parse().ok()?),
        None => (new_range.parse().ok()?, 1),
    };
    Some((start, count))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReviewEvent {
    Approve,
    RequestChanges,
    Comment,
}

impl ReviewEvent {
    pub fn api_name(self) -> &'static str {
        match self {
            ReviewEvent::Approve => "APPROVE",
            ReviewEvent::RequestChanges => "REQUEST_CHANGES",
            ReviewEvent::Comment => "COMMENT",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ReviewEvent::Approve => "Approve",
            ReviewEvent::RequestChanges => "Request Changes",
            ReviewEvent::Comment => "Comment",
        }
    }
}

/// The JSON body for `POST /repos/{owner}/{repo}/pulls/{number}/reviews`.
/// Comments on lines outside the diff are moved into the review body, since
/// GitHub rejects them.
pub(crate) fn review_payload(
    notes: &ReviewNotes,
    commentable: &HashMap<String, BTreeSet<u32>>,
    commit_id: &str,
    event: ReviewEvent,
) -> serde_json::Value {
    let (inline, outside): (Vec<_>, Vec<_>) = notes.comments.iter().partition(|comment| {
        commentable
            .get(&comment.path)
            .is_some_and(|lines| lines.contains(&comment.line))
    });

    let mut body = notes.body.clone();
    if !outside.is_empty() {
        let moved = outside
            .iter()
            .map(|comment| {
                let text = comment
                    .text
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("- `{}:{}`: {text}", comment.path, comment.line)
            })
            .collect::<Vec<_>>()
            .join("\n");
        body = format!("{body}\n\nComments on lines outside the diff:\n{moved}")
            .trim()
            .to_string();
    }

    serde_json::json!({
        "commit_id": commit_id,
        "body": body,
        "event": event.api_name(),
        "comments": inline
            .iter()
            .map(|comment| serde_json::json!({
                "path": comment.path,
                "line": comment.line,
                "side": "RIGHT",
                "body": comment.text,
            }))
            .collect::<Vec<_>>(),
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GhUser {
    pub login: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PullRequestListItem {
    pub number: u32,
    pub title: String,
    pub author: GhUser,
    pub is_draft: bool,
}

pub(crate) const PULL_REQUEST_LIST_FIELDS: &str = "number,title,author,isDraft";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PullRequestComment {
    pub author: GhUser,
    pub body: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PullRequestReview {
    pub author: GhUser,
    pub state: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GhLabel {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PullRequestDetails {
    pub number: u32,
    pub title: String,
    pub author: GhUser,
    pub base_ref_name: String,
    pub head_ref_name: String,
    pub state: String,
    pub is_draft: bool,
    #[serde(default)]
    pub labels: Vec<GhLabel>,
    #[serde(default)]
    pub review_decision: Option<String>,
    #[serde(default)]
    pub latest_reviews: Vec<PullRequestReview>,
    pub changed_files: u32,
    pub additions: u32,
    pub deletions: u32,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub comments: Vec<PullRequestComment>,
}

pub(crate) const PULL_REQUEST_DETAIL_FIELDS: &str = "number,title,author,baseRefName,headRefName,state,isDraft,labels,reviewDecision,latestReviews,changedFiles,additions,deletions,body,comments";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PullRequestCheck {
    pub name: String,
    pub state: String,
}

/// A review comment already posted on a line of the PR.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PostedLineComment {
    pub path: String,
    pub line: Option<u32>,
    pub original_line: Option<u32>,
    pub user: GhUser,
    pub body: String,
}

impl PostedLineComment {
    pub fn display_line(&self) -> Option<u32> {
        self.line.or(self.original_line)
    }
}

/// `gh api --paginate` prints one JSON array per page back to back.
pub(crate) fn parse_paginated<T: for<'de> Deserialize<'de>>(output: &str) -> Result<Vec<T>> {
    let mut items = Vec::new();
    for page in serde_json::Deserializer::from_str(output).into_iter::<Vec<T>>() {
        items.extend(page.context("reading GitHub API response")?);
    }
    Ok(items)
}

async fn run(program: &str, args: &[&str], cwd: &Path, envs: &[(&str, &Path)]) -> Result<String> {
    let mut command = util::command::new_command(program);
    command.args(args).current_dir(cwd);
    for (key, value) in envs {
        command.env(key, value);
    }
    let output = command
        .output()
        .await
        .with_context(|| format!("running {program}; is it installed and on PATH?"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("{program} {}: {}", args.join(" "), stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string())
}

async fn git(dir: &Path, args: &[&str]) -> Result<String> {
    run("git", args, dir, &[]).await
}

async fn gh(dir: &Path, args: &[&str]) -> Result<String> {
    run("gh", args, dir, &[]).await
}

/// The main checkout of the repository, also when `dir` is one of its worktrees.
pub(crate) async fn main_repository(dir: &Path) -> Result<PathBuf> {
    let common = git(
        dir,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )
    .await?;
    Path::new(&common)
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| anyhow!("unexpected git directory {common}"))
}

pub(crate) async fn list_pull_requests(repo: &Path) -> Result<Vec<PullRequestListItem>> {
    let output = gh(
        repo,
        &[
            "pr",
            "list",
            "--limit",
            "100",
            "--json",
            PULL_REQUEST_LIST_FIELDS,
        ],
    )
    .await?;
    serde_json::from_str(&output).context("reading the pull request list")
}

/// A tree object of the directory's working files, new files included.
async fn working_tree(dir: &Path) -> Result<String> {
    let index = git(
        dir,
        &["rev-parse", "--path-format=absolute", "--git-path", "index"],
    )
    .await?;
    let scratch_index = std::env::temp_dir().join(format!(
        "zed-pr-review-index-{}-{}",
        std::process::id(),
        dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("copy")
    ));
    smol::fs::copy(&index, &scratch_index)
        .await
        .context("copying the git index")?;
    let envs = [("GIT_INDEX_FILE", scratch_index.as_path())];
    let result = async {
        run("git", &["add", "-A"], dir, &envs).await?;
        run("git", &["write-tree"], dir, &envs).await
    }
    .await;
    smol::fs::remove_file(&scratch_index).await.log_err();
    result
}

async fn has_local_edits(dir: &Path, loaded_commit: &str) -> Result<bool> {
    let tree = working_tree(dir).await?;
    let loaded_tree = git(dir, &["rev-parse", &format!("{loaded_commit}^{{tree}}")]).await?;
    Ok(tree != loaded_tree)
}

/// Puts the PR's files in the copy with HEAD at the merge base, so every
/// change in the PR is an unstaged change in the git panel.
async fn load_review_copy(dir: &Path, head: &str, base_branch: &str) -> Result<()> {
    git(dir, &["reset", "--quiet", "--hard", head]).await?;
    let merge_base = git(dir, &["merge-base", &format!("origin/{base_branch}"), head]).await?;
    git(dir, &["reset", "--quiet", &merge_base]).await?;
    Ok(())
}

async fn existing_worktree_for_branch(repo: &Path, branch: &str) -> Result<Option<PathBuf>> {
    let list = git(repo, &["worktree", "list", "--porcelain"]).await?;
    let branch_line = format!("branch refs/heads/{branch}");
    let mut current_dir: Option<&str> = None;
    for line in list.lines() {
        if let Some(dir) = line.strip_prefix("worktree ") {
            current_dir = Some(dir);
        } else if line == branch_line
            && let Some(dir) = current_dir
        {
            return Ok(Some(PathBuf::from(dir)));
        }
    }
    Ok(None)
}

pub(crate) struct OpenedReviewCopy {
    pub copy: ReviewCopy,
    /// Set when the PR has new commits but the copy was kept because it has your edits.
    pub kept_local_edits: bool,
}

/// Creates or refreshes the review copy for a PR in `repo`.
pub(crate) async fn prepare_review_copy(repo: &Path, pr_number: u32) -> Result<OpenedReviewCopy> {
    let repo = main_repository(repo).await?;
    let number = pr_number.to_string();
    let base_branch = gh(
        &repo,
        &[
            "pr",
            "view",
            &number,
            "--json",
            "baseRefName",
            "--jq",
            ".baseRefName",
        ],
    )
    .await?;
    git(
        &repo,
        &["fetch", "--quiet", "--no-prune", "origin", &base_branch],
    )
    .await?;
    git(
        &repo,
        &[
            "fetch",
            "--quiet",
            "--no-prune",
            "origin",
            &format!("refs/pull/{pr_number}/head"),
        ],
    )
    .await?;
    let head = git(&repo, &["rev-parse", "FETCH_HEAD"]).await?;

    let repo_name = repo
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("unexpected repository path {}", repo.display()))?;
    let copy = ReviewCopy::new(repo_name, pr_number);
    let head_ref = copy.head_ref();
    let previous_head = git(&repo, &["rev-parse", "--quiet", "--verify", &head_ref])
        .await
        .ok();

    // A copy made earlier in another reviews folder keeps the branch checked out,
    // so reuse it instead of failing to add a second worktree for that branch.
    let copy = match existing_worktree_for_branch(&repo, &copy.branch()).await? {
        Some(existing_dir) if existing_dir != copy.dir => ReviewCopy {
            dir: existing_dir,
            ..copy
        },
        _ => copy,
    };

    let mut kept_local_edits = false;
    if !smol::fs::metadata(&copy.dir).await.is_ok() {
        git(&repo, &["update-ref", &head_ref, &head]).await?;
        smol::fs::create_dir_all(reviews_dir()).await?;
        let dir = copy.dir.to_string_lossy().to_string();
        git(
            &repo,
            &[
                "worktree",
                "add",
                "--quiet",
                "-B",
                &copy.branch(),
                &dir,
                &head,
            ],
        )
        .await?;
        load_review_copy(&copy.dir, &head, &base_branch).await?;
    } else if previous_head.as_deref() != Some(head.as_str()) {
        let has_edits = match &previous_head {
            Some(previous_head) => has_local_edits(&copy.dir, previous_head).await?,
            None => false,
        };
        if has_edits {
            kept_local_edits = true;
        } else {
            git(&repo, &["update-ref", &head_ref, &head]).await?;
            load_review_copy(&copy.dir, &head, &base_branch).await?;
        }
    }

    Ok(OpenedReviewCopy {
        copy,
        kept_local_edits,
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PullRequestOverview {
    pub details: PullRequestDetails,
    pub checks: Vec<PullRequestCheck>,
    pub line_comments: Vec<PostedLineComment>,
}

pub(crate) async fn load_overview(copy: &ReviewCopy) -> Result<PullRequestOverview> {
    let repo = main_repository(&copy.dir).await?;
    let number = copy.pr_number.to_string();
    // The three GitHub calls are independent; running them together keeps the
    // overview from waiting on each round trip in turn.
    let details_args = ["pr", "view", &number, "--json", PULL_REQUEST_DETAIL_FIELDS];
    // `gh pr checks` exits non-zero while checks are pending or failing.
    let checks_args = ["pr", "checks", &number, "--json", "name,state"];
    let (details, checks, line_comments) = futures::join!(
        gh(&repo, &details_args),
        run("gh", &checks_args, &repo, &[]),
        load_line_comments(&repo, copy.pr_number),
    );
    let details: PullRequestDetails =
        serde_json::from_str(&details?).context("reading the pull request")?;
    let checks = checks
        .ok()
        .and_then(|output| serde_json::from_str(&output).log_err())
        .unwrap_or_default();
    let overview = PullRequestOverview {
        details,
        checks,
        line_comments: line_comments?,
    };
    // A cache lets the overview tab show the last state instantly next time.
    if let Some(json) = serde_json::to_string(&overview).log_err() {
        smol::fs::create_dir_all(notes_dir()).await.log_err();
        smol::fs::write(copy.overview_cache_path(), json)
            .await
            .log_err();
    }
    Ok(overview)
}

/// The overview saved by the last successful load, if any.
pub(crate) async fn read_cached_overview(copy: &ReviewCopy) -> Option<PullRequestOverview> {
    let json = smol::fs::read_to_string(copy.overview_cache_path())
        .await
        .ok()?;
    serde_json::from_str(&json).log_err()
}

pub(crate) async fn load_line_comments(
    repo: &Path,
    pr_number: u32,
) -> Result<Vec<PostedLineComment>> {
    let output = gh(
        repo,
        &[
            "api",
            &format!("repos/{{owner}}/{{repo}}/pulls/{pr_number}/comments"),
            "--paginate",
        ],
    )
    .await?;
    parse_paginated(&output)
}

pub(crate) async fn read_notes(copy: &ReviewCopy) -> Result<Option<String>> {
    match smol::fs::read_to_string(copy.notes_path()).await {
        Ok(notes) => Ok(Some(notes)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).context("reading the review notes"),
    }
}

pub(crate) async fn write_notes(copy: &ReviewCopy, notes: &str) -> Result<()> {
    smol::fs::create_dir_all(notes_dir()).await?;
    smol::fs::write(copy.notes_path(), notes)
        .await
        .context("saving the review notes")
}

/// Posts the pending notes as one review and returns its URL.
pub(crate) async fn submit_review(copy: &ReviewCopy, event: ReviewEvent) -> Result<String> {
    let repo = main_repository(&copy.dir).await?;
    let notes = ReviewNotes::parse(&read_notes(copy).await?.unwrap_or_default());
    if notes.is_empty() && event != ReviewEvent::Approve {
        anyhow::bail!("Write a summary or add a line comment first.");
    }
    // Comment on the commit you reviewed; GitHub maps comments if the PR moved on.
    let head = git(&repo, &["rev-parse", &copy.head_ref()]).await?;
    let merge_base = git(&copy.dir, &["rev-parse", "HEAD"]).await?;
    let diff = git(
        &repo,
        &[
            "diff",
            "--no-color",
            "--no-ext-diff",
            "-U3",
            &merge_base,
            &head,
        ],
    )
    .await?;
    let payload = review_payload(&notes, &commentable_lines(&diff), &head, event);

    smol::fs::create_dir_all(notes_dir()).await?;
    let payload_path = notes_dir().join(format!("{}.payload.json", copy.name));
    smol::fs::write(&payload_path, payload.to_string()).await?;
    let payload_arg = payload_path.to_string_lossy().to_string();
    let result = gh(
        &repo,
        &[
            "api",
            "-X",
            "POST",
            &format!("repos/{{owner}}/{{repo}}/pulls/{}/reviews", copy.pr_number),
            "--input",
            &payload_arg,
            "--jq",
            ".html_url",
        ],
    )
    .await;
    smol::fs::remove_file(&payload_path).await.log_err();
    let url = result.context("GitHub rejected the review; your notes are unchanged")?;

    if smol::fs::metadata(copy.notes_path()).await.is_ok() {
        smol::fs::rename(copy.notes_path(), copy.submitted_notes_path()).await?;
    }
    Ok(url)
}

pub(crate) async fn open_on_github(copy: &ReviewCopy) -> Result<()> {
    let repo = main_repository(&copy.dir).await?;
    gh(&repo, &["pr", "view", &copy.pr_number.to_string(), "--web"]).await?;
    Ok(())
}

pub(crate) fn register(workspace: &mut Workspace) {
    workspace.register_action(start_review);
    workspace.register_action(|workspace, _: &OpenOverview, window, cx| {
        if let Some(copy) = require_review_copy(workspace, cx) {
            overview::open_overview(workspace, copy, window, cx);
        }
    });
    workspace.register_action(|workspace, _: &AddComment, window, cx| {
        let Some(copy) = require_review_copy(workspace, cx) else {
            return;
        };
        let Some((path, line)) = active_file_and_line(workspace, cx) else {
            show_message(
                workspace,
                "Put the cursor on a line in one of this pull request's files first.",
                cx,
            );
            return;
        };
        inline_comments::open_composer(workspace, copy, path, line, window, cx);
    });
    workspace.register_action(|workspace, _: &SubmitReview, window, cx| {
        if let Some(copy) = require_review_copy(workspace, cx) {
            let view = overview::open_overview(workspace, copy, window, cx);
            view.update(cx, |view, cx| view.focus_review_editor(window, cx));
        }
    });
    workspace.register_action(|workspace, _: &OpenOnGitHub, window, cx| {
        let Some(copy) = require_review_copy(workspace, cx) else {
            return;
        };
        cx.background_spawn(async move { open_on_github(&copy).await })
            .detach_and_notify_err(cx.weak_entity(), window, cx);
    });
}

pub(crate) fn init(cx: &mut App) {
    inline_comments::init(cx);
}

/// The review copy this window is reviewing, if any.
pub(crate) fn current_review_copy(workspace: &Workspace, cx: &App) -> Option<ReviewCopy> {
    workspace
        .project()
        .read(cx)
        .visible_worktrees(cx)
        .find_map(|worktree| ReviewCopy::for_worktree_root(&worktree.read(cx).abs_path()))
}

fn require_review_copy(
    workspace: &mut Workspace,
    cx: &mut Context<Workspace>,
) -> Option<ReviewCopy> {
    let copy = current_review_copy(workspace, cx);
    if copy.is_none() {
        show_message(
            workspace,
            "This window is not reviewing a pull request. Run \"pr review: start review\" first.",
            cx,
        );
    }
    copy
}

pub(crate) fn show_message(
    workspace: &mut Workspace,
    message: impl Into<std::borrow::Cow<'static, str>>,
    cx: &mut Context<Workspace>,
) {
    workspace.show_toast(
        Toast::new(NotificationId::unique::<StartReview>(), message).autohide(),
        cx,
    );
}

/// The repository-relative path and 1-based line under the cursor of the active editor,
/// including in diff views.
fn active_file_and_line(workspace: &Workspace, cx: &mut App) -> Option<(String, u32)> {
    let editor = workspace.active_item_as::<Editor>(cx)?;
    editor.update(cx, |editor, cx| {
        let snapshot = editor.display_snapshot(cx);
        let head = editor.selections.newest::<Point>(&snapshot).head();
        let (buffer, point) = editor.buffer().read(cx).point_to_buffer_point(head, cx)?;
        let file = buffer.read(cx).file()?.clone();
        Some((file.path().as_unix_str().to_string(), point.row + 1))
    })
}

/// Opens a file of the review copy with the cursor on a 1-based line.
pub(crate) fn open_file_at_line(
    workspace: WeakEntity<Workspace>,
    abs_path: PathBuf,
    line: u32,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace_entity) = workspace.upgrade() else {
        return;
    };
    let open = workspace_entity.update(cx, |workspace, cx| {
        workspace.open_abs_path(abs_path, OpenOptions::default(), window, cx)
    });
    window
        .spawn(cx, async move |cx| {
            let item = open.await?;
            let editor = cx
                .update(|_, cx| item.act_as::<Editor>(cx))?
                .context("this file can't be opened in an editor")?;
            editor.update_in(cx, |editor, window, cx| {
                editor.go_to_singleton_buffer_point(
                    Point::new(line.saturating_sub(1), 0),
                    window,
                    cx,
                )
            })?;
            anyhow::Ok(())
        })
        .detach_and_notify_err(workspace, window, cx);
}

/// Reloads every view that shows review data after the notes or the PR changed.
pub(crate) fn refresh_review_views(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
    for view in workspace
        .items_of_type::<overview::PullRequestOverviewView>(cx)
        .collect::<Vec<_>>()
    {
        view.update(cx, |view, cx| view.reload(cx));
    }
    inline_comments::refresh(workspace, cx);
}

fn start_review(
    workspace: &mut Workspace,
    _: &StartReview,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(repo_dir) = workspace
        .project()
        .read(cx)
        .active_repository(cx)
        .map(|repo| repo.read(cx).work_directory_abs_path.to_path_buf())
    else {
        show_message(
            workspace,
            "Open a git repository with a GitHub remote first.",
            cx,
        );
        return;
    };
    let app_state = workspace.app_state().clone();
    let workspace_handle = cx.weak_entity();
    show_message(workspace, "Loading pull requests…", cx);
    cx.spawn_in(window, {
        let workspace_handle = workspace_handle.clone();
        async move |_, cx| {
            let pull_requests = cx
                .background_spawn({
                    let repo_dir = repo_dir.clone();
                    async move { list_pull_requests(&repo_dir).await }
                })
                .await?;
            if pull_requests.is_empty() {
                workspace_handle.update(cx, |workspace, cx| {
                    show_message(workspace, "This repository has no open pull requests.", cx)
                })?;
                return anyhow::Ok(());
            }
            let options = pull_requests
                .iter()
                .map(|pull_request| {
                    SharedString::from(format!(
                        "#{} {}{}  @{}",
                        pull_request.number,
                        pull_request.title,
                        if pull_request.is_draft { " (draft)" } else { "" },
                        pull_request.author.login
                    ))
                })
                .collect();
            let selection = cx
                .update(|window, cx| {
                    picker_prompt::prompt(
                        "Review a pull request",
                        options,
                        workspace_handle.clone(),
                        window,
                        cx,
                    )
                })?
                .await;
            let Some(pull_request) = selection.and_then(|ix| pull_requests.get(ix)) else {
                return Ok(());
            };
            let pr_number = pull_request.number;
            workspace_handle.update(cx, |workspace, cx| {
                show_message(
                    workspace,
                    format!("Preparing a review copy of #{pr_number}…"),
                    cx,
                )
            })?;
            let opened = cx
                .background_spawn(async move { prepare_review_copy(&repo_dir, pr_number).await })
                .await?;
            let open = cx.update(|_, cx| {
                workspace::open_paths(
                    &[opened.copy.dir.clone()],
                    app_state,
                    OpenOptions {
                        open_mode: OpenMode::NewWindow,
                        ..Default::default()
                    },
                    cx,
                )
            })?;
            let result = open.await?;
            let copy = opened.copy;
            let kept_local_edits = opened.kept_local_edits;
            result.window.update(cx, |_, window, cx| {
                result.workspace.update(cx, |workspace, cx| {
                    workspace.open_panel::<crate::git_panel::GitPanel>(window, cx);
                    overview::open_overview(workspace, copy, window, cx);
                    if kept_local_edits {
                        show_message(
                            workspace,
                            "The pull request has new commits, but this review copy has your edits, so it was not refreshed.",
                            cx,
                        );
                    }
                })
            })?;
            Ok(())
        }
    })
    .detach_and_notify_err(workspace_handle, window, cx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_review_copy_for_worktree_root() {
        let reviews = Path::new("/home/me/.cache/pr-reviews");
        let copy = ReviewCopy::for_worktree_root_in(&reviews.join("kip-pr-42"), reviews).unwrap();
        assert_eq!(copy.pr_number, 42);
        assert_eq!(copy.name, "kip-pr-42");
        assert_eq!(copy.branch(), "review/pr-42");
        assert_eq!(copy.head_ref(), "refs/pr-review/pr-42");

        let hyphenated = reviews.join("my-pr-tool-pr-7");
        assert_eq!(
            ReviewCopy::for_worktree_root_in(&hyphenated, reviews).map(|copy| copy.pr_number),
            Some(7)
        );
        assert!(
            ReviewCopy::for_worktree_root_in(&reviews.join("kip-branch-main"), reviews).is_none()
        );
        assert!(ReviewCopy::for_worktree_root_in(Path::new("/src/kip-pr-42"), reviews).is_none());
    }

    #[test]
    fn test_parse_notes() {
        let notes = "## Summary\n\nLooks good overall.\n\n### src/main.rs:12\n\nRename this.\nIt is unclear.\n\n### src/lib.rs:3\n\n\n### README.md:1\n\nTypo.\n";
        let parsed = ReviewNotes::parse(notes);
        assert_eq!(parsed.body, "Looks good overall.");
        assert_eq!(
            parsed.comments,
            vec![
                PendingComment {
                    path: "src/main.rs".into(),
                    line: 12,
                    text: "Rename this.\nIt is unclear.".into(),
                },
                PendingComment {
                    path: "README.md".into(),
                    line: 1,
                    text: "Typo.".into(),
                },
            ]
        );
        assert!(ReviewNotes::parse("## Summary\n\n").is_empty());
    }

    #[test]
    fn test_remove_pending_comment() {
        let notes = append_pending_comment(None, "a.rs", 4, "One");
        let notes = append_pending_comment(Some(&notes), "a.rs", 4, "Two");
        let notes = set_review_body(Some(&notes), "Body");
        let notes = remove_pending_comment(&notes, "a.rs", 4, "One");
        let parsed = ReviewNotes::parse(&notes);
        assert_eq!(parsed.body, "Body");
        assert_eq!(
            parsed.comments,
            vec![PendingComment {
                path: "a.rs".into(),
                line: 4,
                text: "Two".into(),
            }]
        );
    }

    #[test]
    fn test_append_and_set_body() {
        let notes = append_pending_comment(None, "a.rs", 4, "  Why?  ");
        assert_eq!(notes, "## Summary\n\n\n### a.rs:4\n\nWhy?\n");
        let notes = append_pending_comment(Some(&notes), "b.rs", 9, "Nit");
        let notes = set_review_body(Some(&notes), "Ship it");
        let parsed = ReviewNotes::parse(&notes);
        assert_eq!(parsed.body, "Ship it");
        assert_eq!(parsed.comments.len(), 2);
        assert_eq!(parsed.comments[1].path, "b.rs");
    }

    #[test]
    fn test_commentable_lines_and_payload() {
        let diff = "diff --git a/src/a.rs b/src/a.rs\n--- a/src/a.rs\n+++ b/src/a.rs\n@@ -1,3 +1,4 @@ fn main\n a\n+b\n c\n d\n@@ -20 +21 @@\n-x\n+y\ndiff --git a/gone.rs b/gone.rs\n--- a/gone.rs\n+++ /dev/null\n@@ -1 +0,0 @@\n-z\n";
        let lines = commentable_lines(diff);
        assert_eq!(
            lines.get("src/a.rs").cloned().unwrap_or_default(),
            BTreeSet::from([1, 2, 3, 4, 21])
        );
        assert!(!lines.contains_key("gone.rs"));

        let notes = ReviewNotes {
            body: "Body".into(),
            comments: vec![
                PendingComment {
                    path: "src/a.rs".into(),
                    line: 2,
                    text: "Inline".into(),
                },
                PendingComment {
                    path: "src/a.rs".into(),
                    line: 50,
                    text: "Far\naway".into(),
                },
            ],
        };
        let payload = review_payload(&notes, &lines, "abc123", ReviewEvent::RequestChanges);
        assert_eq!(payload["event"], "REQUEST_CHANGES");
        assert_eq!(payload["commit_id"], "abc123");
        assert_eq!(payload["comments"].as_array().map(Vec::len), Some(1));
        assert_eq!(payload["comments"][0]["line"], 2);
        assert_eq!(
            payload["body"],
            "Body\n\nComments on lines outside the diff:\n- `src/a.rs:50`: Far away"
        );
    }

    #[test]
    fn test_parse_paginated() {
        let output = r#"[{"path":"a","line":1,"original_line":1,"user":{"login":"x"},"body":"hi"}][{"path":"b","line":null,"original_line":7,"user":{"login":"y"},"body":"yo"}]"#;
        let comments: Vec<PostedLineComment> = parse_paginated(output).unwrap();
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[1].display_line(), Some(7));
    }
}
