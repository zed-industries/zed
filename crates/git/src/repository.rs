use crate::commit::parse_git_diff_name_status;
use crate::status::{GitStatus, StatusCode};
use crate::{Oid, SHORT_SHA_LENGTH};
use anyhow::{Context as _, Result, anyhow, bail};
use collections::HashMap;
use futures::future::BoxFuture;
use futures::{AsyncWriteExt, FutureExt as _, select_biased};
use git2::BranchType;
use gpui::{AppContext as _, AsyncApp, BackgroundExecutor, SharedString};
use parking_lot::Mutex;
use rope::Rope;
use schemars::JsonSchema;
use serde::Deserialize;
use std::borrow::{Borrow, Cow};
use std::ffi::{OsStr, OsString};
use std::io::prelude::*;
use std::path::Component;
use std::process::{ExitStatus, Stdio};
use std::sync::LazyLock;
use std::{
    cmp::Ordering,
    future,
    io::{BufRead, BufReader, BufWriter, Read},
    path::{Path, PathBuf},
    sync::Arc,
};
use sum_tree::MapSeekTarget;
use thiserror::Error;
use util::ResultExt;
use util::command::{new_smol_command, new_std_command};
use uuid::Uuid;

pub use askpass::{AskPassDelegate, AskPassResult, AskPassSession};

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

#[derive(Clone, Copy, Default)]
pub struct CommitOptions {
    pub amend: bool,
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

pub trait GitRepository: Send + Sync {
    fn reload_index(&self);

    /// Returns the contents of an entry in the repository's index, or None if there is no entry for the given path.
    ///
    /// Also returns `None` for symlinks.
    fn load_index_text(&self, path: RepoPath) -> BoxFuture<Option<String>>;

    /// Returns the contents of an entry in the repository's HEAD, or None if HEAD does not exist or has no entry for the given path.
    ///
    /// Also returns `None` for symlinks.
    fn load_committed_text(&self, path: RepoPath) -> BoxFuture<Option<String>>;

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<anyhow::Result<()>>;

    /// Returns the URL of the remote with the given name.
    fn remote_url(&self, name: &str) -> Option<String>;

    /// Resolve a list of refs to SHAs.
    fn revparse_batch(&self, revs: Vec<String>) -> BoxFuture<Result<Vec<Option<String>>>>;

