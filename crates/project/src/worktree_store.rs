use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    pin::pin,
    sync::{Arc, atomic::AtomicUsize},
};

use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::{
    FutureExt, SinkExt,
    future::{BoxFuture, Shared},
};
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, EventEmitter, Task, WeakEntity,
};
use postage::oneshot;
use rpc::{
    AnyProtoClient, ErrorExt, TypedEnvelope,
    proto::{self, FromProto, REMOTE_SERVER_PROJECT_ID, ToProto},
};
use smol::{
    channel::{Receiver, Sender},
    stream::StreamExt,
};
use text::ReplicaId;
use util::{
    ResultExt,
    paths::{PathStyle, RemotePathBuf, SanitizedPath},
};
use worktree::{
    Entry, ProjectEntryId, UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree, WorktreeId,
    WorktreeSettings,
};

use crate::{ProjectPath, search::SearchQuery};

struct MatchingEntry {
    worktree_path: Arc<Path>,
    path: ProjectPath,
    respond: oneshot::Sender<ProjectPath>,
}

enum WorktreeStoreState {
    Local {
        fs: Arc<dyn Fs>,
    },
    Remote {
        upstream_client: AnyProtoClient,
        upstream_project_id: u64,
        path_style: PathStyle,
    },
}

pub struct WorktreeStore {
    next_entry_id: Arc<AtomicUsize>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    retain_worktrees: bool,
    worktrees: Vec<WorktreeHandle>,
    worktrees_reordered: bool,
    #[allow(clippy::type_complexity)]
    loading_worktrees:
        HashMap<Arc<SanitizedPath>, Shared<Task<Result<Entity<Worktree>, Arc<anyhow::Error>>>>>,
    state: WorktreeStoreState,
}

#[derive(Debug)]
pub enum WorktreeStoreEvent {
    WorktreeAdded(Entity<Worktree>),
    WorktreeRemoved(EntityId, WorktreeId),
    WorktreeReleased(EntityId, WorktreeId),
    WorktreeOrderChanged,
    WorktreeUpdateSent(Entity<Worktree>),
    WorktreeUpdatedEntries(WorktreeId, UpdatedEntriesSet),
    WorktreeUpdatedGitRepositories(WorktreeId, UpdatedGitRepositoriesSet),
    WorktreeDeletedEntry(WorktreeId, ProjectEntryId),
}

impl EventEmitter<WorktreeStoreEvent> for WorktreeStore {}

