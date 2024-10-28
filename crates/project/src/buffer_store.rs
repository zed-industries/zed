use crate::{
    search::SearchQuery,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    Item, NoRepositoryError, ProjectPath,
};
use ::git::{parse_git_remote_url, BuildPermalinkParams, GitHostingProviderRegistry};
use anyhow::{anyhow, Context as _, Result};
use client::Client;
use collections::{hash_map, HashMap, HashSet};
use fs::Fs;
use futures::{channel::oneshot, stream::FuturesUnordered, StreamExt};
use git::blame::Blame;
use gpui::{
    AppContext, AsyncAppContext, Context as _, EventEmitter, Model, ModelContext, Subscription,
    Task, WeakModel,
};
use http_client::Url;
use language::{
    proto::{
        deserialize_line_ending, deserialize_version, serialize_line_ending, serialize_version,
        split_operations,
    },
    Buffer, BufferEvent, Capability, File as _, Language, Operation,
};
use rpc::{proto, AnyProtoClient, ErrorExt as _, TypedEnvelope};
use smol::channel::Receiver;
use std::{io, ops::Range, path::Path, str::FromStr as _, sync::Arc, time::Instant};
use text::BufferId;
use util::{debug_panic, maybe, ResultExt as _, TryFutureExt};
use worktree::{File, PathChange, ProjectEntryId, UpdatedGitRepositoriesSet, Worktree, WorktreeId};

