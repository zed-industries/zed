use anyhow::Result;
use collections::HashMap;
use git2::ErrorCode;
use parking_lot::Mutex;
use rpc::proto;
use serde_derive::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    ffi::OsStr,
    os::unix::prelude::OsStrExt,
    path::{Component, Path, PathBuf},
    sync::Arc,
};
use sum_tree::{MapSeekTarget, TreeMap};
use util::ResultExt;

pub use git2::Repository as LibGitRepository;

#[async_trait::async_trait]
pub trait GitRepository: Send {
    fn reload_index(&self);

    fn load_index_text(&self, relative_file_path: &Path) -> Option<String>;

    fn branch_name(&self) -> Option<String>;

    fn statuses(&self) -> Option<TreeMap<RepoPath, GitFileStatus>>;

    fn status(&self, path: &RepoPath) -> Result<Option<GitFileStatus>>;
}

impl std::fmt::Debug for dyn GitRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn GitRepository<...>").finish()
    }
}

#[async_trait::async_trait]
impl GitRepository for LibGitRepository {
    fn reload_index(&self) {
        if let Ok(mut index) = self.index() {
            _ = index.read(false);
        }
    }

    fn load_index_text(&self, relative_file_path: &Path) -> Option<String> {
        fn logic(repo: &LibGitRepository, relative_file_path: &Path) -> Result<Option<String>> {
            const STAGE_NORMAL: i32 = 0;
            let index = repo.index()?;

            // This check is required because index.get_path() unwraps internally :(
            check_path_to_repo_path_errors(relative_file_path)?;

            let oid = match index.get_path(&relative_file_path, STAGE_NORMAL) {
                Some(entry) => entry.id,
                None => return Ok(None),
            };

            let content = repo.find_blob(oid)?.content().to_owned();
            Ok(Some(String::from_utf8(content)?))
        }

        match logic(&self, relative_file_path) {
            Ok(value) => return value,
            Err(err) => log::error!("Error loading head text: {:?}", err),
        }
        None
    }

    fn branch_name(&self) -> Option<String> {
        let head = self.head().log_err()?;
        let branch = String::from_utf8_lossy(head.shorthand_bytes());
        Some(branch.to_string())
    }

    fn statuses(&self) -> Option<TreeMap<RepoPath, GitFileStatus>> {
        let statuses = self.statuses(None).log_err()?;

        let mut map = TreeMap::default();

        for status in statuses
            .iter()
            .filter(|status| !status.status().contains(git2::Status::IGNORED))
        {
            let path = RepoPath(PathBuf::from(OsStr::from_bytes(status.path_bytes())));
            let Some(status) = read_status(status.status()) else {
                continue
            };

            map.insert(path, status)
        }

        Some(map)
    }

    fn status(&self, path: &RepoPath) -> Result<Option<GitFileStatus>> {
        let status = self.status_file(path);
        match status {
            Ok(status) => Ok(read_status(status)),
            Err(e) => {
                if e.code() == ErrorCode::NotFound {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }
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
    pub worktree_statuses: HashMap<RepoPath, GitFileStatus>,
    pub branch_name: Option<String>,
}

impl FakeGitRepository {
    pub fn open(state: Arc<Mutex<FakeGitRepositoryState>>) -> Arc<Mutex<dyn GitRepository>> {
        Arc::new(Mutex::new(FakeGitRepository { state }))
    }
}

#[async_trait::async_trait]
impl GitRepository for FakeGitRepository {
    fn reload_index(&self) {}

    fn load_index_text(&self, path: &Path) -> Option<String> {
        let state = self.state.lock();
        state.index_contents.get(path).cloned()
    }

    fn branch_name(&self) -> Option<String> {
        let state = self.state.lock();
        state.branch_name.clone()
    }

    fn statuses(&self) -> Option<TreeMap<RepoPath, GitFileStatus>> {
        let state = self.state.lock();
        let mut map = TreeMap::default();
        for (repo_path, status) in state.worktree_statuses.iter() {
            map.insert(repo_path.to_owned(), status.to_owned());
        }
        Some(map)
    }

    fn status(&self, path: &RepoPath) -> Result<Option<GitFileStatus>> {
        let state = self.state.lock();
        Ok(state.worktree_statuses.get(path).cloned())
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
        } else {
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

    pub fn from_proto(git_status: Option<i32>) -> Option<GitFileStatus> {
        git_status.and_then(|status| {
            proto::GitStatus::from_i32(status).map(|status| match status {
                proto::GitStatus::Added => GitFileStatus::Added,
                proto::GitStatus::Modified => GitFileStatus::Modified,
                proto::GitStatus::Conflict => GitFileStatus::Conflict,
            })
        })
    }

    pub fn to_proto(self) -> i32 {
        match self {
            GitFileStatus::Added => proto::GitStatus::Added as i32,
            GitFileStatus::Modified => proto::GitStatus::Modified as i32,
            GitFileStatus::Conflict => proto::GitStatus::Conflict as i32,
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
        if key.starts_with(&self.0) {
            Ordering::Greater
        } else {
            self.0.cmp(key)
        }
    }
}