impl WorktreeStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_create_project_entry);
        client.add_entity_request_handler(Self::handle_copy_project_entry);
        client.add_entity_request_handler(Self::handle_delete_project_entry);
        client.add_entity_request_handler(Self::handle_expand_project_entry);
        client.add_entity_request_handler(Self::handle_expand_all_for_project_entry);
    }

    pub fn local(retain_worktrees: bool, fs: Arc<dyn Fs>) -> Self {
        Self {
            next_entry_id: Default::default(),
            loading_worktrees: Default::default(),
            downstream_client: None,
            worktrees: Vec::new(),
            worktrees_reordered: false,
            retain_worktrees,
            state: WorktreeStoreState::Local { fs },
        }
    }

    pub fn remote(
        retain_worktrees: bool,
        upstream_client: AnyProtoClient,
        upstream_project_id: u64,
        path_style: PathStyle,
    ) -> Self {
        Self {
            next_entry_id: Default::default(),
            loading_worktrees: Default::default(),
            downstream_client: None,
            worktrees: Vec::new(),
            worktrees_reordered: false,
            retain_worktrees,
            state: WorktreeStoreState::Remote {
                upstream_client,
                upstream_project_id,
                path_style,
            },
        }
    }

    /// Iterates through all worktrees, including ones that don't appear in the project panel
    pub fn worktrees(&self) -> impl '_ + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktrees
            .iter()
            .filter_map(move |worktree| worktree.upgrade())
    }

    /// Iterates through all user-visible worktrees, the ones that appear in the project panel.
    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktrees()
            .filter(|worktree| worktree.read(cx).is_visible())
    }

    pub fn worktree_for_id(&self, id: WorktreeId, cx: &App) -> Option<Entity<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &App,
    ) -> Option<Entity<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).contains_entry(entry_id))
    }

    pub fn find_worktree(
        &self,
        abs_path: impl AsRef<Path>,
        cx: &App,
    ) -> Option<(Entity<Worktree>, PathBuf)> {
        let abs_path = SanitizedPath::new(&abs_path);
        for tree in self.worktrees() {
            if let Ok(relative_path) = abs_path.as_path().strip_prefix(tree.read(cx).abs_path()) {
                return Some((tree.clone(), relative_path.into()));
            }
        }
        None
    }

    pub fn absolutize(&self, project_path: &ProjectPath, cx: &App) -> Option<PathBuf> {
        let worktree = self.worktree_for_id(project_path.worktree_id, cx)?;
        worktree.read(cx).absolutize(&project_path.path).ok()
    }

    pub fn find_or_create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Entity<Worktree>, PathBuf)>> {
        let abs_path = abs_path.as_ref();
        if let Some((tree, relative_path)) = self.find_worktree(abs_path, cx) {
            Task::ready(Ok((tree, relative_path)))
        } else {
            let worktree = self.create_worktree(abs_path, visible, cx);
            cx.background_spawn(async move { Ok((worktree.await?, PathBuf::new())) })
        }
    }

    pub fn entry_for_id<'a>(&'a self, entry_id: ProjectEntryId, cx: &'a App) -> Option<&'a Entry> {
        self.worktrees()
            .find_map(|worktree| worktree.read(cx).entry_for_id(entry_id))
    }

    pub fn worktree_and_entry_for_id<'a>(
        &'a self,
        entry_id: ProjectEntryId,
        cx: &'a App,
    ) -> Option<(Entity<Worktree>, &'a Entry)> {
        self.worktrees().find_map(|worktree| {
            worktree
                .read(cx)
                .entry_for_id(entry_id)
                .map(|e| (worktree.clone(), e))
        })
    }

    pub fn entry_for_path(&self, path: &ProjectPath, cx: &App) -> Option<Entry> {
        self.worktree_for_id(path.worktree_id, cx)?
            .read(cx)
            .entry_for_path(&path.path)
            .cloned()
    }

    pub fn create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Worktree>>> {
        let abs_path: Arc<SanitizedPath> = SanitizedPath::new_arc(&abs_path);
        if !self.loading_worktrees.contains_key(&abs_path) {
            let task = match &self.state {
                WorktreeStoreState::Remote {
                    upstream_client,
                    path_style,
                    ..
                } => {
                    if upstream_client.is_via_collab() {
                        Task::ready(Err(Arc::new(anyhow!("cannot create worktrees via collab"))))
                    } else {
                        let abs_path = RemotePathBuf::new(abs_path.to_path_buf(), *path_style);
                        self.create_remote_worktree(upstream_client.clone(), abs_path, visible, cx)
                    }
                }
                WorktreeStoreState::Local { fs } => {
                    self.create_local_worktree(fs.clone(), abs_path.clone(), visible, cx)
                }
            };

            self.loading_worktrees
                .insert(abs_path.clone(), task.shared());
        }
        let task = self.loading_worktrees.get(&abs_path).unwrap().clone();
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, _| this.loading_worktrees.remove(&abs_path))
                .ok();
            match result {
                Ok(worktree) => Ok(worktree),
                Err(err) => Err((*err).cloned()),
            }
        })
    }

    fn create_remote_worktree(
        &mut self,
        client: AnyProtoClient,
        abs_path: RemotePathBuf,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Worktree>, Arc<anyhow::Error>>> {
        let path_style = abs_path.path_style();
        let mut abs_path = abs_path.to_string();
        // If we start with `/~` that means the ssh path was something like `ssh://user@host/~/home-dir-folder/`
        // in which case want to strip the leading the `/`.
        // On the host-side, the `~` will get expanded.
        // That's what git does too: https://github.com/libgit2/libgit2/issues/3345#issuecomment-127050850
        if abs_path.starts_with("/~") {
            abs_path = abs_path[1..].to_string();
        }
        if abs_path.is_empty() {
            abs_path = "~/".to_string();
        }

        cx.spawn(async move |this, cx| {
            let this = this.upgrade().context("Dropped worktree store")?;

            let path = RemotePathBuf::new(abs_path.into(), path_style);
            let response = client
                .request(proto::AddWorktree {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                    path: path.to_proto(),
                    visible,
                })
                .await?;

            if let Some(existing_worktree) = this.read_with(cx, |this, cx| {
                this.worktree_for_id(WorktreeId::from_proto(response.worktree_id), cx)
            })? {
                return Ok(existing_worktree);
            }

            let root_path_buf = PathBuf::from_proto(response.canonicalized_path.clone());
            let root_name = root_path_buf
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or(root_path_buf.to_string_lossy().to_string());

            let worktree = cx.update(|cx| {
                Worktree::remote(
                    REMOTE_SERVER_PROJECT_ID,
                    0,
                    proto::WorktreeMetadata {
                        id: response.worktree_id,
                        root_name,
                        visible,
                        abs_path: response.canonicalized_path,
                    },
                    client,
                    cx,
                )
            })?;

            this.update(cx, |this, cx| {
                this.add(&worktree, cx);
            })?;
            Ok(worktree)
        })
    }

    fn create_local_worktree(
        &mut self,
        fs: Arc<dyn Fs>,
        abs_path: Arc<SanitizedPath>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Worktree>, Arc<anyhow::Error>>> {
        let next_entry_id = self.next_entry_id.clone();

        cx.spawn(async move |this, cx| {
            let worktree = Worktree::local(
                SanitizedPath::cast_arc(abs_path.clone()),
                visible,
                fs,
                next_entry_id,
                cx,
            )
            .await;

            let worktree = worktree?;

            this.update(cx, |this, cx| this.add(&worktree, cx))?;

            if visible {
                cx.update(|cx| {
                    cx.add_recent_document(abs_path.as_path());
                })
                .log_err();
            }

            Ok(worktree)
        })
    }

    pub fn add(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        let worktree_id = worktree.read(cx).id();
        debug_assert!(self.worktrees().all(|w| w.read(cx).id() != worktree_id));

        let push_strong_handle = self.retain_worktrees || worktree.read(cx).is_visible();
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
        self.send_project_updates(cx);

        let handle_id = worktree.entity_id();
        cx.subscribe(worktree, |_, worktree, event, cx| {
            let worktree_id = worktree.read(cx).id();
            match event {
                worktree::Event::UpdatedEntries(changes) => {
                    cx.emit(WorktreeStoreEvent::WorktreeUpdatedEntries(
                        worktree_id,
                        changes.clone(),
                    ));
                }
                worktree::Event::UpdatedGitRepositories(set) => {
                    cx.emit(WorktreeStoreEvent::WorktreeUpdatedGitRepositories(
                        worktree_id,
                        set.clone(),
                    ));
                }
                worktree::Event::DeletedEntry(id) => {
                    cx.emit(WorktreeStoreEvent::WorktreeDeletedEntry(worktree_id, *id))
                }
            }
        })
        .detach();
        cx.observe_release(worktree, move |this, worktree, cx| {
            cx.emit(WorktreeStoreEvent::WorktreeReleased(
                handle_id,
                worktree.id(),
            ));
            cx.emit(WorktreeStoreEvent::WorktreeRemoved(
                handle_id,
                worktree.id(),
            ));
            this.send_project_updates(cx);
        })
        .detach();
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
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
        self.send_project_updates(cx);
    }

    pub fn set_worktrees_reordered(&mut self, worktrees_reordered: bool) {
        self.worktrees_reordered = worktrees_reordered;
    }

    fn upstream_client(&self) -> Option<(AnyProtoClient, u64)> {
        match &self.state {
            WorktreeStoreState::Remote {
                upstream_client,
                upstream_project_id,
                ..
            } => Some((upstream_client.clone(), *upstream_project_id)),
            WorktreeStoreState::Local { .. } => None,
        }
    }

    pub fn set_worktrees_from_proto(
        &mut self,
        worktrees: Vec<proto::WorktreeMetadata>,
        replica_id: ReplicaId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let mut old_worktrees_by_id = self
            .worktrees
            .drain(..)
            .filter_map(|worktree| {
                let worktree = worktree.upgrade()?;
                Some((worktree.read(cx).id(), worktree))
            })
            .collect::<HashMap<_, _>>();

        let (client, project_id) = self.upstream_client().context("invalid project")?;

        for worktree in worktrees {
            if let Some(old_worktree) =
                old_worktrees_by_id.remove(&WorktreeId::from_proto(worktree.id))
            {
                let push_strong_handle =
                    self.retain_worktrees || old_worktree.read(cx).is_visible();
                let handle = if push_strong_handle {
                    WorktreeHandle::Strong(old_worktree.clone())
                } else {
                    WorktreeHandle::Weak(old_worktree.downgrade())
                };
                self.worktrees.push(handle);
            } else {
                self.add(
                    &Worktree::remote(project_id, replica_id, worktree, client.clone(), cx),
                    cx,
                );
            }
        }
        self.send_project_updates(cx);

        Ok(())
    }

    pub fn move_worktree(
        &mut self,
        source: WorktreeId,
        destination: WorktreeId,
        cx: &mut Context<Self>,
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

    pub fn disconnected_from_host(&mut self, cx: &mut App) {
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

    pub fn send_project_updates(&mut self, cx: &mut Context<Self>) {
        let Some((downstream_client, project_id)) = self.downstream_client.clone() else {
            return;
        };

        let update = proto::UpdateProject {
            project_id,
            worktrees: self.worktree_metadata_protos(cx),
        };

        // collab has bad concurrency guarantees, so we send requests in serial.
        let update_project = if downstream_client.is_via_collab() {
            Some(downstream_client.request(update))
        } else {
            downstream_client.send(update).log_err();
            None
        };
        cx.spawn(async move |this, cx| {
            if let Some(update_project) = update_project {
                update_project.await?;
            }

            this.update(cx, |this, cx| {
                let worktrees = this.worktrees().collect::<Vec<_>>();

                for worktree in worktrees {
                    worktree.update(cx, |worktree, cx| {
                        let client = downstream_client.clone();
                        worktree.observe_updates(project_id, cx, {
                            move |update| {
                                let client = client.clone();
                                async move {
                                    if client.is_via_collab() {
                                        client
                                            .request(update)
                                            .map(|result| result.log_err().is_some())
                                            .await
                                    } else {
                                        client.send(update).log_err().is_some()
                                    }
                                }
                            }
                        });
                    });

                    cx.emit(WorktreeStoreEvent::WorktreeUpdateSent(worktree.clone()))
                }

                anyhow::Ok(())
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn worktree_metadata_protos(&self, cx: &App) -> Vec<proto::WorktreeMetadata> {
        self.worktrees()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                proto::WorktreeMetadata {
                    id: worktree.id().to_proto(),
                    root_name: worktree.root_name().into(),
                    visible: worktree.is_visible(),
                    abs_path: worktree.abs_path().to_proto(),
                }
            })
            .collect()
    }

    pub fn shared(
        &mut self,
        remote_id: u64,
        downstream_client: AnyProtoClient,
        cx: &mut Context<Self>,
    ) {
        self.retain_worktrees = true;
        self.downstream_client = Some((downstream_client, remote_id));

        // When shared, retain all worktrees
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
        self.send_project_updates(cx);
    }

    pub fn unshared(&mut self, cx: &mut Context<Self>) {
        self.retain_worktrees = false;
        self.downstream_client.take();

        // When not shared, only retain the visible worktrees
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

    /// search over all worktrees and return buffers that *might* match the search.
    pub fn find_search_candidates(
        &self,
        query: SearchQuery,
        limit: usize,
        open_entries: HashSet<ProjectEntryId>,
        fs: Arc<dyn Fs>,
        cx: &Context<Self>,
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
        let (output_tx, output_rx) = smol::channel::bounded(64);
        let (matching_paths_tx, matching_paths_rx) = smol::channel::unbounded();

        let input = cx.background_spawn({
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
        let filters = cx.background_spawn(async move {
            let fs = &fs;
            let query = &query;
            executor
                .scoped(move |scope| {
                    for _ in 0..MAX_CONCURRENT_FILE_SCANS {
                        let filter_rx = filter_rx.clone();
                        scope.spawn(async move {
                            Self::filter_paths(fs, filter_rx, query)
                                .await
                                .log_with_level(log::Level::Debug);
                        })
                    }
                })
                .await;
        });
        cx.background_spawn(async move {
            let mut matched = 0;
            while let Ok(mut receiver) = output_rx.recv().await {
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
        matching_paths_rx
    }

    fn scan_ignored_dir<'a>(
        fs: &'a Arc<dyn Fs>,
        snapshot: &'a worktree::Snapshot,
        path: &'a Path,
        query: &'a SearchQuery,
        filter_tx: &'a Sender<MatchingEntry>,
        output_tx: &'a Sender<oneshot::Receiver<ProjectPath>>,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let abs_path = snapshot.abs_path().join(path);
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
            results.sort_by(|(a_path, _), (b_path, _)| a_path.cmp(b_path));
            for (path, is_file) in results {
                if is_file {
                    if query.filters_path() {
                        let matched_path = if query.match_full_paths() {
                            let mut full_path = PathBuf::from(snapshot.root_name());
                            full_path.push(&path);
                            query.match_path(&full_path)
                        } else {
                            query.match_path(&path)
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
                    Self::scan_ignored_dir(fs, snapshot, &path, query, filter_tx, output_tx)
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
        for (snapshot, settings) in snapshots {
            for entry in snapshot.entries(query.include_ignored(), 0) {
                if entry.is_dir() && entry.is_ignored {
                    if !settings.is_path_excluded(&entry.path) {
                        Self::scan_ignored_dir(
                            &fs,
                            &snapshot,
                            &entry.path,
                            &query,
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
                    let matched_path = if query.match_full_paths() {
                        let mut full_path = PathBuf::from(snapshot.root_name());
                        full_path.push(&entry.path);
                        query.match_path(&full_path)
                    } else {
                        query.match_path(&entry.path)
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
        input: Receiver<MatchingEntry>,
        query: &SearchQuery,
    ) -> Result<()> {
        let mut input = pin!(input);
        while let Some(mut entry) = input.next().await {
            let abs_path = entry.worktree_path.join(&entry.path.path);
            let Some(file) = fs.open_sync(&abs_path).await.log_err() else {
                continue;
            };

            let mut file = BufReader::new(file);
            let file_start = file.fill_buf()?;

            if let Err(Some(starting_position)) =
                std::str::from_utf8(file_start).map_err(|e| e.error_len())
            {
                // Before attempting to match the file content, throw away files that have invalid UTF-8 sequences early on;
                // That way we can still match files in a streaming fashion without having look at "obviously binary" files.
                log::debug!(
                    "Invalid UTF-8 sequence in file {abs_path:?} at byte position {starting_position}"
                );
                continue;
            }

            if query.detect(file).unwrap_or(false) {
                entry.respond.send(entry.path).await?
            }
        }

        Ok(())
    }

    pub async fn handle_create_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CreateProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let worktree = this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            this.worktree_for_id(worktree_id, cx)
                .context("worktree not found")
        })??;
        Worktree::handle_create_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_copy_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CopyProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .context("worktree not found")
        })??;
        Worktree::handle_copy_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_delete_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::DeleteProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .context("worktree not found")
        })??;
        Worktree::handle_delete_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_expand_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExpandProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ExpandProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this
            .update(&mut cx, |this, cx| this.worktree_for_entry(entry_id, cx))?
            .context("invalid request")?;
        Worktree::handle_expand_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_expand_all_for_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExpandAllForProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ExpandAllForProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this
            .update(&mut cx, |this, cx| this.worktree_for_entry(entry_id, cx))?
            .context("invalid request")?;
        Worktree::handle_expand_all_for_entry(worktree, envelope.payload, cx).await
    }

    pub fn fs(&self) -> Option<Arc<dyn Fs>> {
        match &self.state {
            WorktreeStoreState::Local { fs } => Some(fs.clone()),
            WorktreeStoreState::Remote { .. } => None,
        }
    }
}

#[derive(Clone, Debug)]
enum WorktreeHandle {
    Strong(Entity<Worktree>),
    Weak(WeakEntity<Worktree>),
}

impl WorktreeHandle {
    fn upgrade(&self) -> Option<Entity<Worktree>> {
        match self {
            WorktreeHandle::Strong(handle) => Some(handle.clone()),
            WorktreeHandle::Weak(handle) => handle.upgrade(),
        }
    }
}
