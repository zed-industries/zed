use crate::status::GitStatus;
use crate::{Oid, SHORT_SHA_LENGTH};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use futures::future::BoxFuture;
use futures::{select_biased, AsyncWriteExt, FutureExt as _};
use git2::BranchType;
use gpui::{AsyncApp, BackgroundExecutor, SharedString};
use parking_lot::Mutex;
use rope::Rope;
use schemars::JsonSchema;
use serde::Deserialize;
use std::borrow::{Borrow, Cow};
use std::ffi::{OsStr, OsString};
use std::path::Component;
use std::process::{ExitStatus, Stdio};
use std::sync::LazyLock;
use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
    sync::Arc,
};
use std::{future, mem};
use sum_tree::MapSeekTarget;
use thiserror::Error;
use util::command::{new_smol_command, new_std_command};
use util::ResultExt;
use uuid::Uuid;

pub use askpass::{AskPassResult, AskPassSession};

pub const REMOTE_CANCELLED_BY_USER: &str = "Operation cancelled by user";

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Branch {
    pub is_head: bool,
    pub name: SharedString,
    pub upstream: Option<Upstream>,
    pub most_recent_commit: Option<CommitSummary>,
}

impl Branch {
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
    pub fn remote_name(&self) -> Option<&str> {
        self.ref_name
            .strip_prefix("refs/remotes/")
            .and_then(|stripped| stripped.split("/").next())
    }
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
    pub has_parent: bool,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CommitDetails {
    pub sha: SharedString,
    pub message: SharedString,
    pub commit_timestamp: i64,
    pub committer_email: SharedString,
    pub committer_name: SharedString,
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
    // reset the branch pointer, leave index and worktree unchanged
    // (this will make it look like things that were committed are now
    // staged)
    Soft,
    // reset the branch pointer and index, leave worktree unchanged
    // (this makes it look as though things that were committed are now
    // unstaged)
    Mixed,
}

pub trait GitRepository: Send + Sync {
    fn reload_index(&self);

    /// Returns the contents of an entry in the repository's index, or None if there is no entry for the given path.
    ///
    /// Also returns `None` for symlinks.
    fn load_index_text(&self, index: Option<GitIndex>, path: RepoPath)
        -> BoxFuture<Option<String>>;

    /// Returns the contents of an entry in the repository's HEAD, or None if HEAD does not exist or has no entry for the given path.
    ///
    /// Also returns `None` for symlinks.
    fn load_committed_text(&self, path: RepoPath) -> BoxFuture<Option<String>>;

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        env: HashMap<String, String>,
    ) -> BoxFuture<anyhow::Result<()>>;

    /// Returns the URL of the remote with the given name.
    fn remote_url(&self, name: &str) -> Option<String>;

    /// Returns the SHA of the current HEAD.
    fn head_sha(&self) -> Option<String>;

    fn merge_head_shas(&self) -> Vec<String>;

    fn status(
        &self,
        index: Option<GitIndex>,
        path_prefixes: &[RepoPath],
    ) -> BoxFuture<'static, Result<GitStatus>>;
    fn status_blocking(&self, path_prefixes: &[RepoPath]) -> Result<GitStatus>;

    fn branches(&self) -> BoxFuture<Result<Vec<Branch>>>;

    fn change_branch(&self, name: String) -> BoxFuture<Result<()>>;
    fn create_branch(&self, name: String) -> BoxFuture<Result<()>>;

