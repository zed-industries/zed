use anyhow::{Context, Result};
use collections::HashMap;
use git::blame::Blame;
use git2::{BranchType, StatusShow};
use parking_lot::Mutex;
use rope::Rope;
use serde_derive::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    path::{Component, Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use sum_tree::{MapSeekTarget, TreeMap};
use util::{paths::PathExt, ResultExt};

pub use git2::Repository as LibGitRepository;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Branch {
    pub name: Box<str>,
    /// Timestamp of most recent commit, normalized to Unix Epoch format.
    pub unix_timestamp: Option<i64>,
}

pub trait GitRepository: Send {
    fn reload_index(&self);
    fn load_index_text(&self, relative_file_path: &Path) -> Option<String>;

    /// Returns the URL of the remote with the given name.
    fn remote_url(&self, name: &str) -> Option<String>;
    fn branch_name(&self) -> Option<String>;

    /// Returns the SHA of the current HEAD.
    fn head_sha(&self) -> Option<String>;

    /// Get the statuses of all of the files in the index that start with the given
    /// path and have changes with respect to the HEAD commit. This is fast because
    /// the index stores hashes of trees, so that unchanged directories can be skipped.
    fn staged_statuses(&self, path_prefix: &Path) -> TreeMap<RepoPath, GitFileStatus>;

    /// Get the status of a given file in the working directory with respect to
    /// the index. In the common case, when there are no changes, this only requires
    /// an index lookup. The index stores the mtime of each file when it was added,
    /// so there's no work to do if the mtime matches.
    fn unstaged_status(&self, path: &RepoPath, mtime: SystemTime) -> Option<GitFileStatus>;

    /// Get the status of a given file in the working directory with respect to
    /// the HEAD commit. In the common case, when there are no changes, this only
    /// requires an index lookup and blob comparison between the index and the HEAD
    /// commit. The index stores the mtime of each file when it was added, so there's
    /// no need to consider the working directory file if the mtime matches.
    fn status(&self, path: &RepoPath, mtime: SystemTime) -> Option<GitFileStatus>;

    fn branches(&self) -> Result<Vec<Branch>>;
    fn change_branch(&self, _: &str) -> Result<()>;
    fn create_branch(&self, _: &str) -> Result<()>;

    fn blame(&self, path: &Path, content: Rope) -> Result<git::blame::Blame>;
}

impl std::fmt::Debug for dyn GitRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn GitRepository<...>").finish()
    }
}

pub struct RealGitRepository {
    pub repository: LibGitRepository,
    pub git_binary_path: PathBuf,
}

impl RealGitRepository {
    pub fn new(repository: LibGitRepository, git_binary_path: Option<PathBuf>) -> Self {
        Self {
            repository,
            git_binary_path: git_binary_path.unwrap_or_else(|| PathBuf::from("git")),
        }
    }
}

impl GitRepository for RealGitRepository {
    fn reload_index(&self) {
        if let Ok(mut index) = self.repository.index() {
            _ = index.read(false);
        }
    }

    fn load_index_text(&self, relative_file_path: &Path) -> Option<String> {
        fn logic(repo: &LibGitRepository, relative_file_path: &Path) -> Result<Option<String>> {
            const STAGE_NORMAL: i32 = 0;
            let index = repo.index()?;

            // This check is required because index.get_path() unwraps internally :(
            check_path_to_repo_path_errors(relative_file_path)?;

            let oid = match index.get_path(relative_file_path, STAGE_NORMAL) {
                Some(entry) => entry.id,
                None => return Ok(None),
            };

            let content = repo.find_blob(oid)?.content().to_owned();
            Ok(Some(String::from_utf8(content)?))
        }

        match logic(&self.repository, relative_file_path) {
            Ok(value) => return value,
            Err(err) => log::error!("Error loading head text: {:?}", err),
        }
        None
    }

    fn remote_url(&self, name: &str) -> Option<String> {
        let remote = self.repository.find_remote(name).ok()?;
        remote.url().map(|url| url.to_string())
    }

    fn branch_name(&self) -> Option<String> {
        let head = self.repository.head().log_err()?;
        let branch = String::from_utf8_lossy(head.shorthand_bytes());
        Some(branch.to_string())
    }

    fn head_sha(&self) -> Option<String> {
        let head = self.repository.head().ok()?;
        head.target().map(|oid| oid.to_string())
    }

