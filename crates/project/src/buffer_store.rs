use crate::{
    ProjectPath,
    lsp_store::OpenLspBufferHandle,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};
use anyhow::{Context as _, Result, anyhow};

use collections::{HashMap, HashSet, hash_map};
use futures::{Future, FutureExt as _, channel::oneshot, future::Shared};
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity,
};
use language::{
    Buffer, BufferEvent, Capability, DiskState, File as _, Language, LineEnding, Operation,
    language_settings::{AllLanguageSettings, LineEndingSetting},
    proto::{deserialize_line_ending, deserialize_version, serialize_version},
};
use rpc::{
    AnyProtoClient, ErrorCode, ErrorExt as _, TypedEnvelope,
    proto::{self, PeerId},
};

use settings::Settings;
use std::{io, sync::Arc, time::Instant};
use text::{BufferId, ReplicaId};
use util::{ResultExt as _, debug_panic, maybe, rel_path::RelPath};
use worktree::{File, PathChange, ProjectEntryId, Worktree, WorktreeId, WorktreeSettings};

/// A set of open buffers.
pub struct BufferStore {
    state: BufferStoreState,
    #[allow(clippy::type_complexity)]
    loading_buffers: HashMap<ProjectPath, Shared<Task<Result<Entity<Buffer>, Arc<anyhow::Error>>>>>,
    worktree_store: Entity<WorktreeStore>,
    opened_buffers: HashMap<BufferId, OpenBuffer>,
    path_to_buffer_id: HashMap<ProjectPath, BufferId>,
    non_searchable_buffers: HashSet<BufferId>,
    project_search: RemoteProjectSearchState,
}