    fn reset(
        &self,
        commit: String,
        mode: ResetMode,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>>;

    fn checkout_files(
        &self,
        commit: String,
        paths: Vec<RepoPath>,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>>;

    fn show(&self, commit: String) -> BoxFuture<Result<CommitDetails>>;

    fn blame(&self, path: RepoPath, content: Rope) -> BoxFuture<Result<crate::blame::Blame>>;

    /// Returns the absolute path to the repository. For worktrees, this will be the path to the
    /// worktree's gitdir within the main repository (typically `.git/worktrees/<name>`).
    fn path(&self) -> PathBuf;

    /// Returns the absolute path to the ".git" dir for the main repository, typically a `.git`
    /// folder. For worktrees, this will be the path to the repository the worktree was created
    /// from. Otherwise, this is the same value as `path()`.
    ///
    /// Git documentation calls this the "commondir", and for git CLI is overridden by
    /// `GIT_COMMON_DIR`.
    fn main_repository_path(&self) -> PathBuf;

    /// Updates the index to match the worktree at the given paths.
    ///
    /// If any of the paths have been deleted from the worktree, they will be removed from the index if found there.
    fn stage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>>;
    /// Updates the index to match HEAD at the given paths.
    ///
    /// If any of the paths were previously staged but do not exist in HEAD, they will be removed from the index.
    fn unstage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>>;

    fn commit(
        &self,
        message: SharedString,
        name_and_email: Option<(SharedString, SharedString)>,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>>;

    fn push(
        &self,
        branch_name: String,
        upstream_name: String,
        options: Option<PushOptions>,
        askpass: AskPassSession,
        env: HashMap<String, String>,
        // This method takes an AsyncApp to ensure it's invoked on the main thread,
        // otherwise git-credentials-manager won't work.
        cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>>;

    fn pull(
        &self,
        branch_name: String,
        upstream_name: String,
        askpass: AskPassSession,
        env: HashMap<String, String>,
        // This method takes an AsyncApp to ensure it's invoked on the main thread,
        // otherwise git-credentials-manager won't work.
        cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>>;

    fn fetch(
        &self,
        askpass: AskPassSession,
        env: HashMap<String, String>,
        // This method takes an AsyncApp to ensure it's invoked on the main thread,
        // otherwise git-credentials-manager won't work.
        cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>>;

    fn get_remotes(&self, branch_name: Option<String>) -> BoxFuture<Result<Vec<Remote>>>;

    /// returns a list of remote branches that contain HEAD
    fn check_for_pushed_commit(&self) -> BoxFuture<Result<Vec<SharedString>>>;

    /// Run git diff
    fn diff(&self, diff: DiffType) -> BoxFuture<Result<String>>;

    /// Creates a checkpoint for the repository.
    fn checkpoint(&self) -> BoxFuture<'static, Result<GitRepositoryCheckpoint>>;

    /// Resets to a previously-created checkpoint.
    fn restore_checkpoint(&self, checkpoint: GitRepositoryCheckpoint) -> BoxFuture<Result<()>>;

    /// Compares two checkpoints, returning true if they are equal
    fn compare_checkpoints(
        &self,
        left: GitRepositoryCheckpoint,
        right: GitRepositoryCheckpoint,
    ) -> BoxFuture<Result<bool>>;

    /// Deletes a previously-created checkpoint.
    fn delete_checkpoint(&self, checkpoint: GitRepositoryCheckpoint) -> BoxFuture<Result<()>>;

    /// Computes a diff between two checkpoints.
    fn diff_checkpoints(
        &self,
        base_checkpoint: GitRepositoryCheckpoint,
        target_checkpoint: GitRepositoryCheckpoint,
    ) -> BoxFuture<Result<String>>;

    /// Creates a new index for the repository.
    fn create_index(&self) -> BoxFuture<Result<GitIndex>>;

    /// Applies a diff to the repository's index.
    fn apply_diff(&self, index: GitIndex, diff: String) -> BoxFuture<Result<()>>;
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
    pub git_binary_path: PathBuf,
    executor: BackgroundExecutor,
}

impl RealGitRepository {
    pub fn new(
        dotgit_path: &Path,
        git_binary_path: Option<PathBuf>,
        executor: BackgroundExecutor,
    ) -> Option<Self> {
        let workdir_root = dotgit_path.parent()?;
        let repository = git2::Repository::open(workdir_root).log_err()?;
        Some(Self {
            repository: Arc::new(Mutex::new(repository)),
            git_binary_path: git_binary_path.unwrap_or_else(|| PathBuf::from("git")),
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
    ref_name: String,
    head_sha: Option<Oid>,
    commit_sha: Oid,
}

#[derive(Copy, Clone, Debug)]
pub struct GitIndex {
    id: Uuid,
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

    fn show(&self, commit: String) -> BoxFuture<Result<CommitDetails>> {
        let repo = self.repository.clone();
        self.executor
            .spawn(async move {
                let repo = repo.lock();
                let Ok(commit) = repo.revparse_single(&commit)?.into_commit() else {
                    anyhow::bail!("{} is not a commit", commit);
                };
                let details = CommitDetails {
                    sha: commit.id().to_string().into(),
                    message: String::from_utf8_lossy(commit.message_raw_bytes())
                        .to_string()
                        .into(),
                    commit_timestamp: commit.time().seconds(),
                    committer_email: String::from_utf8_lossy(commit.committer().email_bytes())
                        .to_string()
                        .into(),
                    committer_name: String::from_utf8_lossy(commit.committer().name_bytes())
                        .to_string()
                        .into(),
                };
                Ok(details)
            })
            .boxed()
    }

    fn reset(
        &self,
        commit: String,
        mode: ResetMode,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>> {
        async move {
            let working_directory = self.working_directory();

            let mode_flag = match mode {
                ResetMode::Mixed => "--mixed",
                ResetMode::Soft => "--soft",
            };

            let output = new_smol_command(&self.git_binary_path)
                .envs(env)
                .current_dir(&working_directory?)
                .args(["reset", mode_flag, &commit])
                .output()
                .await?;
            if !output.status.success() {
                return Err(anyhow!(
                    "Failed to reset:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Ok(())
        }
        .boxed()
    }

    fn checkout_files(
        &self,
        commit: String,
        paths: Vec<RepoPath>,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        async move {
            if paths.is_empty() {
                return Ok(());
            }

            let output = new_smol_command(&git_binary_path)
                .current_dir(&working_directory?)
                .envs(env)
                .args(["checkout", &commit, "--"])
                .args(paths.iter().map(|path| path.as_ref()))
                .output()
                .await?;
            if !output.status.success() {
                return Err(anyhow!(
                    "Failed to checkout files:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Ok(())
        }
        .boxed()
    }

    fn load_index_text(
        &self,
        index: Option<GitIndex>,
        path: RepoPath,
    ) -> BoxFuture<Option<String>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                match check_path_to_repo_path_errors(&path) {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("Error with repo path: {:?}", err);
                        return None;
                    }
                }

                let working_directory = match working_directory {
                    Ok(dir) => dir,
                    Err(err) => {
                        log::error!("Error getting working directory: {:?}", err);
                        return None;
                    }
                };

                let mut git = GitBinary::new(git_binary_path, working_directory, executor);
                let text = git
                    .with_option_index(index, async |git| {
                        // First check if the file is a symlink using ls-files
                        let ls_files_output = git
                            .run(&[
                                OsStr::new("ls-files"),
                                OsStr::new("--stage"),
                                path.to_unix_style().as_ref(),
                            ])
                            .await
                            .context("error running ls-files")?;

                        // Parse ls-files output to check if it's a symlink
                        // Format is: "100644 <sha> 0 <filename>" where 100644 is the mode
                        if ls_files_output.is_empty() {
                            return Ok(None); // File not in index
                        }

                        let parts: Vec<&str> = ls_files_output.split_whitespace().collect();
                        if parts.len() < 2 {
                            return Err(anyhow!(
                                "unexpected ls-files output format: {}",
                                ls_files_output
                            ));
                        }

                        // Check if it's a symlink (120000 mode)
                        if parts[0] == "120000" {
                            return Ok(None);
                        }

                        let sha = parts[1];

                        // Now get the content
                        Ok(Some(
                            git.run_raw(&["cat-file", "blob", sha])
                                .await
                                .context("error getting blob content")?,
                        ))
                    })
                    .await;

                match text {
                    Ok(text) => text,
                    Err(error) => {
                        log::error!("Error getting text: {}", error);
                        None
                    }
                }
            })
            .boxed()
    }

    fn load_committed_text(&self, path: RepoPath) -> BoxFuture<Option<String>> {
        let repo = self.repository.clone();
        self.executor
            .spawn(async move {
                let repo = repo.lock();
                let head = repo.head().ok()?.peel_to_tree().log_err()?;
                let entry = head.get_path(&path).ok()?;
                if entry.filemode() == i32::from(git2::FileMode::Link) {
                    return None;
                }
                let content = repo.find_blob(entry.id()).log_err()?.content().to_owned();
                let content = String::from_utf8(content).log_err()?;
                Some(content)
            })
            .boxed()
    }

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        env: HashMap<String, String>,
    ) -> BoxFuture<anyhow::Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                if let Some(content) = content {
                    let mut child = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .envs(&env)
                        .args(["hash-object", "-w", "--stdin"])
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()?;
                    child
                        .stdin
                        .take()
                        .unwrap()
                        .write_all(content.as_bytes())
                        .await?;
                    let output = child.output().await?.stdout;
                    let sha = String::from_utf8(output)?;

                    log::debug!("indexing SHA: {sha}, path {path:?}");

                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .envs(env)
                        .args(["update-index", "--add", "--cacheinfo", "100644", &sha])
                        .arg(path.to_unix_style())
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(anyhow!(
                            "Failed to stage:\n{}",
                            String::from_utf8_lossy(&output.stderr)
                        ));
                    }
                } else {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .envs(env)
                        .args(["update-index", "--force-remove"])
                        .arg(path.to_unix_style())
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(anyhow!(
                            "Failed to unstage:\n{}",
                            String::from_utf8_lossy(&output.stderr)
                        ));
                    }
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

    fn head_sha(&self) -> Option<String> {
        Some(self.repository.lock().head().ok()?.target()?.to_string())
    }

    fn merge_head_shas(&self) -> Vec<String> {
        let mut shas = Vec::default();
        self.repository
            .lock()
            .mergehead_foreach(|oid| {
                shas.push(oid.to_string());
                true
            })
            .ok();
        if let Some(oid) = self
            .repository
            .lock()
            .find_reference("CHERRY_PICK_HEAD")
            .ok()
            .and_then(|reference| reference.target())
        {
            shas.push(oid.to_string())
        }
        shas
    }

    fn status(
        &self,
        index: Option<GitIndex>,
        path_prefixes: &[RepoPath],
    ) -> BoxFuture<'static, Result<GitStatus>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        let executor = self.executor.clone();
        let mut args = vec![
            OsString::from("--no-optional-locks"),
            OsString::from("status"),
            OsString::from("--porcelain=v1"),
            OsString::from("--untracked-files=all"),
            OsString::from("--no-renames"),
            OsString::from("-z"),
        ];
        args.extend(path_prefixes.iter().map(|path_prefix| {
            if path_prefix.0.as_ref() == Path::new("") {
                Path::new(".").into()
            } else {
                path_prefix.as_os_str().into()
            }
        }));
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let mut git = GitBinary::new(git_binary_path, working_directory, executor);
                git.with_option_index(index, async |git| git.run(&args).await)
                    .await?
                    .parse()
            })
            .boxed()
    }

    fn status_blocking(&self, path_prefixes: &[RepoPath]) -> Result<GitStatus> {
        let output = new_std_command(&self.git_binary_path)
            .current_dir(self.working_directory()?)
            .args(git_status_args(path_prefixes))
            .output()?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.parse()
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("git status failed: {}", stderr))
        }
    }

    fn branches(&self) -> BoxFuture<Result<Vec<Branch>>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        async move {
            let fields = [
                "%(HEAD)",
                "%(objectname)",
                "%(parent)",
                "%(refname)",
                "%(upstream)",
                "%(upstream:track)",
                "%(committerdate:unix)",
                "%(contents:subject)",
            ]
            .join("%00");
            let args = vec!["for-each-ref", "refs/heads/**/*", "--format", &fields];
            let working_directory = working_directory?;
            let output = new_smol_command(&git_binary_path)
                .current_dir(&working_directory)
                .args(args)
                .output()
                .await?;

            if !output.status.success() {
                return Err(anyhow!(
                    "Failed to git git branches:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            let input = String::from_utf8_lossy(&output.stdout);

            let mut branches = parse_branch_input(&input)?;
            if branches.is_empty() {
                let args = vec!["symbolic-ref", "--quiet", "--short", "HEAD"];

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
                        name: name.into(),
                        is_head: true,
                        upstream: None,
                        most_recent_commit: None,
                    });
                }
            }

            Ok(branches)
        }
        .boxed()
    }

    fn change_branch(&self, name: String) -> BoxFuture<Result<()>> {
        let repo = self.repository.clone();
        self.executor
            .spawn(async move {
                let repo = repo.lock();
                let revision = repo.find_branch(&name, BranchType::Local)?;
                let revision = revision.get();
                let as_tree = revision.peel_to_tree()?;
                repo.checkout_tree(as_tree.as_object(), None)?;
                repo.set_head(
                    revision
                        .name()
                        .ok_or_else(|| anyhow!("Branch name could not be retrieved"))?,
                )?;
                Ok(())
            })
            .boxed()
    }

    fn create_branch(&self, name: String) -> BoxFuture<Result<()>> {
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

    fn blame(&self, path: RepoPath, content: Rope) -> BoxFuture<Result<crate::blame::Blame>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

        const REMOTE_NAME: &str = "origin";
        let remote_url = self.remote_url(REMOTE_NAME);

        self.executor
            .spawn(async move {
                crate::blame::Blame::for_path(
                    &git_binary_path,
                    &working_directory?,
                    &path,
                    &content,
                    remote_url,
                )
                .await
            })
            .boxed()
    }

    fn diff(&self, diff: DiffType) -> BoxFuture<Result<String>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
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

                if !output.status.success() {
                    return Err(anyhow!(
                        "Failed to run git diff:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            })
            .boxed()
    }

    fn stage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        self.executor
            .spawn(async move {
                if !paths.is_empty() {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory?)
                        .envs(env)
                        .args(["update-index", "--add", "--remove", "--"])
                        .args(paths.iter().map(|p| p.to_unix_style()))
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(anyhow!(
                            "Failed to stage paths:\n{}",
                            String::from_utf8_lossy(&output.stderr)
                        ));
                    }
                }
                Ok(())
            })
            .boxed()
    }

    fn unstage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

        self.executor
            .spawn(async move {
                if !paths.is_empty() {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory?)
                        .envs(env)
                        .args(["reset", "--quiet", "--"])
                        .args(paths.iter().map(|p| p.as_ref()))
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(anyhow!(
                            "Failed to unstage:\n{}",
                            String::from_utf8_lossy(&output.stderr)
                        ));
                    }
                }
                Ok(())
            })
            .boxed()
    }

