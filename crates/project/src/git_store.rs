pub mod git_traversal;

use crate::{
    buffer_store::{BufferStore, BufferStoreEvent},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    ProjectEnvironment, ProjectItem, ProjectPath,
};
use anyhow::{anyhow, bail, Context as _, Result};
use askpass::{AskPassDelegate, AskPassSession};
use buffer_diff::{BufferDiff, BufferDiffEvent};
use client::ProjectId;
use collections::HashMap;
use fs::Fs;
use futures::{
    channel::{mpsc, oneshot},
    future::{self, OptionFuture, Shared},
    FutureExt as _, StreamExt as _,
};
use git::{
    blame::Blame,
    parse_git_remote_url,
    repository::{
        Branch, CommitDetails, DiffType, GitIndex, GitRepository, GitRepositoryCheckpoint,
        PushOptions, Remote, RemoteCommandOutput, RepoPath, ResetMode,
    },
    status::{FileStatus, GitStatus},
    BuildPermalinkParams, GitHostingProviderRegistry,
};
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Subscription, Task,
    WeakEntity,
};
use language::{
    proto::{deserialize_version, serialize_version},
    Buffer, BufferEvent, Language, LanguageRegistry,
};
use parking_lot::Mutex;
use rpc::{
    proto::{self, git_reset, FromProto, ToProto, SSH_PROJECT_ID},
    AnyProtoClient, TypedEnvelope,
};
use serde::Deserialize;
use settings::WorktreeId;
use std::{
    collections::{hash_map, VecDeque},
    future::Future,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use sum_tree::TreeSet;
use text::BufferId;
use util::{debug_panic, maybe, ResultExt};
use worktree::{
    proto_to_branch, File, PathKey, ProjectEntryId, RepositoryEntry, StatusEntry,
    UpdatedGitRepositoriesSet, Worktree,
};

pub struct GitStore {
    state: GitStoreState,
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    repositories: HashMap<ProjectEntryId, Entity<Repository>>,
    active_repo_id: Option<ProjectEntryId>,
    #[allow(clippy::type_complexity)]
    loading_diffs:
        HashMap<(BufferId, DiffKind), Shared<Task<Result<Entity<BufferDiff>, Arc<anyhow::Error>>>>>,
    diffs: HashMap<BufferId, Entity<BufferDiffState>>,
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
    hunk_staging_operation_count: usize,

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
        downstream_client: Option<LocalDownstreamState>,
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

enum DownstreamUpdate {
    UpdateRepository(RepositoryEntry),
    RemoveRepository(ProjectEntryId),
}

struct LocalDownstreamState {
    client: AnyProtoClient,
    project_id: ProjectId,
    updates_tx: mpsc::UnboundedSender<DownstreamUpdate>,
    _task: Task<Result<()>>,
}

#[derive(Clone)]
pub struct GitStoreCheckpoint {
    checkpoints_by_work_dir_abs_path: HashMap<PathBuf, GitRepositoryCheckpoint>,
}

#[derive(Clone, Debug)]
pub struct GitStoreDiff {
    diffs_by_work_dir_abs_path: HashMap<PathBuf, String>,
}

#[derive(Clone, Debug)]
pub struct GitStoreIndex {
    indices_by_work_dir_abs_path: HashMap<PathBuf, GitIndex>,
}

#[derive(Default)]
pub struct GitStoreStatus {
    #[allow(dead_code)]
    statuses_by_work_dir_abs_path: HashMap<PathBuf, GitStatus>,
}

pub struct Repository {
    pub repository_entry: RepositoryEntry,
    pub merge_message: Option<String>,
    pub completed_scan_id: usize,
    commit_message_buffer: Option<Entity<Buffer>>,
    git_store: WeakEntity<GitStore>,
    project_environment: Option<WeakEntity<ProjectEnvironment>>,
    pub worktree_id: Option<WorktreeId>,
    state: RepositoryState,
    job_sender: mpsc::UnboundedSender<GitJob>,
    askpass_delegates: Arc<Mutex<HashMap<u64, AskPassDelegate>>>,
    latest_askpass_id: u64,
}

#[derive(Clone)]
enum RepositoryState {
    Local(Arc<dyn GitRepository>),
    Remote {
        project_id: ProjectId,
        client: AnyProtoClient,
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
    BatchReadIndex(ProjectEntryId),
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
            worktree_store.clone(),
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
            worktree_store.clone(),
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
            worktree_store.clone(),
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
        worktree_store: Entity<WorktreeStore>,
        buffer_store: Entity<BufferStore>,
        state: GitStoreState,
        cx: &mut Context<Self>,
    ) -> Self {
        let update_sender = Self::spawn_git_worker(cx);
        let _subscriptions = [
            cx.subscribe(&worktree_store, Self::on_worktree_store_event),
            cx.subscribe(&buffer_store, Self::on_buffer_store_event),
        ];

        GitStore {
            state,
            buffer_store,
            worktree_store,
            repositories: HashMap::default(),
            active_repo_id: None,
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
        client.add_entity_request_handler(Self::handle_get_permalink_to_line);
        client.add_entity_request_handler(Self::handle_blame_buffer);
        client.add_entity_message_handler(Self::handle_update_repository);
        client.add_entity_message_handler(Self::handle_remove_repository);
    }

    pub fn is_local(&self) -> bool {
        matches!(self.state, GitStoreState::Local { .. })
    }

    pub fn shared(&mut self, project_id: u64, client: AnyProtoClient, cx: &mut Context<Self>) {
        match &mut self.state {
            GitStoreState::Ssh {
                downstream_client, ..
            } => {
                for repo in self.repositories.values() {
                    client
                        .send(repo.read(cx).repository_entry.initial_update(project_id))
                        .log_err();
                }
                *downstream_client = Some((client, ProjectId(project_id)));
            }
            GitStoreState::Local {
                downstream_client, ..
            } => {
                let mut snapshots = HashMap::default();
                let (updates_tx, mut updates_rx) = mpsc::unbounded();
                for repo in self.repositories.values() {
                    updates_tx
                        .unbounded_send(DownstreamUpdate::UpdateRepository(
                            repo.read(cx).repository_entry.clone(),
                        ))
                        .ok();
                }
                *downstream_client = Some(LocalDownstreamState {
                    client: client.clone(),
                    project_id: ProjectId(project_id),
                    updates_tx,
                    _task: cx.spawn(async move |this, cx| {
                        cx.background_spawn(async move {
                            while let Some(update) = updates_rx.next().await {
                                match update {
                                    DownstreamUpdate::UpdateRepository(snapshot) => {
                                        if let Some(old_snapshot) =
                                            snapshots.get_mut(&snapshot.work_directory_id)
                                        {
                                            let update =
                                                snapshot.build_update(old_snapshot, project_id);
                                            *old_snapshot = snapshot;
                                            client.send(update)?;
                                        } else {
                                            let update = snapshot.initial_update(project_id);
                                            client.send(update)?;
                                            snapshots.insert(snapshot.work_directory_id, snapshot);
                                        }
                                    }
                                    DownstreamUpdate::RemoveRepository(id) => {
                                        client.send(proto::RemoveRepository {
                                            project_id,
                                            id: id.to_proto(),
                                        })?;
                                    }
                                }
                            }
                            anyhow::Ok(())
                        })
                        .await
                        .ok();
                        this.update(cx, |this, _| {
                            if let GitStoreState::Local {
                                downstream_client, ..
                            } = &mut this.state
                            {
                                downstream_client.take();
                            } else {
                                unreachable!("unshared called on remote store");
                            }
                        })
                    }),
                });
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
            } => {
                downstream_client.take();
            }
            GitStoreState::Ssh {
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
        self.active_repo_id
            .as_ref()
            .map(|id| self.repositories[&id].clone())
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
                        cx.spawn(async move |this, cx| {
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
                        cx.spawn(async move |this, cx| {
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
        cx: &mut AsyncApp,
    ) -> Result<Entity<BufferDiff>> {
        let diff_bases_change = match texts {
            Err(e) => {
                this.update(cx, |this, cx| {
                    let buffer = buffer_entity.read(cx);
                    let buffer_id = buffer.remote_id();
                    this.loading_diffs.remove(&(buffer_id, kind));
                })?;
                return Err(e);
            }
            Ok(change) => change,
        };

        this.update(cx, |this, cx| {
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

                let rx = diff_state.diff_bases_changed(text_snapshot, diff_bases_change, 0, cx);

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

    pub fn project_path_git_status(
        &self,
        project_path: &ProjectPath,
        cx: &App,
    ) -> Option<FileStatus> {
        let (repo, repo_path) = self.repository_and_path_for_project_path(project_path, cx)?;
        Some(
            repo.read(cx)
                .repository_entry
                .status_for_path(&repo_path)?
                .status,
        )
    }

    pub fn checkpoint(&self, cx: &App) -> Task<Result<GitStoreCheckpoint>> {
        let mut work_directory_abs_paths = Vec::new();
        let mut checkpoints = Vec::new();
        for repository in self.repositories.values() {
            let repository = repository.read(cx);
            work_directory_abs_paths
                .push(repository.repository_entry.work_directory_abs_path.clone());
            checkpoints.push(repository.checkpoint().map(|checkpoint| checkpoint?));
        }

        cx.background_executor().spawn(async move {
            let checkpoints = future::try_join_all(checkpoints).await?;
            Ok(GitStoreCheckpoint {
                checkpoints_by_work_dir_abs_path: work_directory_abs_paths
                    .into_iter()
                    .zip(checkpoints)
                    .collect(),
            })
        })
    }

    pub fn restore_checkpoint(&self, checkpoint: GitStoreCheckpoint, cx: &App) -> Task<Result<()>> {
        let repositories_by_work_dir_abs_path = self
            .repositories
            .values()
            .map(|repo| {
                (
                    repo.read(cx)
                        .repository_entry
                        .work_directory_abs_path
                        .clone(),
                    repo,
                )
            })
            .collect::<HashMap<_, _>>();

        let mut tasks = Vec::new();
        for (work_dir_abs_path, checkpoint) in checkpoint.checkpoints_by_work_dir_abs_path {
            if let Some(repository) = repositories_by_work_dir_abs_path.get(&work_dir_abs_path) {
                let restore = repository.read(cx).restore_checkpoint(checkpoint);
                tasks.push(async move { restore.await? });
            }
        }
        cx.background_spawn(async move {
            future::try_join_all(tasks).await?;
            Ok(())
        })
    }

    /// Compares two checkpoints, returning true if they are equal.
    pub fn compare_checkpoints(
        &self,
        left: GitStoreCheckpoint,
        mut right: GitStoreCheckpoint,
        cx: &App,
    ) -> Task<Result<bool>> {
        let repositories_by_work_dir_abs_path = self
            .repositories
            .values()
            .map(|repo| {
                (
                    repo.read(cx)
                        .repository_entry
                        .work_directory_abs_path
                        .clone(),
                    repo,
                )
            })
            .collect::<HashMap<_, _>>();

        let mut tasks = Vec::new();
        for (work_dir_abs_path, left_checkpoint) in left.checkpoints_by_work_dir_abs_path {
            if let Some(right_checkpoint) = right
                .checkpoints_by_work_dir_abs_path
                .remove(&work_dir_abs_path)
            {
                if let Some(repository) = repositories_by_work_dir_abs_path.get(&work_dir_abs_path)
                {
                    let compare = repository
                        .read(cx)
                        .compare_checkpoints(left_checkpoint, right_checkpoint);
                    tasks.push(async move { compare.await? });
                }
            } else {
                return Task::ready(Ok(false));
            }
        }
        cx.background_spawn(async move {
            Ok(future::try_join_all(tasks)
                .await?
                .into_iter()
                .all(|result| result))
        })
    }

    pub fn delete_checkpoint(&self, checkpoint: GitStoreCheckpoint, cx: &App) -> Task<Result<()>> {
        let repositories_by_work_directory_abs_path = self
            .repositories
            .values()
            .map(|repo| {
                (
                    repo.read(cx)
                        .repository_entry
                        .work_directory_abs_path
                        .clone(),
                    repo,
                )
            })
            .collect::<HashMap<_, _>>();

        let mut tasks = Vec::new();
        for (work_dir_abs_path, checkpoint) in checkpoint.checkpoints_by_work_dir_abs_path {
            if let Some(repository) =
                repositories_by_work_directory_abs_path.get(&work_dir_abs_path)
            {
                let delete = repository.read(cx).delete_checkpoint(checkpoint);
                tasks.push(async move { delete.await? });
            }
        }
        cx.background_spawn(async move {
            future::try_join_all(tasks).await?;
            Ok(())
        })
    }

    pub fn diff_checkpoints(
        &self,
        base_checkpoint: GitStoreCheckpoint,
        target_checkpoint: GitStoreCheckpoint,
        cx: &App,
    ) -> Task<Result<GitStoreDiff>> {
        let repositories_by_work_dir_abs_path = self
            .repositories
            .values()
            .map(|repo| {
                (
                    repo.read(cx)
                        .repository_entry
                        .work_directory_abs_path
                        .clone(),
                    repo,
                )
            })
            .collect::<HashMap<_, _>>();

        let mut tasks = Vec::new();
        for (work_dir_abs_path, base_checkpoint) in base_checkpoint.checkpoints_by_work_dir_abs_path
        {
            if let Some(target_checkpoint) = target_checkpoint
                .checkpoints_by_work_dir_abs_path
                .get(&work_dir_abs_path)
                .cloned()
            {
                if let Some(repository) = repositories_by_work_dir_abs_path.get(&work_dir_abs_path)
                {
                    let diff = repository
                        .read(cx)
                        .diff_checkpoints(base_checkpoint, target_checkpoint);
                    tasks.push(async move {
                        let diff = diff.await??;
                        anyhow::Ok((work_dir_abs_path, diff))
                    });
                }
            }
        }

        cx.background_spawn(async move {
            let diffs_by_path = future::try_join_all(tasks).await?;
            Ok(GitStoreDiff {
                diffs_by_work_dir_abs_path: diffs_by_path.into_iter().collect(),
            })
        })
    }

    pub fn create_index(&self, cx: &App) -> Task<Result<GitStoreIndex>> {
        let mut indices = Vec::new();
        for repository in self.repositories.values() {
            let repository = repository.read(cx);
            let work_dir_abs_path = repository.repository_entry.work_directory_abs_path.clone();
            let index = repository.create_index().map(|index| index?);
            indices.push(async move {
                let index = index.await?;
                anyhow::Ok((work_dir_abs_path, index))
            });
        }

        cx.background_executor().spawn(async move {
            let indices = future::try_join_all(indices).await?;
            Ok(GitStoreIndex {
                indices_by_work_dir_abs_path: indices.into_iter().collect(),
            })
        })
    }

    pub fn apply_diff(
        &self,
        mut index: GitStoreIndex,
        diff: GitStoreDiff,
        cx: &App,
    ) -> Task<Result<()>> {
        let repositories_by_work_dir_abs_path = self
            .repositories
            .values()
            .map(|repo| {
                (
                    repo.read(cx)
                        .repository_entry
                        .work_directory_abs_path
                        .clone(),
                    repo,
                )
            })
            .collect::<HashMap<_, _>>();

        let mut tasks = Vec::new();
        for (work_dir_abs_path, diff) in diff.diffs_by_work_dir_abs_path {
            if let Some(repository) = repositories_by_work_dir_abs_path.get(&work_dir_abs_path) {
                if let Some(branch) = index
                    .indices_by_work_dir_abs_path
                    .remove(&work_dir_abs_path)
                {
                    let apply = repository.read(cx).apply_diff(branch, diff);
                    tasks.push(async move { apply.await? });
                }
            }
        }
        cx.background_spawn(async move {
            future::try_join_all(tasks).await?;
            Ok(())
        })
    }

    /// Blames a buffer.
    pub fn blame_buffer(
        &self,
        buffer: &Entity<Buffer>,
        version: Option<clock::Global>,
        cx: &App,
    ) -> Task<Result<Option<Blame>>> {
        let buffer = buffer.read(cx);
        let Some(file) = File::from_dyn(buffer.file()) else {
            return Task::ready(Err(anyhow!("buffer has no file")));
        };

        match file.worktree.clone().read(cx) {
            Worktree::Local(worktree) => {
                let worktree = worktree.snapshot();
                let blame_params = maybe!({
                    let local_repo = match worktree.local_repo_containing_path(&file.path) {
                        Some(repo_for_path) => repo_for_path,
                        None => return Ok(None),
                    };

                    let relative_path = local_repo
                        .relativize(&file.path)
                        .context("failed to relativize buffer path")?;

                    let repo = local_repo.repo().clone();

                    let content = match version {
                        Some(version) => buffer.rope_for_version(&version).clone(),
                        None => buffer.as_rope().clone(),
                    };

                    anyhow::Ok(Some((repo, relative_path, content)))
                });

                cx.spawn(async move |_cx| {
                    let Some((repo, relative_path, content)) = blame_params? else {
                        return Ok(None);
                    };
                    repo.blame(relative_path.clone(), content)
                        .await
                        .with_context(|| format!("Failed to blame {:?}", relative_path.0))
                        .map(Some)
                })
            }
            Worktree::Remote(worktree) => {
                let buffer_id = buffer.remote_id();
                let version = buffer.version();
                let project_id = worktree.project_id();
                let client = worktree.client();
                cx.spawn(async move |_| {
                    let response = client
                        .request(proto::BlameBuffer {
                            project_id,
                            buffer_id: buffer_id.into(),
                            version: serialize_version(&version),
                        })
                        .await?;
                    Ok(deserialize_blame_buffer_response(response))
                })
            }
        }
    }

    pub fn get_permalink_to_line(
        &self,
        buffer: &Entity<Buffer>,
        selection: Range<u32>,
        cx: &App,
    ) -> Task<Result<url::Url>> {
        let buffer = buffer.read(cx);
        let Some(file) = File::from_dyn(buffer.file()) else {
            return Task::ready(Err(anyhow!("buffer has no file")));
        };

        match file.worktree.read(cx) {
            Worktree::Local(worktree) => {
                let repository = self
                    .repository_and_path_for_project_path(
                        &(worktree.id(), file.path.clone()).into(),
                        cx,
                    )
                    .map(|(repository, _)| repository);
                let Some((local_repo_entry, repo_entry)) = repository.and_then(|repository| {
                    let repository = repository.read(cx);
                    let repo_entry = repository.repository_entry.clone();
                    Some((worktree.get_local_repo(&repo_entry)?, repo_entry))
                }) else {
                    // If we're not in a Git repo, check whether this is a Rust source
                    // file in the Cargo registry (presumably opened with go-to-definition
                    // from a normal Rust file). If so, we can put together a permalink
                    // using crate metadata.
                    if buffer
                        .language()
                        .is_none_or(|lang| lang.name() != "Rust".into())
                    {
                        return Task::ready(Err(anyhow!("no permalink available")));
                    }
                    let Some(file_path) = worktree.absolutize(&file.path).ok() else {
                        return Task::ready(Err(anyhow!("no permalink available")));
                    };
                    return cx.spawn(async move |cx| {
                        let provider_registry =
                            cx.update(GitHostingProviderRegistry::default_global)?;
                        get_permalink_in_rust_registry_src(provider_registry, file_path, selection)
                            .map_err(|_| anyhow!("no permalink available"))
                    });
                };

                let path = match local_repo_entry.relativize(&file.path) {
                    Ok(RepoPath(path)) => path,
                    Err(e) => return Task::ready(Err(e)),
                };

                let remote = repo_entry
                    .branch()
                    .and_then(|b| b.upstream.as_ref())
                    .and_then(|b| b.remote_name())
                    .unwrap_or("origin")
                    .to_string();

                let repo = local_repo_entry.repo().clone();
                cx.spawn(async move |cx| {
                    let origin_url = repo
                        .remote_url(&remote)
                        .ok_or_else(|| anyhow!("remote \"{remote}\" not found"))?;

                    let sha = repo
                        .head_sha()
                        .ok_or_else(|| anyhow!("failed to read HEAD SHA"))?;

                    let provider_registry =
                        cx.update(GitHostingProviderRegistry::default_global)?;

                    let (provider, remote) =
                        parse_git_remote_url(provider_registry, &origin_url)
                            .ok_or_else(|| anyhow!("failed to parse Git remote URL"))?;

                    let path = path
                        .to_str()
                        .ok_or_else(|| anyhow!("failed to convert path to string"))?;

                    Ok(provider.build_permalink(
                        remote,
                        BuildPermalinkParams {
                            sha: &sha,
                            path,
                            selection: Some(selection),
                        },
                    ))
                })
            }
            Worktree::Remote(worktree) => {
                let buffer_id = buffer.remote_id();
                let project_id = worktree.project_id();
                let client = worktree.client();
                cx.spawn(async move |_| {
                    let response = client
                        .request(proto::GetPermalinkToLine {
                            project_id,
                            buffer_id: buffer_id.into(),
                            selection: Some(proto::Range {
                                start: selection.start as u64,
                                end: selection.end as u64,
                            }),
                        })
                        .await?;

                    url::Url::parse(&response.permalink).context("failed to parse permalink")
                })
            }
        }
    }

    fn downstream_client(&self) -> Option<(AnyProtoClient, ProjectId)> {
        match &self.state {
            GitStoreState::Local {
                downstream_client, ..
            } => downstream_client
                .as_ref()
                .map(|state| (state.client.clone(), state.project_id)),
            GitStoreState::Ssh {
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
        match event {
            WorktreeStoreEvent::WorktreeUpdatedGitRepositories(worktree_id, changed_repos) => {
                // We should only get this event for a local project.
                self.update_repositories(&worktree_store, cx);
                if self.is_local() {
                    if let Some(worktree) =
                        worktree_store.read(cx).worktree_for_id(*worktree_id, cx)
                    {
                        self.local_worktree_git_repos_changed(worktree, changed_repos, cx);
                    }
                }
                cx.emit(GitEvent::GitStateUpdated);
            }
            WorktreeStoreEvent::WorktreeAdded(_) => {}
            _ => {
                cx.emit(GitEvent::FileSystemUpdated);
            }
        }
    }

    fn update_repositories(
        &mut self,
        worktree_store: &Entity<WorktreeStore>,
        cx: &mut Context<'_, GitStore>,
    ) {
        let mut new_repositories = HashMap::default();
        let git_store = cx.weak_entity();
        worktree_store.update(cx, |worktree_store, cx| {
            for worktree in worktree_store.worktrees() {
                worktree.update(cx, |worktree, cx| {
                    let snapshot = worktree.snapshot();
                    for repo_entry in snapshot.repositories().iter() {
                        let git_repo_and_merge_message = worktree
                            .as_local()
                            .and_then(|local_worktree| local_worktree.get_local_repo(repo_entry))
                            .map(|local_repo| {
                                (
                                    RepositoryState::Local(local_repo.repo().clone()),
                                    local_repo.merge_message.clone(),
                                )
                            })
                            .or_else(|| {
                                let git_repo = RepositoryState::Remote {
                                    project_id: self.project_id()?,
                                    client: self
                                        .upstream_client()
                                        .context("no upstream client")
                                        .log_err()?
                                        .clone(),
                                    work_directory_id: repo_entry.work_directory_id(),
                                };
                                Some((git_repo, None))
                            });

                        let Some((git_repo, merge_message)) = git_repo_and_merge_message else {
                            continue;
                        };

                        let existing_repo = self
                            .repositories
                            .values()
                            .find(|repo| repo.read(cx).id() == repo_entry.work_directory_id());

                        let repo = if let Some(existing_repo) = existing_repo {
                            // Update the statuses and merge message but keep everything else.
                            let existing_repo = existing_repo.clone();
                            existing_repo.update(cx, |existing_repo, _| {
                                existing_repo.repository_entry = repo_entry.clone();
                                if matches!(git_repo, RepositoryState::Local { .. }) {
                                    existing_repo.merge_message = merge_message;
                                    existing_repo.completed_scan_id = worktree.completed_scan_id();
                                }
                            });
                            existing_repo
                        } else {
                            cx.new(|_| Repository {
                                worktree_id: Some(worktree.id()),
                                project_environment: self
                                    .project_environment()
                                    .as_ref()
                                    .map(|env| env.downgrade()),
                                git_store: git_store.clone(),
                                askpass_delegates: Default::default(),
                                latest_askpass_id: 0,
                                repository_entry: repo_entry.clone(),
                                job_sender: self.update_sender.clone(),
                                merge_message,
                                commit_message_buffer: None,
                                completed_scan_id: worktree.completed_scan_id(),
                                state: git_repo,
                            })
                        };

                        // TODO only send out messages for repository snapshots that have changed
                        let snapshot = repo.read(cx).repository_entry.clone();
                        if let GitStoreState::Local {
                            downstream_client: Some(state),
                            ..
                        } = &self.state
                        {
                            state
                                .updates_tx
                                .unbounded_send(DownstreamUpdate::UpdateRepository(snapshot))
                                .ok();
                        }
                        new_repositories.insert(repo_entry.work_directory_id(), repo);
                        self.repositories.remove(&repo_entry.work_directory_id());
                    }
                })
            }
        });

        if let GitStoreState::Local {
            downstream_client: Some(state),
            ..
        } = &self.state
        {
            for id in self.repositories.keys().cloned() {
                state
                    .updates_tx
                    .unbounded_send(DownstreamUpdate::RemoveRepository(id))
                    .ok();
            }
        }

        self.repositories = new_repositories;
        if let Some(id) = self.active_repo_id.as_ref() {
            if !self.repositories.contains_key(id) {
                self.active_repo_id = None;
            }
        } else if let Some(&first_id) = self.repositories.keys().next() {
            self.active_repo_id = Some(first_id);
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
                    diff_state.recalculate_diffs(
                        buffer,
                        diff_state.hunk_staging_operation_count,
                        cx,
                    )
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
            if let Some(diff_state) = self.diffs.get(&buffer_id) {
                diff_state.update(cx, |diff_state, _| {
                    diff_state.hunk_staging_operation_count += 1;
                });
            }
            if let Some((repo, path)) = self.repository_and_path_for_buffer_id(buffer_id, cx) {
                let recv = repo.update(cx, |repo, cx| {
                    log::debug!("updating index text for buffer {}", path.display());
                    repo.spawn_set_index_text_job(
                        path,
                        new_index_text.as_ref().map(|rope| rope.to_string()),
                        cx,
                    )
                });
                let diff = diff.downgrade();
                cx.spawn(async move |this, cx| {
                    if let Ok(Err(error)) = cx.background_spawn(recv).await {
                        diff.update(cx, |diff, cx| {
                            diff.clear_pending_hunks(cx);
                        })
                        .ok();
                        this.update(cx, |_, cx| cx.emit(GitEvent::IndexWriteError(error)))
                            .ok();
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

        let Some(active_repo) = self.active_repository() else {
            log::error!("local worktree changed but we have no active repository");
            return;
        };

        let mut diff_state_updates = HashMap::<ProjectEntryId, Vec<_>>::default();
        for (buffer_id, diff_state) in &self.diffs {
            let Some(buffer) = self.buffer_store.read(cx).get(*buffer_id) else {
                continue;
            };
            let Some(file) = File::from_dyn(buffer.read(cx).file()) else {
                continue;
            };
            if file.worktree != worktree {
                continue;
            }
            let Some(repo_id) = changed_repos
                .iter()
                .map(|(entry, _)| entry.id)
                .find(|repo_id| self.repositories().contains_key(&repo_id))
            else {
                continue;
            };

            let diff_state = diff_state.read(cx);
            let has_unstaged_diff = diff_state
                .unstaged_diff
                .as_ref()
                .is_some_and(|diff| diff.is_upgradable());
            let has_uncommitted_diff = diff_state
                .uncommitted_diff
                .as_ref()
                .is_some_and(|set| set.is_upgradable());

            let update = (
                buffer,
                file.path.clone(),
                has_unstaged_diff.then(|| diff_state.index_text.clone()),
                has_uncommitted_diff.then(|| diff_state.head_text.clone()),
                diff_state.hunk_staging_operation_count,
            );
            diff_state_updates.entry(repo_id).or_default().push(update);
        }

        if diff_state_updates.is_empty() {
            return;
        }

        for (repo_id, repo_diff_state_updates) in diff_state_updates.into_iter() {
            let worktree = worktree.downgrade();
            let git_store = cx.weak_entity();

            let _ = active_repo.read(cx).send_keyed_job(
                Some(GitJobKey::BatchReadIndex(repo_id)),
                |_, mut cx| async move {
                    let snapshot = worktree.update(&mut cx, |tree, _| {
                        tree.as_local().map(|local_tree| local_tree.snapshot())
                    });
                    let Ok(Some(snapshot)) = snapshot else {
                        return;
                    };

                    let mut diff_bases_changes_by_buffer = Vec::new();
                    for (
                        buffer,
                        path,
                        current_index_text,
                        current_head_text,
                        hunk_staging_operation_count,
                    ) in &repo_diff_state_updates
                    {
                        let Some(local_repo) = snapshot.local_repo_containing_path(&path) else {
                            continue;
                        };
                        let Some(relative_path) = local_repo.relativize(&path).ok() else {
                            continue;
                        };

                        log::debug!("reloading git state for buffer {}", path.display());
                        let index_text = if current_index_text.is_some() {
                            local_repo
                                .repo()
                                .load_index_text(None, relative_path.clone())
                                .await
                        } else {
                            None
                        };
                        let head_text = if current_head_text.is_some() {
                            local_repo.repo().load_committed_text(relative_path).await
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

                        diff_bases_changes_by_buffer.push((
                            buffer,
                            diff_bases_change,
                            *hunk_staging_operation_count,
                        ))
                    }

                    git_store
                        .update(&mut cx, |git_store, cx| {
                            for (buffer, diff_bases_change, hunk_staging_operation_count) in
                                diff_bases_changes_by_buffer
                            {
                                let Some(diff_state) =
                                    git_store.diffs.get(&buffer.read(cx).remote_id())
                                else {
                                    continue;
                                };
                                let Some(diff_bases_change) = diff_bases_change else {
                                    continue;
                                };

                                let downstream_client = git_store.downstream_client();
                                diff_state.update(cx, |diff_state, cx| {
                                    use proto::update_diff_bases::Mode;

                                    let buffer = buffer.read(cx);
                                    if let Some((client, project_id)) = downstream_client {
                                        let (staged_text, committed_text, mode) =
                                            match diff_bases_change.clone() {
                                                DiffBasesChange::SetIndex(index) => {
                                                    (index, None, Mode::IndexOnly)
                                                }
                                                DiffBasesChange::SetHead(head) => {
                                                    (None, head, Mode::HeadOnly)
                                                }
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
                                        hunk_staging_operation_count,
                                        cx,
                                    );
                                });
                            }
                        })
                        .ok();
                },
            );
        }
    }

    pub fn repositories(&self) -> &HashMap<ProjectEntryId, Entity<Repository>> {
        &self.repositories
    }

    pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus> {
        let (repo, path) = self.repository_and_path_for_buffer_id(buffer_id, cx)?;
        let status = repo.read(cx).repository_entry.status_for_path(&path)?;
        Some(status.status)
    }

    pub fn status(&self, index: Option<GitStoreIndex>, cx: &App) -> Task<Result<GitStoreStatus>> {
        let repositories_by_work_dir_abs_path = self
            .repositories
            .values()
            .map(|repo| {
                (
                    repo.read(cx)
                        .repository_entry
                        .work_directory_abs_path
                        .clone(),
                    repo,
                )
            })
            .collect::<HashMap<_, _>>();

        let mut tasks = Vec::new();

        if let Some(index) = index {
            // When we have an index, just check the repositories that are part of it
            for (work_dir_abs_path, git_index) in index.indices_by_work_dir_abs_path {
                if let Some(repository) = repositories_by_work_dir_abs_path.get(&work_dir_abs_path)
                {
                    let status = repository.read(cx).status(Some(git_index));
                    tasks.push(
                        async move {
                            let status = status.await??;
                            anyhow::Ok((work_dir_abs_path, status))
                        }
                        .boxed(),
                    );
                }
            }
        } else {
            // Otherwise, check all repositories
            for repository in self.repositories.values() {
                let repository = repository.read(cx);
                let work_dir_abs_path = repository.repository_entry.work_directory_abs_path.clone();
                let status = repository.status(None);
                tasks.push(
                    async move {
                        let status = status.await??;
                        anyhow::Ok((work_dir_abs_path, status))
                    }
                    .boxed(),
                );
            }
        }

        cx.background_executor().spawn(async move {
            let statuses = future::try_join_all(tasks).await?;
            Ok(GitStoreStatus {
                statuses_by_work_dir_abs_path: statuses.into_iter().collect(),
            })
        })
    }

    pub fn load_index_text(
        &self,
        index: Option<GitStoreIndex>,
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Task<Option<String>> {
        let Some((repository, path)) =
            self.repository_and_path_for_buffer_id(buffer.read(cx).remote_id(), cx)
        else {
            return Task::ready(None);
        };

        let git_index = index.and_then(|index| {
            index
                .indices_by_work_dir_abs_path
                .get(&repository.read(cx).repository_entry.work_directory_abs_path)
                .copied()
        });
        let text = repository.read(cx).load_index_text(git_index, path);
        cx.background_spawn(async move {
            let text = text.await;
            text.ok().flatten()
        })
    }

    pub fn repository_and_path_for_buffer_id(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Option<(Entity<Repository>, RepoPath)> {
        let buffer = self.buffer_store.read(cx).get(buffer_id)?;
        let project_path = buffer.read(cx).project_path(cx)?;
        self.repository_and_path_for_project_path(&project_path, cx)
    }

    pub fn repository_and_path_for_project_path(
        &self,
        path: &ProjectPath,
        cx: &App,
    ) -> Option<(Entity<Repository>, RepoPath)> {
        let abs_path = self.worktree_store.read(cx).absolutize(path, cx)?;
        self.repositories
            .values()
            .filter_map(|repo_handle| {
                let repo = repo_handle.read(cx);
                let relative_path = repo.repository_entry.relativize_abs_path(&abs_path)?;
                Some((repo_handle.clone(), relative_path))
            })
            .max_by_key(|(repo, _)| {
                repo.read(cx)
                    .repository_entry
                    .work_directory_abs_path
                    .clone()
            })
    }

    fn spawn_git_worker(cx: &mut Context<GitStore>) -> mpsc::UnboundedSender<GitJob> {
        let (job_tx, mut job_rx) = mpsc::unbounded::<GitJob>();

        cx.spawn(async move |_, cx| {
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
                    (job.job)(cx).await;
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

    async fn handle_update_repository(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateRepository>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let mut update = envelope.payload;

            let work_directory_id = ProjectEntryId::from_proto(update.id);
            let client = this
                .upstream_client()
                .context("no upstream client")?
                .clone();

            let repo = this
                .repositories
                .entry(work_directory_id)
                .or_insert_with(|| {
                    let git_store = cx.weak_entity();

                    cx.new(|_| Repository {
                        commit_message_buffer: None,
                        git_store,
                        project_environment: None,
                        worktree_id: None,
                        repository_entry: RepositoryEntry {
                            work_directory_id,
                            current_branch: None,
                            statuses_by_path: Default::default(),
                            current_merge_conflicts: Default::default(),
                            work_directory_abs_path: update.abs_path.clone().into(),
                            worktree_scan_id: update.scan_id as usize,
                        },
                        merge_message: None,
                        completed_scan_id: update.scan_id as usize,
                        state: RepositoryState::Remote {
                            project_id: ProjectId(update.project_id),
                            client,
                            work_directory_id,
                        },
                        job_sender: this.update_sender.clone(),
                        askpass_delegates: Default::default(),
                        latest_askpass_id: 0,
                    })
                });

            repo.update(cx, |repo, _cx| repo.apply_remote_update(update.clone()))?;
            cx.emit(GitEvent::GitStateUpdated);
            this.active_repo_id.get_or_insert_with(|| {
                cx.emit(GitEvent::ActiveRepositoryChanged);
                work_directory_id
            });

            if let Some((client, project_id)) = this.downstream_client() {
                update.project_id = project_id.to_proto();
                client.send(update).log_err();
            }
            Ok(())
        })?
    }

    async fn handle_remove_repository(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RemoveRepository>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let mut update = envelope.payload;
            let id = ProjectEntryId::from_proto(update.id);
            this.repositories.remove(&id);
            if let Some((client, project_id)) = this.downstream_client() {
                update.project_id = project_id.to_proto();
                client.send(update).log_err();
            }
            if this.active_repo_id == Some(id) {
                this.active_repo_id = None;
                cx.emit(GitEvent::ActiveRepositoryChanged);
            }
            cx.emit(GitEvent::GitStateUpdated);
        })
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;
        let askpass_id = envelope.payload.askpass_id;

        let askpass = make_remote_delegate(
            this,
            envelope.payload.project_id,
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

        let askpass_id = envelope.payload.askpass_id;
        let askpass = make_remote_delegate(
            this,
            envelope.payload.project_id,
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;
        let askpass_id = envelope.payload.askpass_id;
        let askpass = make_remote_delegate(
            this,
            envelope.payload.project_id,
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.spawn_set_index_text_job(
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository = Self::repository_for_request(&this, work_directory_id, &mut cx)?;
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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;

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
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle = Self::repository_for_request(&this, work_directory_id, &mut cx)?;
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

    async fn handle_open_unstaged_diff(
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

    async fn handle_open_uncommitted_diff(
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

    async fn handle_update_diff_bases(
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

    async fn handle_blame_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::BlameBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::BlameBufferResponse> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let version = deserialize_version(&envelope.payload.version);
        let buffer = this.read_with(&cx, |this, cx| {
            this.buffer_store.read(cx).get_existing(buffer_id)
        })??;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(version.clone())
            })?
            .await?;
        let blame = this
            .update(&mut cx, |this, cx| {
                this.blame_buffer(&buffer, Some(version), cx)
            })?
            .await?;
        Ok(serialize_blame_buffer_response(blame))
    }

    async fn handle_get_permalink_to_line(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetPermalinkToLine>,
        mut cx: AsyncApp,
    ) -> Result<proto::GetPermalinkToLineResponse> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        // let version = deserialize_version(&envelope.payload.version);
        let selection = {
            let proto_selection = envelope
                .payload
                .selection
                .context("no selection to get permalink for defined")?;
            proto_selection.start as u32..proto_selection.end as u32
        };
        let buffer = this.read_with(&cx, |this, cx| {
            this.buffer_store.read(cx).get_existing(buffer_id)
        })??;
        let permalink = this
            .update(&mut cx, |this, cx| {
                this.get_permalink_to_line(&buffer, selection, cx)
            })?
            .await?;
        Ok(proto::GetPermalinkToLineResponse {
            permalink: permalink.to_string(),
        })
    }

    fn repository_for_request(
        this: &Entity<Self>,
        work_directory_id: ProjectEntryId,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Repository>> {
        this.update(cx, |this, cx| {
            this.repositories
                .values()
                .find(|repository_handle| {
                    repository_handle
                        .read(cx)
                        .repository_entry
                        .work_directory_id()
                        == work_directory_id
                })
                .context("missing repository handle")
                .cloned()
        })?
    }

    pub fn repo_snapshots(&self, cx: &App) -> HashMap<ProjectEntryId, RepositoryEntry> {
        self.repositories
            .iter()
            .map(|(id, repo)| (*id, repo.read(cx).repository_entry.clone()))
            .collect()
    }
}

impl BufferDiffState {
    fn buffer_language_changed(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.language = buffer.read(cx).language().cloned();
        self.language_changed = true;
        let _ = self.recalculate_diffs(
            buffer.read(cx).text_snapshot(),
            self.hunk_staging_operation_count,
            cx,
        );
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

        let _ = self.diff_bases_changed(
            buffer,
            diff_bases_change,
            self.hunk_staging_operation_count,
            cx,
        );
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
        prev_hunk_staging_operation_count: usize,
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

        self.recalculate_diffs(buffer, prev_hunk_staging_operation_count, cx)
    }

    fn recalculate_diffs(
        &mut self,
        buffer: text::BufferSnapshot,
        prev_hunk_staging_operation_count: usize,
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
        self.recalculate_diff_task = Some(cx.spawn(async move |this, cx| {
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
                        cx,
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
                            cx,
                        )
                        .await?,
                    )
                }
            }

            if this.update(cx, |this, _| {
                this.hunk_staging_operation_count > prev_hunk_staging_operation_count
            })? {
                return Ok(());
            }

            let unstaged_changed_range = if let Some((unstaged_diff, new_unstaged_diff)) =
                unstaged_diff.as_ref().zip(new_unstaged_diff.clone())
            {
                unstaged_diff.update(cx, |diff, cx| {
                    diff.set_snapshot(&buffer, new_unstaged_diff, language_changed, None, cx)
                })?
            } else {
                None
            };

            if let Some((uncommitted_diff, new_uncommitted_diff)) =
                uncommitted_diff.as_ref().zip(new_uncommitted_diff.clone())
            {
                uncommitted_diff.update(cx, |uncommitted_diff, cx| {
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
                this.update(cx, |this, _| {
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
                work_directory_id: work_directory_id.to_proto(),
                askpass_id,
                prompt,
            });
            cx.spawn(async move |_, _| {
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

    fn id(&self) -> ProjectEntryId {
        self.repository_entry.work_directory_id()
    }

    pub fn current_branch(&self) -> Option<&Branch> {
        self.repository_entry.branch()
    }

    pub fn status_for_path(&self, path: &RepoPath) -> Option<StatusEntry> {
        self.repository_entry.status_for_path(path)
    }

    fn send_job<F, Fut, R>(&self, job: F) -> oneshot::Receiver<R>
    where
        F: FnOnce(RepositoryState, AsyncApp) -> Fut + 'static,
        Fut: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        self.send_keyed_job(None, job)
    }

    fn send_keyed_job<F, Fut, R>(&self, key: Option<GitJobKey>, job: F) -> oneshot::Receiver<R>
    where
        F: FnOnce(RepositoryState, AsyncApp) -> Fut + 'static,
        Fut: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        let git_repo = self.state.clone();
        self.job_sender
            .unbounded_send(GitJob {
                key,
                job: Box::new(|cx: &mut AsyncApp| {
                    let job = job(git_repo, cx.clone());
                    cx.spawn(async move |_| {
                        let result = job.await;
                        result_tx.send(result).ok();
                    })
                }),
            })
            .ok();
        result_rx
    }

    /// This is the name that will be displayed in the repository selector for this repository.
    pub fn display_name(&self) -> SharedString {
        self.repository_entry
            .work_directory_abs_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            .into()
    }

    pub fn set_as_active_repository(&self, cx: &mut Context<Self>) {
        let Some(git_store) = self.git_store.upgrade() else {
            return;
        };
        let entity = cx.entity();
        git_store.update(cx, |git_store, cx| {
            let Some((&id, _)) = git_store
                .repositories
                .iter()
                .find(|(_, handle)| *handle == &entity)
            else {
                return;
            };
            git_store.active_repo_id = Some(id);
            cx.emit(GitEvent::ActiveRepositoryChanged);
        });
    }

    pub fn cached_status(&self) -> impl '_ + Iterator<Item = StatusEntry> {
        self.repository_entry.status()
    }

    pub fn status(&self, index: Option<GitIndex>) -> oneshot::Receiver<Result<GitStatus>> {
        self.send_job(move |repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => git_repository.status(index, &[]).await,
                RepositoryState::Remote { .. } => Err(anyhow!("not implemented yet")),
            }
        })
    }

    pub fn load_index_text(
        &self,
        index: Option<GitIndex>,
        path: RepoPath,
    ) -> oneshot::Receiver<Option<String>> {
        self.send_job(move |repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.load_index_text(index, path).await
                }
                RepositoryState::Remote { .. } => None,
            }
        })
    }

    pub fn has_conflict(&self, path: &RepoPath) -> bool {
        self.repository_entry
            .current_merge_conflicts
            .contains(&path)
    }

    pub fn repo_path_to_project_path(&self, path: &RepoPath, cx: &App) -> Option<ProjectPath> {
        let git_store = self.git_store.upgrade()?;
        let worktree_store = git_store.read(cx).worktree_store.read(cx);
        let abs_path = self.repository_entry.work_directory_abs_path.join(&path.0);
        let (worktree, relative_path) = worktree_store.find_worktree(abs_path, cx)?;
        Some(ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: relative_path.into(),
        })
    }

    pub fn project_path_to_repo_path(&self, path: &ProjectPath, cx: &App) -> Option<RepoPath> {
        let git_store = self.git_store.upgrade()?;
        let worktree_store = git_store.read(cx).worktree_store.read(cx);
        let abs_path = worktree_store.absolutize(path, cx)?;
        self.repository_entry.relativize_abs_path(&abs_path)
    }

    pub fn contains_sub_repo(&self, other: &Entity<Self>, cx: &App) -> bool {
        other
            .read(cx)
            .repository_entry
            .work_directory_abs_path
            .starts_with(&self.repository_entry.work_directory_abs_path)
    }

    pub fn local_repository(&self) -> Option<Arc<dyn GitRepository>> {
        match &self.state {
            RepositoryState::Local(git_repository) => Some(git_repository.clone()),
            RepositoryState::Remote { .. } => None,
        }
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

        if let RepositoryState::Remote {
            project_id,
            client,
            work_directory_id,
        } = self.state.clone()
        {
            let client = client.clone();
            cx.spawn(async move |repository, cx| {
                let request = client.request(proto::OpenCommitMessageBuffer {
                    project_id: project_id.0,
                    work_directory_id: work_directory_id.to_proto(),
                });
                let response = request.await.context("requesting to open commit buffer")?;
                let buffer_id = BufferId::new(response.buffer_id)?;
                let buffer = buffer_store
                    .update(cx, |buffer_store, cx| {
                        buffer_store.wait_for_remote_buffer(buffer_id, cx)
                    })?
                    .await?;
                if let Some(language_registry) = languages {
                    let git_commit_language =
                        language_registry.language_for_name("Git Commit").await?;
                    buffer.update(cx, |buffer, cx| {
                        buffer.set_language(Some(git_commit_language), cx);
                    })?;
                }
                repository.update(cx, |repository, _| {
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
        cx.spawn(async move |repository, cx| {
            let buffer = buffer_store
                .update(cx, |buffer_store, cx| buffer_store.create_buffer(cx))?
                .await?;

            if let Some(language_registry) = language_registry {
                let git_commit_language = language_registry.language_for_name("Git Commit").await?;
                buffer.update(cx, |buffer, cx| {
                    buffer.set_language(Some(git_commit_language), cx);
                })?;
            }

            repository.update(cx, |repository, _| {
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
                RepositoryState::Local(repo) => repo.checkout_files(commit, paths, env.await).await,
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    client
                        .request(proto::GitCheckoutFiles {
                            project_id: project_id.0,
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
                RepositoryState::Local(git_repo) => {
                    let env = env.await;
                    git_repo.reset(commit, reset_mode, env).await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    client
                        .request(proto::GitReset {
                            project_id: project_id.0,
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
        self.send_job(|git_repo, _cx| async move {
            match git_repo {
                RepositoryState::Local(git_repository) => git_repository.show(commit).await,
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    let resp = client
                        .request(proto::GitShow {
                            project_id: project_id.0,
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
                    let Some(project_path) = self.repo_path_to_project_path(path, cx) else {
                        continue;
                    };
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

        cx.spawn(async move |this, cx| {
            for save_future in save_futures {
                save_future.await?;
            }
            let env = env.await;

            this.update(cx, |this, _| {
                this.send_job(|git_repo, _cx| async move {
                    match git_repo {
                        RepositoryState::Local(repo) => repo.stage_paths(entries, env).await,
                        RepositoryState::Remote {
                            project_id,
                            client,
                            work_directory_id,
                        } => {
                            client
                                .request(proto::Stage {
                                    project_id: project_id.0,
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
                    let Some(project_path) = self.repo_path_to_project_path(path, cx) else {
                        continue;
                    };
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

        cx.spawn(async move |this, cx| {
            for save_future in save_futures {
                save_future.await?;
            }
            let env = env.await;

            this.update(cx, |this, _| {
                this.send_job(|git_repo, _cx| async move {
                    match git_repo {
                        RepositoryState::Local(repo) => repo.unstage_paths(entries, env).await,
                        RepositoryState::Remote {
                            project_id,
                            client,
                            work_directory_id,
                        } => {
                            client
                                .request(proto::Unstage {
                                    project_id: project_id.0,
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
                    self.worktree_id,
                    Some(
                        self.repository_entry
                            .work_directory_abs_path
                            .as_path()
                            .into(),
                    ),
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
        self.send_job(|git_repo, _cx| async move {
            match git_repo {
                RepositoryState::Local(repo) => {
                    let env = env.await;
                    repo.commit(message, name_and_email, env).await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    let (name, email) = name_and_email.unzip();
                    client
                        .request(proto::Commit {
                            project_id: project_id.0,
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
                RepositoryState::Local(git_repository) => {
                    let askpass = AskPassSession::new(&executor, askpass).await?;
                    let env = env.await;
                    git_repository.fetch(askpass, env, cx).await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
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
                RepositoryState::Local(git_repository) => {
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
                RepositoryState::Remote {
                    project_id,
                    client,
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
                RepositoryState::Local(git_repository) => {
                    let askpass = AskPassSession::new(&executor, askpass).await?;
                    let env = env.await;
                    git_repository
                        .pull(branch.to_string(), remote.to_string(), askpass, env, cx)
                        .await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
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

    fn spawn_set_index_text_job(
        &self,
        path: RepoPath,
        content: Option<String>,
        cx: &mut App,
    ) -> oneshot::Receiver<anyhow::Result<()>> {
        let env = self.worktree_environment(cx);

        self.send_keyed_job(
            Some(GitJobKey::WriteIndex(path.clone())),
            |git_repo, _cx| async {
                match git_repo {
                    RepositoryState::Local(repo) => {
                        repo.set_index_text(path, content, env.await).await
                    }
                    RepositoryState::Remote {
                        project_id,
                        client,
                        work_directory_id,
                    } => {
                        client
                            .request(proto::SetIndexText {
                                project_id: project_id.0,
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
        self.send_job(|repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.get_remotes(branch_name).await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    let response = client
                        .request(proto::GetRemotes {
                            project_id: project_id.0,
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

    pub fn branch(&self) -> Option<&Branch> {
        self.repository_entry.branch()
    }

    pub fn branches(&self) -> oneshot::Receiver<Result<Vec<Branch>>> {
        self.send_job(|repo, cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    let git_repository = git_repository.clone();
                    cx.background_spawn(async move { git_repository.branches().await })
                        .await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    let response = client
                        .request(proto::GitGetBranches {
                            project_id: project_id.0,
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
        self.send_job(|repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => git_repository.diff(diff_type).await,
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                    ..
                } => {
                    let response = client
                        .request(proto::GitDiff {
                            project_id: project_id.0,
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
        self.send_job(|repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.create_branch(branch_name).await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    client
                        .request(proto::GitCreateBranch {
                            project_id: project_id.0,
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
        self.send_job(|repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.change_branch(branch_name).await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    client
                        .request(proto::GitChangeBranch {
                            project_id: project_id.0,
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
        self.send_job(|repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.check_for_pushed_commit().await
                }
                RepositoryState::Remote {
                    project_id,
                    client,
                    work_directory_id,
                } => {
                    let response = client
                        .request(proto::CheckForPushedCommits {
                            project_id: project_id.0,
                            work_directory_id: work_directory_id.to_proto(),
                        })
                        .await?;

                    let branches = response.pushed_to.into_iter().map(Into::into).collect();

                    Ok(branches)
                }
            }
        })
    }

    pub fn checkpoint(&self) -> oneshot::Receiver<Result<GitRepositoryCheckpoint>> {
        self.send_job(|repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => git_repository.checkpoint().await,
                RepositoryState::Remote { .. } => Err(anyhow!("not implemented yet")),
            }
        })
    }

    pub fn restore_checkpoint(
        &self,
        checkpoint: GitRepositoryCheckpoint,
    ) -> oneshot::Receiver<Result<()>> {
        self.send_job(move |repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.restore_checkpoint(checkpoint).await
                }
                RepositoryState::Remote { .. } => Err(anyhow!("not implemented yet")),
            }
        })
    }

    pub(crate) fn apply_remote_update(&mut self, update: proto::UpdateRepository) -> Result<()> {
        let conflicted_paths = TreeSet::from_ordered_entries(
            update
                .current_merge_conflicts
                .into_iter()
                .map(|path| RepoPath(Path::new(&path).into())),
        );
        self.repository_entry.current_branch = update.branch_summary.as_ref().map(proto_to_branch);
        self.repository_entry.current_merge_conflicts = conflicted_paths;

        let edits = update
            .removed_statuses
            .into_iter()
            .map(|path| sum_tree::Edit::Remove(PathKey(FromProto::from_proto(path))))
            .chain(
                update
                    .updated_statuses
                    .into_iter()
                    .filter_map(|updated_status| {
                        Some(sum_tree::Edit::Insert(updated_status.try_into().log_err()?))
                    }),
            )
            .collect::<Vec<_>>();
        self.repository_entry.statuses_by_path.edit(edits, &());
        Ok(())
    }

    pub fn compare_checkpoints(
        &self,
        left: GitRepositoryCheckpoint,
        right: GitRepositoryCheckpoint,
    ) -> oneshot::Receiver<Result<bool>> {
        self.send_job(move |repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.compare_checkpoints(left, right).await
                }
                RepositoryState::Remote { .. } => Err(anyhow!("not implemented yet")),
            }
        })
    }

    pub fn delete_checkpoint(
        &self,
        checkpoint: GitRepositoryCheckpoint,
    ) -> oneshot::Receiver<Result<()>> {
        self.send_job(move |repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.delete_checkpoint(checkpoint).await
                }
                RepositoryState::Remote { .. } => Err(anyhow!("not implemented yet")),
            }
        })
    }

    pub fn diff_checkpoints(
        &self,
        base_checkpoint: GitRepositoryCheckpoint,
        target_checkpoint: GitRepositoryCheckpoint,
    ) -> oneshot::Receiver<Result<String>> {
        self.send_job(move |repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository
                        .diff_checkpoints(base_checkpoint, target_checkpoint)
                        .await
                }
                RepositoryState::Remote { .. } => Err(anyhow!("not implemented yet")),
            }
        })
    }

    pub fn create_index(&self) -> oneshot::Receiver<Result<GitIndex>> {
        self.send_job(move |repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => git_repository.create_index().await,
                RepositoryState::Remote { .. } => Err(anyhow!("not implemented yet")),
            }
        })
    }

    pub fn apply_diff(&self, index: GitIndex, diff: String) -> oneshot::Receiver<Result<()>> {
        self.send_job(move |repo, _cx| async move {
            match repo {
                RepositoryState::Local(git_repository) => {
                    git_repository.apply_diff(index, diff).await
                }
                RepositoryState::Remote { .. } => Err(anyhow!("not implemented yet")),
            }
        })
    }
}

fn get_permalink_in_rust_registry_src(
    provider_registry: Arc<GitHostingProviderRegistry>,
    path: PathBuf,
    selection: Range<u32>,
) -> Result<url::Url> {
    #[derive(Deserialize)]
    struct CargoVcsGit {
        sha1: String,
    }

    #[derive(Deserialize)]
    struct CargoVcsInfo {
        git: CargoVcsGit,
        path_in_vcs: String,
    }

    #[derive(Deserialize)]
    struct CargoPackage {
        repository: String,
    }

    #[derive(Deserialize)]
    struct CargoToml {
        package: CargoPackage,
    }

    let Some((dir, cargo_vcs_info_json)) = path.ancestors().skip(1).find_map(|dir| {
        let json = std::fs::read_to_string(dir.join(".cargo_vcs_info.json")).ok()?;
        Some((dir, json))
    }) else {
        bail!("No .cargo_vcs_info.json found in parent directories")
    };
    let cargo_vcs_info = serde_json::from_str::<CargoVcsInfo>(&cargo_vcs_info_json)?;
    let cargo_toml = std::fs::read_to_string(dir.join("Cargo.toml"))?;
    let manifest = toml::from_str::<CargoToml>(&cargo_toml)?;
    let (provider, remote) = parse_git_remote_url(provider_registry, &manifest.package.repository)
        .ok_or_else(|| anyhow!("Failed to parse package.repository field of manifest"))?;
    let path = PathBuf::from(cargo_vcs_info.path_in_vcs).join(path.strip_prefix(dir).unwrap());
    let permalink = provider.build_permalink(
        remote,
        BuildPermalinkParams {
            sha: &cargo_vcs_info.git.sha1,
            path: &path.to_string_lossy(),
            selection: Some(selection),
        },
    );
    Ok(permalink)
}

fn serialize_blame_buffer_response(blame: Option<git::blame::Blame>) -> proto::BlameBufferResponse {
    let Some(blame) = blame else {
        return proto::BlameBufferResponse {
            blame_response: None,
        };
    };

    let entries = blame
        .entries
        .into_iter()
        .map(|entry| proto::BlameEntry {
            sha: entry.sha.as_bytes().into(),
            start_line: entry.range.start,
            end_line: entry.range.end,
            original_line_number: entry.original_line_number,
            author: entry.author.clone(),
            author_mail: entry.author_mail.clone(),
            author_time: entry.author_time,
            author_tz: entry.author_tz.clone(),
            committer: entry.committer_name.clone(),
            committer_mail: entry.committer_email.clone(),
            committer_time: entry.committer_time,
            committer_tz: entry.committer_tz.clone(),
            summary: entry.summary.clone(),
            previous: entry.previous.clone(),
            filename: entry.filename.clone(),
        })
        .collect::<Vec<_>>();

    let messages = blame
        .messages
        .into_iter()
        .map(|(oid, message)| proto::CommitMessage {
            oid: oid.as_bytes().into(),
            message,
        })
        .collect::<Vec<_>>();

    proto::BlameBufferResponse {
        blame_response: Some(proto::blame_buffer_response::BlameResponse {
            entries,
            messages,
            remote_url: blame.remote_url,
        }),
    }
}

fn deserialize_blame_buffer_response(
    response: proto::BlameBufferResponse,
) -> Option<git::blame::Blame> {
    let response = response.blame_response?;
    let entries = response
        .entries
        .into_iter()
        .filter_map(|entry| {
            Some(git::blame::BlameEntry {
                sha: git::Oid::from_bytes(&entry.sha).ok()?,
                range: entry.start_line..entry.end_line,
                original_line_number: entry.original_line_number,
                committer_name: entry.committer,
                committer_time: entry.committer_time,
                committer_tz: entry.committer_tz,
                committer_email: entry.committer_mail,
                author: entry.author,
                author_mail: entry.author_mail,
                author_time: entry.author_time,
                author_tz: entry.author_tz,
                summary: entry.summary,
                previous: entry.previous,
                filename: entry.filename,
            })
        })
        .collect::<Vec<_>>();

    let messages = response
        .messages
        .into_iter()
        .filter_map(|message| Some((git::Oid::from_bytes(&message.oid).ok()?, message.message)))
        .collect::<HashMap<_, _>>();

    Some(Blame {
        entries,
        messages,
        remote_url: response.remote_url,
    })
}
