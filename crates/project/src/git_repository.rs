use git2::Repository;
use parking_lot::Mutex;
use std::{path::Path, sync::Arc};
use util::ResultExt;

#[derive(Clone)]
pub struct GitRepository {
    // Path to folder containing the .git file or directory
    content_path: Arc<Path>,
    // Path to the actual .git folder.
    // Note: if .git is a file, this points to the folder indicated by the .git file
    git_dir_path: Arc<Path>,
    last_scan_id: usize,
    libgit_repository: Arc<Mutex<git2::Repository>>,
}

impl GitRepository {
    pub fn open(dotgit_path: &Path) -> Option<GitRepository> {
        Repository::open(&dotgit_path)
            .log_err()
            .and_then(|libgit_repository| {
                Some(Self {
                    content_path: libgit_repository.workdir()?.into(),
                    git_dir_path: dotgit_path.canonicalize().log_err()?.into(),
                    last_scan_id: 0,
                    libgit_repository: Arc::new(parking_lot::Mutex::new(libgit_repository)),
                })
            })
    }

    pub fn is_path_managed_by(&self, path: &Path) -> bool {
        path.canonicalize()
            .map(|path| path.starts_with(&self.content_path))
            .unwrap_or(false)
    }

    pub fn is_path_in_git_folder(&self, path: &Path) -> bool {
        path.canonicalize()
            .map(|path| path.starts_with(&self.git_dir_path))
            .unwrap_or(false)
    }

    pub fn content_path(&self) -> &Path {
        self.content_path.as_ref()
    }

    pub fn git_dir_path(&self) -> &Path {
        self.git_dir_path.as_ref()
    }

    pub fn last_scan_id(&self) -> usize {
        self.last_scan_id
    }

    pub fn set_scan_id(&mut self, scan_id: usize) {
        self.last_scan_id = scan_id;
    }

    pub fn with_repo(&mut self, f: Box<dyn FnOnce(&mut git2::Repository)>) {
        let mut git2 = self.libgit_repository.lock();
        f(&mut git2)
    }
}
