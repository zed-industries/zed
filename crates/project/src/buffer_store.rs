use crate::ProjectPath;
use anyhow::{anyhow, Context as _, Result};
use collections::{hash_map, HashMap};
use futures::{channel::oneshot, StreamExt as _};
use gpui::{
    AppContext, AsyncAppContext, Context as _, EventEmitter, Model, ModelContext, Task, WeakModel,
};
use language::{
    proto::{deserialize_version, serialize_version, split_operations},
    Buffer, Capability, Language, Operation,
};
use rpc::{
    proto::{self, AnyProtoClient, PeerId},
    ErrorExt as _, TypedEnvelope,
};
use std::{io, path::Path, sync::Arc};
use text::BufferId;
use util::{debug_panic, maybe, ResultExt as _};
use worktree::{File, ProjectEntryId, RemoteWorktree, Worktree};

/// A set of open buffers.
pub struct BufferStore {
    retain_buffers: bool,
    opened_buffers: HashMap<BufferId, OpenBuffer>,
    local_buffer_ids_by_path: HashMap<ProjectPath, BufferId>,
    local_buffer_ids_by_entry_id: HashMap<ProjectEntryId, BufferId>,
    #[allow(clippy::type_complexity)]
    loading_buffers_by_path: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<Model<Buffer>, Arc<anyhow::Error>>>>,
    >,
    loading_remote_buffers_by_id: HashMap<BufferId, Model<Buffer>>,
    remote_buffer_listeners:
        HashMap<BufferId, Vec<oneshot::Sender<Result<Model<Buffer>, anyhow::Error>>>>,
}

enum OpenBuffer {
    Strong(Model<Buffer>),
    Weak(WeakModel<Buffer>),
    Operations(Vec<Operation>),
}

pub enum BufferStoreEvent {
    BufferAdded(Model<Buffer>),
    BufferChangedFilePath {
        buffer: Model<Buffer>,
        old_file: Option<Arc<File>>,
    },
    BufferSaved {
        buffer: Model<Buffer>,
        has_changed_file: bool,
        saved_version: clock::Global,
    },
}

impl EventEmitter<BufferStoreEvent> for BufferStore {}

impl BufferStore {
    /// Creates a buffer store, optionally retaining its buffers.
    ///
    /// If `retain_buffers` is `true`, then buffers are owned by the buffer store
    /// and won't be released unless they are explicitly removed, or `retain_buffers`
    /// is set to `false` via `set_retain_buffers`. Otherwise, buffers are stored as
    /// weak handles.
    pub fn new(retain_buffers: bool) -> Self {
        Self {
            retain_buffers,
            opened_buffers: Default::default(),
            remote_buffer_listeners: Default::default(),
            loading_remote_buffers_by_id: Default::default(),
            local_buffer_ids_by_path: Default::default(),
            local_buffer_ids_by_entry_id: Default::default(),
            loading_buffers_by_path: Default::default(),
        }
    }

    pub fn open_buffer(
        &mut self,
        project_path: ProjectPath,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        let existing_buffer = self.get_by_path(&project_path, cx);
        if let Some(existing_buffer) = existing_buffer {
            return Task::ready(Ok(existing_buffer));
        }

        let loading_watch = match self.loading_buffers_by_path.entry(project_path.clone()) {
            // If the given path is already being loaded, then wait for that existing
            // task to complete and return the same buffer.
            hash_map::Entry::Occupied(e) => e.get().clone(),

            // Otherwise, record the fact that this path is now being loaded.
            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let project_path = project_path.clone();
                let load_buffer = match worktree.read(cx) {
                    Worktree::Local(_) => {
                        self.open_local_buffer_internal(project_path.path.clone(), worktree, cx)
                    }
                    Worktree::Remote(tree) => {
                        self.open_remote_buffer_internal(&project_path.path, tree, cx)
                    }
                };

                cx.spawn(move |this, mut cx| async move {
                    let load_result = load_buffer.await;
                    *tx.borrow_mut() = Some(this.update(&mut cx, |this, _| {
                        // Record the fact that the buffer is no longer loading.
                        this.loading_buffers_by_path.remove(&project_path);
                        let buffer = load_result.map_err(Arc::new)?;
                        Ok(buffer)
                    })?);
                    anyhow::Ok(())
                })
                .detach();
                rx
            }
        };

