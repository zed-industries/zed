use ignore::gitignore::Gitignore;
use std::{path::Path, sync::Arc};

pub enum IgnoreStack {
    None,
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

    pub fn is_all(&self) -> bool {
        matches!(self, IgnoreStack::All)
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
}
