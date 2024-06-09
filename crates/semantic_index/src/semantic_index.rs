mod chunking;
mod embedding;
mod project_index_debug_view;

use anyhow::{anyhow, Context as _, Result};
use chunking::{chunk_text, Chunk};
use collections::{Bound, HashMap, HashSet};
pub use embedding::*;
use fs::Fs;
use futures::{future::Shared, stream::StreamExt, FutureExt};
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{
    AppContext, AsyncAppContext, BorrowAppContext, Context, Entity, EntityId, EventEmitter, Global,
    Model, ModelContext, Subscription, Task, WeakModel,
};
use heed::types::{SerdeBincode, Str};
use language::LanguageRegistry;
use parking_lot::Mutex;
use project::{Entry, Project, ProjectEntryId, UpdatedEntriesSet, Worktree, WorktreeId};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{
    cmp::Ordering,
    future::Future,
    iter,
    num::NonZeroUsize,
    ops::Range,
    path::{Path, PathBuf},
    sync::{Arc, Weak},
    time::{Duration, SystemTime},
};
use util::ResultExt;
use worktree::LocalSnapshot;

pub use project_index_debug_view::ProjectIndexDebugView;

pub struct SemanticIndex {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    db_connection: heed::Env,
    project_indices: HashMap<WeakModel<Project>, Model<ProjectIndex>>,
}

impl Global for SemanticIndex {}

impl SemanticIndex {
    pub async fn new(
        db_path: PathBuf,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        let db_connection = cx
            .background_executor()
            .spawn(async move {
                std::fs::create_dir_all(&db_path)?;
                unsafe {
                    heed::EnvOpenOptions::new()
                        .map_size(1024 * 1024 * 1024)
                        .max_dbs(3000)
                        .open(db_path)
                }
            })
            .await
            .context("opening database connection")?;

        Ok(SemanticIndex {
            db_connection,
            embedding_provider,
            project_indices: HashMap::default(),
        })
    }

    pub fn project_index(
        &mut self,
        project: Model<Project>,
        cx: &mut AppContext,
    ) -> Model<ProjectIndex> {
        let project_weak = project.downgrade();
        project.update(cx, move |_, cx| {
            cx.on_release(move |_, cx| {
                if cx.has_global::<SemanticIndex>() {
                    cx.update_global::<SemanticIndex, _>(|this, _| {
                        this.project_indices.remove(&project_weak);
                    })
                }
            })
            .detach();
        });

        self.project_indices
            .entry(project.downgrade())
            .or_insert_with(|| {
                cx.new_model(|cx| {
                    ProjectIndex::new(
                        project,
                        self.db_connection.clone(),
                        self.embedding_provider.clone(),
                        cx,
                    )
                })
            })
            .clone()
    }
}

pub struct ProjectIndex {
    db_connection: heed::Env,
    project: WeakModel<Project>,
    worktree_indices: HashMap<EntityId, WorktreeIndexHandle>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    last_status: Status,
    status_tx: channel::Sender<()>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    _maintain_status: Task<()>,
    _subscription: Subscription,
}

#[derive(Clone)]
enum WorktreeIndexHandle {
    Loading {
        index: Shared<Task<Result<Model<WorktreeIndex>, Arc<anyhow::Error>>>>,
    },
    Loaded {
        index: Model<WorktreeIndex>,
    },
}

impl ProjectIndex {
    fn new(
        project: Model<Project>,
        db_connection: heed::Env,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let language_registry = project.read(cx).languages().clone();
        let fs = project.read(cx).fs().clone();
        let (status_tx, mut status_rx) = channel::unbounded();
        let mut this = ProjectIndex {
            db_connection,
            project: project.downgrade(),
            worktree_indices: HashMap::default(),
            language_registry,
            fs,
            status_tx,
            last_status: Status::Idle,
            embedding_provider,
            _subscription: cx.subscribe(&project, Self::handle_project_event),
            _maintain_status: cx.spawn(|this, mut cx| async move {
                while status_rx.next().await.is_some() {
                    if this
                        .update(&mut cx, |this, cx| this.update_status(cx))
                        .is_err()
                    {
                        break;
                    }
                }
            }),
        };
        this.update_worktree_indices(cx);
        this
    }

    pub fn status(&self) -> Status {
        self.last_status
    }

    pub fn project(&self) -> WeakModel<Project> {
        self.project.clone()
    }

    pub fn fs(&self) -> Arc<dyn Fs> {
        self.fs.clone()
    }

