use std::{
    cmp,
    collections::VecDeque,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};

use anyhow::{anyhow, Context as _, Result};
use collections::{HashMap, HashSet};
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, EntityId, EventEmitter, Model, ModelContext, WeakModel};
use rpc::{
    proto::{self, AnyProtoClient},
    TypedEnvelope,
};
use smol::{
    channel::{Receiver, Sender},
    lock::Semaphore,
    stream::StreamExt,
};
use text::ReplicaId;
use util::ResultExt;
use worktree::{Entry, ProjectEntryId, Snapshot, Worktree, WorktreeId, WorktreeSettings};

use crate::{search::SearchQuery, ProjectPath};

pub struct WorktreeStore {
    is_shared: bool,
    worktrees: Vec<WorktreeHandle>,
    worktrees_reordered: bool,
}

pub enum WorktreeStoreEvent {
    WorktreeAdded(Model<Worktree>),
    WorktreeRemoved(EntityId, WorktreeId),
    WorktreeOrderChanged,
}

impl EventEmitter<WorktreeStoreEvent> for WorktreeStore {}

impl WorktreeStore {
    pub fn new(retain_worktrees: bool) -> Self {
        Self {
            is_shared: retain_worktrees,
            worktrees: Vec::new(),
            worktrees_reordered: false,
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

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<Model<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).contains_entry(entry_id))
    }

    pub fn entry_for_id<'a>(
        &'a self,
        entry_id: ProjectEntryId,
        cx: &'a AppContext,
    ) -> Option<&'a Entry> {
        self.worktrees()
            .find_map(|worktree| worktree.read(cx).entry_for_id(entry_id))
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

    /// search over all worktrees (ignoring open buffers)
    /// the query is tested against the file on disk and matching files are returned.
    pub fn find_search_candidates(
        &self,
        query: SearchQuery,
        limit: usize,
        skip_entries: HashSet<ProjectEntryId>,
        fs: Arc<dyn Fs>,
        cx: &ModelContext<Self>,
    ) -> Receiver<ProjectPath> {
        let (matching_paths_tx, matching_paths_rx) = smol::channel::bounded(1024);
        let snapshots = self
            .visible_worktrees(cx)
            .filter_map(|tree| {
                let tree = tree.read(cx);
                Some((tree.snapshot(), tree.as_local()?.settings()))
            })
            .collect::<Vec<_>>();
        let include_root = snapshots.len() > 1;
        let path_count: usize = snapshots
            .iter()
            .map(|(snapshot, _)| {
                if query.include_ignored() {
                    snapshot.file_count()
                } else {
                    snapshot.visible_file_count()
                }
            })
            .sum();

        let remaining_paths = AtomicUsize::new(limit);
        if path_count == 0 {
            return matching_paths_rx;
        }
        let workers = cx.background_executor().num_cpus().min(path_count);
        let paths_per_worker = (path_count + workers - 1) / workers;

        let executor = cx.background_executor().clone();
        cx.background_executor()
            .spawn(async move {
                let fs = &fs;
                let query = &query;
                let matching_paths_tx = &matching_paths_tx;
                let snapshots = &snapshots;
                let remaining_paths = &remaining_paths;

                executor
                    .scoped(move |scope| {
                        let max_concurrent_workers = Arc::new(Semaphore::new(workers));

                        for worker_ix in 0..workers {
                            let snapshots = snapshots.clone();
                            let worker_start_ix = worker_ix * paths_per_worker;
                            let worker_end_ix = worker_start_ix + paths_per_worker;
                            let skip_entries = skip_entries.clone();
                            let limiter = Arc::clone(&max_concurrent_workers);
                            scope.spawn(async move {
                                let _guard = limiter.acquire().await;
                                Self::search_snapshots(
                                    &snapshots,
                                    worker_start_ix,
                                    worker_end_ix,
                                    &query,
                                    remaining_paths,
                                    &matching_paths_tx,
                                    &skip_entries,
                                    include_root,
                                    fs,
                                )
                                .await;
                            });
                        }

                        if query.include_ignored() {
                            for (snapshot, settings) in snapshots {
                                for ignored_entry in
                                    snapshot.entries(true, 0).filter(|e| e.is_ignored)
                                {
                                    let limiter = Arc::clone(&max_concurrent_workers);
                                    scope.spawn(async move {
                                        let _guard = limiter.acquire().await;
                                        if remaining_paths.load(SeqCst) == 0 {
                                            return;
                                        }

                                        Self::search_ignored_entry(
                                            &snapshot,
                                            &settings,
                                            ignored_entry,
                                            &fs,
                                            &query,
                                            remaining_paths,
                                            &matching_paths_tx,
                                        )
                                        .await;
                                    });
                                }
                            }
                        }
                    })
                    .await
            })
            .detach();
        return matching_paths_rx;
    }

    #[allow(clippy::too_many_arguments)]
    async fn search_snapshots(
        snapshots: &Vec<(worktree::Snapshot, WorktreeSettings)>,
        worker_start_ix: usize,
        worker_end_ix: usize,
        query: &SearchQuery,
        remaining_paths: &AtomicUsize,
        results_tx: &Sender<ProjectPath>,
        skip_entries: &HashSet<ProjectEntryId>,
        include_root: bool,
        fs: &Arc<dyn Fs>,
    ) {
        let mut snapshot_start_ix = 0;
        let mut abs_path = PathBuf::new();

        for (snapshot, _) in snapshots {
            let snapshot_end_ix = snapshot_start_ix
                + if query.include_ignored() {
                    snapshot.file_count()
                } else {
                    snapshot.visible_file_count()
                };
            if worker_end_ix <= snapshot_start_ix {
                break;
            } else if worker_start_ix > snapshot_end_ix {
                snapshot_start_ix = snapshot_end_ix;
                continue;
            } else {
                let start_in_snapshot = worker_start_ix.saturating_sub(snapshot_start_ix);
                let end_in_snapshot = cmp::min(worker_end_ix, snapshot_end_ix) - snapshot_start_ix;

                for entry in snapshot
                    .files(false, start_in_snapshot)
                    .take(end_in_snapshot - start_in_snapshot)
                {
                    if results_tx.is_closed() {
                        break;
                    }
                    if skip_entries.contains(&entry.id) {
                        continue;
                    }
                    if entry.is_fifo {
                        continue;
                    }

                    let matched_path = if include_root {
                        let mut full_path = PathBuf::from(snapshot.root_name());
                        full_path.push(&entry.path);
                        query.file_matches(Some(&full_path))
                    } else {
                        query.file_matches(Some(&entry.path))
                    };

                    let matches = if matched_path {
                        abs_path.clear();
                        abs_path.push(&snapshot.abs_path());
                        abs_path.push(&entry.path);
                        if let Some(file) = fs.open_sync(&abs_path).await.log_err() {
                            query.detect(file).unwrap_or(false)
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if matches {
                        if remaining_paths
                            .fetch_update(SeqCst, SeqCst, |value| {
                                if value > 0 {
                                    Some(value - 1)
                                } else {
                                    None
                                }
                            })
                            .is_err()
                        {
                            return;
                        }

                        let project_path = ProjectPath {
                            worktree_id: snapshot.id(),
                            path: entry.path.clone(),
                        };
                        if results_tx.send(project_path).await.is_err() {
                            return;
                        }
                    }
                }

                snapshot_start_ix = snapshot_end_ix;
            }
        }
    }

    async fn search_ignored_entry(
        snapshot: &Snapshot,
        settings: &WorktreeSettings,
        ignored_entry: &Entry,
        fs: &Arc<dyn Fs>,
        query: &SearchQuery,
        remaining_paths: &AtomicUsize,
        counter_tx: &Sender<ProjectPath>,
    ) {
        let mut ignored_paths_to_process =
            VecDeque::from([snapshot.abs_path().join(&ignored_entry.path)]);

        while let Some(ignored_abs_path) = ignored_paths_to_process.pop_front() {
            let metadata = fs
                .metadata(&ignored_abs_path)
                .await
                .with_context(|| format!("fetching fs metadata for {ignored_abs_path:?}"))
                .log_err()
                .flatten();

            if let Some(fs_metadata) = metadata {
                if fs_metadata.is_dir {
                    let files = fs
                        .read_dir(&ignored_abs_path)
                        .await
                        .with_context(|| format!("listing ignored path {ignored_abs_path:?}"))
                        .log_err();

                    if let Some(mut subfiles) = files {
                        while let Some(subfile) = subfiles.next().await {
                            if let Some(subfile) = subfile.log_err() {
                                ignored_paths_to_process.push_back(subfile);
                            }
                        }
                    }
                } else if !fs_metadata.is_symlink {
                    if !query.file_matches(Some(&ignored_abs_path))
                        || settings.is_path_excluded(&ignored_entry.path)
                    {
                        continue;
                    }
                    let matches = if let Some(file) = fs
                        .open_sync(&ignored_abs_path)
                        .await
                        .with_context(|| format!("Opening ignored path {ignored_abs_path:?}"))
                        .log_err()
                    {
                        query.detect(file).unwrap_or(false)
                    } else {
                        false
                    };

                    if matches {
                        if remaining_paths
                            .fetch_update(SeqCst, SeqCst, |value| {
                                if value > 0 {
                                    Some(value - 1)
                                } else {
                                    None
                                }
                            })
                            .is_err()
                        {
                            return;
                        }

                        let project_path = ProjectPath {
                            worktree_id: snapshot.id(),
                            path: Arc::from(
                                ignored_abs_path
                                    .strip_prefix(snapshot.abs_path())
                                    .expect("scanning worktree-related files"),
                            ),
                        };
                        if counter_tx.send(project_path).await.is_err() {
                            return;
                        }
                    }
                }
            }
        }
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
