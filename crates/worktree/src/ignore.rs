use ignore::gitignore::Gitignore;
use git::repository::RepoPath;
use std::{ffi::OsStr, path::Path, sync::Arc};
use util::{paths::PathStyle, rel_path::RelPath};

#[derive(Clone, Debug, Default)]
pub struct TrackedPaths {
    paths: Arc<[RepoPath]>,
}

impl TrackedPaths {
    pub fn new(mut paths: Vec<RepoPath>) -> Self {
        paths.sort();
        paths.dedup();
        Self { paths: paths.into() }
    }

    pub fn contains(&self, path: &RepoPath) -> bool {
        self.paths.binary_search(path).is_ok()
    }

    pub fn contains_path_or_descendant(&self, path: &RepoPath) -> bool {
        let index = match self.paths.binary_search(path) {
            Ok(index) | Err(index) => index,
        };
        self.paths
            .get(index)
            .is_some_and(|candidate| candidate.starts_with(path))
    }
}

#[derive(Clone, Debug)]
pub struct IgnoreStack {
    pub repo_root: Option<Arc<Path>>,
    pub tracked_paths: Option<Arc<TrackedPaths>>,
    pub top: Arc<IgnoreStackEntry>,
}

#[derive(Debug)]
pub enum IgnoreStackEntry {
    None,
    Global {
        ignore: Arc<Gitignore>,
    },
    RepoExclude {
        ignore: Arc<Gitignore>,
        parent: Arc<IgnoreStackEntry>,
    },
    Some {
        abs_base_path: Arc<Path>,
        ignore: Arc<Gitignore>,
        parent: Arc<IgnoreStackEntry>,
    },
    All,
}

#[derive(Debug)]
pub enum IgnoreKind {
    Gitignore(Arc<Path>),
    RepoExclude,
}

impl IgnoreStack {
    pub fn none() -> Self {
        Self {
            repo_root: None,
            tracked_paths: None,
            top: Arc::new(IgnoreStackEntry::None),
        }
    }

    pub fn all() -> Self {
        Self {
            repo_root: None,
            tracked_paths: None,
            top: Arc::new(IgnoreStackEntry::All),
        }
    }

    pub fn global(ignore: Arc<Gitignore>) -> Self {
        Self {
            repo_root: None,
            tracked_paths: None,
            top: Arc::new(IgnoreStackEntry::Global { ignore }),
        }
    }

    pub fn append(self, kind: IgnoreKind, ignore: Arc<Gitignore>) -> Self {
        let top = match self.top.as_ref() {
            IgnoreStackEntry::All => self.top.clone(),
            _ => Arc::new(match kind {
                IgnoreKind::Gitignore(abs_base_path) => IgnoreStackEntry::Some {
                    abs_base_path,
                    ignore,
                    parent: self.top.clone(),
                },
                IgnoreKind::RepoExclude => IgnoreStackEntry::RepoExclude {
                    ignore,
                    parent: self.top.clone(),
                },
            }),
        };
        Self {
            repo_root: self.repo_root,
            tracked_paths: self.tracked_paths,
            top,
        }
    }

    fn matched_path_is_tracked(&self, abs_path: &Path, is_dir: bool) -> bool {
        let Some(repo_root) = self.repo_root.as_ref() else {
            return false;
        };
        let Some(tracked_paths) = self.tracked_paths.as_ref() else {
            return false;
        };
        let Ok(path_in_repo) = abs_path.strip_prefix(repo_root) else {
            return false;
        };
        let Ok(path_in_repo) = RelPath::new(path_in_repo, PathStyle::local()) else {
            return false;
        };
        let repo_path = RepoPath::from_rel_path(&path_in_repo);
        if is_dir {
            tracked_paths.contains_path_or_descendant(&repo_path)
        } else {
            tracked_paths.contains(&repo_path)
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
                    ignore::Match::Ignore(_) => !self.matched_path_is_tracked(abs_path, is_dir),
                    ignore::Match::Whitelist(_) => false,
                }
            }
            IgnoreStackEntry::RepoExclude { ignore, parent } => {
                match ignore.matched(abs_path, is_dir) {
                    ignore::Match::None => IgnoreStack {
                        repo_root: self.repo_root.clone(),
                        tracked_paths: self.tracked_paths.clone(),
                        top: parent.clone(),
                    }
                    .is_abs_path_ignored(abs_path, is_dir),
                    ignore::Match::Ignore(_) => !self.matched_path_is_tracked(abs_path, is_dir),
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
                    tracked_paths: self.tracked_paths.clone(),
                    top: prev.clone(),
                }
                .is_abs_path_ignored(abs_path, is_dir),
                ignore::Match::Ignore(_) => !self.matched_path_is_tracked(abs_path, is_dir),
                ignore::Match::Whitelist(_) => false,
            },
        }
    }
}
