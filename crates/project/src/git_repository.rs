use git2::Repository;
use parking_lot::Mutex;
use std::{path::Path, sync::Arc};
use util::ResultExt;

pub trait GitRepository: Send + Sync {
    fn boxed_clone(&self) -> Box<dyn GitRepository>;
    fn is_path_managed_by(&self, path: &Path) -> bool;
    fn is_path_in_git_folder(&self, path: &Path) -> bool;
    fn content_path(&self) -> &Path;
    fn git_dir_path(&self) -> &Path;
    fn last_scan_id(&self) -> usize;
    fn set_scan_id(&mut self, scan_id: usize);
}

#[derive(Clone)]
pub struct RealGitRepository {
    // Path to folder containing the .git file or directory
    content_path: Arc<Path>,
    // Path to the actual .git folder.
    // Note: if .git is a file, this points to the folder indicated by the .git file
    git_dir_path: Arc<Path>,
    last_scan_id: usize,
    libgit_repository: Arc<Mutex<git2::Repository>>,
}

impl RealGitRepository {
    pub fn open(
        abs_dotgit_path: &Path,
        content_path: &Arc<Path>,
    ) -> Option<Box<dyn GitRepository>> {
        Repository::open(&abs_dotgit_path)
            .log_err()
            .map::<Box<dyn GitRepository>, _>(|libgit_repository| {
                Box::new(Self {
                    content_path: content_path.clone(),
                    git_dir_path: libgit_repository.path().into(),
                    last_scan_id: 0,
                    libgit_repository: Arc::new(parking_lot::Mutex::new(libgit_repository)),
                })
            })
    }
}

impl GitRepository for RealGitRepository {
    fn boxed_clone(&self) -> Box<dyn GitRepository> {
        Box::new(self.clone())
    }

    fn is_path_managed_by(&self, path: &Path) -> bool {
        path.starts_with(&self.content_path)
    }

    fn is_path_in_git_folder(&self, path: &Path) -> bool {
        path.starts_with(&self.git_dir_path)
    }

    fn content_path(&self) -> &Path {
        self.content_path.as_ref()
    }

    fn git_dir_path(&self) -> &Path {
        self.git_dir_path.as_ref()
    }

    fn last_scan_id(&self) -> usize {
        self.last_scan_id
    }

    fn set_scan_id(&mut self, scan_id: usize) {
        self.last_scan_id = scan_id;
    }
}

impl PartialEq for &Box<dyn GitRepository> {
    fn eq(&self, other: &Self) -> bool {
        self.content_path() == other.content_path()
    }
}
impl Eq for &Box<dyn GitRepository> {}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone)]
pub struct FakeGitRepository {
    // Path to folder containing the .git file or directory
    content_path: Arc<Path>,
    // Path to the actual .git folder.
    // Note: if .git is a file, this points to the folder indicated by the .git file
    git_dir_path: Arc<Path>,
    last_scan_id: usize,
}

impl FakeGitRepository {
    pub fn new(abs_dotgit_path: &Path, content_path: &Arc<Path>) -> FakeGitRepository {
        Self {
            content_path: content_path.clone(),
            git_dir_path: abs_dotgit_path.into(),
            last_scan_id: 0,
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl GitRepository for FakeGitRepository {
    fn boxed_clone(&self) -> Box<dyn GitRepository> {
        Box::new(self.clone())
    }

    fn is_path_managed_by(&self, path: &Path) -> bool {
        path.starts_with(&self.content_path)
    }

    fn is_path_in_git_folder(&self, path: &Path) -> bool {
        path.starts_with(&self.git_dir_path)
    }

    fn content_path(&self) -> &Path {
        self.content_path.as_ref()
    }

    fn git_dir_path(&self) -> &Path {
        self.git_dir_path.as_ref()
    }

    fn last_scan_id(&self) -> usize {
        self.last_scan_id
    }

    fn set_scan_id(&mut self, scan_id: usize) {
        self.last_scan_id = scan_id;
    }
}
