use ignore::gitignore::Gitignore;
use std::{ffi::OsStr, path::Path, sync::Arc};

#[derive(Clone, Debug)]
pub struct IgnoreStack {
    pub repo_root: Option<Arc<Path>>,
    pub top: Arc<IgnoreStackEntry>,
}

#[derive(Debug)]
pub enum IgnoreStackEntry {
    None,
    Global {
        ignore: Arc<Gitignore>,
    },
    Some {
        abs_base_path: Arc<Path>,
        ignore: Arc<Gitignore>,
        parent: Arc<IgnoreStackEntry>,
    },
    All,
}

impl IgnoreStack {
    pub fn none() -> Self {
        Self {
            repo_root: None,
            top: Arc::new(IgnoreStackEntry::None),
        }
    }

    pub fn all() -> Self {
        Self {
            repo_root: None,
            top: Arc::new(IgnoreStackEntry::All),
        }
    }

    pub fn global(ignore: Arc<Gitignore>) -> Self {
        Self {
            repo_root: None,
            top: Arc::new(IgnoreStackEntry::Global { ignore }),
        }
    }

    pub fn append(self, abs_base_path: Arc<Path>, ignore: Arc<Gitignore>) -> Self {
        let top = match self.top.as_ref() {
            IgnoreStackEntry::All => self.top.clone(),
            _ => Arc::new(IgnoreStackEntry::Some {
                abs_base_path,
                ignore,
                parent: self.top.clone(),
            }),
        };
        Self {
            repo_root: self.repo_root,
            top,
        }
    }

    pub fn is_abs_path_ignored(&self, abs_path: &Path, is_dir: bool) -> bool {
        if is_dir && abs_path.file_name() == Some(OsStr::new(".git")) {
            return true;
        }

        match self.top.as_ref() {
            IgnoreStackEntry::None => false,
            IgnoreStackEntry::All => true,
            IgnoreStackEntry::Global { ignore } => {
                let combined_path;
                let abs_path = if let Some(repo_root) = self.repo_root.as_ref() {
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
            IgnoreStackEntry::Some {
                abs_base_path,
                ignore,
                parent: prev,
            } => match ignore.matched(abs_path.strip_prefix(abs_base_path).unwrap(), is_dir) {
                ignore::Match::None => IgnoreStack {
                    repo_root: self.repo_root.clone(),
                    top: prev.clone(),
                }
                .is_abs_path_ignored(abs_path, is_dir),
                ignore::Match::Ignore(_) => true,
                ignore::Match::Whitelist(_) => false,
            },
        }
    }
}
