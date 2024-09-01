use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, Arc},
};

use anyhow::{anyhow, Context as _, Result};
use client::DevServerProjectId;
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::{
    future::{BoxFuture, Shared},
    FutureExt, SinkExt,
};
use gpui::{
    AppContext, AsyncAppContext, EntityId, EventEmitter, Model, ModelContext, Task, WeakModel,
};
use postage::oneshot;
use rpc::{
    proto::{self, AnyProtoClient, SSH_PROJECT_ID},
    TypedEnvelope,
};
use smol::{
    channel::{Receiver, Sender},
    stream::StreamExt,
};
use text::ReplicaId;
use util::{paths::compare_paths, ResultExt};
use worktree::{Entry, ProjectEntryId, Worktree, WorktreeId, WorktreeSettings};

use crate::{search::SearchQuery, ProjectPath};

struct MatchingEntry {
    worktree_path: Arc<Path>,
    path: ProjectPath,
    respond: oneshot::Sender<ProjectPath>,
}

pub struct WorktreeStore {
    next_entry_id: Arc<AtomicUsize>,
    upstream_client: Option<AnyProtoClient>,
    dev_server_project_id: Option<DevServerProjectId>,
    is_shared: bool,
    worktrees: Vec<WorktreeHandle>,
    worktrees_reordered: bool,
    #[allow(clippy::type_complexity)]
    loading_worktrees:
        HashMap<Arc<Path>, Shared<Task<Result<Model<Worktree>, Arc<anyhow::Error>>>>>,
    fs: Arc<dyn Fs>,
}

pub enum WorktreeStoreEvent {
    WorktreeAdded(Model<Worktree>),
    WorktreeRemoved(EntityId, WorktreeId),
    WorktreeOrderChanged,
}

impl EventEmitter<WorktreeStoreEvent> for WorktreeStore {}