        cx.background_executor().spawn(async move {
            Self::wait_for_loading_buffer(loading_watch)
                .await
                .map_err(|e| e.cloned())
        })
    }

    fn open_local_buffer_internal(
        &mut self,
        path: Arc<Path>,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        let load_buffer = worktree.update(cx, |worktree, cx| {
            let load_file = worktree.load_file(path.as_ref(), cx);
            let reservation = cx.reserve_model();
            let buffer_id = BufferId::from(reservation.entity_id().as_non_zero_u64());
            cx.spawn(move |_, mut cx| async move {
                let loaded = load_file.await?;
                let text_buffer = cx
                    .background_executor()
                    .spawn(async move { text::Buffer::new(0, buffer_id, loaded.text) })
                    .await;
                cx.insert_model(reservation, |_| {
                    Buffer::build(
                        text_buffer,
                        loaded.diff_base,
                        Some(loaded.file),
                        Capability::ReadWrite,
                    )
                })
            })
        });

        cx.spawn(move |this, mut cx| async move {
            let buffer = match load_buffer.await {
                Ok(buffer) => Ok(buffer),
                Err(error) if is_not_found_error(&error) => cx.new_model(|cx| {
                    let buffer_id = BufferId::from(cx.entity_id().as_non_zero_u64());
                    let text_buffer = text::Buffer::new(0, buffer_id, "".into());
                    Buffer::build(
                        text_buffer,
                        None,
                        Some(Arc::new(File {
                            worktree,
                            path,
                            mtime: None,
                            entry_id: None,
                            is_local: true,
                            is_deleted: false,
                            is_private: false,
                        })),
                        Capability::ReadWrite,
                    )
                }),
                Err(e) => Err(e),
            }?;
            this.update(&mut cx, |this, cx| {
                this.add_buffer(buffer.clone(), cx).log_err();
            })?;
            Ok(buffer)
        })
    }

    fn open_remote_buffer_internal(
        &self,
        path: &Arc<Path>,
        worktree: &RemoteWorktree,
        cx: &ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        let worktree_id = worktree.id().to_proto();
        let project_id = worktree.project_id();
        let client = worktree.client();
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
            this.update(&mut cx, |this, cx| {
                this.wait_for_remote_buffer(buffer_id, cx)
            })?
            .await
        })
    }

    pub fn create_buffer(
        &mut self,
        remote_client: Option<(AnyProtoClient, u64)>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        if let Some((remote_client, project_id)) = remote_client {
            let create = remote_client.request(proto::OpenNewBuffer { project_id });
            cx.spawn(|this, mut cx| async move {
                let response = create.await?;
                let buffer_id = BufferId::new(response.buffer_id)?;

                this.update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(buffer_id, cx)
                })?
                .await
            })
        } else {
            Task::ready(Ok(self.create_local_buffer("", None, cx)))
        }
    }

    pub fn create_local_buffer(
        &mut self,
        text: &str,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Model<Buffer> {
        let buffer = cx.new_model(|cx| {
            Buffer::local(text, cx)
                .with_language(language.unwrap_or_else(|| language::PLAIN_TEXT.clone()), cx)
        });
        self.add_buffer(buffer.clone(), cx).log_err();
        buffer
    }

    pub fn save_buffer(
        &mut self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(file) = File::from_dyn(buffer.read(cx).file()) else {
            return Task::ready(Err(anyhow!("buffer doesn't have a file")));
        };
        match file.worktree.read(cx) {
            Worktree::Local(_) => {
                self.save_local_buffer(file.worktree.clone(), buffer, file.path.clone(), false, cx)
            }
            Worktree::Remote(tree) => self.save_remote_buffer(buffer, None, tree, cx),
        }
    }

    pub fn save_buffer_as(
        &mut self,
        buffer: Model<Buffer>,
        path: ProjectPath,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let old_file = File::from_dyn(buffer.read(cx).file())
            .cloned()
            .map(Arc::new);

        let task = match worktree.read(cx) {
            Worktree::Local(_) => {
                self.save_local_buffer(worktree, buffer.clone(), path.path, true, cx)
            }
            Worktree::Remote(tree) => {
                self.save_remote_buffer(buffer.clone(), Some(path.to_proto()), tree, cx)
            }
        };
        cx.spawn(|this, mut cx| async move {
            task.await?;
            this.update(&mut cx, |_, cx| {
                cx.emit(BufferStoreEvent::BufferChangedFilePath { buffer, old_file });
            })
        })
    }

    fn save_local_buffer(
        &self,
        worktree: Model<Worktree>,
        buffer_handle: Model<Buffer>,
        path: Arc<Path>,
        mut has_changed_file: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);
        let text = buffer.as_rope().clone();
        let line_ending = buffer.line_ending();
        let version = buffer.version();
        if buffer.file().is_some_and(|file| !file.is_created()) {
            has_changed_file = true;
        }

        let save = worktree.update(cx, |worktree, cx| {
            worktree.write_file(path.as_ref(), text, line_ending, cx)
        });

        cx.spawn(move |this, mut cx| async move {
            let new_file = save.await?;
            let mtime = new_file.mtime;
            buffer_handle.update(&mut cx, |buffer, cx| {
                if has_changed_file {
                    buffer.file_updated(new_file, cx);
                }
                buffer.did_save(version.clone(), mtime, cx);
            })?;
            this.update(&mut cx, |_, cx| {
                cx.emit(BufferStoreEvent::BufferSaved {
                    buffer: buffer_handle,
                    has_changed_file,
                    saved_version: version,
                })
            })?;
            Ok(())
        })
    }

    fn save_remote_buffer(
        &self,
        buffer_handle: Model<Buffer>,
        new_path: Option<proto::ProjectPath>,
        tree: &RemoteWorktree,
        cx: &ModelContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id().into();
        let version = buffer.version();
        let rpc = tree.client();
        let project_id = tree.project_id();
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

    fn add_buffer(&mut self, buffer: Model<Buffer>, cx: &mut ModelContext<Self>) -> Result<()> {
        let remote_id = buffer.read(cx).remote_id();
        let is_remote = buffer.read(cx).replica_id() != 0;
        let open_buffer = if self.retain_buffers {
            OpenBuffer::Strong(buffer.clone())
        } else {
            OpenBuffer::Weak(buffer.downgrade())
        };

        match self.opened_buffers.entry(remote_id) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(open_buffer);
            }
            hash_map::Entry::Occupied(mut entry) => {
                if let OpenBuffer::Operations(operations) = entry.get_mut() {
                    buffer.update(cx, |b, cx| b.apply_ops(operations.drain(..), cx))?;
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

        if let Some(senders) = self.remote_buffer_listeners.remove(&remote_id) {
            for sender in senders {
                sender.send(Ok(buffer.clone())).ok();
            }
        }

        if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
            if file.is_local {
                self.local_buffer_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path.clone(),
                    },
                    remote_id,
                );

                if let Some(entry_id) = file.entry_id {
                    self.local_buffer_ids_by_entry_id
                        .insert(entry_id, remote_id);
                }
            }
        }

        cx.emit(BufferStoreEvent::BufferAdded(buffer));
        Ok(())
    }

    pub fn buffers(&self) -> impl '_ + Iterator<Item = Model<Buffer>> {
        self.opened_buffers
            .values()
            .filter_map(|buffer| buffer.upgrade())
    }

    pub fn loading_buffers(
        &self,
    ) -> impl Iterator<
        Item = (
            &ProjectPath,
            postage::watch::Receiver<Option<Result<Model<Buffer>, Arc<anyhow::Error>>>>,
        ),
    > {
        self.loading_buffers_by_path
            .iter()
            .map(|(path, rx)| (path, rx.clone()))
    }

    pub fn get_by_path(&self, path: &ProjectPath, cx: &AppContext) -> Option<Model<Buffer>> {
        self.buffers().find_map(|buffer| {
            let file = File::from_dyn(buffer.read(cx).file())?;
            if file.worktree_id(cx) == path.worktree_id && &file.path == &path.path {
                Some(buffer)
            } else {
                None
            }
        })
    }

    pub fn get(&self, buffer_id: BufferId) -> Option<Model<Buffer>> {
        self.opened_buffers
            .get(&buffer_id)
            .and_then(|buffer| buffer.upgrade())
    }

    pub fn get_existing(&self, buffer_id: BufferId) -> Result<Model<Buffer>> {
        self.get(buffer_id)
            .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))
    }

    pub fn get_possibly_incomplete(&self, buffer_id: BufferId) -> Option<Model<Buffer>> {
        self.get(buffer_id)
            .or_else(|| self.loading_remote_buffers_by_id.get(&buffer_id).cloned())
    }

    fn get_or_remove_by_path(
        &mut self,
        entry_id: ProjectEntryId,
        project_path: &ProjectPath,
    ) -> Option<(BufferId, Model<Buffer>)> {
        let buffer_id = match self.local_buffer_ids_by_entry_id.get(&entry_id) {
            Some(&buffer_id) => buffer_id,
            None => match self.local_buffer_ids_by_path.get(project_path) {
                Some(&buffer_id) => buffer_id,
                None => {
                    return None;
                }
            },
        };
        let buffer = if let Some(buffer) = self.get(buffer_id) {
            buffer
        } else {
            self.opened_buffers.remove(&buffer_id);
            self.local_buffer_ids_by_path.remove(project_path);
            self.local_buffer_ids_by_entry_id.remove(&entry_id);
            return None;
        };
        Some((buffer_id, buffer))
    }

    pub fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Buffer>>> {
        let buffer = self.get(id);
        if let Some(buffer) = buffer {
            return Task::ready(Ok(buffer));
        }
        let (tx, rx) = oneshot::channel();
        self.remote_buffer_listeners.entry(id).or_default().push(tx);
        cx.background_executor().spawn(async move { rx.await? })
    }

    pub fn buffer_version_info(
        &self,
        cx: &AppContext,
    ) -> (Vec<proto::BufferVersion>, Vec<BufferId>) {
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
            .loading_remote_buffers_by_id
            .keys()
            .copied()
            .collect::<Vec<_>>();
        (buffers, incomplete_buffer_ids)
    }

    pub fn disconnected_from_host(&mut self, cx: &mut AppContext) {
        self.set_retain_buffers(false, cx);

        for buffer in self.buffers() {
            buffer.update(cx, |buffer, cx| {
                buffer.set_capability(Capability::ReadOnly, cx)
            });
        }

        // Wake up all futures currently waiting on a buffer to get opened,
        // to give them a chance to fail now that we've disconnected.
        self.remote_buffer_listeners.clear();
    }

    pub fn set_retain_buffers(&mut self, retain_buffers: bool, cx: &mut AppContext) {
        self.retain_buffers = retain_buffers;
        for open_buffer in self.opened_buffers.values_mut() {
            if retain_buffers {
                if let OpenBuffer::Weak(buffer) = open_buffer {
                    if let Some(buffer) = buffer.upgrade() {
                        *open_buffer = OpenBuffer::Strong(buffer);
                    }
                }
            } else {
                if let Some(buffer) = open_buffer.upgrade() {
                    buffer.update(cx, |buffer, _| buffer.give_up_waiting());
                }
                if let OpenBuffer::Strong(buffer) = open_buffer {
                    *open_buffer = OpenBuffer::Weak(buffer.downgrade());
                }
            }
        }
    }

    pub fn discard_incomplete(&mut self) {
        self.opened_buffers
            .retain(|_, buffer| !matches!(buffer, OpenBuffer::Operations(_)));
    }

    pub fn file_changed(
        &mut self,
        path: Arc<Path>,
        entry_id: ProjectEntryId,
        worktree_handle: &Model<worktree::Worktree>,
        snapshot: &worktree::Snapshot,
        cx: &mut ModelContext<Self>,
    ) -> Option<(Model<Buffer>, Arc<File>, Arc<File>)> {
        let (buffer_id, buffer) = self.get_or_remove_by_path(
            entry_id,
            &ProjectPath {
                worktree_id: snapshot.id(),
                path,
            },
        )?;

        let result = buffer.update(cx, |buffer, cx| {
            let old_file = File::from_dyn(buffer.file())?;
            if old_file.worktree != *worktree_handle {
                return None;
            }

            let new_file = if let Some(entry) = old_file
                .entry_id
                .and_then(|entry_id| snapshot.entry_for_id(entry_id))
            {
                File {
                    is_local: true,
                    entry_id: Some(entry.id),
                    mtime: entry.mtime,
                    path: entry.path.clone(),
                    worktree: worktree_handle.clone(),
                    is_deleted: false,
                    is_private: entry.is_private,
                }
            } else if let Some(entry) = snapshot.entry_for_path(old_file.path.as_ref()) {
                File {
                    is_local: true,
                    entry_id: Some(entry.id),
                    mtime: entry.mtime,
                    path: entry.path.clone(),
                    worktree: worktree_handle.clone(),
                    is_deleted: false,
                    is_private: entry.is_private,
                }
            } else {
                File {
                    is_local: true,
                    entry_id: old_file.entry_id,
                    path: old_file.path.clone(),
                    mtime: old_file.mtime,
                    worktree: worktree_handle.clone(),
                    is_deleted: true,
                    is_private: old_file.is_private,
                }
            };

            if new_file == *old_file {
                return None;
            }

            let old_file = Arc::new(old_file.clone());
            let new_file = Arc::new(new_file);
            buffer.file_updated(new_file.clone(), cx);
            Some((cx.handle(), old_file, new_file))
        });

        if let Some((buffer, old_file, new_file)) = &result {
            if new_file.path != old_file.path {
                self.local_buffer_ids_by_path.remove(&ProjectPath {
                    path: old_file.path.clone(),
                    worktree_id: old_file.worktree_id(cx),
                });
                self.local_buffer_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: new_file.worktree_id(cx),
                        path: new_file.path.clone(),
                    },
                    buffer_id,
                );
                cx.emit(BufferStoreEvent::BufferChangedFilePath {
                    buffer: buffer.clone(),
                    old_file: Some(old_file.clone()),
                });
            }

            if new_file.entry_id != old_file.entry_id {
                if let Some(entry_id) = old_file.entry_id {
                    self.local_buffer_ids_by_entry_id.remove(&entry_id);
                }
                if let Some(entry_id) = new_file.entry_id {
                    self.local_buffer_ids_by_entry_id
                        .insert(entry_id, buffer_id);
                }
            }
        }

        result
    }

    pub fn buffer_changed_file(
        &mut self,
        buffer: Model<Buffer>,
        cx: &mut AppContext,
    ) -> Option<()> {
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

    pub async fn create_buffer_for_peer(
        this: Model<Self>,
        peer_id: PeerId,
        buffer_id: BufferId,
        project_id: u64,
        client: AnyProtoClient,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let Some(buffer) = this.update(cx, |this, _| this.get(buffer_id))? else {
            return Ok(());
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
    }

    pub fn handle_update_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        is_remote: bool,
        cx: &mut AppContext,
    ) -> Result<proto::Ack> {
        let payload = envelope.payload.clone();
        let buffer_id = BufferId::new(payload.buffer_id)?;
        let ops = payload
            .operations
            .into_iter()
            .map(language::proto::deserialize_operation)
            .collect::<Result<Vec<_>, _>>()?;
        match self.opened_buffers.entry(buffer_id) {
            hash_map::Entry::Occupied(mut e) => match e.get_mut() {
                OpenBuffer::Strong(buffer) => {
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
                }
                OpenBuffer::Operations(operations) => operations.extend_from_slice(&ops),
                OpenBuffer::Weak(_) => {}
            },
            hash_map::Entry::Vacant(e) => {
                if !is_remote {
                    debug_panic!(
                        "received buffer update from {:?}",
                        envelope.original_sender_id
                    );
                    return Err(anyhow!("received buffer update for non-remote project"));
                }
                e.insert(OpenBuffer::Operations(ops));
            }
        }
        Ok(proto::Ack {})
    }

    pub fn handle_create_buffer_for_peer(
        &mut self,
        envelope: TypedEnvelope<proto::CreateBufferForPeer>,
        mut worktrees: impl Iterator<Item = Model<Worktree>>,
        replica_id: u16,
        capability: Capability,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
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
                        let worktree = worktrees
                            .find(|worktree| worktree.read(cx).id() == worktree_id)
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
                        let buffer = cx.new_model(|_| buffer);
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
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(operations, cx))
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
                    self.add_buffer(buffer, cx)?;
                }
            }
        }

        Ok(())
    }

    pub async fn handle_save_buffer(
        this: Model<Self>,
        project_id: u64,
        worktree: Option<Model<Worktree>>,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::BufferSaved> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let buffer = this.update(&mut cx, |this, _| this.get_existing(buffer_id))??;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&envelope.payload.version))
            })?
            .await?;
        let buffer_id = buffer.update(&mut cx, |buffer, _| buffer.remote_id())?;

        if let Some(new_path) = envelope.payload.new_path {
            let worktree = worktree.context("no such worktree")?;
            let new_path = ProjectPath::from_proto(new_path);
            this.update(&mut cx, |this, cx| {
                this.save_buffer_as(buffer.clone(), new_path, worktree, cx)
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

    pub async fn wait_for_loading_buffer(
        mut receiver: postage::watch::Receiver<Option<Result<Model<Buffer>, Arc<anyhow::Error>>>>,
    ) -> Result<Model<Buffer>, Arc<anyhow::Error>> {
        loop {
            if let Some(result) = receiver.borrow().as_ref() {
                match result {
                    Ok(buffer) => return Ok(buffer.to_owned()),
                    Err(e) => return Err(e.to_owned()),
                }
            }
            receiver.next().await;
        }
    }
}

impl OpenBuffer {
    fn upgrade(&self) -> Option<Model<Buffer>> {
        match self {
            OpenBuffer::Strong(handle) => Some(handle.clone()),
            OpenBuffer::Weak(handle) => handle.upgrade(),
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