    fn commit(
        &self,
        message: SharedString,
        name_and_email: Option<(SharedString, SharedString)>,
        env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        self.executor
            .spawn(async move {
                let mut cmd = new_smol_command("git");
                cmd.current_dir(&working_directory?)
                    .envs(env)
                    .args(["commit", "--quiet", "-m"])
                    .arg(&message.to_string())
                    .arg("--cleanup=strip");

                if let Some((name, email)) = name_and_email {
                    cmd.arg("--author").arg(&format!("{name} <{email}>"));
                }

                let output = cmd.output().await?;

                if !output.status.success() {
                    return Err(anyhow!(
                        "Failed to commit:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
                Ok(())
            })
            .boxed()
    }

    fn push(
        &self,
        branch_name: String,
        remote_name: String,
        options: Option<PushOptions>,
        ask_pass: AskPassSession,
        env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        async move {
            let working_directory = working_directory?;

            let mut command = new_smol_command("git");
            command
                .envs(env)
                .env("GIT_ASKPASS", ask_pass.script_path())
                .env("SSH_ASKPASS", ask_pass.script_path())
                .env("SSH_ASKPASS_REQUIRE", "force")
                .env("GIT_HTTP_USER_AGENT", "Zed")
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
            let git_process = command.spawn()?;

            run_remote_command(ask_pass, git_process).await
        }
        .boxed()
    }

    fn pull(
        &self,
        branch_name: String,
        remote_name: String,
        ask_pass: AskPassSession,
        env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        async {
            let mut command = new_smol_command("git");
            command
                .envs(env)
                .env("GIT_ASKPASS", ask_pass.script_path())
                .env("SSH_ASKPASS", ask_pass.script_path())
                .env("SSH_ASKPASS_REQUIRE", "force")
                .current_dir(&working_directory?)
                .args(["pull"])
                .arg(remote_name)
                .arg(branch_name)
                .stdout(smol::process::Stdio::piped())
                .stderr(smol::process::Stdio::piped());
            let git_process = command.spawn()?;

            run_remote_command(ask_pass, git_process).await
        }
        .boxed()
    }

    fn fetch(
        &self,
        ask_pass: AskPassSession,
        env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        async {
            let mut command = new_smol_command("git");
            command
                .envs(env)
                .env("GIT_ASKPASS", ask_pass.script_path())
                .env("SSH_ASKPASS", ask_pass.script_path())
                .env("SSH_ASKPASS_REQUIRE", "force")
                .current_dir(&working_directory?)
                .args(["fetch", "--all"])
                .stdout(smol::process::Stdio::piped())
                .stderr(smol::process::Stdio::piped());
            let git_process = command.spawn()?;

            run_remote_command(ask_pass, git_process).await
        }
        .boxed()
    }

    fn get_remotes(&self, branch_name: Option<String>) -> BoxFuture<Result<Vec<Remote>>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
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

                if output.status.success() {
                    let remote_names = String::from_utf8_lossy(&output.stdout)
                        .split('\n')
                        .filter(|name| !name.is_empty())
                        .map(|name| Remote {
                            name: name.trim().to_string().into(),
                        })
                        .collect();

                    return Ok(remote_names);
                } else {
                    return Err(anyhow!(
                        "Failed to get remotes:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            })
            .boxed()
    }

    fn check_for_pushed_commit(&self) -> BoxFuture<Result<Vec<SharedString>>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let git_cmd = async |args: &[&str]| -> Result<String> {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory)
                        .args(args)
                        .output()
                        .await?;
                    if output.status.success() {
                        Ok(String::from_utf8(output.stdout)?)
                    } else {
                        Err(anyhow!(String::from_utf8_lossy(&output.stderr).to_string()))
                    }
                };

                let head = git_cmd(&["rev-parse", "HEAD"])
                    .await
                    .context("Failed to get HEAD")?
                    .trim()
                    .to_owned();

                let mut remote_branches = vec![];
                let mut add_if_matching = async |remote_head: &str| {
                    if let Ok(merge_base) = git_cmd(&["merge-base", &head, remote_head]).await {
                        if merge_base.trim() == head {
                            if let Some(s) = remote_head.strip_prefix("refs/remotes/") {
                                remote_branches.push(s.to_owned().into());
                            }
                        }
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
        let git_binary_path = self.git_binary_path.clone();
        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let mut git = GitBinary::new(git_binary_path, working_directory, executor)
                    .envs(checkpoint_author_envs());
                git.with_temp_index(async |git| {
                    let head_sha = git.run(&["rev-parse", "HEAD"]).await.ok();
                    git.run(&["add", "--all"]).await?;
                    let tree = git.run(&["write-tree"]).await?;
                    let checkpoint_sha = if let Some(head_sha) = head_sha.as_deref() {
                        git.run(&["commit-tree", &tree, "-p", head_sha, "-m", "Checkpoint"])
                            .await?
                    } else {
                        git.run(&["commit-tree", &tree, "-m", "Checkpoint"]).await?
                    };
                    let ref_name = format!("refs/zed/{}", Uuid::new_v4());
                    git.run(&["update-ref", &ref_name, &checkpoint_sha]).await?;

                    Ok(GitRepositoryCheckpoint {
                        ref_name,
                        head_sha: if let Some(head_sha) = head_sha {
                            Some(head_sha.parse()?)
                        } else {
                            None
                        },
                        commit_sha: checkpoint_sha.parse()?,
                    })
                })
                .await
            })
            .boxed()
    }

    fn restore_checkpoint(&self, checkpoint: GitRepositoryCheckpoint) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;

                let mut git = GitBinary::new(git_binary_path, working_directory, executor);
                git.run(&[
                    "restore",
                    "--source",
                    &checkpoint.commit_sha.to_string(),
                    "--worktree",
                    ".",
                ])
                .await?;

                git.with_temp_index(async move |git| {
                    git.run(&["read-tree", &checkpoint.commit_sha.to_string()])
                        .await?;
                    git.run(&["clean", "-d", "--force"]).await
                })
                .await?;

                if let Some(head_sha) = checkpoint.head_sha {
                    git.run(&["reset", "--mixed", &head_sha.to_string()])
                        .await?;
                } else {
                    git.run(&["update-ref", "-d", "HEAD"]).await?;
                }

                Ok(())
            })
            .boxed()
    }

