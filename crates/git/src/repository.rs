use crate::commit::parse_git_diff_name_status;
use crate::stash::GitStash;
use crate::status::{DiffTreeType, GitStatus, StatusCode, TreeDiff};
use crate::{Oid, SHORT_SHA_LENGTH};
use anyhow::{Context as _, Result, anyhow, bail};
use collections::HashMap;
use futures::future::BoxFuture;
use futures::io::BufWriter;
use futures::{AsyncWriteExt, FutureExt as _, select_biased};
use git2::BranchType;
use gpui::{AppContext as _, AsyncApp, BackgroundExecutor, SharedString, Task};
use parking_lot::Mutex;
use rope::Rope;
use schemars::JsonSchema;
use serde::Deserialize;
use smol::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::process::{ExitStatus, Stdio};
use std::{
    cmp::Ordering,
    future,
    path::{Path, PathBuf},
    sync::Arc,
};
use sum_tree::MapSeekTarget;
use thiserror::Error;
use util::command::new_smol_command;
use util::paths::PathStyle;
use util::rel_path::RelPath;
use util::{ResultExt, paths};
use uuid::Uuid;

pub use askpass::{AskPassDelegate, AskPassResult, AskPassSession};

pub const REMOTE_CANCELLED_BY_USER: &str = "Operation cancelled by user";

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Branch {
    pub is_head: bool,
    pub ref_name: SharedString,
    pub upstream: Option<Upstream>,
    pub most_recent_commit: Option<CommitSummary>,
}

impl Branch {
    pub fn name(&self) -> &str {
        self.ref_name
            .as_ref()
            .strip_prefix("refs/heads/")
            .or_else(|| self.ref_name.as_ref().strip_prefix("refs/remotes/"))
            .unwrap_or(self.ref_name.as_ref())
    }

    pub fn is_remote(&self) -> bool {
        self.ref_name.starts_with("refs/remotes/")
    }

    pub fn tracking_status(&self) -> Option<UpstreamTrackingStatus> {
        self.upstream
            .as_ref()
            .and_then(|upstream| upstream.tracking.status())
    }

