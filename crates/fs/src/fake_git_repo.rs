use std::path::Path;

use crate::{FakeFs, FakeFsEntry, Fs, RemoveOptions, RenameOptions};
use anyhow::{Context as _, Result, bail};
use collections::{HashMap, HashSet};
use futures::future::{self, BoxFuture, join_all};
use git::repository::GitCommitTemplate;
use git::{
    Oid, RunHook,
    blame::Blame,
    repository::{
        AskPassDelegate, Branch, CommitDataReader, CommitDetails, CommitOptions,
        CreateWorktreeTarget, FetchOptions, GRAPH_CHUNK_SIZE, GitRepository,
        GitRepositoryCheckpoint, InitialGraphCommitData, LogOrder, LogSource, PushOptions, RefEdit,
        Remote, RepoPath, ResetMode, SearchCommitArgs, Worktree,
    },
    stash::GitStash,
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
use std::{path::PathBuf, sync::Arc, sync::atomic::AtomicBool};
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
    pub(crate) is_trusted: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
pub struct FakeCommitSnapshot {
    pub head_contents: HashMap<RepoPath, String>,
    pub index_contents: HashMap<RepoPath, String>,
    pub sha: String,
}

#[derive(Debug, Clone)]
pub struct FakeGitRepositoryState {
    pub commit_history: Vec<FakeCommitSnapshot>,
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
    pub simulated_graph_error: Option<String>,
    pub refs: HashMap<String, String>,
    pub graph_commits: Vec<Arc<InitialGraphCommitData>>,
    pub stash_entries: GitStash,
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
            simulated_graph_error: None,
            refs: HashMap::from_iter([("HEAD".into(), "abc".into())]),
            merge_base_contents: Default::default(),
            oids: Default::default(),
            remotes: HashMap::default(),
            graph_commits: Vec::new(),
            commit_history: Vec::new(),
            stash_entries: Default::default(),
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

    fn edit_ref(&self, edit: RefEdit) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            match edit {
                RefEdit::Update { ref_name, commit } => {
                    state.refs.insert(ref_name, commit);
                }
                RefEdit::Delete { ref_name } => {
                    state.refs.remove(&ref_name);
                }
            }
            Ok(())
        })
    }

    /// Scans `.git/worktrees/*/gitdir` to find the admin entry directory for a
    /// worktree at the given checkout path. Used when the working tree directory
    /// has already been deleted and we can't read its `.git` pointer file.
    async fn find_worktree_entry_dir_by_path(&self, path: &Path) -> Option<PathBuf> {
        use futures::StreamExt;

        let worktrees_dir = self.common_dir_path.join("worktrees");
        let mut entries = self.fs.read_dir(&worktrees_dir).await.ok()?;
        while let Some(Ok(entry_path)) = entries.next().await {
            if let Ok(gitdir_content) = self.fs.load(&entry_path.join("gitdir")).await {
                let worktree_path = PathBuf::from(gitdir_content.trim())
                    .parent()
                    .map(PathBuf::from)
                    .unwrap_or_default();
                if worktree_path == path {
                    return Some(entry_path);
                }
            }
        }
        None
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

    fn load_commit_template(&self) -> BoxFuture<'_, Result<Option<GitCommitTemplate>>> {
        async { Ok(None) }.boxed()
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
        commit: String,
        mode: ResetMode,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            let pop_count = if commit == "HEAD~" || commit == "HEAD^" {
                1
            } else if let Some(suffix) = commit.strip_prefix("HEAD~") {
                suffix
                    .parse::<usize>()
                    .with_context(|| format!("Invalid HEAD~ offset: {commit}"))?
            } else {
                match state
                    .commit_history
                    .iter()
                    .rposition(|entry| entry.sha == commit)
                {
                    Some(index) => state.commit_history.len() - index,
                    None => anyhow::bail!("Unknown commit ref: {commit}"),
                }
            };

            if pop_count == 0 || pop_count > state.commit_history.len() {
                anyhow::bail!(
                    "Cannot reset {pop_count} commit(s): only {} in history",
                    state.commit_history.len()
                );
            }

            let target_index = state.commit_history.len() - pop_count;
            let snapshot = state.commit_history[target_index].clone();
            state.commit_history.truncate(target_index);

            match mode {
                ResetMode::Soft => {
                    state.head_contents = snapshot.head_contents;
                }
                ResetMode::Mixed => {
                    state.head_contents = snapshot.head_contents;
                    state.index_contents = state.head_contents.clone();
                }
            }

            state.refs.insert("HEAD".into(), snapshot.sha);
            Ok(())
        })
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
        self.with_state_async(false, |state| Ok(state.stash_entries.clone()))
    }

    fn branches(&self) -> BoxFuture<'_, Result<Vec<Branch>>> {
        self.with_state_async(false, move |state| {
            let current_branch = &state.current_branch_name;
            let mut branches = state
                .branches
                .iter()
                .map(|branch_name| {
                    let ref_name = if branch_name.starts_with("refs/") {
                        branch_name.into()
                    } else if branch_name.contains('/') {
                        format!("refs/remotes/{branch_name}").into()
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
                .collect::<Vec<_>>();
            // compute snapshot expects these to be sorted by ref_name
            // because that's what git itself does
            branches.sort_by(|a, b| a.ref_name.cmp(&b.ref_name));
            Ok(branches)
        })
    }

    fn worktrees(&self) -> BoxFuture<'_, Result<Vec<Worktree>>> {
        let fs = self.fs.clone();
        let common_dir_path = self.common_dir_path.clone();
        let executor = self.executor.clone();

        async move {
            executor.simulate_random_delay().await;

            let (main_worktree, refs) = fs.with_git_state(&common_dir_path, false, |state| {
                let work_dir = common_dir_path
                    .parent()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| common_dir_path.clone());
                let head_sha = state
                    .refs
                    .get("HEAD")
                    .cloned()
                    .unwrap_or_else(|| "0000000".to_string());
                let branch_ref = state
                    .current_branch_name
                    .as_ref()
                    .map(|name| format!("refs/heads/{name}"))
                    .unwrap_or_else(|| "refs/heads/main".to_string());
                let main_wt = Worktree {
                    path: work_dir,
                    ref_name: Some(branch_ref.into()),
                    sha: head_sha.into(),
                    is_main: true,
                    is_bare: false,
                };
                (main_wt, state.refs.clone())
            })?;

            let mut all = vec![main_worktree];

            let worktrees_dir = common_dir_path.join("worktrees");
            if let Ok(mut entries) = fs.read_dir(&worktrees_dir).await {
                use futures::StreamExt;
                while let Some(Ok(entry_path)) = entries.next().await {
                    let head_content = match fs.load(&entry_path.join("HEAD")).await {
                        Ok(content) => content,
                        Err(_) => continue,
                    };
                    let gitdir_content = match fs.load(&entry_path.join("gitdir")).await {
                        Ok(content) => content,
                        Err(_) => continue,
                    };

                    let ref_name = head_content
                        .strip_prefix("ref: ")
                        .map(|s| s.trim().to_string());
                    let sha = ref_name
                        .as_ref()
                        .and_then(|r| refs.get(r))
                        .cloned()
                        .unwrap_or_else(|| head_content.trim().to_string());

                    let worktree_path = PathBuf::from(gitdir_content.trim())
                        .parent()
                        .map(PathBuf::from)
                        .unwrap_or_default();

                    all.push(Worktree {
                        path: worktree_path,
                        ref_name: ref_name.map(Into::into),
                        sha: sha.into(),
                        is_main: false,
                        is_bare: false,
                    });
                }
            }

            Ok(all)
        }
        .boxed()
    }

    fn create_worktree(
        &self,
        target: CreateWorktreeTarget,
        path: PathBuf,
    ) -> BoxFuture<'_, Result<()>> {
        let fs = self.fs.clone();
        let executor = self.executor.clone();
        let dot_git_path = self.dot_git_path.clone();
        let common_dir_path = self.common_dir_path.clone();
        async move {
            executor.simulate_random_delay().await;

            let branch_name = target.branch_name().map(ToOwned::to_owned);
            let create_branch_ref = matches!(target, CreateWorktreeTarget::NewBranch { .. });

            // Check for simulated error and validate branch state before any side effects.
            fs.with_git_state(&dot_git_path, false, {
                let branch_name = branch_name.clone();
                move |state| {
                    if let Some(message) = &state.simulated_create_worktree_error {
                        anyhow::bail!("{message}");
                    }

                    match (create_branch_ref, branch_name.as_ref()) {
                        (true, Some(branch_name)) => {
                            if state.branches.contains(branch_name) {
                                bail!("a branch named '{}' already exists", branch_name);
                            }
                        }
                        (false, Some(branch_name)) => {
                            if !state.branches.contains(branch_name) {
                                bail!("no branch named '{}' exists", branch_name);
                            }
                        }
                        (false, None) => {}
                        (true, None) => bail!("branch name is required to create a branch"),
                    }

                    Ok(())
                }
            })??;

            let (branch_name, sha, create_branch_ref) = match target {
                CreateWorktreeTarget::ExistingBranch { branch_name } => {
                    let ref_name = format!("refs/heads/{branch_name}");
                    let sha = fs.with_git_state(&dot_git_path, false, {
                        move |state| {
                            Ok::<_, anyhow::Error>(
                                state
                                    .refs
                                    .get(&ref_name)
                                    .cloned()
                                    .unwrap_or_else(|| "fake-sha".to_string()),
                            )
                        }
                    })??;
                    (Some(branch_name), sha, false)
                }
                CreateWorktreeTarget::NewBranch {
                    branch_name,
                    base_sha: start_point,
                } => (
                    Some(branch_name),
                    start_point.unwrap_or_else(|| "fake-sha".to_string()),
                    true,
                ),
                CreateWorktreeTarget::Detached {
                    base_sha: start_point,
                } => (
                    None,
                    start_point.unwrap_or_else(|| "fake-sha".to_string()),
                    false,
                ),
            };

            // Create the worktree checkout directory.
            fs.create_dir(&path).await?;

            // Create .git/worktrees/<name>/ directory with HEAD, commondir, gitdir.
            let worktree_entry_name = branch_name.as_deref().unwrap_or_else(|| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("detached")
            });
            let worktrees_entry_dir = common_dir_path.join("worktrees").join(worktree_entry_name);
            fs.create_dir(&worktrees_entry_dir).await?;

            let head_content = if let Some(ref branch_name) = branch_name {
                let ref_name = format!("refs/heads/{branch_name}");
                format!("ref: {ref_name}")
            } else {
                sha.clone()
            };
            fs.write_file_internal(
                worktrees_entry_dir.join("HEAD"),
                head_content.into_bytes(),
                false,
            )?;
            fs.write_file_internal(
                worktrees_entry_dir.join("commondir"),
                common_dir_path.to_string_lossy().into_owned().into_bytes(),
                false,
            )?;
            let worktree_dot_git = path.join(".git");
            fs.write_file_internal(
                worktrees_entry_dir.join("gitdir"),
                worktree_dot_git.to_string_lossy().into_owned().into_bytes(),
                false,
            )?;

            // Create .git file in the worktree checkout.
            fs.write_file_internal(
                &worktree_dot_git,
                format!("gitdir: {}", worktrees_entry_dir.display()).into_bytes(),
                false,
            )?;

            // Update git state for newly created branches.
            if create_branch_ref {
                fs.with_git_state(&dot_git_path, true, {
                    let branch_name = branch_name.clone();
                    let sha = sha.clone();
                    move |state| {
                        if let Some(branch_name) = branch_name {
                            let ref_name = format!("refs/heads/{branch_name}");
                            state.refs.insert(ref_name, sha);
                            state.branches.insert(branch_name);
                        }
                        Ok::<(), anyhow::Error>(())
                    }
                })??;
            }

            Ok(())
        }
        .boxed()
    }

    fn remove_worktree(&self, path: PathBuf, _force: bool) -> BoxFuture<'_, Result<()>> {
        let fs = self.fs.clone();
        let executor = self.executor.clone();
        let common_dir_path = self.common_dir_path.clone();
        async move {
            executor.simulate_random_delay().await;

            // Try to read the worktree's .git file to find its entry
            // directory. If the working tree is already gone (e.g. the
            // caller deleted it before asking git to clean up), fall back
            // to scanning `.git/worktrees/*/gitdir` for a matching path,
            // mirroring real git's behavior with `--force`.
            let dot_git_file = path.join(".git");
            let worktree_entry_dir = if let Ok(content) = fs.load(&dot_git_file).await {
                let gitdir = content
                    .strip_prefix("gitdir:")
                    .context("invalid .git file in worktree")?
                    .trim();
                PathBuf::from(gitdir)
            } else {
                self.find_worktree_entry_dir_by_path(&path)
                    .await
                    .with_context(|| format!("no worktree found at path: {}", path.display()))?
            };

            // Remove the worktree checkout directory if it still exists.
            fs.remove_dir(
                &path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

            // Remove the .git/worktrees/<name>/ directory.
            fs.remove_dir(
                &worktree_entry_dir,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: false,
                },
            )
            .await?;

            // Emit a git event on the main .git directory so the scanner
            // notices the change.
            fs.with_git_state(&common_dir_path, true, |_| {})?;

            Ok(())
        }
        .boxed()
    }

    fn rename_worktree(&self, old_path: PathBuf, new_path: PathBuf) -> BoxFuture<'_, Result<()>> {
        let fs = self.fs.clone();
        let executor = self.executor.clone();
        let common_dir_path = self.common_dir_path.clone();
        async move {
            executor.simulate_random_delay().await;

            // Read the worktree's .git file to find its entry directory.
            let dot_git_file = old_path.join(".git");
            let content = fs
                .load(&dot_git_file)
                .await
                .with_context(|| format!("no worktree found at path: {}", old_path.display()))?;
            let gitdir = content
                .strip_prefix("gitdir:")
                .context("invalid .git file in worktree")?
                .trim();
            let worktree_entry_dir = PathBuf::from(gitdir);

            // Move the worktree checkout directory.
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

            // Update the gitdir file in .git/worktrees/<name>/ to point to the
            // new location.
            let new_dot_git = new_path.join(".git");
            fs.write_file_internal(
                worktree_entry_dir.join("gitdir"),
                new_dot_git.to_string_lossy().into_owned().into_bytes(),
                false,
            )?;

            // Update the .git file in the moved worktree checkout.
            fs.write_file_internal(
                &new_dot_git,
                format!("gitdir: {}", worktree_entry_dir.display()).into_bytes(),
                false,
            )?;

            // Emit a git event on the main .git directory so the scanner
            // notices the change.
            fs.with_git_state(&common_dir_path, true, |_| {})?;

            Ok(())
        }
        .boxed()
    }

    fn checkout_branch_in_worktree(
        &self,
        _branch_name: String,
        _worktree_path: PathBuf,
        _create: bool,
    ) -> BoxFuture<'_, Result<()>> {
        async { Ok(()) }.boxed()
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
            if let Some((remote, _)) = name.split_once('/')
                && !state.remotes.contains_key(remote)
            {
                state.remotes.insert(remote.to_owned(), "".to_owned());
            }
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

    fn delete_branch(&self, _is_remote: bool, name: String) -> BoxFuture<'_, Result<()>> {
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
        options: CommitOptions,
        _askpass: AskPassDelegate,
        _env: Arc<HashMap<String, String>>,
    ) -> BoxFuture<'_, Result<()>> {
        self.with_state_async(true, move |state| {
            if !options.allow_empty && !options.amend && state.index_contents == state.head_contents
            {
                anyhow::bail!("nothing to commit (use allow_empty to create an empty commit)");
            }

            let old_sha = state.refs.get("HEAD").cloned().unwrap_or_default();
            state.commit_history.push(FakeCommitSnapshot {
                head_contents: state.head_contents.clone(),
                index_contents: state.index_contents.clone(),
                sha: old_sha,
            });

            state.head_contents = state.index_contents.clone();

            let new_sha = format!("fake-commit-{}", state.commit_history.len());
            state.refs.insert("HEAD".into(), new_sha);

            Ok(())
        })
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
        future::ready(Ok(String::new())).boxed()
    }

    fn diff_stat(
        &self,
        path_prefixes: &[RepoPath],
    ) -> BoxFuture<'_, Result<git::status::GitDiffStat>> {
        fn count_lines(s: &str) -> u32 {
            if s.is_empty() {
                0
            } else {
                s.lines().count() as u32
            }
        }

        fn matches_prefixes(path: &RepoPath, prefixes: &[RepoPath]) -> bool {
            if prefixes.is_empty() {
                return true;
            }
            prefixes.iter().any(|prefix| {
                let prefix_str = prefix.as_unix_str();
                if prefix_str == "." {
                    return true;
                }
                path == prefix || path.starts_with(&prefix)
            })
        }

        let path_prefixes = path_prefixes.to_vec();

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
            let mut entries = Vec::new();
            let all_paths: HashSet<&RepoPath> = state
                .head_contents
                .keys()
                .chain(
                    worktree_files
                        .keys()
                        .filter(|p| state.index_contents.contains_key(*p)),
                )
                .collect();
            for path in all_paths {
                if !matches_prefixes(path, &path_prefixes) {
                    continue;
                }
                let head = state.head_contents.get(path);
                let worktree = worktree_files.get(path);
                match (head, worktree) {
                    (Some(old), Some(new)) if old != new => {
                        entries.push((
                            path.clone(),
                            git::status::DiffStat {
                                added: count_lines(new),
                                deleted: count_lines(old),
                            },
                        ));
                    }
                    (Some(old), None) => {
                        entries.push((
                            path.clone(),
                            git::status::DiffStat {
                                added: 0,
                                deleted: count_lines(old),
                            },
                        ));
                    }
                    (None, Some(new)) => {
                        entries.push((
                            path.clone(),
                            git::status::DiffStat {
                                added: count_lines(new),
                                deleted: 0,
                            },
                        ));
                    }
                    _ => {}
                }
            }
            entries.sort_by(|(a, _), (b, _)| a.cmp(b));
            Ok(git::status::GitDiffStat {
                entries: entries.into(),
            })
        })
        .boxed()
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

    fn create_archive_checkpoint(&self) -> BoxFuture<'_, Result<(String, String)>> {
        let executor = self.executor.clone();
        let fs = self.fs.clone();
        let checkpoints = self.checkpoints.clone();
        let repository_dir_path = self.repository_dir_path.parent().unwrap().to_path_buf();
        async move {
            executor.simulate_random_delay().await;
            let staged_oid = git::Oid::random(&mut *executor.rng().lock());
            let unstaged_oid = git::Oid::random(&mut *executor.rng().lock());
            let entry = fs.entry(&repository_dir_path)?;
            checkpoints.lock().insert(staged_oid, entry.clone());
            checkpoints.lock().insert(unstaged_oid, entry);
            Ok((staged_oid.to_string(), unstaged_oid.to_string()))
        }
        .boxed()
    }

    fn restore_archive_checkpoint(
        &self,
        // The fake filesystem doesn't model a separate index, so only the
        // unstaged (full working directory) snapshot is restored.
        _staged_sha: String,
        unstaged_sha: String,
    ) -> BoxFuture<'_, Result<()>> {
        match unstaged_sha.parse() {
            Ok(commit_sha) => self.restore_checkpoint(GitRepositoryCheckpoint { commit_sha }),
            Err(error) => async move {
                Err(anyhow::anyhow!(error).context("failed to parse unstaged SHA as Oid"))
            }
            .boxed(),
        }
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
        base_checkpoint: GitRepositoryCheckpoint,
        target_checkpoint: GitRepositoryCheckpoint,
    ) -> BoxFuture<'_, Result<String>> {
        let executor = self.executor.clone();
        let checkpoints = self.checkpoints.clone();
        async move {
            executor.simulate_random_delay().await;
            let checkpoints = checkpoints.lock();
            let base = checkpoints
                .get(&base_checkpoint.commit_sha)
                .context(format!(
                    "invalid base checkpoint: {}",
                    base_checkpoint.commit_sha
                ))?;
            let target = checkpoints
                .get(&target_checkpoint.commit_sha)
                .context(format!(
                    "invalid target checkpoint: {}",
                    target_checkpoint.commit_sha
                ))?;

            fn collect_files(
                entry: &FakeFsEntry,
                prefix: String,
                out: &mut std::collections::BTreeMap<String, String>,
            ) {
                match entry {
                    FakeFsEntry::File { content, .. } => {
                        out.insert(prefix, String::from_utf8_lossy(content).into_owned());
                    }
                    FakeFsEntry::Dir { entries, .. } => {
                        for (name, child) in entries {
                            let path = if prefix.is_empty() {
                                name.clone()
                            } else {
                                format!("{prefix}/{name}")
                            };
                            collect_files(child, path, out);
                        }
                    }
                    FakeFsEntry::Symlink { .. } => {}
                }
            }

            let mut base_files = std::collections::BTreeMap::new();
            let mut target_files = std::collections::BTreeMap::new();
            collect_files(base, String::new(), &mut base_files);
            collect_files(target, String::new(), &mut target_files);

            let all_paths: std::collections::BTreeSet<&String> =
                base_files.keys().chain(target_files.keys()).collect();

            let mut diff = String::new();
            for path in all_paths {
                match (base_files.get(path), target_files.get(path)) {
                    (Some(base_content), Some(target_content))
                        if base_content != target_content =>
                    {
                        diff.push_str(&format!("diff --git a/{path} b/{path}\n"));
                        diff.push_str(&format!("--- a/{path}\n"));
                        diff.push_str(&format!("+++ b/{path}\n"));
                        for line in base_content.lines() {
                            diff.push_str(&format!("-{line}\n"));
                        }
                        for line in target_content.lines() {
                            diff.push_str(&format!("+{line}\n"));
                        }
                    }
                    (Some(_), None) => {
                        diff.push_str(&format!("diff --git a/{path} /dev/null\n"));
                        diff.push_str("deleted file\n");
                    }
                    (None, Some(_)) => {
                        diff.push_str(&format!("diff --git /dev/null b/{path}\n"));
                        diff.push_str("new file\n");
                    }
                    _ => {}
                }
            }
            Ok(diff)
        }
        .boxed()
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
            state.branches.retain(|branch| {
                branch
                    .split_once('/')
                    .is_none_or(|(remote, _)| remote != name)
            });
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
            let (graph_commits, simulated_error) =
                fs.with_git_state(&dot_git_path, false, |state| {
                    (
                        state.graph_commits.clone(),
                        state.simulated_graph_error.clone(),
                    )
                })?;

            if let Some(error) = simulated_error {
                anyhow::bail!("{}", error);
            }

            for chunk in graph_commits.chunks(GRAPH_CHUNK_SIZE) {
                request_tx.send(chunk.to_vec()).await.ok();
            }
            Ok(())
        }
        .boxed()
    }

    fn search_commits(
        &self,
        _log_source: LogSource,
        _search_args: SearchCommitArgs,
        _request_tx: Sender<Oid>,
    ) -> BoxFuture<'_, Result<()>> {
        async { bail!("search_commits not supported for FakeGitRepository") }.boxed()
    }

    fn commit_data_reader(&self) -> Result<CommitDataReader> {
        anyhow::bail!("commit_data_reader not supported for FakeGitRepository")
    }

    fn update_ref(&self, ref_name: String, commit: String) -> BoxFuture<'_, Result<()>> {
        self.edit_ref(RefEdit::Update { ref_name, commit })
    }

    fn delete_ref(&self, ref_name: String) -> BoxFuture<'_, Result<()>> {
        self.edit_ref(RefEdit::Delete { ref_name })
    }

    fn repair_worktrees(&self) -> BoxFuture<'_, Result<()>> {
        async { Ok(()) }.boxed()
    }

    fn set_trusted(&self, trusted: bool) {
        self.is_trusted
            .store(trusted, std::sync::atomic::Ordering::Release);
    }

    fn is_trusted(&self) -> bool {
        self.is_trusted.load(std::sync::atomic::Ordering::Acquire)
    }
}
