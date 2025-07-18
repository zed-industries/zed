use ignore::gitignore::Gitignore;
use std::{ffi::OsStr, path::Path, sync::Arc};

#[derive(Debug)]
pub enum IgnoreStack {
    None,
    Global {
        repo_root: Option<Arc<Path>>,
        ignore: Arc<Gitignore>,
    },
    Some {
        abs_base_path: Arc<Path>,
        ignore: Arc<Gitignore>,
        parent: Arc<IgnoreStack>,
    },
    All,
}

impl IgnoreStack {
    pub fn none() -> Arc<Self> {
        Arc::new(Self::None)
    }

    pub fn all() -> Arc<Self> {
        Arc::new(Self::All)
    }

    pub fn global(repo_root: Option<Arc<Path>>, ignore: Arc<Gitignore>) -> Arc<Self> {
        Arc::new(Self::Global { repo_root, ignore })
    }

    pub fn append(self: Arc<Self>, abs_base_path: Arc<Path>, ignore: Arc<Gitignore>) -> Arc<Self> {
        match self.as_ref() {
            IgnoreStack::All => self,
            _ => Arc::new(Self::Some {
                abs_base_path,
                ignore,
                parent: self,
            }),
        }
    }

    pub fn is_abs_path_ignored(&self, abs_path: &Path, is_dir: bool) -> bool {
        if is_dir && abs_path.file_name() == Some(OsStr::new(".git")) {
            return true;
        }

        match self {
            Self::None => false,
            Self::All => true,
            Self::Global { repo_root, ignore } => {
                let combined_path;
                let abs_path = if let Some(repo_root) = repo_root {
                    combined_path = ignore.path().join(
                        abs_path
                            .strip_prefix(repo_root)
                            .expect("repo root should be a parent of matched path"),
                    );
                    &combined_path
                } else {
                    abs_path
                };
                match ignore.matched(abs_path, is_dir) {
                    ignore::Match::None => false,
                    ignore::Match::Ignore(_) => true,
                    ignore::Match::Whitelist(_) => false,
                }
            }
            Self::Some {
                abs_base_path,
                ignore,
                parent: prev,
            } => match ignore.matched(abs_path.strip_prefix(abs_base_path).unwrap(), is_dir) {
                ignore::Match::None => prev.is_abs_path_ignored(abs_path, is_dir),
                ignore::Match::Ignore(_) => true,
                ignore::Match::Whitelist(_) => false,
            },
        }
    }
}
