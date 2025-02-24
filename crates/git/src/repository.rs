use crate::status::FileStatus;
use crate::GitHostingProviderRegistry;
use crate::{blame::Blame, status::GitStatus};
use anyhow::{anyhow, Context, Result};
use collections::{HashMap, HashSet};
use git2::BranchType;
use gpui::SharedString;
use parking_lot::Mutex;
use rope::Rope;
use std::borrow::Borrow;
use std::io::Write as _;
use std::process::Stdio;
use std::sync::LazyLock;
use std::{
    cmp::Ordering,
    path::{Component, Path, PathBuf},
    sync::Arc,
};
use sum_tree::MapSeekTarget;
use util::command::new_std_command;
use util::ResultExt;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Branch {
    pub is_head: bool,
    pub name: SharedString,
    pub upstream: Option<Upstream>,
    pub most_recent_commit: Option<CommitSummary>,
}

impl Branch {
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
    pub tracking: Option<UpstreamTracking>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UpstreamTracking {
    pub ahead: u32,
    pub behind: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CommitSummary {
    pub sha: SharedString,
    pub subject: SharedString,
    /// This is a unix timestamp
    pub commit_timestamp: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CommitDetails {
    pub sha: SharedString,
    pub message: SharedString,
    pub commit_timestamp: i64,
    pub committer_email: SharedString,
    pub committer_name: SharedString,
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
    fn load_index_text(&self, path: &RepoPath) -> Option<String>;

    /// Returns the contents of an entry in the repository's HEAD, or None if HEAD does not exist or has no entry for the given path.
    ///
    /// Also returns `None` for symlinks.
    fn load_committed_text(&self, path: &RepoPath) -> Option<String>;

    fn set_index_text(&self, path: &RepoPath, content: Option<String>) -> anyhow::Result<()>;

    /// Returns the URL of the remote with the given name.
    fn remote_url(&self, name: &str) -> Option<String>;

    /// Returns the SHA of the current HEAD.
    fn head_sha(&self) -> Option<String>;

    fn merge_head_shas(&self) -> Vec<String>;

    /// Returns the list of git statuses, sorted by path
    fn status(&self, path_prefixes: &[RepoPath]) -> Result<GitStatus>;

    fn branches(&self) -> Result<Vec<Branch>>;
    fn change_branch(&self, _: &str) -> Result<()>;
    fn create_branch(&self, _: &str) -> Result<()>;
    fn branch_exits(&self, _: &str) -> Result<bool>;

    fn reset(&self, commit: &str, mode: ResetMode) -> Result<()>;
    fn checkout_files(&self, commit: &str, paths: &[RepoPath]) -> Result<()>;

    fn show(&self, commit: &str) -> Result<CommitDetails>;

    fn blame(&self, path: &Path, content: Rope) -> Result<crate::blame::Blame>;

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
    fn stage_paths(&self, paths: &[RepoPath]) -> Result<()>;
    /// Updates the index to match HEAD at the given paths.
    ///
    /// If any of the paths were previously staged but do not exist in HEAD, they will be removed from the index.
    fn unstage_paths(&self, paths: &[RepoPath]) -> Result<()>;

    fn commit(&self, message: &str, name_and_email: Option<(&str, &str)>) -> Result<()>;
}

impl std::fmt::Debug for dyn GitRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn GitRepository<...>").finish()
    }
}

pub struct RealGitRepository {
    pub repository: Mutex<git2::Repository>,
    pub git_binary_path: PathBuf,
    hosting_provider_registry: Arc<GitHostingProviderRegistry>,
}

impl RealGitRepository {
    pub fn new(
        repository: git2::Repository,
        git_binary_path: Option<PathBuf>,
        hosting_provider_registry: Arc<GitHostingProviderRegistry>,
    ) -> Self {
        Self {
            repository: Mutex::new(repository),
            git_binary_path: git_binary_path.unwrap_or_else(|| PathBuf::from("git")),
            hosting_provider_registry,
        }
    }
}

// https://git-scm.com/book/en/v2/Git-Internals-Git-Objects
const GIT_MODE_SYMLINK: u32 = 0o120000;

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