    fn compare_checkpoints(
        &self,
        left: GitRepositoryCheckpoint,
        right: GitRepositoryCheckpoint,
    ) -> BoxFuture<Result<bool>> {
        if left.head_sha != right.head_sha {
            return future::ready(Ok(false)).boxed();
        }

        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

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
                        {
                            if status.code() == Some(1) {
                                return Ok(false);
                            }
                        }

                        Err(error)
                    }
                }
            })
            .boxed()
    }

    fn delete_checkpoint(&self, checkpoint: GitRepositoryCheckpoint) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let git = GitBinary::new(git_binary_path, working_directory, executor);
                git.run(&["update-ref", "-d", &checkpoint.ref_name]).await?;
                Ok(())
            })
            .boxed()
    }

    fn diff_checkpoints(
        &self,
        base_checkpoint: GitRepositoryCheckpoint,
        target_checkpoint: GitRepositoryCheckpoint,
    ) -> BoxFuture<Result<String>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let git = GitBinary::new(git_binary_path, working_directory, executor);
                git.run(&[
                    "diff",
                    "--find-renames",
                    "--patch",
                    &base_checkpoint.ref_name,
                    &target_checkpoint.ref_name,
                ])
                .await
            })
            .boxed()
    }

    fn create_index(&self) -> BoxFuture<Result<GitIndex>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let mut git = GitBinary::new(git_binary_path, working_directory, executor);
                let index = GitIndex { id: Uuid::new_v4() };
                git.with_index(index, async move |git| git.run(&["add", "--all"]).await)
                    .await?;
                Ok(index)
            })
            .boxed()
    }

    fn apply_diff(&self, index: GitIndex, diff: String) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

        let executor = self.executor.clone();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let mut git = GitBinary::new(git_binary_path, working_directory, executor);
                git.with_index(index, async move |git| {
                    git.run_with_stdin(&["apply", "--cached", "-"], diff).await
                })
                .await?;
                Ok(())
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
        if path_prefix.0.as_ref() == Path::new("") {
            Path::new(".").into()
        } else {
            path_prefix.as_os_str().into()
        }
    }));
    args
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

    fn envs(mut self, envs: HashMap<String, String>) -> Self {
        self.envs = envs;
        self
    }

    pub async fn with_temp_index<R>(
        &mut self,
        f: impl AsyncFnOnce(&Self) -> Result<R>,
    ) -> Result<R> {
        let index_file_path = self.path_for_index(GitIndex { id: Uuid::new_v4() });

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

        self.index_file_path = Some(index_file_path.clone());
        let result = f(self).await;
        self.index_file_path = None;
        let result = result?;

        smol::fs::remove_file(index_file_path).await.ok();
        delete_temp_index.abort();

        Ok(result)
    }

    pub async fn with_index<R>(
        &mut self,
        index: GitIndex,
        f: impl AsyncFnOnce(&Self) -> Result<R>,
    ) -> Result<R> {
        self.with_option_index(Some(index), f).await
    }

    pub async fn with_option_index<R>(
        &mut self,
        index: Option<GitIndex>,
        f: impl AsyncFnOnce(&Self) -> Result<R>,
    ) -> Result<R> {
        let new_index_path = index.map(|index| self.path_for_index(index));
        let old_index_path = mem::replace(&mut self.index_file_path, new_index_path);
        let result = f(self).await;
        self.index_file_path = old_index_path;
        result
    }

    fn path_for_index(&self, index: GitIndex) -> PathBuf {
        self.working_directory
            .join(".git")
            .join(format!("index-{}.tmp", index.id))
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
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            Err(anyhow!(GitBinaryCommandError {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                status: output.status,
            }))
        }
    }

    pub async fn run_with_stdin(&self, args: &[&str], stdin: String) -> Result<String> {
        let mut command = self.build_command(args);
        command.stdin(Stdio::piped());
        let mut child = command.spawn()?;

        let mut child_stdin = child.stdin.take().context("failed to write to stdin")?;
        child_stdin.write_all(stdin.as_bytes()).await?;
        drop(child_stdin);

        let output = child.output().await?;
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?.trim_end().to_string())
        } else {
            Err(anyhow!(GitBinaryCommandError {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                status: output.status,
            }))
        }
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
#[error("Git command failed: {stdout}")]
struct GitBinaryCommandError {
    stdout: String,
    status: ExitStatus,
}

