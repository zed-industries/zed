use anyhow::Result;
use collections::HashMap;
use parking_lot::Mutex;
use sum_tree::TreeMap;
use std::{
    path::{Component, Path, PathBuf},
    sync::Arc, ffi::OsStr, os::unix::prelude::OsStrExt,
};
use util::ResultExt;

pub use git2::Repository as LibGitRepository;

#[async_trait::async_trait]
pub trait GitRepository: Send {
    fn reload_index(&self);

    fn load_index_text(&self, relative_file_path: &Path) -> Option<String>;

    fn branch_name(&self) -> Option<String>;

    fn statuses(&self) -> Option<TreeMap<RepoPath, GitStatus>>;
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

    fn statuses(&self) -> Option<TreeMap<RepoPath, GitStatus>> {
        let statuses = self.statuses(None).log_err()?;

        let mut map = TreeMap::default();

        for status in statuses.iter() {
            let path = RepoPath(PathBuf::from(OsStr::from_bytes(status.path_bytes())));

            let status_data = status.status();

            let status = if status_data.contains(git2::Status::CONFLICTED) {
                GitStatus::Conflict
            } else if status_data.intersects(git2::Status::INDEX_MODIFIED
                | git2::Status::WT_MODIFIED
                | git2::Status::INDEX_RENAMED
                | git2::Status::WT_RENAMED) {
                GitStatus::Modified
            } else if status_data.intersects(git2::Status::INDEX_NEW | git2::Status::WT_NEW) {
                GitStatus::Added
            } else {
                GitStatus::Untracked
            };

            map.insert(path, status)
        }

        Some(map)
    }
}

#[derive(Debug, Clone, Default)]
pub enum GitStatus {
    Added,
    Modified,
    Conflict,
    #[default]
    Untracked,
}

#[derive(Clone, Debug, Ord, Hash, PartialOrd, Eq, PartialEq)]
pub struct RepoPath(PathBuf);

impl From<&Path> for RepoPath {
    fn from(value: &Path) -> Self {
        RepoPath(value.to_path_buf())
    }
}

impl From<PathBuf> for RepoPath {
    fn from(value: PathBuf) -> Self {
        RepoPath(value)
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

#[derive(Debug, Clone, Default)]
pub struct FakeGitRepository {
    state: Arc<Mutex<FakeGitRepositoryState>>,
}

#[derive(Debug, Clone, Default)]
pub struct FakeGitRepositoryState {
    pub index_contents: HashMap<PathBuf, String>,
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

    fn statuses(&self) -> Option<TreeMap<RepoPath, GitStatus>>{
        todo!()
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