    pub fn priority_key(&self) -> (bool, Option<i64>) {
        (
            self.is_head,
            self.most_recent_commit
                .as_ref()
                .map(|commit| commit.commit_timestamp),
        )
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Upstream {
    pub ref_name: SharedString,
    pub tracking: UpstreamTracking,
}

impl Upstream {
    pub fn is_remote(&self) -> bool {
        self.remote_name().is_some()
    }

    pub fn remote_name(&self) -> Option<&str> {
        self.ref_name
            .strip_prefix("refs/remotes/")
            .and_then(|stripped| stripped.split("/").next())
    }

    pub fn stripped_ref_name(&self) -> Option<&str> {
        self.ref_name.strip_prefix("refs/remotes/")
    }
}

#[derive(Clone, Copy, Default)]
pub struct CommitOptions {
    pub amend: bool,
    pub signoff: bool,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum UpstreamTracking {
    /// Remote ref not present in local repository.
    Gone,
    /// Remote ref present in local repository (fetched from remote).
    Tracked(UpstreamTrackingStatus),
}

impl From<UpstreamTrackingStatus> for UpstreamTracking {
    fn from(status: UpstreamTrackingStatus) -> Self {
        UpstreamTracking::Tracked(status)
    }
}

impl UpstreamTracking {
    pub fn is_gone(&self) -> bool {
        matches!(self, UpstreamTracking::Gone)
    }

    pub fn status(&self) -> Option<UpstreamTrackingStatus> {
        match self {
            UpstreamTracking::Gone => None,
            UpstreamTracking::Tracked(status) => Some(*status),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemoteCommandOutput {
    pub stdout: String,
    pub stderr: String,
}

impl RemoteCommandOutput {
    pub fn is_empty(&self) -> bool {
        self.stdout.is_empty() && self.stderr.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct UpstreamTrackingStatus {
    pub ahead: u32,
    pub behind: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CommitSummary {
    pub sha: SharedString,
    pub subject: SharedString,
    /// This is a unix timestamp
    pub commit_timestamp: i64,
    pub author_name: SharedString,
    pub has_parent: bool,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct CommitDetails {
    pub sha: SharedString,
    pub message: SharedString,
    pub commit_timestamp: i64,
    pub author_email: SharedString,
    pub author_name: SharedString,
}

#[derive(Debug)]
pub struct CommitDiff {
    pub files: Vec<CommitFile>,
}

#[derive(Debug)]
pub struct CommitFile {
    pub path: RepoPath,
    pub old_text: Option<String>,
    pub new_text: Option<String>,
}

impl CommitDetails {
    pub fn short_sha(&self) -> SharedString {
        self.sha[..SHORT_SHA_LENGTH].to_string().into()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Remote {
    pub name: SharedString,
}

pub enum ResetMode {
    /// Reset the branch pointer, leave index and worktree unchanged (this will make it look like things that were
    /// committed are now staged).
    Soft,
    /// Reset the branch pointer and index, leave worktree unchanged (this makes it look as though things that were
    /// committed are now unstaged).
    Mixed,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FetchOptions {
    All,
    Remote(Remote),
}

impl FetchOptions {
    pub fn to_proto(&self) -> Option<String> {
        match self {
            FetchOptions::All => None,
            FetchOptions::Remote(remote) => Some(remote.clone().name.into()),
        }
    }

    pub fn from_proto(remote_name: Option<String>) -> Self {
        match remote_name {
            Some(name) => FetchOptions::Remote(Remote { name: name.into() }),
            None => FetchOptions::All,
        }
    }

    pub fn name(&self) -> SharedString {
        match self {
            Self::All => "Fetch all remotes".into(),
            Self::Remote(remote) => remote.name.clone(),
        }
    }
}

impl std::fmt::Display for FetchOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchOptions::All => write!(f, "--all"),
            FetchOptions::Remote(remote) => write!(f, "{}", remote.name),
        }
    }
}

/// Modifies .git/info/exclude temporarily
pub struct GitExcludeOverride {
    git_exclude_path: PathBuf,
    original_excludes: Option<String>,
    added_excludes: Option<String>,
}

impl GitExcludeOverride {
    const START_BLOCK_MARKER: &str = "\n\n#  ====== Auto-added by Zed: =======\n";
    const END_BLOCK_MARKER: &str = "\n#  ====== End of auto-added by Zed =======\n";

    pub async fn new(git_exclude_path: PathBuf) -> Result<Self> {
        let original_excludes =
            smol::fs::read_to_string(&git_exclude_path)
                .await
                .ok()
                .map(|content| {
                    // Auto-generated lines are normally cleaned up in
                    // `restore_original()` or `drop()`, but may stuck in rare cases.
                    // Make sure to remove them.
                    Self::remove_auto_generated_block(&content)
                });

        Ok(GitExcludeOverride {
            git_exclude_path,
            original_excludes,
            added_excludes: None,
        })
    }

    pub async fn add_excludes(&mut self, excludes: &str) -> Result<()> {
        self.added_excludes = Some(if let Some(ref already_added) = self.added_excludes {
            format!("{already_added}\n{excludes}")
        } else {
            excludes.to_string()
        });

        let mut content = self.original_excludes.clone().unwrap_or_default();

        content.push_str(Self::START_BLOCK_MARKER);
        content.push_str(self.added_excludes.as_ref().unwrap());
        content.push_str(Self::END_BLOCK_MARKER);

        smol::fs::write(&self.git_exclude_path, content).await?;
        Ok(())
    }

    pub async fn restore_original(&mut self) -> Result<()> {
        if let Some(ref original) = self.original_excludes {
            smol::fs::write(&self.git_exclude_path, original).await?;
        } else if self.git_exclude_path.exists() {
            smol::fs::remove_file(&self.git_exclude_path).await?;
        }

        self.added_excludes = None;

        Ok(())
    }

    fn remove_auto_generated_block(content: &str) -> String {
        let start_marker = Self::START_BLOCK_MARKER;
        let end_marker = Self::END_BLOCK_MARKER;
        let mut content = content.to_string();

        let start_index = content.find(start_marker);
        let end_index = content.rfind(end_marker);

        if let (Some(start), Some(end)) = (start_index, end_index) {
            if end > start {
                content.replace_range(start..end + end_marker.len(), "");
            }
        }

        // Older versions of Zed didn't have end-of-block markers,
        // so it's impossible to determine auto-generated lines.
        // Conservatively remove the standard list of excludes
        let standard_excludes = format!(
            "{}{}",
            Self::START_BLOCK_MARKER,
            include_str!("./checkpoint.gitignore")
        );
        content = content.replace(&standard_excludes, "");

        content
    }
}

impl Drop for GitExcludeOverride {
    fn drop(&mut self) {
        if self.added_excludes.is_some() {
            let git_exclude_path = self.git_exclude_path.clone();
            let original_excludes = self.original_excludes.clone();
            smol::spawn(async move {
                if let Some(original) = original_excludes {
                    smol::fs::write(&git_exclude_path, original).await
                } else {
                    smol::fs::remove_file(&git_exclude_path).await
                }
            })
            .detach();
        }
    }
}

pub trait GitRepository: Send + Sync {
    fn reload_index(&self);

    /// Returns the contents of an entry in the repository's index, or None if there is no entry for the given path.
    ///
    /// Also returns `None` for symlinks.
    fn load_index_text(&self, path: RepoPath) -> BoxFuture<'_, Option<String>>;

    /// Returns the contents of an entry in the repository's HEAD, or None if HEAD does not exist or has no entry for the given path.
    ///
    /// Also returns `None` for symlinks.
    fn load_committed_text(&self, path: RepoPath) -> BoxFuture<'_, Option<String>>;
    fn load_blob_content(&self, oid: Oid) -> BoxFuture<'_, Result<String>>;

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, anyhow::Result<()>>;

    /// Returns the URL of the remote with the given name.
    fn remote_url(&self, name: &str) -> Option<String>;

    /// Resolve a list of refs to SHAs.
    fn revparse_batch(&self, revs: Vec<String>) -> BoxFuture<'_, Result<Vec<Option<String>>>>;

    fn head_sha(&self) -> BoxFuture<'_, Option<String>> {
        async move {
            self.revparse_batch(vec!["HEAD".into()])
                .await
                .unwrap_or_default()
                .into_iter()
                .next()
                .flatten()
        }
        .boxed()
    }

    fn merge_message(&self) -> BoxFuture<'_, Option<String>>;

    fn status(&self, path_prefixes: &[RepoPath]) -> Task<Result<GitStatus>>;
    fn diff_tree(&self, request: DiffTreeType) -> BoxFuture<'_, Result<TreeDiff>>;

    fn stash_entries(&self) -> BoxFuture<'_, Result<GitStash>>;

    fn branches(&self) -> BoxFuture<'_, Result<Vec<Branch>>>;

    fn change_branch(&self, name: String) -> BoxFuture<'_, Result<()>>;
    fn create_branch(&self, name: String) -> BoxFuture<'_, Result<()>>;
    fn rename_branch(&self, branch: String, new_name: String) -> BoxFuture<'_, Result<()>>;

    fn reset(
        &self,
        commit: String,
        mode: ResetMode,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;

    fn checkout_files(
        &self,
        commit: String,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;

    fn show(&self, commit: String) -> BoxFuture<'_, Result<CommitDetails>>;

    fn load_commit(&self, commit: String, cx: AsyncApp) -> BoxFuture<'_, Result<CommitDiff>>;
    fn blame(&self, path: RepoPath, content: Rope) -> BoxFuture<'_, Result<crate::blame::Blame>>;

    /// Returns the absolute path to the repository. For worktrees, this will be the path to the
    /// worktree's gitdir within the main repository (typically `.git/worktrees/<name>`).
    fn path(&self) -> PathBuf;

    fn main_repository_path(&self) -> PathBuf;

    /// Updates the index to match the worktree at the given paths.
    ///
    /// If any of the paths have been deleted from the worktree, they will be removed from the index if found there.
    fn stage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;
    /// Updates the index to match HEAD at the given paths.
    ///
    /// If any of the paths were previously staged but do not exist in HEAD, they will be removed from the index.
    fn unstage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;

    fn commit(
        &self,
        message: SharedString,
        name_and_email: Option<(SharedString, SharedString)>,
        options: CommitOptions,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;

    fn stash_paths(
        &self,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;

    fn stash_pop(
        &self,
        index: Option<usize>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;

    fn stash_apply(
        &self,
        index: Option<usize>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;

    fn stash_drop(
        &self,
        index: Option<usize>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>>;

    fn push(
        &self,
        branch_name: String,
        upstream_name: String,
        options: Option<PushOptions>,
        askpass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        // This method takes an AsyncApp to ensure it's invoked on the main thread,
        // otherwise git-credentials-manager won't work.
        cx: AsyncApp,
    ) -> BoxFuture<'_, Result<RemoteCommandOutput>>;

    fn pull(
        &self,
        branch_name: String,
        upstream_name: String,
        askpass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        // This method takes an AsyncApp to ensure it's invoked on the main thread,
        // otherwise git-credentials-manager won't work.
        cx: AsyncApp,
    ) -> BoxFuture<'_, Result<RemoteCommandOutput>>;

    fn fetch(
        &self,
        fetch_options: FetchOptions,
        askpass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        // This method takes an AsyncApp to ensure it's invoked on the main thread,
        // otherwise git-credentials-manager won't work.
        cx: AsyncApp,
    ) -> BoxFuture<'_, Result<RemoteCommandOutput>>;

    fn get_remotes(&self, branch_name: Option<String>) -> BoxFuture<'_, Result<Vec<Remote>>>;

    /// returns a list of remote branches that contain HEAD
    fn check_for_pushed_commit(&self) -> BoxFuture<'_, Result<Vec<SharedString>>>;

    /// Run git diff
    fn diff(&self, diff: DiffType) -> BoxFuture<'_, Result<String>>;

    /// Creates a checkpoint for the repository.
    fn checkpoint(&self) -> BoxFuture<'static, Result<GitRepositoryCheckpoint>>;

    /// Resets to a previously-created checkpoint.
    fn restore_checkpoint(&self, checkpoint: GitRepositoryCheckpoint) -> BoxFuture<'_, Result<()>>;

    /// Compares two checkpoints, returning true if they are equal
    fn compare_checkpoints(
        &self,
        left: GitRepositoryCheckpoint,
        right: GitRepositoryCheckpoint,
    ) -> BoxFuture<'_, Result<bool>>;

    /// Computes a diff between two checkpoints.
    fn diff_checkpoints(
        &self,
        base_checkpoint: GitRepositoryCheckpoint,
        target_checkpoint: GitRepositoryCheckpoint,
    ) -> BoxFuture<'_, Result<String>>;

    fn default_branch(&self) -> BoxFuture<'_, Result<Option<SharedString>>>;
}

pub enum DiffType {
    HeadToIndex,
    HeadToWorktree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
pub enum PushOptions {
    SetUpstream,
    Force,
}

impl std::fmt::Debug for dyn GitRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn GitRepository<...>").finish()
    }
}

pub struct RealGitRepository {
    pub repository: Arc<Mutex<git2::Repository>>,
    pub system_git_binary_path: Option<PathBuf>,
    pub any_git_binary_path: PathBuf,
    executor: BackgroundExecutor,
}

impl RealGitRepository {
    pub fn new(
        dotgit_path: &Path,
        bundled_git_binary_path: Option<PathBuf>,
        system_git_binary_path: Option<PathBuf>,
        executor: BackgroundExecutor,
    ) -> Option<Self> {
        let any_git_binary_path = system_git_binary_path.clone().or(bundled_git_binary_path)?;
        let workdir_root = dotgit_path.parent()?;
        let repository = git2::Repository::open(workdir_root).log_err()?;
        Some(Self {
            repository: Arc::new(Mutex::new(repository)),
            system_git_binary_path,
            any_git_binary_path,
            executor,
        })
    }

    fn working_directory(&self) -> Result<PathBuf> {
        self.repository
            .lock()
            .workdir()
            .context("failed to read git work directory")
            .map(Path::to_path_buf)
    }
}

#[derive(Clone, Debug)]
pub struct GitRepositoryCheckpoint {
    pub commit_sha: Oid,
}

#[derive(Debug)]
pub struct GitCommitter {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub async fn get_git_committer(cx: &AsyncApp) -> GitCommitter {
    if cfg!(any(feature = "test-support", test)) {
        return GitCommitter {
            name: None,
            email: None,
        };
    }

    let git_binary_path =
        if cfg!(target_os = "macos") && option_env!("ZED_BUNDLE").as_deref() == Some("true") {
            cx.update(|cx| {
                cx.path_for_auxiliary_executable("git")
                    .context("could not find git binary path")
                    .log_err()
            })
            .ok()
            .flatten()
        } else {
            None
        };

    let git = GitBinary::new(
        git_binary_path.unwrap_or(PathBuf::from("git")),
        paths::home_dir().clone(),
        cx.background_executor().clone(),
    );

    cx.background_spawn(async move {
        let name = git.run(["config", "--global", "user.name"]).await.log_err();
        let email = git
            .run(["config", "--global", "user.email"])
            .await
            .log_err();
        GitCommitter { name, email }
    })
    .await
}

impl GitRepository for RealGitRepository {
    fn reload_index(&self) {
        if let Ok(mut index) = self.repository.lock().index() {
            _ = index.read(false);
        }
    }

    fn path(&self) -> PathBuf {
        let repo = self.repository.lock();
        repo.path().into()
    }

    fn main_repository_path(&self) -> PathBuf {
        let repo = self.repository.lock();
        repo.commondir().into()
    }

    fn show(&self, commit: String) -> BoxFuture<'_, Result<CommitDetails>> {
        let git_binary_path = self.any_git_binary_path.clone();
        let working_directory = self.working_directory();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let output = new_smol_command(git_binary_path)
                    .current_dir(&working_directory)
                    .args([
                        "--no-optional-locks",
                        "show",
                        "--no-patch",
                        "--format=%H%x00%B%x00%at%x00%ae%x00%an%x00",
                        &commit,
                    ])
                    .output()
                    .await?;
                let output = std::str::from_utf8(&output.stdout)?;
                let fields = output.split('\0').collect::<Vec<_>>();
                if fields.len() != 6 {
                    bail!("unexpected git-show output for {commit:?}: {output:?}")
                }
                let sha = fields[0].to_string().into();
                let message = fields[1].to_string().into();
                let commit_timestamp = fields[2].parse()?;
                let author_email = fields[3].to_string().into();
                let author_name = fields[4].to_string().into();
                Ok(CommitDetails {
                    sha,
                    message,
                    commit_timestamp,
                    author_email,
                    author_name,
                })
            })
            .boxed()
    }

    fn load_commit(&self, commit: String, cx: AsyncApp) -> BoxFuture<'_, Result<CommitDiff>> {
        let Some(working_directory) = self.repository.lock().workdir().map(ToOwned::to_owned)
        else {
            return future::ready(Err(anyhow!("no working directory"))).boxed();
        };
        let git_binary_path = self.any_git_binary_path.clone();
        cx.background_spawn(async move {
            let show_output = util::command::new_smol_command(&git_binary_path)
                .current_dir(&working_directory)
                .args([
                    "--no-optional-locks",
                    "show",
                    "--format=",
                    "-z",
                    "--no-renames",
                    "--name-status",
                    "--first-parent",
                ])
                .arg(&commit)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
                .context("starting git show process")?;

            let show_stdout = String::from_utf8_lossy(&show_output.stdout);
            let changes = parse_git_diff_name_status(&show_stdout);
            let parent_sha = format!("{}^", commit);

            let mut cat_file_process = util::command::new_smol_command(&git_binary_path)
                .current_dir(&working_directory)
                .args(["--no-optional-locks", "cat-file", "--batch=%(objectsize)"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .context("starting git cat-file process")?;

            let mut files = Vec::<CommitFile>::new();
            let mut stdin = BufWriter::with_capacity(512, cat_file_process.stdin.take().unwrap());
            let mut stdout = BufReader::new(cat_file_process.stdout.take().unwrap());
            let mut info_line = String::new();
            let mut newline = [b'\0'];
            for (path, status_code) in changes {
                // git-show outputs `/`-delimited paths even on Windows.
                let Some(rel_path) = RelPath::unix(path).log_err() else {
                    continue;
                };

                match status_code {
                    StatusCode::Modified => {
                        stdin.write_all(commit.as_bytes()).await?;
                        stdin.write_all(b":").await?;
                        stdin.write_all(path.as_bytes()).await?;
                        stdin.write_all(b"\n").await?;
                        stdin.write_all(parent_sha.as_bytes()).await?;
                        stdin.write_all(b":").await?;
                        stdin.write_all(path.as_bytes()).await?;
                        stdin.write_all(b"\n").await?;
                    }
                    StatusCode::Added => {
                        stdin.write_all(commit.as_bytes()).await?;
                        stdin.write_all(b":").await?;
                        stdin.write_all(path.as_bytes()).await?;
                        stdin.write_all(b"\n").await?;
                    }
                    StatusCode::Deleted => {
                        stdin.write_all(parent_sha.as_bytes()).await?;
                        stdin.write_all(b":").await?;
                        stdin.write_all(path.as_bytes()).await?;
                        stdin.write_all(b"\n").await?;
                    }
                    _ => continue,
                }
                stdin.flush().await?;

                info_line.clear();
                stdout.read_line(&mut info_line).await?;

                let len = info_line.trim_end().parse().with_context(|| {
                    format!("invalid object size output from cat-file {info_line}")
                })?;
                let mut text = vec![0; len];
                stdout.read_exact(&mut text).await?;
                stdout.read_exact(&mut newline).await?;
                let text = String::from_utf8_lossy(&text).to_string();

                let mut old_text = None;
                let mut new_text = None;
                match status_code {
                    StatusCode::Modified => {
                        info_line.clear();
                        stdout.read_line(&mut info_line).await?;
                        let len = info_line.trim_end().parse().with_context(|| {
                            format!("invalid object size output from cat-file {}", info_line)
                        })?;
                        let mut parent_text = vec![0; len];
                        stdout.read_exact(&mut parent_text).await?;
                        stdout.read_exact(&mut newline).await?;
                        old_text = Some(String::from_utf8_lossy(&parent_text).to_string());
                        new_text = Some(text);
                    }
                    StatusCode::Added => new_text = Some(text),
                    StatusCode::Deleted => old_text = Some(text),
                    _ => continue,
                }

                files.push(CommitFile {
                    path: rel_path.into(),
                    old_text,
                    new_text,
                })
            }

            Ok(CommitDiff { files })
        })
        .boxed()
    }

    fn reset(
        &self,
        commit: String,
        mode: ResetMode,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        async move {
            let working_directory = self.working_directory();

            let mode_flag = match mode {
                ResetMode::Mixed => "--mixed",
                ResetMode::Soft => "--soft",
            };

            let output = new_smol_command(&self.any_git_binary_path)
                .envs(env.iter())
                .current_dir(&working_directory?)
                .args(["reset", mode_flag, &commit])
                .output()
                .await?;
            anyhow::ensure!(
                output.status.success(),
                "Failed to reset:\n{}",
                String::from_utf8_lossy(&output.stderr),
            );
            Ok(())
        }
        .boxed()
    }

    fn checkout_files(
        &self,
        commit: String,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        async move {
            if paths.is_empty() {
                return Ok(());
            }

            let output = new_smol_command(&git_binary_path)
                .current_dir(&working_directory?)
                .envs(env.iter())
                .args(["checkout", &commit, "--"])
                .args(paths.iter().map(|path| path.as_unix_str()))
                .output()
                .await?;
            anyhow::ensure!(
                output.status.success(),
                "Failed to checkout files:\n{}",
                String::from_utf8_lossy(&output.stderr),
            );
            Ok(())
        }
        .boxed()
    }

    fn load_index_text(&self, path: RepoPath) -> BoxFuture<'_, Option<String>> {
        // https://git-scm.com/book/en/v2/Git-Internals-Git-Objects
        const GIT_MODE_SYMLINK: u32 = 0o120000;

        let repo = self.repository.clone();
        self.executor
            .spawn(async move {
                fn logic(repo: &git2::Repository, path: &RepoPath) -> Result<Option<String>> {
                    // This check is required because index.get_path() unwraps internally :(
                    let mut index = repo.index()?;
                    index.read(false)?;

                    const STAGE_NORMAL: i32 = 0;
                    let oid = match index.get_path(path.as_std_path(), STAGE_NORMAL) {
                        Some(entry) if entry.mode != GIT_MODE_SYMLINK => entry.id,
                        _ => return Ok(None),
                    };

                    let content = repo.find_blob(oid)?.content().to_owned();
                    Ok(String::from_utf8(content).ok())
                }

                match logic(&repo.lock(), &path) {
                    Ok(value) => return value,
                    Err(err) => log::error!("Error loading index text: {:?}", err),
                }
                None
            })
            .boxed()
    }

    fn load_committed_text(&self, path: RepoPath) -> BoxFuture<'_, Option<String>> {
        let repo = self.repository.clone();
        self.executor
            .spawn(async move {
                let repo = repo.lock();
                let head = repo.head().ok()?.peel_to_tree().log_err()?;
                let entry = head.get_path(path.as_std_path()).ok()?;
                if entry.filemode() == i32::from(git2::FileMode::Link) {
                    return None;
                }
                let content = repo.find_blob(entry.id()).log_err()?.content().to_owned();
                String::from_utf8(content).ok()
            })
            .boxed()
    }

    fn load_blob_content(&self, oid: Oid) -> BoxFuture<'_, Result<String>> {
        let repo = self.repository.clone();
        self.executor
            .spawn(async move {
                let repo = repo.lock();
                let content = repo.find_blob(oid.0)?.content().to_owned();
                Ok(String::from_utf8(content)?)
            })
            .boxed()
    }

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, anyhow::Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                if let Some(content) = content {
                    let mut child = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .envs(env.iter())
                        .args(["hash-object", "-w", "--stdin"])
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()?;
                    let mut stdin = child.stdin.take().unwrap();
                    stdin.write_all(content.as_bytes()).await?;
                    stdin.flush().await?;
                    drop(stdin);
                    let output = child.output().await?.stdout;
                    let sha = str::from_utf8(&output)?.trim();

                    log::debug!("indexing SHA: {sha}, path {path:?}");

                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .envs(env.iter())
                        .args(["update-index", "--add", "--cacheinfo", "100644", sha])
                        .arg(path.as_unix_str())
                        .output()
                        .await?;

                    anyhow::ensure!(
                        output.status.success(),
                        "Failed to stage:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                } else {
                    log::debug!("removing path {path:?} from the index");
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .envs(env.iter())
                        .args(["update-index", "--force-remove"])
                        .arg(path.as_unix_str())
                        .output()
                        .await?;
                    anyhow::ensure!(
                        output.status.success(),
                        "Failed to unstage:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }

                Ok(())
            })
            .boxed()
    }

    fn remote_url(&self, name: &str) -> Option<String> {
        let repo = self.repository.lock();
        let remote = repo.find_remote(name).ok()?;
        remote.url().map(|url| url.to_string())
    }

    fn revparse_batch(&self, revs: Vec<String>) -> BoxFuture<'_, Result<Vec<Option<String>>>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let mut process = new_smol_command(&git_binary_path)
                    .current_dir(&working_directory)
                    .args([
                        "--no-optional-locks",
                        "cat-file",
                        "--batch-check=%(objectname)",
                    ])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let stdin = process
                    .stdin
                    .take()
                    .context("no stdin for git cat-file subprocess")?;
                let mut stdin = BufWriter::new(stdin);
                for rev in &revs {
                    stdin.write_all(rev.as_bytes()).await?;
                    stdin.write_all(b"\n").await?;
                }
                stdin.flush().await?;
                drop(stdin);

                let output = process.output().await?;
                let output = std::str::from_utf8(&output.stdout)?;
                let shas = output
                    .lines()
                    .map(|line| {
                        if line.ends_with("missing") {
                            None
                        } else {
                            Some(line.to_string())
                        }
                    })
                    .collect::<Vec<_>>();

                if shas.len() != revs.len() {
                    // In an octopus merge, git cat-file still only outputs the first sha from MERGE_HEAD.
                    bail!("unexpected number of shas")
                }

                Ok(shas)
            })
            .boxed()
    }

    fn merge_message(&self) -> BoxFuture<'_, Option<String>> {
        let path = self.path().join("MERGE_MSG");
        self.executor
            .spawn(async move { std::fs::read_to_string(&path).ok() })
            .boxed()
    }

    fn status(&self, path_prefixes: &[RepoPath]) -> Task<Result<GitStatus>> {
        let git_binary_path = self.any_git_binary_path.clone();
        let working_directory = match self.working_directory() {
            Ok(working_directory) => working_directory,
            Err(e) => return Task::ready(Err(e)),
        };
        let args = git_status_args(path_prefixes);
        log::debug!("Checking for git status in {path_prefixes:?}");
        self.executor.spawn(async move {
            let output = new_smol_command(&git_binary_path)
                .current_dir(working_directory)
                .args(args)
                .output()
                .await?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.parse()
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("git status failed: {stderr}");
            }
        })
    }

    fn diff_tree(&self, request: DiffTreeType) -> BoxFuture<'_, Result<TreeDiff>> {
        let git_binary_path = self.any_git_binary_path.clone();
        let working_directory = match self.working_directory() {
            Ok(working_directory) => working_directory,
            Err(e) => return Task::ready(Err(e)).boxed(),
        };

        let mut args = vec![
            OsString::from("--no-optional-locks"),
            OsString::from("diff-tree"),
            OsString::from("-r"),
            OsString::from("-z"),
            OsString::from("--no-renames"),
        ];
        match request {
            DiffTreeType::MergeBase { base, head } => {
                args.push("--merge-base".into());
                args.push(OsString::from(base.as_str()));
                args.push(OsString::from(head.as_str()));
            }
            DiffTreeType::Since { base, head } => {
                args.push(OsString::from(base.as_str()));
                args.push(OsString::from(head.as_str()));
            }
        }

        self.executor
            .spawn(async move {
                let output = new_smol_command(&git_binary_path)
                    .current_dir(working_directory)
                    .args(args)
                    .output()
                    .await?;
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.parse()
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("git status failed: {stderr}");
                }
            })
            .boxed()
    }

    fn stash_entries(&self) -> BoxFuture<'_, Result<GitStash>> {
        let git_binary_path = self.any_git_binary_path.clone();
        let working_directory = self.working_directory();
        self.executor
            .spawn(async move {
                let output = new_smol_command(&git_binary_path)
                    .current_dir(working_directory?)
                    .args(&["stash", "list", "--pretty=format:%gd%x00%H%x00%ct%x00%s"])
                    .output()
                    .await?;
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.parse()
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("git status failed: {stderr}");
                }
            })
            .boxed()
    }

    fn branches(&self) -> BoxFuture<'_, Result<Vec<Branch>>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let fields = [
                    "%(HEAD)",
                    "%(objectname)",
                    "%(parent)",
                    "%(refname)",
                    "%(upstream)",
                    "%(upstream:track)",
                    "%(committerdate:unix)",
                    "%(authorname)",
                    "%(contents:subject)",
                ]
                .join("%00");
                let args = vec![
                    "for-each-ref",
                    "refs/heads/**/*",
                    "refs/remotes/**/*",
                    "--format",
                    &fields,
                ];
                let working_directory = working_directory?;
                let output = new_smol_command(&git_binary_path)
                    .current_dir(&working_directory)
                    .args(args)
                    .output()
                    .await?;

                anyhow::ensure!(
                    output.status.success(),
                    "Failed to git git branches:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );

                let input = String::from_utf8_lossy(&output.stdout);

                let mut branches = parse_branch_input(&input)?;
                if branches.is_empty() {
                    let args = vec!["symbolic-ref", "--quiet", "HEAD"];

                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .args(args)
                        .output()
                        .await?;

                    // git symbolic-ref returns a non-0 exit code if HEAD points
                    // to something other than a branch
                    if output.status.success() {
                        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();

                        branches.push(Branch {
                            ref_name: name.into(),
                            is_head: true,
                            upstream: None,
                            most_recent_commit: None,
                        });
                    }
                }

                Ok(branches)
            })
            .boxed()
    }

    fn change_branch(&self, name: String) -> BoxFuture<'_, Result<()>> {
        let repo = self.repository.clone();
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        let executor = self.executor.clone();
        let branch = self.executor.spawn(async move {
            let repo = repo.lock();
            let branch = if let Ok(branch) = repo.find_branch(&name, BranchType::Local) {
                branch
            } else if let Ok(revision) = repo.find_branch(&name, BranchType::Remote) {
                let (_, branch_name) = name.split_once("/").context("Unexpected branch format")?;
                let revision = revision.get();
                let branch_commit = revision.peel_to_commit()?;
                let mut branch = repo.branch(&branch_name, &branch_commit, false)?;
                branch.set_upstream(Some(&name))?;
                branch
            } else {
                anyhow::bail!("Branch '{}' not found", name);
            };

            Ok(branch
                .name()?
                .context("cannot checkout anonymous branch")?
                .to_string())
        });

        self.executor
            .spawn(async move {
                let branch = branch.await?;

                GitBinary::new(git_binary_path, working_directory?, executor)
                    .run(&["checkout", &branch])
                    .await?;
                anyhow::Ok(())
            })
            .boxed()
    }

    fn create_branch(&self, name: String) -> BoxFuture<'_, Result<()>> {
        let repo = self.repository.clone();
        self.executor
            .spawn(async move {
                let repo = repo.lock();
                let current_commit = repo.head()?.peel_to_commit()?;
                repo.branch(&name, &current_commit, false)?;
                Ok(())
            })
            .boxed()
    }

    fn rename_branch(&self, branch: String, new_name: String) -> BoxFuture<'_, Result<()>> {
        let git_binary_path = self.any_git_binary_path.clone();
        let working_directory = self.working_directory();
        let executor = self.executor.clone();

        self.executor
            .spawn(async move {
                GitBinary::new(git_binary_path, working_directory?, executor)
                    .run(&["branch", "-m", &branch, &new_name])
                    .await?;
                anyhow::Ok(())
            })
            .boxed()
    }

    fn blame(&self, path: RepoPath, content: Rope) -> BoxFuture<'_, Result<crate::blame::Blame>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();

        let remote_url = self
            .remote_url("upstream")
            .or_else(|| self.remote_url("origin"));

        async move {
            crate::blame::Blame::for_path(
                &git_binary_path,
                &working_directory?,
                &path,
                &content,
                remote_url,
            )
            .await
        }
        .boxed()
    }

    fn diff(&self, diff: DiffType) -> BoxFuture<'_, Result<String>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let args = match diff {
                    DiffType::HeadToIndex => Some("--staged"),
                    DiffType::HeadToWorktree => None,
                };

                let output = new_smol_command(&git_binary_path)
                    .current_dir(&working_directory?)
                    .args(["diff"])
                    .args(args)
                    .output()
                    .await?;

                anyhow::ensure!(
                    output.status.success(),
                    "Failed to run git diff:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            })
            .boxed()
    }

    fn stage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                if !paths.is_empty() {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory?)
                        .envs(env.iter())
                        .args(["update-index", "--add", "--remove", "--"])
                        .args(paths.iter().map(|p| p.as_unix_str()))
                        .output()
                        .await?;
                    anyhow::ensure!(
                        output.status.success(),
                        "Failed to stage paths:\n{}",
                        String::from_utf8_lossy(&output.stderr),
                    );
                }
                Ok(())
            })
            .boxed()
    }

    fn unstage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();

        self.executor
            .spawn(async move {
                if !paths.is_empty() {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory?)
                        .envs(env.iter())
                        .args(["reset", "--quiet", "--"])
                        .args(paths.iter().map(|p| p.as_std_path()))
                        .output()
                        .await?;

                    anyhow::ensure!(
                        output.status.success(),
                        "Failed to unstage:\n{}",
                        String::from_utf8_lossy(&output.stderr),
                    );
                }
                Ok(())
            })
            .boxed()
    }

    fn stash_paths(
        &self,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let mut cmd = new_smol_command(&git_binary_path);
                cmd.current_dir(&working_directory?)
                    .envs(env.iter())
                    .args(["stash", "push", "--quiet"])
                    .arg("--include-untracked");

                cmd.args(paths.iter().map(|p| p.as_unix_str()));

                let output = cmd.output().await?;

                anyhow::ensure!(
                    output.status.success(),
                    "Failed to stash:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                Ok(())
            })
            .boxed()
    }

    fn stash_pop(
        &self,
        index: Option<usize>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let mut cmd = new_smol_command(git_binary_path);
                let mut args = vec!["stash".to_string(), "pop".to_string()];
                if let Some(index) = index {
                    args.push(format!("stash@{{{}}}", index));
                }
                cmd.current_dir(&working_directory?)
                    .envs(env.iter())
                    .args(args);

                let output = cmd.output().await?;

                anyhow::ensure!(
                    output.status.success(),
                    "Failed to stash pop:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                Ok(())
            })
            .boxed()
    }

    fn stash_apply(
        &self,
        index: Option<usize>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let mut cmd = new_smol_command(git_binary_path);
                let mut args = vec!["stash".to_string(), "apply".to_string()];
                if let Some(index) = index {
                    args.push(format!("stash@{{{}}}", index));
                }
                cmd.current_dir(&working_directory?)
                    .envs(env.iter())
                    .args(args);

                let output = cmd.output().await?;

                anyhow::ensure!(
                    output.status.success(),
                    "Failed to apply stash:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                Ok(())
            })
            .boxed()
    }

    fn stash_drop(
        &self,
        index: Option<usize>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let mut cmd = new_smol_command(git_binary_path);
                let mut args = vec!["stash".to_string(), "drop".to_string()];
                if let Some(index) = index {
                    args.push(format!("stash@{{{}}}", index));
                }
                cmd.current_dir(&working_directory?)
                    .envs(env.iter())
                    .args(args);

                let output = cmd.output().await?;

                anyhow::ensure!(
                    output.status.success(),
                    "Failed to stash drop:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                Ok(())
            })
            .boxed()
    }

    fn commit(
        &self,
        message: SharedString,
        name_and_email: Option<(SharedString, SharedString)>,
        options: CommitOptions,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let mut cmd = new_smol_command(git_binary_path);
                cmd.current_dir(&working_directory?)
                    .envs(env.iter())
                    .args(["commit", "--quiet", "-m"])
                    .arg(&message.to_string())
                    .arg("--cleanup=strip");

                if options.amend {
                    cmd.arg("--amend");
                }

                if options.signoff {
                    cmd.arg("--signoff");
                }

                if let Some((name, email)) = name_and_email {
                    cmd.arg("--author").arg(&format!("{name} <{email}>"));
                }

                let output = cmd.output().await?;

                anyhow::ensure!(
                    output.status.success(),
                    "Failed to commit:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                Ok(())
            })
            .boxed()
    }

    fn push(
        &self,
        branch_name: String,
        remote_name: String,
        options: Option<PushOptions>,
        ask_pass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        cx: AsyncApp,
    ) -> BoxFuture<'_, Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        let executor = cx.background_executor().clone();
        let git_binary_path = self.system_git_binary_path.clone();
        async move {
            let git_binary_path = git_binary_path.context("git not found on $PATH, can't push")?;
            let working_directory = working_directory?;
            let mut command = new_smol_command(git_binary_path);
            command
                .envs(env.iter())
                .current_dir(&working_directory)
                .args(["push"])
                .args(options.map(|option| match option {
                    PushOptions::SetUpstream => "--set-upstream",
                    PushOptions::Force => "--force-with-lease",
                }))
                .arg(remote_name)
                .arg(format!("{}:{}", branch_name, branch_name))
                .stdin(smol::process::Stdio::null())
                .stdout(smol::process::Stdio::piped())
                .stderr(smol::process::Stdio::piped());

            run_git_command(env, ask_pass, command, &executor).await
        }
        .boxed()
    }

    fn pull(
        &self,
        branch_name: String,
        remote_name: String,
        ask_pass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        cx: AsyncApp,
    ) -> BoxFuture<'_, Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        let executor = cx.background_executor().clone();
        let git_binary_path = self.system_git_binary_path.clone();
        async move {
            let git_binary_path = git_binary_path.context("git not found on $PATH, can't pull")?;
            let mut command = new_smol_command(git_binary_path);
            command
                .envs(env.iter())
                .current_dir(&working_directory?)
                .args(["pull"])
                .arg(remote_name)
                .arg(branch_name)
                .stdout(smol::process::Stdio::piped())
                .stderr(smol::process::Stdio::piped());

            run_git_command(env, ask_pass, command, &executor).await
        }
        .boxed()
    }

    fn fetch(
        &self,
        fetch_options: FetchOptions,
        ask_pass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        cx: AsyncApp,
    ) -> BoxFuture<'_, Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        let remote_name = format!("{}", fetch_options);
        let git_binary_path = self.system_git_binary_path.clone();
        let executor = cx.background_executor().clone();
        async move {
            let git_binary_path = git_binary_path.context("git not found on $PATH, can't fetch")?;
            let mut command = new_smol_command(git_binary_path);
            command
                .envs(env.iter())
                .current_dir(&working_directory?)
                .args(["fetch", &remote_name])
                .stdout(smol::process::Stdio::piped())
                .stderr(smol::process::Stdio::piped());

            run_git_command(env, ask_pass, command, &executor).await
        }
        .boxed()
    }

    fn get_remotes(&self, branch_name: Option<String>) -> BoxFuture<'_, Result<Vec<Remote>>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                if let Some(branch_name) = branch_name {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .args(["config", "--get"])
                        .arg(format!("branch.{}.remote", branch_name))
                        .output()
                        .await?;

                    if output.status.success() {
                        let remote_name = String::from_utf8_lossy(&output.stdout);

                        return Ok(vec![Remote {
                            name: remote_name.trim().to_string().into(),
                        }]);
                    }
                }

                let output = new_smol_command(&git_binary_path)
                    .current_dir(&working_directory)
                    .args(["remote"])
                    .output()
                    .await?;

                anyhow::ensure!(
                    output.status.success(),
                    "Failed to get remotes:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                let remote_names = String::from_utf8_lossy(&output.stdout)
                    .split('\n')
                    .filter(|name| !name.is_empty())
                    .map(|name| Remote {
                        name: name.trim().to_string().into(),
                    })
                    .collect();
                Ok(remote_names)
            })
            .boxed()
    }

    fn check_for_pushed_commit(&self) -> BoxFuture<'_, Result<Vec<SharedString>>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let git_cmd = async |args: &[&str]| -> Result<String> {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .args(args)
                        .output()
                        .await?;
                    anyhow::ensure!(
                        output.status.success(),
                        String::from_utf8_lossy(&output.stderr).to_string()
                    );
                    Ok(String::from_utf8(output.stdout)?)
                };

                let head = git_cmd(&["rev-parse", "HEAD"])
                    .await
                    .context("Failed to get HEAD")?
                    .trim()
                    .to_owned();

                let mut remote_branches = vec![];
                let mut add_if_matching = async |remote_head: &str| {
                    if let Ok(merge_base) = git_cmd(&["merge-base", &head, remote_head]).await
                        && merge_base.trim() == head
                        && let Some(s) = remote_head.strip_prefix("refs/remotes/")
                    {
                        remote_branches.push(s.to_owned().into());
                    }
                };

                // check the main branch of each remote
                let remotes = git_cmd(&["remote"])
                    .await
                    .context("Failed to get remotes")?;
                for remote in remotes.lines() {
                    if let Ok(remote_head) =
                        git_cmd(&["symbolic-ref", &format!("refs/remotes/{remote}/HEAD")]).await
                    {
                        add_if_matching(remote_head.trim()).await;
                    }
                }

                // ... and the remote branch that the checked-out one is tracking
                if let Ok(remote_head) =
                    git_cmd(&["rev-parse", "--symbolic-full-name", "@{u}"]).await
                {
                    add_if_matching(remote_head.trim()).await;
                }

                Ok(remote_branches)
            })
            .boxed()
    }

    fn checkpoint(&self) -> BoxFuture<'static, Result<GitRepositoryCheckpoint>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();
        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let mut git = GitBinary::new(git_binary_path, working_directory.clone(), executor)
                    .envs(checkpoint_author_envs());
                git.with_temp_index(async |git| {
                    let head_sha = git.run(&["rev-parse", "HEAD"]).await.ok();
                    let mut excludes = exclude_files(git).await?;

                    git.run(&["add", "--all"]).await?;
                    let tree = git.run(&["write-tree"]).await?;
                    let checkpoint_sha = if let Some(head_sha) = head_sha.as_deref() {
                        git.run(&["commit-tree", &tree, "-p", head_sha, "-m", "Checkpoint"])
                            .await?
                    } else {
                        git.run(&["commit-tree", &tree, "-m", "Checkpoint"]).await?
                    };

                    excludes.restore_original().await?;

                    Ok(GitRepositoryCheckpoint {
                        commit_sha: checkpoint_sha.parse()?,
                    })
                })
                .await
            })
            .boxed()
    }

    fn restore_checkpoint(&self, checkpoint: GitRepositoryCheckpoint) -> BoxFuture<'_, Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;

                let git = GitBinary::new(git_binary_path, working_directory, executor);
                git.run(&[
                    "restore",
                    "--source",
                    &checkpoint.commit_sha.to_string(),
                    "--worktree",
                    ".",
                ])
                .await?;

                // TODO: We don't track binary and large files anymore,
                //       so the following call would delete them.
                //       Implement an alternative way to track files added by agent.
                //
                // git.with_temp_index(async move |git| {
                //     git.run(&["read-tree", &checkpoint.commit_sha.to_string()])
                //         .await?;
                //     git.run(&["clean", "-d", "--force"]).await
                // })
                // .await?;

                Ok(())
            })
            .boxed()
    }

    fn compare_checkpoints(
        &self,
        left: GitRepositoryCheckpoint,
        right: GitRepositoryCheckpoint,
    ) -> BoxFuture<'_, Result<bool>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let git = GitBinary::new(git_binary_path, working_directory, executor);
                let result = git
                    .run(&[
                        "diff-tree",
                        "--quiet",
                        &left.commit_sha.to_string(),
                        &right.commit_sha.to_string(),
                    ])
                    .await;
                match result {
                    Ok(_) => Ok(true),
                    Err(error) => {
                        if let Some(GitBinaryCommandError { status, .. }) =
                            error.downcast_ref::<GitBinaryCommandError>()
                            && status.code() == Some(1)
                        {
                            return Ok(false);
                        }

                        Err(error)
                    }
                }
            })
            .boxed()
    }

    fn diff_checkpoints(
        &self,
        base_checkpoint: GitRepositoryCheckpoint,
        target_checkpoint: GitRepositoryCheckpoint,
    ) -> BoxFuture<'_, Result<String>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let git = GitBinary::new(git_binary_path, working_directory, executor);
                git.run(&[
                    "diff",
                    "--find-renames",
                    "--patch",
                    &base_checkpoint.commit_sha.to_string(),
                    &target_checkpoint.commit_sha.to_string(),
                ])
                .await
            })
            .boxed()
    }

    fn default_branch(&self) -> BoxFuture<'_, Result<Option<SharedString>>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.any_git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let git = GitBinary::new(git_binary_path, working_directory, executor);

                if let Ok(output) = git
                    .run(&["symbolic-ref", "refs/remotes/upstream/HEAD"])
                    .await
                {
                    let output = output
                        .strip_prefix("refs/remotes/upstream/")
                        .map(|s| SharedString::from(s.to_owned()));
                    return Ok(output);
                }

                if let Ok(output) = git.run(&["symbolic-ref", "refs/remotes/origin/HEAD"]).await {
                    return Ok(output
                        .strip_prefix("refs/remotes/origin/")
                        .map(|s| SharedString::from(s.to_owned())));
                }

                if let Ok(default_branch) = git.run(&["config", "init.defaultBranch"]).await {
                    if git.run(&["rev-parse", &default_branch]).await.is_ok() {
                        return Ok(Some(default_branch.into()));
                    }
                }

                if git.run(&["rev-parse", "master"]).await.is_ok() {
                    return Ok(Some("master".into()));
                }

                Ok(None)
            })
            .boxed()
    }
}

