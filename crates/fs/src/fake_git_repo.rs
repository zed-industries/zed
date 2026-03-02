use crate::{FakeFs, FakeFsEntry, Fs, RemoveOptions, RenameOptions};
use anyhow::{Context as _, Result, bail};
use collections::{HashMap, HashSet};
use futures::future::{self, BoxFuture, join_all};
use git::{
    Oid, RunHook,
    blame::Blame,
    repository::{
        AskPassDelegate, Branch, CommitDataReader, CommitDetails, CommitOptions, FetchOptions,
        GRAPH_CHUNK_SIZE, GitRepository, GitRepositoryCheckpoint, InitialGraphCommitData, LogOrder,
        LogSource, PushOptions, Remote, RepoPath, ResetMode, Worktree,
    },
    status::{
        DiffTreeType, FileStatus, GitStatus, StatusCode, TrackedStatus, TreeDiff, TreeDiffStatus,
        UnmergedStatus,
    },
};
use gpui::{AsyncApp, BackgroundExecutor, SharedString, Task};
use ignore::gitignore::GitignoreBuilder;
use parking_lot::Mutex;
use rope::Rope;
use smol::{channel::Sender, future::FutureExt as _};
use std::{path::PathBuf, sync::Arc};
use text::LineEnding;
use util::{paths::PathStyle, rel_path::RelPath};

#[derive(Clone)]
pub struct FakeGitRepository {
    pub(crate) fs: Arc<FakeFs>,
    pub(crate) checkpoints: Arc<Mutex<HashMap<Oid, FakeFsEntry>>>,
    pub(crate) executor: BackgroundExecutor,
    pub(crate) dot_git_path: PathBuf,
    pub(crate) repository_dir_path: PathBuf,
    pub(crate) common_dir_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FakeGitRepositoryState {
    pub event_emitter: smol::channel::Sender<PathBuf>,
    pub unmerged_paths: HashMap<RepoPath, UnmergedStatus>,
    pub head_contents: HashMap<RepoPath, String>,
    pub index_contents: HashMap<RepoPath, String>,
    // everything in commit contents is in oids
    pub merge_base_contents: HashMap<RepoPath, Oid>,
    pub oids: HashMap<Oid, String>,
    pub blames: HashMap<RepoPath, Blame>,
    pub current_branch_name: Option<String>,
    pub branches: HashSet<String>,
    /// List of remotes, keys are names and values are URLs
    pub remotes: HashMap<String, String>,
    pub simulated_index_write_error_message: Option<String>,
    pub simulated_create_worktree_error: Option<String>,
    pub refs: HashMap<String, String>,
    pub graph_commits: Vec<Arc<InitialGraphCommitData>>,
    pub worktrees: Vec<Worktree>,
}

impl FakeGitRepositoryState {
    pub fn new(event_emitter: smol::channel::Sender<PathBuf>) -> Self {
        FakeGitRepositoryState {
            event_emitter,
            head_contents: Default::default(),
            index_contents: Default::default(),
            unmerged_paths: Default::default(),
            blames: Default::default(),
            current_branch_name: Default::default(),
            branches: Default::default(),
            simulated_index_write_error_message: Default::default(),
            simulated_create_worktree_error: Default::default(),
            refs: HashMap::from_iter([("HEAD".into(), "abc".into())]),
            merge_base_contents: Default::default(),
            oids: Default::default(),
            remotes: HashMap::default(),
            graph_commits: Vec::new(),
            worktrees: Vec::new(),
        }
    }
}

impl FakeGitRepository {
    fn with_state_async<F, T>(&self, write: bool, f: F) -> BoxFuture<'static, Result<T>>
    where
        F: 'static + Send + FnOnce(&mut FakeGitRepositoryState) -> Result<T>,
        T: Send,
    {
        let fs = self.fs.clone();
        let executor = self.executor.clone();
        let dot_git_path = self.dot_git_path.clone();
        async move {
            executor.simulate_random_delay().await;
            fs.with_git_state(&dot_git_path, write, f)?
        }
        .boxed()
    }
}

impl GitRepository for FakeGitRepository {
    fn reload_index(&self) {}

    fn load_index_text(&self, path: RepoPath) -> BoxFuture<'_, Option<String>> {
        let fut = self.with_state_async(false, move |state| {
            state
                .index_contents
                .get(&path)
                .context("not present in index")
                .cloned()
        });
        self.executor.spawn(async move { fut.await.ok() }).boxed()
    }

