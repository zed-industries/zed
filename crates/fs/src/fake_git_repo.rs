use crate::FakeFs;
use anyhow::{anyhow, Context as _, Result};
use collections::{HashMap, HashSet};
use futures::future::{self, BoxFuture};
use git::{
    blame::Blame,
    repository::{
        AskPassSession, Branch, CommitDetails, GitRepository, PushOptions, Remote, RepoPath,
        ResetMode,
    },
    status::{FileStatus, GitStatus, StatusCode, TrackedStatus, UnmergedStatus},
};
use gpui::AsyncApp;
use rope::Rope;
use smol::future::FutureExt as _;
use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct FakeGitRepository {
    pub(crate) fs: Arc<FakeFs>,
    pub(crate) dot_git_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FakeGitRepositoryState {
    pub path: PathBuf,
    pub event_emitter: smol::channel::Sender<PathBuf>,
    pub unmerged_paths: HashMap<RepoPath, UnmergedStatus>,
    pub head_contents: HashMap<RepoPath, String>,
    pub index_contents: HashMap<RepoPath, String>,
    pub blames: HashMap<RepoPath, Blame>,
    pub current_branch_name: Option<String>,
    pub branches: HashSet<String>,
    pub simulated_index_write_error_message: Option<String>,
}

impl FakeGitRepositoryState {
    pub fn new(path: PathBuf, event_emitter: smol::channel::Sender<PathBuf>) -> Self {
        FakeGitRepositoryState {
            path,
            event_emitter,
            head_contents: Default::default(),
            index_contents: Default::default(),
            unmerged_paths: Default::default(),
            blames: Default::default(),
            current_branch_name: Default::default(),
            branches: Default::default(),
            simulated_index_write_error_message: Default::default(),
        }
    }
}

impl FakeGitRepository {
    fn with_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut FakeGitRepositoryState) -> T,
    {
        self.fs.with_git_state(&self.dot_git_path, false, f)
    }
}

impl GitRepository for FakeGitRepository {
    fn reload_index(&self) {}

    fn load_index_text(&self, path: RepoPath, _cx: AsyncApp) -> BoxFuture<Option<String>> {
        future::ready(self.with_state(|state| state.index_contents.get(path.as_ref()).cloned()))
            .boxed()
    }

