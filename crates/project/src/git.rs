use crate::{
    buffer_store::{BufferStore, BufferStoreEvent},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    Project, ProjectEnvironment, ProjectItem, ProjectPath,
};
use anyhow::{anyhow, Context as _, Result};
use askpass::{AskPassDelegate, AskPassSession};
use buffer_diff::{BufferDiff, BufferDiffEvent};
use client::ProjectId;
use collections::HashMap;
use fs::Fs;
use futures::{
    channel::{mpsc, oneshot},
    future::{OptionFuture, Shared},
    FutureExt as _, StreamExt as _,
};
use git::repository::DiffType;
use git::{
    repository::{
        Branch, CommitDetails, GitRepository, PushOptions, Remote, RemoteCommandOutput, RepoPath,
        ResetMode,
    },
    status::FileStatus,
};
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Subscription, Task,
    WeakEntity,
};
use language::{Buffer, BufferEvent, Language, LanguageRegistry};
use parking_lot::Mutex;
use rpc::{
    proto::{self, git_reset, ToProto, SSH_PROJECT_ID},
    AnyProtoClient, TypedEnvelope,
};
use settings::WorktreeId;
use std::{
    collections::{hash_map, VecDeque},
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};

use text::BufferId;
use util::{debug_panic, maybe, ResultExt};
use worktree::{
    File, ProjectEntryId, RepositoryEntry, StatusEntry, UpdatedGitRepositoriesSet, WorkDirectory,
    Worktree,
};

pub struct GitStore {
    state: GitStoreState,
    buffer_store: Entity<BufferStore>,
    repositories: Vec<Entity<Repository>>,
    #[allow(clippy::type_complexity)]
    loading_diffs:
        HashMap<(BufferId, DiffKind), Shared<Task<Result<Entity<BufferDiff>, Arc<anyhow::Error>>>>>,
    diffs: HashMap<BufferId, Entity<BufferDiffState>>,
    active_index: Option<usize>,
    update_sender: mpsc::UnboundedSender<GitJob>,
    shared_diffs: HashMap<proto::PeerId, HashMap<BufferId, SharedDiffs>>,
    _subscriptions: [Subscription; 2],
}

#[derive(Default)]
struct SharedDiffs {
    unstaged: Option<Entity<BufferDiff>>,
    uncommitted: Option<Entity<BufferDiff>>,
}

#[derive(Default)]
struct BufferDiffState {
    unstaged_diff: Option<WeakEntity<BufferDiff>>,
    uncommitted_diff: Option<WeakEntity<BufferDiff>>,
    recalculate_diff_task: Option<Task<Result<()>>>,
    language: Option<Arc<Language>>,
    language_registry: Option<Arc<LanguageRegistry>>,
    diff_updated_futures: Vec<oneshot::Sender<()>>,

    head_text: Option<Arc<String>>,
    index_text: Option<Arc<String>>,
    head_changed: bool,
    index_changed: bool,
    language_changed: bool,
}

#[derive(Clone, Debug)]
enum DiffBasesChange {
    SetIndex(Option<String>),
    SetHead(Option<String>),
    SetEach {
        index: Option<String>,
        head: Option<String>,
    },
    SetBoth(Option<String>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DiffKind {
    Unstaged,
    Uncommitted,
}

enum GitStoreState {
    Local {
        downstream_client: Option<(AnyProtoClient, ProjectId)>,
        environment: Entity<ProjectEnvironment>,
        fs: Arc<dyn Fs>,
    },
    Ssh {
        upstream_client: AnyProtoClient,
        upstream_project_id: ProjectId,
        downstream_client: Option<(AnyProtoClient, ProjectId)>,
        environment: Entity<ProjectEnvironment>,
    },
    Remote {
        upstream_client: AnyProtoClient,
        project_id: ProjectId,
    },
}

pub struct Repository {
    commit_message_buffer: Option<Entity<Buffer>>,
    git_store: WeakEntity<GitStore>,
    project_environment: Option<WeakEntity<ProjectEnvironment>>,
    pub worktree_id: WorktreeId,
    pub repository_entry: RepositoryEntry,
    pub dot_git_abs_path: PathBuf,
    pub worktree_abs_path: Arc<Path>,
    pub is_from_single_file_worktree: bool,
    pub git_repo: GitRepo,
    pub merge_message: Option<String>,
    job_sender: mpsc::UnboundedSender<GitJob>,
    askpass_delegates: Arc<Mutex<HashMap<u64, AskPassDelegate>>>,
    latest_askpass_id: u64,
}

#[derive(Clone)]
pub enum GitRepo {
    Local(Arc<dyn GitRepository>),
    Remote {
        project_id: ProjectId,
        client: AnyProtoClient,
        worktree_id: WorktreeId,
        work_directory_id: ProjectEntryId,
    },
}

#[derive(Debug)]
pub enum GitEvent {
    ActiveRepositoryChanged,
    FileSystemUpdated,
    GitStateUpdated,
    IndexWriteError(anyhow::Error),
}

struct GitJob {
    job: Box<dyn FnOnce(&mut AsyncApp) -> Task<()>>,
    key: Option<GitJobKey>,
}

#[derive(PartialEq, Eq)]
enum GitJobKey {
    WriteIndex(RepoPath),
}

impl EventEmitter<GitEvent> for GitStore {}

impl GitStore {
    pub fn local(
        worktree_store: &Entity<WorktreeStore>,
        buffer_store: Entity<BufferStore>,
        environment: Entity<ProjectEnvironment>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(
            worktree_store,
            buffer_store,
            GitStoreState::Local {
                downstream_client: None,
                environment,
                fs,
            },
            cx,
        )
    }

    pub fn remote(
        worktree_store: &Entity<WorktreeStore>,
        buffer_store: Entity<BufferStore>,
        upstream_client: AnyProtoClient,
        project_id: ProjectId,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(
            worktree_store,
            buffer_store,
            GitStoreState::Remote {
                upstream_client,
                project_id,
            },
            cx,
        )
    }

    pub fn ssh(
        worktree_store: &Entity<WorktreeStore>,
        buffer_store: Entity<BufferStore>,
        environment: Entity<ProjectEnvironment>,
        upstream_client: AnyProtoClient,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(
            worktree_store,
            buffer_store,
            GitStoreState::Ssh {
                upstream_client,
                upstream_project_id: ProjectId(SSH_PROJECT_ID),
                downstream_client: None,
                environment,
            },
            cx,
        )
    }

    fn new(
        worktree_store: &Entity<WorktreeStore>,
        buffer_store: Entity<BufferStore>,
        state: GitStoreState,
        cx: &mut Context<Self>,
    ) -> Self {
        let update_sender = Self::spawn_git_worker(cx);
        let _subscriptions = [
            cx.subscribe(worktree_store, Self::on_worktree_store_event),
            cx.subscribe(&buffer_store, Self::on_buffer_store_event),
        ];

        GitStore {
            state,
            buffer_store,
            repositories: Vec::new(),
            active_index: None,
            update_sender,
            _subscriptions,
            loading_diffs: HashMap::default(),
            shared_diffs: HashMap::default(),
            diffs: HashMap::default(),
        }
    }

    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_get_remotes);
        client.add_entity_request_handler(Self::handle_get_branches);
        client.add_entity_request_handler(Self::handle_change_branch);
        client.add_entity_request_handler(Self::handle_create_branch);
        client.add_entity_request_handler(Self::handle_git_init);
        client.add_entity_request_handler(Self::handle_push);
        client.add_entity_request_handler(Self::handle_pull);
        client.add_entity_request_handler(Self::handle_fetch);
        client.add_entity_request_handler(Self::handle_stage);
        client.add_entity_request_handler(Self::handle_unstage);
        client.add_entity_request_handler(Self::handle_commit);
        client.add_entity_request_handler(Self::handle_reset);
        client.add_entity_request_handler(Self::handle_show);
        client.add_entity_request_handler(Self::handle_checkout_files);
        client.add_entity_request_handler(Self::handle_open_commit_message_buffer);
        client.add_entity_request_handler(Self::handle_set_index_text);
        client.add_entity_request_handler(Self::handle_askpass);
        client.add_entity_request_handler(Self::handle_check_for_pushed_commits);
        client.add_entity_request_handler(Self::handle_git_diff);
        client.add_entity_request_handler(Self::handle_open_unstaged_diff);
        client.add_entity_request_handler(Self::handle_open_uncommitted_diff);
        client.add_entity_message_handler(Self::handle_update_diff_bases);
    }

    pub fn is_local(&self) -> bool {
        matches!(self.state, GitStoreState::Local { .. })
    }

    pub fn shared(&mut self, remote_id: u64, client: AnyProtoClient, _cx: &mut App) {
        match &mut self.state {
            GitStoreState::Local {
                downstream_client, ..
            }
            | GitStoreState::Ssh {
                downstream_client, ..
            } => {
                *downstream_client = Some((client, ProjectId(remote_id)));
            }
            GitStoreState::Remote { .. } => {
                debug_panic!("shared called on remote store");
            }
        }
    }

    pub fn unshared(&mut self, _cx: &mut Context<Self>) {
        match &mut self.state {
            GitStoreState::Local {
                downstream_client, ..
            }
            | GitStoreState::Ssh {
                downstream_client, ..
            } => {
                downstream_client.take();
            }
            GitStoreState::Remote { .. } => {
                debug_panic!("unshared called on remote store");
            }
        }
        self.shared_diffs.clear();
    }

    pub(crate) fn forget_shared_diffs_for(&mut self, peer_id: &proto::PeerId) {
        self.shared_diffs.remove(peer_id);
    }

