use crate::{
    lsp_store::OpenLspBufferHandle,
    search::SearchQuery,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    ProjectItem as _, ProjectPath,
};
use anyhow::{anyhow, Context as _, Result};
use client::Client;
use collections::{hash_map, HashMap, HashSet};
use fs::Fs;
use futures::{channel::oneshot, future::Shared, Future, FutureExt as _, StreamExt};
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity,
};
use language::{
    proto::{
        deserialize_line_ending, deserialize_version, serialize_line_ending, serialize_version,
        split_operations,
    },
    Buffer, BufferEvent, Capability, DiskState, File as _, Language, Operation,
};
use rpc::{
    proto::{self, ToProto},
    AnyProtoClient, ErrorExt as _, TypedEnvelope,
};
use smol::channel::Receiver;
use std::{io, path::Path, pin::pin, sync::Arc, time::Instant};
use text::BufferId;
use util::{debug_panic, maybe, ResultExt as _, TryFutureExt};
use worktree::{File, PathChange, ProjectEntryId, Worktree, WorktreeId};

/// A set of open buffers.
pub struct BufferStore {
    state: BufferStoreState,
    #[allow(clippy::type_complexity)]
    loading_buffers: HashMap<ProjectPath, Shared<Task<Result<Entity<Buffer>, Arc<anyhow::Error>>>>>,
    worktree_store: Entity<WorktreeStore>,
    opened_buffers: HashMap<BufferId, OpenBuffer>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    shared_buffers: HashMap<proto::PeerId, HashMap<BufferId, SharedBuffer>>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct SharedBuffer {
    buffer: Entity<Buffer>,
    lsp_handle: Option<OpenLspBufferHandle>,
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
    Complete { buffer: WeakEntity<Buffer> },
    Operations(Vec<Operation>),
}

pub enum BufferStoreEvent {
    BufferAdded(Entity<Buffer>),
    BufferOpened {
        buffer: Entity<Buffer>,
        project_path: ProjectPath,
    },
    SharedBufferClosed(proto::PeerId, BufferId),
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
    pub fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<Entity<Buffer>>> {
        let (tx, rx) = oneshot::channel();
        self.remote_buffer_listeners.entry(id).or_default().push(tx);

        cx.spawn(async move |this, cx| {
            if let Some(buffer) = this
                .read_with(cx, |buffer_store, _| buffer_store.get(id))
                .ok()
                .flatten()
            {
                return Ok(buffer);
            }

            cx.background_spawn(async move { rx.await? }).await
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
        cx.spawn(async move |_, cx| {
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

            buffer_handle.update(cx, |buffer, cx| {
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
        cx.spawn(async move |this, cx| {
            let mut project_transaction = ProjectTransaction::default();
            for (buffer_id, transaction) in message.buffer_ids.into_iter().zip(message.transactions)
            {
                let buffer_id = BufferId::new(buffer_id)?;
                let buffer = this
                    .update(cx, |this, cx| this.wait_for_remote_buffer(buffer_id, cx))?
                    .await?;
                let transaction = language::proto::deserialize_transaction(transaction)?;
                project_transaction.0.insert(buffer, transaction);
            }

            for (buffer, transaction) in &project_transaction.0 {
                buffer
                    .update(cx, |buffer, _| {
                        buffer.wait_for_edits(transaction.edit_ids.iter().copied())
                    })?
                    .await?;

                if push_to_history {
                    buffer.update(cx, |buffer, _| {
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
        cx.spawn(async move |this, cx| {
            let response = client
                .request(proto::OpenBufferByPath {
                    project_id,
                    worktree_id,
                    path: path.to_proto(),
                })
                .await?;
            let buffer_id = BufferId::new(response.buffer_id)?;

            let buffer = this
                .update(cx, {
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
        cx.spawn(async move |this, cx| {
            let response = create.await?;
            let buffer_id = BufferId::new(response.buffer_id)?;

            this.update(cx, |this, cx| this.wait_for_remote_buffer(buffer_id, cx))?
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

        cx.spawn(async move |this, cx| {
            let response = request
                .await?
                .transaction
                .ok_or_else(|| anyhow!("missing transaction"))?;
            this.update(cx, |this, cx| {
                this.deserialize_project_transaction(response, push_to_history, cx)
            })?
            .await
        })
    }
}

impl LocalBufferStore {
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

        cx.spawn(async move |this, cx| {
            let new_file = save.await?;
            let mtime = new_file.disk_state().mtime();
            this.update(cx, |this, cx| {
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
            buffer_handle.update(cx, |buffer, cx| {
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
            cx.spawn(async move |_, cx| {
                let loaded = load_file.await?;
                let text_buffer = cx
                    .background_spawn(async move { text::Buffer::new(0, buffer_id, loaded.text) })
                    .await;
                cx.insert_entity(reservation, |_| {
                    Buffer::build(text_buffer, Some(loaded.file), Capability::ReadWrite)
                })
            })
        });

        cx.spawn(async move |this, cx| {
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
            this.update(cx, |this, cx| {
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
        cx.spawn(async move |buffer_store, cx| {
            let buffer =
                cx.new(|cx| Buffer::local("", cx).with_language(language::PLAIN_TEXT.clone(), cx))?;
            buffer_store.update(cx, |buffer_store, cx| {
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
        cx.spawn(async move |_, cx| {
            let mut project_transaction = ProjectTransaction::default();
            for buffer in buffers {
                let transaction = buffer.update(cx, |buffer, cx| buffer.reload(cx))?.await?;
                buffer.update(cx, |buffer, cx| {
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
        client.add_entity_request_handler(Self::handle_reload_buffers);
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
            shared_buffers: Default::default(),
            worktree_store,
        }
    }

    fn as_local(&self) -> Option<&LocalBufferStore> {
        match &self.state {
            BufferStoreState::Local(state) => Some(state),
            _ => None,
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
            cx.emit(BufferStoreEvent::BufferOpened {
                buffer: buffer.clone(),
                project_path,
            });

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
                        cx.spawn(async move |this, cx| {
                            let load_result = load_buffer.await;
                            this.update(cx, |this, cx| {
                                // Record the fact that the buffer is no longer loading.
                                this.loading_buffers.remove(&project_path);

                                let buffer = load_result.map_err(Arc::new)?;
                                cx.emit(BufferStoreEvent::BufferOpened {
                                    buffer: buffer.clone(),
                                    project_path,
                                });

                                Ok(buffer)
                            })?
                        })
                        .shared(),
                    )
                    .clone()
            }
        };

        cx.background_spawn(async move { task.await.map_err(|e| anyhow!("{e}")) })
    }

    pub(crate) fn worktree_for_buffer(
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
        cx.spawn(async move |this, cx| {
            task.await?;
            this.update(cx, |_, cx| {
                cx.emit(BufferStoreEvent::BufferChangedFilePath { buffer, old_file });
            })
        })
    }

    fn add_buffer(&mut self, buffer_entity: Entity<Buffer>, cx: &mut Context<Self>) -> Result<()> {
        let buffer = buffer_entity.read(cx);
        let remote_id = buffer.remote_id();
        let is_remote = buffer.replica_id() != 0;
        let open_buffer = OpenBuffer::Complete {
            buffer: buffer_entity.downgrade(),
        };

        let handle = cx.entity().downgrade();
        buffer_entity.update(cx, move |_, cx| {
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
                    buffer_entity.update(cx, |b, cx| b.apply_ops(operations.drain(..), cx));
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

        cx.subscribe(&buffer_entity, Self::on_buffer_event).detach();
        cx.emit(BufferStoreEvent::BufferAdded(buffer_entity));
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

    pub fn buffer_id_for_project_path(&self, project_path: &ProjectPath) -> Option<&BufferId> {
        self.as_local()
            .and_then(|state| state.local_buffer_ids_by_path.get(project_path))
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

        cx.spawn(async move |this, cx| {
            for buffer in unnamed_buffers {
                tx.send(buffer).await.ok();
            }

            let mut project_paths_rx = pin!(project_paths_rx);
            while let Some(project_paths) = project_paths_rx.next().await {
                let buffers = this.update(cx, |this, cx| {
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
            BufferEvent::LanguageChanged => {}
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

                cx.background_spawn(
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
        this.update(&mut cx, |this, cx| {
            if let Some(shared) = this.shared_buffers.get_mut(&peer_id) {
                if shared.remove(&buffer_id).is_some() {
                    cx.emit(BufferStoreEvent::SharedBufferClosed(peer_id, buffer_id));
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
                lsp_handle: None,
            },
        );

        let Some((client, project_id)) = self.downstream_client.clone() else {
            return Task::ready(Ok(()));
        };

        cx.spawn(async move |this, cx| {
            let Some(buffer) = this.update(cx, |this, _| this.get(buffer_id))? else {
                return anyhow::Ok(());
            };

            let operations = buffer.update(cx, |b, cx| b.serialize_ops(None, cx))?;
            let operations = operations.await;
            let state = buffer.update(cx, |buffer, cx| buffer.to_proto(cx))?;

            let initial_state = proto::CreateBufferForPeer {
                project_id,
                peer_id: Some(peer_id),
                variant: Some(proto::create_buffer_for_peer::Variant::State(state)),
            };

            if client.send(initial_state).log_err().is_some() {
                let client = client.clone();
                cx.background_spawn(async move {
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
