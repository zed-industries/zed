use std::{path::Path, sync::Arc};

use ignore::gitignore::Gitignore;

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

    pub fn append(self: Arc<Self>, base: Arc<Path>, ignore: Arc<Gitignore>) -> Arc<Self> {
        log::info!("appending ignore {:?}", base);
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
        println!("is_path_ignored? {:?} {}", path, is_dir);
        match self {
            Self::None => {
                println!("none case");
                false
            }
            Self::All => {
                println!("all case");
                true
            }
            Self::Some {
                base,
                ignore,
                parent: prev,
            } => {
                println!(
                    "some case {:?} {:?}",
                    base,
                    path.strip_prefix(base).unwrap()
                );

                match ignore.matched(path.strip_prefix(base).unwrap(), is_dir) {
                    ignore::Match::None => prev.is_path_ignored(path, is_dir),
                    ignore::Match::Ignore(_) => true,
                    ignore::Match::Whitelist(_) => false,
                }
            }
        }
    }
}