    fn staged_statuses(&self, path_prefix: &Path) -> TreeMap<RepoPath, GitFileStatus> {
        let mut map = TreeMap::default();

        let mut options = git2::StatusOptions::new();
        options.pathspec(path_prefix);
        options.show(StatusShow::Index);

        if let Some(statuses) = self.repository.statuses(Some(&mut options)).log_err() {
            for status in statuses.iter() {
                let path = RepoPath(PathBuf::try_from_bytes(status.path_bytes()).unwrap());
                let status = status.status();
                if !status.contains(git2::Status::IGNORED) {
                    if let Some(status) = read_status(status) {
                        map.insert(path, status)
                    }
                }
            }
        }
        map
    }

    fn unstaged_status(&self, path: &RepoPath, mtime: SystemTime) -> Option<GitFileStatus> {
        // If the file has not changed since it was added to the index, then
        // there can't be any changes.
        if matches_index(&self.repository, path, mtime) {
            return None;
        }

        let mut options = git2::StatusOptions::new();
        options.pathspec(&path.0);
        options.disable_pathspec_match(true);
        options.include_untracked(true);
        options.recurse_untracked_dirs(true);
        options.include_unmodified(true);
        options.show(StatusShow::Workdir);

        let statuses = self.repository.statuses(Some(&mut options)).log_err()?;
        let status = statuses.get(0).and_then(|s| read_status(s.status()));
        status
    }

    fn status(&self, path: &RepoPath, mtime: SystemTime) -> Option<GitFileStatus> {
        let mut options = git2::StatusOptions::new();
        options.pathspec(&path.0);
        options.disable_pathspec_match(true);
        options.include_untracked(true);
        options.recurse_untracked_dirs(true);
        options.include_unmodified(true);

        // If the file has not changed since it was added to the index, then
        // there's no need to examine the working directory file: just compare
        // the blob in the index to the one in the HEAD commit.
        if matches_index(&self.repository, path, mtime) {
            options.show(StatusShow::Index);
        }

        let statuses = self.repository.statuses(Some(&mut options)).log_err()?;
        let status = statuses.get(0).and_then(|s| read_status(s.status()));
        status
    }

    fn branches(&self) -> Result<Vec<Branch>> {
        let local_branches = self.repository.branches(Some(BranchType::Local))?;
        let valid_branches = local_branches
            .filter_map(|branch| {
                branch.ok().and_then(|(branch, _)| {
                    let name = branch.name().ok().flatten().map(Box::from)?;
                    let timestamp = branch.get().peel_to_commit().ok()?.time();
                    let unix_timestamp = timestamp.seconds();
                    let timezone_offset = timestamp.offset_minutes();
                    let utc_offset =
                        time::UtcOffset::from_whole_seconds(timezone_offset * 60).ok()?;
                    let unix_timestamp =
                        time::OffsetDateTime::from_unix_timestamp(unix_timestamp).ok()?;
                    Some(Branch {
                        name,
                        unix_timestamp: Some(unix_timestamp.to_offset(utc_offset).unix_timestamp()),
                    })
                })
            })
            .collect();
        Ok(valid_branches)
    }
    fn change_branch(&self, name: &str) -> Result<()> {
        let revision = self.repository.find_branch(name, BranchType::Local)?;
        let revision = revision.get();
        let as_tree = revision.peel_to_tree()?;
        self.repository.checkout_tree(as_tree.as_object(), None)?;
        self.repository.set_head(
            revision
                .name()
                .ok_or_else(|| anyhow::anyhow!("Branch name could not be retrieved"))?,
        )?;
        Ok(())
    }
    fn create_branch(&self, name: &str) -> Result<()> {
        let current_commit = self.repository.head()?.peel_to_commit()?;
        self.repository.branch(name, &current_commit, false)?;

        Ok(())
    }

    fn blame(&self, path: &Path, content: Rope) -> Result<git::blame::Blame> {
        let git_dir_path = self.repository.path();
        let working_directory = git_dir_path.parent().with_context(|| {
            format!("failed to get git working directory for {:?}", git_dir_path)
        })?;

        const REMOTE_NAME: &str = "origin";
        let remote_url = self.remote_url(REMOTE_NAME);

        git::blame::Blame::for_path(
            &self.git_binary_path,
            working_directory,
            path,
            &content,
            remote_url,
        )
    }
}

fn matches_index(repo: &LibGitRepository, path: &RepoPath, mtime: SystemTime) -> bool {
    if let Some(index) = repo.index().log_err() {
        if let Some(entry) = index.get_path(path, 0) {
            if let Some(mtime) = mtime.duration_since(SystemTime::UNIX_EPOCH).log_err() {
                if entry.mtime.seconds() == mtime.as_secs() as i32
                    && entry.mtime.nanoseconds() == mtime.subsec_nanos()
                {
                    return true;
                }
            }
        }
    }
    false
}