async fn run_remote_command(
    mut ask_pass: AskPassSession,
    git_process: smol::process::Child,
) -> std::result::Result<RemoteCommandOutput, anyhow::Error> {
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
            if !output.status.success() {
                Err(anyhow!(
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            } else {
                Ok(RemoteCommandOutput {
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                })
            }
        }
    }
}

pub static WORK_DIRECTORY_REPO_PATH: LazyLock<RepoPath> =
    LazyLock::new(|| RepoPath(Path::new("").into()));

#[derive(Clone, Debug, Ord, Hash, PartialOrd, Eq, PartialEq)]
pub struct RepoPath(pub Arc<Path>);

impl RepoPath {
    pub fn new(path: PathBuf) -> Self {
        debug_assert!(path.is_relative(), "Repo paths must be relative");

        RepoPath(path.into())
    }

    pub fn from_str(path: &str) -> Self {
        let path = Path::new(path);
        debug_assert!(path.is_relative(), "Repo paths must be relative");

        RepoPath(path.into())
    }

    pub fn to_unix_style(&self) -> Cow<'_, OsStr> {
        #[cfg(target_os = "windows")]
        {
            use std::ffi::OsString;

            let path = self.0.as_os_str().to_string_lossy().replace("\\", "/");
            Cow::Owned(OsString::from(path))
        }
        #[cfg(not(target_os = "windows"))]
        {
            Cow::Borrowed(self.0.as_os_str())
        }
    }
}

