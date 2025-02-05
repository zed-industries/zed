use crate::status::FileStatus;
use crate::GitHostingProviderRegistry;
use crate::{blame::Blame, status::GitStatus};
use anyhow::{anyhow, Context as _, Result};
use collections::{HashMap, HashSet};
use git2::BranchType;
use gpui::SharedString;
use parking_lot::Mutex;
use rope::Rope;
use std::borrow::Borrow;
use std::sync::LazyLock;
use std::{
    cmp::Ordering,
    path::{Component, Path, PathBuf},
    sync::Arc,
};
use sum_tree::MapSeekTarget;
use util::command::new_std_command;
use util::ResultExt;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Branch {
    pub is_head: bool,
    pub name: SharedString,
    /// Timestamp of most recent commit, normalized to Unix Epoch format.
    pub unix_timestamp: Option<i64>,
}

pub trait GitRepository: Send + Sync {
    fn reload_index(&self);

    /// Returns the contents of an entry in the repository's index, or None if there is no entry for the given path.
    ///
    /// Note that for symlink entries, this will return the contents of the symlink, not the target.
    fn load_index_text(&self, path: &RepoPath) -> Option<String>;

    /// Returns the contents of an entry in the repository's HEAD, or None if HEAD does not exist or has no entry for the given path.
    ///
    /// Note that for symlink entries, this will return the contents of the symlink, not the target.
    fn load_committed_text(&self, path: &RepoPath) -> Option<String>;

    /// Returns the URL of the remote with the given name.
    fn remote_url(&self, name: &str) -> Option<String>;
    fn branch_name(&self) -> Option<String>;

    /// Returns the SHA of the current HEAD.
    fn head_sha(&self) -> Option<String>;

    /// Returns the list of git statuses, sorted by path
    fn status(&self, path_prefixes: &[RepoPath]) -> Result<GitStatus>;

    fn branches(&self) -> Result<Vec<Branch>>;
    fn change_branch(&self, _: &str) -> Result<()>;
    fn create_branch(&self, _: &str) -> Result<()>;
    fn branch_exits(&self, _: &str) -> Result<bool>;

    fn blame(&self, path: &Path, content: Rope) -> Result<crate::blame::Blame>;

    /// Returns the path to the repository, typically the `.git` folder.
    fn dot_git_dir(&self) -> PathBuf;

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

    fn dot_git_dir(&self) -> PathBuf {
        let repo = self.repository.lock();
        repo.path().into()
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
        let oid = head.get_path(path).ok()?.id();
        let content = repo.find_blob(oid).log_err()?.content().to_owned();
        let content = String::from_utf8(content).log_err()?;
        Some(content)
    }

    fn remote_url(&self, name: &str) -> Option<String> {
        let repo = self.repository.lock();
        let remote = repo.find_remote(name).ok()?;
        remote.url().map(|url| url.to_string())
    }

    fn branch_name(&self) -> Option<String> {
        let repo = self.repository.lock();
        let head = repo.head().log_err()?;
        let branch = String::from_utf8_lossy(head.shorthand_bytes());
        Some(branch.to_string())
    }

    fn head_sha(&self) -> Option<String> {
        Some(self.repository.lock().head().ok()?.target()?.to_string())
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
        let repo = self.repository.lock();
        let local_branches = repo.branches(Some(BranchType::Local))?;
        let valid_branches = local_branches
            .filter_map(|branch| {
                branch.ok().and_then(|(branch, _)| {
                    let is_head = branch.is_head();
                    let name = branch
                        .name()
                        .ok()
                        .flatten()
                        .map(|name| name.to_string().into())?;
                    let timestamp = branch.get().peel_to_commit().ok()?.time();
                    let unix_timestamp = timestamp.seconds();
                    let timezone_offset = timestamp.offset_minutes();
                    let utc_offset =
                        time::UtcOffset::from_whole_seconds(timezone_offset * 60).ok()?;
                    let unix_timestamp =
                        time::OffsetDateTime::from_unix_timestamp(unix_timestamp).ok()?;
                    Some(Branch {
                        is_head,
                        name,
                        unix_timestamp: Some(unix_timestamp.to_offset(utc_offset).unix_timestamp()),
                    })
                })
            })
            .collect();
        Ok(valid_branches)
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
            let status = new_std_command(&self.git_binary_path)
                .current_dir(&working_directory)
                .args(["update-index", "--add", "--remove", "--"])
                .args(paths.iter().map(|p| p.as_ref()))
                .status()?;
            if !status.success() {
                return Err(anyhow!("Failed to stage paths: {status}"));
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
            let cmd = new_std_command(&self.git_binary_path)
                .current_dir(&working_directory)
                .args(["reset", "--quiet", "--"])
                .args(paths.iter().map(|p| p.as_ref()))
                .status()?;
            if !cmd.success() {
                return Err(anyhow!("Failed to unstage paths: {cmd}"));
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

        let cmd = new_std_command(&self.git_binary_path)
            .current_dir(&working_directory)
            .args(args)
            .status()?;
        if !cmd.success() {
            return Err(anyhow!("Failed to commit: {cmd}"));
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
    pub dot_git_dir: PathBuf,
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
    pub fn new(dot_git_dir: PathBuf, event_emitter: smol::channel::Sender<PathBuf>) -> Self {
        FakeGitRepositoryState {
            dot_git_dir,
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

    fn remote_url(&self, _name: &str) -> Option<String> {
        None
    }

    fn branch_name(&self) -> Option<String> {
        let state = self.state.lock();
        state.current_branch_name.clone()
    }

    fn head_sha(&self) -> Option<String> {
        None
    }

    fn dot_git_dir(&self) -> PathBuf {
        let state = self.state.lock();
        state.dot_git_dir.clone()
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
                unix_timestamp: None,
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
            .try_send(state.dot_git_dir.clone())
            .expect("Dropped repo change event");
        Ok(())
    }

    fn create_branch(&self, name: &str) -> Result<()> {
        let mut state = self.state.lock();
        state.branches.insert(name.to_owned());
        state
            .event_emitter
            .try_send(state.dot_git_dir.clone())
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

    pub fn to_proto(&self) -> String {
        self.0.to_string_lossy().to_string()
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