impl WorktreeStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_model_request_handler(Self::handle_create_project_entry);
        client.add_model_request_handler(Self::handle_rename_project_entry);
        client.add_model_request_handler(Self::handle_copy_project_entry);
        client.add_model_request_handler(Self::handle_delete_project_entry);
        client.add_model_request_handler(Self::handle_expand_project_entry);
    }

    pub fn new(retain_worktrees: bool, fs: Arc<dyn Fs>) -> Self {
        Self {
            next_entry_id: Default::default(),
            loading_worktrees: Default::default(),
            upstream_client: None,
            dev_server_project_id: None,
            is_shared: retain_worktrees,
            worktrees: Vec::new(),
            worktrees_reordered: false,
            fs,
        }
    }

    pub fn set_upstream_client(&mut self, client: AnyProtoClient) {
        self.upstream_client = Some(client);
    }

    pub fn set_dev_server_project_id(&mut self, id: DevServerProjectId) {
        self.dev_server_project_id = Some(id);
    }

    /// Iterates through all worktrees, including ones that don't appear in the project panel
    pub fn worktrees(&self) -> impl '_ + DoubleEndedIterator<Item = Model<Worktree>> {
        self.worktrees
            .iter()
            .filter_map(move |worktree| worktree.upgrade())
    }

    /// Iterates through all user-visible worktrees, the ones that appear in the project panel.
    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + DoubleEndedIterator<Item = Model<Worktree>> {
        self.worktrees()
            .filter(|worktree| worktree.read(cx).is_visible())
    }

    pub fn worktree_for_id(&self, id: WorktreeId, cx: &AppContext) -> Option<Model<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<Model<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).contains_entry(entry_id))
    }

    pub fn find_worktree(
        &self,
        abs_path: &Path,
        cx: &AppContext,
    ) -> Option<(Model<Worktree>, PathBuf)> {
        for tree in self.worktrees() {
            if let Ok(relative_path) = abs_path.strip_prefix(tree.read(cx).abs_path()) {
                return Some((tree.clone(), relative_path.into()));
            }
        }
        None
    }

    pub fn entry_for_id<'a>(
        &'a self,
        entry_id: ProjectEntryId,
        cx: &'a AppContext,
    ) -> Option<&'a Entry> {
        self.worktrees()
            .find_map(|worktree| worktree.read(cx).entry_for_id(entry_id))
    }

    pub fn entry_for_path(&self, path: &ProjectPath, cx: &AppContext) -> Option<Entry> {
        self.worktree_for_id(path.worktree_id, cx)?
            .read(cx)
            .entry_for_path(&path.path)
            .cloned()
    }

    pub fn create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>>> {
        let path: Arc<Path> = abs_path.as_ref().into();
        if !self.loading_worktrees.contains_key(&path) {
            let task = if let Some(client) = self.upstream_client.clone() {
                if let Some(dev_server_project_id) = self.dev_server_project_id {
                    self.create_dev_server_worktree(client, dev_server_project_id, abs_path, cx)
                } else {
                    self.create_ssh_worktree(client, abs_path, visible, cx)
                }
            } else {
                self.create_local_worktree(abs_path, visible, cx)
            };

            self.loading_worktrees.insert(path.clone(), task.shared());
        }
        let task = self.loading_worktrees.get(&path).unwrap().clone();
        cx.background_executor().spawn(async move {
            let result = match task.await {
                Ok(worktree) => Ok(worktree),
                Err(err) => Err(anyhow!("{}", err)),
            };
            result
        })
    }

    fn create_ssh_worktree(
        &mut self,
        client: AnyProtoClient,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>, Arc<anyhow::Error>>> {
        let abs_path = abs_path.as_ref();
        let root_name = abs_path.file_name().unwrap().to_string_lossy().to_string();
        let path = abs_path.to_string_lossy().to_string();
        cx.spawn(|this, mut cx| async move {
            let response = client
                .request(proto::AddWorktree {
                    project_id: SSH_PROJECT_ID,
                    path: path.clone(),
                })
                .await?;
            let worktree = cx.update(|cx| {
                Worktree::remote(
                    0,
                    0,
                    proto::WorktreeMetadata {
                        id: response.worktree_id,
                        root_name,
                        visible,
                        abs_path: path,
                    },
                    client,
                    cx,
                )
            })?;

            this.update(&mut cx, |this, cx| this.add(&worktree, cx))?;

            Ok(worktree)
        })
    }

    fn create_local_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>, Arc<anyhow::Error>>> {
        let fs = self.fs.clone();
        let next_entry_id = self.next_entry_id.clone();
        let path: Arc<Path> = abs_path.as_ref().into();

        cx.spawn(move |this, mut cx| async move {
            let worktree = Worktree::local(path.clone(), visible, fs, next_entry_id, &mut cx).await;

            this.update(&mut cx, |project, _| {
                project.loading_worktrees.remove(&path);
            })?;

            let worktree = worktree?;
            this.update(&mut cx, |this, cx| this.add(&worktree, cx))?;

            if visible {
                cx.update(|cx| {
                    cx.add_recent_document(&path);
                })
                .log_err();
            }

            Ok(worktree)
        })
    }

    fn create_dev_server_worktree(
        &mut self,
        client: AnyProtoClient,
        dev_server_project_id: DevServerProjectId,
        abs_path: impl AsRef<Path>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>, Arc<anyhow::Error>>> {
        let path: Arc<Path> = abs_path.as_ref().into();
        let mut paths: Vec<String> = self
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().to_string())
            .collect();
        paths.push(path.to_string_lossy().to_string());
        let request = client.request(proto::UpdateDevServerProject {
            dev_server_project_id: dev_server_project_id.0,
            paths,
        });

        let abs_path = abs_path.as_ref().to_path_buf();
        cx.spawn(move |project, mut cx| async move {
            let (tx, rx) = futures::channel::oneshot::channel();
            let tx = RefCell::new(Some(tx));
            let Some(project) = project.upgrade() else {
                return Err(anyhow!("project dropped"))?;
            };
            let observer = cx.update(|cx| {
                cx.observe(&project, move |project, cx| {
                    let abs_path = abs_path.clone();
                    project.update(cx, |project, cx| {
                        if let Some((worktree, _)) = project.find_worktree(&abs_path, cx) {
                            if let Some(tx) = tx.borrow_mut().take() {
                                tx.send(worktree).ok();
                            }
                        }
                    })
                })
            })?;

            request.await?;
            let worktree = rx.await.map_err(|e| anyhow!(e))?;
            drop(observer);
            project.update(&mut cx, |project, _| {
                project.loading_worktrees.remove(&path);
            })?;
            Ok(worktree)
        })
    }

    pub fn add(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
        let push_strong_handle = self.is_shared || worktree.read(cx).is_visible();
        let handle = if push_strong_handle {
            WorktreeHandle::Strong(worktree.clone())
        } else {
            WorktreeHandle::Weak(worktree.downgrade())
        };
        if self.worktrees_reordered {
            self.worktrees.push(handle);
        } else {
            let i = match self
                .worktrees
                .binary_search_by_key(&Some(worktree.read(cx).abs_path()), |other| {
                    other.upgrade().map(|worktree| worktree.read(cx).abs_path())
                }) {
                Ok(i) | Err(i) => i,
            };
            self.worktrees.insert(i, handle);
        }

        cx.emit(WorktreeStoreEvent::WorktreeAdded(worktree.clone()));

        let handle_id = worktree.entity_id();
        cx.observe_release(worktree, move |_, worktree, cx| {
            cx.emit(WorktreeStoreEvent::WorktreeRemoved(
                handle_id,
                worktree.id(),
            ));
        })
        .detach();
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut ModelContext<Self>) {
        self.worktrees.retain(|worktree| {
            if let Some(worktree) = worktree.upgrade() {
                if worktree.read(cx).id() == id_to_remove {
                    cx.emit(WorktreeStoreEvent::WorktreeRemoved(
                        worktree.entity_id(),
                        id_to_remove,
                    ));
                    false
                } else {
                    true
                }
            } else {
                false
            }
        });
    }

    pub fn set_worktrees_reordered(&mut self, worktrees_reordered: bool) {
        self.worktrees_reordered = worktrees_reordered;
    }

    pub fn set_worktrees_from_proto(
        &mut self,
        worktrees: Vec<proto::WorktreeMetadata>,
        replica_id: ReplicaId,
        remote_id: u64,
        client: AnyProtoClient,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let mut old_worktrees_by_id = self
            .worktrees
            .drain(..)
            .filter_map(|worktree| {
                let worktree = worktree.upgrade()?;
                Some((worktree.read(cx).id(), worktree))
            })
            .collect::<HashMap<_, _>>();

        for worktree in worktrees {
            if let Some(old_worktree) =
                old_worktrees_by_id.remove(&WorktreeId::from_proto(worktree.id))
            {
                self.worktrees.push(WorktreeHandle::Strong(old_worktree));
            } else {
                self.add(
                    &Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx),
                    cx,
                );
            }
        }

        Ok(())
    }

    pub fn move_worktree(
        &mut self,
        source: WorktreeId,
        destination: WorktreeId,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if source == destination {
            return Ok(());
        }

        let mut source_index = None;
        let mut destination_index = None;
        for (i, worktree) in self.worktrees.iter().enumerate() {
            if let Some(worktree) = worktree.upgrade() {
                let worktree_id = worktree.read(cx).id();
                if worktree_id == source {
                    source_index = Some(i);
                    if destination_index.is_some() {
                        break;
                    }
                } else if worktree_id == destination {
                    destination_index = Some(i);
                    if source_index.is_some() {
                        break;
                    }
                }
            }
        }

        let source_index =
            source_index.with_context(|| format!("Missing worktree for id {source}"))?;
        let destination_index =
            destination_index.with_context(|| format!("Missing worktree for id {destination}"))?;

        if source_index == destination_index {
            return Ok(());
        }

        let worktree_to_move = self.worktrees.remove(source_index);
        self.worktrees.insert(destination_index, worktree_to_move);
        self.worktrees_reordered = true;
        cx.emit(WorktreeStoreEvent::WorktreeOrderChanged);
        cx.notify();
        Ok(())
    }

    pub fn disconnected_from_host(&mut self, cx: &mut AppContext) {
        for worktree in &self.worktrees {
            if let Some(worktree) = worktree.upgrade() {
                worktree.update(cx, |worktree, _| {
                    if let Some(worktree) = worktree.as_remote_mut() {
                        worktree.disconnected_from_host();
                    }
                });
            }
        }
    }

    pub fn set_shared(&mut self, is_shared: bool, cx: &mut ModelContext<Self>) {
        self.is_shared = is_shared;

        // When shared, retain all worktrees
        if is_shared {
            for worktree_handle in self.worktrees.iter_mut() {
                match worktree_handle {
                    WorktreeHandle::Strong(_) => {}
                    WorktreeHandle::Weak(worktree) => {
                        if let Some(worktree) = worktree.upgrade() {
                            *worktree_handle = WorktreeHandle::Strong(worktree);
                        }
                    }
                }
            }
        }
        // When not shared, only retain the visible worktrees
        else {
            for worktree_handle in self.worktrees.iter_mut() {
                if let WorktreeHandle::Strong(worktree) = worktree_handle {
                    let is_visible = worktree.update(cx, |worktree, _| {
                        worktree.stop_observing_updates();
                        worktree.is_visible()
                    });
                    if !is_visible {
                        *worktree_handle = WorktreeHandle::Weak(worktree.downgrade());
                    }
                }
            }
        }
    }

    /// search over all worktrees and return buffers that *might* match the search.
    pub fn find_search_candidates(
        &self,
        query: SearchQuery,
        limit: usize,
        open_entries: HashSet<ProjectEntryId>,
        fs: Arc<dyn Fs>,
        cx: &ModelContext<Self>,
    ) -> Receiver<ProjectPath> {
        let snapshots = self
            .visible_worktrees(cx)
            .filter_map(|tree| {
                let tree = tree.read(cx);
                Some((tree.snapshot(), tree.as_local()?.settings()))
            })
            .collect::<Vec<_>>();

        let executor = cx.background_executor().clone();

        // We want to return entries in the order they are in the worktrees, so we have one
        // thread that iterates over the worktrees (and ignored directories) as necessary,
        // and pushes a oneshot::Receiver to the output channel and a oneshot::Sender to the filter
        // channel.
        // We spawn a number of workers that take items from the filter channel and check the query
        // against the version of the file on disk.
        let (filter_tx, filter_rx) = smol::channel::bounded(64);
        let (output_tx, mut output_rx) = smol::channel::bounded(64);
        let (matching_paths_tx, matching_paths_rx) = smol::channel::unbounded();

        let input = cx.background_executor().spawn({
            let fs = fs.clone();
            let query = query.clone();
            async move {
                Self::find_candidate_paths(
                    fs,
                    snapshots,
                    open_entries,
                    query,
                    filter_tx,
                    output_tx,
                )
                .await
                .log_err();
            }
        });
        const MAX_CONCURRENT_FILE_SCANS: usize = 64;
        let filters = cx.background_executor().spawn(async move {
            let fs = &fs;
            let query = &query;
            executor
                .scoped(move |scope| {
                    for _ in 0..MAX_CONCURRENT_FILE_SCANS {
                        let filter_rx = filter_rx.clone();
                        scope.spawn(async move {
                            Self::filter_paths(fs, filter_rx, query).await.log_err();
                        })
                    }
                })
                .await;
        });
        cx.background_executor()
            .spawn(async move {
                let mut matched = 0;
                while let Some(mut receiver) = output_rx.next().await {
                    let Some(path) = receiver.next().await else {
                        continue;
                    };
                    let Ok(_) = matching_paths_tx.send(path).await else {
                        break;
                    };
                    matched += 1;
                    if matched == limit {
                        break;
                    }
                }
                drop(input);
                drop(filters);
            })
            .detach();
        return matching_paths_rx;
    }

    fn scan_ignored_dir<'a>(
        fs: &'a Arc<dyn Fs>,
        snapshot: &'a worktree::Snapshot,
        path: &'a Path,
        query: &'a SearchQuery,
        include_root: bool,
        filter_tx: &'a Sender<MatchingEntry>,
        output_tx: &'a Sender<oneshot::Receiver<ProjectPath>>,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let abs_path = snapshot.abs_path().join(&path);
            let Some(mut files) = fs
                .read_dir(&abs_path)
                .await
                .with_context(|| format!("listing ignored path {abs_path:?}"))
                .log_err()
            else {
                return Ok(());
            };

            let mut results = Vec::new();

            while let Some(Ok(file)) = files.next().await {
                let Some(metadata) = fs
                    .metadata(&file)
                    .await
                    .with_context(|| format!("fetching fs metadata for {abs_path:?}"))
                    .log_err()
                    .flatten()
                else {
                    continue;
                };
                if metadata.is_symlink || metadata.is_fifo {
                    continue;
                }
                results.push((
                    file.strip_prefix(snapshot.abs_path())?.to_path_buf(),
                    !metadata.is_dir,
                ))
            }
            results.sort_by(|(a_path, a_is_file), (b_path, b_is_file)| {
                compare_paths((a_path, *a_is_file), (b_path, *b_is_file))
            });
            for (path, is_file) in results {
                if is_file {
                    if query.filters_path() {
                        let matched_path = if include_root {
                            let mut full_path = PathBuf::from(snapshot.root_name());
                            full_path.push(&path);
                            query.file_matches(&full_path)
                        } else {
                            query.file_matches(&path)
                        };
                        if !matched_path {
                            continue;
                        }
                    }
                    let (tx, rx) = oneshot::channel();
                    output_tx.send(rx).await?;
                    filter_tx
                        .send(MatchingEntry {
                            respond: tx,
                            worktree_path: snapshot.abs_path().clone(),
                            path: ProjectPath {
                                worktree_id: snapshot.id(),
                                path: Arc::from(path),
                            },
                        })
                        .await?;
                } else {
                    Self::scan_ignored_dir(
                        fs,
                        snapshot,
                        &path,
                        query,
                        include_root,
                        filter_tx,
                        output_tx,
                    )
                    .await?;
                }
            }
            Ok(())
        }
        .boxed()
    }

    async fn find_candidate_paths(
        fs: Arc<dyn Fs>,
        snapshots: Vec<(worktree::Snapshot, WorktreeSettings)>,
        open_entries: HashSet<ProjectEntryId>,
        query: SearchQuery,
        filter_tx: Sender<MatchingEntry>,
        output_tx: Sender<oneshot::Receiver<ProjectPath>>,
    ) -> Result<()> {
        let include_root = snapshots.len() > 1;
        for (snapshot, settings) in snapshots {
            let mut entries: Vec<_> = snapshot.entries(query.include_ignored(), 0).collect();
            entries.sort_by(|a, b| compare_paths((&a.path, a.is_file()), (&b.path, b.is_file())));
            for entry in entries {
                if entry.is_dir() && entry.is_ignored {
                    if !settings.is_path_excluded(&entry.path) {
                        Self::scan_ignored_dir(
                            &fs,
                            &snapshot,
                            &entry.path,
                            &query,
                            include_root,
                            &filter_tx,
                            &output_tx,
                        )
                        .await?;
                    }
                    continue;
                }

                if entry.is_fifo || !entry.is_file() {
                    continue;
                }

                if query.filters_path() {
                    let matched_path = if include_root {
                        let mut full_path = PathBuf::from(snapshot.root_name());
                        full_path.push(&entry.path);
                        query.file_matches(&full_path)
                    } else {
                        query.file_matches(&entry.path)
                    };
                    if !matched_path {
                        continue;
                    }
                }

                let (mut tx, rx) = oneshot::channel();

                if open_entries.contains(&entry.id) {
                    tx.send(ProjectPath {
                        worktree_id: snapshot.id(),
                        path: entry.path.clone(),
                    })
                    .await?;
                } else {
                    filter_tx
                        .send(MatchingEntry {
                            respond: tx,
                            worktree_path: snapshot.abs_path().clone(),
                            path: ProjectPath {
                                worktree_id: snapshot.id(),
                                path: entry.path.clone(),
                            },
                        })
                        .await?;
                }

                output_tx.send(rx).await?;
            }
        }
        Ok(())
    }

    async fn filter_paths(
        fs: &Arc<dyn Fs>,
        mut input: Receiver<MatchingEntry>,
        query: &SearchQuery,
    ) -> Result<()> {
        while let Some(mut entry) = input.next().await {
            let abs_path = entry.worktree_path.join(&entry.path.path);
            let Some(file) = fs.open_sync(&abs_path).await.log_err() else {
                continue;
            };
            if query.detect(file).unwrap_or(false) {
                entry.respond.send(entry.path).await?
            }
        }

        Ok(())
    }

    pub async fn handle_create_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CreateProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let worktree = this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            this.worktree_for_id(worktree_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        Worktree::handle_create_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_rename_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::RenameProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        Worktree::handle_rename_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_copy_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CopyProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        Worktree::handle_copy_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_delete_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DeleteProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        Worktree::handle_delete_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_expand_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ExpandProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ExpandProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this
            .update(&mut cx, |this, cx| this.worktree_for_entry(entry_id, cx))?
            .ok_or_else(|| anyhow!("invalid request"))?;
        Worktree::handle_expand_entry(worktree, envelope.payload, cx).await
    }
}

#[derive(Clone)]
enum WorktreeHandle {
    Strong(Model<Worktree>),
    Weak(WeakModel<Worktree>),
}

impl WorktreeHandle {
    fn upgrade(&self) -> Option<Model<Worktree>> {
        match self {
            WorktreeHandle::Strong(handle) => Some(handle.clone()),
            WorktreeHandle::Weak(handle) => handle.upgrade(),
        }
    }
}