    fn show(&self, commit: &str) -> Result<CommitDetails> {
        let repo = self.repository.lock();
        let Ok(commit) = repo.revparse_single(commit)?.into_commit() else {
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
    }

    fn reset(&self, commit: &str, mode: ResetMode) -> Result<()> {
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .context("failed to read git work directory")?
            .to_path_buf();

        let mode_flag = match mode {
            ResetMode::Mixed => "--mixed",
            ResetMode::Soft => "--soft",
        };

        let output = new_std_command(&self.git_binary_path)
            .current_dir(&working_directory)
            .args(["reset", mode_flag, commit])
            .output()?;
        if !output.status.success() {
            return Err(anyhow!(
                "Failed to reset:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(())
    }

    fn checkout_files(&self, commit: &str, paths: &[RepoPath]) -> Result<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .context("failed to read git work directory")?
            .to_path_buf();

        let output = new_std_command(&self.git_binary_path)
            .current_dir(&working_directory)
            .args(["checkout", commit, "--"])
            .args(paths.iter().map(|path| path.as_ref()))
            .output()?;
        if !output.status.success() {
            return Err(anyhow!(
                "Failed to checkout files:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(())
    }

    fn load_index_text(&self, path: &RepoPath) -> Option<String> {
        fn logic(repo: &git2::Repository, path: &RepoPath) -> Result<Option<String>> {
            const STAGE_NORMAL: i32 = 0;
            let index = repo.index()?;

            // This check is required because index.get_path() unwraps internally :(
            check_path_to_repo_path_errors(path)?;

            let oid = match index.get_path(path, STAGE_NORMAL) {
                Some(entry) if entry.mode != GIT_MODE_SYMLINK => entry.id,
                _ => return Ok(None),
            };

            let content = repo.find_blob(oid)?.content().to_owned();
            Ok(Some(String::from_utf8(content)?))
        }

        match logic(&self.repository.lock(), path) {
            Ok(value) => return value,
            Err(err) => log::error!("Error loading index text: {:?}", err),
        }
        None
    }

    fn load_committed_text(&self, path: &RepoPath) -> Option<String> {
        let repo = self.repository.lock();
        let head = repo.head().ok()?.peel_to_tree().log_err()?;
        let entry = head.get_path(path).ok()?;
        if entry.filemode() == i32::from(git2::FileMode::Link) {
            return None;
        }
        let content = repo.find_blob(entry.id()).log_err()?.content().to_owned();
        let content = String::from_utf8(content).log_err()?;
        Some(content)
    }

    fn set_index_text(&self, path: &RepoPath, content: Option<String>) -> anyhow::Result<()> {
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .context("failed to read git work directory")?
            .to_path_buf();
        if let Some(content) = content {
            let mut child = new_std_command(&self.git_binary_path)
                .current_dir(&working_directory)
                .args(["hash-object", "-w", "--stdin"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            child.stdin.take().unwrap().write_all(content.as_bytes())?;
            let output = child.wait_with_output()?.stdout;
            let sha = String::from_utf8(output)?;

            log::debug!("indexing SHA: {sha}, path {path:?}");

            let status = new_std_command(&self.git_binary_path)
                .current_dir(&working_directory)
                .args(["update-index", "--add", "--cacheinfo", "100644", &sha])
                .arg(path.as_ref())
                .status()?;

            if !status.success() {
                return Err(anyhow!("Failed to add to index: {status:?}"));
            }
        } else {
            let status = new_std_command(&self.git_binary_path)
                .current_dir(&working_directory)
                .args(["update-index", "--force-remove"])
                .arg(path.as_ref())
                .status()?;

            if !status.success() {
                return Err(anyhow!("Failed to remove from index: {status:?}"));
            }
        }

        Ok(())
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
        shas
    }

    fn status(&self, path_prefixes: &[RepoPath]) -> Result<GitStatus> {
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .context("failed to read git work directory")?
            .to_path_buf();
        GitStatus::new(&self.git_binary_path, &working_directory, path_prefixes)
    }

    fn branch_exits(&self, name: &str) -> Result<bool> {
        let repo = self.repository.lock();
        let branch = repo.find_branch(name, BranchType::Local);
        match branch {
            Ok(_) => Ok(true),
            Err(e) => match e.code() {
                git2::ErrorCode::NotFound => Ok(false),
                _ => Err(anyhow!(e)),
            },
        }
    }

    fn branches(&self) -> Result<Vec<Branch>> {
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .context("failed to read git work directory")?
            .to_path_buf();
        let fields = [
            "%(HEAD)",
            "%(objectname)",
            "%(refname)",
            "%(upstream)",
            "%(upstream:track)",
            "%(committerdate:unix)",
            "%(contents:subject)",
        ]
        .join("%00");
        let args = vec!["for-each-ref", "refs/heads/**/*", "--format", &fields];

        let output = new_std_command(&self.git_binary_path)
            .current_dir(&working_directory)
            .args(args)
            .output()?;

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

            let output = new_std_command(&self.git_binary_path)
                .current_dir(&working_directory)
                .args(args)
                .output()?;

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

    fn change_branch(&self, name: &str) -> Result<()> {
        let repo = self.repository.lock();
        let revision = repo.find_branch(name, BranchType::Local)?;
        let revision = revision.get();
        let as_tree = revision.peel_to_tree()?;
        repo.checkout_tree(as_tree.as_object(), None)?;
        repo.set_head(
            revision
                .name()
                .ok_or_else(|| anyhow!("Branch name could not be retrieved"))?,
        )?;
        Ok(())
    }

    fn create_branch(&self, name: &str) -> Result<()> {
        let repo = self.repository.lock();
        let current_commit = repo.head()?.peel_to_commit()?;
        repo.branch(name, &current_commit, false)?;
        Ok(())
    }

    fn blame(&self, path: &Path, content: Rope) -> Result<crate::blame::Blame> {
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .with_context(|| format!("failed to get git working directory for file {:?}", path))?
            .to_path_buf();

        const REMOTE_NAME: &str = "origin";
        let remote_url = self.remote_url(REMOTE_NAME);

        crate::blame::Blame::for_path(
            &self.git_binary_path,
            &working_directory,
            path,
            &content,
            remote_url,
            self.hosting_provider_registry.clone(),
        )
    }

    fn stage_paths(&self, paths: &[RepoPath]) -> Result<()> {
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .context("failed to read git work directory")?
            .to_path_buf();

        if !paths.is_empty() {
            let output = new_std_command(&self.git_binary_path)
                .current_dir(&working_directory)
                .args(["update-index", "--add", "--remove", "--"])
                .args(paths.iter().map(|p| p.as_ref()))
                .output()?;
            if !output.status.success() {
                return Err(anyhow!(
                    "Failed to stage paths:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        Ok(())
    }

    fn unstage_paths(&self, paths: &[RepoPath]) -> Result<()> {
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .context("failed to read git work directory")?
            .to_path_buf();

        if !paths.is_empty() {
            let output = new_std_command(&self.git_binary_path)
                .current_dir(&working_directory)
                .args(["reset", "--quiet", "--"])
                .args(paths.iter().map(|p| p.as_ref()))
                .output()?;
            if !output.status.success() {
                return Err(anyhow!(
                    "Failed to unstage:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        Ok(())
    }

    fn commit(&self, message: &str, name_and_email: Option<(&str, &str)>) -> Result<()> {
        let working_directory = self
            .repository
            .lock()
            .workdir()
            .context("failed to read git work directory")?
            .to_path_buf();
        let mut args = vec!["commit", "--quiet", "-m", message, "--cleanup=strip"];
        let author = name_and_email.map(|(name, email)| format!("{name} <{email}>"));
        if let Some(author) = author.as_deref() {
            args.push("--author");
            args.push(author);
        }

        let output = new_std_command(&self.git_binary_path)
            .current_dir(&working_directory)
            .args(args)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to commit:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FakeGitRepository {
    state: Arc<Mutex<FakeGitRepositoryState>>,
}

#[derive(Debug, Clone)]
pub struct FakeGitRepositoryState {
    pub path: PathBuf,
    pub event_emitter: smol::channel::Sender<PathBuf>,
    pub head_contents: HashMap<RepoPath, String>,
    pub index_contents: HashMap<RepoPath, String>,
    pub blames: HashMap<RepoPath, Blame>,
    pub statuses: HashMap<RepoPath, FileStatus>,
    pub current_branch_name: Option<String>,
    pub branches: HashSet<String>,
}

impl FakeGitRepository {
    pub fn open(state: Arc<Mutex<FakeGitRepositoryState>>) -> Arc<dyn GitRepository> {
        Arc::new(FakeGitRepository { state })
    }
}

impl FakeGitRepositoryState {
    pub fn new(path: PathBuf, event_emitter: smol::channel::Sender<PathBuf>) -> Self {
        FakeGitRepositoryState {
            path,
            event_emitter,
            head_contents: Default::default(),
            index_contents: Default::default(),
            blames: Default::default(),
            statuses: Default::default(),
            current_branch_name: Default::default(),
            branches: Default::default(),
        }
    }
}

impl GitRepository for FakeGitRepository {
    fn reload_index(&self) {}

    fn load_index_text(&self, path: &RepoPath) -> Option<String> {
        let state = self.state.lock();
        state.index_contents.get(path.as_ref()).cloned()
    }

    fn load_committed_text(&self, path: &RepoPath) -> Option<String> {
        let state = self.state.lock();
        state.head_contents.get(path.as_ref()).cloned()
    }

    fn set_index_text(&self, path: &RepoPath, content: Option<String>) -> anyhow::Result<()> {
        let mut state = self.state.lock();
        if let Some(content) = content {
            state.index_contents.insert(path.clone(), content);
        } else {
            state.index_contents.remove(path);
        }
        state
            .event_emitter
            .try_send(state.path.clone())
            .expect("Dropped repo change event");
        Ok(())
    }

    fn remote_url(&self, _name: &str) -> Option<String> {
        None
    }

    fn head_sha(&self) -> Option<String> {
        None
    }

    fn merge_head_shas(&self) -> Vec<String> {
        vec![]
    }

    fn show(&self, _: &str) -> Result<CommitDetails> {
        unimplemented!()
    }

    fn reset(&self, _: &str, _: ResetMode) -> Result<()> {
        unimplemented!()
    }

    fn checkout_files(&self, _: &str, _: &[RepoPath]) -> Result<()> {
        unimplemented!()
    }

    fn path(&self) -> PathBuf {
        let state = self.state.lock();
        state.path.clone()
    }

    fn main_repository_path(&self) -> PathBuf {
        self.path()
    }

    fn status(&self, path_prefixes: &[RepoPath]) -> Result<GitStatus> {
        let state = self.state.lock();

        let mut entries = state
            .statuses
            .iter()
            .filter_map(|(repo_path, status)| {
                if path_prefixes
                    .iter()
                    .any(|path_prefix| repo_path.0.starts_with(path_prefix))
                {
                    Some((repo_path.to_owned(), *status))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        entries.sort_unstable_by(|(a, _), (b, _)| a.cmp(&b));

        Ok(GitStatus {
            entries: entries.into(),
        })
    }

    fn branches(&self) -> Result<Vec<Branch>> {
        let state = self.state.lock();
        let current_branch = &state.current_branch_name;
        Ok(state
            .branches
            .iter()
            .map(|branch_name| Branch {
                is_head: Some(branch_name) == current_branch.as_ref(),
                name: branch_name.into(),
                most_recent_commit: None,
                upstream: None,
            })
            .collect())
    }

    fn branch_exits(&self, name: &str) -> Result<bool> {
        let state = self.state.lock();
        Ok(state.branches.contains(name))
    }

    fn change_branch(&self, name: &str) -> Result<()> {
        let mut state = self.state.lock();
        state.current_branch_name = Some(name.to_owned());
        state
            .event_emitter
            .try_send(state.path.clone())
            .expect("Dropped repo change event");
        Ok(())
    }

    fn create_branch(&self, name: &str) -> Result<()> {
        let mut state = self.state.lock();
        state.branches.insert(name.to_owned());
        state
            .event_emitter
            .try_send(state.path.clone())
            .expect("Dropped repo change event");
        Ok(())
    }

    fn blame(&self, path: &Path, _content: Rope) -> Result<crate::blame::Blame> {
        let state = self.state.lock();
        state
            .blames
            .get(path)
            .with_context(|| format!("failed to get blame for {:?}", path))
            .cloned()
    }

    fn stage_paths(&self, _paths: &[RepoPath]) -> Result<()> {
        unimplemented!()
    }

    fn unstage_paths(&self, _paths: &[RepoPath]) -> Result<()> {
        unimplemented!()
    }

    fn commit(&self, _message: &str, _name_and_email: Option<(&str, &str)>) -> Result<()> {
        unimplemented!()
    }
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

impl<'a> MapSeekTarget<RepoPath> for RepoPathDescendants<'a> {
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

fn parse_upstream_track(upstream_track: &str) -> Result<Option<UpstreamTracking>> {
    if upstream_track == "" {
        return Ok(Some(UpstreamTracking {
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
            return Ok(None);
        }
        if let Some(ahead_num) = component.strip_prefix("ahead ") {
            ahead = ahead_num.parse::<u32>()?;
        }
        if let Some(behind_num) = component.strip_prefix("behind ") {
            behind = behind_num.parse::<u32>()?;
        }
    }
    Ok(Some(UpstreamTracking { ahead, behind }))
}

#[test]
fn test_branches_parsing() {
    // suppress "help: octal escapes are not supported, `\0` is always null"
    #[allow(clippy::octal_escapes)]
    let input = "*\0060964da10574cd9bf06463a53bf6e0769c5c45e\0refs/heads/zed-patches\0refs/remotes/origin/zed-patches\0\01733187470\0generated protobuf\n";
    assert_eq!(
        parse_branch_input(&input).unwrap(),
        vec![Branch {
            is_head: true,
            name: "zed-patches".into(),
            upstream: Some(Upstream {
                ref_name: "refs/remotes/origin/zed-patches".into(),
                tracking: Some(UpstreamTracking {
                    ahead: 0,
                    behind: 0
                })
            }),
            most_recent_commit: Some(CommitSummary {
                sha: "060964da10574cd9bf06463a53bf6e0769c5c45e".into(),
                subject: "generated protobuf".into(),
                commit_timestamp: 1733187470,
            })
        }]
    )
}
