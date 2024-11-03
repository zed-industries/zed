use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, Arc},
};

use anyhow::{anyhow, Context as _, Result};
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
    proto::{self, SSH_PROJECT_ID},
    AnyProtoClient, ErrorExt, TypedEnvelope,
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

enum WorktreeStoreState {
    Local {
        fs: Arc<dyn Fs>,
    },
    Remote {
        upstream_client: AnyProtoClient,
        upstream_project_id: u64,
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
        HashMap<Arc<Path>, Shared<Task<Result<Model<Worktree>, Arc<anyhow::Error>>>>>,
    state: WorktreeStoreState,
}

pub enum WorktreeStoreEvent {
    WorktreeAdded(Model<Worktree>),
    WorktreeRemoved(EntityId, WorktreeId),
    WorktreeReleased(EntityId, WorktreeId),
    WorktreeOrderChanged,
    WorktreeUpdateSent(Model<Worktree>),
}

impl EventEmitter<WorktreeStoreEvent> for WorktreeStore {}

impl WorktreeStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_model_request_handler(Self::handle_create_project_entry);
        client.add_model_request_handler(Self::handle_rename_project_entry);
        client.add_model_request_handler(Self::handle_copy_project_entry);
        client.add_model_request_handler(Self::handle_delete_project_entry);
        client.add_model_request_handler(Self::handle_expand_project_entry);
        client.add_model_request_handler(Self::handle_git_branches);
        client.add_model_request_handler(Self::handle_update_branch);
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
            },
        }
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

    pub fn current_branch(&self, repository: ProjectPath, cx: &AppContext) -> Option<Arc<str>> {
        self.worktree_for_id(repository.worktree_id, cx)?
            .read(cx)
            .git_entry(repository.path)?
            .branch()
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

    pub fn find_or_create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(Model<Worktree>, PathBuf)>> {
        let abs_path = abs_path.as_ref();
        if let Some((tree, relative_path)) = self.find_worktree(abs_path, cx) {
            Task::ready(Ok((tree, relative_path)))
        } else {
            let worktree = self.create_worktree(abs_path, visible, cx);
            cx.background_executor()
                .spawn(async move { Ok((worktree.await?, PathBuf::new())) })
        }
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
            let task = match &self.state {
                WorktreeStoreState::Remote {
                    upstream_client, ..
                } => {
                    if upstream_client.is_via_collab() {
                        Task::ready(Err(Arc::new(anyhow!("cannot create worktrees via collab"))))
                    } else {
                        self.create_ssh_worktree(upstream_client.clone(), abs_path, visible, cx)
                    }
                }
                WorktreeStoreState::Local { fs } => {
                    self.create_local_worktree(fs.clone(), abs_path, visible, cx)
                }
            };

            self.loading_worktrees.insert(path.clone(), task.shared());
        }
        let task = self.loading_worktrees.get(&path).unwrap().clone();
        cx.spawn(|this, mut cx| async move {
            let result = task.await;
            this.update(&mut cx, |this, _| this.loading_worktrees.remove(&path))
                .ok();
            match result {
                Ok(worktree) => Ok(worktree),
                Err(err) => Err((*err).cloned()),
            }
        })
    }

    fn create_ssh_worktree(
        &mut self,
        client: AnyProtoClient,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>, Arc<anyhow::Error>>> {
        let path_key: Arc<Path> = abs_path.as_ref().into();
        let mut abs_path = path_key.clone().to_string_lossy().to_string();
        // If we start with `/~` that means the ssh path was something like `ssh://user@host/~/home-dir-folder/`
        // in which case want to strip the leading the `/`.
        // On the host-side, the `~` will get expanded.
        // That's what git does too: https://github.com/libgit2/libgit2/issues/3345#issuecomment-127050850
        if abs_path.starts_with("/~") {
            abs_path = abs_path[1..].to_string();
        }
        if abs_path.is_empty() || abs_path == "/" {
            abs_path = "~/".to_string();
        }
        cx.spawn(|this, mut cx| async move {
            let this = this.upgrade().context("Dropped worktree store")?;

            let response = client
                .request(proto::AddWorktree {
                    project_id: SSH_PROJECT_ID,
                    path: abs_path.clone(),
                    visible,
                })
                .await?;

            if let Some(existing_worktree) = this.read_with(&cx, |this, cx| {
                this.worktree_for_id(WorktreeId::from_proto(response.worktree_id), cx)
            })? {
                return Ok(existing_worktree);
            }

            let root_name = PathBuf::from(&response.canonicalized_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or(response.canonicalized_path.to_string());

            let worktree = cx.update(|cx| {
                Worktree::remote(
                    SSH_PROJECT_ID,
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

            this.update(&mut cx, |this, cx| {
                this.add(&worktree, cx);
            })?;
            Ok(worktree)
        })
    }

    fn create_local_worktree(
        &mut self,
        fs: Arc<dyn Fs>,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>, Arc<anyhow::Error>>> {
        let next_entry_id = self.next_entry_id.clone();
        let path: Arc<Path> = abs_path.as_ref().into();

        cx.spawn(move |this, mut cx| async move {
            let worktree = Worktree::local(path.clone(), visible, fs, next_entry_id, &mut cx).await;

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

    pub fn add(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
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

        let (client, project_id) = self
            .upstream_client()
            .clone()
            .ok_or_else(|| anyhow!("invalid project"))?;

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

    pub fn send_project_updates(&mut self, cx: &mut ModelContext<Self>) {
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
        cx.spawn(|this, mut cx| async move {
            if let Some(update_project) = update_project {
                update_project.await?;
            }

            this.update(&mut cx, |this, cx| {
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

    pub fn worktree_metadata_protos(&self, cx: &AppContext) -> Vec<proto::WorktreeMetadata> {
        self.worktrees()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                proto::WorktreeMetadata {
                    id: worktree.id().to_proto(),
                    root_name: worktree.root_name().into(),
                    visible: worktree.is_visible(),
                    abs_path: worktree.abs_path().to_string_lossy().into(),
                }
            })
            .collect()
    }

    pub fn shared(
        &mut self,
        remote_id: u64,
        downsteam_client: AnyProtoClient,
        cx: &mut ModelContext<Self>,
    ) {
        self.retain_worktrees = true;
        self.downstream_client = Some((downsteam_client, remote_id));

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

    pub fn unshared(&mut self, cx: &mut ModelContext<Self>) {
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
        matching_paths_rx
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

    pub fn branches(
        &self,
        project_path: ProjectPath,
        cx: &AppContext,
    ) -> Task<Result<Vec<git::repository::Branch>>> {
        let Some(worktree) = self.worktree_for_id(project_path.worktree_id, cx) else {
            return Task::ready(Err(anyhow!("No worktree found for ProjectPath")));
        };

        match worktree.read(cx) {
            Worktree::Local(local_worktree) => {
                let branches = util::maybe!({
                    let worktree_error = |error| {
                        format!(
                            "{} for worktree {}",
                            error,
                            local_worktree.abs_path().to_string_lossy()
                        )
                    };

                    let entry = local_worktree
                        .git_entry(project_path.path)
                        .with_context(|| worktree_error("No git entry found"))?;

                    let repo = local_worktree
                        .get_local_repo(&entry)
                        .with_context(|| worktree_error("No repository found"))?
                        .repo()
                        .clone();

                    repo.branches()
                });

                Task::ready(branches)
            }
            Worktree::Remote(remote_worktree) => {
                let request = remote_worktree.client().request(proto::GitBranches {
                    project_id: remote_worktree.project_id(),
                    repository: Some(proto::ProjectPath {
                        worktree_id: project_path.worktree_id.to_proto(),
                        path: project_path.path.to_string_lossy().to_string(), // Root path
                    }),
                });

                cx.background_executor().spawn(async move {
                    let response = request.await?;

                    let branches = response
                        .branches
                        .into_iter()
                        .map(|proto_branch| git::repository::Branch {
                            is_head: proto_branch.is_head,
                            name: proto_branch.name.into(),
                            unix_timestamp: proto_branch
                                .unix_timestamp
                                .map(|timestamp| timestamp as i64),
                        })
                        .collect();

                    Ok(branches)
                })
            }
        }
    }

    pub fn update_or_create_branch(
        &self,
        repository: ProjectPath,
        new_branch: String,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let Some(worktree) = self.worktree_for_id(repository.worktree_id, cx) else {
            return Task::ready(Err(anyhow!("No worktree found for ProjectPath")));
        };

        match worktree.read(cx) {
            Worktree::Local(local_worktree) => {
                let result = util::maybe!({
                    let worktree_error = |error| {
                        format!(
                            "{} for worktree {}",
                            error,
                            local_worktree.abs_path().to_string_lossy()
                        )
                    };

                    let entry = local_worktree
                        .git_entry(repository.path)
                        .with_context(|| worktree_error("No git entry found"))?;

                    let repo = local_worktree
                        .get_local_repo(&entry)
                        .with_context(|| worktree_error("No repository found"))?
                        .repo()
                        .clone();

                    if !repo.branch_exits(&new_branch)? {
                        repo.create_branch(&new_branch)?;
                    }

                    repo.change_branch(&new_branch)?;

                    Ok(())
                });

                Task::ready(result)
            }
            Worktree::Remote(remote_worktree) => {
                let request = remote_worktree.client().request(proto::UpdateGitBranch {
                    project_id: remote_worktree.project_id(),
                    repository: Some(proto::ProjectPath {
                        worktree_id: repository.worktree_id.to_proto(),
                        path: repository.path.to_string_lossy().to_string(), // Root path
                    }),
                    branch_name: new_branch,
                });

                cx.background_executor().spawn(async move {
                    request.await?;
                    Ok(())
                })
            }
        }
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

    pub async fn handle_git_branches(
        this: Model<Self>,
        branches: TypedEnvelope<proto::GitBranches>,
        cx: AsyncAppContext,
    ) -> Result<proto::GitBranchesResponse> {
        let project_path = branches
            .payload
            .repository
            .clone()
            .context("Invalid GitBranches call")?;
        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_proto(project_path.worktree_id),
            path: Path::new(&project_path.path).into(),
        };

        let branches = this
            .read_with(&cx, |this, cx| this.branches(project_path, cx))?
            .await?;

        Ok(proto::GitBranchesResponse {
            branches: branches
                .into_iter()
                .map(|branch| proto::Branch {
                    is_head: branch.is_head,
                    name: branch.name.to_string(),
                    unix_timestamp: branch.unix_timestamp.map(|timestamp| timestamp as u64),
                })
                .collect(),
        })
    }

    pub async fn handle_update_branch(
        this: Model<Self>,
        update_branch: TypedEnvelope<proto::UpdateGitBranch>,
        cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        let project_path = update_branch
            .payload
            .repository
            .clone()
            .context("Invalid GitBranches call")?;
        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_proto(project_path.worktree_id),
            path: Path::new(&project_path.path).into(),
        };
        let new_branch = update_branch.payload.branch_name;

        this.read_with(&cx, |this, cx| {
            this.update_or_create_branch(project_path, new_branch, cx)
        })?
        .await?;

        Ok(proto::Ack {})
    }
}

#[derive(Clone, Debug)]
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