fn read_status(status: git2::Status) -> Option<GitFileStatus> {
    if status.contains(git2::Status::CONFLICTED) {
        Some(GitFileStatus::Conflict)
    } else if status.intersects(
        git2::Status::WT_MODIFIED
            | git2::Status::WT_RENAMED
            | git2::Status::INDEX_MODIFIED
            | git2::Status::INDEX_RENAMED,
    ) {
        Some(GitFileStatus::Modified)
    } else if status.intersects(git2::Status::WT_NEW | git2::Status::INDEX_NEW) {
        Some(GitFileStatus::Added)
    } else {
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct FakeGitRepository {
    state: Arc<Mutex<FakeGitRepositoryState>>,
}

#[derive(Debug, Clone, Default)]
pub struct FakeGitRepositoryState {
    pub index_contents: HashMap<PathBuf, String>,
    pub blames: HashMap<PathBuf, Blame>,
    pub worktree_statuses: HashMap<RepoPath, GitFileStatus>,
    pub branch_name: Option<String>,
}

impl FakeGitRepository {
    pub fn open(state: Arc<Mutex<FakeGitRepositoryState>>) -> Arc<Mutex<dyn GitRepository>> {
        Arc::new(Mutex::new(FakeGitRepository { state }))
    }
}

impl GitRepository for FakeGitRepository {
    fn reload_index(&self) {}

    fn load_index_text(&self, path: &Path) -> Option<String> {
        let state = self.state.lock();
        state.index_contents.get(path).cloned()
    }

    fn remote_url(&self, _name: &str) -> Option<String> {
        None
    }

    fn branch_name(&self) -> Option<String> {
        let state = self.state.lock();
        state.branch_name.clone()
    }

    fn head_sha(&self) -> Option<String> {
        None
    }

    fn staged_statuses(&self, path_prefix: &Path) -> TreeMap<RepoPath, GitFileStatus> {
        let mut map = TreeMap::default();
        let state = self.state.lock();
        for (repo_path, status) in state.worktree_statuses.iter() {
            if repo_path.0.starts_with(path_prefix) {
                map.insert(repo_path.to_owned(), status.to_owned());
            }
        }
        map
    }

    fn unstaged_status(&self, _path: &RepoPath, _mtime: SystemTime) -> Option<GitFileStatus> {
        None
    }

    fn status(&self, path: &RepoPath, _mtime: SystemTime) -> Option<GitFileStatus> {
        let state = self.state.lock();
        state.worktree_statuses.get(path).cloned()
    }

    fn branches(&self) -> Result<Vec<Branch>> {
        Ok(vec![])
    }

    fn change_branch(&self, name: &str) -> Result<()> {
        let mut state = self.state.lock();
        state.branch_name = Some(name.to_owned());
        Ok(())
    }

    fn create_branch(&self, name: &str) -> Result<()> {
        let mut state = self.state.lock();
        state.branch_name = Some(name.to_owned());
        Ok(())
    }

    fn blame(&self, path: &Path, _content: Rope) -> Result<git::blame::Blame> {
        let state = self.state.lock();
        state
            .blames
            .get(path)
            .with_context(|| format!("failed to get blame for {:?}", path))
            .cloned()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitFileStatus {
    Added,
    Modified,
    Conflict,
}

impl GitFileStatus {
    pub fn merge(
        this: Option<GitFileStatus>,
        other: Option<GitFileStatus>,
        prefer_other: bool,
    ) -> Option<GitFileStatus> {
        if prefer_other {
            return other;
        }

        match (this, other) {
            (Some(GitFileStatus::Conflict), _) | (_, Some(GitFileStatus::Conflict)) => {
                Some(GitFileStatus::Conflict)
            }
            (Some(GitFileStatus::Modified), _) | (_, Some(GitFileStatus::Modified)) => {
                Some(GitFileStatus::Modified)
            }
            (Some(GitFileStatus::Added), _) | (_, Some(GitFileStatus::Added)) => {
                Some(GitFileStatus::Added)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Ord, Hash, PartialOrd, Eq, PartialEq)]
pub struct RepoPath(pub PathBuf);

impl RepoPath {
    pub fn new(path: PathBuf) -> Self {
        debug_assert!(path.is_relative(), "Repo paths must be relative");

        RepoPath(path)
    }
}

impl From<&Path> for RepoPath {
    fn from(value: &Path) -> Self {
        RepoPath::new(value.to_path_buf())
    }
}

impl From<PathBuf> for RepoPath {
    fn from(value: PathBuf) -> Self {
        RepoPath::new(value)
    }
}

impl Default for RepoPath {
    fn default() -> Self {
        RepoPath(PathBuf::new())
    }
}

impl AsRef<Path> for RepoPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl std::ops::Deref for RepoPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
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
