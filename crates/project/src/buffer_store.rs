use crate::{
    lsp_store::OpenLspBufferHandle,
    search::SearchQuery,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    ProjectItem as _, ProjectPath,
};
use ::git::{parse_git_remote_url, BuildPermalinkParams, GitHostingProviderRegistry};
use anyhow::{anyhow, bail, Context as _, Result};
use client::Client;
use collections::{hash_map, HashMap, HashSet};
use fs::Fs;
use futures::{
    channel::oneshot,
    future::{OptionFuture, Shared},
    Future, FutureExt as _, StreamExt,
};
use git::{blame::Blame, diff::BufferDiff, repository::RepoPath};
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity,
};
use http_client::Url;
use language::{
    proto::{
        deserialize_line_ending, deserialize_version, serialize_line_ending, serialize_version,
        split_operations,
    },
    Buffer, BufferEvent, Capability, DiskState, File as _, Language, LanguageRegistry, Operation,
};
use rpc::{proto, AnyProtoClient, ErrorExt as _, TypedEnvelope};
use serde::Deserialize;
use smol::channel::Receiver;
use std::{
    io,
    ops::Range,
    path::{Path, PathBuf},
    pin::pin,
    str::FromStr as _,
    sync::Arc,
    time::Instant,
};
use text::{BufferId, Rope};
use util::{debug_panic, maybe, ResultExt as _, TryFutureExt};
use worktree::{File, PathChange, ProjectEntryId, UpdatedGitRepositoriesSet, Worktree, WorktreeId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ChangeSetKind {
    Unstaged,
    Uncommitted,
}

/// A set of open buffers.
pub struct BufferStore {
    state: BufferStoreState,
    #[allow(clippy::type_complexity)]
    loading_buffers: HashMap<ProjectPath, Shared<Task<Result<Entity<Buffer>, Arc<anyhow::Error>>>>>,
    #[allow(clippy::type_complexity)]
    loading_change_sets: HashMap<
        (BufferId, ChangeSetKind),
        Shared<Task<Result<Entity<BufferChangeSet>, Arc<anyhow::Error>>>>,
    >,
    worktree_store: Entity<WorktreeStore>,
    opened_buffers: HashMap<BufferId, OpenBuffer>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    shared_buffers: HashMap<proto::PeerId, HashMap<BufferId, SharedBuffer>>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct SharedBuffer {
    buffer: Entity<Buffer>,
    change_set: Option<Entity<BufferChangeSet>>,
    lsp_handle: Option<OpenLspBufferHandle>,
}

#[derive(Default)]
struct BufferChangeSetState {
    unstaged_changes: Option<WeakEntity<BufferChangeSet>>,
    uncommitted_changes: Option<WeakEntity<BufferChangeSet>>,
    recalculate_diff_task: Option<Task<Result<()>>>,
    language: Option<Arc<Language>>,
    language_registry: Option<Arc<LanguageRegistry>>,
    diff_updated_futures: Vec<oneshot::Sender<()>>,
    buffer_subscription: Option<Subscription>,

    head_text: Option<Arc<String>>,
    index_text: Option<Arc<String>>,
    head_changed: bool,
    index_changed: bool,
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

impl BufferChangeSetState {
    fn buffer_language_changed(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.language = buffer.read(cx).language().cloned();
        self.index_changed = self.index_text.is_some();
        self.head_changed = self.head_text.is_some();
        let _ = self.recalculate_diffs(buffer.read(cx).text_snapshot(), cx);
    }

    fn unstaged_changes(&self) -> Option<Entity<BufferChangeSet>> {
        self.unstaged_changes.as_ref().and_then(|set| set.upgrade())
    }

    fn uncommitted_changes(&self) -> Option<Entity<BufferChangeSet>> {
        self.uncommitted_changes
            .as_ref()
            .and_then(|set| set.upgrade())
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
            Mode::IndexMatchesHead => DiffBasesChange::SetBoth(message.staged_text),
            Mode::IndexAndHead => DiffBasesChange::SetEach {
                index: message.staged_text,
                head: message.committed_text,
            },
        };

        let _ = self.diff_bases_changed(buffer, diff_bases_change, cx);
    }

    fn diff_bases_changed(
        &mut self,
        buffer: text::BufferSnapshot,
        diff_bases_change: DiffBasesChange,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<()> {
        match diff_bases_change {
            DiffBasesChange::SetIndex(index) => {
                self.index_text = index.map(|mut text| {
                    text::LineEnding::normalize(&mut text);
                    Arc::new(text)
                });
                self.index_changed = true;
            }
            DiffBasesChange::SetHead(head) => {
                self.head_text = head.map(|mut text| {
                    text::LineEnding::normalize(&mut text);
                    Arc::new(text)
                });
                self.head_changed = true;
            }
            DiffBasesChange::SetBoth(mut text) => {
                if let Some(text) = text.as_mut() {
                    text::LineEnding::normalize(text);
                }
                self.head_text = text.map(Arc::new);
                self.index_text = self.head_text.clone();
                self.head_changed = true;
                self.index_changed = true;
            }
            DiffBasesChange::SetEach { index, head } => {
                self.index_text = index.map(|mut text| {
                    text::LineEnding::normalize(&mut text);
                    Arc::new(text)
                });
                self.head_text = head.map(|mut text| {
                    text::LineEnding::normalize(&mut text);
                    Arc::new(text)
                });
                self.head_changed = true;
                self.index_changed = true;
            }
        }

        self.recalculate_diffs(buffer, cx)
    }

    fn recalculate_diffs(
        &mut self,
        buffer: text::BufferSnapshot,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();
        self.diff_updated_futures.push(tx);

        let language = self.language.clone();
        let language_registry = self.language_registry.clone();
        let unstaged_changes = self.unstaged_changes();
        let uncommitted_changes = self.uncommitted_changes();
        let head = self.head_text.clone();
        let index = self.index_text.clone();
        let index_changed = self.index_changed;
        let head_changed = self.head_changed;
        let index_matches_head = match (self.index_text.as_ref(), self.head_text.as_ref()) {
            (Some(index), Some(head)) => Arc::ptr_eq(index, head),
            (None, None) => true,
            _ => false,
        };
        self.recalculate_diff_task = Some(cx.spawn(|this, mut cx| async move {
            let snapshot = if index_changed {
                let snapshot = cx.update(|cx| {
                    index.as_ref().map(|head| {
                        language::Buffer::build_snapshot(
                            Rope::from(head.as_str()),
                            language.clone(),
                            language_registry.clone(),
                            cx,
                        )
                    })
                })?;
                cx.background_executor()
                    .spawn(OptionFuture::from(snapshot))
                    .await
            } else if let Some(unstaged_changes) = &unstaged_changes {
                unstaged_changes.read_with(&cx, |change_set, _| change_set.base_text.clone())?
            } else if let Some(uncommitted_changes) = &uncommitted_changes {
                uncommitted_changes
                    .read_with(&cx, |change_set, _| change_set.staged_text.clone())?
            } else {
                return Ok(());
            };

            if let Some(unstaged_changes) = &unstaged_changes {
                let diff = cx
                    .background_executor()
                    .spawn({
                        let buffer = buffer.clone();
                        async move {
                            BufferDiff::build(index.as_ref().map(|index| index.as_str()), &buffer)
                        }
                    })
                    .await;

                unstaged_changes.update(&mut cx, |unstaged_changes, cx| {
                    unstaged_changes.set_state(snapshot.clone(), diff, &buffer, cx);
                })?;

                if let Some(uncommitted_changes) = &uncommitted_changes {
                    uncommitted_changes.update(&mut cx, |uncommitted_changes, _| {
                        uncommitted_changes.staged_text = snapshot;
                    })?;
                }
            }

            if let Some(uncommitted_changes) = &uncommitted_changes {
                let (snapshot, diff) = if let (Some(unstaged_changes), true) =
                    (&unstaged_changes, index_matches_head)
                {
                    unstaged_changes.read_with(&cx, |change_set, _| {
                        (
                            change_set.base_text.clone(),
                            change_set.diff_to_buffer.clone(),
                        )
                    })?
                } else {
                    let snapshot = cx.update(|cx| {
                        head.as_deref().map(|head| {
                            language::Buffer::build_snapshot(
                                Rope::from(head.as_str()),
                                language.clone(),
                                language_registry.clone(),
                                cx,
                            )
                        })
                    })?;
                    let snapshot = cx.background_executor().spawn(OptionFuture::from(snapshot));
                    let diff = cx.background_executor().spawn({
                        let buffer = buffer.clone();
                        let head = head.clone();
                        async move {
                            BufferDiff::build(head.as_ref().map(|head| head.as_str()), &buffer)
                        }
                    });
                    futures::join!(snapshot, diff)
                };

                uncommitted_changes.update(&mut cx, |change_set, cx| {
                    change_set.set_state(snapshot, diff, &buffer, cx);
                })?;

                if index_changed || head_changed {
                    let staged_text = uncommitted_changes
                        .read_with(&cx, |change_set, _| change_set.staged_text.clone())?;

                    let diff = if index_matches_head {
                        staged_text.as_ref().map(|buffer| BufferDiff::new(buffer))
                    } else if let Some(staged_text) = staged_text {
                        Some(
                            cx.background_executor()
                                .spawn(async move {
                                    BufferDiff::build(
                                        head.as_ref().map(|head| head.as_str()),
                                        &staged_text,
                                    )
                                })
                                .await,
                        )
                    } else {
                        None
                    };

                    uncommitted_changes.update(&mut cx, |change_set, _| {
                        change_set.staged_diff = diff;
                    })?;
                }
            }

            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, _| {
                    this.index_changed = false;
                    this.head_changed = false;
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

pub struct BufferChangeSet {
    pub buffer_id: BufferId,
    pub base_text: Option<language::BufferSnapshot>,
    pub diff_to_buffer: BufferDiff,
    pub staged_text: Option<language::BufferSnapshot>,
    // For an uncommitted changeset, this is the diff between HEAD and the index.
    pub staged_diff: Option<BufferDiff>,
}

impl std::fmt::Debug for BufferChangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferChangeSet")
            .field("buffer_id", &self.buffer_id)
            .field("base_text", &self.base_text.as_ref().map(|s| s.text()))
            .field("diff_to_buffer", &self.diff_to_buffer)
            .field("staged_text", &self.staged_text.as_ref().map(|s| s.text()))
            .field("staged_diff", &self.staged_diff)
            .finish()
    }
}

pub enum BufferChangeSetEvent {
    DiffChanged { changed_range: Range<text::Anchor> },
}

enum BufferStoreState {
    Local(LocalBufferStore),
    Remote(RemoteBufferStore),
}

struct RemoteBufferStore {
    shared_with_me: HashSet<Entity<Buffer>>,
    upstream_client: AnyProtoClient,
    project_id: u64,
    loading_remote_buffers_by_id: HashMap<BufferId, Entity<Buffer>>,
    remote_buffer_listeners:
        HashMap<BufferId, Vec<oneshot::Sender<Result<Entity<Buffer>, anyhow::Error>>>>,
    worktree_store: Entity<WorktreeStore>,
}

struct LocalBufferStore {
    local_buffer_ids_by_path: HashMap<ProjectPath, BufferId>,
    local_buffer_ids_by_entry_id: HashMap<ProjectEntryId, BufferId>,
    worktree_store: Entity<WorktreeStore>,
    _subscription: Subscription,
}

enum OpenBuffer {
    Complete {
        buffer: WeakEntity<Buffer>,
        change_set_state: Entity<BufferChangeSetState>,
    },
    Operations(Vec<Operation>),
}

pub enum BufferStoreEvent {
    BufferAdded(Entity<Buffer>),
    BufferDropped(BufferId),
    BufferChangedFilePath {
        buffer: Entity<Buffer>,
        old_file: Option<Arc<dyn language::File>>,
    },
}

#[derive(Default, Debug)]
pub struct ProjectTransaction(pub HashMap<Entity<Buffer>, language::Transaction>);

impl EventEmitter<BufferStoreEvent> for BufferStore {}

impl RemoteBufferStore {
    fn open_unstaged_changes(&self, buffer_id: BufferId, cx: &App) -> Task<Result<Option<String>>> {
        let project_id = self.project_id;
        let client = self.upstream_client.clone();
        cx.background_executor().spawn(async move {
            let response = client
                .request(proto::OpenUnstagedChanges {
                    project_id,
                    buffer_id: buffer_id.to_proto(),
                })
                .await?;
            Ok(response.staged_text)
        })
    }

    fn open_uncommitted_changes(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Task<Result<DiffBasesChange>> {
        use proto::open_uncommitted_changes_response::Mode;

        let project_id = self.project_id;
        let client = self.upstream_client.clone();
        cx.background_executor().spawn(async move {
            let response = client
                .request(proto::OpenUncommittedChanges {
                    project_id,
                    buffer_id: buffer_id.to_proto(),
                })
                .await?;
            let mode = Mode::from_i32(response.mode).ok_or_else(|| anyhow!("Invalid mode"))?;
            let bases = match mode {
                Mode::IndexMatchesHead => DiffBasesChange::SetBoth(response.staged_text),
                Mode::IndexAndHead => DiffBasesChange::SetEach {
                    head: response.committed_text,
                    index: response.staged_text,
                },
            };
            Ok(bases)
        })
    }

    pub fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<Entity<Buffer>>> {
        let (tx, rx) = oneshot::channel();
        self.remote_buffer_listeners.entry(id).or_default().push(tx);

        cx.spawn(|this, cx| async move {
            if let Some(buffer) = this
                .read_with(&cx, |buffer_store, _| buffer_store.get(id))
                .ok()
                .flatten()
            {
                return Ok(buffer);
            }

            cx.background_executor()
                .spawn(async move { rx.await? })
                .await
        })
    }

    fn save_remote_buffer(
        &self,
        buffer_handle: Entity<Buffer>,
        new_path: Option<proto::ProjectPath>,
        cx: &Context<BufferStore>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id().into();
        let version = buffer.version();
        let rpc = self.upstream_client.clone();
        let project_id = self.project_id;
        cx.spawn(move |_, mut cx| async move {
            let response = rpc
                .request(proto::SaveBuffer {
                    project_id,
                    buffer_id,
                    new_path,
                    version: serialize_version(&version),
                })
                .await?;
            let version = deserialize_version(&response.version);
            let mtime = response.mtime.map(|mtime| mtime.into());

            buffer_handle.update(&mut cx, |buffer, cx| {
                buffer.did_save(version.clone(), mtime, cx);
            })?;

            Ok(())
        })
    }

    pub fn handle_create_buffer_for_peer(
        &mut self,
        envelope: TypedEnvelope<proto::CreateBufferForPeer>,
        replica_id: u16,
        capability: Capability,
        cx: &mut Context<BufferStore>,
    ) -> Result<Option<Entity<Buffer>>> {
        match envelope
            .payload
            .variant
            .ok_or_else(|| anyhow!("missing variant"))?
        {
            proto::create_buffer_for_peer::Variant::State(mut state) => {
                let buffer_id = BufferId::new(state.id)?;

                let buffer_result = maybe!({
                    let mut buffer_file = None;
                    if let Some(file) = state.file.take() {
                        let worktree_id = worktree::WorktreeId::from_proto(file.worktree_id);
                        let worktree = self
                            .worktree_store
                            .read(cx)
                            .worktree_for_id(worktree_id, cx)
                            .ok_or_else(|| {
                                anyhow!("no worktree found for id {}", file.worktree_id)
                            })?;
                        buffer_file = Some(Arc::new(File::from_proto(file, worktree.clone(), cx)?)
                            as Arc<dyn language::File>);
                    }
                    Buffer::from_proto(replica_id, capability, state, buffer_file)
                });

                match buffer_result {
                    Ok(buffer) => {
                        let buffer = cx.new(|_| buffer);
                        self.loading_remote_buffers_by_id.insert(buffer_id, buffer);
                    }
                    Err(error) => {
                        if let Some(listeners) = self.remote_buffer_listeners.remove(&buffer_id) {
                            for listener in listeners {
                                listener.send(Err(anyhow!(error.cloned()))).ok();
                            }
                        }
                    }
                }
            }
            proto::create_buffer_for_peer::Variant::Chunk(chunk) => {
                let buffer_id = BufferId::new(chunk.buffer_id)?;
                let buffer = self
                    .loading_remote_buffers_by_id
                    .get(&buffer_id)
                    .cloned()
                    .ok_or_else(|| {
                        anyhow!(
                            "received chunk for buffer {} without initial state",
                            chunk.buffer_id
                        )
                    })?;

                let result = maybe!({
                    let operations = chunk
                        .operations
                        .into_iter()
                        .map(language::proto::deserialize_operation)
                        .collect::<Result<Vec<_>>>()?;
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(operations, cx));
                    anyhow::Ok(())
                });

                if let Err(error) = result {
                    self.loading_remote_buffers_by_id.remove(&buffer_id);
                    if let Some(listeners) = self.remote_buffer_listeners.remove(&buffer_id) {
                        for listener in listeners {
                            listener.send(Err(error.cloned())).ok();
                        }
                    }
                } else if chunk.is_last {
                    self.loading_remote_buffers_by_id.remove(&buffer_id);
                    if self.upstream_client.is_via_collab() {
                        // retain buffers sent by peers to avoid races.
                        self.shared_with_me.insert(buffer.clone());
                    }

                    if let Some(senders) = self.remote_buffer_listeners.remove(&buffer_id) {
                        for sender in senders {
                            sender.send(Ok(buffer.clone())).ok();
                        }
                    }
                    return Ok(Some(buffer));
                }
            }
        }
        return Ok(None);
    }

    pub fn incomplete_buffer_ids(&self) -> Vec<BufferId> {
        self.loading_remote_buffers_by_id
            .keys()
            .copied()
            .collect::<Vec<_>>()
    }

    pub fn deserialize_project_transaction(
        &self,
        message: proto::ProjectTransaction,
        push_to_history: bool,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<ProjectTransaction>> {
        cx.spawn(|this, mut cx| async move {
            let mut project_transaction = ProjectTransaction::default();
            for (buffer_id, transaction) in message.buffer_ids.into_iter().zip(message.transactions)
            {
                let buffer_id = BufferId::new(buffer_id)?;
                let buffer = this
                    .update(&mut cx, |this, cx| {
                        this.wait_for_remote_buffer(buffer_id, cx)
                    })?
                    .await?;
                let transaction = language::proto::deserialize_transaction(transaction)?;
                project_transaction.0.insert(buffer, transaction);
            }

            for (buffer, transaction) in &project_transaction.0 {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_edits(transaction.edit_ids.iter().copied())
                    })?
                    .await?;

                if push_to_history {
                    buffer.update(&mut cx, |buffer, _| {
                        buffer.push_transaction(transaction.clone(), Instant::now());
                    })?;
                }
            }

            Ok(project_transaction)
        })
    }

    fn open_buffer(
        &self,
        path: Arc<Path>,
        worktree: Entity<Worktree>,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<Entity<Buffer>>> {
        let worktree_id = worktree.read(cx).id().to_proto();
        let project_id = self.project_id;
        let client = self.upstream_client.clone();
        let path_string = path.clone().to_string_lossy().to_string();
        cx.spawn(move |this, mut cx| async move {
            let response = client
                .request(proto::OpenBufferByPath {
                    project_id,
                    worktree_id,
                    path: path_string,
                })
                .await?;
            let buffer_id = BufferId::new(response.buffer_id)?;

            let buffer = this
                .update(&mut cx, {
                    |this, cx| this.wait_for_remote_buffer(buffer_id, cx)
                })?
                .await?;

            Ok(buffer)
        })
    }

    fn create_buffer(&self, cx: &mut Context<BufferStore>) -> Task<Result<Entity<Buffer>>> {
        let create = self.upstream_client.request(proto::OpenNewBuffer {
            project_id: self.project_id,
        });
        cx.spawn(|this, mut cx| async move {
            let response = create.await?;
            let buffer_id = BufferId::new(response.buffer_id)?;

            this.update(&mut cx, |this, cx| {
                this.wait_for_remote_buffer(buffer_id, cx)
            })?
            .await
        })
    }

    fn reload_buffers(
        &self,
        buffers: HashSet<Entity<Buffer>>,
        push_to_history: bool,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<ProjectTransaction>> {
        let request = self.upstream_client.request(proto::ReloadBuffers {
            project_id: self.project_id,
            buffer_ids: buffers
                .iter()
                .map(|buffer| buffer.read(cx).remote_id().to_proto())
                .collect(),
        });

        cx.spawn(|this, mut cx| async move {
            let response = request
                .await?
                .transaction
                .ok_or_else(|| anyhow!("missing transaction"))?;
            this.update(&mut cx, |this, cx| {
                this.deserialize_project_transaction(response, push_to_history, cx)
            })?
            .await
        })
    }
}

impl LocalBufferStore {
    fn worktree_for_buffer(
        &self,
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Option<(Entity<Worktree>, Arc<Path>)> {
        let file = buffer.read(cx).file()?;
        let worktree_id = file.worktree_id(cx);
        let path = file.path().clone();
        let worktree = self
            .worktree_store
            .read(cx)
            .worktree_for_id(worktree_id, cx)?;
        Some((worktree, path))
    }

    fn load_staged_text(&self, buffer: &Entity<Buffer>, cx: &App) -> Task<Result<Option<String>>> {
        if let Some((worktree, path)) = self.worktree_for_buffer(buffer, cx) {
            worktree.read(cx).load_staged_file(path.as_ref(), cx)
        } else {
            return Task::ready(Err(anyhow!("no such worktree")));
        }
    }

    fn load_committed_text(
        &self,
        buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Task<Result<Option<String>>> {
        if let Some((worktree, path)) = self.worktree_for_buffer(buffer, cx) {
            worktree.read(cx).load_committed_file(path.as_ref(), cx)
        } else {
            Task::ready(Err(anyhow!("no such worktree")))
        }
    }

    fn save_local_buffer(
        &self,
        buffer_handle: Entity<Buffer>,
        worktree: Entity<Worktree>,
        path: Arc<Path>,
        mut has_changed_file: bool,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);

        let text = buffer.as_rope().clone();
        let line_ending = buffer.line_ending();
        let version = buffer.version();
        let buffer_id = buffer.remote_id();
        if buffer
            .file()
            .is_some_and(|file| file.disk_state() == DiskState::New)
        {
            has_changed_file = true;
        }

        let save = worktree.update(cx, |worktree, cx| {
            worktree.write_file(path.as_ref(), text, line_ending, cx)
        });

        cx.spawn(move |this, mut cx| async move {
            let new_file = save.await?;
            let mtime = new_file.disk_state().mtime();
            this.update(&mut cx, |this, cx| {
                if let Some((downstream_client, project_id)) = this.downstream_client.clone() {
                    if has_changed_file {
                        downstream_client
                            .send(proto::UpdateBufferFile {
                                project_id,
                                buffer_id: buffer_id.to_proto(),
                                file: Some(language::File::to_proto(&*new_file, cx)),
                            })
                            .log_err();
                    }
                    downstream_client
                        .send(proto::BufferSaved {
                            project_id,
                            buffer_id: buffer_id.to_proto(),
                            version: serialize_version(&version),
                            mtime: mtime.map(|time| time.into()),
                        })
                        .log_err();
                }
            })?;
            buffer_handle.update(&mut cx, |buffer, cx| {
                if has_changed_file {
                    buffer.file_updated(new_file, cx);
                }
                buffer.did_save(version.clone(), mtime, cx);
            })
        })
    }

    fn subscribe_to_worktree(
        &mut self,
        worktree: &Entity<Worktree>,
        cx: &mut Context<BufferStore>,
    ) {
        cx.subscribe(worktree, |this, worktree, event, cx| {
            if worktree.read(cx).is_local() {
                match event {
                    worktree::Event::UpdatedEntries(changes) => {
                        Self::local_worktree_entries_changed(this, &worktree, changes, cx);
                    }
                    worktree::Event::UpdatedGitRepositories(updated_repos) => {
                        Self::local_worktree_git_repos_changed(
                            this,
                            worktree.clone(),
                            updated_repos,
                            cx,
                        )
                    }
                    _ => {}
                }
            }
        })
        .detach();
    }

    fn local_worktree_entries_changed(
        this: &mut BufferStore,
        worktree_handle: &Entity<Worktree>,
        changes: &[(Arc<Path>, ProjectEntryId, PathChange)],
        cx: &mut Context<BufferStore>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();
        for (path, entry_id, _) in changes {
            Self::local_worktree_entry_changed(
                this,
                *entry_id,
                path,
                worktree_handle,
                &snapshot,
                cx,
            );
        }
    }

    fn local_worktree_git_repos_changed(
        this: &mut BufferStore,
        worktree_handle: Entity<Worktree>,
        changed_repos: &UpdatedGitRepositoriesSet,
        cx: &mut Context<BufferStore>,
    ) {
        debug_assert!(worktree_handle.read(cx).is_local());

        let mut change_set_state_updates = Vec::new();
        for buffer in this.opened_buffers.values() {
            let OpenBuffer::Complete {
                buffer,
                change_set_state,
            } = buffer
            else {
                continue;
            };
            let Some(buffer) = buffer.upgrade() else {
                continue;
            };
            let buffer = buffer.read(cx);
            let Some(file) = File::from_dyn(buffer.file()) else {
                continue;
            };
            if file.worktree != worktree_handle {
                continue;
            }
            let change_set_state = change_set_state.read(cx);
            if changed_repos
                .iter()
                .any(|(work_dir, _)| file.path.starts_with(work_dir))
            {
                let snapshot = buffer.text_snapshot();
                change_set_state_updates.push((
                    snapshot.clone(),
                    file.path.clone(),
                    change_set_state
                        .unstaged_changes
                        .as_ref()
                        .and_then(|set| set.upgrade())
                        .is_some(),
                    change_set_state
                        .uncommitted_changes
                        .as_ref()
                        .and_then(|set| set.upgrade())
                        .is_some(),
                ))
            }
        }

        if change_set_state_updates.is_empty() {
            return;
        }

        cx.spawn(move |this, mut cx| async move {
            let snapshot =
                worktree_handle.update(&mut cx, |tree, _| tree.as_local().unwrap().snapshot())?;
            let diff_bases_changes_by_buffer = cx
                .background_executor()
                .spawn(async move {
                    change_set_state_updates
                        .into_iter()
                        .filter_map(
                            |(buffer_snapshot, path, needs_staged_text, needs_committed_text)| {
                                let local_repo = snapshot.local_repo_for_path(&path)?;
                                let relative_path = local_repo.relativize(&path).ok()?;
                                let staged_text = if needs_staged_text {
                                    local_repo.repo().load_index_text(&relative_path)
                                } else {
                                    None
                                };
                                let committed_text = if needs_committed_text {
                                    local_repo.repo().load_committed_text(&relative_path)
                                } else {
                                    None
                                };
                                let diff_bases_change =
                                    match (needs_staged_text, needs_committed_text) {
                                        (true, true) => Some(if staged_text == committed_text {
                                            DiffBasesChange::SetBoth(staged_text)
                                        } else {
                                            DiffBasesChange::SetEach {
                                                index: staged_text,
                                                head: committed_text,
                                            }
                                        }),
                                        (true, false) => {
                                            Some(DiffBasesChange::SetIndex(staged_text))
                                        }
                                        (false, true) => {
                                            Some(DiffBasesChange::SetHead(committed_text))
                                        }
                                        (false, false) => None,
                                    };
                                Some((buffer_snapshot, diff_bases_change))
                            },
                        )
                        .collect::<Vec<_>>()
                })
                .await;

            this.update(&mut cx, |this, cx| {
                for (buffer_snapshot, diff_bases_change) in diff_bases_changes_by_buffer {
                    let Some(OpenBuffer::Complete {
                        change_set_state, ..
                    }) = this.opened_buffers.get_mut(&buffer_snapshot.remote_id())
                    else {
                        continue;
                    };
                    let Some(diff_bases_change) = diff_bases_change else {
                        continue;
                    };

                    change_set_state.update(cx, |change_set_state, cx| {
                        use proto::update_diff_bases::Mode;

                        if let Some((client, project_id)) = this.downstream_client.as_ref() {
                            let buffer_id = buffer_snapshot.remote_id().to_proto();
                            let (staged_text, committed_text, mode) = match diff_bases_change
                                .clone()
                            {
                                DiffBasesChange::SetIndex(index) => (index, None, Mode::IndexOnly),
                                DiffBasesChange::SetHead(head) => (None, head, Mode::HeadOnly),
                                DiffBasesChange::SetEach { index, head } => {
                                    (index, head, Mode::IndexAndHead)
                                }
                                DiffBasesChange::SetBoth(text) => {
                                    (text, None, Mode::IndexMatchesHead)
                                }
                            };
                            let message = proto::UpdateDiffBases {
                                project_id: *project_id,
                                buffer_id,
                                staged_text,
                                committed_text,
                                mode: mode as i32,
                            };

                            client.send(message).log_err();
                        }

                        let _ = change_set_state.diff_bases_changed(
                            buffer_snapshot,
                            diff_bases_change,
                            cx,
                        );
                    });
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn local_worktree_entry_changed(
        this: &mut BufferStore,
        entry_id: ProjectEntryId,
        path: &Arc<Path>,
        worktree: &Entity<worktree::Worktree>,
        snapshot: &worktree::Snapshot,
        cx: &mut Context<BufferStore>,
    ) -> Option<()> {
        let project_path = ProjectPath {
            worktree_id: snapshot.id(),
            path: path.clone(),
        };

        let buffer_id = {
            let local = this.as_local_mut()?;
            match local.local_buffer_ids_by_entry_id.get(&entry_id) {
                Some(&buffer_id) => buffer_id,
                None => local.local_buffer_ids_by_path.get(&project_path).copied()?,
            }
        };

        let buffer = if let Some(buffer) = this.get(buffer_id) {
            Some(buffer)
        } else {
            this.opened_buffers.remove(&buffer_id);
            None
        };

        let buffer = if let Some(buffer) = buffer {
            buffer
        } else {
            let this = this.as_local_mut()?;
            this.local_buffer_ids_by_path.remove(&project_path);
            this.local_buffer_ids_by_entry_id.remove(&entry_id);
            return None;
        };

        let events = buffer.update(cx, |buffer, cx| {
            let local = this.as_local_mut()?;
            let file = buffer.file()?;
            let old_file = File::from_dyn(Some(file))?;
            if old_file.worktree != *worktree {
                return None;
            }

            let snapshot_entry = old_file
                .entry_id
                .and_then(|entry_id| snapshot.entry_for_id(entry_id))
                .or_else(|| snapshot.entry_for_path(old_file.path.as_ref()));

            let new_file = if let Some(entry) = snapshot_entry {
                File {
                    disk_state: match entry.mtime {
                        Some(mtime) => DiskState::Present { mtime },
                        None => old_file.disk_state,
                    },
                    is_local: true,
                    entry_id: Some(entry.id),
                    path: entry.path.clone(),
                    worktree: worktree.clone(),
                    is_private: entry.is_private,
                }
            } else {
                File {
                    disk_state: DiskState::Deleted,
                    is_local: true,
                    entry_id: old_file.entry_id,
                    path: old_file.path.clone(),
                    worktree: worktree.clone(),
                    is_private: old_file.is_private,
                }
            };

            if new_file == *old_file {
                return None;
            }

            let mut events = Vec::new();
            if new_file.path != old_file.path {
                local.local_buffer_ids_by_path.remove(&ProjectPath {
                    path: old_file.path.clone(),
                    worktree_id: old_file.worktree_id(cx),
                });
                local.local_buffer_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: new_file.worktree_id(cx),
                        path: new_file.path.clone(),
                    },
                    buffer_id,
                );
                events.push(BufferStoreEvent::BufferChangedFilePath {
                    buffer: cx.entity(),
                    old_file: buffer.file().cloned(),
                });
            }

            if new_file.entry_id != old_file.entry_id {
                if let Some(entry_id) = old_file.entry_id {
                    local.local_buffer_ids_by_entry_id.remove(&entry_id);
                }
                if let Some(entry_id) = new_file.entry_id {
                    local
                        .local_buffer_ids_by_entry_id
                        .insert(entry_id, buffer_id);
                }
            }

            if let Some((client, project_id)) = &this.downstream_client {
                client
                    .send(proto::UpdateBufferFile {
                        project_id: *project_id,
                        buffer_id: buffer_id.to_proto(),
                        file: Some(new_file.to_proto(cx)),
                    })
                    .ok();
            }

            buffer.file_updated(Arc::new(new_file), cx);
            Some(events)
        })?;

        for event in events {
            cx.emit(event);
        }

        None
    }

    fn buffer_changed_file(&mut self, buffer: Entity<Buffer>, cx: &mut App) -> Option<()> {
        let file = File::from_dyn(buffer.read(cx).file())?;

        let remote_id = buffer.read(cx).remote_id();
        if let Some(entry_id) = file.entry_id {
            match self.local_buffer_ids_by_entry_id.get(&entry_id) {
                Some(_) => {
                    return None;
                }
                None => {
                    self.local_buffer_ids_by_entry_id
                        .insert(entry_id, remote_id);
                }
            }
        };
        self.local_buffer_ids_by_path.insert(
            ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path.clone(),
            },
            remote_id,
        );

        Some(())
    }

    fn save_buffer(
        &self,
        buffer: Entity<Buffer>,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<()>> {
        let Some(file) = File::from_dyn(buffer.read(cx).file()) else {
            return Task::ready(Err(anyhow!("buffer doesn't have a file")));
        };
        let worktree = file.worktree.clone();
        self.save_local_buffer(buffer, worktree, file.path.clone(), false, cx)
    }

    fn save_buffer_as(
        &self,
        buffer: Entity<Buffer>,
        path: ProjectPath,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<()>> {
        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };
        self.save_local_buffer(buffer, worktree, path.path.clone(), true, cx)
    }

    fn open_buffer(
        &self,
        path: Arc<Path>,
        worktree: Entity<Worktree>,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<Entity<Buffer>>> {
        let load_buffer = worktree.update(cx, |worktree, cx| {
            let load_file = worktree.load_file(path.as_ref(), cx);
            let reservation = cx.reserve_entity();
            let buffer_id = BufferId::from(reservation.entity_id().as_non_zero_u64());
            cx.spawn(move |_, mut cx| async move {
                let loaded = load_file.await?;
                let text_buffer = cx
                    .background_executor()
                    .spawn(async move { text::Buffer::new(0, buffer_id, loaded.text) })
                    .await;
                cx.insert_entity(reservation, |_| {
                    Buffer::build(text_buffer, Some(loaded.file), Capability::ReadWrite)
                })
            })
        });

        cx.spawn(move |this, mut cx| async move {
            let buffer = match load_buffer.await {
                Ok(buffer) => Ok(buffer),
                Err(error) if is_not_found_error(&error) => cx.new(|cx| {
                    let buffer_id = BufferId::from(cx.entity_id().as_non_zero_u64());
                    let text_buffer = text::Buffer::new(0, buffer_id, "".into());
                    Buffer::build(
                        text_buffer,
                        Some(Arc::new(File {
                            worktree,
                            path,
                            disk_state: DiskState::New,
                            entry_id: None,
                            is_local: true,
                            is_private: false,
                        })),
                        Capability::ReadWrite,
                    )
                }),
                Err(e) => Err(e),
            }?;
            this.update(&mut cx, |this, cx| {
                this.add_buffer(buffer.clone(), cx)?;
                let buffer_id = buffer.read(cx).remote_id();
                if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
                    let this = this.as_local_mut().unwrap();
                    this.local_buffer_ids_by_path.insert(
                        ProjectPath {
                            worktree_id: file.worktree_id(cx),
                            path: file.path.clone(),
                        },
                        buffer_id,
                    );

                    if let Some(entry_id) = file.entry_id {
                        this.local_buffer_ids_by_entry_id
                            .insert(entry_id, buffer_id);
                    }
                }

                anyhow::Ok(())
            })??;

            Ok(buffer)
        })
    }

    fn create_buffer(&self, cx: &mut Context<BufferStore>) -> Task<Result<Entity<Buffer>>> {
        cx.spawn(|buffer_store, mut cx| async move {
            let buffer =
                cx.new(|cx| Buffer::local("", cx).with_language(language::PLAIN_TEXT.clone(), cx))?;
            buffer_store.update(&mut cx, |buffer_store, cx| {
                buffer_store.add_buffer(buffer.clone(), cx).log_err();
            })?;
            Ok(buffer)
        })
    }

    fn reload_buffers(
        &self,
        buffers: HashSet<Entity<Buffer>>,
        push_to_history: bool,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<ProjectTransaction>> {
        cx.spawn(move |_, mut cx| async move {
            let mut project_transaction = ProjectTransaction::default();
            for buffer in buffers {
                let transaction = buffer
                    .update(&mut cx, |buffer, cx| buffer.reload(cx))?
                    .await?;
                buffer.update(&mut cx, |buffer, cx| {
                    if let Some(transaction) = transaction {
                        if !push_to_history {
                            buffer.forget_transaction(transaction.id);
                        }
                        project_transaction.0.insert(cx.entity(), transaction);
                    }
                })?;
            }

            Ok(project_transaction)
        })
    }
}

impl BufferStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_message_handler(Self::handle_buffer_reloaded);
        client.add_entity_message_handler(Self::handle_buffer_saved);
        client.add_entity_message_handler(Self::handle_update_buffer_file);
        client.add_entity_request_handler(Self::handle_save_buffer);
        client.add_entity_request_handler(Self::handle_blame_buffer);
        client.add_entity_request_handler(Self::handle_reload_buffers);
        client.add_entity_request_handler(Self::handle_get_permalink_to_line);
        client.add_entity_request_handler(Self::handle_open_unstaged_changes);
        client.add_entity_request_handler(Self::handle_open_uncommitted_changes);
        client.add_entity_message_handler(Self::handle_update_diff_bases);
    }

    /// Creates a buffer store, optionally retaining its buffers.
    pub fn local(worktree_store: Entity<WorktreeStore>, cx: &mut Context<Self>) -> Self {
        Self {
            state: BufferStoreState::Local(LocalBufferStore {
                local_buffer_ids_by_path: Default::default(),
                local_buffer_ids_by_entry_id: Default::default(),
                worktree_store: worktree_store.clone(),
                _subscription: cx.subscribe(&worktree_store, |this, _, event, cx| {
                    if let WorktreeStoreEvent::WorktreeAdded(worktree) = event {
                        let this = this.as_local_mut().unwrap();
                        this.subscribe_to_worktree(worktree, cx);
                    }
                }),
            }),
            downstream_client: None,
            opened_buffers: Default::default(),
            shared_buffers: Default::default(),
            loading_buffers: Default::default(),
            loading_change_sets: Default::default(),
            worktree_store,
        }
    }

    pub fn remote(
        worktree_store: Entity<WorktreeStore>,
        upstream_client: AnyProtoClient,
        remote_id: u64,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            state: BufferStoreState::Remote(RemoteBufferStore {
                shared_with_me: Default::default(),
                loading_remote_buffers_by_id: Default::default(),
                remote_buffer_listeners: Default::default(),
                project_id: remote_id,
                upstream_client,
                worktree_store: worktree_store.clone(),
            }),
            downstream_client: None,
            opened_buffers: Default::default(),
            loading_buffers: Default::default(),
            loading_change_sets: Default::default(),
            shared_buffers: Default::default(),
            worktree_store,
        }
    }

    fn as_local_mut(&mut self) -> Option<&mut LocalBufferStore> {
        match &mut self.state {
            BufferStoreState::Local(state) => Some(state),
            _ => None,
        }
    }

    fn as_remote_mut(&mut self) -> Option<&mut RemoteBufferStore> {
        match &mut self.state {
            BufferStoreState::Remote(state) => Some(state),
            _ => None,
        }
    }

    fn as_remote(&self) -> Option<&RemoteBufferStore> {
        match &self.state {
            BufferStoreState::Remote(state) => Some(state),
            _ => None,
        }
    }

    pub fn open_buffer(
        &mut self,
        project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        if let Some(buffer) = self.get_by_path(&project_path, cx) {
            return Task::ready(Ok(buffer));
        }

        let task = match self.loading_buffers.entry(project_path.clone()) {
            hash_map::Entry::Occupied(e) => e.get().clone(),
            hash_map::Entry::Vacant(entry) => {
                let path = project_path.path.clone();
                let Some(worktree) = self
                    .worktree_store
                    .read(cx)
                    .worktree_for_id(project_path.worktree_id, cx)
                else {
                    return Task::ready(Err(anyhow!("no such worktree")));
                };
                let load_buffer = match &self.state {
                    BufferStoreState::Local(this) => this.open_buffer(path, worktree, cx),
                    BufferStoreState::Remote(this) => this.open_buffer(path, worktree, cx),
                };

                entry
                    .insert(
                        cx.spawn(move |this, mut cx| async move {
                            let load_result = load_buffer.await;
                            this.update(&mut cx, |this, _cx| {
                                // Record the fact that the buffer is no longer loading.
                                this.loading_buffers.remove(&project_path);
                            })
                            .ok();
                            load_result.map_err(Arc::new)
                        })
                        .shared(),
                    )
                    .clone()
            }
        };

        cx.background_executor()
            .spawn(async move { task.await.map_err(|e| anyhow!("{e}")) })
    }

    pub fn open_unstaged_changes(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferChangeSet>>> {
        let buffer_id = buffer.read(cx).remote_id();
        if let Some(change_set) = self.get_unstaged_changes(buffer_id, cx) {
            return Task::ready(Ok(change_set));
        }

        let task = match self
            .loading_change_sets
            .entry((buffer_id, ChangeSetKind::Unstaged))
        {
            hash_map::Entry::Occupied(e) => e.get().clone(),
            hash_map::Entry::Vacant(entry) => {
                let staged_text = match &self.state {
                    BufferStoreState::Local(this) => this.load_staged_text(&buffer, cx),
                    BufferStoreState::Remote(this) => this.open_unstaged_changes(buffer_id, cx),
                };

                entry
                    .insert(
                        cx.spawn(move |this, cx| async move {
                            Self::open_change_set_internal(
                                this,
                                ChangeSetKind::Unstaged,
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

        cx.background_executor()
            .spawn(async move { task.await.map_err(|e| anyhow!("{e}")) })
    }

    pub fn open_uncommitted_changes(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferChangeSet>>> {
        let buffer_id = buffer.read(cx).remote_id();
        if let Some(change_set) = self.get_uncommitted_changes(buffer_id, cx) {
            return Task::ready(Ok(change_set));
        }

        let task = match self
            .loading_change_sets
            .entry((buffer_id, ChangeSetKind::Uncommitted))
        {
            hash_map::Entry::Occupied(e) => e.get().clone(),
            hash_map::Entry::Vacant(entry) => {
                let changes = match &self.state {
                    BufferStoreState::Local(this) => {
                        let committed_text = this.load_committed_text(&buffer, cx);
                        let staged_text = this.load_staged_text(&buffer, cx);
                        cx.background_executor().spawn(async move {
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
                    }
                    BufferStoreState::Remote(this) => this.open_uncommitted_changes(buffer_id, cx),
                };

                entry
                    .insert(
                        cx.spawn(move |this, cx| async move {
                            Self::open_change_set_internal(
                                this,
                                ChangeSetKind::Uncommitted,
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

        cx.background_executor()
            .spawn(async move { task.await.map_err(|e| anyhow!("{e}")) })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_unstaged_change_set(
        &mut self,
        buffer_id: BufferId,
        change_set: Entity<BufferChangeSet>,
    ) {
        self.loading_change_sets.insert(
            (buffer_id, ChangeSetKind::Unstaged),
            Task::ready(Ok(change_set)).shared(),
        );
    }

    async fn open_change_set_internal(
        this: WeakEntity<Self>,
        kind: ChangeSetKind,
        texts: Result<DiffBasesChange>,
        buffer: Entity<Buffer>,
        mut cx: AsyncApp,
    ) -> Result<Entity<BufferChangeSet>> {
        let diff_bases_change = match texts {
            Err(e) => {
                this.update(&mut cx, |this, cx| {
                    let buffer_id = buffer.read(cx).remote_id();
                    this.loading_change_sets.remove(&(buffer_id, kind));
                })?;
                return Err(e);
            }
            Ok(change) => change,
        };

        this.update(&mut cx, |this, cx| {
            let buffer_id = buffer.read(cx).remote_id();
            this.loading_change_sets.remove(&(buffer_id, kind));

            if let Some(OpenBuffer::Complete {
                change_set_state, ..
            }) = this.opened_buffers.get_mut(&buffer.read(cx).remote_id())
            {
                change_set_state.update(cx, |change_set_state, cx| {
                    let buffer_id = buffer.read(cx).remote_id();
                    change_set_state.buffer_subscription.get_or_insert_with(|| {
                        cx.subscribe(&buffer, |this, buffer, event, cx| match event {
                            BufferEvent::LanguageChanged => {
                                this.buffer_language_changed(buffer, cx)
                            }
                            _ => {}
                        })
                    });

                    let change_set = cx.new(|cx| BufferChangeSet {
                        buffer_id,
                        base_text: None,
                        diff_to_buffer: BufferDiff::new(&buffer.read(cx).text_snapshot()),
                        staged_text: None,
                        staged_diff: None,
                    });
                    match kind {
                        ChangeSetKind::Unstaged => {
                            change_set_state.unstaged_changes = Some(change_set.downgrade())
                        }
                        ChangeSetKind::Uncommitted => {
                            change_set_state.uncommitted_changes = Some(change_set.downgrade())
                        }
                    };

                    let buffer = buffer.read(cx).text_snapshot();
                    let rx = change_set_state.diff_bases_changed(buffer, diff_bases_change, cx);

                    Ok(async move {
                        rx.await.ok();
                        Ok(change_set)
                    })
                })
            } else {
                Err(anyhow!("buffer was closed"))
            }
        })??
        .await
    }

    pub fn create_buffer(&mut self, cx: &mut Context<Self>) -> Task<Result<Entity<Buffer>>> {
        match &self.state {
            BufferStoreState::Local(this) => this.create_buffer(cx),
            BufferStoreState::Remote(this) => this.create_buffer(cx),
        }
    }

    pub fn save_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        match &mut self.state {
            BufferStoreState::Local(this) => this.save_buffer(buffer, cx),
            BufferStoreState::Remote(this) => this.save_remote_buffer(buffer.clone(), None, cx),
        }
    }

    pub fn save_buffer_as(
        &mut self,
        buffer: Entity<Buffer>,
        path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let old_file = buffer.read(cx).file().cloned();
        let task = match &self.state {
            BufferStoreState::Local(this) => this.save_buffer_as(buffer.clone(), path, cx),
            BufferStoreState::Remote(this) => {
                this.save_remote_buffer(buffer.clone(), Some(path.to_proto()), cx)
            }
        };
        cx.spawn(|this, mut cx| async move {
            task.await?;
            this.update(&mut cx, |_, cx| {
                cx.emit(BufferStoreEvent::BufferChangedFilePath { buffer, old_file });
            })
        })
    }

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
                    let local_repo = match worktree.local_repo_for_path(&file.path) {
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

                cx.background_executor().spawn(async move {
                    let Some((repo, relative_path, content)) = blame_params? else {
                        return Ok(None);
                    };
                    repo.blame(&relative_path, content)
                        .with_context(|| format!("Failed to blame {:?}", relative_path.0))
                        .map(Some)
                })
            }
            Worktree::Remote(worktree) => {
                let buffer_id = buffer.remote_id();
                let version = buffer.version();
                let project_id = worktree.project_id();
                let client = worktree.client();
                cx.spawn(|_| async move {
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
                let worktree_path = worktree.abs_path().clone();
                let Some((repo_entry, repo)) =
                    worktree.repository_for_path(file.path()).and_then(|entry| {
                        let repo = worktree.get_local_repo(&entry)?.repo().clone();
                        Some((entry, repo))
                    })
                else {
                    // If we're not in a Git repo, check whether this is a Rust source
                    // file in the Cargo registry (presumably opened with go-to-definition
                    // from a normal Rust file). If so, we can put together a permalink
                    // using crate metadata.
                    if !buffer
                        .language()
                        .is_some_and(|lang| lang.name() == "Rust".into())
                    {
                        return Task::ready(Err(anyhow!("no permalink available")));
                    }
                    let file_path = worktree_path.join(file.path());
                    return cx.spawn(|cx| async move {
                        let provider_registry =
                            cx.update(GitHostingProviderRegistry::default_global)?;
                        get_permalink_in_rust_registry_src(provider_registry, file_path, selection)
                            .map_err(|_| anyhow!("no permalink available"))
                    });
                };

                let path = match repo_entry.relativize(file.path()) {
                    Ok(RepoPath(path)) => path,
                    Err(e) => return Task::ready(Err(e)),
                };

                cx.spawn(|cx| async move {
                    const REMOTE_NAME: &str = "origin";
                    let origin_url = repo
                        .remote_url(REMOTE_NAME)
                        .ok_or_else(|| anyhow!("remote \"{REMOTE_NAME}\" not found"))?;

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
                cx.spawn(|_| async move {
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

    fn add_buffer(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Result<()> {
        let remote_id = buffer.read(cx).remote_id();
        let is_remote = buffer.read(cx).replica_id() != 0;
        let open_buffer = OpenBuffer::Complete {
            buffer: buffer.downgrade(),
            change_set_state: cx.new(|_| BufferChangeSetState::default()),
        };

        let handle = cx.entity().downgrade();
        buffer.update(cx, move |_, cx| {
            cx.on_release(move |buffer, cx| {
                handle
                    .update(cx, |_, cx| {
                        cx.emit(BufferStoreEvent::BufferDropped(buffer.remote_id()))
                    })
                    .ok();
            })
            .detach()
        });

        match self.opened_buffers.entry(remote_id) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(open_buffer);
            }
            hash_map::Entry::Occupied(mut entry) => {
                if let OpenBuffer::Operations(operations) = entry.get_mut() {
                    buffer.update(cx, |b, cx| b.apply_ops(operations.drain(..), cx));
                } else if entry.get().upgrade().is_some() {
                    if is_remote {
                        return Ok(());
                    } else {
                        debug_panic!("buffer {} was already registered", remote_id);
                        Err(anyhow!("buffer {} was already registered", remote_id))?;
                    }
                }
                entry.insert(open_buffer);
            }
        }

        cx.subscribe(&buffer, Self::on_buffer_event).detach();
        cx.emit(BufferStoreEvent::BufferAdded(buffer));
        Ok(())
    }

    pub fn buffers(&self) -> impl '_ + Iterator<Item = Entity<Buffer>> {
        self.opened_buffers
            .values()
            .filter_map(|buffer| buffer.upgrade())
    }

    pub fn loading_buffers(
        &self,
    ) -> impl Iterator<Item = (&ProjectPath, impl Future<Output = Result<Entity<Buffer>>>)> {
        self.loading_buffers.iter().map(|(path, task)| {
            let task = task.clone();
            (path, async move { task.await.map_err(|e| anyhow!("{e}")) })
        })
    }

    pub fn get_by_path(&self, path: &ProjectPath, cx: &App) -> Option<Entity<Buffer>> {
        self.buffers().find_map(|buffer| {
            let file = File::from_dyn(buffer.read(cx).file())?;
            if file.worktree_id(cx) == path.worktree_id && file.path == path.path {
                Some(buffer)
            } else {
                None
            }
        })
    }

    pub fn get(&self, buffer_id: BufferId) -> Option<Entity<Buffer>> {
        self.opened_buffers.get(&buffer_id)?.upgrade()
    }

    pub fn get_existing(&self, buffer_id: BufferId) -> Result<Entity<Buffer>> {
        self.get(buffer_id)
            .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))
    }

    pub fn get_possibly_incomplete(&self, buffer_id: BufferId) -> Option<Entity<Buffer>> {
        self.get(buffer_id).or_else(|| {
            self.as_remote()
                .and_then(|remote| remote.loading_remote_buffers_by_id.get(&buffer_id).cloned())
        })
    }

    pub fn get_unstaged_changes(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Option<Entity<BufferChangeSet>> {
        if let OpenBuffer::Complete {
            change_set_state, ..
        } = self.opened_buffers.get(&buffer_id)?
        {
            change_set_state
                .read(cx)
                .unstaged_changes
                .as_ref()?
                .upgrade()
        } else {
            None
        }
    }

    pub fn get_uncommitted_changes(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Option<Entity<BufferChangeSet>> {
        if let OpenBuffer::Complete {
            change_set_state, ..
        } = self.opened_buffers.get(&buffer_id)?
        {
            change_set_state
                .read(cx)
                .uncommitted_changes
                .as_ref()?
                .upgrade()
        } else {
            None
        }
    }

    pub fn buffer_version_info(&self, cx: &App) -> (Vec<proto::BufferVersion>, Vec<BufferId>) {
        let buffers = self
            .buffers()
            .map(|buffer| {
                let buffer = buffer.read(cx);
                proto::BufferVersion {
                    id: buffer.remote_id().into(),
                    version: language::proto::serialize_version(&buffer.version),
                }
            })
            .collect();
        let incomplete_buffer_ids = self
            .as_remote()
            .map(|remote| remote.incomplete_buffer_ids())
            .unwrap_or_default();
        (buffers, incomplete_buffer_ids)
    }

    pub fn disconnected_from_host(&mut self, cx: &mut App) {
        for open_buffer in self.opened_buffers.values_mut() {
            if let Some(buffer) = open_buffer.upgrade() {
                buffer.update(cx, |buffer, _| buffer.give_up_waiting());
            }
        }

        for buffer in self.buffers() {
            buffer.update(cx, |buffer, cx| {
                buffer.set_capability(Capability::ReadOnly, cx)
            });
        }

        if let Some(remote) = self.as_remote_mut() {
            // Wake up all futures currently waiting on a buffer to get opened,
            // to give them a chance to fail now that we've disconnected.
            remote.remote_buffer_listeners.clear()
        }
    }

    pub fn shared(&mut self, remote_id: u64, downstream_client: AnyProtoClient, _cx: &mut App) {
        self.downstream_client = Some((downstream_client, remote_id));
    }

    pub fn unshared(&mut self, _cx: &mut Context<Self>) {
        self.downstream_client.take();
        self.forget_shared_buffers();
    }

    pub fn discard_incomplete(&mut self) {
        self.opened_buffers
            .retain(|_, buffer| !matches!(buffer, OpenBuffer::Operations(_)));
    }

    pub fn find_search_candidates(
        &mut self,
        query: &SearchQuery,
        mut limit: usize,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) -> Receiver<Entity<Buffer>> {
        let (tx, rx) = smol::channel::unbounded();
        let mut open_buffers = HashSet::default();
        let mut unnamed_buffers = Vec::new();
        for handle in self.buffers() {
            let buffer = handle.read(cx);
            if let Some(entry_id) = buffer.entry_id(cx) {
                open_buffers.insert(entry_id);
            } else {
                limit = limit.saturating_sub(1);
                unnamed_buffers.push(handle)
            };
        }

        const MAX_CONCURRENT_BUFFER_OPENS: usize = 64;
        let project_paths_rx = self
            .worktree_store
            .update(cx, |worktree_store, cx| {
                worktree_store.find_search_candidates(query.clone(), limit, open_buffers, fs, cx)
            })
            .chunks(MAX_CONCURRENT_BUFFER_OPENS);

        cx.spawn(|this, mut cx| async move {
            for buffer in unnamed_buffers {
                tx.send(buffer).await.ok();
            }

            let mut project_paths_rx = pin!(project_paths_rx);
            while let Some(project_paths) = project_paths_rx.next().await {
                let buffers = this.update(&mut cx, |this, cx| {
                    project_paths
                        .into_iter()
                        .map(|project_path| this.open_buffer(project_path, cx))
                        .collect::<Vec<_>>()
                })?;
                for buffer_task in buffers {
                    if let Some(buffer) = buffer_task.await.log_err() {
                        if tx.send(buffer).await.is_err() {
                            return anyhow::Ok(());
                        }
                    }
                }
            }
            anyhow::Ok(())
        })
        .detach();
        rx
    }

    pub fn recalculate_buffer_diffs(
        &mut self,
        buffers: Vec<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = ()> {
        let mut futures = Vec::new();
        for buffer in buffers {
            if let Some(OpenBuffer::Complete {
                change_set_state, ..
            }) = self.opened_buffers.get_mut(&buffer.read(cx).remote_id())
            {
                let buffer = buffer.read(cx).text_snapshot();
                futures.push(change_set_state.update(cx, |change_set_state, cx| {
                    change_set_state.recalculate_diffs(buffer, cx)
                }));
            }
        }
        async move {
            futures::future::join_all(futures).await;
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            BufferEvent::FileHandleChanged => {
                if let Some(local) = self.as_local_mut() {
                    local.buffer_changed_file(buffer, cx);
                }
            }
            BufferEvent::Reloaded => {
                let Some((downstream_client, project_id)) = self.downstream_client.as_ref() else {
                    return;
                };
                let buffer = buffer.read(cx);
                downstream_client
                    .send(proto::BufferReloaded {
                        project_id: *project_id,
                        buffer_id: buffer.remote_id().to_proto(),
                        version: serialize_version(&buffer.version()),
                        mtime: buffer.saved_mtime().map(|t| t.into()),
                        line_ending: serialize_line_ending(buffer.line_ending()) as i32,
                    })
                    .log_err();
            }
            _ => {}
        }
    }

    pub async fn handle_update_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let payload = envelope.payload.clone();
        let buffer_id = BufferId::new(payload.buffer_id)?;
        let ops = payload
            .operations
            .into_iter()
            .map(language::proto::deserialize_operation)
            .collect::<Result<Vec<_>, _>>()?;
        this.update(&mut cx, |this, cx| {
            match this.opened_buffers.entry(buffer_id) {
                hash_map::Entry::Occupied(mut e) => match e.get_mut() {
                    OpenBuffer::Operations(operations) => operations.extend_from_slice(&ops),
                    OpenBuffer::Complete { buffer, .. } => {
                        if let Some(buffer) = buffer.upgrade() {
                            buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx));
                        }
                    }
                },
                hash_map::Entry::Vacant(e) => {
                    e.insert(OpenBuffer::Operations(ops));
                }
            }
            Ok(proto::Ack {})
        })?
    }

    pub fn register_shared_lsp_handle(
        &mut self,
        peer_id: proto::PeerId,
        buffer_id: BufferId,
        handle: OpenLspBufferHandle,
    ) {
        if let Some(shared_buffers) = self.shared_buffers.get_mut(&peer_id) {
            if let Some(buffer) = shared_buffers.get_mut(&buffer_id) {
                buffer.lsp_handle = Some(handle);
                return;
            }
        }
        debug_panic!("tried to register shared lsp handle, but buffer was not shared")
    }

    pub fn handle_synchronize_buffers(
        &mut self,
        envelope: TypedEnvelope<proto::SynchronizeBuffers>,
        cx: &mut Context<Self>,
        client: Arc<Client>,
    ) -> Result<proto::SynchronizeBuffersResponse> {
        let project_id = envelope.payload.project_id;
        let mut response = proto::SynchronizeBuffersResponse {
            buffers: Default::default(),
        };
        let Some(guest_id) = envelope.original_sender_id else {
            anyhow::bail!("missing original_sender_id on SynchronizeBuffers request");
        };

        self.shared_buffers.entry(guest_id).or_default().clear();
        for buffer in envelope.payload.buffers {
            let buffer_id = BufferId::new(buffer.id)?;
            let remote_version = language::proto::deserialize_version(&buffer.version);
            if let Some(buffer) = self.get(buffer_id) {
                self.shared_buffers
                    .entry(guest_id)
                    .or_default()
                    .entry(buffer_id)
                    .or_insert_with(|| SharedBuffer {
                        buffer: buffer.clone(),
                        change_set: None,
                        lsp_handle: None,
                    });

                let buffer = buffer.read(cx);
                response.buffers.push(proto::BufferVersion {
                    id: buffer_id.into(),
                    version: language::proto::serialize_version(&buffer.version),
                });

                let operations = buffer.serialize_ops(Some(remote_version), cx);
                let client = client.clone();
                if let Some(file) = buffer.file() {
                    client
                        .send(proto::UpdateBufferFile {
                            project_id,
                            buffer_id: buffer_id.into(),
                            file: Some(file.to_proto(cx)),
                        })
                        .log_err();
                }

                // TODO(max): do something
                // client
                //     .send(proto::UpdateStagedText {
                //         project_id,
                //         buffer_id: buffer_id.into(),
                //         diff_base: buffer.diff_base().map(ToString::to_string),
                //     })
                //     .log_err();

                client
                    .send(proto::BufferReloaded {
                        project_id,
                        buffer_id: buffer_id.into(),
                        version: language::proto::serialize_version(buffer.saved_version()),
                        mtime: buffer.saved_mtime().map(|time| time.into()),
                        line_ending: language::proto::serialize_line_ending(buffer.line_ending())
                            as i32,
                    })
                    .log_err();

                cx.background_executor()
                    .spawn(
                        async move {
                            let operations = operations.await;
                            for chunk in split_operations(operations) {
                                client
                                    .request(proto::UpdateBuffer {
                                        project_id,
                                        buffer_id: buffer_id.into(),
                                        operations: chunk,
                                    })
                                    .await?;
                            }
                            anyhow::Ok(())
                        }
                        .log_err(),
                    )
                    .detach();
            }
        }
        Ok(response)
    }

    pub fn handle_create_buffer_for_peer(
        &mut self,
        envelope: TypedEnvelope<proto::CreateBufferForPeer>,
        replica_id: u16,
        capability: Capability,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let Some(remote) = self.as_remote_mut() else {
            return Err(anyhow!("buffer store is not a remote"));
        };

        if let Some(buffer) =
            remote.handle_create_buffer_for_peer(envelope, replica_id, capability, cx)?
        {
            self.add_buffer(buffer, cx)?;
        }

        Ok(())
    }

    pub async fn handle_update_buffer_file(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateBufferFile>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let buffer_id = envelope.payload.buffer_id;
        let buffer_id = BufferId::new(buffer_id)?;

        this.update(&mut cx, |this, cx| {
            let payload = envelope.payload.clone();
            if let Some(buffer) = this.get_possibly_incomplete(buffer_id) {
                let file = payload.file.ok_or_else(|| anyhow!("invalid file"))?;
                let worktree = this
                    .worktree_store
                    .read(cx)
                    .worktree_for_id(WorktreeId::from_proto(file.worktree_id), cx)
                    .ok_or_else(|| anyhow!("no such worktree"))?;
                let file = File::from_proto(file, worktree, cx)?;
                let old_file = buffer.update(cx, |buffer, cx| {
                    let old_file = buffer.file().cloned();
                    let new_path = file.path.clone();
                    buffer.file_updated(Arc::new(file), cx);
                    if old_file
                        .as_ref()
                        .map_or(true, |old| *old.path() != new_path)
                    {
                        Some(old_file)
                    } else {
                        None
                    }
                });
                if let Some(old_file) = old_file {
                    cx.emit(BufferStoreEvent::BufferChangedFilePath { buffer, old_file });
                }
            }
            if let Some((downstream_client, project_id)) = this.downstream_client.as_ref() {
                downstream_client
                    .send(proto::UpdateBufferFile {
                        project_id: *project_id,
                        buffer_id: buffer_id.into(),
                        file: envelope.payload.file,
                    })
                    .log_err();
            }
            Ok(())
        })?
    }

    pub async fn handle_save_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::BufferSaved> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let (buffer, project_id) = this.update(&mut cx, |this, _| {
            anyhow::Ok((
                this.get_existing(buffer_id)?,
                this.downstream_client
                    .as_ref()
                    .map(|(_, project_id)| *project_id)
                    .context("project is not shared")?,
            ))
        })??;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&envelope.payload.version))
            })?
            .await?;
        let buffer_id = buffer.update(&mut cx, |buffer, _| buffer.remote_id())?;

        if let Some(new_path) = envelope.payload.new_path {
            let new_path = ProjectPath::from_proto(new_path);
            this.update(&mut cx, |this, cx| {
                this.save_buffer_as(buffer.clone(), new_path, cx)
            })?
            .await?;
        } else {
            this.update(&mut cx, |this, cx| this.save_buffer(buffer.clone(), cx))?
                .await?;
        }

        buffer.update(&mut cx, |buffer, _| proto::BufferSaved {
            project_id,
            buffer_id: buffer_id.into(),
            version: serialize_version(buffer.saved_version()),
            mtime: buffer.saved_mtime().map(|time| time.into()),
        })
    }

    pub async fn handle_close_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let peer_id = envelope.sender_id;
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        this.update(&mut cx, |this, _| {
            if let Some(shared) = this.shared_buffers.get_mut(&peer_id) {
                if shared.remove(&buffer_id).is_some() {
                    if shared.is_empty() {
                        this.shared_buffers.remove(&peer_id);
                    }
                    return;
                }
            }
            debug_panic!(
                "peer_id {} closed buffer_id {} which was either not open or already closed",
                peer_id,
                buffer_id
            )
        })
    }

    pub async fn handle_buffer_saved(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::BufferSaved>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let version = deserialize_version(&envelope.payload.version);
        let mtime = envelope.payload.mtime.clone().map(|time| time.into());
        this.update(&mut cx, move |this, cx| {
            if let Some(buffer) = this.get_possibly_incomplete(buffer_id) {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_save(version, mtime, cx);
                });
            }

            if let Some((downstream_client, project_id)) = this.downstream_client.as_ref() {
                downstream_client
                    .send(proto::BufferSaved {
                        project_id: *project_id,
                        buffer_id: buffer_id.into(),
                        mtime: envelope.payload.mtime,
                        version: envelope.payload.version,
                    })
                    .log_err();
            }
        })
    }

    pub async fn handle_buffer_reloaded(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::BufferReloaded>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let version = deserialize_version(&envelope.payload.version);
        let mtime = envelope.payload.mtime.clone().map(|time| time.into());
        let line_ending = deserialize_line_ending(
            proto::LineEnding::from_i32(envelope.payload.line_ending)
                .ok_or_else(|| anyhow!("missing line ending"))?,
        );
        this.update(&mut cx, |this, cx| {
            if let Some(buffer) = this.get_possibly_incomplete(buffer_id) {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_reload(version, line_ending, mtime, cx);
                });
            }

            if let Some((downstream_client, project_id)) = this.downstream_client.as_ref() {
                downstream_client
                    .send(proto::BufferReloaded {
                        project_id: *project_id,
                        buffer_id: buffer_id.into(),
                        mtime: envelope.payload.mtime,
                        version: envelope.payload.version,
                        line_ending: envelope.payload.line_ending,
                    })
                    .log_err();
            }
        })
    }

    pub async fn handle_blame_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::BlameBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::BlameBufferResponse> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let version = deserialize_version(&envelope.payload.version);
        let buffer = this.read_with(&cx, |this, _| this.get_existing(buffer_id))??;
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

    pub async fn handle_get_permalink_to_line(
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
        let buffer = this.read_with(&cx, |this, _| this.get_existing(buffer_id))??;
        let permalink = this
            .update(&mut cx, |this, cx| {
                this.get_permalink_to_line(&buffer, selection, cx)
            })?
            .await?;
        Ok(proto::GetPermalinkToLineResponse {
            permalink: permalink.to_string(),
        })
    }

    pub async fn handle_open_unstaged_changes(
        this: Entity<Self>,
        request: TypedEnvelope<proto::OpenUnstagedChanges>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenUnstagedChangesResponse> {
        let buffer_id = BufferId::new(request.payload.buffer_id)?;
        let change_set = this
            .update(&mut cx, |this, cx| {
                let buffer = this.get(buffer_id)?;
                Some(this.open_unstaged_changes(buffer, cx))
            })?
            .ok_or_else(|| anyhow!("no such buffer"))?
            .await?;
        this.update(&mut cx, |this, _| {
            let shared_buffers = this
                .shared_buffers
                .entry(request.original_sender_id.unwrap_or(request.sender_id))
                .or_default();
            debug_assert!(shared_buffers.contains_key(&buffer_id));
            if let Some(shared) = shared_buffers.get_mut(&buffer_id) {
                shared.change_set = Some(change_set.clone());
            }
        })?;
        let staged_text = change_set.read_with(&cx, |change_set, _| {
            change_set.base_text.as_ref().map(|buffer| buffer.text())
        })?;
        Ok(proto::OpenUnstagedChangesResponse { staged_text })
    }

    pub async fn handle_open_uncommitted_changes(
        this: Entity<Self>,
        request: TypedEnvelope<proto::OpenUncommittedChanges>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenUncommittedChangesResponse> {
        let buffer_id = BufferId::new(request.payload.buffer_id)?;
        let change_set = this
            .update(&mut cx, |this, cx| {
                let buffer = this.get(buffer_id)?;
                Some(this.open_uncommitted_changes(buffer, cx))
            })?
            .ok_or_else(|| anyhow!("no such buffer"))?
            .await?;
        this.update(&mut cx, |this, _| {
            let shared_buffers = this
                .shared_buffers
                .entry(request.original_sender_id.unwrap_or(request.sender_id))
                .or_default();
            debug_assert!(shared_buffers.contains_key(&buffer_id));
            if let Some(shared) = shared_buffers.get_mut(&buffer_id) {
                shared.change_set = Some(change_set.clone());
            }
        })?;
        change_set.read_with(&cx, |change_set, _| {
            use proto::open_uncommitted_changes_response::Mode;

            let mode;
            let staged_text;
            let committed_text;
            if let Some(committed_buffer) = &change_set.base_text {
                committed_text = Some(committed_buffer.text());
                if let Some(staged_buffer) = &change_set.staged_text {
                    if staged_buffer.remote_id() == committed_buffer.remote_id() {
                        mode = Mode::IndexMatchesHead;
                        staged_text = None;
                    } else {
                        mode = Mode::IndexAndHead;
                        staged_text = Some(staged_buffer.text());
                    }
                } else {
                    mode = Mode::IndexAndHead;
                    staged_text = None;
                }
            } else {
                mode = Mode::IndexAndHead;
                committed_text = None;
                staged_text = change_set.staged_text.as_ref().map(|buffer| buffer.text());
            }

            proto::OpenUncommittedChangesResponse {
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
            if let Some(OpenBuffer::Complete {
                change_set_state,
                buffer,
            }) = this.opened_buffers.get_mut(&buffer_id)
            {
                if let Some(buffer) = buffer.upgrade() {
                    let buffer = buffer.read(cx).text_snapshot();
                    change_set_state.update(cx, |change_set_state, cx| {
                        change_set_state.handle_base_texts_updated(buffer, request.payload, cx);
                    })
                }
            }
        })
    }

    pub fn reload_buffers(
        &self,
        buffers: HashSet<Entity<Buffer>>,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        if buffers.is_empty() {
            return Task::ready(Ok(ProjectTransaction::default()));
        }
        match &self.state {
            BufferStoreState::Local(this) => this.reload_buffers(buffers, push_to_history, cx),
            BufferStoreState::Remote(this) => this.reload_buffers(buffers, push_to_history, cx),
        }
    }

    async fn handle_reload_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ReloadBuffers>,
        mut cx: AsyncApp,
    ) -> Result<proto::ReloadBuffersResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let reload = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(this.get_existing(buffer_id)?);
            }
            Ok::<_, anyhow::Error>(this.reload_buffers(buffers, false, cx))
        })??;

        let project_transaction = reload.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        })?;
        Ok(proto::ReloadBuffersResponse {
            transaction: Some(project_transaction),
        })
    }

    pub fn create_buffer_for_peer(
        &mut self,
        buffer: &Entity<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let buffer_id = buffer.read(cx).remote_id();
        let shared_buffers = self.shared_buffers.entry(peer_id).or_default();
        if shared_buffers.contains_key(&buffer_id) {
            return Task::ready(Ok(()));
        }
        shared_buffers.insert(
            buffer_id,
            SharedBuffer {
                buffer: buffer.clone(),
                change_set: None,
                lsp_handle: None,
            },
        );

        let Some((client, project_id)) = self.downstream_client.clone() else {
            return Task::ready(Ok(()));
        };

        cx.spawn(|this, mut cx| async move {
            let Some(buffer) = this.update(&mut cx, |this, _| this.get(buffer_id))? else {
                return anyhow::Ok(());
            };

            let operations = buffer.update(&mut cx, |b, cx| b.serialize_ops(None, cx))?;
            let operations = operations.await;
            let state = buffer.update(&mut cx, |buffer, cx| buffer.to_proto(cx))?;

            let initial_state = proto::CreateBufferForPeer {
                project_id,
                peer_id: Some(peer_id),
                variant: Some(proto::create_buffer_for_peer::Variant::State(state)),
            };

            if client.send(initial_state).log_err().is_some() {
                let client = client.clone();
                cx.background_executor()
                    .spawn(async move {
                        let mut chunks = split_operations(operations).peekable();
                        while let Some(chunk) = chunks.next() {
                            let is_last = chunks.peek().is_none();
                            client.send(proto::CreateBufferForPeer {
                                project_id,
                                peer_id: Some(peer_id),
                                variant: Some(proto::create_buffer_for_peer::Variant::Chunk(
                                    proto::BufferChunk {
                                        buffer_id: buffer_id.into(),
                                        operations: chunk,
                                        is_last,
                                    },
                                )),
                            })?;
                        }
                        anyhow::Ok(())
                    })
                    .await
                    .log_err();
            }
            Ok(())
        })
    }

    pub fn forget_shared_buffers(&mut self) {
        self.shared_buffers.clear();
    }

    pub fn forget_shared_buffers_for(&mut self, peer_id: &proto::PeerId) {
        self.shared_buffers.remove(peer_id);
    }

    pub fn update_peer_id(&mut self, old_peer_id: &proto::PeerId, new_peer_id: proto::PeerId) {
        if let Some(buffers) = self.shared_buffers.remove(old_peer_id) {
            self.shared_buffers.insert(new_peer_id, buffers);
        }
    }

    pub fn has_shared_buffers(&self) -> bool {
        !self.shared_buffers.is_empty()
    }

    pub fn create_local_buffer(
        &mut self,
        text: &str,
        language: Option<Arc<Language>>,
        cx: &mut Context<Self>,
    ) -> Entity<Buffer> {
        let buffer = cx.new(|cx| {
            Buffer::local(text, cx)
                .with_language(language.unwrap_or_else(|| language::PLAIN_TEXT.clone()), cx)
        });

        self.add_buffer(buffer.clone(), cx).log_err();
        let buffer_id = buffer.read(cx).remote_id();

        let this = self
            .as_local_mut()
            .expect("local-only method called in a non-local context");
        if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
            this.local_buffer_ids_by_path.insert(
                ProjectPath {
                    worktree_id: file.worktree_id(cx),
                    path: file.path.clone(),
                },
                buffer_id,
            );

            if let Some(entry_id) = file.entry_id {
                this.local_buffer_ids_by_entry_id
                    .insert(entry_id, buffer_id);
            }
        }
        buffer
    }

    pub fn deserialize_project_transaction(
        &mut self,
        message: proto::ProjectTransaction,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        if let Some(this) = self.as_remote_mut() {
            this.deserialize_project_transaction(message, push_to_history, cx)
        } else {
            debug_panic!("not a remote buffer store");
            Task::ready(Err(anyhow!("not a remote buffer store")))
        }
    }

    pub fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<Entity<Buffer>>> {
        if let Some(this) = self.as_remote_mut() {
            this.wait_for_remote_buffer(id, cx)
        } else {
            debug_panic!("not a remote buffer store");
            Task::ready(Err(anyhow!("not a remote buffer store")))
        }
    }

    pub fn serialize_project_transaction_for_peer(
        &mut self,
        project_transaction: ProjectTransaction,
        peer_id: proto::PeerId,
        cx: &mut Context<Self>,
    ) -> proto::ProjectTransaction {
        let mut serialized_transaction = proto::ProjectTransaction {
            buffer_ids: Default::default(),
            transactions: Default::default(),
        };
        for (buffer, transaction) in project_transaction.0 {
            self.create_buffer_for_peer(&buffer, peer_id, cx)
                .detach_and_log_err(cx);
            serialized_transaction
                .buffer_ids
                .push(buffer.read(cx).remote_id().into());
            serialized_transaction
                .transactions
                .push(language::proto::serialize_transaction(&transaction));
        }
        serialized_transaction
    }
}

impl EventEmitter<BufferChangeSetEvent> for BufferChangeSet {}

impl BufferChangeSet {
    fn set_state(
        &mut self,
        base_text: Option<language::BufferSnapshot>,
        diff: BufferDiff,
        buffer: &text::BufferSnapshot,
        cx: &mut Context<Self>,
    ) {
        if let Some(base_text) = base_text.as_ref() {
            let changed_range = if Some(base_text.remote_id())
                != self.base_text.as_ref().map(|buffer| buffer.remote_id())
            {
                Some(text::Anchor::MIN..text::Anchor::MAX)
            } else {
                diff.compare(&self.diff_to_buffer, buffer)
            };
            if let Some(changed_range) = changed_range {
                cx.emit(BufferChangeSetEvent::DiffChanged { changed_range });
            }
        }
        self.base_text = base_text;
        self.diff_to_buffer = diff;
    }

    pub fn diff_hunks_intersecting_range<'a>(
        &'a self,
        range: Range<text::Anchor>,
        buffer_snapshot: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = git::diff::DiffHunk> {
        self.diff_to_buffer
            .hunks_intersecting_range(range, buffer_snapshot)
    }

    pub fn diff_hunks_intersecting_range_rev<'a>(
        &'a self,
        range: Range<text::Anchor>,
        buffer_snapshot: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = git::diff::DiffHunk> {
        self.diff_to_buffer
            .hunks_intersecting_range_rev(range, buffer_snapshot)
    }

    /// Used in cases where the change set isn't derived from git.
    pub fn set_base_text(
        &mut self,
        base_buffer: Entity<language::Buffer>,
        buffer: text::BufferSnapshot,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();
        let this = cx.weak_entity();
        let base_buffer = base_buffer.read(cx).snapshot();
        cx.spawn(|_, mut cx| async move {
            let diff = cx
                .background_executor()
                .spawn({
                    let base_buffer = base_buffer.clone();
                    let buffer = buffer.clone();
                    async move { BufferDiff::build(Some(&base_buffer.text()), &buffer) }
                })
                .await;
            let Some(this) = this.upgrade() else {
                tx.send(()).ok();
                return;
            };
            this.update(&mut cx, |this, cx| {
                this.set_state(Some(base_buffer), diff, &buffer, cx);
            })
            .log_err();
            tx.send(()).ok();
        })
        .detach();
        rx
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn base_text_string(&self) -> Option<String> {
        self.base_text.as_ref().map(|buffer| buffer.text())
    }

    pub fn new(buffer: &Entity<Buffer>, cx: &mut App) -> Self {
        BufferChangeSet {
            buffer_id: buffer.read(cx).remote_id(),
            base_text: None,
            diff_to_buffer: BufferDiff::new(&buffer.read(cx).text_snapshot()),
            staged_text: None,
            staged_diff: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn new_with_base_text(base_text: &str, buffer: &Entity<Buffer>, cx: &mut App) -> Self {
        let mut base_text = base_text.to_owned();
        text::LineEnding::normalize(&mut base_text);
        let diff_to_buffer = BufferDiff::build(Some(&base_text), &buffer.read(cx).text_snapshot());
        let base_text = language::Buffer::build_snapshot_sync(base_text.into(), None, None, cx);
        BufferChangeSet {
            buffer_id: buffer.read(cx).remote_id(),
            base_text: Some(base_text),
            diff_to_buffer,
            staged_text: None,
            staged_diff: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn recalculate_diff_sync(
        &mut self,
        snapshot: text::BufferSnapshot,
        cx: &mut Context<Self>,
    ) {
        let mut base_text = self.base_text.as_ref().map(|buffer| buffer.text());
        if let Some(base_text) = base_text.as_mut() {
            text::LineEnding::normalize(base_text);
        }
        let diff_to_buffer = BufferDiff::build(base_text.as_deref(), &snapshot);
        self.set_state(self.base_text.clone(), diff_to_buffer, &snapshot, cx);
    }
}

impl OpenBuffer {
    fn upgrade(&self) -> Option<Entity<Buffer>> {
        match self {
            OpenBuffer::Complete { buffer, .. } => buffer.upgrade(),
            OpenBuffer::Operations(_) => None,
        }
    }
}

fn is_not_found_error(error: &anyhow::Error) -> bool {
    error
        .root_cause()
        .downcast_ref::<io::Error>()
        .is_some_and(|err| err.kind() == io::ErrorKind::NotFound)
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
            committer: entry.committer.clone(),
            committer_mail: entry.committer_mail.clone(),
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

    let permalinks = blame
        .permalinks
        .into_iter()
        .map(|(oid, url)| proto::CommitPermalink {
            oid: oid.as_bytes().into(),
            permalink: url.to_string(),
        })
        .collect::<Vec<_>>();

    proto::BlameBufferResponse {
        blame_response: Some(proto::blame_buffer_response::BlameResponse {
            entries,
            messages,
            permalinks,
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
                committer: entry.committer,
                committer_time: entry.committer_time,
                committer_tz: entry.committer_tz,
                committer_mail: entry.committer_mail,
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

    let permalinks = response
        .permalinks
        .into_iter()
        .filter_map(|permalink| {
            Some((
                git::Oid::from_bytes(&permalink.oid).ok()?,
                Url::from_str(&permalink.permalink).ok()?,
            ))
        })
        .collect::<HashMap<_, _>>();

    Some(Blame {
        entries,
        permalinks,
        messages,
        remote_url: response.remote_url,
    })
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
