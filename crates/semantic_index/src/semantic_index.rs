mod chunking;
pub mod embedding;

use anyhow::{anyhow, Context as _, Result};
use chunking::{chunk_text, Chunk};
use collections::{Bound, HashMap};
use embedding::{Embedding, EmbeddingProvider};
use fs::Fs;
use futures::stream::StreamExt;
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{
    AppContext, AsyncAppContext, Context, EntityId, EventEmitter, Global, Model, ModelContext,
    Subscription, Task, WeakModel,
};
use heed::types::{SerdeBincode, Str};
use language::LanguageRegistry;
use project::{Entry, Project, Worktree};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{
    cmp::Ordering,
    ops::Range,
    path::Path,
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
    pub fn new(
        db_path: &Path,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut AppContext,
    ) -> Task<Result<Self>> {
        let db_path = db_path.to_path_buf();
        cx.spawn(|cx| async move {
            let db_connection = cx
                .background_executor()
                .spawn(async move {
                    unsafe {
                        heed::EnvOpenOptions::new()
                            .map_size(10 * 1024 * 1024)
                            .max_dbs(3000)
                            .open(db_path)
                    }
                })
                .await?;

            Ok(SemanticIndex {
                db_connection,
                embedding_provider,
                project_indices: HashMap::default(),
            })
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
    project: WeakModel<Project>,
    worktree_indices: HashMap<EntityId, WorktreeIndexHandle>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    last_status: Status,
    embedding_provider: Arc<dyn EmbeddingProvider>,
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
            project: project.downgrade(),
            worktree_indices: HashMap::default(),
            language_registry,
            fs,
            last_status: Status::Idle,
            embedding_provider,
        };

        for worktree in project.read(cx).worktrees().collect::<Vec<_>>() {
            this.add_worktree(worktree, cx);
        }

        this
    }

    fn add_worktree(&mut self, worktree: Model<Worktree>, cx: &mut ModelContext<Self>) {
        if !worktree.read(cx).is_local() {
            return;
        }

        let worktree_entity_id = worktree.entity_id();
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
                        worktree_entity_id,
                        WorktreeIndexHandle::Loaded {
                            _subscription: cx.observe(&index, |this, _, cx| this.update_status(cx)),
                            index,
                        },
                    );
                })?;
            } else {
                this.update(&mut cx, |this, _cx| {
                    this.worktree_indices.remove(&worktree_entity_id)
                })?;
            }

            this.update(&mut cx, |this, cx| this.update_status(cx))
        });
        self.worktree_indices.insert(
            worktree_entity_id,
            WorktreeIndexHandle::Loading {
                _task: load_worktree,
            },
        );
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
    pub entry: Entry,
    pub range: Range<usize>,
    pub text: String,
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
    status: Status,
    index_entries: Task<Result<()>>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
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
        Self {
            db_connection,
            db,
            worktree,
            language_registry,
            fs,
            status: Status::Idle,
            index_entries: cx.spawn(Self::index_entries),
            embedding_provider,
        }
    }

    async fn index_entries(this: WeakModel<Self>, mut cx: AsyncAppContext) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.status = Status::Scanning;
            cx.notify();
        })?;

        let worktree = this.read_with(&cx, |this, cx| {
            this.worktree.read(cx).as_local().unwrap().snapshot()
        })?;
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = Self::scan_entries(&this, worktree.clone(), &mut cx)?;
        let chunk = Self::chunk_files(&this, worktree_abs_path, scan.updated_entries, &mut cx)?;
        let embed = Self::embed_files(&this, chunk.files, &mut cx)?;
        let persist =
            Self::persist_embeddings(&this, scan.deleted_entry_ranges, embed.files, &mut cx)?;

        futures::try_join!(scan.task, chunk.task, embed.task, persist).log_err();
        this.update(&mut cx, |this, cx| {
            this.status = Status::Idle;
            cx.notify();
        })?;

        // todo!(React to files changing and re-run the pipeline above only for the changed entries)

        Ok(())
    }

    fn scan_entries(
        this: &WeakModel<Self>,
        worktree: LocalSnapshot,
        cx: &mut AsyncAppContext,
    ) -> Result<ScanEntries> {
        let (updated_entries_tx, updated_entries_rx) = channel::bounded(512);
        let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) = channel::bounded(128);
        let (db_connection, db) =
            this.read_with(cx, |this, _| (this.db_connection.clone(), this.db.clone()))?;
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

        Ok(ScanEntries {
            updated_entries: updated_entries_rx,
            deleted_entry_ranges: deleted_entry_ranges_rx,
            task,
        })
    }

    fn chunk_files(
        this: &WeakModel<Self>,
        worktree_abs_path: Arc<Path>,
        entries: channel::Receiver<Entry>,
        cx: &mut AsyncAppContext,
    ) -> Result<ChunkFiles> {
        let language_registry = this.read_with(cx, |this, _| this.language_registry.clone())?;
        let fs = this.read_with(cx, |this, _| this.fs.clone())?;
        let (chunked_files_tx, chunked_files_rx) = channel::bounded(2048);
        let task = cx.spawn(|cx| async move {
            cx.background_executor()
                .scoped(|cx| {
                    for _ in 0..cx.num_cpus() {
                        cx.spawn(async {
                            while let Ok(entry) = entries.recv().await {
                                let entry_abs_path = worktree_abs_path.join(&entry.path);
                                let Some(text) = fs.load(&entry_abs_path).await.log_err() else {
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
        Ok(ChunkFiles {
            files: chunked_files_rx,
            task,
        })
    }

    fn embed_files(
        this: &WeakModel<Self>,
        chunked_files: channel::Receiver<ChunkedFile>,
        cx: &mut AsyncAppContext,
    ) -> Result<EmbedFiles> {
        let embedding_provider = this.read_with(cx, |this, _| this.embedding_provider.clone())?;
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
                        file.chunks
                            .iter()
                            .map(|chunk| &file.text[chunk.range.clone()])
                    })
                    .collect::<Vec<_>>();

                let mut embeddings = Vec::new();
                for embedding_batch in chunks.chunks(embedding_provider.batch_size()) {
                    // todo!("add a retry facility")
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
        Ok(EmbedFiles {
            files: embedded_files_rx,
            task,
        })
    }

    fn persist_embeddings(
        this: &WeakModel<Self>,
        mut deleted_entry_ranges: channel::Receiver<(Bound<String>, Bound<String>)>,
        embedded_files: channel::Receiver<EmbeddedFile>,
        cx: &mut AsyncAppContext,
    ) -> Result<Task<Result<()>>> {
        let (db_connection, db) =
            this.read_with(cx, |this, _| (this.db_connection.clone(), this.db.clone()))?;
        Ok(cx.background_executor().spawn(async move {
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
        }))
    }

    fn search(
        &self,
        query: &str,
        limit: usize,
        cx: &AppContext,
    ) -> Task<Result<Vec<SearchResult>>> {
        let embedding_provider = Arc::clone(&self.embedding_provider);

        let query = query.to_owned();
        cx.background_executor().spawn(async move {
            // Embed the query as a vector
            let query_embedding = embedding_provider.embed(&[&query]).await?;
            let query_embedding = query_embedding
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("no embedding for query"))?;

            // Load all the embeddings from the database
            // todo!("already have this loaded into memory?")
            println!("{}", query_embedding);

            // Compute the scores across all embeddings

            // Sort the scores and return the top N
            Ok(vec![])
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
    worktree_root: Arc<Path>,
    entry: Entry,
    text: String,
    chunks: Vec<Chunk>,
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
