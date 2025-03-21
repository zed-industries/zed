use crate::FakeFs;
use anyhow::{anyhow, Context as _, Result};
use collections::{HashMap, HashSet};
use futures::future::{self, BoxFuture};
use git::{
    blame::Blame,
    repository::{
        AskPassSession, Branch, CommitDetails, GitRepository, GitRepositoryCheckpoint, PushOptions,
        Remote, RepoPath, ResetMode,
    },
    status::{FileStatus, GitStatus, StatusCode, TrackedStatus, UnmergedStatus},
};
use gpui::{AsyncApp, BackgroundExecutor};
use ignore::gitignore::GitignoreBuilder;
use rope::Rope;
use smol::future::FutureExt as _;
use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct FakeGitRepository {
    pub(crate) fs: Arc<FakeFs>,
    pub(crate) executor: BackgroundExecutor,
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

    fn with_state_async<F, T>(&self, write: bool, f: F) -> BoxFuture<T>
    where
        F: 'static + Send + FnOnce(&mut FakeGitRepositoryState) -> T,
        T: Send,
    {
        let fs = self.fs.clone();
        let executor = self.executor.clone();
        let dot_git_path = self.dot_git_path.clone();
        async move {
            executor.simulate_random_delay().await;
            fs.with_git_state(&dot_git_path, write, f)
        }
        .boxed()
    }
}

impl GitRepository for FakeGitRepository {
    fn reload_index(&self) {}

    fn load_index_text(&self, path: RepoPath, _cx: AsyncApp) -> BoxFuture<Option<String>> {
        self.with_state_async(false, move |state| {
            state.index_contents.get(path.as_ref()).cloned()
        })
    }

    fn load_committed_text(&self, path: RepoPath, _cx: AsyncApp) -> BoxFuture<Option<String>> {
        self.with_state_async(false, move |state| {
            state.head_contents.get(path.as_ref()).cloned()
        })
    }

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        _env: HashMap<String, String>,
        _cx: AsyncApp,
    ) -> BoxFuture<anyhow::Result<()>> {
        self.with_state_async(true, move |state| {
            if let Some(message) = state.simulated_index_write_error_message.clone() {
                return Err(anyhow!("{}", message));
            } else if let Some(content) = content {
                state.index_contents.insert(path, content);
            } else {
                state.index_contents.remove(&path);
            }
            Ok(())
        })
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
        let workdir_path = self.dot_git_path.parent().unwrap();

        // Load gitignores
        let ignores = workdir_path
            .ancestors()
            .filter_map(|dir| {
                let ignore_path = dir.join(".gitignore");
                let content = self.fs.read_file_sync(ignore_path).ok()?;
                let content = String::from_utf8(content).ok()?;
                let mut builder = GitignoreBuilder::new(dir);
                for line in content.lines() {
                    builder.add_line(Some(dir.into()), line).ok()?;
                }
                builder.build().ok()
            })
            .collect::<Vec<_>>();

        // Load working copy files.
        let git_files: HashMap<RepoPath, (String, bool)> = self
            .fs
            .files()
            .iter()
            .filter_map(|path| {
                let repo_path = path.strip_prefix(workdir_path).ok()?;
                let mut is_ignored = false;
                for ignore in &ignores {
                    match ignore.matched_path_or_any_parents(path, false) {
                        ignore::Match::None => {}
                        ignore::Match::Ignore(_) => is_ignored = true,
                        ignore::Match::Whitelist(_) => break,
                    }
                }
                let content = self
                    .fs
                    .read_file_sync(path)
                    .ok()
                    .map(|content| String::from_utf8(content).unwrap())?;
                Some((repo_path.into(), (content, is_ignored)))
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
                    (Some(unmerged), _, _, _) => FileStatus::Unmerged(*unmerged),
                    (_, Some(head), Some(index), Some((fs, _))) => {
                        FileStatus::Tracked(TrackedStatus {
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
                        })
                    }
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
                    (_, None, Some(index), Some((fs, _))) => FileStatus::Tracked(TrackedStatus {
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
                    (_, None, None, Some((_, is_ignored))) => {
                        if *is_ignored {
                            continue;
                        }
                        FileStatus::Untracked
                    }
                    (_, None, None, None) => {
                        unreachable!();
                    }
                };
                if status
                    != FileStatus::Tracked(TrackedStatus {
                        index_status: StatusCode::Unmodified,
                        worktree_status: StatusCode::Unmodified,
                    })
                {
                    entries.push((path.clone(), status));
                }
            }
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Ok(GitStatus {
                entries: entries.into(),
            })
        })
    }

    fn branches(&self) -> BoxFuture<Result<Vec<Branch>>> {
        self.with_state_async(false, move |state| {
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
        })
    }

    fn change_branch(&self, name: String, _cx: AsyncApp) -> BoxFuture<Result<()>> {
        self.with_state_async(true, |state| {
            state.current_branch_name = Some(name);
            Ok(())
        })
    }

    fn create_branch(&self, name: String, _: AsyncApp) -> BoxFuture<Result<()>> {
        self.with_state_async(true, move |state| {
            state.branches.insert(name.to_owned());
            Ok(())
        })
    }

    fn blame(
        &self,
        path: RepoPath,
        _content: Rope,
        _cx: &mut AsyncApp,
    ) -> BoxFuture<Result<git::blame::Blame>> {
        self.with_state_async(false, move |state| {
            state
                .blames
                .get(&path)
                .with_context(|| format!("failed to get blame for {:?}", path.0))
                .cloned()
        })
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

    fn checkpoint(&self, _cx: AsyncApp) -> BoxFuture<Result<GitRepositoryCheckpoint>> {
        unimplemented!()
    }

    fn restore_checkpoint(
        &self,
        _checkpoint: GitRepositoryCheckpoint,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }

    fn compare_checkpoints(
        &self,
        _left: GitRepositoryCheckpoint,
        _right: GitRepositoryCheckpoint,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<bool>> {
        unimplemented!()
    }

    fn delete_checkpoint(
        &self,
        _checkpoint: GitRepositoryCheckpoint,
        _cx: AsyncApp,
    ) -> BoxFuture<Result<()>> {
        unimplemented!()
    }
}