    pub fn active_repository(&self) -> Option<Entity<Repository>> {
        self.active_index
            .map(|index| self.repositories[index].clone())
    }

    pub fn open_unstaged_diff(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferDiff>>> {
        let buffer_id = buffer.read(cx).remote_id();
        if let Some(diff_state) = self.diffs.get(&buffer_id) {
            if let Some(unstaged_diff) = diff_state
                .read(cx)
                .unstaged_diff
                .as_ref()
                .and_then(|weak| weak.upgrade())
            {
                if let Some(task) =
                    diff_state.update(cx, |diff_state, _| diff_state.wait_for_recalculation())
                {
                    return cx.background_executor().spawn(async move {
                        task.await?;
                        Ok(unstaged_diff)
                    });
                }
                return Task::ready(Ok(unstaged_diff));
            }
        }

        let task = match self.loading_diffs.entry((buffer_id, DiffKind::Unstaged)) {
            hash_map::Entry::Occupied(e) => e.get().clone(),
            hash_map::Entry::Vacant(entry) => {
                let staged_text = self.state.load_staged_text(&buffer, &self.buffer_store, cx);
                entry
                    .insert(
                        cx.spawn(move |this, cx| async move {
                            Self::open_diff_internal(
                                this,
                                DiffKind::Unstaged,
                                staged_text.await.map(DiffBasesChange::SetIndex),
                                buffer,
                                cx,
                            )
                            .await
                            .map_err(Arc::new)
                        })
                        .shared(),
                    )
                    .clone()
            }
        };

        cx.background_spawn(async move { task.await.map_err(|e| anyhow!("{e}")) })
    }

    pub fn open_uncommitted_diff(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferDiff>>> {
        let buffer_id = buffer.read(cx).remote_id();

        if let Some(diff_state) = self.diffs.get(&buffer_id) {
            if let Some(uncommitted_diff) = diff_state
                .read(cx)
                .uncommitted_diff
                .as_ref()
                .and_then(|weak| weak.upgrade())
            {
                if let Some(task) =
                    diff_state.update(cx, |diff_state, _| diff_state.wait_for_recalculation())
                {
                    return cx.background_executor().spawn(async move {
                        task.await?;
                        Ok(uncommitted_diff)
                    });
                }
                return Task::ready(Ok(uncommitted_diff));
            }
        }

        let task = match self.loading_diffs.entry((buffer_id, DiffKind::Uncommitted)) {
            hash_map::Entry::Occupied(e) => e.get().clone(),
            hash_map::Entry::Vacant(entry) => {
                let changes = self
                    .state
                    .load_committed_text(&buffer, &self.buffer_store, cx);

                entry
                    .insert(
                        cx.spawn(move |this, cx| async move {
                            Self::open_diff_internal(
                                this,
                                DiffKind::Uncommitted,
                                changes.await,
                                buffer,
                                cx,
                            )
                            .await
                            .map_err(Arc::new)
                        })
                        .shared(),
                    )
                    .clone()
            }
        };

        cx.background_spawn(async move { task.await.map_err(|e| anyhow!("{e}")) })
    }

    async fn open_diff_internal(
        this: WeakEntity<Self>,
        kind: DiffKind,
        texts: Result<DiffBasesChange>,
        buffer_entity: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Entity<BufferDiff>> {
        let diff_bases_change = match texts {
            Err(e) => {
                this.update(&mut cx, |this, cx| {
                    let buffer = buffer_entity.read(cx);
                    let buffer_id = buffer.remote_id();
                    this.loading_diffs.remove(&(buffer_id, kind));
                })?;
                return Err(e);
            }
            Ok(change) => change,
        };

        this.update(&mut cx, |this, cx| {
            let buffer = buffer_entity.read(cx);
            let buffer_id = buffer.remote_id();
            let language = buffer.language().cloned();
            let language_registry = buffer.language_registry();
            let text_snapshot = buffer.text_snapshot();
            this.loading_diffs.remove(&(buffer_id, kind));

            let diff_state = this
                .diffs
                .entry(buffer_id)
                .or_insert_with(|| cx.new(|_| BufferDiffState::default()));

            let diff = cx.new(|cx| BufferDiff::new(&text_snapshot, cx));

            cx.subscribe(&diff, Self::on_buffer_diff_event).detach();
            diff_state.update(cx, |diff_state, cx| {
                diff_state.language = language;
                diff_state.language_registry = language_registry;

                match kind {
                    DiffKind::Unstaged => diff_state.unstaged_diff = Some(diff.downgrade()),
                    DiffKind::Uncommitted => {
                        let unstaged_diff = if let Some(diff) = diff_state.unstaged_diff() {
                            diff
                        } else {
                            let unstaged_diff = cx.new(|cx| BufferDiff::new(&text_snapshot, cx));
                            diff_state.unstaged_diff = Some(unstaged_diff.downgrade());
                            unstaged_diff
                        };

                        diff.update(cx, |diff, _| diff.set_secondary_diff(unstaged_diff));
                        diff_state.uncommitted_diff = Some(diff.downgrade())
                    }
                }

                let rx = diff_state.diff_bases_changed(text_snapshot, diff_bases_change, cx);

                anyhow::Ok(async move {
                    rx.await.ok();
                    Ok(diff)
                })
            })
        })??
        .await
    }

    pub fn get_unstaged_diff(&self, buffer_id: BufferId, cx: &App) -> Option<Entity<BufferDiff>> {
        let diff_state = self.diffs.get(&buffer_id)?;
        diff_state.read(cx).unstaged_diff.as_ref()?.upgrade()
    }

    pub fn get_uncommitted_diff(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Option<Entity<BufferDiff>> {
        let diff_state = self.diffs.get(&buffer_id)?;
        diff_state.read(cx).uncommitted_diff.as_ref()?.upgrade()
    }

    fn downstream_client(&self) -> Option<(AnyProtoClient, ProjectId)> {
        match &self.state {
            GitStoreState::Local {
                downstream_client, ..
            }
            | GitStoreState::Ssh {
                downstream_client, ..
            } => downstream_client.clone(),
            GitStoreState::Remote { .. } => None,
        }
    }

    fn upstream_client(&self) -> Option<AnyProtoClient> {
        match &self.state {
            GitStoreState::Local { .. } => None,
            GitStoreState::Ssh {
                upstream_client, ..
            }
            | GitStoreState::Remote {
                upstream_client, ..
            } => Some(upstream_client.clone()),
        }
    }

    fn project_environment(&self) -> Option<Entity<ProjectEnvironment>> {
        match &self.state {
            GitStoreState::Local { environment, .. } => Some(environment.clone()),
            GitStoreState::Ssh { environment, .. } => Some(environment.clone()),
            GitStoreState::Remote { .. } => None,
        }
    }

    fn project_id(&self) -> Option<ProjectId> {
        match &self.state {
            GitStoreState::Local { .. } => None,
            GitStoreState::Ssh { .. } => Some(ProjectId(proto::SSH_PROJECT_ID)),
            GitStoreState::Remote { project_id, .. } => Some(*project_id),
        }
    }

    fn on_worktree_store_event(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        let mut new_repositories = Vec::new();
        let mut new_active_index = None;
        let this = cx.weak_entity();
        let upstream_client = self.upstream_client();
        let project_id = self.project_id();

        worktree_store.update(cx, |worktree_store, cx| {
            for worktree in worktree_store.worktrees() {
                worktree.update(cx, |worktree, cx| {
                    let snapshot = worktree.snapshot();
                    for repo in snapshot.repositories().iter() {
                        let git_data = worktree
                            .as_local()
                            .and_then(|local_worktree| local_worktree.get_local_repo(repo))
                            .map(|local_repo| {
                                (
                                    GitRepo::Local(local_repo.repo().clone()),
                                    local_repo.merge_message.clone(),
                                )
                            })
                            .or_else(|| {
                                let client = upstream_client
                                    .clone()
                                    .context("no upstream client")
                                    .log_err()?;
                                let project_id = project_id?;
                                Some((
                                    GitRepo::Remote {
                                        project_id,
                                        client,
                                        worktree_id: worktree.id(),
                                        work_directory_id: repo.work_directory_id(),
                                    },
                                    None,
                                ))
                            });
                        let Some((git_repo, merge_message)) = git_data else {
                            continue;
                        };
                        let worktree_id = worktree.id();
                        let existing =
                            self.repositories
                                .iter()
                                .enumerate()
                                .find(|(_, existing_handle)| {
                                    existing_handle.read(cx).id()
                                        == (worktree_id, repo.work_directory_id())
                                });
                        let handle = if let Some((index, handle)) = existing {
                            if self.active_index == Some(index) {
                                new_active_index = Some(new_repositories.len());
                            }
                            // Update the statuses and merge message but keep everything else.
                            let existing_handle = handle.clone();
                            existing_handle.update(cx, |existing_handle, cx| {
                                existing_handle.repository_entry = repo.clone();
                                if matches!(git_repo, GitRepo::Local { .. })
                                    && existing_handle.merge_message != merge_message
                                {
                                    if let (Some(merge_message), Some(buffer)) =
                                        (&merge_message, &existing_handle.commit_message_buffer)
                                    {
                                        buffer.update(cx, |buffer, cx| {
                                            if buffer.is_empty() {
                                                buffer.set_text(merge_message.as_str(), cx);
                                            }
                                        })
                                    }
                                    existing_handle.merge_message = merge_message;
                                }
                            });
                            existing_handle
                        } else {
                            let environment = self.project_environment();
                            cx.new(|_| Repository {
                                project_environment: environment
                                    .as_ref()
                                    .map(|env| env.downgrade()),
                                git_store: this.clone(),
                                worktree_id,
                                askpass_delegates: Default::default(),
                                latest_askpass_id: 0,
                                repository_entry: repo.clone(),
                                dot_git_abs_path: worktree.dot_git_abs_path(&repo.work_directory),
                                worktree_abs_path: worktree.abs_path(),
                                is_from_single_file_worktree: worktree.is_single_file(),
                                git_repo,
                                job_sender: self.update_sender.clone(),
                                merge_message,
                                commit_message_buffer: None,
                            })
                        };
                        new_repositories.push(handle);
                    }
                })
            }
        });

        if new_active_index == None && new_repositories.len() > 0 {
            new_active_index = Some(0);
        }

        self.repositories = new_repositories;
        self.active_index = new_active_index;

        match event {
            WorktreeStoreEvent::WorktreeUpdatedGitRepositories(_) => {
                cx.emit(GitEvent::GitStateUpdated);
            }
            WorktreeStoreEvent::WorktreeAdded(worktree) => {
                if self.is_local() {
                    cx.subscribe(worktree, Self::on_worktree_event).detach();
                }
            }
            _ => {
                cx.emit(GitEvent::FileSystemUpdated);
            }
        }
    }

    fn on_worktree_event(
        &mut self,
        worktree: Entity<Worktree>,
        event: &worktree::Event,
        cx: &mut Context<Self>,
    ) {
        if let worktree::Event::UpdatedGitRepositories(changed_repos) = event {
            self.local_worktree_git_repos_changed(worktree, changed_repos, cx);
        }
    }

    fn on_buffer_store_event(
        &mut self,
        _: Entity<BufferStore>,
        event: &BufferStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            BufferStoreEvent::BufferAdded(buffer) => {
                cx.subscribe(&buffer, |this, buffer, event, cx| {
                    if let BufferEvent::LanguageChanged = event {
                        let buffer_id = buffer.read(cx).remote_id();
                        if let Some(diff_state) = this.diffs.get(&buffer_id) {
                            diff_state.update(cx, |diff_state, cx| {
                                diff_state.buffer_language_changed(buffer, cx);
                            });
                        }
                    }
                })
                .detach();
            }
            BufferStoreEvent::SharedBufferClosed(peer_id, buffer_id) => {
                if let Some(diffs) = self.shared_diffs.get_mut(peer_id) {
                    diffs.remove(buffer_id);
                }
            }
            BufferStoreEvent::BufferDropped(buffer_id) => {
                self.diffs.remove(&buffer_id);
                for diffs in self.shared_diffs.values_mut() {
                    diffs.remove(buffer_id);
                }
            }

            _ => {}
        }
    }

    pub fn recalculate_buffer_diffs(
        &mut self,
        buffers: Vec<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = ()> {
        let mut futures = Vec::new();
        for buffer in buffers {
            if let Some(diff_state) = self.diffs.get_mut(&buffer.read(cx).remote_id()) {
                let buffer = buffer.read(cx).text_snapshot();
                futures.push(diff_state.update(cx, |diff_state, cx| {
                    diff_state.recalculate_diffs(buffer, cx)
                }));
            }
        }
        async move {
            futures::future::join_all(futures).await;
        }
    }

    fn on_buffer_diff_event(
        &mut self,
        diff: Entity<buffer_diff::BufferDiff>,
        event: &BufferDiffEvent,
        cx: &mut Context<Self>,
    ) {
        if let BufferDiffEvent::HunksStagedOrUnstaged(new_index_text) = event {
            let buffer_id = diff.read(cx).buffer_id;
            if let Some((repo, path)) = self.repository_and_path_for_buffer_id(buffer_id, cx) {
                let recv = repo.update(cx, |repo, cx| {
                    log::debug!("updating index text for buffer {}", path.display());
                    repo.set_index_text(
                        path,
                        new_index_text.as_ref().map(|rope| rope.to_string()),
                        cx,
                    )
                });
                let diff = diff.downgrade();
                cx.spawn(|this, mut cx| async move {
                    if let Some(result) = cx.background_spawn(async move { recv.await.ok() }).await
                    {
                        if let Err(error) = result {
                            diff.update(&mut cx, |diff, cx| {
                                diff.clear_pending_hunks(cx);
                            })
                            .ok();
                            this.update(&mut cx, |_, cx| cx.emit(GitEvent::IndexWriteError(error)))
                                .ok();
                        }
                    }
                })
                .detach();
            }
        }
    }

    fn local_worktree_git_repos_changed(
        &mut self,
        worktree: Entity<Worktree>,
        changed_repos: &UpdatedGitRepositoriesSet,
        cx: &mut Context<Self>,
    ) {
        debug_assert!(worktree.read(cx).is_local());

        let mut diff_state_updates = Vec::new();
        for (buffer_id, diff_state) in &self.diffs {
            let Some(buffer) = self.buffer_store.read(cx).get(*buffer_id) else {
                continue;
            };
            let Some(file) = File::from_dyn(buffer.read(cx).file()) else {
                continue;
            };
            if file.worktree != worktree
                || !changed_repos
                    .iter()
                    .any(|(work_dir, _)| file.path.starts_with(work_dir))
            {
                continue;
            }

            let diff_state = diff_state.read(cx);
            let has_unstaged_diff = diff_state
                .unstaged_diff
                .as_ref()
                .is_some_and(|diff| diff.is_upgradable());
            let has_uncommitted_diff = diff_state
                .uncommitted_diff
                .as_ref()
                .is_some_and(|set| set.is_upgradable());
            diff_state_updates.push((
                buffer,
                file.path.clone(),
                has_unstaged_diff.then(|| diff_state.index_text.clone()),
                has_uncommitted_diff.then(|| diff_state.head_text.clone()),
            ));
        }

        if diff_state_updates.is_empty() {
            return;
        }

        cx.spawn(move |this, mut cx| async move {
            let snapshot =
                worktree.update(&mut cx, |tree, _| tree.as_local().unwrap().snapshot())?;

            let mut diff_bases_changes_by_buffer = Vec::new();
            for (buffer, path, current_index_text, current_head_text) in diff_state_updates {
                log::debug!("reloading git state for buffer {}", path.display());
                let Some(local_repo) = snapshot.local_repo_for_path(&path) else {
                    continue;
                };
                let Some(relative_path) = local_repo.relativize(&path).ok() else {
                    continue;
                };
                let index_text = if current_index_text.is_some() {
                    local_repo
                        .repo()
                        .load_index_text(relative_path.clone(), cx.clone())
                        .await
                } else {
                    None
                };
                let head_text = if current_head_text.is_some() {
                    local_repo
                        .repo()
                        .load_committed_text(relative_path, cx.clone())
                        .await
                } else {
                    None
                };

                // Avoid triggering a diff update if the base text has not changed.
                if let Some((current_index, current_head)) =
                    current_index_text.as_ref().zip(current_head_text.as_ref())
                {
                    if current_index.as_deref() == index_text.as_ref()
                        && current_head.as_deref() == head_text.as_ref()
                    {
                        continue;
                    }
                }

                let diff_bases_change =
                    match (current_index_text.is_some(), current_head_text.is_some()) {
                        (true, true) => Some(if index_text == head_text {
                            DiffBasesChange::SetBoth(head_text)
                        } else {
                            DiffBasesChange::SetEach {
                                index: index_text,
                                head: head_text,
                            }
                        }),
                        (true, false) => Some(DiffBasesChange::SetIndex(index_text)),
                        (false, true) => Some(DiffBasesChange::SetHead(head_text)),
                        (false, false) => None,
                    };

                diff_bases_changes_by_buffer.push((buffer, diff_bases_change))
            }

            this.update(&mut cx, |this, cx| {
                for (buffer, diff_bases_change) in diff_bases_changes_by_buffer {
                    let Some(diff_state) = this.diffs.get(&buffer.read(cx).remote_id()) else {
                        continue;
                    };
                    let Some(diff_bases_change) = diff_bases_change else {
                        continue;
                    };

                    let downstream_client = this.downstream_client();
                    diff_state.update(cx, |diff_state, cx| {
                        use proto::update_diff_bases::Mode;

                        let buffer = buffer.read(cx);
                        if let Some((client, project_id)) = downstream_client {
                            let (staged_text, committed_text, mode) = match diff_bases_change
                                .clone()
                            {
                                DiffBasesChange::SetIndex(index) => (index, None, Mode::IndexOnly),
                                DiffBasesChange::SetHead(head) => (None, head, Mode::HeadOnly),
                                DiffBasesChange::SetEach { index, head } => {
                                    (index, head, Mode::IndexAndHead)
                                }
                                DiffBasesChange::SetBoth(text) => {
                                    (None, text, Mode::IndexMatchesHead)
                                }
                            };
                            let message = proto::UpdateDiffBases {
                                project_id: project_id.to_proto(),
                                buffer_id: buffer.remote_id().to_proto(),
                                staged_text,
                                committed_text,
                                mode: mode as i32,
                            };

                            client.send(message).log_err();
                        }

                        let _ = diff_state.diff_bases_changed(
                            buffer.text_snapshot(),
                            diff_bases_change,
                            cx,
                        );
                    });
                }
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn all_repositories(&self) -> Vec<Entity<Repository>> {
        self.repositories.clone()
    }

    pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus> {
        let (repo, path) = self.repository_and_path_for_buffer_id(buffer_id, cx)?;
        let status = repo.read(cx).repository_entry.status_for_path(&path)?;
        Some(status.status)
    }

    fn repository_and_path_for_buffer_id(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Option<(Entity<Repository>, RepoPath)> {
        let buffer = self.buffer_store.read(cx).get(buffer_id)?;
        let path = buffer.read(cx).project_path(cx)?;
        let mut result: Option<(Entity<Repository>, RepoPath)> = None;
        for repo_handle in &self.repositories {
            let repo = repo_handle.read(cx);
            if repo.worktree_id == path.worktree_id {
                if let Ok(relative_path) = repo.repository_entry.relativize(&path.path) {
                    if result
                        .as_ref()
                        .is_none_or(|(result, _)| !repo.contains_sub_repo(result, cx))
                    {
                        result = Some((repo_handle.clone(), relative_path))
                    }
                }
            }
        }
        result
    }

    fn spawn_git_worker(cx: &mut Context<GitStore>) -> mpsc::UnboundedSender<GitJob> {
        let (job_tx, mut job_rx) = mpsc::unbounded::<GitJob>();

        cx.spawn(|_, mut cx| async move {
            let mut jobs = VecDeque::new();
            loop {
                while let Ok(Some(next_job)) = job_rx.try_next() {
                    jobs.push_back(next_job);
                }

                if let Some(job) = jobs.pop_front() {
                    if let Some(current_key) = &job.key {
                        if jobs
                            .iter()
                            .any(|other_job| other_job.key.as_ref() == Some(current_key))
                        {
                            continue;
                        }
                    }
                    (job.job)(&mut cx).await;
                } else if let Some(job) = job_rx.next().await {
                    jobs.push_back(job);
                } else {
                    break;
                }
            }
        })
        .detach();
        job_tx
    }

    pub fn git_init(
        &self,
        path: Arc<Path>,
        fallback_branch_name: String,
        cx: &App,
    ) -> Task<Result<()>> {
        match &self.state {
            GitStoreState::Local { fs, .. } => {
                let fs = fs.clone();
                cx.background_executor()
                    .spawn(async move { fs.git_init(&path, fallback_branch_name) })
            }
            GitStoreState::Ssh {
                upstream_client,
                upstream_project_id: project_id,
                ..
            }
            | GitStoreState::Remote {
                upstream_client,
                project_id,
                ..
            } => {
                let client = upstream_client.clone();
                let project_id = *project_id;
                cx.background_executor().spawn(async move {
                    client
                        .request(proto::GitInit {
                            project_id: project_id.0,
                            abs_path: path.to_string_lossy().to_string(),
                            fallback_branch_name,
                        })
                        .await?;
                    Ok(())
                })
            }
        }
    }

    async fn handle_git_init(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitInit>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let path: Arc<Path> = PathBuf::from(envelope.payload.abs_path).into();
        let name = envelope.payload.fallback_branch_name;
        cx.update(|cx| this.read(cx).git_init(path, name, cx))?
            .await?;

        Ok(proto::Ack {})
    }

    async fn handle_fetch(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Fetch>,
        mut cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let askpass_id = envelope.payload.askpass_id;

        let askpass = make_remote_delegate(
            this,
            envelope.payload.project_id,
            worktree_id,
            work_directory_id,
            askpass_id,
            &mut cx,
        );

        let remote_output = repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.fetch(askpass, cx)
            })?
            .await??;

        Ok(proto::RemoteMessageResponse {
            stdout: remote_output.stdout,
            stderr: remote_output.stderr,
        })
    }

    async fn handle_push(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Push>,
        mut cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let askpass_id = envelope.payload.askpass_id;
        let askpass = make_remote_delegate(
            this,
            envelope.payload.project_id,
            worktree_id,
            work_directory_id,
            askpass_id,
            &mut cx,
        );

        let options = envelope
            .payload
            .options
            .as_ref()
            .map(|_| match envelope.payload.options() {
                proto::push::PushOptions::SetUpstream => git::repository::PushOptions::SetUpstream,
                proto::push::PushOptions::Force => git::repository::PushOptions::Force,
            });

        let branch_name = envelope.payload.branch_name.into();
        let remote_name = envelope.payload.remote_name.into();

        let remote_output = repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.push(branch_name, remote_name, options, askpass, cx)
            })?
            .await??;
        Ok(proto::RemoteMessageResponse {
            stdout: remote_output.stdout,
            stderr: remote_output.stderr,
        })
    }

    async fn handle_pull(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Pull>,
        mut cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let askpass_id = envelope.payload.askpass_id;
        let askpass = make_remote_delegate(
            this,
            envelope.payload.project_id,
            worktree_id,
            work_directory_id,
            askpass_id,
            &mut cx,
        );

        let branch_name = envelope.payload.branch_name.into();
        let remote_name = envelope.payload.remote_name.into();

        let remote_message = repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.pull(branch_name, remote_name, askpass, cx)
            })?
            .await??;

        Ok(proto::RemoteMessageResponse {
            stdout: remote_message.stdout,
            stderr: remote_message.stderr,
        })
    }

    async fn handle_stage(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Stage>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let entries = envelope
            .payload
            .paths
            .into_iter()
            .map(PathBuf::from)
            .map(RepoPath::new)
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.stage_entries(entries, cx)
            })?
            .await?;
        Ok(proto::Ack {})
    }

    async fn handle_unstage(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Unstage>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let entries = envelope
            .payload
            .paths
            .into_iter()
            .map(PathBuf::from)
            .map(RepoPath::new)
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.unstage_entries(entries, cx)
            })?
            .await?;

        Ok(proto::Ack {})
    }

