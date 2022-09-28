use anyhow::Result;
use git2::{Repository as LibGitRepository, RepositoryOpenFlags as LibGitRepositoryOpenFlags};
use parking_lot::Mutex;
use std::{ffi::OsStr, path::Path, sync::Arc};
use util::ResultExt;

#[derive(Clone)]
pub struct GitRepository {
    // Path to folder containing the .git file or directory
    content_path: Arc<Path>,
    // Path to the actual .git folder.
    // Note: if .git is a file, this points to the folder indicated by the .git file
    git_dir_path: Arc<Path>,
    scan_id: usize,
    libgit_repository: Arc<Mutex<LibGitRepository>>,
}

impl GitRepository {
    pub fn open(dotgit_path: &Path) -> Option<GitRepository> {
        LibGitRepository::open(&dotgit_path)
            .log_err()
            .and_then(|libgit_repository| {
                Some(Self {
                    content_path: libgit_repository.workdir()?.into(),
                    git_dir_path: dotgit_path.canonicalize().log_err()?.into(),
                    scan_id: 0,
                    libgit_repository: Arc::new(parking_lot::Mutex::new(libgit_repository)),
                })
            })
    }

    pub fn manages(&self, path: &Path) -> bool {
        path.canonicalize()
            .map(|path| path.starts_with(&self.content_path))
            .unwrap_or(false)
    }

    pub fn in_dot_git(&self, path: &Path) -> bool {
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

    pub fn scan_id(&self) -> usize {
        self.scan_id
    }

    pub(super) fn set_scan_id(&mut self, scan_id: usize) {
        self.scan_id = scan_id;
    }

    pub fn with_repo<F: FnOnce(&mut git2::Repository)>(&mut self, f: F) {
        let mut git2 = self.libgit_repository.lock();
        f(&mut git2)
    }

    pub async fn load_head_text(&self, file_path: &Path) -> Option<String> {
        fn logic(repo: &LibGitRepository, file_path: &Path) -> Result<Option<String>> {
            let object = repo
                .head()?
                .peel_to_tree()?
                .get_path(file_path)?
                .to_object(&repo)?;

            let content = match object.as_blob() {
                Some(blob) => blob.content().to_owned(),
                None => return Ok(None),
            };

            let head_text = String::from_utf8(content.to_owned())?;
            Ok(Some(head_text))
        }

        match logic(&self.libgit_repository.lock(), file_path) {
            Ok(value) => return value,
            Err(err) => log::error!("Error loading head text: {:?}", err),
        }
        None
    }
}

impl std::fmt::Debug for GitRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitRepository")
            .field("content_path", &self.content_path)
            .field("git_dir_path", &self.git_dir_path)
            .field("scan_id", &self.scan_id)
            .field("libgit_repository", &"LibGitRepository")
            .finish()
    }
}