    fn load_committed_text(&self, path: RepoPath) -> BoxFuture<'_, Option<String>> {
        let fut = self.with_state_async(false, move |state| {
            state
                .head_contents
                .get(&path)
                .context("not present in HEAD")
                .cloned()
        });
        self.executor.spawn(async move { fut.await.ok() }).boxed()
    }

    fn load_blob_content(&self, oid: git::Oid) -> BoxFuture<'_, Result<String>> {
        self.with_state_async(false, move |state| {
            state.oids.get(&oid).cloned().context("oid does not exist")
        })
        .boxed()
    }

    fn load_commit(
        &self,
        _commit: String,
        _cx: AsyncApp,
    ) -> BoxFuture<'_, Result<git::repository::CommitDiff>> {
        unimplemented!()
    }

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        _env: Arc<HashMap<String, String>>,
        _is_executable: bool,
    ) -> BoxFuture<'_, anyhow::Result<()>> {
        self.with_state_async(true, move |state| {
            if let Some(message) = &state.simulated_index_write_error_message {
                anyhow::bail!("{message}");
            } else if let Some(content) = content {
                state.index_contents.insert(path, content);
            } else {
                state.index_contents.remove(&path);
            }
            Ok(())
        })
    }

    fn remote_url(&self, name: &str) -> BoxFuture<'_, Option<String>> {
        let name = name.to_string();
        let fut = self.with_state_async(false, move |state| {
            state
                .remotes
                .get(&name)
                .context("remote not found")
                .cloned()
        });
        async move { fut.await.ok() }.boxed()
    }

    fn diff_tree(&self, _request: DiffTreeType) -> BoxFuture<'_, Result<TreeDiff>> {
        let mut entries = HashMap::default();
        self.with_state_async(false, |state| {
            for (path, content) in &state.head_contents {
                let status = if let Some((oid, original)) = state
                    .merge_base_contents
                    .get(path)
                    .map(|oid| (oid, &state.oids[oid]))
                {
                    if original == content {
                        continue;
                    }
                    TreeDiffStatus::Modified { old: *oid }
                } else {
                    TreeDiffStatus::Added
                };
                entries.insert(path.clone(), status);
            }
            for (path, oid) in &state.merge_base_contents {
                if !entries.contains_key(path) {
                    entries.insert(path.clone(), TreeDiffStatus::Deleted { old: *oid });
                }
            }
            Ok(TreeDiff { entries })
        })
        .boxed()
    }

    fn revparse_batch(&self, revs: Vec<String>) -> BoxFuture<'_, Result<Vec<Option<String>>>> {
        self.with_state_async(false, |state| {
            Ok(revs
                .into_iter()
                .map(|rev| state.refs.get(&rev).cloned())
                .collect())
        })
    }

    fn show(&self, commit: String) -> BoxFuture<'_, Result<CommitDetails>> {
        async {
            Ok(CommitDetails {
                sha: commit.into(),
                message: "initial commit".into(),
                ..Default::default()
            })
        }
        .boxed()
    }

    fn reset(
        &self,
        _commit: String,
        _mode: ResetMode,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        unimplemented!()
    }

    fn checkout_files(
        &self,
        _commit: String,
        _paths: Vec<RepoPath>,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        unimplemented!()
    }

    fn path(&self) -> PathBuf {
        self.repository_dir_path.clone()
    }

    fn main_repository_path(&self) -> PathBuf {
        self.common_dir_path.clone()
    }

    fn merge_message(&self) -> BoxFuture<'_, Option<String>> {
        async move { None }.boxed()
    }

    fn status(&self, path_prefixes: &[RepoPath]) -> Task<Result<GitStatus>> {
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
                // TODO better simulate git status output in the case of submodules and worktrees
                let repo_path = path.strip_prefix(workdir_path).ok()?;
                let mut is_ignored = repo_path.starts_with(".git");
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
                let repo_path = RelPath::new(repo_path, PathStyle::local()).ok()?;
                Some((RepoPath::from_rel_path(&repo_path), (content, is_ignored)))
            })
            .collect();

        let result = self.fs.with_git_state(&self.dot_git_path, false, |state| {
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
            anyhow::Ok(GitStatus {
                entries: entries.into(),
            })
        });
        Task::ready(match result {
            Ok(result) => result,
            Err(e) => Err(e),
        })
    }

    fn stash_entries(&self) -> BoxFuture<'_, Result<git::stash::GitStash>> {
        async { Ok(git::stash::GitStash::default()) }.boxed()
    }

    fn branches(&self) -> BoxFuture<'_, Result<Vec<Branch>>> {
        self.with_state_async(false, move |state| {
            let current_branch = &state.current_branch_name;
            Ok(state
                .branches
                .iter()
                .map(|branch_name| {
                    let ref_name = if branch_name.starts_with("refs/") {
                        branch_name.into()
                    } else {
                        format!("refs/heads/{branch_name}").into()
                    };
                    Branch {
                        is_head: Some(branch_name) == current_branch.as_ref(),
                        ref_name,
                        most_recent_commit: None,
                        upstream: None,
                    }
                })
                .collect())
        })
    }

    fn worktrees(&self) -> BoxFuture<'_, Result<Vec<Worktree>>> {
        self.with_state_async(false, |state| Ok(state.worktrees.clone()))
    }

    fn create_worktree(
        &self,
        name: String,
        directory: PathBuf,
        from_commit: Option<String>,
    ) -> BoxFuture<'_, Result<()>> {
        let fs = self.fs.clone();
        let executor = self.executor.clone();
        let dot_git_path = self.dot_git_path.clone();
        async move {
            let path = directory.join(&name);
            executor.simulate_random_delay().await;
            // Check for simulated error before any side effects
            fs.with_git_state(&dot_git_path, false, |state| {
                if let Some(message) = &state.simulated_create_worktree_error {
                    anyhow::bail!("{message}");
                }
                Ok(())
            })??;
            // Create directory before updating state so state is never
            // inconsistent with the filesystem
            fs.create_dir(&path).await?;
            fs.with_git_state(&dot_git_path, true, {
                let path = path.clone();
                move |state| {
                    if state.branches.contains(&name) {
                        bail!("a branch named '{}' already exists", name);
                    }
                    let ref_name = format!("refs/heads/{name}");
                    let sha = from_commit.unwrap_or_else(|| "fake-sha".to_string());
                    state.refs.insert(ref_name.clone(), sha.clone());
                    state.worktrees.push(Worktree {
                        path,
                        ref_name: ref_name.into(),
                        sha: sha.into(),
                    });
                    state.branches.insert(name);
                    Ok::<(), anyhow::Error>(())
                }
            })??;
            Ok(())
        }
        .boxed()
    }

    fn remove_worktree(&self, path: PathBuf, _force: bool) -> BoxFuture<'_, Result<()>> {
        let fs = self.fs.clone();
        let executor = self.executor.clone();
        let dot_git_path = self.dot_git_path.clone();
        async move {
            executor.simulate_random_delay().await;
            // Validate the worktree exists in state before touching the filesystem
            fs.with_git_state(&dot_git_path, false, {
                let path = path.clone();
                move |state| {
                    if !state.worktrees.iter().any(|w| w.path == path) {
                        bail!("no worktree found at path: {}", path.display());
                    }
                    Ok(())
                }
            })??;
            // Now remove the directory
            fs.remove_dir(
                &path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: false,
                },
            )
            .await?;
            // Update state
            fs.with_git_state(&dot_git_path, true, move |state| {
                state.worktrees.retain(|worktree| worktree.path != path);
                Ok::<(), anyhow::Error>(())
            })??;
            Ok(())
        }
        .boxed()
    }

    fn rename_worktree(&self, old_path: PathBuf, new_path: PathBuf) -> BoxFuture<'_, Result<()>> {
        let fs = self.fs.clone();
        let executor = self.executor.clone();
        let dot_git_path = self.dot_git_path.clone();
        async move {
            executor.simulate_random_delay().await;
            // Validate the worktree exists in state before touching the filesystem
            fs.with_git_state(&dot_git_path, false, {
                let old_path = old_path.clone();
                move |state| {
                    if !state.worktrees.iter().any(|w| w.path == old_path) {
                        bail!("no worktree found at path: {}", old_path.display());
                    }
                    Ok(())
                }
            })??;
            // Now move the directory
            fs.rename(
                &old_path,
                &new_path,
                RenameOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                    create_parents: true,
                },
            )
            .await?;
            // Update state
            fs.with_git_state(&dot_git_path, true, move |state| {
                let worktree = state
                    .worktrees
                    .iter_mut()
                    .find(|worktree| worktree.path == old_path)
                    .expect("worktree was validated above");
                worktree.path = new_path;
                Ok::<(), anyhow::Error>(())
            })??;
            Ok(())
        }
        .boxed()
    }

    fn change_branch(&self, name: String) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, |state| {
            state.current_branch_name = Some(name);
            Ok(())
        })
    }

    fn create_branch(
        &self,
        name: String,
        _base_branch: Option<String>,
    ) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            state.branches.insert(name);
            Ok(())
        })
    }

    fn rename_branch(&self, branch: String, new_name: String) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            if !state.branches.remove(&branch) {
                bail!("no such branch: {branch}");
            }
            state.branches.insert(new_name.clone());
            if state.current_branch_name == Some(branch) {
                state.current_branch_name = Some(new_name);
            }
            Ok(())
        })
    }

    fn delete_branch(&self, name: String) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            if !state.branches.remove(&name) {
                bail!("no such branch: {name}");
            }
            Ok(())
        })
    }

    fn blame(
        &self,
        path: RepoPath,
        _content: Rope,
        _line_ending: LineEnding,
    ) -> BoxFuture<'_, Result<git::blame::Blame>> {
        self.with_state_async(false, move |state| {
            state
                .blames
                .get(&path)
                .with_context(|| format!("failed to get blame for {:?}", path))
                .cloned()
        })
    }

    fn file_history(&self, path: RepoPath) -> BoxFuture<'_, Result<git::repository::FileHistory>> {
        self.file_history_paginated(path, 0, None)
    }

    fn file_history_paginated(
        &self,
        path: RepoPath,
        _skip: usize,
        _limit: Option<usize>,
    ) -> BoxFuture<'_, Result<git::repository::FileHistory>> {
        async move {
            Ok(git::repository::FileHistory {
                entries: Vec::new(),
                path,
            })
        }
        .boxed()
    }

    fn stage_paths(
        &self,
        paths: Vec<RepoPath>,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            let contents = paths
                .into_iter()
                .map(|path| {
                    let abs_path = self
                        .dot_git_path
                        .parent()
                        .unwrap()
                        .join(&path.as_std_path());
                    Box::pin(async move { (path.clone(), self.fs.load(&abs_path).await.ok()) })
                })
                .collect::<Vec<_>>();
            let contents = join_all(contents).await;
            self.with_state_async(true, move |state| {
                for (path, content) in contents {
                    if let Some(content) = content {
                        state.index_contents.insert(path, content);
                    } else {
                        state.index_contents.remove(&path);
                    }
                }
                Ok(())
            })
            .await
        })
    }

    fn unstage_paths(
        &self,
        paths: Vec<RepoPath>,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            for path in paths {
                match state.head_contents.get(&path) {
                    Some(content) => state.index_contents.insert(path, content.clone()),
                    None => state.index_contents.remove(&path),
                };
            }
            Ok(())
        })
    }

    fn stash_paths(
        &self,
        _paths: Vec<RepoPath>,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        unimplemented!()
    }

    fn stash_pop(
        &self,
        _index: Option<usize>,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        unimplemented!()
    }

    fn stash_apply(
        &self,
        _index: Option<usize>,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        unimplemented!()
    }

    fn stash_drop(
        &self,
        _index: Option<usize>,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        unimplemented!()
    }

    fn commit(
        &self,
        _message: gpui::SharedString,
        _name_and_email: Option<(gpui::SharedString, gpui::SharedString)>,
        _options: CommitOptions,
        _askpass: AskPassDelegate,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        async { Ok(()) }.boxed()
    }

    fn run_hook(
        &self,
        _hook: RunHook,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        async { Ok(()) }.boxed()
    }

    fn push(
        &self,
        _branch: String,
        _remote_branch: String,
        _remote: String,
        _options: Option<PushOptions>,
        _askpass: AskPassDelegate,
        _env: Arc<HashMap<String, String>>,
        _cx: AsyncApp,
    ) -> BoxFuture<'_, Result<git::repository::RemoteCommandOutput>> {
        unimplemented!()
    }

    fn pull(
        &self,
        _branch: Option<String>,
        _remote: String,
        _rebase: bool,
        _askpass: AskPassDelegate,
        _env: Arc<HashMap<String, String>>,
        _cx: AsyncApp,
    ) -> BoxFuture<'_, Result<git::repository::RemoteCommandOutput>> {
        unimplemented!()
    }

    fn fetch(
        &self,
        _fetch_options: FetchOptions,
        _askpass: AskPassDelegate,
        _env: Arc<HashMap<String, String>>,
        _cx: AsyncApp,
    ) -> BoxFuture<'_, Result<git::repository::RemoteCommandOutput>> {
        unimplemented!()
    }

    fn get_all_remotes(&self) -> BoxFuture<'_, Result<Vec<Remote>>> {
        self.with_state_async(false, move |state| {
            let remotes = state
                .remotes
                .keys()
                .map(|r| Remote {
                    name: r.clone().into(),
                })
                .collect::<Vec<_>>();
            Ok(remotes)
        })
    }

    fn get_push_remote(&self, _branch: String) -> BoxFuture<'_, Result<Option<Remote>>> {
        unimplemented!()
    }

    fn get_branch_remote(&self, _branch: String) -> BoxFuture<'_, Result<Option<Remote>>> {
        unimplemented!()
    }

    fn check_for_pushed_commit(&self) -> BoxFuture<'_, Result<Vec<gpui::SharedString>>> {
        future::ready(Ok(Vec::new())).boxed()
    }

    fn diff(&self, _diff: git::repository::DiffType) -> BoxFuture<'_, Result<String>> {
        unimplemented!()
    }

    fn diff_stat(
        &self,
        diff_type: git::repository::DiffType,
    ) -> BoxFuture<'_, Result<HashMap<RepoPath, git::status::DiffStat>>> {
        fn count_lines(s: &str) -> u32 {
            if s.is_empty() {
                0
            } else {
                s.lines().count() as u32
            }
        }

        match diff_type {
            git::repository::DiffType::HeadToIndex => self
                .with_state_async(false, |state| {
                    let mut result = HashMap::default();
                    let all_paths: HashSet<&RepoPath> = state
                        .head_contents
                        .keys()
                        .chain(state.index_contents.keys())
                        .collect();
                    for path in all_paths {
                        let head = state.head_contents.get(path);
                        let index = state.index_contents.get(path);
                        match (head, index) {
                            (Some(old), Some(new)) if old != new => {
                                result.insert(
                                    path.clone(),
                                    git::status::DiffStat {
                                        added: count_lines(new),
                                        deleted: count_lines(old),
                                    },
                                );
                            }
                            (Some(old), None) => {
                                result.insert(
                                    path.clone(),
                                    git::status::DiffStat {
                                        added: 0,
                                        deleted: count_lines(old),
                                    },
                                );
                            }
                            (None, Some(new)) => {
                                result.insert(
                                    path.clone(),
                                    git::status::DiffStat {
                                        added: count_lines(new),
                                        deleted: 0,
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    Ok(result)
                })
                .boxed(),
            git::repository::DiffType::HeadToWorktree => {
                let workdir_path = self.dot_git_path.parent().unwrap().to_path_buf();
                let worktree_files: HashMap<RepoPath, String> = self
                    .fs
                    .files()
                    .iter()
                    .filter_map(|path| {
                        let repo_path = path.strip_prefix(&workdir_path).ok()?;
                        if repo_path.starts_with(".git") {
                            return None;
                        }
                        let content = self
                            .fs
                            .read_file_sync(path)
                            .ok()
                            .and_then(|bytes| String::from_utf8(bytes).ok())?;
                        let repo_path = RelPath::new(repo_path, PathStyle::local()).ok()?;
                        Some((RepoPath::from_rel_path(&repo_path), content))
                    })
                    .collect();

                self.with_state_async(false, move |state| {
                    let mut result = HashMap::default();
                    let all_paths: HashSet<&RepoPath> = state
                        .head_contents
                        .keys()
                        .chain(worktree_files.keys())
                        .collect();
                    for path in all_paths {
                        let head = state.head_contents.get(path);
                        let worktree = worktree_files.get(path);
                        match (head, worktree) {
                            (Some(old), Some(new)) if old != new => {
                                result.insert(
                                    path.clone(),
                                    git::status::DiffStat {
                                        added: count_lines(new),
                                        deleted: count_lines(old),
                                    },
                                );
                            }
                            (Some(old), None) => {
                                result.insert(
                                    path.clone(),
                                    git::status::DiffStat {
                                        added: 0,
                                        deleted: count_lines(old),
                                    },
                                );
                            }
                            (None, Some(new)) => {
                                result.insert(
                                    path.clone(),
                                    git::status::DiffStat {
                                        added: count_lines(new),
                                        deleted: 0,
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    Ok(result)
                })
                .boxed()
            }
            git::repository::DiffType::MergeBase { .. } => {
                future::ready(Ok(HashMap::default())).boxed()
            }
        }
    }

    fn checkpoint(&self) -> BoxFuture<'static, Result<GitRepositoryCheckpoint>> {
        let executor = self.executor.clone();
        let fs = self.fs.clone();
        let checkpoints = self.checkpoints.clone();
        let repository_dir_path = self.repository_dir_path.parent().unwrap().to_path_buf();
        async move {
            executor.simulate_random_delay().await;
            let oid = git::Oid::random(&mut *executor.rng().lock());
            let entry = fs.entry(&repository_dir_path)?;
            checkpoints.lock().insert(oid, entry);
            Ok(GitRepositoryCheckpoint { commit_sha: oid })
        }
        .boxed()
    }

    fn restore_checkpoint(&self, checkpoint: GitRepositoryCheckpoint) -> BoxFuture<'_, Result<()>> {
        let executor = self.executor.clone();
        let fs = self.fs.clone();
        let checkpoints = self.checkpoints.clone();
        let repository_dir_path = self.repository_dir_path.parent().unwrap().to_path_buf();
        async move {
            executor.simulate_random_delay().await;
            let checkpoints = checkpoints.lock();
            let entry = checkpoints
                .get(&checkpoint.commit_sha)
                .context(format!("invalid checkpoint: {}", checkpoint.commit_sha))?;
            fs.insert_entry(&repository_dir_path, entry.clone())?;
            Ok(())
        }
        .boxed()
    }

    fn compare_checkpoints(
        &self,
        left: GitRepositoryCheckpoint,
        right: GitRepositoryCheckpoint,
    ) -> BoxFuture<'_, Result<bool>> {
        let executor = self.executor.clone();
        let checkpoints = self.checkpoints.clone();
        async move {
            executor.simulate_random_delay().await;
            let checkpoints = checkpoints.lock();
            let left = checkpoints
                .get(&left.commit_sha)
                .context(format!("invalid left checkpoint: {}", left.commit_sha))?;
            let right = checkpoints
                .get(&right.commit_sha)
                .context(format!("invalid right checkpoint: {}", right.commit_sha))?;

            Ok(left == right)
        }
        .boxed()
    }

    fn diff_checkpoints(
        &self,
        _base_checkpoint: GitRepositoryCheckpoint,
        _target_checkpoint: GitRepositoryCheckpoint,
    ) -> BoxFuture<'_, Result<String>> {
        unimplemented!()
    }

    fn default_branch(
        &self,
        include_remote_name: bool,
    ) -> BoxFuture<'_, Result<Option<SharedString>>> {
        async move {
            Ok(Some(if include_remote_name {
                "origin/main".into()
            } else {
                "main".into()
            }))
        }
        .boxed()
    }

    fn create_remote(&self, name: String, url: String) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            state.remotes.insert(name, url);
            Ok(())
        })
    }

    fn remove_remote(&self, name: String) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            state.remotes.remove(&name);
            Ok(())
        })
    }

    fn initial_graph_data(
        &self,
        _log_source: LogSource,
        _log_order: LogOrder,
        request_tx: Sender<Vec<Arc<InitialGraphCommitData>>>,
    ) -> BoxFuture<'_, Result<()>> {
        let fs = self.fs.clone();
        let dot_git_path = self.dot_git_path.clone();
        async move {
            let graph_commits =
                fs.with_git_state(&dot_git_path, false, |state| state.graph_commits.clone())?;

            for chunk in graph_commits.chunks(GRAPH_CHUNK_SIZE) {
                request_tx.send(chunk.to_vec()).await.ok();
            }
            Ok(())
        }
        .boxed()
    }

    fn commit_data_reader(&self) -> Result<CommitDataReader> {
        anyhow::bail!("commit_data_reader not supported for FakeGitRepository")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FakeFs, Fs};
    use gpui::TestAppContext;
    use serde_json::json;
    use std::path::Path;

    #[gpui::test]
    async fn test_fake_worktree_lifecycle(cx: &mut TestAppContext) {
        let worktree_dir_settings = &["../worktrees", ".git/zed-worktrees", "my-worktrees/"];

        for worktree_dir_setting in worktree_dir_settings {
            let fs = FakeFs::new(cx.executor());
            fs.insert_tree("/project", json!({".git": {}, "file.txt": "content"}))
                .await;
            let repo = fs
                .open_repo(Path::new("/project/.git"), None)
                .expect("should open fake repo");

            // Initially no worktrees
            let worktrees = repo.worktrees().await.unwrap();
            assert!(worktrees.is_empty());

            let expected_dir = git::repository::resolve_worktree_directory(
                Path::new("/project"),
                worktree_dir_setting,
            );

            // Create a worktree
            repo.create_worktree(
                "feature-branch".to_string(),
                expected_dir.clone(),
                Some("abc123".to_string()),
            )
            .await
            .unwrap();

            // List worktrees — should have one
            let worktrees = repo.worktrees().await.unwrap();
            assert_eq!(worktrees.len(), 1);
            assert_eq!(
                worktrees[0].path,
                expected_dir.join("feature-branch"),
                "failed for worktree_directory setting: {worktree_dir_setting:?}"
            );
            assert_eq!(worktrees[0].ref_name.as_ref(), "refs/heads/feature-branch");
            assert_eq!(worktrees[0].sha.as_ref(), "abc123");

            // Directory should exist in FakeFs after create
            assert!(
                fs.is_dir(&expected_dir.join("feature-branch")).await,
                "worktree directory should be created in FakeFs for setting {worktree_dir_setting:?}"
            );

            // Create a second worktree (without explicit commit)
            repo.create_worktree("bugfix-branch".to_string(), expected_dir.clone(), None)
                .await
                .unwrap();

            let worktrees = repo.worktrees().await.unwrap();
            assert_eq!(worktrees.len(), 2);
            assert!(
                fs.is_dir(&expected_dir.join("bugfix-branch")).await,
                "second worktree directory should be created in FakeFs for setting {worktree_dir_setting:?}"
            );

            // Rename the first worktree
            repo.rename_worktree(
                expected_dir.join("feature-branch"),
                expected_dir.join("renamed-branch"),
            )
            .await
            .unwrap();

            let worktrees = repo.worktrees().await.unwrap();
            assert_eq!(worktrees.len(), 2);
            assert!(
                worktrees
                    .iter()
                    .any(|w| w.path == expected_dir.join("renamed-branch")),
                "renamed worktree should exist at new path for setting {worktree_dir_setting:?}"
            );
            assert!(
                worktrees
                    .iter()
                    .all(|w| w.path != expected_dir.join("feature-branch")),
                "old path should no longer exist for setting {worktree_dir_setting:?}"
            );

            // Directory should be moved in FakeFs after rename
            assert!(
                !fs.is_dir(&expected_dir.join("feature-branch")).await,
                "old worktree directory should not exist after rename for setting {worktree_dir_setting:?}"
            );
            assert!(
                fs.is_dir(&expected_dir.join("renamed-branch")).await,
                "new worktree directory should exist after rename for setting {worktree_dir_setting:?}"
            );

            // Rename a nonexistent worktree should fail
            let result = repo
                .rename_worktree(PathBuf::from("/nonexistent"), PathBuf::from("/somewhere"))
                .await;
            assert!(result.is_err());

            // Remove a worktree
            repo.remove_worktree(expected_dir.join("renamed-branch"), false)
                .await
                .unwrap();

            let worktrees = repo.worktrees().await.unwrap();
            assert_eq!(worktrees.len(), 1);
            assert_eq!(worktrees[0].path, expected_dir.join("bugfix-branch"));

            // Directory should be removed from FakeFs after remove
            assert!(
                !fs.is_dir(&expected_dir.join("renamed-branch")).await,
                "worktree directory should be removed from FakeFs for setting {worktree_dir_setting:?}"
            );

            // Remove a nonexistent worktree should fail
            let result = repo
                .remove_worktree(PathBuf::from("/nonexistent"), false)
                .await;
            assert!(result.is_err());

            // Remove the last worktree
            repo.remove_worktree(expected_dir.join("bugfix-branch"), false)
                .await
                .unwrap();

            let worktrees = repo.worktrees().await.unwrap();
            assert!(worktrees.is_empty());
            assert!(
                !fs.is_dir(&expected_dir.join("bugfix-branch")).await,
                "last worktree directory should be removed from FakeFs for setting {worktree_dir_setting:?}"
            );
        }
    }
}