    async fn handle_set_index_text(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SetIndexText>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.set_index_text(
                    RepoPath::from_str(&envelope.payload.path),
                    envelope.payload.text,
                    cx,
                )
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_commit(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Commit>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let message = SharedString::from(envelope.payload.message);
        let name = envelope.payload.name.map(SharedString::from);
        let email = envelope.payload.email.map(SharedString::from);

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.commit(message, name.zip(email), cx)
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_get_remotes(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetRemotes>,
        mut cx: AsyncApp,
    ) -> Result<proto::GetRemotesResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let branch_name = envelope.payload.branch_name;

        let remotes = repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.get_remotes(branch_name)
            })?
            .await??;

        Ok(proto::GetRemotesResponse {
            remotes: remotes
                .into_iter()
                .map(|remotes| proto::get_remotes_response::Remote {
                    name: remotes.name.to_string(),
                })
                .collect::<Vec<_>>(),
        })
    }

    async fn handle_get_branches(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitGetBranches>,
        mut cx: AsyncApp,
    ) -> Result<proto::GitBranchesResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let branches = repository_handle
            .update(&mut cx, |repository_handle, _| repository_handle.branches())?
            .await??;

        Ok(proto::GitBranchesResponse {
            branches: branches
                .into_iter()
                .map(|branch| worktree::branch_to_proto(&branch))
                .collect::<Vec<_>>(),
        })
    }
    async fn handle_create_branch(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitCreateBranch>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let branch_name = envelope.payload.branch_name;

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.create_branch(branch_name)
            })?
            .await??;

        Ok(proto::Ack {})
    }

    async fn handle_change_branch(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitChangeBranch>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let branch_name = envelope.payload.branch_name;

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.change_branch(branch_name)
            })?
            .await??;

        Ok(proto::Ack {})
    }

    async fn handle_show(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitShow>,
        mut cx: AsyncApp,
    ) -> Result<proto::GitCommitDetails> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let commit = repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.show(envelope.payload.commit)
            })?
            .await??;
        Ok(proto::GitCommitDetails {
            sha: commit.sha.into(),
            message: commit.message.into(),
            commit_timestamp: commit.commit_timestamp,
            committer_email: commit.committer_email.into(),
            committer_name: commit.committer_name.into(),
        })
    }

    async fn handle_reset(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitReset>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let mode = match envelope.payload.mode() {
            git_reset::ResetMode::Soft => ResetMode::Soft,
            git_reset::ResetMode::Mixed => ResetMode::Mixed,
        };

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.reset(envelope.payload.commit, mode, cx)
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_checkout_files(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitCheckoutFiles>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let paths = envelope
            .payload
            .paths
            .iter()
            .map(|s| RepoPath::from_str(s))
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.checkout_files(&envelope.payload.commit, paths, cx)
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_open_commit_message_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenCommitMessageBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let buffer = repository
            .update(&mut cx, |repository, cx| {
                repository.open_commit_buffer(None, this.read(cx).buffer_store.clone(), cx)
            })?
            .await?;

        let buffer_id = buffer.read_with(&cx, |buffer, _| buffer.remote_id())?;
        this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store
                    .create_buffer_for_peer(
                        &buffer,
                        envelope.original_sender_id.unwrap_or(envelope.sender_id),
                        cx,
                    )
                    .detach_and_log_err(cx);
            })
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    async fn handle_askpass(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::AskPassRequest>,
        mut cx: AsyncApp,
    ) -> Result<proto::AskPassResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let delegates = cx.update(|cx| repository.read(cx).askpass_delegates.clone())?;
        let Some(mut askpass) = delegates.lock().remove(&envelope.payload.askpass_id) else {
            debug_panic!("no askpass found");
            return Err(anyhow::anyhow!("no askpass found"));
        };

        let response = askpass.ask_password(envelope.payload.prompt).await?;

        delegates
            .lock()
            .insert(envelope.payload.askpass_id, askpass);

        Ok(proto::AskPassResponse { response })
    }

    async fn handle_check_for_pushed_commits(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CheckForPushedCommits>,
        mut cx: AsyncApp,
    ) -> Result<proto::CheckForPushedCommitsResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let branches = repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.check_for_pushed_commits()
            })?
            .await??;
        Ok(proto::CheckForPushedCommitsResponse {
            pushed_to: branches
                .into_iter()
                .map(|commit| commit.to_string())
                .collect(),
        })
    }

    async fn handle_git_diff(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitDiff>,
        mut cx: AsyncApp,
    ) -> Result<proto::GitDiffResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let diff_type = match envelope.payload.diff_type() {
            proto::git_diff::DiffType::HeadToIndex => DiffType::HeadToIndex,
            proto::git_diff::DiffType::HeadToWorktree => DiffType::HeadToWorktree,
        };

        let mut diff = repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.diff(diff_type, cx)
            })?
            .await??;
        const ONE_MB: usize = 1_000_000;
        if diff.len() > ONE_MB {
            diff = diff.chars().take(ONE_MB).collect()
        }

        Ok(proto::GitDiffResponse { diff })
    }

    pub async fn handle_open_unstaged_diff(
        this: Entity<Self>,
        request: TypedEnvelope<proto::OpenUnstagedDiff>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenUnstagedDiffResponse> {
        let buffer_id = BufferId::new(request.payload.buffer_id)?;
        let diff = this
            .update(&mut cx, |this, cx| {
                let buffer = this.buffer_store.read(cx).get(buffer_id)?;
                Some(this.open_unstaged_diff(buffer, cx))
            })?
            .ok_or_else(|| anyhow!("no such buffer"))?
            .await?;
        this.update(&mut cx, |this, _| {
            let shared_diffs = this
                .shared_diffs
                .entry(request.original_sender_id.unwrap_or(request.sender_id))
                .or_default();
            shared_diffs.entry(buffer_id).or_default().unstaged = Some(diff.clone());
        })?;
        let staged_text = diff.read_with(&cx, |diff, _| diff.base_text_string())?;
        Ok(proto::OpenUnstagedDiffResponse { staged_text })
    }

    pub async fn handle_open_uncommitted_diff(
        this: Entity<Self>,
        request: TypedEnvelope<proto::OpenUncommittedDiff>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenUncommittedDiffResponse> {
        let buffer_id = BufferId::new(request.payload.buffer_id)?;
        let diff = this
            .update(&mut cx, |this, cx| {
                let buffer = this.buffer_store.read(cx).get(buffer_id)?;
                Some(this.open_uncommitted_diff(buffer, cx))
            })?
            .ok_or_else(|| anyhow!("no such buffer"))?
            .await?;
        this.update(&mut cx, |this, _| {
            let shared_diffs = this
                .shared_diffs
                .entry(request.original_sender_id.unwrap_or(request.sender_id))
                .or_default();
            shared_diffs.entry(buffer_id).or_default().uncommitted = Some(diff.clone());
        })?;
        diff.read_with(&cx, |diff, cx| {
            use proto::open_uncommitted_diff_response::Mode;

            let unstaged_diff = diff.secondary_diff();
            let index_snapshot = unstaged_diff.and_then(|diff| {
                let diff = diff.read(cx);
                diff.base_text_exists().then(|| diff.base_text())
            });

            let mode;
            let staged_text;
            let committed_text;
            if diff.base_text_exists() {
                let committed_snapshot = diff.base_text();
                committed_text = Some(committed_snapshot.text());
                if let Some(index_text) = index_snapshot {
                    if index_text.remote_id() == committed_snapshot.remote_id() {
                        mode = Mode::IndexMatchesHead;
                        staged_text = None;
                    } else {
                        mode = Mode::IndexAndHead;
                        staged_text = Some(index_text.text());
                    }
                } else {
                    mode = Mode::IndexAndHead;
                    staged_text = None;
                }
            } else {
                mode = Mode::IndexAndHead;
                committed_text = None;
                staged_text = index_snapshot.as_ref().map(|buffer| buffer.text());
            }

            proto::OpenUncommittedDiffResponse {
                committed_text,
                staged_text,
                mode: mode.into(),
            }
        })
    }

    pub async fn handle_update_diff_bases(
        this: Entity<Self>,
        request: TypedEnvelope<proto::UpdateDiffBases>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let buffer_id = BufferId::new(request.payload.buffer_id)?;
        this.update(&mut cx, |this, cx| {
            if let Some(diff_state) = this.diffs.get_mut(&buffer_id) {
                if let Some(buffer) = this.buffer_store.read(cx).get(buffer_id) {
                    let buffer = buffer.read(cx).text_snapshot();
                    diff_state.update(cx, |diff_state, cx| {
                        diff_state.handle_base_texts_updated(buffer, request.payload, cx);
                    })
                }
            }
        })
    }

    fn repository_for_request(
        this: &Entity<Self>,
        worktree_id: WorktreeId,
        work_directory_id: ProjectEntryId,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Repository>> {
        this.update(cx, |this, cx| {
            this.repositories
                .iter()
                .find(|repository_handle| {
                    repository_handle.read(cx).worktree_id == worktree_id
                        && repository_handle
                            .read(cx)
                            .repository_entry
                            .work_directory_id()
                            == work_directory_id
                })
                .context("missing repository handle")
                .cloned()
        })?
    }
}

impl BufferDiffState {
    fn buffer_language_changed(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.language = buffer.read(cx).language().cloned();
        self.language_changed = true;
        let _ = self.recalculate_diffs(buffer.read(cx).text_snapshot(), cx);
    }

    fn unstaged_diff(&self) -> Option<Entity<BufferDiff>> {
        self.unstaged_diff.as_ref().and_then(|set| set.upgrade())
    }

    fn uncommitted_diff(&self) -> Option<Entity<BufferDiff>> {
        self.uncommitted_diff.as_ref().and_then(|set| set.upgrade())
    }

    fn handle_base_texts_updated(
        &mut self,
        buffer: text::BufferSnapshot,
        message: proto::UpdateDiffBases,
        cx: &mut Context<Self>,
    ) {
        use proto::update_diff_bases::Mode;

        let Some(mode) = Mode::from_i32(message.mode) else {
            return;
        };

        let diff_bases_change = match mode {
            Mode::HeadOnly => DiffBasesChange::SetHead(message.committed_text),
            Mode::IndexOnly => DiffBasesChange::SetIndex(message.staged_text),
            Mode::IndexMatchesHead => DiffBasesChange::SetBoth(message.committed_text),
            Mode::IndexAndHead => DiffBasesChange::SetEach {
                index: message.staged_text,
                head: message.committed_text,
            },
        };

        let _ = self.diff_bases_changed(buffer, diff_bases_change, cx);
    }

    pub fn wait_for_recalculation(&mut self) -> Option<oneshot::Receiver<()>> {
        if self.diff_updated_futures.is_empty() {
            return None;
        }
        let (tx, rx) = oneshot::channel();
        self.diff_updated_futures.push(tx);
        Some(rx)
    }

    fn diff_bases_changed(
        &mut self,
        buffer: text::BufferSnapshot,
        diff_bases_change: DiffBasesChange,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<()> {
        match diff_bases_change {
            DiffBasesChange::SetIndex(index) => {
                self.index_text = index.map(|mut index| {
                    text::LineEnding::normalize(&mut index);
                    Arc::new(index)
                });
                self.index_changed = true;
            }
            DiffBasesChange::SetHead(head) => {
                self.head_text = head.map(|mut head| {
                    text::LineEnding::normalize(&mut head);
                    Arc::new(head)
                });
                self.head_changed = true;
            }
            DiffBasesChange::SetBoth(text) => {
                let text = text.map(|mut text| {
                    text::LineEnding::normalize(&mut text);
                    Arc::new(text)
                });
                self.head_text = text.clone();
                self.index_text = text;
                self.head_changed = true;
                self.index_changed = true;
            }
            DiffBasesChange::SetEach { index, head } => {
                self.index_text = index.map(|mut index| {
                    text::LineEnding::normalize(&mut index);
                    Arc::new(index)
                });
                self.index_changed = true;
                self.head_text = head.map(|mut head| {
                    text::LineEnding::normalize(&mut head);
                    Arc::new(head)
                });
                self.head_changed = true;
            }
        }

        self.recalculate_diffs(buffer, cx)
    }

    fn recalculate_diffs(
        &mut self,
        buffer: text::BufferSnapshot,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<()> {
        log::debug!("recalculate diffs");
        let (tx, rx) = oneshot::channel();
        self.diff_updated_futures.push(tx);

        let language = self.language.clone();
        let language_registry = self.language_registry.clone();
        let unstaged_diff = self.unstaged_diff();
        let uncommitted_diff = self.uncommitted_diff();
        let head = self.head_text.clone();
        let index = self.index_text.clone();
        let index_changed = self.index_changed;
        let head_changed = self.head_changed;
        let language_changed = self.language_changed;
        let index_matches_head = match (self.index_text.as_ref(), self.head_text.as_ref()) {
            (Some(index), Some(head)) => Arc::ptr_eq(index, head),
            (None, None) => true,
            _ => false,
        };
        self.recalculate_diff_task = Some(cx.spawn(|this, mut cx| async move {
            let mut new_unstaged_diff = None;
            if let Some(unstaged_diff) = &unstaged_diff {
                new_unstaged_diff = Some(
                    BufferDiff::update_diff(
                        unstaged_diff.clone(),
                        buffer.clone(),
                        index,
                        index_changed,
                        language_changed,
                        language.clone(),
                        language_registry.clone(),
                        &mut cx,
                    )
                    .await?,
                );
            }

            let mut new_uncommitted_diff = None;
            if let Some(uncommitted_diff) = &uncommitted_diff {
                new_uncommitted_diff = if index_matches_head {
                    new_unstaged_diff.clone()
                } else {
                    Some(
                        BufferDiff::update_diff(
                            uncommitted_diff.clone(),
                            buffer.clone(),
                            head,
                            head_changed,
                            language_changed,
                            language.clone(),
                            language_registry.clone(),
                            &mut cx,
                        )
                        .await?,
                    )
                }
            }

            let unstaged_changed_range = if let Some((unstaged_diff, new_unstaged_diff)) =
                unstaged_diff.as_ref().zip(new_unstaged_diff.clone())
            {
                unstaged_diff.update(&mut cx, |diff, cx| {
                    diff.set_snapshot(&buffer, new_unstaged_diff, language_changed, None, cx)
                })?
            } else {
                None
            };

            if let Some((uncommitted_diff, new_uncommitted_diff)) =
                uncommitted_diff.as_ref().zip(new_uncommitted_diff.clone())
            {
                uncommitted_diff.update(&mut cx, |uncommitted_diff, cx| {
                    uncommitted_diff.set_snapshot(
                        &buffer,
                        new_uncommitted_diff,
                        language_changed,
                        unstaged_changed_range,
                        cx,
                    );
                })?;
            }

            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, _| {
                    this.index_changed = false;
                    this.head_changed = false;
                    this.language_changed = false;
                    for tx in this.diff_updated_futures.drain(..) {
                        tx.send(()).ok();
                    }
                })?;
            }

            Ok(())
        }));

        rx
    }
}

fn make_remote_delegate(
    this: Entity<GitStore>,
    project_id: u64,
    worktree_id: WorktreeId,
    work_directory_id: ProjectEntryId,
    askpass_id: u64,
    cx: &mut AsyncApp,
) -> AskPassDelegate {
    AskPassDelegate::new(cx, move |prompt, tx, cx| {
        this.update(cx, |this, cx| {
            let Some((client, _)) = this.downstream_client() else {
                return;
            };
            let response = client.request(proto::AskPassRequest {
                project_id,
                worktree_id: worktree_id.to_proto(),
                work_directory_id: work_directory_id.to_proto(),
                askpass_id,
                prompt,
            });
            cx.spawn(|_, _| async move {
                tx.send(response.await?.response).ok();
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        })
        .log_err();
    })
}

impl GitStoreState {
    fn load_staged_text(
        &self,
        buffer: &Entity<Buffer>,
        buffer_store: &Entity<BufferStore>,
        cx: &App,
    ) -> Task<Result<Option<String>>> {
        match self {
            GitStoreState::Local { .. } => {
                if let Some((worktree, path)) =
                    buffer_store.read(cx).worktree_for_buffer(buffer, cx)
                {
                    worktree.read(cx).load_staged_file(path.as_ref(), cx)
                } else {
                    return Task::ready(Err(anyhow!("no such worktree")));
                }
            }
            GitStoreState::Ssh {
                upstream_client,
                upstream_project_id: project_id,
                ..
            }
            | GitStoreState::Remote {
                upstream_client,
                project_id,
            } => {
                let buffer_id = buffer.read(cx).remote_id();
                let project_id = *project_id;
                let client = upstream_client.clone();
                cx.background_spawn(async move {
                    let response = client
                        .request(proto::OpenUnstagedDiff {
                            project_id: project_id.to_proto(),
                            buffer_id: buffer_id.to_proto(),
                        })
                        .await?;
                    Ok(response.staged_text)
                })
            }
        }
    }

    fn load_committed_text(
        &self,
        buffer: &Entity<Buffer>,
        buffer_store: &Entity<BufferStore>,
        cx: &App,
    ) -> Task<Result<DiffBasesChange>> {
        match self {
            GitStoreState::Local { .. } => {
                if let Some((worktree, path)) =
                    buffer_store.read(cx).worktree_for_buffer(buffer, cx)
                {
                    let worktree = worktree.read(cx);
                    let committed_text = worktree.load_committed_file(&path, cx);
                    let staged_text = worktree.load_staged_file(&path, cx);
                    cx.background_spawn(async move {
                        let committed_text = committed_text.await?;
                        let staged_text = staged_text.await?;
                        let diff_bases_change = if committed_text == staged_text {
                            DiffBasesChange::SetBoth(committed_text)
                        } else {
                            DiffBasesChange::SetEach {
                                index: staged_text,
                                head: committed_text,
                            }
                        };
                        Ok(diff_bases_change)
                    })
                } else {
                    Task::ready(Err(anyhow!("no such worktree")))
                }
            }
            GitStoreState::Ssh {
                upstream_client,
                upstream_project_id: project_id,
                ..
            }
            | GitStoreState::Remote {
                upstream_client,
                project_id,
            } => {
                use proto::open_uncommitted_diff_response::Mode;

                let buffer_id = buffer.read(cx).remote_id();
                let project_id = *project_id;
                let client = upstream_client.clone();
                cx.background_spawn(async move {
                    let response = client
                        .request(proto::OpenUncommittedDiff {
                            project_id: project_id.to_proto(),
                            buffer_id: buffer_id.to_proto(),
                        })
                        .await?;
                    let mode =
                        Mode::from_i32(response.mode).ok_or_else(|| anyhow!("Invalid mode"))?;
                    let bases = match mode {
                        Mode::IndexMatchesHead => DiffBasesChange::SetBoth(response.committed_text),
                        Mode::IndexAndHead => DiffBasesChange::SetEach {
                            head: response.committed_text,
                            index: response.staged_text,
                        },
                    };
                    Ok(bases)
                })
            }
        }
    }
}

impl Repository {
    pub fn git_store(&self) -> Option<Entity<GitStore>> {
        self.git_store.upgrade()
    }

    fn id(&self) -> (WorktreeId, ProjectEntryId) {
        (self.worktree_id, self.repository_entry.work_directory_id())
    }

    pub fn current_branch(&self) -> Option<&Branch> {
        self.repository_entry.branch()
    }

    fn send_job<F, Fut, R>(&self, job: F) -> oneshot::Receiver<R>
    where
        F: FnOnce(GitRepo, AsyncApp) -> Fut + 'static,
        Fut: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        self.send_keyed_job(None, job)
    }

    fn send_keyed_job<F, Fut, R>(&self, key: Option<GitJobKey>, job: F) -> oneshot::Receiver<R>
    where
        F: FnOnce(GitRepo, AsyncApp) -> Fut + 'static,
        Fut: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        let git_repo = self.git_repo.clone();
        self.job_sender
            .unbounded_send(GitJob {
                key,
                job: Box::new(|cx: &mut AsyncApp| {
                    let job = job(git_repo, cx.clone());
                    cx.spawn(|_| async move {
                        let result = job.await;
                        result_tx.send(result).ok();
                    })
                }),
            })
            .ok();
        result_rx
    }

    pub fn display_name(&self, project: &Project, cx: &App) -> SharedString {
        maybe!({
            let project_path = self.repo_path_to_project_path(&"".into())?;
            let worktree_name = project
                .worktree_for_id(project_path.worktree_id, cx)?
                .read(cx)
                .root_name();

            let mut path = PathBuf::new();
            path = path.join(worktree_name);
            if project_path.path.components().count() > 0 {
                path = path.join(project_path.path);
            }
            Some(path.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| self.repository_entry.work_directory.display_name())
        .into()
    }

    pub fn activate(&self, cx: &mut Context<Self>) {
        let Some(git_store) = self.git_store.upgrade() else {
            return;
        };
        let entity = cx.entity();
        git_store.update(cx, |git_store, cx| {
            let Some(index) = git_store
                .repositories
                .iter()
                .position(|handle| *handle == entity)
            else {
                return;
            };
            git_store.active_index = Some(index);
            cx.emit(GitEvent::ActiveRepositoryChanged);
        });
    }

    pub fn status(&self) -> impl '_ + Iterator<Item = StatusEntry> {
        self.repository_entry.status()
    }

    pub fn has_conflict(&self, path: &RepoPath) -> bool {
        self.repository_entry
            .current_merge_conflicts
            .contains(&path)
    }

    pub fn repo_path_to_project_path(&self, path: &RepoPath) -> Option<ProjectPath> {
        let path = self.repository_entry.try_unrelativize(path)?;
        Some((self.worktree_id, path).into())
    }

    pub fn project_path_to_repo_path(&self, path: &ProjectPath) -> Option<RepoPath> {
        self.worktree_id_path_to_repo_path(path.worktree_id, &path.path)
    }

    // note: callers must verify these come from the same worktree
    pub fn contains_sub_repo(&self, other: &Entity<Self>, cx: &App) -> bool {
        let other_work_dir = &other.read(cx).repository_entry.work_directory;
        match (&self.repository_entry.work_directory, other_work_dir) {
            (WorkDirectory::InProject { .. }, WorkDirectory::AboveProject { .. }) => false,
            (WorkDirectory::AboveProject { .. }, WorkDirectory::InProject { .. }) => true,
            (
                WorkDirectory::InProject {
                    relative_path: this_path,
                },
                WorkDirectory::InProject {
                    relative_path: other_path,
                },
            ) => other_path.starts_with(this_path),
            (
                WorkDirectory::AboveProject {
                    absolute_path: this_path,
                    ..
                },
                WorkDirectory::AboveProject {
                    absolute_path: other_path,
                    ..
                },
            ) => other_path.starts_with(this_path),
        }
    }

    pub fn worktree_id_path_to_repo_path(
        &self,
        worktree_id: WorktreeId,
        path: &Path,
    ) -> Option<RepoPath> {
        if worktree_id != self.worktree_id {
            return None;
        }
        self.repository_entry.relativize(path).log_err()
    }

    pub fn open_commit_buffer(
        &mut self,
        languages: Option<Arc<LanguageRegistry>>,
        buffer_store: Entity<BufferStore>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        if let Some(buffer) = self.commit_message_buffer.clone() {
            return Task::ready(Ok(buffer));
        }

        if let GitRepo::Remote {
            project_id,
            client,
            worktree_id,
            work_directory_id,
        } = self.git_repo.clone()
        {
            let client = client.clone();
            cx.spawn(|repository, mut cx| async move {
                let request = client.request(proto::OpenCommitMessageBuffer {
                    project_id: project_id.0,
                    worktree_id: worktree_id.to_proto(),
                    work_directory_id: work_directory_id.to_proto(),
                });
                let response = request.await.context("requesting to open commit buffer")?;
                let buffer_id = BufferId::new(response.buffer_id)?;
                let buffer = buffer_store
                    .update(&mut cx, |buffer_store, cx| {
                        buffer_store.wait_for_remote_buffer(buffer_id, cx)
                    })?
                    .await?;
                if let Some(language_registry) = languages {
                    let git_commit_language =
                        language_registry.language_for_name("Git Commit").await?;
                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.set_language(Some(git_commit_language), cx);
                    })?;
                }
                repository.update(&mut cx, |repository, _| {
                    repository.commit_message_buffer = Some(buffer.clone());
                })?;
                Ok(buffer)
            })
        } else {
            self.open_local_commit_buffer(languages, buffer_store, cx)
        }
    }

    fn open_local_commit_buffer(
        &mut self,
        language_registry: Option<Arc<LanguageRegistry>>,
        buffer_store: Entity<BufferStore>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        let merge_message = self.merge_message.clone();
        cx.spawn(|repository, mut cx| async move {
            let buffer = buffer_store
                .update(&mut cx, |buffer_store, cx| buffer_store.create_buffer(cx))?
                .await?;

            if let Some(language_registry) = language_registry {
                let git_commit_language = language_registry.language_for_name("Git Commit").await?;
                buffer.update(&mut cx, |buffer, cx| {
                    buffer.set_language(Some(git_commit_language), cx);
                })?;
            }

            if let Some(merge_message) = merge_message {
                buffer.update(&mut cx, |buffer, cx| {
                    buffer.set_text(merge_message.as_str(), cx)
                })?;
            }

            repository.update(&mut cx, |repository, _| {
                repository.commit_message_buffer = Some(buffer.clone());
            })?;
            Ok(buffer)
        })
    }

    pub fn checkout_files(
        &self,
        commit: &str,
        paths: Vec<RepoPath>,
        cx: &mut App,
    ) -> oneshot::Receiver<Result<()>> {
        let commit = commit.to_string();
        let env = self.worktree_environment(cx);

        self.send_job(|git_repo, _| async move {
            match git_repo {
                GitRepo::Local(repo) => repo.checkout_files(commit, paths, env.await).await,
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    client
                        .request(proto::GitCheckoutFiles {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            commit,
                            paths: paths
                                .into_iter()
                                .map(|p| p.to_string_lossy().to_string())
                                .collect(),
                        })
                        .await?;

                    Ok(())
                }
            }
        })
    }

    pub fn reset(
        &self,
        commit: String,
        reset_mode: ResetMode,
        cx: &mut App,
    ) -> oneshot::Receiver<Result<()>> {
        let commit = commit.to_string();
        let env = self.worktree_environment(cx);
        self.send_job(|git_repo, _| async move {
            match git_repo {
                GitRepo::Local(git_repo) => {
                    let env = env.await;
                    git_repo.reset(commit, reset_mode, env).await
                }
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    client
                        .request(proto::GitReset {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            commit,
                            mode: match reset_mode {
                                ResetMode::Soft => git_reset::ResetMode::Soft.into(),
                                ResetMode::Mixed => git_reset::ResetMode::Mixed.into(),
                            },
                        })
                        .await?;

                    Ok(())
                }
            }
        })
    }

    pub fn show(&self, commit: String) -> oneshot::Receiver<Result<CommitDetails>> {
        self.send_job(|git_repo, cx| async move {
            match git_repo {
                GitRepo::Local(git_repository) => git_repository.show(commit, cx).await,
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    let resp = client
                        .request(proto::GitShow {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            commit,
                        })
                        .await?;

                    Ok(CommitDetails {
                        sha: resp.sha.into(),
                        message: resp.message.into(),
                        commit_timestamp: resp.commit_timestamp,
                        committer_email: resp.committer_email.into(),
                        committer_name: resp.committer_name.into(),
                    })
                }
            }
        })
    }

    fn buffer_store(&self, cx: &App) -> Option<Entity<BufferStore>> {
        Some(self.git_store.upgrade()?.read(cx).buffer_store.clone())
    }

    pub fn stage_entries(
        &self,
        entries: Vec<RepoPath>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        if entries.is_empty() {
            return Task::ready(Ok(()));
        }
        let env = self.worktree_environment(cx);

        let mut save_futures = Vec::new();
        if let Some(buffer_store) = self.buffer_store(cx) {
            buffer_store.update(cx, |buffer_store, cx| {
                for path in &entries {
                    let Some(path) = self.repository_entry.try_unrelativize(path) else {
                        continue;
                    };
                    let project_path = (self.worktree_id, path).into();
                    if let Some(buffer) = buffer_store.get_by_path(&project_path, cx) {
                        if buffer
                            .read(cx)
                            .file()
                            .map_or(false, |file| file.disk_state().exists())
                        {
                            save_futures.push(buffer_store.save_buffer(buffer, cx));
                        }
                    }
                }
            })
        }

        cx.spawn(|this, mut cx| async move {
            for save_future in save_futures {
                save_future.await?;
            }
            let env = env.await;

            this.update(&mut cx, |this, _| {
                this.send_job(|git_repo, cx| async move {
                    match git_repo {
                        GitRepo::Local(repo) => repo.stage_paths(entries, env, cx).await,
                        GitRepo::Remote {
                            project_id,
                            client,
                            worktree_id,
                            work_directory_id,
                        } => {
                            client
                                .request(proto::Stage {
                                    project_id: project_id.0,
                                    worktree_id: worktree_id.to_proto(),
                                    work_directory_id: work_directory_id.to_proto(),
                                    paths: entries
                                        .into_iter()
                                        .map(|repo_path| repo_path.as_ref().to_proto())
                                        .collect(),
                                })
                                .await
                                .context("sending stage request")?;

                            Ok(())
                        }
                    }
                })
            })?
            .await??;

            Ok(())
        })
    }

    pub fn unstage_entries(
        &self,
        entries: Vec<RepoPath>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        if entries.is_empty() {
            return Task::ready(Ok(()));
        }
        let env = self.worktree_environment(cx);

        let mut save_futures = Vec::new();
        if let Some(buffer_store) = self.buffer_store(cx) {
            buffer_store.update(cx, |buffer_store, cx| {
                for path in &entries {
                    let Some(path) = self.repository_entry.try_unrelativize(path) else {
                        continue;
                    };
                    let project_path = (self.worktree_id, path).into();
                    if let Some(buffer) = buffer_store.get_by_path(&project_path, cx) {
                        if buffer
                            .read(cx)
                            .file()
                            .map_or(false, |file| file.disk_state().exists())
                        {
                            save_futures.push(buffer_store.save_buffer(buffer, cx));
                        }
                    }
                }
            })
        }

        cx.spawn(move |this, mut cx| async move {
            for save_future in save_futures {
                save_future.await?;
            }
            let env = env.await;

            this.update(&mut cx, |this, _| {
                this.send_job(|git_repo, cx| async move {
                    match git_repo {
                        GitRepo::Local(repo) => repo.unstage_paths(entries, env, cx).await,
                        GitRepo::Remote {
                            project_id,
                            client,
                            worktree_id,
                            work_directory_id,
                        } => {
                            client
                                .request(proto::Unstage {
                                    project_id: project_id.0,
                                    worktree_id: worktree_id.to_proto(),
                                    work_directory_id: work_directory_id.to_proto(),
                                    paths: entries
                                        .into_iter()
                                        .map(|repo_path| repo_path.as_ref().to_proto())
                                        .collect(),
                                })
                                .await
                                .context("sending unstage request")?;

                            Ok(())
                        }
                    }
                })
            })?
            .await??;

            Ok(())
        })
    }

    pub fn stage_all(&self, cx: &mut Context<Self>) -> Task<anyhow::Result<()>> {
        let to_stage = self
            .repository_entry
            .status()
            .filter(|entry| !entry.status.staging().is_fully_staged())
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.stage_entries(to_stage, cx)
    }

    pub fn unstage_all(&self, cx: &mut Context<Self>) -> Task<anyhow::Result<()>> {
        let to_unstage = self
            .repository_entry
            .status()
            .filter(|entry| entry.status.staging().has_staged())
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.unstage_entries(to_unstage, cx)
    }

    /// Get a count of all entries in the active repository, including
    /// untracked files.
    pub fn entry_count(&self) -> usize {
        self.repository_entry.status_len()
    }

    fn worktree_environment(
        &self,
        cx: &mut App,
    ) -> impl Future<Output = HashMap<String, String>> + 'static {
        let task = self.project_environment.as_ref().and_then(|env| {
            env.update(cx, |env, cx| {
                env.get_environment(
                    Some(self.worktree_id),
                    Some(self.worktree_abs_path.clone()),
                    cx,
                )
            })
            .ok()
        });
        async move { OptionFuture::from(task).await.flatten().unwrap_or_default() }
    }

    pub fn commit(
        &self,
        message: SharedString,
        name_and_email: Option<(SharedString, SharedString)>,
        cx: &mut App,
    ) -> oneshot::Receiver<Result<()>> {
        let env = self.worktree_environment(cx);
        self.send_job(|git_repo, cx| async move {
            match git_repo {
                GitRepo::Local(repo) => {
                    let env = env.await;
                    repo.commit(message, name_and_email, env, cx).await
                }
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    let (name, email) = name_and_email.unzip();
                    client
                        .request(proto::Commit {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            message: String::from(message),
                            name: name.map(String::from),
                            email: email.map(String::from),
                        })
                        .await
                        .context("sending commit request")?;

                    Ok(())
                }
            }
        })
    }

    pub fn fetch(
        &mut self,
        askpass: AskPassDelegate,
        cx: &mut App,
    ) -> oneshot::Receiver<Result<RemoteCommandOutput>> {
        let executor = cx.background_executor().clone();
        let askpass_delegates = self.askpass_delegates.clone();
        let askpass_id = util::post_inc(&mut self.latest_askpass_id);
        let env = self.worktree_environment(cx);

        self.send_job(move |git_repo, cx| async move {
            match git_repo {
                GitRepo::Local(git_repository) => {
                    let askpass = AskPassSession::new(&executor, askpass).await?;
                    let env = env.await;
                    git_repository.fetch(askpass, env, cx).await
                }
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    askpass_delegates.lock().insert(askpass_id, askpass);
                    let _defer = util::defer(|| {
                        let askpass_delegate = askpass_delegates.lock().remove(&askpass_id);
                        debug_assert!(askpass_delegate.is_some());
                    });

                    let response = client
                        .request(proto::Fetch {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            askpass_id,
                        })
                        .await
                        .context("sending fetch request")?;

                    Ok(RemoteCommandOutput {
                        stdout: response.stdout,
                        stderr: response.stderr,
                    })
                }
            }
        })
    }

    pub fn push(
        &mut self,
        branch: SharedString,
        remote: SharedString,
        options: Option<PushOptions>,
        askpass: AskPassDelegate,
        cx: &mut App,
    ) -> oneshot::Receiver<Result<RemoteCommandOutput>> {
        let executor = cx.background_executor().clone();
        let askpass_delegates = self.askpass_delegates.clone();
        let askpass_id = util::post_inc(&mut self.latest_askpass_id);
        let env = self.worktree_environment(cx);

        self.send_job(move |git_repo, cx| async move {
            match git_repo {
                GitRepo::Local(git_repository) => {
                    let env = env.await;
                    let askpass = AskPassSession::new(&executor, askpass).await?;
                    git_repository
                        .push(
                            branch.to_string(),
                            remote.to_string(),
                            options,
                            askpass,
                            env,
                            cx,
                        )
                        .await
                }
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    askpass_delegates.lock().insert(askpass_id, askpass);
                    let _defer = util::defer(|| {
                        let askpass_delegate = askpass_delegates.lock().remove(&askpass_id);
                        debug_assert!(askpass_delegate.is_some());
                    });
                    let response = client
                        .request(proto::Push {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            askpass_id,
                            branch_name: branch.to_string(),
                            remote_name: remote.to_string(),
                            options: options.map(|options| match options {
                                PushOptions::Force => proto::push::PushOptions::Force,
                                PushOptions::SetUpstream => proto::push::PushOptions::SetUpstream,
                            } as i32),
                        })
                        .await
                        .context("sending push request")?;

                    Ok(RemoteCommandOutput {
                        stdout: response.stdout,
                        stderr: response.stderr,
                    })
                }
            }
        })
    }

    pub fn pull(
        &mut self,
        branch: SharedString,
        remote: SharedString,
        askpass: AskPassDelegate,
        cx: &mut App,
    ) -> oneshot::Receiver<Result<RemoteCommandOutput>> {
        let executor = cx.background_executor().clone();
        let askpass_delegates = self.askpass_delegates.clone();
        let askpass_id = util::post_inc(&mut self.latest_askpass_id);
        let env = self.worktree_environment(cx);

        self.send_job(move |git_repo, cx| async move {
            match git_repo {
                GitRepo::Local(git_repository) => {
                    let askpass = AskPassSession::new(&executor, askpass).await?;
                    let env = env.await;
                    git_repository
                        .pull(branch.to_string(), remote.to_string(), askpass, env, cx)
                        .await
                }
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    askpass_delegates.lock().insert(askpass_id, askpass);
                    let _defer = util::defer(|| {
                        let askpass_delegate = askpass_delegates.lock().remove(&askpass_id);
                        debug_assert!(askpass_delegate.is_some());
                    });
                    let response = client
                        .request(proto::Pull {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            askpass_id,
                            branch_name: branch.to_string(),
                            remote_name: remote.to_string(),
                        })
                        .await
                        .context("sending pull request")?;

                    Ok(RemoteCommandOutput {
                        stdout: response.stdout,
                        stderr: response.stderr,
                    })
                }
            }
        })
    }

    fn set_index_text(
        &self,
        path: RepoPath,
        content: Option<String>,
        cx: &mut App,
    ) -> oneshot::Receiver<anyhow::Result<()>> {
        let env = self.worktree_environment(cx);

        self.send_keyed_job(
            Some(GitJobKey::WriteIndex(path.clone())),
            |git_repo, cx| async move {
                match git_repo {
                    GitRepo::Local(repo) => repo.set_index_text(path, content, env.await, cx).await,
                    GitRepo::Remote {
                        project_id,
                        client,
                        worktree_id,
                        work_directory_id,
                    } => {
                        client
                            .request(proto::SetIndexText {
                                project_id: project_id.0,
                                worktree_id: worktree_id.to_proto(),
                                work_directory_id: work_directory_id.to_proto(),
                                path: path.as_ref().to_proto(),
                                text: content,
                            })
                            .await?;
                        Ok(())
                    }
                }
            },
        )
    }

    pub fn get_remotes(
        &self,
        branch_name: Option<String>,
    ) -> oneshot::Receiver<Result<Vec<Remote>>> {
        self.send_job(|repo, cx| async move {
            match repo {
                GitRepo::Local(git_repository) => git_repository.get_remotes(branch_name, cx).await,
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    let response = client
                        .request(proto::GetRemotes {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            branch_name,
                        })
                        .await?;

                    let remotes = response
                        .remotes
                        .into_iter()
                        .map(|remotes| git::repository::Remote {
                            name: remotes.name.into(),
                        })
                        .collect();

                    Ok(remotes)
                }
            }
        })
    }

    pub fn branches(&self) -> oneshot::Receiver<Result<Vec<Branch>>> {
        self.send_job(|repo, cx| async move {
            match repo {
                GitRepo::Local(git_repository) => {
                    let git_repository = git_repository.clone();
                    cx.background_spawn(async move { git_repository.branches().await })
                        .await
                }
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    let response = client
                        .request(proto::GitGetBranches {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                        })
                        .await?;

                    let branches = response
                        .branches
                        .into_iter()
                        .map(|branch| worktree::proto_to_branch(&branch))
                        .collect();

                    Ok(branches)
                }
            }
        })
    }

    pub fn diff(&self, diff_type: DiffType, _cx: &App) -> oneshot::Receiver<Result<String>> {
        self.send_job(|repo, cx| async move {
            match repo {
                GitRepo::Local(git_repository) => git_repository.diff(diff_type, cx).await,
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                    ..
                } => {
                    let response = client
                        .request(proto::GitDiff {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            diff_type: match diff_type {
                                DiffType::HeadToIndex => {
                                    proto::git_diff::DiffType::HeadToIndex.into()
                                }
                                DiffType::HeadToWorktree => {
                                    proto::git_diff::DiffType::HeadToWorktree.into()
                                }
                            },
                        })
                        .await?;

                    Ok(response.diff)
                }
            }
        })
    }

    pub fn create_branch(&self, branch_name: String) -> oneshot::Receiver<Result<()>> {
        self.send_job(|repo, cx| async move {
            match repo {
                GitRepo::Local(git_repository) => {
                    git_repository.create_branch(branch_name, cx).await
                }
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    client
                        .request(proto::GitCreateBranch {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            branch_name,
                        })
                        .await?;

                    Ok(())
                }
            }
        })
    }

    pub fn change_branch(&self, branch_name: String) -> oneshot::Receiver<Result<()>> {
        self.send_job(|repo, cx| async move {
            match repo {
                GitRepo::Local(git_repository) => {
                    git_repository.change_branch(branch_name, cx).await
                }
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    client
                        .request(proto::GitChangeBranch {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                            branch_name,
                        })
                        .await?;

                    Ok(())
                }
            }
        })
    }

    pub fn check_for_pushed_commits(&self) -> oneshot::Receiver<Result<Vec<SharedString>>> {
        self.send_job(|repo, cx| async move {
            match repo {
                GitRepo::Local(git_repository) => git_repository.check_for_pushed_commit(cx).await,
                GitRepo::Remote {
                    project_id,
                    client,
                    worktree_id,
                    work_directory_id,
                } => {
                    let response = client
                        .request(proto::CheckForPushedCommits {
                            project_id: project_id.0,
                            worktree_id: worktree_id.to_proto(),
                            work_directory_id: work_directory_id.to_proto(),
                        })
                        .await?;

                    let branches = response.pushed_to.into_iter().map(Into::into).collect();

                    Ok(branches)
                }
            }
        })
    }
}
