mod chunking;
mod embedding;

use anyhow::{anyhow, Context as _, Result};
use chunking::{chunk_text, Chunk};
use collections::{Bound, HashMap};
pub use embedding::*;
use fs::Fs;
use futures::stream::StreamExt;
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{
    AppContext, AsyncAppContext, Context, EntityId, EventEmitter, Global, Model, ModelContext,
    Subscription, Task, WeakModel,
};
use heed::types::{SerdeBincode, Str};
use language::LanguageRegistry;
use project::{Entry, Project, UpdatedEntriesSet, Worktree};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{
    cmp::Ordering,
    future::Future,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};
use util::ResultExt;
use worktree::LocalSnapshot;

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
    project: Model<Project>,
    worktree_indices: HashMap<EntityId, WorktreeIndexHandle>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    pub last_status: Status,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    _subscription: Subscription,
}

enum WorktreeIndexHandle {
    Loading {
        _task: Task<Result<()>>,
    },
    Loaded {
        index: Model<WorktreeIndex>,
        _subscription: Subscription,
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
        let mut this = ProjectIndex {
            db_connection,
            project: project.clone(),
            worktree_indices: HashMap::default(),
            language_registry,
            fs,
            last_status: Status::Idle,
            embedding_provider,
            _subscription: cx.subscribe(&project, Self::handle_project_event),
        };
        this.update_worktree_indices(cx);
        this
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
        let worktrees = self
            .project
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
                    self.embedding_provider.clone(),
                    cx,
                );

                let load_worktree = cx.spawn(|this, mut cx| async move {
                    if let Some(index) = worktree_index.await.log_err() {
                        this.update(&mut cx, |this, cx| {
                            this.worktree_indices.insert(
                                worktree_id,
                                WorktreeIndexHandle::Loaded {
                                    _subscription: cx
                                        .observe(&index, |this, _, cx| this.update_status(cx)),
                                    index,
                                },
                            );
                        })?;
                    } else {
                        this.update(&mut cx, |this, _cx| {
                            this.worktree_indices.remove(&worktree_id)
                        })?;
                    }

                    this.update(&mut cx, |this, cx| this.update_status(cx))
                });

                WorktreeIndexHandle::Loading {
                    _task: load_worktree,
                }
            });
        }

        self.update_status(cx);
    }

    fn update_status(&mut self, cx: &mut ModelContext<Self>) {
        let mut status = Status::Idle;
        for index in self.worktree_indices.values() {
            match index {
                WorktreeIndexHandle::Loading { .. } => {
                    status = Status::Scanning;
                    break;
                }
                WorktreeIndexHandle::Loaded { index, .. } => {
                    if index.read(cx).status == Status::Scanning {
                        status = Status::Scanning;
                        break;
                    }
                }
            }
        }

        if status != self.last_status {
            self.last_status = status;
            cx.emit(status);
        }
    }

    pub fn search(&self, query: &str, limit: usize, cx: &AppContext) -> Task<Vec<SearchResult>> {
        let mut worktree_searches = Vec::new();
        for worktree_index in self.worktree_indices.values() {
            if let WorktreeIndexHandle::Loaded { index, .. } = worktree_index {
                worktree_searches
                    .push(index.read_with(cx, |index, cx| index.search(query, limit, cx)));
            }
        }

        cx.spawn(|_| async move {
            let mut results = Vec::new();
            let worktree_searches = futures::future::join_all(worktree_searches).await;

            for worktree_search_results in worktree_searches {
                if let Some(worktree_search_results) = worktree_search_results.log_err() {
                    results.extend(worktree_search_results);
                }
            }

            results
                .sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
            results.truncate(limit);

            results
        })
    }
}

pub struct SearchResult {
    pub worktree: Model<Worktree>,
    pub path: Arc<Path>,
    pub range: Range<usize>,
    pub score: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Idle,
    Scanning,
}

impl EventEmitter<Status> for ProjectIndex {}

struct WorktreeIndex {
    worktree: Model<Worktree>,
    db_connection: heed::Env,
    db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    status: Status,
    _index_entries: Task<Result<()>>,
    _subscription: Subscription,
}