fn git_status_args(path_prefixes: &[RepoPath]) -> Vec<OsString> {
    let mut args = vec![
        OsString::from("--no-optional-locks"),
        OsString::from("status"),
        OsString::from("--porcelain=v1"),
        OsString::from("--untracked-files=all"),
        OsString::from("--no-renames"),
        OsString::from("-z"),
    ];
    args.extend(path_prefixes.iter().map(|path_prefix| {
        if path_prefix.is_empty() {
            Path::new(".").into()
        } else {
            path_prefix.as_std_path().into()
        }
    }));
    args
}

/// Temporarily git-ignore commonly ignored files and files over 2MB
async fn exclude_files(git: &GitBinary) -> Result<GitExcludeOverride> {
    const MAX_SIZE: u64 = 2 * 1024 * 1024; // 2 MB
    let mut excludes = git.with_exclude_overrides().await?;
    excludes
        .add_excludes(include_str!("./checkpoint.gitignore"))
        .await?;

    let working_directory = git.working_directory.clone();
    let untracked_files = git.list_untracked_files().await?;
    let excluded_paths = untracked_files.into_iter().map(|path| {
        let working_directory = working_directory.clone();
        smol::spawn(async move {
            let full_path = working_directory.join(path.clone());
            match smol::fs::metadata(&full_path).await {
                Ok(metadata) if metadata.is_file() && metadata.len() >= MAX_SIZE => {
                    Some(PathBuf::from("/").join(path.clone()))
                }
                _ => None,
            }
        })
    });

    let excluded_paths = futures::future::join_all(excluded_paths).await;
    let excluded_paths = excluded_paths.into_iter().flatten().collect::<Vec<_>>();

    if !excluded_paths.is_empty() {
        let exclude_patterns = excluded_paths
            .into_iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("\n");
        excludes.add_excludes(&exclude_patterns).await?;
    }

    Ok(excludes)
}