    fn head_sha(&self) -> BoxFuture<Option<String>> {
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

    fn merge_message(&self) -> BoxFuture<Option<String>>;

    fn status(&self, path_prefixes: &[RepoPath]) -> BoxFuture<Result<GitStatus>>;

    fn branches(&self) -> BoxFuture<Result<Vec<Branch>>>;

    fn change_branch(&self, name: String) -> BoxFuture<Result<()>>;
    fn create_branch(&self, name: String) -> BoxFuture<Result<()>>;

    fn reset(
        &self,
        commit: String,
        mode: ResetMode,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<Result<()>>;

    fn checkout_files(
        &self,
        commit: String,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<Result<()>>;

    fn show(&self, commit: String) -> BoxFuture<Result<CommitDetails>>;

    fn load_commit(&self, commit: String, cx: AsyncApp) -> BoxFuture<Result<CommitDiff>>;
    fn blame(&self, path: RepoPath, content: Rope) -> BoxFuture<Result<crate::blame::Blame>>;

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
    ) -> BoxFuture<Result<()>>;
    /// Updates the index to match HEAD at the given paths.
    ///
    /// If any of the paths were previously staged but do not exist in HEAD, they will be removed from the index.
    fn unstage_paths(
        &self,
        paths: Vec<RepoPath>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<Result<()>>;

    fn commit(
        &self,
        message: SharedString,
        name_and_email: Option<(SharedString, SharedString)>,
        options: CommitOptions,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<Result<()>>;

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
    ) -> BoxFuture<Result<RemoteCommandOutput>>;

    fn pull(
        &self,
        branch_name: String,
        upstream_name: String,
        askpass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        // This method takes an AsyncApp to ensure it's invoked on the main thread,
        // otherwise git-credentials-manager won't work.
        cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>>;

    fn fetch(
        &self,
        askpass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
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

    /// Computes a diff between two checkpoints.
    fn diff_checkpoints(
        &self,
        base_checkpoint: GitRepositoryCheckpoint,
        target_checkpoint: GitRepositoryCheckpoint,
    ) -> BoxFuture<Result<String>>;
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
    pub commit_sha: Oid,
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
        let working_directory = self.working_directory();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let output = new_std_command("git")
                    .current_dir(&working_directory)
                    .args([
                        "--no-optional-locks",
                        "show",
                        "--no-patch",
                        "--format=%H%x00%B%x00%at%x00%ae%x00%an",
                        &commit,
                    ])
                    .output()?;
                let output = std::str::from_utf8(&output.stdout)?;
                let fields = output.split('\0').collect::<Vec<_>>();
                if fields.len() != 5 {
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

    fn load_commit(&self, commit: String, cx: AsyncApp) -> BoxFuture<Result<CommitDiff>> {
        let Some(working_directory) = self.repository.lock().workdir().map(ToOwned::to_owned)
        else {
            return future::ready(Err(anyhow!("no working directory"))).boxed();
        };
        cx.background_spawn(async move {
            let show_output = util::command::new_std_command("git")
                .current_dir(&working_directory)
                .args([
                    "--no-optional-locks",
                    "show",
                    "--format=%P",
                    "-z",
                    "--no-renames",
                    "--name-status",
                ])
                .arg(&commit)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| anyhow!("Failed to start git show process: {e}"))?;

            let show_stdout = String::from_utf8_lossy(&show_output.stdout);
            let mut lines = show_stdout.split('\n');
            let parent_sha = lines.next().unwrap().trim().trim_end_matches('\0');
            let changes = parse_git_diff_name_status(lines.next().unwrap_or(""));

            let mut cat_file_process = util::command::new_std_command("git")
                .current_dir(&working_directory)
                .args(["--no-optional-locks", "cat-file", "--batch=%(objectsize)"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| anyhow!("Failed to start git cat-file process: {e}"))?;

            use std::io::Write as _;
            let mut files = Vec::<CommitFile>::new();
            let mut stdin = BufWriter::with_capacity(512, cat_file_process.stdin.take().unwrap());
            let mut stdout = BufReader::new(cat_file_process.stdout.take().unwrap());
            let mut info_line = String::new();
            let mut newline = [b'\0'];
            for (path, status_code) in changes {
                match status_code {
                    StatusCode::Modified => {
                        writeln!(&mut stdin, "{commit}:{}", path.display())?;
                        writeln!(&mut stdin, "{parent_sha}:{}", path.display())?;
                    }
                    StatusCode::Added => {
                        writeln!(&mut stdin, "{commit}:{}", path.display())?;
                    }
                    StatusCode::Deleted => {
                        writeln!(&mut stdin, "{parent_sha}:{}", path.display())?;
                    }
                    _ => continue,
                }
                stdin.flush()?;

                info_line.clear();
                stdout.read_line(&mut info_line)?;

                let len = info_line.trim_end().parse().with_context(|| {
                    format!("invalid object size output from cat-file {info_line}")
                })?;
                let mut text = vec![0; len];
                stdout.read_exact(&mut text)?;
                stdout.read_exact(&mut newline)?;
                let text = String::from_utf8_lossy(&text).to_string();

                let mut old_text = None;
                let mut new_text = None;
                match status_code {
                    StatusCode::Modified => {
                        info_line.clear();
                        stdout.read_line(&mut info_line)?;
                        let len = info_line.trim_end().parse().with_context(|| {
                            format!("invalid object size output from cat-file {}", info_line)
                        })?;
                        let mut parent_text = vec![0; len];
                        stdout.read_exact(&mut parent_text)?;
                        stdout.read_exact(&mut newline)?;
                        old_text = Some(String::from_utf8_lossy(&parent_text).to_string());
                        new_text = Some(text);
                    }
                    StatusCode::Added => new_text = Some(text),
                    StatusCode::Deleted => old_text = Some(text),
                    _ => continue,
                }

                files.push(CommitFile {
                    path: path.into(),
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
    ) -> BoxFuture<Result<()>> {
        async move {
            let working_directory = self.working_directory();

            let mode_flag = match mode {
                ResetMode::Mixed => "--mixed",
                ResetMode::Soft => "--soft",
            };

            let output = new_smol_command(&self.git_binary_path)
                .envs(env.iter())
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
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        async move {
            if paths.is_empty() {
                return Ok(());
            }

            let output = new_smol_command(&git_binary_path)
                .current_dir(&working_directory?)
                .envs(env.iter())
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

    fn load_index_text(&self, path: RepoPath) -> BoxFuture<Option<String>> {
        // https://git-scm.com/book/en/v2/Git-Internals-Git-Objects
        const GIT_MODE_SYMLINK: u32 = 0o120000;

        let repo = self.repository.clone();
        self.executor
            .spawn(async move {
                fn logic(repo: &git2::Repository, path: &RepoPath) -> Result<Option<String>> {
                    // This check is required because index.get_path() unwraps internally :(
                    check_path_to_repo_path_errors(path)?;

                    let mut index = repo.index()?;
                    index.read(false)?;

                    const STAGE_NORMAL: i32 = 0;
                    let oid = match index.get_path(path, STAGE_NORMAL) {
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
                String::from_utf8(content).ok()
            })
            .boxed()
    }

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<anyhow::Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
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
                        .envs(env.iter())
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
                        .envs(env.iter())
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

    fn revparse_batch(&self, revs: Vec<String>) -> BoxFuture<Result<Vec<Option<String>>>> {
        let working_directory = self.working_directory();
        self.executor
            .spawn(async move {
                let working_directory = working_directory?;
                let mut process = new_std_command("git")
                    .current_dir(&working_directory)
                    .args([
                        "--no-optional-locks",
                        "cat-file",
                        "--batch-check=%(objectname)",
                        "-z",
                    ])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let stdin = process
                    .stdin
                    .take()
                    .ok_or_else(|| anyhow!("no stdin for git cat-file subprocess"))?;
                let mut stdin = BufWriter::new(stdin);
                for rev in &revs {
                    write!(&mut stdin, "{rev}\0")?;
                }
                drop(stdin);

                let output = process.wait_with_output()?;
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

    fn merge_message(&self) -> BoxFuture<Option<String>> {
        let path = self.path().join("MERGE_MSG");
        self.executor
            .spawn(async move { std::fs::read_to_string(&path).ok() })
            .boxed()
    }

    fn status(&self, path_prefixes: &[RepoPath]) -> BoxFuture<Result<GitStatus>> {
        let git_binary_path = self.git_binary_path.clone();
        let working_directory = self.working_directory();
        let path_prefixes = path_prefixes.to_owned();
        self.executor
            .spawn(async move {
                let output = new_std_command(&git_binary_path)
                    .current_dir(working_directory?)
                    .args(git_status_args(&path_prefixes))
                    .output()?;
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.parse()
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(anyhow!("git status failed: {}", stderr))
                }
            })
            .boxed()
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
                let branch = if let Ok(branch) = repo.find_branch(&name, BranchType::Local) {
                    branch
                } else if let Ok(revision) = repo.find_branch(&name, BranchType::Remote) {
                    let (_, branch_name) =
                        name.split_once("/").context("Unexpected branch format")?;
                    let revision = revision.get();
                    let branch_commit = revision.peel_to_commit()?;
                    let mut branch = repo.branch(&branch_name, &branch_commit, false)?;
                    branch.set_upstream(Some(&name))?;
                    branch
                } else {
                    return Err(anyhow!("Branch not found"));
                };

                let revision = branch.get();
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
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();
        self.executor
            .spawn(async move {
                if !paths.is_empty() {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory?)
                        .envs(env.iter())
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
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        let git_binary_path = self.git_binary_path.clone();

        self.executor
            .spawn(async move {
                if !paths.is_empty() {
                    let output = new_smol_command(&git_binary_path)
                        .current_dir(&working_directory?)
                        .envs(env.iter())
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
        options: CommitOptions,
        env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<Result<()>> {
        let working_directory = self.working_directory();
        self.executor
            .spawn(async move {
                let mut cmd = new_smol_command("git");
                cmd.current_dir(&working_directory?)
                    .envs(env.iter())
                    .args(["commit", "--quiet", "-m"])
                    .arg(&message.to_string())
                    .arg("--cleanup=strip");

                if options.amend {
                    cmd.arg("--amend");
                }

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
        ask_pass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        let executor = cx.background_executor().clone();
        async move {
            let working_directory = working_directory?;
            let mut command = new_smol_command("git");
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
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        let executor = cx.background_executor().clone();
        async move {
            let mut command = new_smol_command("git");
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
        ask_pass: AskPassDelegate,
        env: Arc<HashMap<String, String>>,
        cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        let working_directory = self.working_directory();
        let executor = cx.background_executor().clone();
        async move {
            let mut command = new_smol_command("git");
            command
                .envs(env.iter())
                .current_dir(&working_directory?)
                .args(["fetch", "--all"])
                .stdout(smol::process::Stdio::piped())
                .stderr(smol::process::Stdio::piped());

            run_git_command(env, ask_pass, command, &executor).await
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

                    Ok(GitRepositoryCheckpoint {
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

                Ok(())
            })
            .boxed()
    }

    fn compare_checkpoints(
        &self,
        left: GitRepositoryCheckpoint,
        right: GitRepositoryCheckpoint,
    ) -> BoxFuture<Result<bool>> {
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
                    &base_checkpoint.commit_sha.to_string(),
                    &target_checkpoint.commit_sha.to_string(),
                ])
                .await
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
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
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

async fn run_git_command(
    env: Arc<HashMap<String, String>>,
    ask_pass: AskPassDelegate,
    mut command: smol::process::Command,
    executor: &BackgroundExecutor,
) -> Result<RemoteCommandOutput> {
    if env.contains_key("GIT_ASKPASS") {
        let git_process = command.spawn()?;
        let output = git_process.output().await?;
        if !output.status.success() {
            Err(anyhow!("{}", String::from_utf8_lossy(&output.stderr)))
        } else {
            Ok(RemoteCommandOutput {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
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
        let raw_ref_name = fields.next().context("no refname")?;
        let ref_name: SharedString =
            if let Some(ref_name) = raw_ref_name.strip_prefix("refs/heads/") {
                ref_name.to_string().into()
            } else if let Some(ref_name) = raw_ref_name.strip_prefix("refs/remotes/") {
                ref_name.to_string().into()
            } else {
                return Err(anyhow!("unexpected format for refname"));
            };
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
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_checkpoint_basic(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let repo_dir = tempfile::tempdir().unwrap();

        git2::Repository::init(repo_dir.path()).unwrap();
        let file_path = repo_dir.path().join("file");
        smol::fs::write(&file_path, "initial").await.unwrap();

        let repo =
            RealGitRepository::new(&repo_dir.path().join(".git"), None, cx.executor()).unwrap();
        repo.stage_paths(
            vec![RepoPath::from_str("file")],
            Arc::new(HashMap::default()),
        )
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
        repo.stage_paths(
            vec![RepoPath::from_str("file")],
            Arc::new(HashMap::default()),
        )
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
        assert_eq!(
            smol::fs::read_to_string(repo_dir.path().join("new_file_after_checkpoint"))
                .await
                .ok(),
            None
        );
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
                    git.run(&["gc", "--prune"]).await?;
                    Ok(())
                })
                .boxed()
        }
    }
}