    fn load_committed_text(&self, path: RepoPath, _cx: AsyncApp) -> BoxFuture<Option<String>> {
        future::ready(self.with_state(|state| state.head_contents.get(path.as_ref()).cloned()))
            .boxed()
    }

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<anyhow::Result<()>> {
        future::ready(self.with_state(|state| {
            if let Some(message) = state.simulated_index_write_error_message.clone() {
                return Err(anyhow!("{}", message));
            } else if let Some(content) = content {
                state.index_contents.insert(path, content);
            } else {
                state.index_contents.remove(&path);
            }
            state
                .event_emitter
                .try_send(state.path.clone())
                .expect("Dropped repo change event");
            Ok(())
        }))
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

    fn show(&self, _commit: String, _cx: AsyncApp) -> BoxFuture<Result<CommitDetails>> {
        unimplemented!()
    }

    fn reset(
        &self,
        _commit: String,
        _mode: ResetMode,
        _env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn checkout_files(
        &self,
        _commit: String,
        _paths: Vec<RepoPath>,
        _env: HashMap<String, String>,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn path(&self) -> PathBuf {
        self.with_state(|state| state.path.clone())
    }

    fn main_repository_path(&self) -> PathBuf {
        self.path()
    }

    fn status(&self, path_prefixes: &[RepoPath]) -> Result<GitStatus> {
        let git_files: HashMap<_, _> = self
            .fs
            .files()
            .iter()
            .filter_map(|path| {
                let repo_path = RepoPath::new(
                    path.strip_prefix(self.dot_git_path.parent().unwrap())
                        .ok()?
                        .into(),
                );
                let content = self
                    .fs
                    .read_file_sync(path)
                    .ok()
                    .map(|content| String::from_utf8(content).unwrap());
                Some((repo_path, content?))
            })
            .collect();
        self.with_state(|state| {
            let mut entries = Vec::new();
            let paths = state
                .head_contents
                .keys()
                .chain(state.index_contents.keys())
                .chain(git_files.keys())
                .collect::<HashSet<_>>();
            for path in paths {
                if !path_prefixes.iter().any(|prefix| path.starts_with(prefix)) {
                    continue;
                }

                let head = state.head_contents.get(path);
                let index = state.index_contents.get(path);
                let unmerged = state.unmerged_paths.get(path);
                let fs = git_files.get(path);
                let status = match (unmerged, head, index, fs) {
                    (Some(unmerged), _, _, _) => FileStatus::Unmerged(unmerged.clone()),
                    (_, Some(head), Some(index), Some(fs)) => FileStatus::Tracked(TrackedStatus {
                        index_status: if head == index {
                            StatusCode::Unmodified
                        } else {
                            StatusCode::Modified
                        },
                        worktree_status: if fs == index {
                            StatusCode::Unmodified
                        } else {
                            StatusCode::Modified
                        },
                    }),
                    (_, Some(head), Some(index), None) => FileStatus::Tracked(TrackedStatus {
                        index_status: if head == index {
                            StatusCode::Unmodified
                        } else {
                            StatusCode::Modified
                        },
                        worktree_status: StatusCode::Deleted,
                    }),
                    (_, Some(_), None, Some(_)) => FileStatus::Tracked(TrackedStatus {
                        index_status: StatusCode::Deleted,
                        worktree_status: StatusCode::Added,
                    }),
                    (_, Some(_), None, None) => FileStatus::Tracked(TrackedStatus {
                        index_status: StatusCode::Deleted,
                        worktree_status: StatusCode::Deleted,
                    }),
                    (_, None, Some(index), Some(fs)) => FileStatus::Tracked(TrackedStatus {
                        index_status: StatusCode::Added,
                        worktree_status: if fs == index {
                            StatusCode::Unmodified
                        } else {
                            StatusCode::Modified
                        },
                    }),
                    (_, None, Some(_), None) => FileStatus::Tracked(TrackedStatus {
                        index_status: StatusCode::Added,
                        worktree_status: StatusCode::Deleted,
                    }),
                    (_, None, None, Some(_)) => FileStatus::Untracked,
                    (_, None, None, None) => {
                        unreachable!();
                    }
                };
                entries.push((path.clone(), status));
            }
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Ok(GitStatus {
                entries: entries.into(),
            })
        })
    }

    fn branches(&self) -> BoxFuture<Result<Vec<Branch>>> {
        future::ready(self.with_state(|state| {
            let current_branch = &state.current_branch_name;
            Ok(state
                .branches
                .iter()
                .map(|branch_name| Branch {
                    is_head: Some(branch_name) == current_branch.as_ref(),
                    name: branch_name.into(),
                    most_recent_commit: None,
                    upstream: None,
                })
                .collect())
        }))
        .boxed()
    }

    fn change_branch(&self, name: String, _cx: AsyncApp) -> BoxFuture<Result<()>> {
        future::ready(self.with_state(|state| {
            state.current_branch_name = Some(name);
            state
                .event_emitter
                .try_send(state.path.clone())
                .expect("Dropped repo change event");
            Ok(())
        }))
        .boxed()
    }

    fn create_branch(&self, name: String, _: AsyncApp) -> BoxFuture<Result<()>> {
        future::ready(self.with_state(|state| {
            state.branches.insert(name.to_owned());
            state
                .event_emitter
                .try_send(state.path.clone())
                .expect("Dropped repo change event");
            Ok(())
        }))
        .boxed()
    }

    fn blame(
        &self,
        path: RepoPath,
        _content: Rope,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<git::blame::Blame>> {
        future::ready(self.with_state(|state| {
            state
                .blames
                .get(&path)
                .with_context(|| format!("failed to get blame for {:?}", path))
                .cloned()
        }))
        .boxed()
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
        _message: gpui::SharedString,
        _name_and_email: Option<(gpui::SharedString, gpui::SharedString)>,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn push(
        &self,
        _branch: String,
        _remote: String,
        _options: Option<PushOptions>,
        _askpass: AskPassSession,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<git::repository::RemoteCommandOutput>> {
        unimplemented!()
    }

    fn pull(
        &self,
        _branch: String,
        _remote: String,
        _askpass: AskPassSession,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<git::repository::RemoteCommandOutput>> {
        unimplemented!()
    }

    fn fetch(
        &self,
        _askpass: AskPassSession,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<git::repository::RemoteCommandOutput>> {
        unimplemented!()
    }

    fn get_remotes(
        &self,
        _branch: Option<String>,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<Vec<Remote>>> {
        unimplemented!()
    }

    fn check_for_pushed_commit(
        &self,
        _cx: gpui::AsyncApp,
    ) -> BoxFuture<Result<Vec<gpui::SharedString>>> {
        future::ready(Ok(Vec::new())).boxed()
    }

    fn diff(
        &self,
        _diff: git::repository::DiffType,
        _cx: gpui::AsyncApp,
    ) -> BoxFuture<Result<String>> {
        unimplemented!()
    }
}