trait BufferStoreImpl {
    fn open_buffer(
        &self,
        path: Arc<Path>,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<Model<Buffer>>>;

    fn save_buffer(
        &self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<()>>;

    fn save_buffer_as(
        &self,
        buffer: Model<Buffer>,
        path: ProjectPath,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<()>>;

    fn create_buffer(&self, cx: &mut ModelContext<BufferStore>) -> Task<Result<Model<Buffer>>>;

    fn reload_buffers(
        &self,
        buffers: HashSet<Model<Buffer>>,
        push_to_history: bool,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<ProjectTransaction>>;

    fn as_remote(&self) -> Option<Model<RemoteBufferStore>>;
    fn as_local(&self) -> Option<Model<LocalBufferStore>>;
}

struct RemoteBufferStore {
    shared_with_me: HashSet<Model<Buffer>>,
    upstream_client: AnyProtoClient,
    project_id: u64,
    loading_remote_buffers_by_id: HashMap<BufferId, Model<Buffer>>,
    remote_buffer_listeners:
        HashMap<BufferId, Vec<oneshot::Sender<Result<Model<Buffer>, anyhow::Error>>>>,
    worktree_store: Model<WorktreeStore>,
    buffer_store: WeakModel<BufferStore>,
}

struct LocalBufferStore {
    local_buffer_ids_by_path: HashMap<ProjectPath, BufferId>,
    local_buffer_ids_by_entry_id: HashMap<ProjectEntryId, BufferId>,
    buffer_store: WeakModel<BufferStore>,
    worktree_store: Model<WorktreeStore>,
    _subscription: Subscription,
}

/// A set of open buffers.
pub struct BufferStore {
    state: Box<dyn BufferStoreImpl>,
    #[allow(clippy::type_complexity)]
    loading_buffers_by_path: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<Model<Buffer>, Arc<anyhow::Error>>>>,
    >,
    worktree_store: Model<WorktreeStore>,
    opened_buffers: HashMap<BufferId, OpenBuffer>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    shared_buffers: HashMap<proto::PeerId, HashSet<Model<Buffer>>>,
}

enum OpenBuffer {
    Buffer(WeakModel<Buffer>),
    Operations(Vec<Operation>),
}

pub enum BufferStoreEvent {
    BufferAdded(Model<Buffer>),
    BufferDropped(BufferId),
    BufferChangedFilePath {
        buffer: Model<Buffer>,
        old_file: Option<Arc<dyn language::File>>,
    },
}

#[derive(Default, Debug)]
pub struct ProjectTransaction(pub HashMap<Model<Buffer>, language::Transaction>);

impl EventEmitter<BufferStoreEvent> for BufferStore {}

impl RemoteBufferStore {
    pub fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Buffer>>> {
        let buffer_store = self.buffer_store.clone();
        let (tx, rx) = oneshot::channel();
        self.remote_buffer_listeners.entry(id).or_default().push(tx);

        cx.spawn(|cx| async move {
            if let Some(buffer) = buffer_store
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
        buffer_handle: Model<Buffer>,
        new_path: Option<proto::ProjectPath>,
        cx: &ModelContext<Self>,
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
        cx: &mut ModelContext<Self>,
    ) -> Result<Option<Model<Buffer>>> {
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
        cx: &mut ModelContext<Self>,
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
}

impl BufferStoreImpl for Model<RemoteBufferStore> {
    fn as_remote(&self) -> Option<Model<RemoteBufferStore>> {
        Some(self.clone())
    }

    fn as_local(&self) -> Option<Model<LocalBufferStore>> {
        None
    }

    fn save_buffer(
        &self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<()>> {
        self.update(cx, |this, cx| {
            this.save_remote_buffer(buffer.clone(), None, cx)
        })
    }
    fn save_buffer_as(
        &self,
        buffer: Model<Buffer>,
        path: ProjectPath,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<()>> {
        self.update(cx, |this, cx| {
            this.save_remote_buffer(buffer, Some(path.to_proto()), cx)
        })
    }

    fn open_buffer(
        &self,
        path: Arc<Path>,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<Model<Buffer>>> {
        self.update(cx, |this, cx| {
            let worktree_id = worktree.read(cx).id().to_proto();
            let project_id = this.project_id;
            let client = this.upstream_client.clone();
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
        })
    }

    fn create_buffer(&self, cx: &mut ModelContext<BufferStore>) -> Task<Result<Model<Buffer>>> {
        self.update(cx, |this, cx| {
            let create = this.upstream_client.request(proto::OpenNewBuffer {
                project_id: this.project_id,
            });
            cx.spawn(|this, mut cx| async move {
                let response = create.await?;
                let buffer_id = BufferId::new(response.buffer_id)?;

                this.update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(buffer_id, cx)
                })?
                .await
            })
        })
    }

    fn reload_buffers(
        &self,
        buffers: HashSet<Model<Buffer>>,
        push_to_history: bool,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<ProjectTransaction>> {
        self.update(cx, |this, cx| {
            let request = this.upstream_client.request(proto::ReloadBuffers {
                project_id: this.project_id,
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
        })
    }
}

impl LocalBufferStore {
    fn save_local_buffer(
        &self,
        buffer_handle: Model<Buffer>,
        worktree: Model<Worktree>,
        path: Arc<Path>,
        mut has_changed_file: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);

        let text = buffer.as_rope().clone();
        let line_ending = buffer.line_ending();
        let version = buffer.version();
        let buffer_id = buffer.remote_id();
        if buffer.file().is_some_and(|file| !file.is_created()) {
            has_changed_file = true;
        }

        let save = worktree.update(cx, |worktree, cx| {
            worktree.write_file(path.as_ref(), text, line_ending, cx)
        });

        cx.spawn(move |this, mut cx| async move {
            let new_file = save.await?;
            let mtime = new_file.mtime;
            this.update(&mut cx, |this, cx| {
                if let Some((downstream_client, project_id)) = this.downstream_client(cx) {
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

    fn subscribe_to_worktree(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
        cx.subscribe(worktree, |this, worktree, event, cx| {
            if worktree.read(cx).is_local() {
                match event {
                    worktree::Event::UpdatedEntries(changes) => {
                        this.local_worktree_entries_changed(&worktree, changes, cx);
                    }
                    worktree::Event::UpdatedGitRepositories(updated_repos) => {
                        this.local_worktree_git_repos_changed(worktree.clone(), updated_repos, cx)
                    }
                    _ => {}
                }
            }
        })
        .detach();
    }

    fn local_worktree_entries_changed(
        &mut self,
        worktree_handle: &Model<Worktree>,
        changes: &[(Arc<Path>, ProjectEntryId, PathChange)],
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();
        for (path, entry_id, _) in changes {
            self.local_worktree_entry_changed(*entry_id, path, worktree_handle, &snapshot, cx);
        }
    }

    fn local_worktree_git_repos_changed(
        &mut self,
        worktree_handle: Model<Worktree>,
        changed_repos: &UpdatedGitRepositoriesSet,
        cx: &mut ModelContext<Self>,
    ) {
        debug_assert!(worktree_handle.read(cx).is_local());
        let Some(buffer_store) = self.buffer_store.upgrade() else {
            return;
        };

        // Identify the loading buffers whose containing repository that has changed.
        let (future_buffers, current_buffers) = buffer_store.update(cx, |buffer_store, cx| {
            let future_buffers = buffer_store
                .loading_buffers()
                .filter_map(|(project_path, receiver)| {
                    if project_path.worktree_id != worktree_handle.read(cx).id() {
                        return None;
                    }
                    let path = &project_path.path;
                    changed_repos
                        .iter()
                        .find(|(work_dir, _)| path.starts_with(work_dir))?;
                    let path = path.clone();
                    Some(async move {
                        BufferStore::wait_for_loading_buffer(receiver)
                            .await
                            .ok()
                            .map(|buffer| (buffer, path))
                    })
                })
                .collect::<FuturesUnordered<_>>();

            // Identify the current buffers whose containing repository has changed.
            let current_buffers = buffer_store
                .buffers()
                .filter_map(|buffer| {
                    let file = File::from_dyn(buffer.read(cx).file())?;
                    if file.worktree != worktree_handle {
                        return None;
                    }
                    changed_repos
                        .iter()
                        .find(|(work_dir, _)| file.path.starts_with(work_dir))?;
                    Some((buffer, file.path.clone()))
                })
                .collect::<Vec<_>>();
            (future_buffers, current_buffers)
        });

        if future_buffers.len() + current_buffers.len() == 0 {
            return;
        }

        cx.spawn(move |this, mut cx| async move {
            // Wait for all of the buffers to load.
            let future_buffers = future_buffers.collect::<Vec<_>>().await;

            // Reload the diff base for every buffer whose containing git repository has changed.
            let snapshot =
                worktree_handle.update(&mut cx, |tree, _| tree.as_local().unwrap().snapshot())?;
            let diff_bases_by_buffer = cx
                .background_executor()
                .spawn(async move {
                    let mut diff_base_tasks = future_buffers
                        .into_iter()
                        .flatten()
                        .chain(current_buffers)
                        .filter_map(|(buffer, path)| {
                            let (repo_entry, local_repo_entry) = snapshot.repo_for_path(&path)?;
                            let relative_path = repo_entry.relativize(&snapshot, &path).ok()?;
                            Some(async move {
                                let base_text =
                                    local_repo_entry.repo().load_index_text(&relative_path);
                                Some((buffer, base_text))
                            })
                        })
                        .collect::<FuturesUnordered<_>>();

                    let mut diff_bases = Vec::with_capacity(diff_base_tasks.len());
                    while let Some(diff_base) = diff_base_tasks.next().await {
                        if let Some(diff_base) = diff_base {
                            diff_bases.push(diff_base);
                        }
                    }
                    diff_bases
                })
                .await;

            this.update(&mut cx, |this, cx| {
                // Assign the new diff bases on all of the buffers.
                for (buffer, diff_base) in diff_bases_by_buffer {
                    let buffer_id = buffer.update(cx, |buffer, cx| {
                        buffer.set_diff_base(diff_base.clone(), cx);
                        buffer.remote_id().to_proto()
                    });
                    if let Some((client, project_id)) = &this.downstream_client(cx) {
                        client
                            .send(proto::UpdateDiffBase {
                                project_id: *project_id,
                                buffer_id,
                                diff_base,
                            })
                            .log_err();
                    }
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn local_worktree_entry_changed(
        &mut self,
        entry_id: ProjectEntryId,
        path: &Arc<Path>,
        worktree: &Model<worktree::Worktree>,
        snapshot: &worktree::Snapshot,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let project_path = ProjectPath {
            worktree_id: snapshot.id(),
            path: path.clone(),
        };
        let buffer_id = match self.local_buffer_ids_by_entry_id.get(&entry_id) {
            Some(&buffer_id) => buffer_id,
            None => self.local_buffer_ids_by_path.get(&project_path).copied()?,
        };
        let buffer = self
            .buffer_store
            .update(cx, |buffer_store, _| {
                if let Some(buffer) = buffer_store.get(buffer_id) {
                    Some(buffer)
                } else {
                    buffer_store.opened_buffers.remove(&buffer_id);
                    None
                }
            })
            .ok()
            .flatten();
        let buffer = if let Some(buffer) = buffer {
            buffer
        } else {
            self.local_buffer_ids_by_path.remove(&project_path);
            self.local_buffer_ids_by_entry_id.remove(&entry_id);
            return None;
        };

        let events = buffer.update(cx, |buffer, cx| {
            let file = buffer.file()?;
            let old_file = File::from_dyn(Some(file))?;
            if old_file.worktree != *worktree {
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
                    worktree: worktree.clone(),
                    is_deleted: false,
                    is_private: entry.is_private,
                }
            } else if let Some(entry) = snapshot.entry_for_path(old_file.path.as_ref()) {
                File {
                    is_local: true,
                    entry_id: Some(entry.id),
                    mtime: entry.mtime,
                    path: entry.path.clone(),
                    worktree: worktree.clone(),
                    is_deleted: false,
                    is_private: entry.is_private,
                }
            } else {
                File {
                    is_local: true,
                    entry_id: old_file.entry_id,
                    path: old_file.path.clone(),
                    mtime: old_file.mtime,
                    worktree: worktree.clone(),
                    is_deleted: true,
                    is_private: old_file.is_private,
                }
            };

            if new_file == *old_file {
                return None;
            }

            let mut events = Vec::new();
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
                events.push(BufferStoreEvent::BufferChangedFilePath {
                    buffer: cx.handle(),
                    old_file: buffer.file().cloned(),
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

            if let Some((client, project_id)) = &self.downstream_client(cx) {
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
        self.buffer_store
            .update(cx, |_buffer_store, cx| {
                for event in events {
                    cx.emit(event);
                }
            })
            .log_err()?;

        None
    }

    fn downstream_client(&self, cx: &AppContext) -> Option<(AnyProtoClient, u64)> {
        self.buffer_store
            .upgrade()?
            .read(cx)
            .downstream_client
            .clone()
    }

    fn buffer_changed_file(&mut self, buffer: Model<Buffer>, cx: &mut AppContext) -> Option<()> {
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
}

impl BufferStoreImpl for Model<LocalBufferStore> {
    fn as_remote(&self) -> Option<Model<RemoteBufferStore>> {
        None
    }

    fn as_local(&self) -> Option<Model<LocalBufferStore>> {
        Some(self.clone())
    }

    fn save_buffer(
        &self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<()>> {
        self.update(cx, |this, cx| {
            let Some(file) = File::from_dyn(buffer.read(cx).file()) else {
                return Task::ready(Err(anyhow!("buffer doesn't have a file")));
            };
            let worktree = file.worktree.clone();
            this.save_local_buffer(buffer, worktree, file.path.clone(), false, cx)
        })
    }

    fn save_buffer_as(
        &self,
        buffer: Model<Buffer>,
        path: ProjectPath,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<()>> {
        self.update(cx, |this, cx| {
            let Some(worktree) = this
                .worktree_store
                .read(cx)
                .worktree_for_id(path.worktree_id, cx)
            else {
                return Task::ready(Err(anyhow!("no such worktree")));
            };
            this.save_local_buffer(buffer, worktree, path.path.clone(), true, cx)
        })
    }

    fn open_buffer(
        &self,
        path: Arc<Path>,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<BufferStore>,
    ) -> Task<Result<Model<Buffer>>> {
        let buffer_store = cx.weak_model();
        self.update(cx, |_, cx| {
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
                    buffer_store.update(cx, |buffer_store, cx| {
                        buffer_store.add_buffer(buffer.clone(), cx)
                    })??;
                    let buffer_id = buffer.read(cx).remote_id();
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

                    anyhow::Ok(())
                })??;

                Ok(buffer)
            })
        })
    }

    fn create_buffer(&self, cx: &mut ModelContext<BufferStore>) -> Task<Result<Model<Buffer>>> {
        let handle = self.clone();
        cx.spawn(|buffer_store, mut cx| async move {
            let buffer = cx.new_model(|cx| {
                Buffer::local("", cx).with_language(language::PLAIN_TEXT.clone(), cx)
            })?;
            buffer_store.update(&mut cx, |buffer_store, cx| {
                buffer_store.add_buffer(buffer.clone(), cx).log_err();
                let buffer_id = buffer.read(cx).remote_id();
                handle.update(cx, |this, cx| {
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
                });
            })?;
            Ok(buffer)
        })
    }

    fn reload_buffers(
        &self,
        buffers: HashSet<Model<Buffer>>,
        push_to_history: bool,
        cx: &mut ModelContext<BufferStore>,
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
                        project_transaction.0.insert(cx.handle(), transaction);
                    }
                })?;
            }

            Ok(project_transaction)
        })
    }
}

impl BufferStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_model_message_handler(Self::handle_buffer_reloaded);
        client.add_model_message_handler(Self::handle_buffer_saved);
        client.add_model_message_handler(Self::handle_update_buffer_file);
        client.add_model_message_handler(Self::handle_update_diff_base);
        client.add_model_request_handler(Self::handle_save_buffer);
        client.add_model_request_handler(Self::handle_blame_buffer);
        client.add_model_request_handler(Self::handle_reload_buffers);
        client.add_model_request_handler(Self::handle_get_permalink_to_line);
    }

    /// Creates a buffer store, optionally retaining its buffers.
    pub fn local(worktree_store: Model<WorktreeStore>, cx: &mut ModelContext<Self>) -> Self {
        let this = cx.weak_model();
        Self {
            state: Box::new(cx.new_model(|cx| {
                let subscription = cx.subscribe(
                    &worktree_store,
                    |this: &mut LocalBufferStore, _, event, cx| {
                        if let WorktreeStoreEvent::WorktreeAdded(worktree) = event {
                            this.subscribe_to_worktree(worktree, cx);
                        }
                    },
                );

                LocalBufferStore {
                    local_buffer_ids_by_path: Default::default(),
                    local_buffer_ids_by_entry_id: Default::default(),
                    buffer_store: this,
                    worktree_store: worktree_store.clone(),
                    _subscription: subscription,
                }
            })),
            downstream_client: None,
            opened_buffers: Default::default(),
            shared_buffers: Default::default(),
            loading_buffers_by_path: Default::default(),
            worktree_store,
        }
    }

    pub fn remote(
        worktree_store: Model<WorktreeStore>,
        upstream_client: AnyProtoClient,
        remote_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let this = cx.weak_model();
        Self {
            state: Box::new(cx.new_model(|_| RemoteBufferStore {
                shared_with_me: Default::default(),
                loading_remote_buffers_by_id: Default::default(),
                remote_buffer_listeners: Default::default(),
                project_id: remote_id,
                upstream_client,
                worktree_store: worktree_store.clone(),
                buffer_store: this,
            })),
            downstream_client: None,
            opened_buffers: Default::default(),
            loading_buffers_by_path: Default::default(),
            shared_buffers: Default::default(),
            worktree_store,
        }
    }

    pub fn open_buffer(
        &mut self,
        project_path: ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        let existing_buffer = self.get_by_path(&project_path, cx);
        if let Some(existing_buffer) = existing_buffer {
            return Task::ready(Ok(existing_buffer));
        }

        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };

        let loading_watch = match self.loading_buffers_by_path.entry(project_path.clone()) {
            // If the given path is already being loaded, then wait for that existing
            // task to complete and return the same buffer.
            hash_map::Entry::Occupied(e) => e.get().clone(),

            // Otherwise, record the fact that this path is now being loaded.
            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let project_path = project_path.clone();
                let load_buffer = self
                    .state
                    .open_buffer(project_path.path.clone(), worktree, cx);

                cx.spawn(move |this, mut cx| async move {
                    let load_result = load_buffer.await;
                    *tx.borrow_mut() = Some(this.update(&mut cx, |this, _cx| {
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

    pub fn create_buffer(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<Model<Buffer>>> {
        self.state.create_buffer(cx)
    }

    pub fn save_buffer(
        &mut self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        self.state.save_buffer(buffer, cx)
    }

    pub fn save_buffer_as(
        &mut self,
        buffer: Model<Buffer>,
        path: ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let old_file = buffer.read(cx).file().cloned();
        let task = self.state.save_buffer_as(buffer.clone(), path, cx);
        cx.spawn(|this, mut cx| async move {
            task.await?;
            this.update(&mut cx, |_, cx| {
                cx.emit(BufferStoreEvent::BufferChangedFilePath { buffer, old_file });
            })
        })
    }

    pub fn blame_buffer(
        &self,
        buffer: &Model<Buffer>,
        version: Option<clock::Global>,
        cx: &AppContext,
    ) -> Task<Result<Blame>> {
        let buffer = buffer.read(cx);
        let Some(file) = File::from_dyn(buffer.file()) else {
            return Task::ready(Err(anyhow!("buffer has no file")));
        };

        match file.worktree.clone().read(cx) {
            Worktree::Local(worktree) => {
                let worktree = worktree.snapshot();
                let blame_params = maybe!({
                    let (repo_entry, local_repo_entry) = match worktree.repo_for_path(&file.path) {
                        Some(repo_for_path) => repo_for_path,
                        None => anyhow::bail!(NoRepositoryError {}),
                    };

                    let relative_path = repo_entry
                        .relativize(&worktree, &file.path)
                        .context("failed to relativize buffer path")?;

                    let repo = local_repo_entry.repo().clone();

                    let content = match version {
                        Some(version) => buffer.rope_for_version(&version).clone(),
                        None => buffer.as_rope().clone(),
                    };

                    anyhow::Ok((repo, relative_path, content))
                });

                cx.background_executor().spawn(async move {
                    let (repo, relative_path, content) = blame_params?;
                    repo.blame(&relative_path, content)
                        .with_context(|| format!("Failed to blame {:?}", relative_path.0))
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
        buffer: &Model<Buffer>,
        selection: Range<u32>,
        cx: &AppContext,
    ) -> Task<Result<url::Url>> {
        let buffer = buffer.read(cx);
        let Some(file) = File::from_dyn(buffer.file()) else {
            return Task::ready(Err(anyhow!("buffer has no file")));
        };

        match file.worktree.clone().read(cx) {
            Worktree::Local(worktree) => {
                let Some(repo) = worktree.local_git_repo(file.path()) else {
                    return Task::ready(Err(anyhow!("no repository for buffer found")));
                };

                let path = file.path().clone();

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
                        .context("failed to convert buffer path to string")?;

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

    fn add_buffer(&mut self, buffer: Model<Buffer>, cx: &mut ModelContext<Self>) -> Result<()> {
        let remote_id = buffer.read(cx).remote_id();
        let is_remote = buffer.read(cx).replica_id() != 0;
        let open_buffer = OpenBuffer::Buffer(buffer.downgrade());

        let handle = cx.handle().downgrade();
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
            if file.worktree_id(cx) == path.worktree_id && file.path == path.path {
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

    pub fn get_possibly_incomplete(
        &self,
        buffer_id: BufferId,
        cx: &AppContext,
    ) -> Option<Model<Buffer>> {
        self.get(buffer_id).or_else(|| {
            self.state.as_remote().and_then(|remote| {
                remote
                    .read(cx)
                    .loading_remote_buffers_by_id
                    .get(&buffer_id)
                    .cloned()
            })
        })
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
            .state
            .as_remote()
            .map(|remote| remote.read(cx).incomplete_buffer_ids())
            .unwrap_or_default();
        (buffers, incomplete_buffer_ids)
    }

    pub fn disconnected_from_host(&mut self, cx: &mut AppContext) {
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

        if let Some(remote) = self.state.as_remote() {
            remote.update(cx, |remote, _| {
                // Wake up all futures currently waiting on a buffer to get opened,
                // to give them a chance to fail now that we've disconnected.
                remote.remote_buffer_listeners.clear()
            })
        }
    }

    pub fn shared(
        &mut self,
        remote_id: u64,
        downstream_client: AnyProtoClient,
        _cx: &mut AppContext,
    ) {
        self.downstream_client = Some((downstream_client, remote_id));
    }

    pub fn unshared(&mut self, _cx: &mut ModelContext<Self>) {
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
        cx: &mut ModelContext<Self>,
    ) -> Receiver<Model<Buffer>> {
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
        let mut project_paths_rx = self
            .worktree_store
            .update(cx, |worktree_store, cx| {
                worktree_store.find_search_candidates(query.clone(), limit, open_buffers, fs, cx)
            })
            .chunks(MAX_CONCURRENT_BUFFER_OPENS);

        cx.spawn(|this, mut cx| async move {
            for buffer in unnamed_buffers {
                tx.send(buffer).await.ok();
            }

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

    fn on_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &BufferEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            BufferEvent::FileHandleChanged => {
                if let Some(local) = self.state.as_local() {
                    local.update(cx, |local, cx| {
                        local.buffer_changed_file(buffer, cx);
                    })
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
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        mut cx: AsyncAppContext,
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
                    OpenBuffer::Buffer(buffer) => {
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

    pub fn handle_synchronize_buffers(
        &mut self,
        envelope: TypedEnvelope<proto::SynchronizeBuffers>,
        cx: &mut ModelContext<Self>,
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
                    .insert(buffer.clone());

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

                client
                    .send(proto::UpdateDiffBase {
                        project_id,
                        buffer_id: buffer_id.into(),
                        diff_base: buffer.diff_base().map(ToString::to_string),
                    })
                    .log_err();

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
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let Some(remote) = self.state.as_remote() else {
            return Err(anyhow!("buffer store is not a remote"));
        };

        if let Some(buffer) = remote.update(cx, |remote, cx| {
            remote.handle_create_buffer_for_peer(envelope, replica_id, capability, cx)
        })? {
            self.add_buffer(buffer, cx)?;
        }

        Ok(())
    }

    pub async fn handle_update_buffer_file(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateBufferFile>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let buffer_id = envelope.payload.buffer_id;
        let buffer_id = BufferId::new(buffer_id)?;

        this.update(&mut cx, |this, cx| {
            let payload = envelope.payload.clone();
            if let Some(buffer) = this.get_possibly_incomplete(buffer_id, cx) {
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

    pub async fn handle_update_diff_base(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateDiffBase>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let buffer_id = envelope.payload.buffer_id;
            let buffer_id = BufferId::new(buffer_id)?;
            if let Some(buffer) = this.get_possibly_incomplete(buffer_id, cx) {
                buffer.update(cx, |buffer, cx| {
                    buffer.set_diff_base(envelope.payload.diff_base.clone(), cx)
                });
            }
            if let Some((downstream_client, project_id)) = this.downstream_client.as_ref() {
                downstream_client
                    .send(proto::UpdateDiffBase {
                        project_id: *project_id,
                        buffer_id: buffer_id.into(),
                        diff_base: envelope.payload.diff_base,
                    })
                    .log_err();
            }
            Ok(())
        })?
    }

    pub async fn handle_save_buffer(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        mut cx: AsyncAppContext,
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
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let peer_id = envelope.sender_id;
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        this.update(&mut cx, |this, _| {
            if let Some(buffer) = this.get(buffer_id) {
                if let Some(shared) = this.shared_buffers.get_mut(&peer_id) {
                    if shared.remove(&buffer) {
                        if shared.is_empty() {
                            this.shared_buffers.remove(&peer_id);
                        }
                        return;
                    }
                }
            };
            debug_panic!(
                "peer_id {} closed buffer_id {} which was either not open or already closed",
                peer_id,
                buffer_id
            )
        })
    }

    pub async fn handle_buffer_saved(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::BufferSaved>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let version = deserialize_version(&envelope.payload.version);
        let mtime = envelope.payload.mtime.clone().map(|time| time.into());
        this.update(&mut cx, move |this, cx| {
            if let Some(buffer) = this.get_possibly_incomplete(buffer_id, cx) {
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
        this: Model<Self>,
        envelope: TypedEnvelope<proto::BufferReloaded>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let version = deserialize_version(&envelope.payload.version);
        let mtime = envelope.payload.mtime.clone().map(|time| time.into());
        let line_ending = deserialize_line_ending(
            proto::LineEnding::from_i32(envelope.payload.line_ending)
                .ok_or_else(|| anyhow!("missing line ending"))?,
        );
        this.update(&mut cx, |this, cx| {
            if let Some(buffer) = this.get_possibly_incomplete(buffer_id, cx) {
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
        this: Model<Self>,
        envelope: TypedEnvelope<proto::BlameBuffer>,
        mut cx: AsyncAppContext,
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
        this: Model<Self>,
        envelope: TypedEnvelope<proto::GetPermalinkToLine>,
        mut cx: AsyncAppContext,
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

    pub fn reload_buffers(
        &self,
        buffers: HashSet<Model<Buffer>>,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        if buffers.is_empty() {
            return Task::ready(Ok(ProjectTransaction::default()));
        }

        self.state.reload_buffers(buffers, push_to_history, cx)
    }

    async fn handle_reload_buffers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ReloadBuffers>,
        mut cx: AsyncAppContext,
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
        buffer: &Model<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let buffer_id = buffer.read(cx).remote_id();
        if !self
            .shared_buffers
            .entry(peer_id)
            .or_default()
            .insert(buffer.clone())
        {
            return Task::ready(Ok(()));
        }

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

    pub fn shared_buffers(&self) -> &HashMap<proto::PeerId, HashSet<Model<Buffer>>> {
        &self.shared_buffers
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
        let buffer_id = buffer.read(cx).remote_id();

        let local = self
            .state
            .as_local()
            .expect("local-only method called in a non-local context");
        local.update(cx, |this, cx| {
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
        });
        buffer
    }

    pub fn deserialize_project_transaction(
        &mut self,
        message: proto::ProjectTransaction,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        if let Some(remote) = self.state.as_remote() {
            remote.update(cx, |remote, cx| {
                remote.deserialize_project_transaction(message, push_to_history, cx)
            })
        } else {
            debug_panic!("not a remote buffer store");
            Task::ready(Err(anyhow!("not a remote buffer store")))
        }
    }

    pub fn wait_for_remote_buffer(
        &self,
        id: BufferId,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Buffer>>> {
        if let Some(remote) = self.state.as_remote() {
            remote.update(cx, |remote, cx| remote.wait_for_remote_buffer(id, cx))
        } else {
            debug_panic!("not a remote buffer store");
            Task::ready(Err(anyhow!("not a remote buffer store")))
        }
    }

    pub fn serialize_project_transaction_for_peer(
        &mut self,
        project_transaction: ProjectTransaction,
        peer_id: proto::PeerId,
        cx: &mut ModelContext<Self>,
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
    fn upgrade(&self) -> Option<Model<Buffer>> {
        match self {
            OpenBuffer::Buffer(handle) => handle.upgrade(),
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

fn serialize_blame_buffer_response(blame: git::blame::Blame) -> proto::BlameBufferResponse {
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
        entries,
        messages,
        permalinks,
        remote_url: blame.remote_url,
    }
}

fn deserialize_blame_buffer_response(response: proto::BlameBufferResponse) -> git::blame::Blame {
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

    Blame {
        entries,
        permalinks,
        messages,
        remote_url: response.remote_url,
    }
}