struct GitBinary {
    git_binary_path: PathBuf,
    working_directory: PathBuf,
    executor: BackgroundExecutor,
    index_file_path: Option<PathBuf>,
    envs: HashMap<String, String>,
}

impl GitBinary {
    fn new(
        git_binary_path: PathBuf,
        working_directory: PathBuf,
        executor: BackgroundExecutor,
    ) -> Self {
        Self {
            git_binary_path,
            working_directory,
            executor,
            index_file_path: None,
            envs: HashMap::default(),
        }
    }

    async fn list_untracked_files(&self) -> Result<Vec<PathBuf>> {
        let status_output = self
            .run(&["status", "--porcelain=v1", "--untracked-files=all", "-z"])
            .await?;

        let paths = status_output
            .split('\0')
            .filter(|entry| entry.len() >= 3 && entry.starts_with("?? "))
            .map(|entry| PathBuf::from(&entry[3..]))
            .collect::<Vec<_>>();
        Ok(paths)
    }

    fn envs(mut self, envs: HashMap<String, String>) -> Self {
        self.envs = envs;
        self
    }

    pub async fn with_temp_index<R>(
        &mut self,
        f: impl AsyncFnOnce(&Self) -> Result<R>,
    ) -> Result<R> {
        let index_file_path = self.path_for_index_id(Uuid::new_v4());

        let delete_temp_index = util::defer({
            let index_file_path = index_file_path.clone();
            let executor = self.executor.clone();
            move || {
                executor
                    .spawn(async move {
                        smol::fs::remove_file(index_file_path).await.log_err();
                    })
                    .detach();
            }
        });

        // Copy the default index file so that Git doesn't have to rebuild the
        // whole index from scratch. This might fail if this is an empty repository.
        smol::fs::copy(
            self.working_directory.join(".git").join("index"),
            &index_file_path,
        )
        .await
        .ok();

        self.index_file_path = Some(index_file_path.clone());
        let result = f(self).await;
        self.index_file_path = None;
        let result = result?;

        smol::fs::remove_file(index_file_path).await.ok();
        delete_temp_index.abort();

        Ok(result)
    }