#[derive(Default)]
struct RemoteProjectSearchState {
    // List of ongoing project search chunks from our remote host. Used by the side issuing a search RPC request.
    chunks: HashMap<u64, async_channel::Sender<BufferId>>,
    // Monotonously-increasing handle to hand out to remote host in order to identify the project search result chunk.
    next_id: u64,
    // Used by the side running the actual search for match candidates to potentially cancel the search prematurely.
    searches_in_progress: HashMap<(PeerId, u64), Task<Result<()>>>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SharedBuffer {
    pub buffer: Entity<Buffer>,
    pub lsp_handle: Option<OpenLspBufferHandle>,
}

/// Abstracts the per-peer buffer-sharing operations that used to live on
/// `BufferStore` and now live on `Project` / `HeadlessProject`. Both impl
/// this trait so that shared LSP serialization paths
/// (`LspCommand::response_to_proto_project`, the cascade rpc handlers)
/// can call into either one without distinguishing them.
pub trait PeerBufferAccess {
    /// Streams a buffer to a collaborator and records that the peer has it.
    fn create_buffer_for_peer(
        &mut self,
        buffer: &Entity<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> Task<Result<()>>;

    /// Serializes a `ProjectTransaction` for a peer, also calling
    /// `create_buffer_for_peer` for each touched buffer.
    fn serialize_project_transaction_for_peer(
        &mut self,
        project_transaction: ProjectTransaction,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> proto::ProjectTransaction;
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
        HashMap<BufferId, Vec<oneshot::Sender<anyhow::Result<Entity<Buffer>>>>>,
    worktree_store: Entity<WorktreeStore>,
}

struct LocalBufferStore {
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
    SharedBufferClosed(proto::PeerId, BufferId),
    BufferDropped(BufferId),
    BufferChangedFilePath {
        buffer: Entity<Buffer>,
        old_file: Option<Arc<dyn language::File>>,
    },
    /// Emitted by inbound rpc handlers after applying a peer-originated
    /// `UpdateBufferFile`. Listeners (`Project`, `HeadlessProject`) forward to
    /// other peers by re-broadcasting the proto.
    UpdateBufferFileForwarded {
        buffer_id: BufferId,
        file: Option<rpc::proto::File>,
    },
    /// Emitted by inbound rpc handlers after applying a peer-originated
    /// `BufferSaved`. Listeners forward to other peers.
    BufferSavedForwarded {
        buffer_id: BufferId,
        version: Vec<rpc::proto::VectorClockEntry>,
        mtime: Option<rpc::proto::Timestamp>,
    },
    /// Emitted by inbound rpc handlers after applying a peer-originated
    /// `BufferReloaded`. Listeners forward to other peers.
    BufferReloadedForwarded {
        buffer_id: BufferId,
        version: Vec<rpc::proto::VectorClockEntry>,
        mtime: Option<rpc::proto::Timestamp>,
        line_ending: i32,
    },
    /// Emitted by `on_buffer_event` when the local buffer reloads. Listeners
    /// broadcast `BufferReloaded` downstream when shared.
    LocalBufferReloaded(Entity<Buffer>),
}

#[derive(Default, Debug, Clone)]
pub struct ProjectTransaction(pub HashMap<Entity<Buffer>, language::Transaction>);

impl PartialEq for ProjectTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self.0.iter().all(|(buffer, transaction)| {
                other.0.get(buffer).is_some_and(|t| t.id == transaction.id)
            })
    }
}

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
            });

            Ok(())
        })
    }

    pub fn handle_create_buffer_for_peer(
        &mut self,
        envelope: TypedEnvelope<proto::CreateBufferForPeer>,
        replica_id: ReplicaId,
        capability: Capability,
        cx: &mut Context<BufferStore>,
    ) -> Result<Option<Entity<Buffer>>> {
        match envelope.payload.variant.context("missing variant")? {
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
                            .with_context(|| {
                                format!("no worktree found for id {}", file.worktree_id)
                            })?;
                        buffer_file = Some(Arc::new(File::from_proto(file, worktree, cx)?)
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
                    .with_context(|| {
                        format!(
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
        Ok(None)
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
                    })
                    .await?;

                if push_to_history {
                    buffer.update(cx, |buffer, _| {
                        buffer.push_transaction(transaction.clone(), Instant::now());
                        buffer.finalize_last_transaction();
                    });
                }
            }

            Ok(project_transaction)
        })
    }

    fn open_buffer(
        &self,
        path: Arc<RelPath>,
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

    fn create_buffer(
        &self,
        language: Option<Arc<Language>>,
        project_searchable: bool,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<Entity<Buffer>>> {
        let create = self.upstream_client.request(proto::OpenNewBuffer {
            project_id: self.project_id,
        });
        cx.spawn(async move |this, cx| {
            let response = create.await?;
            let buffer_id = BufferId::new(response.buffer_id)?;

            let buffer = this
                .update(cx, |this, cx| {
                    if !project_searchable {
                        this.non_searchable_buffers.insert(buffer_id);
                    }
                    this.wait_for_remote_buffer(buffer_id, cx)
                })?
                .await?;
            if let Some(language) = language {
                buffer.update(cx, |buffer, cx| {
                    buffer.set_language(Some(language), cx);
                });
            }
            Ok(buffer)
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
            let response = request.await?.transaction.context("missing transaction")?;
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
        path: Arc<RelPath>,
        mut has_changed_file: bool,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);

        let text = buffer.as_rope().clone();
        let line_ending = buffer.line_ending();
        let encoding = buffer.encoding();
        let has_bom = buffer.has_bom();
        let version = buffer.version();
        let buffer_id = buffer.remote_id();
        let file = buffer.file().cloned();
        if file
            .as_ref()
            .is_some_and(|file| file.disk_state() == DiskState::New)
        {
            has_changed_file = true;
        }

        let save = worktree.update(cx, |worktree, cx| {
            worktree.write_file(path, text, line_ending, encoding, has_bom, cx)
        });

        cx.spawn(async move |this, cx| {
            let new_file = save.await?;
            let mtime = new_file.disk_state().mtime();
            this.update(cx, |_, cx| {
                if has_changed_file {
                    cx.emit(BufferStoreEvent::UpdateBufferFileForwarded {
                        buffer_id,
                        file: Some(language::File::to_proto(&*new_file, cx)),
                    });
                }
                cx.emit(BufferStoreEvent::BufferSavedForwarded {
                    buffer_id,
                    version: serialize_version(&version),
                    mtime: mtime.map(|time| time.into()),
                });
            })?;
            buffer_handle.update(cx, |buffer, cx| {
                if has_changed_file {
                    buffer.file_updated(new_file, cx);
                }
                buffer.did_save(version.clone(), mtime, cx);
            });
            Ok(())
        })
    }

    fn subscribe_to_worktree(
        &mut self,
        worktree: &Entity<Worktree>,
        cx: &mut Context<BufferStore>,
    ) {
        cx.subscribe(worktree, |this, worktree, event, cx| {
            if worktree.read(cx).is_local()
                && let worktree::Event::UpdatedEntries(changes) = event
            {
                Self::local_worktree_entries_changed(this, &worktree, changes, cx);
            }
        })
        .detach();
    }

    fn local_worktree_entries_changed(
        this: &mut BufferStore,
        worktree_handle: &Entity<Worktree>,
        changes: &[(Arc<RelPath>, ProjectEntryId, PathChange)],
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
        path: &Arc<RelPath>,
        worktree: &Entity<worktree::Worktree>,
        snapshot: &worktree::Snapshot,
        cx: &mut Context<BufferStore>,
    ) -> Option<()> {
        let project_path = ProjectPath {
            worktree_id: snapshot.id(),
            path: path.clone(),
        };

        let buffer_id = this
            .as_local_mut()
            .and_then(|local| local.local_buffer_ids_by_entry_id.get(&entry_id))
            .copied()
            .or_else(|| this.path_to_buffer_id.get(&project_path).copied())?;

        let buffer = if let Some(buffer) = this.get(buffer_id) {
            Some(buffer)
        } else {
            this.opened_buffers.remove(&buffer_id);
            this.non_searchable_buffers.remove(&buffer_id);
            None
        };

        let buffer = if let Some(buffer) = buffer {
            buffer
        } else {
            this.path_to_buffer_id.remove(&project_path);
            let this = this.as_local_mut()?;
            this.local_buffer_ids_by_entry_id.remove(&entry_id);
            return None;
        };

        let events = buffer.update(cx, |buffer, cx| {
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
                        Some(mtime) => DiskState::Present {
                            mtime,
                            size: entry.size,
                        },
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
                this.path_to_buffer_id.remove(&ProjectPath {
                    path: old_file.path.clone(),
                    worktree_id: old_file.worktree_id(cx),
                });
                this.path_to_buffer_id.insert(
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
            let local = this.as_local_mut()?;
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

            events.push(BufferStoreEvent::UpdateBufferFileForwarded {
                buffer_id,
                file: Some(new_file.to_proto(cx)),
            });

            buffer.file_updated(Arc::new(new_file), cx);
            Some(events)
        })?;

        for event in events {
            cx.emit(event);
        }

        None
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
        self.save_local_buffer(buffer, worktree, path.path, true, cx)
    }

    #[ztracing::instrument(skip_all)]
    fn open_buffer(
        &self,
        path: Arc<RelPath>,
        worktree: Entity<Worktree>,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<Entity<Buffer>>> {
        let load_file = worktree.update(cx, |worktree, cx| worktree.load_file(path.as_ref(), cx));
        cx.spawn(async move |this, cx| {
            let path = path.clone();
            let buffer = match load_file.await {
                Ok(loaded) => {
                    let reservation = cx.reserve_entity::<Buffer>();
                    let buffer_id = BufferId::from(reservation.entity_id().as_non_zero_u64());
                    let text_buffer = cx
                        .background_spawn(async move {
                            text::Buffer::new(ReplicaId::LOCAL, buffer_id, loaded.text)
                        })
                        .await;
                    cx.insert_entity(reservation, |_| {
                        let mut buffer =
                            Buffer::build(text_buffer, Some(loaded.file), Capability::ReadWrite);
                        buffer.set_encoding(loaded.encoding);
                        buffer.set_has_bom(loaded.has_bom);
                        buffer
                    })
                }
                Err(error) if is_not_found_error(&error) => cx.new(|cx| {
                    let buffer_id = BufferId::from(cx.entity_id().as_non_zero_u64());
                    let text_buffer = text::Buffer::new(ReplicaId::LOCAL, buffer_id, "");
                    let mut buffer = Buffer::build(
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
                    );
                    apply_initial_line_ending(&mut buffer, cx);
                    buffer
                }),
                Err(e) => return Err(e),
            };
            this.update(cx, |this, cx| {
                this.add_buffer(buffer.clone(), cx)?;
                let buffer_id = buffer.read(cx).remote_id();
                if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
                    let project_path = ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path.clone(),
                    };
                    let entry_id = file.entry_id;

                    // Check if the file should be read-only based on settings
                    let settings = WorktreeSettings::get(Some((&project_path).into()), cx);
                    let is_read_only = if project_path.path.is_empty() {
                        settings.is_std_path_read_only(&file.full_path(cx))
                    } else {
                        settings.is_path_read_only(&project_path.path)
                    };
                    if is_read_only {
                        buffer.update(cx, |buffer, cx| {
                            buffer.set_capability(Capability::Read, cx);
                        });
                    }

                    this.path_to_buffer_id.insert(project_path, buffer_id);
                    let this = this.as_local_mut().unwrap();
                    if let Some(entry_id) = entry_id {
                        this.local_buffer_ids_by_entry_id
                            .insert(entry_id, buffer_id);
                    }
                }

                anyhow::Ok(())
            })??;

            Ok(buffer)
        })
    }

    fn create_buffer(
        &self,
        language: Option<Arc<Language>>,
        project_searchable: bool,
        cx: &mut Context<BufferStore>,
    ) -> Task<Result<Entity<Buffer>>> {
        cx.spawn(async move |buffer_store, cx| {
            let buffer = cx.new(|cx| {
                let mut buffer = Buffer::local("", cx)
                    .with_language(language.unwrap_or_else(|| language::PLAIN_TEXT.clone()), cx);
                apply_initial_line_ending(&mut buffer, cx);
                buffer
            });
            buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.add_buffer(buffer.clone(), cx).log_err();
                if !project_searchable {
                    buffer_store
                        .non_searchable_buffers
                        .insert(buffer.read(cx).remote_id());
                }
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
                let transaction = buffer.update(cx, |buffer, cx| buffer.reload(cx)).await?;
                buffer.update(cx, |buffer, cx| {
                    if let Some(transaction) = transaction {
                        if !push_to_history {
                            buffer.forget_transaction(transaction.id);
                        }
                        project_transaction.0.insert(cx.entity(), transaction);
                    }
                });
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
        // handle_synchronize_buffers / handle_close_buffer / handle_reload_buffers
        // moved to `Project` in Phase 1; they mutate the per-project
        // `shared_buffers` map.
    }

    /// Creates a buffer store, optionally retaining its buffers.
    pub fn local(worktree_store: Entity<WorktreeStore>, cx: &mut Context<Self>) -> Self {
        Self {
            state: BufferStoreState::Local(LocalBufferStore {
                local_buffer_ids_by_entry_id: Default::default(),
                worktree_store: worktree_store.clone(),
                _subscription: cx.subscribe(&worktree_store, |this, _, event, cx| {
                    if let WorktreeStoreEvent::WorktreeAdded(worktree) = event {
                        let this = this.as_local_mut().unwrap();
                        this.subscribe_to_worktree(worktree, cx);
                    }
                }),
            }),
            opened_buffers: Default::default(),
            path_to_buffer_id: Default::default(),
            loading_buffers: Default::default(),
            non_searchable_buffers: Default::default(),
            worktree_store,
            project_search: Default::default(),
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
            opened_buffers: Default::default(),
            path_to_buffer_id: Default::default(),
            loading_buffers: Default::default(),
            non_searchable_buffers: Default::default(),
            worktree_store,
            project_search: Default::default(),
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

    #[ztracing::instrument(skip_all)]
    pub fn open_buffer(
        &mut self,
        project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        if let Some(buffer) = self.get_by_path(&project_path) {
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
                            this.update(cx, |this, _cx| {
                                // Record the fact that the buffer is no longer loading.
                                this.loading_buffers.remove(&project_path);

                                let buffer = load_result.map_err(Arc::new)?;
                                Ok(buffer)
                            })?
                        })
                        .shared(),
                    )
                    .clone()
            }
        };

        cx.background_spawn(async move {
            task.await.map_err(|e| {
                if e.error_code() != ErrorCode::Internal {
                    anyhow!(e.error_code())
                } else {
                    anyhow!("{e}")
                }
            })
        })
    }

    pub fn create_buffer(
        &mut self,
        language: Option<Arc<Language>>,
        project_searchable: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        match &self.state {
            BufferStoreState::Local(this) => this.create_buffer(language, project_searchable, cx),
            BufferStoreState::Remote(this) => this.create_buffer(language, project_searchable, cx),
        }
    }

    pub fn save_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        match &mut self.state {
            BufferStoreState::Local(this) => this.save_buffer(buffer, cx),
            BufferStoreState::Remote(this) => this.save_remote_buffer(buffer, None, cx),
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
            this.update(cx, |this, cx| {
                old_file.clone().and_then(|file| {
                    this.path_to_buffer_id.remove(&ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path().clone(),
                    })
                });

                cx.emit(BufferStoreEvent::BufferChangedFilePath { buffer, old_file });
            })
        })
    }

    fn add_buffer(&mut self, buffer_entity: Entity<Buffer>, cx: &mut Context<Self>) -> Result<()> {
        let buffer = buffer_entity.read(cx);
        let remote_id = buffer.remote_id();
        let path = File::from_dyn(buffer.file()).map(|file| ProjectPath {
            path: file.path.clone(),
            worktree_id: file.worktree_id(cx),
        });
        let is_remote = buffer.replica_id().is_remote();
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
        let _expect_path_to_exist;
        match self.opened_buffers.entry(remote_id) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(open_buffer);
                _expect_path_to_exist = false;
            }
            hash_map::Entry::Occupied(mut entry) => {
                if let OpenBuffer::Operations(operations) = entry.get_mut() {
                    buffer_entity.update(cx, |b, cx| b.apply_ops(operations.drain(..), cx));
                } else if entry.get().upgrade().is_some() {
                    if is_remote {
                        return Ok(());
                    } else {
                        debug_panic!("buffer {remote_id} was already registered");
                        anyhow::bail!("buffer {remote_id} was already registered");
                    }
                }
                entry.insert(open_buffer);
                _expect_path_to_exist = true;
            }
        }

        if let Some(path) = path {
            self.path_to_buffer_id.insert(path, remote_id);
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

    pub(crate) fn is_searchable(&self, id: &BufferId) -> bool {
        !self.non_searchable_buffers.contains(&id)
    }

    pub fn loading_buffers(
        &self,
    ) -> impl Iterator<Item = (&ProjectPath, impl Future<Output = Result<Entity<Buffer>>>)> {
        self.loading_buffers.iter().map(|(path, task)| {
            let task = task.clone();
            (path, async move {
                task.await.map_err(|e| {
                    if e.error_code() != ErrorCode::Internal {
                        anyhow!(e.error_code())
                    } else {
                        anyhow!("{e}")
                    }
                })
            })
        })
    }

    pub fn buffer_id_for_project_path(&self, project_path: &ProjectPath) -> Option<&BufferId> {
        self.path_to_buffer_id.get(project_path)
    }

    pub fn get_by_path(&self, path: &ProjectPath) -> Option<Entity<Buffer>> {
        self.path_to_buffer_id
            .get(path)
            .and_then(|buffer_id| self.get(*buffer_id))
    }

    pub fn get(&self, buffer_id: BufferId) -> Option<Entity<Buffer>> {
        self.opened_buffers.get(&buffer_id)?.upgrade()
    }

    pub fn get_existing(&self, buffer_id: BufferId) -> Result<Entity<Buffer>> {
        self.get(buffer_id)
            .with_context(|| format!("unknown buffer id {buffer_id}"))
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

    pub fn discard_incomplete(&mut self) {
        self.opened_buffers
            .retain(|_, buffer| !matches!(buffer, OpenBuffer::Operations(_)));
    }

    fn buffer_changed_file(&mut self, buffer: Entity<Buffer>, cx: &mut App) -> Option<()> {
        let file = File::from_dyn(buffer.read(cx).file())?;

        let remote_id = buffer.read(cx).remote_id();
        if let Some(entry_id) = file.entry_id {
            if let Some(local) = self.as_local_mut() {
                match local.local_buffer_ids_by_entry_id.get(&entry_id) {
                    Some(_) => {
                        return None;
                    }
                    None => {
                        local
                            .local_buffer_ids_by_entry_id
                            .insert(entry_id, remote_id);
                    }
                }
            }
            self.path_to_buffer_id.insert(
                ProjectPath {
                    worktree_id: file.worktree_id(cx),
                    path: file.path.clone(),
                },
                remote_id,
            );
        };

        Some(())
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            BufferEvent::FileHandleChanged => {
                self.buffer_changed_file(buffer, cx);
            }
            BufferEvent::Reloaded => {
                cx.emit(BufferStoreEvent::LocalBufferReloaded(buffer));
            }
            BufferEvent::LanguageChanged(_) => {}
            _ => {}
        }
    }

    pub async fn handle_update_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let payload = envelope.payload;
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
        })
    }

    pub fn handle_create_buffer_for_peer(
        &mut self,
        envelope: TypedEnvelope<proto::CreateBufferForPeer>,
        replica_id: ReplicaId,
        capability: Capability,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let remote = self
            .as_remote_mut()
            .context("buffer store is not a remote")?;

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
                let file = payload.file.context("invalid file")?;
                let worktree = this
                    .worktree_store
                    .read(cx)
                    .worktree_for_id(WorktreeId::from_proto(file.worktree_id), cx)
                    .context("no such worktree")?;
                let file = File::from_proto(file, worktree, cx)?;
                let old_file = buffer.update(cx, |buffer, cx| {
                    let old_file = buffer.file().cloned();
                    let new_path = file.path.clone();

                    buffer.file_updated(Arc::new(file), cx);
                    if old_file.as_ref().is_none_or(|old| *old.path() != new_path) {
                        Some(old_file)
                    } else {
                        None
                    }
                });
                if let Some(old_file) = old_file {
                    cx.emit(BufferStoreEvent::BufferChangedFilePath { buffer, old_file });
                }
            }
            cx.emit(BufferStoreEvent::UpdateBufferFileForwarded {
                buffer_id,
                file: envelope.payload.file,
            });
            Ok(())
        })
    }

    pub async fn handle_save_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::BufferSaved> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let project_id = envelope.payload.project_id;
        let buffer = this.read_with(&cx, |this, _| this.get_existing(buffer_id))?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&envelope.payload.version))
            })
            .await?;
        let buffer_id = buffer.read_with(&cx, |buffer, _| buffer.remote_id());

        if let Some(new_path) = envelope.payload.new_path
            && let Some(new_path) = ProjectPath::from_proto(new_path)
        {
            this.update(&mut cx, |this, cx| {
                this.save_buffer_as(buffer.clone(), new_path, cx)
            })
            .await?;
        } else {
            this.update(&mut cx, |this, cx| this.save_buffer(buffer.clone(), cx))
                .await?;
        }

        Ok(buffer.read_with(&cx, |buffer, _| proto::BufferSaved {
            project_id,
            buffer_id: buffer_id.into(),
            version: serialize_version(buffer.saved_version()),
            mtime: buffer.saved_mtime().map(|time| time.into()),
        }))
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

            cx.emit(BufferStoreEvent::BufferSavedForwarded {
                buffer_id,
                version: envelope.payload.version,
                mtime: envelope.payload.mtime,
            });
        });
        Ok(())
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
                .context("missing line ending")?,
        );
        this.update(&mut cx, |this, cx| {
            if let Some(buffer) = this.get_possibly_incomplete(buffer_id) {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_reload(version, line_ending, mtime, cx);
                });
            }

            cx.emit(BufferStoreEvent::BufferReloadedForwarded {
                buffer_id,
                version: envelope.payload.version,
                mtime: envelope.payload.mtime,
                line_ending: envelope.payload.line_ending,
            });
        });
        Ok(())
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

    pub fn create_local_buffer(
        &mut self,
        text: &str,
        language: Option<Arc<Language>>,
        project_searchable: bool,
        cx: &mut Context<Self>,
    ) -> Entity<Buffer> {
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(text, cx)
                .with_language(language.unwrap_or_else(|| language::PLAIN_TEXT.clone()), cx);
            apply_initial_line_ending(&mut buffer, cx);
            buffer
        });

        self.add_buffer(buffer.clone(), cx).log_err();
        let buffer_id = buffer.read(cx).remote_id();
        if !project_searchable {
            self.non_searchable_buffers.insert(buffer_id);
        }

        if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
            self.path_to_buffer_id.insert(
                ProjectPath {
                    worktree_id: file.worktree_id(cx),
                    path: file.path.clone(),
                },
                buffer_id,
            );
            let this = self
                .as_local_mut()
                .expect("local-only method called in a non-local context");
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

    pub(crate) fn register_project_search_result_handle(
        &mut self,
    ) -> (u64, async_channel::Receiver<BufferId>) {
        let (tx, rx) = async_channel::unbounded();
        let handle = util::post_inc(&mut self.project_search.next_id);
        let _old_entry = self.project_search.chunks.insert(handle, tx);
        debug_assert!(_old_entry.is_none());
        (handle, rx)
    }

    pub fn register_ongoing_project_search(
        &mut self,
        id: (PeerId, u64),
        search: Task<anyhow::Result<()>>,
    ) {
        let _old = self.project_search.searches_in_progress.insert(id, search);
        debug_assert!(_old.is_none());
    }

    pub async fn handle_find_search_candidates_cancel(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidatesCancelled>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let id = (
            envelope.original_sender_id.unwrap_or(envelope.sender_id),
            envelope.payload.handle,
        );
        let _ = this.update(&mut cx, |this, _| {
            this.project_search.searches_in_progress.remove(&id)
        });
        Ok(())
    }

    pub(crate) async fn handle_find_search_candidates_chunk(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidatesChunk>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        use proto::find_search_candidates_chunk::Variant;
        let handle = envelope.payload.handle;

        let buffer_ids = match envelope
            .payload
            .variant
            .context("Expected non-null variant")?
        {
            Variant::Matches(find_search_candidates_matches) => find_search_candidates_matches
                .buffer_ids
                .into_iter()
                .filter_map(|buffer_id| BufferId::new(buffer_id).ok())
                .collect::<Vec<_>>(),
            Variant::Done(_) => {
                this.update(&mut cx, |this, _| {
                    this.project_search.chunks.remove(&handle)
                });
                return Ok(proto::Ack {});
            }
        };
        let Some(sender) = this.read_with(&mut cx, |this, _| {
            this.project_search.chunks.get(&handle).cloned()
        }) else {
            return Ok(proto::Ack {});
        };

        for buffer_id in buffer_ids {
            let Ok(_) = sender.send(buffer_id).await else {
                this.update(&mut cx, |this, _| {
                    this.project_search.chunks.remove(&handle)
                });
                return Ok(proto::Ack {});
            };
        }
        Ok(proto::Ack {})
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

fn apply_initial_line_ending(buffer: &mut Buffer, cx: &mut Context<Buffer>) {
    // Only applies for empty rope or a single line with no trailing newline.
    if buffer.max_point().row > 0 {
        return;
    }
    let location = buffer.file().map(|file| settings::SettingsLocation {
        worktree_id: file.worktree_id(cx),
        path: file.path().as_ref(),
    });
    let language = buffer.language().map(|l| l.name());
    let settings = AllLanguageSettings::get(location, cx).language(location, language.as_ref(), cx);
    let desired = match settings.line_ending {
        LineEndingSetting::Detect => return,
        LineEndingSetting::PreferLf | LineEndingSetting::EnforceLf => LineEnding::Unix,
        LineEndingSetting::PreferCrlf | LineEndingSetting::EnforceCrlf => LineEnding::Windows,
    };
    if buffer.line_ending() != desired {
        buffer.set_line_ending(desired, cx);
    }
}
