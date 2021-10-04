use ignore::gitignore::Gitignore;
use std::{ffi::OsStr, path::Path, sync::Arc};

pub enum IgnoreStack {
    None,
    Some {
        base: Arc<Path>,
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

    pub fn is_all(&self) -> bool {
        matches!(self, IgnoreStack::All)
    }

    pub fn append(self: Arc<Self>, base: Arc<Path>, ignore: Arc<Gitignore>) -> Arc<Self> {
        match self.as_ref() {
            IgnoreStack::All => self,
            _ => Arc::new(Self::Some {
                base,
                ignore,
                parent: self,
            }),
        }
    }

    pub fn is_path_ignored(&self, path: &Path, is_dir: bool) -> bool {
        if is_dir && path.file_name() == Some(OsStr::new(".git")) {
            return true;
        }

        match self {
            Self::None => false,
            Self::All => true,
            Self::Some {
                base,
                ignore,
                parent: prev,
            } => match ignore.matched(path.strip_prefix(base).unwrap(), is_dir) {
                ignore::Match::None => prev.is_path_ignored(path, is_dir),
                ignore::Match::Ignore(_) => true,
                ignore::Match::Whitelist(_) => false,
            },
        }
    }
}