    pub async fn with_exclude_overrides(&self) -> Result<GitExcludeOverride> {
        let path = self
            .working_directory
            .join(".git")
            .join("info")
            .join("exclude");

        GitExcludeOverride::new(path).await
    }

    fn path_for_index_id(&self, id: Uuid) -> PathBuf {
        self.working_directory
            .join(".git")
            .join(format!("index-{}.tmp", id))
    }

    pub async fn run<S>(&self, args: impl IntoIterator<Item = S>) -> Result<String>
    where
        S: AsRef<OsStr>,
    {
        let mut stdout = self.run_raw(args).await?;
        if stdout.chars().last() == Some('\n') {
            stdout.pop();
        }
        Ok(stdout)
    }

    /// Returns the result of the command without trimming the trailing newline.
    pub async fn run_raw<S>(&self, args: impl IntoIterator<Item = S>) -> Result<String>
    where
        S: AsRef<OsStr>,
    {
        let mut command = self.build_command(args);
        let output = command.output().await?;
        anyhow::ensure!(
            output.status.success(),
            GitBinaryCommandError {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                status: output.status,
            }
        );
        Ok(String::from_utf8(output.stdout)?)
    }

    fn build_command<S>(&self, args: impl IntoIterator<Item = S>) -> smol::process::Command
    where
        S: AsRef<OsStr>,
    {
        let mut command = new_smol_command(&self.git_binary_path);
        command.current_dir(&self.working_directory);
        command.args(args);
        if let Some(index_file_path) = self.index_file_path.as_ref() {
            command.env("GIT_INDEX_FILE", index_file_path);
        }
        command.envs(&self.envs);
        command
    }
}

