use anyhow::Result;
use collections::HashMap;
use parking_lot::Mutex;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub use git2::Repository as LibGitRepository;

#[async_trait::async_trait]
pub trait GitRepository: Send {
    fn load_head_text(&self, relative_file_path: &Path) -> Option<String>;
}

#[async_trait::async_trait]
impl GitRepository for LibGitRepository {
    fn load_head_text(&self, relative_file_path: &Path) -> Option<String> {
        fn logic(repo: &LibGitRepository, relative_file_path: &Path) -> Result<Option<String>> {
            const STAGE_NORMAL: i32 = 0;
            let index = repo.index()?;
            let oid = match index.get_path(relative_file_path, STAGE_NORMAL) {
                Some(entry) => entry.id,
                None => return Ok(None),
            };

            let content = repo.find_blob(oid)?.content().to_owned();
            let head_text = String::from_utf8(content)?;
            Ok(Some(head_text))
        }

        match logic(&self, relative_file_path) {
            Ok(value) => return value,
            Err(err) => log::error!("Error loading head text: {:?}", err),
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct FakeGitRepository {
    state: Arc<Mutex<FakeGitRepositoryState>>,
}

#[derive(Debug, Clone, Default)]
pub struct FakeGitRepositoryState {
    pub index_contents: HashMap<PathBuf, String>,
}

impl FakeGitRepository {
    pub fn open(state: Arc<Mutex<FakeGitRepositoryState>>) -> Arc<Mutex<dyn GitRepository>> {
        Arc::new(Mutex::new(FakeGitRepository { state }))
    }
}

#[async_trait::async_trait]
impl GitRepository for FakeGitRepository {
    fn load_head_text(&self, path: &Path) -> Option<String> {
        let state = self.state.lock();
        state.index_contents.get(path).cloned()
    }
}
