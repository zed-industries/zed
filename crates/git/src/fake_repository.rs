use crate::{
    blame::Blame,
    repository::{
        Branch, CommitDetails, DiffType, GitRepository, PushOptions, Remote, RemoteCommandOutput,
        RepoPath, ResetMode,
    },
    status::{FileStatus, GitStatus},
};
use anyhow::{Context, Result};
use askpass::AskPassSession;
use collections::{HashMap, HashSet};
use futures::{future::BoxFuture, FutureExt as _};
use gpui::{AsyncApp, SharedString};
use parking_lot::Mutex;
use rope::Rope;
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone)]
pub struct FakeGitRepository {
    state: Arc<Mutex<FakeGitRepositoryState>>,
}

#[derive(Debug, Clone)]
pub struct FakeGitRepositoryState {
    pub path: PathBuf,
    pub event_emitter: smol::channel::Sender<PathBuf>,
    pub head_contents: HashMap<RepoPath, String>,
    pub index_contents: HashMap<RepoPath, String>,
    pub blames: HashMap<RepoPath, Blame>,
    pub statuses: HashMap<RepoPath, FileStatus>,
    pub current_branch_name: Option<String>,
    pub branches: HashSet<String>,
    pub simulated_index_write_error_message: Option<String>,
}

impl FakeGitRepository {
    pub fn open(state: Arc<Mutex<FakeGitRepositoryState>>) -> Arc<dyn GitRepository> {
        Arc::new(FakeGitRepository { state })
    }
}

impl FakeGitRepositoryState {
    pub fn new(path: PathBuf, event_emitter: smol::channel::Sender<PathBuf>) -> Self {
        FakeGitRepositoryState {
            path,
            event_emitter,
            head_contents: Default::default(),
            index_contents: Default::default(),
            blames: Default::default(),
            statuses: Default::default(),
            current_branch_name: Default::default(),
            branches: Default::default(),
            simulated_index_write_error_message: None,
        }
    }
}

impl GitRepository for FakeGitRepository {
    fn reload_index(&self) {}

    fn load_index_text(&self, path: RepoPath, _: AsyncApp) -> BoxFuture<Option<String>> {
        let state = self.state.lock();
        let content = state.index_contents.get(path.as_ref()).cloned();
        async { content }.boxed()
    }

    fn load_committed_text(&self, path: RepoPath, _: AsyncApp) -> BoxFuture<Option<String>> {
        let state = self.state.lock();
        let content = state.head_contents.get(path.as_ref()).cloned();
        async { content }.boxed()
    }

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        _env: HashMap<String, String>,
        cx: AsyncApp,
    ) -> BoxFuture<anyhow::Result<()>> {
        let state = self.state.clone();
        let executor = cx.background_executor().clone();
        async move {
            executor.simulate_random_delay().await;

            let mut state = state.lock();
            if let Some(message) = state.simulated_index_write_error_message.clone() {
                return Err(anyhow::anyhow!(message));
            }

            if let Some(content) = content {
                state.index_contents.insert(path.clone(), content);
            } else {
                state.index_contents.remove(&path);
            }
            state
                .event_emitter
                .try_send(state.path.clone())
                .expect("Dropped repo change event");

            Ok(())
        }
        .boxed()
    }

    fn remote_url(&self, _name: &str) -> Option<String> {
        None
    }

    fn head_sha(&self) -> Option<String> {
        None
    }

    fn merge_head_shas(&self) -> Vec<String> {
        vec![]
    }

    fn show(&self, _: String, _: AsyncApp) -> BoxFuture<Result<CommitDetails>> {
        unimplemented!()
    }

    fn reset(&self, _: String, _: ResetMode, _: HashMap<String, String>) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn checkout_files(
        &self,
        _: String,
        _: Vec<RepoPath>,
        _: HashMap<String, String>,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn path(&self) -> PathBuf {
        let state = self.state.lock();
        state.path.clone()
    }

    fn main_repository_path(&self) -> PathBuf {
        self.path()
    }

    fn status(&self, path_prefixes: &[RepoPath]) -> Result<GitStatus> {
        let state = self.state.lock();

        let mut entries = state
            .statuses
            .iter()
            .filter_map(|(repo_path, status)| {
                if path_prefixes
                    .iter()
                    .any(|path_prefix| repo_path.0.starts_with(path_prefix))
                {
                    Some((repo_path.to_owned(), *status))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        entries.sort_unstable_by(|(a, _), (b, _)| a.cmp(&b));

        Ok(GitStatus {
            entries: entries.into(),
        })
    }

    fn branches(&self) -> BoxFuture<Result<Vec<Branch>>> {
        let state = self.state.lock();
        let current_branch = &state.current_branch_name;
        let result = Ok(state
            .branches
            .iter()
            .map(|branch_name| Branch {
                is_head: Some(branch_name) == current_branch.as_ref(),
                name: branch_name.into(),
                most_recent_commit: None,
                upstream: None,
            })
            .collect());

        async { result }.boxed()
    }

    fn change_branch(&self, name: String, _: AsyncApp) -> BoxFuture<Result<()>> {
        let mut state = self.state.lock();
        state.current_branch_name = Some(name.to_owned());
        state
            .event_emitter
            .try_send(state.path.clone())
            .expect("Dropped repo change event");
        async { Ok(()) }.boxed()
    }

    fn create_branch(&self, name: String, _: AsyncApp) -> BoxFuture<Result<()>> {
        let mut state = self.state.lock();
        state.branches.insert(name.to_owned());
        state
            .event_emitter
            .try_send(state.path.clone())
            .expect("Dropped repo change event");
        async { Ok(()) }.boxed()
    }

    fn blame(
        &self,
        path: RepoPath,
        _content: Rope,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<crate::blame::Blame>> {
        let state = self.state.lock();
        let result = state
            .blames
            .get(&path)
            .with_context(|| format!("failed to get blame for {:?}", path.0))
            .cloned();
        async { result }.boxed()
    }

    fn stage_paths(
        &self,
        _paths: Vec<RepoPath>,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn unstage_paths(
        &self,
        _paths: Vec<RepoPath>,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn commit(
        &self,
        _message: SharedString,
        _name_and_email: Option<(SharedString, SharedString)>,
        _env: HashMap<String, String>,
        _: AsyncApp,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn push(
        &self,
        _branch: String,
        _remote: String,
        _options: Option<PushOptions>,
        _ask_pass: AskPassSession,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        unimplemented!()
    }

    fn pull(
        &self,
        _branch: String,
        _remote: String,
        _ask_pass: AskPassSession,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        unimplemented!()
    }

    fn fetch(
        &self,
        _ask_pass: AskPassSession,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<RemoteCommandOutput>> {
        unimplemented!()
    }

    fn get_remotes(
        &self,
        _branch: Option<String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<Vec<Remote>>> {
        unimplemented!()
    }

    fn check_for_pushed_commit(&self, _cx: AsyncApp) -> BoxFuture<Result<Vec<SharedString>>> {
        unimplemented!()
    }

    fn diff(&self, _diff: DiffType, _cx: AsyncApp) -> BoxFuture<Result<String>> {
        unimplemented!()
    }
}