    fn handle_project_event(
        &mut self,
        _: Model<Project>,
        event: &project::Event,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            project::Event::WorktreeAdded | project::Event::WorktreeRemoved(_) => {
                self.update_worktree_indices(cx);
            }
            _ => {}
        }
    }

    fn update_worktree_indices(&mut self, cx: &mut ModelContext<Self>) {
        let Some(project) = self.project.upgrade() else {
            return;
        };

        let worktrees = project
            .read(cx)
            .visible_worktrees(cx)
            .filter_map(|worktree| {
                if worktree.read(cx).is_local() {
                    Some((worktree.entity_id(), worktree))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        self.worktree_indices
            .retain(|worktree_id, _| worktrees.contains_key(worktree_id));
        for (worktree_id, worktree) in worktrees {
            self.worktree_indices.entry(worktree_id).or_insert_with(|| {
                let worktree_index = WorktreeIndex::load(
                    worktree.clone(),
                    self.db_connection.clone(),
                    self.language_registry.clone(),
                    self.fs.clone(),
                    self.status_tx.clone(),
                    self.embedding_provider.clone(),
                    cx,
                );

                let load_worktree = cx.spawn(|this, mut cx| async move {
                    let result = match worktree_index.await {
                        Ok(worktree_index) => {
                            this.update(&mut cx, |this, _| {
                                this.worktree_indices.insert(
                                    worktree_id,
                                    WorktreeIndexHandle::Loaded {
                                        index: worktree_index.clone(),
                                    },
                                );
                            })?;
                            Ok(worktree_index)
                        }
                        Err(error) => {
                            this.update(&mut cx, |this, _cx| {
                                this.worktree_indices.remove(&worktree_id)
                            })?;
                            Err(Arc::new(error))
                        }
                    };

                    this.update(&mut cx, |this, cx| this.update_status(cx))?;

                    result
                });

                WorktreeIndexHandle::Loading {
                    index: load_worktree.shared(),
                }
            });
        }

        self.update_status(cx);
    }

    fn update_status(&mut self, cx: &mut ModelContext<Self>) {
        let mut indexing_count = 0;
        let mut any_loading = false;

        for index in self.worktree_indices.values_mut() {
            match index {
                WorktreeIndexHandle::Loading { .. } => {
                    any_loading = true;
                    break;
                }
                WorktreeIndexHandle::Loaded { index, .. } => {
                    indexing_count += index.read(cx).entry_ids_being_indexed.len();
                }
            }
        }

        let status = if any_loading {
            Status::Loading
        } else if let Some(remaining_count) = NonZeroUsize::new(indexing_count) {
            Status::Scanning { remaining_count }
        } else {
            Status::Idle
        };

        if status != self.last_status {
            self.last_status = status;
            cx.emit(status);
        }
    }

    pub fn search(
        &self,
        query: String,
        limit: usize,
        cx: &AppContext,
    ) -> Task<Result<Vec<SearchResult>>> {
        let (chunks_tx, chunks_rx) = channel::bounded(1024);
        let mut worktree_scan_tasks = Vec::new();
        for worktree_index in self.worktree_indices.values() {
            let worktree_index = worktree_index.clone();
            let chunks_tx = chunks_tx.clone();
            worktree_scan_tasks.push(cx.spawn(|cx| async move {
                let index = match worktree_index {
                    WorktreeIndexHandle::Loading { index } => {
                        index.clone().await.map_err(|error| anyhow!(error))?
                    }
                    WorktreeIndexHandle::Loaded { index } => index.clone(),
                };

                index
                    .read_with(&cx, |index, cx| {
                        let worktree_id = index.worktree.read(cx).id();
                        let db_connection = index.db_connection.clone();
                        let db = index.db;
                        cx.background_executor().spawn(async move {
                            let txn = db_connection
                                .read_txn()
                                .context("failed to create read transaction")?;
                            let db_entries = db.iter(&txn).context("failed to iterate database")?;
                            for db_entry in db_entries {
                                let (_key, db_embedded_file) = db_entry?;
                                for chunk in db_embedded_file.chunks {
                                    chunks_tx
                                        .send((worktree_id, db_embedded_file.path.clone(), chunk))
                                        .await?;
                                }
                            }
                            anyhow::Ok(())
                        })
                    })?
                    .await
            }));
        }
        drop(chunks_tx);

        let project = self.project.clone();
        let embedding_provider = self.embedding_provider.clone();
        cx.spawn(|cx| async move {
            #[cfg(debug_assertions)]
            let embedding_query_start = std::time::Instant::now();
            log::info!("Searching for {query}");

            let query_embeddings = embedding_provider
                .embed(&[TextToEmbed::new(&query)])
                .await?;
            let query_embedding = query_embeddings
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("no embedding for query"))?;

            let mut results_by_worker = Vec::new();
            for _ in 0..cx.background_executor().num_cpus() {
                results_by_worker.push(Vec::<WorktreeSearchResult>::new());
            }

            #[cfg(debug_assertions)]
            let search_start = std::time::Instant::now();

            cx.background_executor()
                .scoped(|cx| {
                    for results in results_by_worker.iter_mut() {
                        cx.spawn(async {
                            while let Ok((worktree_id, path, chunk)) = chunks_rx.recv().await {
                                let score = chunk.embedding.similarity(&query_embedding);
                                let ix = match results.binary_search_by(|probe| {
                                    score.partial_cmp(&probe.score).unwrap_or(Ordering::Equal)
                                }) {
                                    Ok(ix) | Err(ix) => ix,
                                };
                                results.insert(
                                    ix,
                                    WorktreeSearchResult {
                                        worktree_id,
                                        path: path.clone(),
                                        range: chunk.chunk.range.clone(),
                                        score,
                                    },
                                );
                                results.truncate(limit);
                            }
                        });
                    }
                })
                .await;

            for scan_task in futures::future::join_all(worktree_scan_tasks).await {
                scan_task.log_err();
            }

            project.read_with(&cx, |project, cx| {
                let mut search_results = Vec::with_capacity(results_by_worker.len() * limit);
                for worker_results in results_by_worker {
                    search_results.extend(worker_results.into_iter().filter_map(|result| {
                        Some(SearchResult {
                            worktree: project.worktree_for_id(result.worktree_id, cx)?,
                            path: result.path,
                            range: result.range,
                            score: result.score,
                        })
                    }));
                }
                search_results.sort_unstable_by(|a, b| {
                    b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal)
                });
                search_results.truncate(limit);

                #[cfg(debug_assertions)]
                {
                    let search_elapsed = search_start.elapsed();
                    log::debug!(
                        "searched {} entries in {:?}",
                        search_results.len(),
                        search_elapsed
                    );
                    let embedding_query_elapsed = embedding_query_start.elapsed();
                    log::debug!("embedding query took {:?}", embedding_query_elapsed);
                }

                search_results
            })
        })
    }

    #[cfg(test)]
    pub fn path_count(&self, cx: &AppContext) -> Result<u64> {
        let mut result = 0;
        for worktree_index in self.worktree_indices.values() {
            if let WorktreeIndexHandle::Loaded { index, .. } = worktree_index {
                result += index.read(cx).path_count()?;
            }
        }
        Ok(result)
    }

    pub(crate) fn worktree_index(
        &self,
        worktree_id: WorktreeId,
        cx: &AppContext,
    ) -> Option<Model<WorktreeIndex>> {
        for index in self.worktree_indices.values() {
            if let WorktreeIndexHandle::Loaded { index, .. } = index {
                if index.read(cx).worktree.read(cx).id() == worktree_id {
                    return Some(index.clone());
                }
            }
        }
        None
    }

    pub(crate) fn worktree_indices(&self, cx: &AppContext) -> Vec<Model<WorktreeIndex>> {
        let mut result = self
            .worktree_indices
            .values()
            .filter_map(|index| {
                if let WorktreeIndexHandle::Loaded { index, .. } = index {
                    Some(index.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        result.sort_by_key(|index| index.read(cx).worktree.read(cx).id());
        result
    }
}

pub struct SearchResult {
    pub worktree: Model<Worktree>,
    pub path: Arc<Path>,
    pub range: Range<usize>,
    pub score: f32,
}

pub struct WorktreeSearchResult {
    pub worktree_id: WorktreeId,
    pub path: Arc<Path>,
    pub range: Range<usize>,
    pub score: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Idle,
    Loading,
    Scanning { remaining_count: NonZeroUsize },
}

impl EventEmitter<Status> for ProjectIndex {}

struct WorktreeIndex {
    worktree: Model<Worktree>,
    db_connection: heed::Env,
    db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    entry_ids_being_indexed: Arc<IndexingEntrySet>,
    _index_entries: Task<Result<()>>,
    _subscription: Subscription,
}

impl WorktreeIndex {
    pub fn load(
        worktree: Model<Worktree>,
        db_connection: heed::Env,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        status_tx: channel::Sender<()>,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        let worktree_abs_path = worktree.read(cx).abs_path();
        cx.spawn(|mut cx| async move {
            let db = cx
                .background_executor()
                .spawn({
                    let db_connection = db_connection.clone();
                    async move {
                        let mut txn = db_connection.write_txn()?;
                        let db_name = worktree_abs_path.to_string_lossy();
                        let db = db_connection.create_database(&mut txn, Some(&db_name))?;
                        txn.commit()?;
                        anyhow::Ok(db)
                    }
                })
                .await?;
            cx.new_model(|cx| {
                Self::new(
                    worktree,
                    db_connection,
                    db,
                    status_tx,
                    language_registry,
                    fs,
                    embedding_provider,
                    cx,
                )
            })
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        worktree: Model<Worktree>,
        db_connection: heed::Env,
        db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
        status: channel::Sender<()>,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let (updated_entries_tx, updated_entries_rx) = channel::unbounded();
        let _subscription = cx.subscribe(&worktree, move |_this, _worktree, event, _cx| {
            if let worktree::Event::UpdatedEntries(update) = event {
                _ = updated_entries_tx.try_send(update.clone());
            }
        });

        Self {
            db_connection,
            db,
            worktree,
            language_registry,
            fs,
            embedding_provider,
            entry_ids_being_indexed: Arc::new(IndexingEntrySet::new(status)),
            _index_entries: cx.spawn(|this, cx| Self::index_entries(this, updated_entries_rx, cx)),
            _subscription,
        }
    }

    async fn index_entries(
        this: WeakModel<Self>,
        updated_entries: channel::Receiver<UpdatedEntriesSet>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let index = this.update(&mut cx, |this, cx| this.index_entries_changed_on_disk(cx))?;
        index.await.log_err();

        while let Ok(updated_entries) = updated_entries.recv().await {
            let index = this.update(&mut cx, |this, cx| {
                this.index_updated_entries(updated_entries, cx)
            })?;
            index.await.log_err();
        }

        Ok(())
    }

    fn index_entries_changed_on_disk(&self, cx: &AppContext) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx).as_local().unwrap().snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_entries(worktree.clone(), cx);
        let chunk = self.chunk_files(worktree_abs_path, scan.updated_entries, cx);
        let embed = Self::embed_files(self.embedding_provider.clone(), chunk.files, cx);
        let persist = self.persist_embeddings(scan.deleted_entry_ranges, embed.files, cx);
        async move {
            futures::try_join!(scan.task, chunk.task, embed.task, persist)?;
            Ok(())
        }
    }

    fn index_updated_entries(
        &self,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx).as_local().unwrap().snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_updated_entries(worktree, updated_entries.clone(), cx);
        let chunk = self.chunk_files(worktree_abs_path, scan.updated_entries, cx);
        let embed = Self::embed_files(self.embedding_provider.clone(), chunk.files, cx);
        let persist = self.persist_embeddings(scan.deleted_entry_ranges, embed.files, cx);
        async move {
            futures::try_join!(scan.task, chunk.task, embed.task, persist)?;
            Ok(())
        }
    }

    fn scan_entries(&self, worktree: LocalSnapshot, cx: &AppContext) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) = channel::bounded(512);
        let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) = channel::bounded(128);
        let db_connection = self.db_connection.clone();
        let db = self.db;
        let entries_being_indexed = self.entry_ids_being_indexed.clone();
        let task = cx.background_executor().spawn(async move {
            let txn = db_connection
                .read_txn()
                .context("failed to create read transaction")?;
            let mut db_entries = db
                .iter(&txn)
                .context("failed to create iterator")?
                .move_between_keys()
                .peekable();

            let mut deletion_range: Option<(Bound<&str>, Bound<&str>)> = None;
            for entry in worktree.files(false, 0) {
                let entry_db_key = db_key_for_path(&entry.path);

                let mut saved_mtime = None;
                while let Some(db_entry) = db_entries.peek() {
                    match db_entry {
                        Ok((db_path, db_embedded_file)) => match (*db_path).cmp(&entry_db_key) {
                            Ordering::Less => {
                                if let Some(deletion_range) = deletion_range.as_mut() {
                                    deletion_range.1 = Bound::Included(db_path);
                                } else {
                                    deletion_range =
                                        Some((Bound::Included(db_path), Bound::Included(db_path)));
                                }

                                db_entries.next();
                            }
                            Ordering::Equal => {
                                if let Some(deletion_range) = deletion_range.take() {
                                    deleted_entry_ranges_tx
                                        .send((
                                            deletion_range.0.map(ToString::to_string),
                                            deletion_range.1.map(ToString::to_string),
                                        ))
                                        .await?;
                                }
                                saved_mtime = db_embedded_file.mtime;
                                db_entries.next();
                                break;
                            }
                            Ordering::Greater => {
                                break;
                            }
                        },
                        Err(_) => return Err(db_entries.next().unwrap().unwrap_err())?,
                    }
                }

                if entry.mtime != saved_mtime {
                    let handle = entries_being_indexed.insert(entry.id);
                    updated_entries_tx.send((entry.clone(), handle)).await?;
                }
            }

            if let Some(db_entry) = db_entries.next() {
                let (db_path, _) = db_entry?;
                deleted_entry_ranges_tx
                    .send((Bound::Included(db_path.to_string()), Bound::Unbounded))
                    .await?;
            }

            Ok(())
        });

        ScanEntries {
            updated_entries: updated_entries_rx,
            deleted_entry_ranges: deleted_entry_ranges_rx,
            task,
        }
    }

    fn scan_updated_entries(
        &self,
        worktree: LocalSnapshot,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) = channel::bounded(512);
        let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) = channel::bounded(128);
        let entries_being_indexed = self.entry_ids_being_indexed.clone();
        let task = cx.background_executor().spawn(async move {
            for (path, entry_id, status) in updated_entries.iter() {
                match status {
                    project::PathChange::Added
                    | project::PathChange::Updated
                    | project::PathChange::AddedOrUpdated => {
                        if let Some(entry) = worktree.entry_for_id(*entry_id) {
                            if entry.is_file() {
                                let handle = entries_being_indexed.insert(entry.id);
                                updated_entries_tx.send((entry.clone(), handle)).await?;
                            }
                        }
                    }
                    project::PathChange::Removed => {
                        let db_path = db_key_for_path(path);
                        deleted_entry_ranges_tx
                            .send((Bound::Included(db_path.clone()), Bound::Included(db_path)))
                            .await?;
                    }
                    project::PathChange::Loaded => {
                        // Do nothing.
                    }
                }
            }

            Ok(())
        });

        ScanEntries {
            updated_entries: updated_entries_rx,
            deleted_entry_ranges: deleted_entry_ranges_rx,
            task,
        }
    }

    fn chunk_files(
        &self,
        worktree_abs_path: Arc<Path>,
        entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
        cx: &AppContext,
    ) -> ChunkFiles {
        let language_registry = self.language_registry.clone();
        let fs = self.fs.clone();
        let (chunked_files_tx, chunked_files_rx) = channel::bounded(2048);
        let task = cx.spawn(|cx| async move {
            cx.background_executor()
                .scoped(|cx| {
                    for _ in 0..cx.num_cpus() {
                        cx.spawn(async {
                            while let Ok((entry, handle)) = entries.recv().await {
                                let entry_abs_path = worktree_abs_path.join(&entry.path);
                                let Some(text) = fs
                                    .load(&entry_abs_path)
                                    .await
                                    .with_context(|| {
                                        format!("failed to read path {entry_abs_path:?}")
                                    })
                                    .log_err()
                                else {
                                    continue;
                                };
                                let language = language_registry
                                    .language_for_file_path(&entry.path)
                                    .await
                                    .ok();
                                let chunked_file = ChunkedFile {
                                    chunks: chunk_text(&text, language.as_ref(), &entry.path),
                                    handle,
                                    path: entry.path,
                                    mtime: entry.mtime,
                                    text,
                                };

                                if chunked_files_tx.send(chunked_file).await.is_err() {
                                    return;
                                }
                            }
                        });
                    }
                })
                .await;
            Ok(())
        });

        ChunkFiles {
            files: chunked_files_rx,
            task,
        }
    }

    fn embed_files(
        embedding_provider: Arc<dyn EmbeddingProvider>,
        chunked_files: channel::Receiver<ChunkedFile>,
        cx: &AppContext,
    ) -> EmbedFiles {
        let embedding_provider = embedding_provider.clone();
        let (embedded_files_tx, embedded_files_rx) = channel::bounded(512);
        let task = cx.background_executor().spawn(async move {
            let mut chunked_file_batches =
                chunked_files.chunks_timeout(512, Duration::from_secs(2));
            while let Some(chunked_files) = chunked_file_batches.next().await {
                // View the batch of files as a vec of chunks
                // Flatten out to a vec of chunks that we can subdivide into batch sized pieces
                // Once those are done, reassemble them back into the files in which they belong
                // If any embeddings fail for a file, the entire file is discarded

                let chunks: Vec<TextToEmbed> = chunked_files
                    .iter()
                    .flat_map(|file| {
                        file.chunks.iter().map(|chunk| TextToEmbed {
                            text: &file.text[chunk.range.clone()],
                            digest: chunk.digest,
                        })
                    })
                    .collect::<Vec<_>>();

                let mut embeddings: Vec<Option<Embedding>> = Vec::new();
                for embedding_batch in chunks.chunks(embedding_provider.batch_size()) {
                    if let Some(batch_embeddings) =
                        embedding_provider.embed(embedding_batch).await.log_err()
                    {
                        if batch_embeddings.len() == embedding_batch.len() {
                            embeddings.extend(batch_embeddings.into_iter().map(Some));
                            continue;
                        }
                        log::error!(
                            "embedding provider returned unexpected embedding count {}, expected {}",
                            batch_embeddings.len(), embedding_batch.len()
                        );
                    }

                    embeddings.extend(iter::repeat(None).take(embedding_batch.len()));
                }

                let mut embeddings = embeddings.into_iter();
                for chunked_file in chunked_files {
                    let mut embedded_file = EmbeddedFile {
                        path: chunked_file.path,
                        mtime: chunked_file.mtime,
                        chunks: Vec::new(),
                    };

                    let mut embedded_all_chunks = true;
                    for (chunk, embedding) in
                        chunked_file.chunks.into_iter().zip(embeddings.by_ref())
                    {
                        if let Some(embedding) = embedding {
                            embedded_file
                                .chunks
                                .push(EmbeddedChunk { chunk, embedding });
                        } else {
                            embedded_all_chunks = false;
                        }
                    }

                    if embedded_all_chunks {
                        embedded_files_tx
                            .send((embedded_file, chunked_file.handle))
                            .await?;
                    }
                }
            }
            Ok(())
        });

        EmbedFiles {
            files: embedded_files_rx,
            task,
        }
    }

    fn persist_embeddings(
        &self,
        mut deleted_entry_ranges: channel::Receiver<(Bound<String>, Bound<String>)>,
        embedded_files: channel::Receiver<(EmbeddedFile, IndexingEntryHandle)>,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let db_connection = self.db_connection.clone();
        let db = self.db;
        cx.background_executor().spawn(async move {
            while let Some(deletion_range) = deleted_entry_ranges.next().await {
                let mut txn = db_connection.write_txn()?;
                let start = deletion_range.0.as_ref().map(|start| start.as_str());
                let end = deletion_range.1.as_ref().map(|end| end.as_str());
                log::debug!("deleting embeddings in range {:?}", &(start, end));
                db.delete_range(&mut txn, &(start, end))?;
                txn.commit()?;
            }

            let mut embedded_files = embedded_files.chunks_timeout(4096, Duration::from_secs(2));
            while let Some(embedded_files) = embedded_files.next().await {
                let mut txn = db_connection.write_txn()?;
                for (file, _) in &embedded_files {
                    log::debug!("saving embedding for file {:?}", file.path);
                    let key = db_key_for_path(&file.path);
                    db.put(&mut txn, &key, file)?;
                }
                txn.commit()?;

                drop(embedded_files);
                log::debug!("committed");
            }

            Ok(())
        })
    }

    fn paths(&self, cx: &AppContext) -> Task<Result<Vec<Arc<Path>>>> {
        let connection = self.db_connection.clone();
        let db = self.db;
        cx.background_executor().spawn(async move {
            let tx = connection
                .read_txn()
                .context("failed to create read transaction")?;
            let result = db
                .iter(&tx)?
                .map(|entry| Ok(entry?.1.path.clone()))
                .collect::<Result<Vec<Arc<Path>>>>();
            drop(tx);
            result
        })
    }

    fn chunks_for_path(
        &self,
        path: Arc<Path>,
        cx: &AppContext,
    ) -> Task<Result<Vec<EmbeddedChunk>>> {
        let connection = self.db_connection.clone();
        let db = self.db;
        cx.background_executor().spawn(async move {
            let tx = connection
                .read_txn()
                .context("failed to create read transaction")?;
            Ok(db
                .get(&tx, &db_key_for_path(&path))?
                .ok_or_else(|| anyhow!("no such path"))?
                .chunks
                .clone())
        })
    }

    #[cfg(test)]
    fn path_count(&self) -> Result<u64> {
        let txn = self
            .db_connection
            .read_txn()
            .context("failed to create read transaction")?;
        Ok(self.db.len(&txn)?)
    }
}