#[derive(Error, Debug)]
#[error("Git command failed:\n{stdout}{stderr}\n")]
struct GitBinaryCommandError {
    stdout: String,
    stderr: String,
    status: ExitStatus,
}

async fn run_git_command(
    env: Arc<HashMap<String, String>>,
    ask_pass: AskPassDelegate,
    mut command: smol::process::Command,
    executor: &BackgroundExecutor,
) -> Result<RemoteCommandOutput> {
    if env.contains_key("GIT_ASKPASS") {
        let git_process = command.spawn()?;
        let output = git_process.output().await?;
        anyhow::ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(RemoteCommandOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    } else {
        let ask_pass = AskPassSession::new(executor, ask_pass).await?;
        command
            .env("GIT_ASKPASS", ask_pass.script_path())
            .env("SSH_ASKPASS", ask_pass.script_path())
            .env("SSH_ASKPASS_REQUIRE", "force");
        let git_process = command.spawn()?;

        run_askpass_command(ask_pass, git_process).await
    }
}

async fn run_askpass_command(
    mut ask_pass: AskPassSession,
    git_process: smol::process::Child,
) -> anyhow::Result<RemoteCommandOutput> {
    select_biased! {
        result = ask_pass.run().fuse() => {
            match result {
                AskPassResult::CancelledByUser => {
                    Err(anyhow!(REMOTE_CANCELLED_BY_USER))?
                }
                AskPassResult::Timedout => {
                    Err(anyhow!("Connecting to host timed out"))?
                }
            }
        }
        output = git_process.output().fuse() => {
            let output = output?;
            anyhow::ensure!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            Ok(RemoteCommandOutput {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }
}

#[derive(Clone, Debug, Ord, Hash, PartialOrd, Eq, PartialEq)]
pub struct RepoPath(pub Arc<RelPath>);

impl RepoPath {
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> Result<Self> {
        let rel_path = RelPath::unix(s.as_ref())?;
        Ok(rel_path.into())
    }

    pub fn from_proto(proto: &str) -> Result<Self> {
        let rel_path = RelPath::from_proto(proto)?;
        Ok(rel_path.into())
    }

    pub fn from_std_path(path: &Path, path_style: PathStyle) -> Result<Self> {
        let rel_path = RelPath::new(path, path_style)?;
        Ok(Self(rel_path.as_ref().into()))
    }
}

#[cfg(any(test, feature = "test-support"))]
pub fn repo_path<S: AsRef<str> + ?Sized>(s: &S) -> RepoPath {
    RepoPath(RelPath::unix(s.as_ref()).unwrap().into())
}

impl From<&RelPath> for RepoPath {
    fn from(value: &RelPath) -> Self {
        RepoPath(value.into())
    }
}

impl<'a> From<Cow<'a, RelPath>> for RepoPath {
    fn from(value: Cow<'a, RelPath>) -> Self {
        value.as_ref().into()
    }
}

impl From<Arc<RelPath>> for RepoPath {
    fn from(value: Arc<RelPath>) -> Self {
        RepoPath(value)
    }
}

impl Default for RepoPath {
    fn default() -> Self {
        RepoPath(RelPath::empty().into())
    }
}

impl std::ops::Deref for RepoPath {
    type Target = RelPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl AsRef<Path> for RepoPath {
//     fn as_ref(&self) -> &Path {
//         RelPath::as_ref(&self.0)
//     }
// }

#[derive(Debug)]
pub struct RepoPathDescendants<'a>(pub &'a RepoPath);

impl MapSeekTarget<RepoPath> for RepoPathDescendants<'_> {
    fn cmp_cursor(&self, key: &RepoPath) -> Ordering {
        if key.starts_with(self.0) {
            Ordering::Greater
        } else {
            self.0.cmp(key)
        }
    }
}

fn parse_branch_input(input: &str) -> Result<Vec<Branch>> {
    let mut branches = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut fields = line.split('\x00');
        let is_current_branch = fields.next().context("no HEAD")? == "*";
        let head_sha: SharedString = fields.next().context("no objectname")?.to_string().into();
        let parent_sha: SharedString = fields.next().context("no parent")?.to_string().into();
        let ref_name = fields.next().context("no refname")?.to_string().into();
        let upstream_name = fields.next().context("no upstream")?.to_string();
        let upstream_tracking = parse_upstream_track(fields.next().context("no upstream:track")?)?;
        let commiterdate = fields.next().context("no committerdate")?.parse::<i64>()?;
        let author_name = fields.next().context("no authorname")?.to_string().into();
        let subject: SharedString = fields
            .next()
            .context("no contents:subject")?
            .to_string()
            .into();

        branches.push(Branch {
            is_head: is_current_branch,
            ref_name,
            most_recent_commit: Some(CommitSummary {
                sha: head_sha,
                subject,
                commit_timestamp: commiterdate,
                author_name: author_name,
                has_parent: !parent_sha.is_empty(),
            }),
            upstream: if upstream_name.is_empty() {
                None
            } else {
                Some(Upstream {
                    ref_name: upstream_name.into(),
                    tracking: upstream_tracking,
                })
            },
        })
    }

    Ok(branches)
}

fn parse_upstream_track(upstream_track: &str) -> Result<UpstreamTracking> {
    if upstream_track.is_empty() {
        return Ok(UpstreamTracking::Tracked(UpstreamTrackingStatus {
            ahead: 0,
            behind: 0,
        }));
    }

    let upstream_track = upstream_track.strip_prefix("[").context("missing [")?;
    let upstream_track = upstream_track.strip_suffix("]").context("missing [")?;
    let mut ahead: u32 = 0;
    let mut behind: u32 = 0;
    for component in upstream_track.split(", ") {
        if component == "gone" {
            return Ok(UpstreamTracking::Gone);
        }
        if let Some(ahead_num) = component.strip_prefix("ahead ") {
            ahead = ahead_num.parse::<u32>()?;
        }
        if let Some(behind_num) = component.strip_prefix("behind ") {
            behind = behind_num.parse::<u32>()?;
        }
    }
    Ok(UpstreamTracking::Tracked(UpstreamTrackingStatus {
        ahead,
        behind,
    }))
}

fn checkpoint_author_envs() -> HashMap<String, String> {
    HashMap::from_iter([
        ("GIT_AUTHOR_NAME".to_string(), "Zed".to_string()),
        ("GIT_AUTHOR_EMAIL".to_string(), "hi@zed.dev".to_string()),
        ("GIT_COMMITTER_NAME".to_string(), "Zed".to_string()),
        ("GIT_COMMITTER_EMAIL".to_string(), "hi@zed.dev".to_string()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_checkpoint_basic(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();

        git2::Repository::init(repo_dir.path()).unwrap();
        let file_path = repo_dir.path().join("file");
        smol::fs::write(&file_path, "initial").await.unwrap();

        let repo = RealGitRepository::new(
            &repo_dir.path().join(".git"),
            None,
            Some("git".into()),
            cx.executor(),
        )
        .unwrap();
        repo.stage_paths(vec![repo_path("file")], Arc::new(HashMap::default()))
            .await
            .unwrap();
        repo.commit(
            "Initial commit".into(),
            None,
            CommitOptions::default(),
            Arc::new(checkpoint_author_envs()),
        )
        .await
        .unwrap();

        smol::fs::write(&file_path, "modified before checkpoint")
            .await
            .unwrap();
        smol::fs::write(repo_dir.path().join("new_file_before_checkpoint"), "1")
            .await
            .unwrap();
        let checkpoint = repo.checkpoint().await.unwrap();

        // Ensure the user can't see any branches after creating a checkpoint.
        assert_eq!(repo.branches().await.unwrap().len(), 1);

        smol::fs::write(&file_path, "modified after checkpoint")
            .await
            .unwrap();
        repo.stage_paths(vec![repo_path("file")], Arc::new(HashMap::default()))
            .await
            .unwrap();
        repo.commit(
            "Commit after checkpoint".into(),
            None,
            CommitOptions::default(),
            Arc::new(checkpoint_author_envs()),
        )
        .await
        .unwrap();

        smol::fs::remove_file(repo_dir.path().join("new_file_before_checkpoint"))
            .await
            .unwrap();
        smol::fs::write(repo_dir.path().join("new_file_after_checkpoint"), "2")
            .await
            .unwrap();

        // Ensure checkpoint stays alive even after a Git GC.
        repo.gc().await.unwrap();
        repo.restore_checkpoint(checkpoint.clone()).await.unwrap();

        assert_eq!(
            smol::fs::read_to_string(&file_path).await.unwrap(),
            "modified before checkpoint"
        );
        assert_eq!(
            smol::fs::read_to_string(repo_dir.path().join("new_file_before_checkpoint"))
                .await
                .unwrap(),
            "1"
        );
        // See TODO above
        // assert_eq!(
        //     smol::fs::read_to_string(repo_dir.path().join("new_file_after_checkpoint"))
        //         .await
        //         .ok(),
        //     None
        // );
    }

    #[gpui::test]
    async fn test_checkpoint_empty_repo(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();
        git2::Repository::init(repo_dir.path()).unwrap();
        let repo = RealGitRepository::new(
            &repo_dir.path().join(".git"),
            None,
            Some("git".into()),
            cx.executor(),
        )
        .unwrap();

        smol::fs::write(repo_dir.path().join("foo"), "foo")
            .await
            .unwrap();
        let checkpoint_sha = repo.checkpoint().await.unwrap();

        // Ensure the user can't see any branches after creating a checkpoint.
        assert_eq!(repo.branches().await.unwrap().len(), 1);

        smol::fs::write(repo_dir.path().join("foo"), "bar")
            .await
            .unwrap();
        smol::fs::write(repo_dir.path().join("baz"), "qux")
            .await
            .unwrap();
        repo.restore_checkpoint(checkpoint_sha).await.unwrap();
        assert_eq!(
            smol::fs::read_to_string(repo_dir.path().join("foo"))
                .await
                .unwrap(),
            "foo"
        );
        // See TODOs above
        // assert_eq!(
        //     smol::fs::read_to_string(repo_dir.path().join("baz"))
        //         .await
        //         .ok(),
        //     None
        // );
    }

    #[gpui::test]
    async fn test_compare_checkpoints(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();
        git2::Repository::init(repo_dir.path()).unwrap();
        let repo = RealGitRepository::new(
            &repo_dir.path().join(".git"),
            None,
            Some("git".into()),
            cx.executor(),
        )
        .unwrap();

        smol::fs::write(repo_dir.path().join("file1"), "content1")
            .await
            .unwrap();
        let checkpoint1 = repo.checkpoint().await.unwrap();

        smol::fs::write(repo_dir.path().join("file2"), "content2")
            .await
            .unwrap();
        let checkpoint2 = repo.checkpoint().await.unwrap();

        assert!(
            !repo
                .compare_checkpoints(checkpoint1, checkpoint2.clone())
                .await
                .unwrap()
        );

        let checkpoint3 = repo.checkpoint().await.unwrap();
        assert!(
            repo.compare_checkpoints(checkpoint2, checkpoint3)
                .await
                .unwrap()
        );
    }

    #[gpui::test]
    async fn test_checkpoint_exclude_binary_files(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();
        let text_path = repo_dir.path().join("main.rs");
        let bin_path = repo_dir.path().join("binary.o");

        git2::Repository::init(repo_dir.path()).unwrap();

        smol::fs::write(&text_path, "fn main() {}").await.unwrap();

        smol::fs::write(&bin_path, "some binary file here")
            .await
            .unwrap();

        let repo = RealGitRepository::new(
            &repo_dir.path().join(".git"),
            None,
            Some("git".into()),
            cx.executor(),
        )
        .unwrap();

        // initial commit
        repo.stage_paths(vec![repo_path("main.rs")], Arc::new(HashMap::default()))
            .await
            .unwrap();
        repo.commit(
            "Initial commit".into(),
            None,
            CommitOptions::default(),
            Arc::new(checkpoint_author_envs()),
        )
        .await
        .unwrap();

        let checkpoint = repo.checkpoint().await.unwrap();

        smol::fs::write(&text_path, "fn main() { println!(\"Modified\"); }")
            .await
            .unwrap();
        smol::fs::write(&bin_path, "Modified binary file")
            .await
            .unwrap();

        repo.restore_checkpoint(checkpoint).await.unwrap();

        // Text files should be restored to checkpoint state,
        // but binaries should not (they aren't tracked)
        assert_eq!(
            smol::fs::read_to_string(&text_path).await.unwrap(),
            "fn main() {}"
        );

        assert_eq!(
            smol::fs::read_to_string(&bin_path).await.unwrap(),
            "Modified binary file"
        );
    }

    #[test]
    fn test_branches_parsing() {
        // suppress "help: octal escapes are not supported, `\0` is always null"
        #[allow(clippy::octal_escapes)]
        let input = "*\0060964da10574cd9bf06463a53bf6e0769c5c45e\0\0refs/heads/zed-patches\0refs/remotes/origin/zed-patches\0\01733187470\0John Doe\0generated protobuf\n";
        assert_eq!(
            parse_branch_input(input).unwrap(),
            vec![Branch {
                is_head: true,
                ref_name: "refs/heads/zed-patches".into(),
                upstream: Some(Upstream {
                    ref_name: "refs/remotes/origin/zed-patches".into(),
                    tracking: UpstreamTracking::Tracked(UpstreamTrackingStatus {
                        ahead: 0,
                        behind: 0
                    })
                }),
                most_recent_commit: Some(CommitSummary {
                    sha: "060964da10574cd9bf06463a53bf6e0769c5c45e".into(),
                    subject: "generated protobuf".into(),
                    commit_timestamp: 1733187470,
                    author_name: SharedString::new("John Doe"),
                    has_parent: false,
                })
            }]
        )
    }

    impl RealGitRepository {
        /// Force a Git garbage collection on the repository.
        fn gc(&self) -> BoxFuture<'_, Result<()>> {
            let working_directory = self.working_directory();
            let git_binary_path = self.any_git_binary_path.clone();
            let executor = self.executor.clone();
            self.executor
                .spawn(async move {
                    let git_binary_path = git_binary_path.clone();
                    let working_directory = working_directory?;
                    let git = GitBinary::new(git_binary_path, working_directory, executor);
                    git.run(&["gc", "--prune"]).await?;
                    Ok(())
                })
                .boxed()
        }
    }
}