impl WorktreeIndex {
    pub fn load(
        worktree: Model<Worktree>,
        db_connection: heed::Env,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
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
                    language_registry,
                    fs,
                    embedding_provider,
                    cx,
                )
            })
        })
    }

    fn new(
        worktree: Model<Worktree>,
        db_connection: heed::Env,
        db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
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
            status: Status::Idle,
            _index_entries: cx.spawn(|this, cx| Self::index_entries(this, updated_entries_rx, cx)),
            _subscription,
        }
    }

    async fn index_entries(
        this: WeakModel<Self>,
        updated_entries: channel::Receiver<UpdatedEntriesSet>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let index = this.update(&mut cx, |this, cx| {
            cx.notify();
            this.status = Status::Scanning;
            this.index_entries_changed_on_disk(cx)
        })?;
        index.await.log_err();
        this.update(&mut cx, |this, cx| {
            this.status = Status::Idle;
            cx.notify();
        })?;

        while let Ok(updated_entries) = updated_entries.recv().await {
            let index = this.update(&mut cx, |this, cx| {
                cx.notify();
                this.status = Status::Scanning;
                this.index_updated_entries(updated_entries, cx)
            })?;
            index.await.log_err();
            this.update(&mut cx, |this, cx| {
                this.status = Status::Idle;
                cx.notify();
            })?;
        }

        Ok(())
    }

    fn index_entries_changed_on_disk(&self, cx: &AppContext) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx).as_local().unwrap().snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_entries(worktree.clone(), cx);
        let chunk = self.chunk_files(worktree_abs_path, scan.updated_entries, cx);
        let embed = self.embed_files(chunk.files, cx);
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
        let embed = self.embed_files(chunk.files, cx);
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
                    updated_entries_tx.send(entry.clone()).await?;
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
        let task = cx.background_executor().spawn(async move {
            for (path, entry_id, status) in updated_entries.iter() {
                match status {
                    project::PathChange::Added
                    | project::PathChange::Updated
                    | project::PathChange::AddedOrUpdated => {
                        if let Some(entry) = worktree.entry_for_id(*entry_id) {
                            if entry.is_file() {
                                updated_entries_tx.send(entry.clone()).await?;
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
        entries: channel::Receiver<Entry>,
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
                            while let Ok(entry) = entries.recv().await {
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
                                let grammar =
                                    language.as_ref().and_then(|language| language.grammar());
                                let chunked_file = ChunkedFile {
                                    worktree_root: worktree_abs_path.clone(),
                                    chunks: chunk_text(&text, grammar),
                                    entry,
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
        &self,
        chunked_files: channel::Receiver<ChunkedFile>,
        cx: &AppContext,
    ) -> EmbedFiles {
        let embedding_provider = self.embedding_provider.clone();
        let (embedded_files_tx, embedded_files_rx) = channel::bounded(512);
        let task = cx.background_executor().spawn(async move {
            let mut chunked_file_batches =
                chunked_files.chunks_timeout(512, Duration::from_secs(2));
            while let Some(chunked_files) = chunked_file_batches.next().await {
                // View the batch of files as a vec of chunks
                // Flatten out to a vec of chunks that we can subdivide into batch sized pieces
                // Once those are done, reassemble it back into which files they belong to

                let chunks = chunked_files
                    .iter()
                    .flat_map(|file| {
                        file.chunks.iter().map(|chunk| TextToEmbed {
                            text: &file.text[chunk.range.clone()],
                            digest: chunk.digest,
                        })
                    })
                    .collect::<Vec<_>>();

                let mut embeddings = Vec::new();
                for embedding_batch in chunks.chunks(embedding_provider.batch_size()) {
                    embeddings.extend(embedding_provider.embed(embedding_batch).await?);
                }

                let mut embeddings = embeddings.into_iter();
                for chunked_file in chunked_files {
                    let chunk_embeddings = embeddings
                        .by_ref()
                        .take(chunked_file.chunks.len())
                        .collect::<Vec<_>>();
                    let embedded_chunks = chunked_file
                        .chunks
                        .into_iter()
                        .zip(chunk_embeddings)
                        .map(|(chunk, embedding)| EmbeddedChunk { chunk, embedding })
                        .collect();
                    let embedded_file = EmbeddedFile {
                        path: chunked_file.entry.path.clone(),
                        mtime: chunked_file.entry.mtime,
                        chunks: embedded_chunks,
                    };

                    embedded_files_tx.send(embedded_file).await?;
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
        embedded_files: channel::Receiver<EmbeddedFile>,
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
                for file in embedded_files {
                    log::debug!("saving embedding for file {:?}", file.path);
                    let key = db_key_for_path(&file.path);
                    db.put(&mut txn, &key, &file)?;
                }
                txn.commit()?;
                log::debug!("committed");
            }

            Ok(())
        })
    }

    fn search(
        &self,
        query: &str,
        limit: usize,
        cx: &AppContext,
    ) -> Task<Result<Vec<SearchResult>>> {
        let (chunks_tx, chunks_rx) = channel::bounded(1024);

        let db_connection = self.db_connection.clone();
        let db = self.db;
        let scan_chunks = cx.background_executor().spawn({
            async move {
                let txn = db_connection
                    .read_txn()
                    .context("failed to create read transaction")?;
                let db_entries = db.iter(&txn).context("failed to iterate database")?;
                for db_entry in db_entries {
                    let (_key, db_embedded_file) = db_entry?;
                    for chunk in db_embedded_file.chunks {
                        chunks_tx
                            .send((db_embedded_file.path.clone(), chunk))
                            .await?;
                    }
                }
                anyhow::Ok(())
            }
        });

        let query = query.to_string();
        let embedding_provider = self.embedding_provider.clone();
        let worktree = self.worktree.clone();
        cx.spawn(|cx| async move {
            #[cfg(debug_assertions)]
            let embedding_query_start = std::time::Instant::now();
            log::info!("Searching for {query}");

            let mut query_embeddings = embedding_provider
                .embed(&[TextToEmbed::new(&query)])
                .await?;
            let query_embedding = query_embeddings
                .pop()
                .ok_or_else(|| anyhow!("no embedding for query"))?;
            let mut workers = Vec::new();
            for _ in 0..cx.background_executor().num_cpus() {
                workers.push(Vec::<SearchResult>::new());
            }

            #[cfg(debug_assertions)]
            let search_start = std::time::Instant::now();

            cx.background_executor()
                .scoped(|cx| {
                    for worker_results in workers.iter_mut() {
                        cx.spawn(async {
                            while let Ok((path, embedded_chunk)) = chunks_rx.recv().await {
                                let score = embedded_chunk.embedding.similarity(&query_embedding);
                                let ix = match worker_results.binary_search_by(|probe| {
                                    score.partial_cmp(&probe.score).unwrap_or(Ordering::Equal)
                                }) {
                                    Ok(ix) | Err(ix) => ix,
                                };
                                worker_results.insert(
                                    ix,
                                    SearchResult {
                                        worktree: worktree.clone(),
                                        path: path.clone(),
                                        range: embedded_chunk.chunk.range.clone(),
                                        score,
                                    },
                                );
                                worker_results.truncate(limit);
                            }
                        });
                    }
                })
                .await;
            scan_chunks.await?;

            let mut search_results = Vec::with_capacity(workers.len() * limit);
            for worker_results in workers {
                search_results.extend(worker_results);
            }
            search_results
                .sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
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

            Ok(search_results)
        })
    }
}

struct ScanEntries {
    updated_entries: channel::Receiver<Entry>,
    deleted_entry_ranges: channel::Receiver<(Bound<String>, Bound<String>)>,
    task: Task<Result<()>>,
}

struct ChunkFiles {
    files: channel::Receiver<ChunkedFile>,
    task: Task<Result<()>>,
}

struct ChunkedFile {
    #[allow(dead_code)]
    pub worktree_root: Arc<Path>,
    pub entry: Entry,
    pub text: String,
    pub chunks: Vec<Chunk>,
}

struct EmbedFiles {
    files: channel::Receiver<EmbeddedFile>,
    task: Task<Result<()>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddedFile {
    path: Arc<Path>,
    mtime: Option<SystemTime>,
    chunks: Vec<EmbeddedChunk>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddedChunk {
    chunk: Chunk,
    embedding: Embedding,
}

fn db_key_for_path(path: &Arc<Path>) -> String {
    path.to_string_lossy().replace('/', "\0")
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::channel::oneshot;
    use futures::{future::BoxFuture, FutureExt};

    use gpui::{Global, TestAppContext};
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

    pub struct TestEmbeddingProvider;

    impl EmbeddingProvider for TestEmbeddingProvider {
        fn embed<'a>(
            &'a self,
            texts: &'a [TextToEmbed<'a>],
        ) -> BoxFuture<'a, Result<Vec<Embedding>>> {
            let embeddings = texts
                .iter()
                .map(|text| {
                    let mut embedding = vec![0f32; 2];
                    // if the text contains garbage, give it a 1 in the first dimension
                    if text.text.contains("garbage in") {
                        embedding[0] = 0.9;
                    } else {
                        embedding[0] = -0.9;
                    }

                    if text.text.contains("garbage out") {
                        embedding[1] = 0.9;
                    } else {
                        embedding[1] = -0.9;
                    }

                    Embedding::new(embedding)
                })
                .collect();
            future::ready(Ok(embeddings)).boxed()
        }

        fn batch_size(&self) -> usize {
            16
        }
    }

    #[gpui::test]
    async fn test_search(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        init_test(cx);

        let temp_dir = tempfile::tempdir().unwrap();

        let mut semantic_index = SemanticIndex::new(
            temp_dir.path().into(),
            Arc::new(TestEmbeddingProvider),
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

        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);
        let subscription = cx.update(|cx| {
            cx.subscribe(&project_index, move |_, event, _| {
                if let Some(tx) = tx.take() {
                    _ = tx.send(*event);
                }
            })
        });

        rx.await.expect("no event emitted");
        drop(subscription);

        let results = cx
            .update(|cx| {
                let project_index = project_index.read(cx);
                let query = "garbage in, garbage out";
                project_index.search(query, 4, cx)
            })
            .await;

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
                let entry_abs_path = worktree.abs_path().join(search_result.path.clone());
                let fs = project.read(cx).fs().clone();
                cx.spawn(|_| async move { fs.load(&entry_abs_path).await.unwrap() })
            })
            .await;

        let range = search_result.range.clone();
        let content = content[range.clone()].to_owned();

        assert!(content.contains("garbage in, garbage out"));
    }
}
