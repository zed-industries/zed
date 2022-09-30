use anyhow::Result;
use collections::HashMap;
use git2::Repository as LibGitRepository;
use parking_lot::Mutex;
use util::ResultExt;
use std::{path::{Path, PathBuf}, sync::Arc};

#[async_trait::async_trait]
pub trait GitRepository: Send {
    // fn manages(&self, path: &Path) -> bool;
    // fn reopen_git_repo(&mut self) -> bool;
    // fn git_repo(&self) -> Arc<Mutex<LibGitRepository>>;
    // fn boxed_clone(&self) -> Box<dyn GitRepository>;

    fn load_head_text(&self, relative_file_path: &Path) -> Option<String>;
    
    fn open_real(dotgit_path: &Path) -> Option<Arc<Mutex<dyn GitRepository>>>
    where Self: Sized
    {
        LibGitRepository::open(&dotgit_path)
            .log_err()
            .and_then::<Arc<Mutex<dyn GitRepository>>, _>(|libgit_repository| {
                Some(Arc::new(Mutex::new(libgit_repository)))
            })
    }
}

#[async_trait::async_trait]
impl GitRepository for LibGitRepository {
    // fn manages(&self, path: &Path) -> bool {
    //     path.canonicalize()
    //         .map(|path| path.starts_with(&self.content_path))
    //         .unwrap_or(false)
    // }


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

#[derive(Debug, Clone)]
pub struct FakeGitRepository {
    content_path: Arc<Path>,
    git_dir_path: Arc<Path>,
    state: Arc<Mutex<FakeGitRepositoryState>>,
}

#[derive(Debug, Clone, Default)]
pub struct FakeGitRepositoryState {
    pub index_contents: HashMap<PathBuf, String>,
}

impl FakeGitRepository {
    pub fn open(dotgit_path: &Path, state: Arc<Mutex<FakeGitRepositoryState>>) -> Box<dyn GitRepository> {
        Box::new(FakeGitRepository {
            content_path: dotgit_path.parent().unwrap().into(),
            git_dir_path: dotgit_path.into(),
            state,
        })
    }
}

#[async_trait::async_trait]
impl GitRepository for FakeGitRepository {
    fn manages(&self, path: &Path) -> bool {
        path.starts_with(self.content_path())
    }

    // fn in_dot_git(&self, path: &Path) -> bool {
    //     path.starts_with(self.git_dir_path())
    // }

    fn content_path(&self) -> &Path {
        &self.content_path
    }

    fn git_dir_path(&self) -> &Path {
        &self.git_dir_path
    }

    async fn load_head_text(&self, path: &Path) -> Option<String> {
        let state = self.state.lock();
        state.index_contents.get(path).cloned()
    }

    fn reopen_git_repo(&mut self) -> bool {
        true
    }

    fn git_repo(&self) -> Arc<Mutex<LibGitRepository>> {
        unimplemented!()
    }

    fn boxed_clone(&self) -> Box<dyn GitRepository> {
        Box::new(self.clone())
    }
}