impl std::fmt::Display for RepoPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.to_string_lossy().fmt(f)
    }
}

impl From<&Path> for RepoPath {
    fn from(value: &Path) -> Self {
        RepoPath::new(value.into())
    }
}

impl From<Arc<Path>> for RepoPath {
    fn from(value: Arc<Path>) -> Self {
        RepoPath(value)
    }
}

impl From<PathBuf> for RepoPath {
    fn from(value: PathBuf) -> Self {
        RepoPath::new(value)
    }
}

impl From<&str> for RepoPath {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl Default for RepoPath {
    fn default() -> Self {
        RepoPath(Path::new("").into())
    }
}

impl AsRef<Path> for RepoPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl std::ops::Deref for RepoPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<Path> for RepoPath {
    fn borrow(&self) -> &Path {
        self.0.as_ref()
    }
}

#[derive(Debug)]
pub struct RepoPathDescendants<'a>(pub &'a Path);

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
        let ref_name: SharedString = fields
            .next()
            .context("no refname")?
            .strip_prefix("refs/heads/")
            .context("unexpected format for refname")?
            .to_string()
            .into();
        let upstream_name = fields.next().context("no upstream")?.to_string();
        let upstream_tracking = parse_upstream_track(fields.next().context("no upstream:track")?)?;
        let commiterdate = fields.next().context("no committerdate")?.parse::<i64>()?;
        let subject: SharedString = fields
            .next()
            .context("no contents:subject")?
            .to_string()
            .into();

        branches.push(Branch {
            is_head: is_current_branch,
            name: ref_name,
            most_recent_commit: Some(CommitSummary {
                sha: head_sha,
                subject,
                commit_timestamp: commiterdate,
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
    if upstream_track == "" {
        return Ok(UpstreamTracking::Tracked(UpstreamTrackingStatus {
            ahead: 0,
            behind: 0,
        }));
    }

    let upstream_track = upstream_track
        .strip_prefix("[")
        .ok_or_else(|| anyhow!("missing ["))?;
    let upstream_track = upstream_track
        .strip_suffix("]")
        .ok_or_else(|| anyhow!("missing ["))?;
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

fn check_path_to_repo_path_errors(relative_file_path: &Path) -> Result<()> {
    match relative_file_path.components().next() {
        None => anyhow::bail!("repo path should not be empty"),
        Some(Component::Prefix(_)) => anyhow::bail!(
            "repo path `{}` should be relative, not a windows prefix",
            relative_file_path.to_string_lossy()
        ),
        Some(Component::RootDir) => {
            anyhow::bail!(
                "repo path `{}` should be relative",
                relative_file_path.to_string_lossy()
            )
        }
        Some(Component::CurDir) => {
            anyhow::bail!(
                "repo path `{}` should not start with `.`",
                relative_file_path.to_string_lossy()
            )
        }
        Some(Component::ParentDir) => {
            anyhow::bail!(
                "repo path `{}` should not start with `..`",
                relative_file_path.to_string_lossy()
            )
        }
        _ => Ok(()),
    }
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
    use crate::status::{FileStatus, StatusCode, TrackedStatus};
    use gpui::TestAppContext;
    use unindent::Unindent;

    #[gpui::test]
    async fn test_checkpoint_basic(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();

        git2::Repository::init(repo_dir.path()).unwrap();
        let file_path = repo_dir.path().join("file");
        smol::fs::write(&file_path, "initial").await.unwrap();

        let repo =
            RealGitRepository::new(&repo_dir.path().join(".git"), None, cx.executor()).unwrap();
        repo.stage_paths(vec![RepoPath::from_str("file")], HashMap::default())
            .await
            .unwrap();
        repo.commit("Initial commit".into(), None, checkpoint_author_envs())
            .await
            .unwrap();

        smol::fs::write(&file_path, "modified before checkpoint")
            .await
            .unwrap();
        smol::fs::write(repo_dir.path().join("new_file_before_checkpoint"), "1")
            .await
            .unwrap();
        let sha_before_checkpoint = repo.head_sha().unwrap();
        let checkpoint = repo.checkpoint().await.unwrap();

        // Ensure the user can't see any branches after creating a checkpoint.
        assert_eq!(repo.branches().await.unwrap().len(), 1);

        smol::fs::write(&file_path, "modified after checkpoint")
            .await
            .unwrap();
        repo.stage_paths(vec![RepoPath::from_str("file")], HashMap::default())
            .await
            .unwrap();
        repo.commit(
            "Commit after checkpoint".into(),
            None,
            checkpoint_author_envs(),
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

        assert_eq!(repo.head_sha().unwrap(), sha_before_checkpoint);
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
        assert_eq!(
            smol::fs::read_to_string(repo_dir.path().join("new_file_after_checkpoint"))
                .await
                .ok(),
            None
        );

        // Garbage collecting after deleting a checkpoint makes it unreachable.
        repo.delete_checkpoint(checkpoint.clone()).await.unwrap();
        repo.gc().await.unwrap();
        repo.restore_checkpoint(checkpoint.clone())
            .await
            .unwrap_err();
    }

    #[gpui::test]
    async fn test_checkpoint_empty_repo(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();
        git2::Repository::init(repo_dir.path()).unwrap();
        let repo =
            RealGitRepository::new(&repo_dir.path().join(".git"), None, cx.executor()).unwrap();

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
        assert_eq!(
            smol::fs::read_to_string(repo_dir.path().join("baz"))
                .await
                .ok(),
            None
        );
    }

    #[gpui::test]
    async fn test_undoing_commit_via_checkpoint(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();

        git2::Repository::init(repo_dir.path()).unwrap();
        let file_path = repo_dir.path().join("file");
        smol::fs::write(&file_path, "initial").await.unwrap();

        let repo =
            RealGitRepository::new(&repo_dir.path().join(".git"), None, cx.executor()).unwrap();
        repo.stage_paths(vec![RepoPath::from_str("file")], HashMap::default())
            .await
            .unwrap();
        repo.commit("Initial commit".into(), None, checkpoint_author_envs())
            .await
            .unwrap();

        let initial_commit_sha = repo.head_sha().unwrap();

        smol::fs::write(repo_dir.path().join("new_file1"), "content1")
            .await
            .unwrap();
        smol::fs::write(repo_dir.path().join("new_file2"), "content2")
            .await
            .unwrap();

        let checkpoint = repo.checkpoint().await.unwrap();

        repo.stage_paths(
            vec![
                RepoPath::from_str("new_file1"),
                RepoPath::from_str("new_file2"),
            ],
            HashMap::default(),
        )
        .await
        .unwrap();
        repo.commit("Commit new files".into(), None, checkpoint_author_envs())
            .await
            .unwrap();

        repo.restore_checkpoint(checkpoint).await.unwrap();
        assert_eq!(repo.head_sha().unwrap(), initial_commit_sha);
        assert_eq!(
            smol::fs::read_to_string(repo_dir.path().join("new_file1"))
                .await
                .unwrap(),
            "content1"
        );
        assert_eq!(
            smol::fs::read_to_string(repo_dir.path().join("new_file2"))
                .await
                .unwrap(),
            "content2"
        );
        assert_eq!(
            repo.status(None, &[]).await.unwrap().entries.as_ref(),
            &[
                (RepoPath::from_str("new_file1"), FileStatus::Untracked),
                (RepoPath::from_str("new_file2"), FileStatus::Untracked)
            ]
        );
    }

    #[gpui::test]
    async fn test_compare_checkpoints(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();
        git2::Repository::init(repo_dir.path()).unwrap();
        let repo =
            RealGitRepository::new(&repo_dir.path().join(".git"), None, cx.executor()).unwrap();

        smol::fs::write(repo_dir.path().join("file1"), "content1")
            .await
            .unwrap();
        let checkpoint1 = repo.checkpoint().await.unwrap();

        smol::fs::write(repo_dir.path().join("file2"), "content2")
            .await
            .unwrap();
        let checkpoint2 = repo.checkpoint().await.unwrap();

        assert!(!repo
            .compare_checkpoints(checkpoint1, checkpoint2.clone())
            .await
            .unwrap());

        let checkpoint3 = repo.checkpoint().await.unwrap();
        assert!(repo
            .compare_checkpoints(checkpoint2, checkpoint3)
            .await
            .unwrap());
    }

    #[gpui::test]
    async fn test_secondary_indices(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();
        git2::Repository::init(repo_dir.path()).unwrap();
        let repo =
            RealGitRepository::new(&repo_dir.path().join(".git"), None, cx.executor()).unwrap();
        let index = repo.create_index().await.unwrap();
        smol::fs::write(repo_dir.path().join("file1"), "file1\n")
            .await
            .unwrap();
        smol::fs::write(repo_dir.path().join("file2"), "file2\n")
            .await
            .unwrap();
        let diff = r#"
            diff --git a/file2 b/file2
            new file mode 100644
            index 0000000..cbc4e2e
            --- /dev/null
            +++ b/file2
            @@ -0,0 +1 @@
            +file2
        "#
        .unindent();
        repo.apply_diff(index, diff.to_string()).await.unwrap();

        assert_eq!(
            repo.status(Some(index), &[])
                .await
                .unwrap()
                .entries
                .as_ref(),
            vec![
                (RepoPath::from_str("file1"), FileStatus::Untracked),
                (
                    RepoPath::from_str("file2"),
                    FileStatus::index(StatusCode::Added)
                )
            ]
        );
        assert_eq!(
            repo.load_index_text(Some(index), RepoPath::from_str("file1"))
                .await,
            None
        );
        assert_eq!(
            repo.load_index_text(Some(index), RepoPath::from_str("file2"))
                .await,
            Some("file2\n".to_string())
        );

        smol::fs::write(repo_dir.path().join("file2"), "file2-changed\n")
            .await
            .unwrap();
        assert_eq!(
            repo.status(Some(index), &[])
                .await
                .unwrap()
                .entries
                .as_ref(),
            vec![
                (RepoPath::from_str("file1"), FileStatus::Untracked),
                (
                    RepoPath::from_str("file2"),
                    FileStatus::Tracked(TrackedStatus {
                        worktree_status: StatusCode::Modified,
                        index_status: StatusCode::Added,
                    })
                )
            ]
        );
        assert_eq!(
            repo.load_index_text(Some(index), RepoPath::from_str("file1"))
                .await,
            None
        );
        assert_eq!(
            repo.load_index_text(Some(index), RepoPath::from_str("file2"))
                .await,
            Some("file2\n".to_string())
        );
    }

    #[test]
    fn test_branches_parsing() {
        // suppress "help: octal escapes are not supported, `\0` is always null"
        #[allow(clippy::octal_escapes)]
        let input = "*\0060964da10574cd9bf06463a53bf6e0769c5c45e\0\0refs/heads/zed-patches\0refs/remotes/origin/zed-patches\0\01733187470\0generated protobuf\n";
        assert_eq!(
            parse_branch_input(&input).unwrap(),
            vec![Branch {
                is_head: true,
                name: "zed-patches".into(),
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
                    has_parent: false,
                })
            }]
        )
    }

    impl RealGitRepository {
        /// Force a Git garbage collection on the repository.
        fn gc(&self) -> BoxFuture<Result<()>> {
            let working_directory = self.working_directory();
            let git_binary_path = self.git_binary_path.clone();
            let executor = self.executor.clone();
            self.executor
                .spawn(async move {
                    let git_binary_path = git_binary_path.clone();
                    let working_directory = working_directory?;
                    let git = GitBinary::new(git_binary_path, working_directory, executor);
                    git.run(&["gc", "--prune=now"]).await?;
                    Ok(())
                })
                .boxed()
        }
    }
}