struct ScanEntries {
    updated_entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
    deleted_entry_ranges: channel::Receiver<(Bound<String>, Bound<String>)>,
    task: Task<Result<()>>,
}

struct ChunkFiles {
    files: channel::Receiver<ChunkedFile>,
    task: Task<Result<()>>,
}

struct ChunkedFile {
    pub path: Arc<Path>,
    pub mtime: Option<SystemTime>,
    pub handle: IndexingEntryHandle,
    pub text: String,
    pub chunks: Vec<Chunk>,
}

struct EmbedFiles {
    files: channel::Receiver<(EmbeddedFile, IndexingEntryHandle)>,
    task: Task<Result<()>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddedFile {
    path: Arc<Path>,
    mtime: Option<SystemTime>,
    chunks: Vec<EmbeddedChunk>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EmbeddedChunk {
    chunk: Chunk,
    embedding: Embedding,
}

/// The set of entries that are currently being indexed.
struct IndexingEntrySet {
    entry_ids: Mutex<HashSet<ProjectEntryId>>,
    tx: channel::Sender<()>,
}

/// When dropped, removes the entry from the set of entries that are being indexed.
#[derive(Clone)]
struct IndexingEntryHandle {
    entry_id: ProjectEntryId,
    set: Weak<IndexingEntrySet>,
}

impl IndexingEntrySet {
    fn new(tx: channel::Sender<()>) -> Self {
        Self {
            entry_ids: Default::default(),
            tx,
        }
    }

    fn insert(self: &Arc<Self>, entry_id: ProjectEntryId) -> IndexingEntryHandle {
        self.entry_ids.lock().insert(entry_id);
        self.tx.send_blocking(()).ok();
        IndexingEntryHandle {
            entry_id,
            set: Arc::downgrade(self),
        }
    }

    pub fn len(&self) -> usize {
        self.entry_ids.lock().len()
    }
}

impl Drop for IndexingEntryHandle {
    fn drop(&mut self) {
        if let Some(set) = self.set.upgrade() {
            set.tx.send_blocking(()).ok();
            set.entry_ids.lock().remove(&self.entry_id);
        }
    }
}

fn db_key_for_path(path: &Arc<Path>) -> String {
    path.to_string_lossy().replace('/', "\0")
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{future::BoxFuture, FutureExt};
    use gpui::TestAppContext;
    use language::language_settings::AllLanguageSettings;
    use project::Project;
    use settings::SettingsStore;
    use std::{future, path::Path, sync::Arc};

    fn init_test(cx: &mut TestAppContext) {
        _ = cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            language::init(cx);
            Project::init_settings(cx);
            SettingsStore::update(cx, |store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
            });
        });
    }

    pub struct TestEmbeddingProvider {
        batch_size: usize,
        compute_embedding: Box<dyn Fn(&str) -> Result<Embedding> + Send + Sync>,
    }

    impl TestEmbeddingProvider {
        pub fn new(
            batch_size: usize,
            compute_embedding: impl 'static + Fn(&str) -> Result<Embedding> + Send + Sync,
        ) -> Self {
            return Self {
                batch_size,
                compute_embedding: Box::new(compute_embedding),
            };
        }
    }

    impl EmbeddingProvider for TestEmbeddingProvider {
        fn embed<'a>(
            &'a self,
            texts: &'a [TextToEmbed<'a>],
        ) -> BoxFuture<'a, Result<Vec<Embedding>>> {
            let embeddings = texts
                .iter()
                .map(|to_embed| (self.compute_embedding)(to_embed.text))
                .collect();
            future::ready(embeddings).boxed()
        }

        fn batch_size(&self) -> usize {
            self.batch_size
        }
    }

    #[gpui::test]
    async fn test_search(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        init_test(cx);

        let temp_dir = tempfile::tempdir().unwrap();

        let mut semantic_index = SemanticIndex::new(
            temp_dir.path().into(),
            Arc::new(TestEmbeddingProvider::new(16, |text| {
                let mut embedding = vec![0f32; 2];
                // if the text contains garbage, give it a 1 in the first dimension
                if text.contains("garbage in") {
                    embedding[0] = 0.9;
                } else {
                    embedding[0] = -0.9;
                }

                if text.contains("garbage out") {
                    embedding[1] = 0.9;
                } else {
                    embedding[1] = -0.9;
                }

                Ok(Embedding::new(embedding))
            })),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let project_path = Path::new("./fixture");

        let project = cx
            .spawn(|mut cx| async move { Project::example([project_path], &mut cx).await })
            .await;

        cx.update(|cx| {
            let language_registry = project.read(cx).languages().clone();
            let node_runtime = project.read(cx).node_runtime().unwrap().clone();
            languages::init(language_registry, node_runtime, cx);
        });

        let project_index = cx.update(|cx| semantic_index.project_index(project.clone(), cx));

        while project_index
            .read_with(cx, |index, cx| index.path_count(cx))
            .unwrap()
            == 0
        {
            project_index.next_event(cx).await;
        }

        let results = cx
            .update(|cx| {
                let project_index = project_index.read(cx);
                let query = "garbage in, garbage out";
                project_index.search(query.into(), 4, cx)
            })
            .await
            .unwrap();

        assert!(results.len() > 1, "should have found some results");

        for result in &results {
            println!("result: {:?}", result.path);
            println!("score: {:?}", result.score);
        }

        // Find result that is greater than 0.5
        let search_result = results.iter().find(|result| result.score > 0.9).unwrap();

        assert_eq!(search_result.path.to_string_lossy(), "needle.md");

        let content = cx
            .update(|cx| {
                let worktree = search_result.worktree.read(cx);
                let entry_abs_path = worktree.abs_path().join(&search_result.path);
                let fs = project.read(cx).fs().clone();
                cx.background_executor()
                    .spawn(async move { fs.load(&entry_abs_path).await.unwrap() })
            })
            .await;

        let range = search_result.range.clone();
        let content = content[range.clone()].to_owned();

        assert!(content.contains("garbage in, garbage out"));
    }

    #[gpui::test]
    async fn test_embed_files(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let provider = Arc::new(TestEmbeddingProvider::new(3, |text| {
            if text.contains('g') {
                Err(anyhow!("cannot embed text containing a 'g' character"))
            } else {
                Ok(Embedding::new(
                    ('a'..'z')
                        .map(|char| text.chars().filter(|c| *c == char).count() as f32)
                        .collect(),
                ))
            }
        }));

        let (indexing_progress_tx, _) = channel::unbounded();
        let indexing_entries = Arc::new(IndexingEntrySet::new(indexing_progress_tx));

        let (chunked_files_tx, chunked_files_rx) = channel::unbounded::<ChunkedFile>();
        chunked_files_tx
            .send_blocking(ChunkedFile {
                path: Path::new("test1.md").into(),
                mtime: None,
                handle: indexing_entries.insert(ProjectEntryId::from_proto(0)),
                text: "abcdefghijklmnop".to_string(),
                chunks: [0..4, 4..8, 8..12, 12..16]
                    .into_iter()
                    .map(|range| Chunk {
                        range,
                        digest: Default::default(),
                    })
                    .collect(),
            })
            .unwrap();
        chunked_files_tx
            .send_blocking(ChunkedFile {
                path: Path::new("test2.md").into(),
                mtime: None,
                handle: indexing_entries.insert(ProjectEntryId::from_proto(1)),
                text: "qrstuvwxyz".to_string(),
                chunks: [0..4, 4..8, 8..10]
                    .into_iter()
                    .map(|range| Chunk {
                        range,
                        digest: Default::default(),
                    })
                    .collect(),
            })
            .unwrap();
        chunked_files_tx.close();

        let embed_files_task =
            cx.update(|cx| WorktreeIndex::embed_files(provider.clone(), chunked_files_rx, cx));
        embed_files_task.task.await.unwrap();

        let mut embedded_files_rx = embed_files_task.files;
        let mut embedded_files = Vec::new();
        while let Some((embedded_file, _)) = embedded_files_rx.next().await {
            embedded_files.push(embedded_file);
        }

        assert_eq!(embedded_files.len(), 1);
        assert_eq!(embedded_files[0].path.as_ref(), Path::new("test2.md"));
        assert_eq!(
            embedded_files[0]
                .chunks
                .iter()
                .map(|embedded_chunk| { embedded_chunk.embedding.clone() })
                .collect::<Vec<Embedding>>(),
            vec![
                (provider.compute_embedding)("qrst").unwrap(),
                (provider.compute_embedding)("uvwx").unwrap(),
                (provider.compute_embedding)("yz").unwrap(),
            ],
        );
    }
}
