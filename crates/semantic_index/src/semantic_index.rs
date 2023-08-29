mod db;
mod embedding;
mod parsing;
pub mod semantic_index_settings;

#[cfg(test)]
mod semantic_index_tests;

use crate::semantic_index_settings::SemanticIndexSettings;
use anyhow::{anyhow, Result};
use db::VectorDatabase;
use embedding::{EmbeddingProvider, OpenAIEmbeddings};
use futures::{channel::oneshot, Future};
use gpui::{AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task, WeakModelHandle};
use language::{Anchor, Buffer, Language, LanguageRegistry};
use parking_lot::Mutex;
use parsing::{CodeContextRetriever, Document, PARSEABLE_ENTIRE_FILE_TYPES};
use postage::watch;
use project::{
    search::PathMatcher, Fs, PathChange, Project, ProjectEntryId, ProjectPath, Worktree, WorktreeId,
};
use smol::channel;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::{Arc, Weak},
    time::{Duration, Instant, SystemTime},
};
use util::{
    channel::{ReleaseChannel, RELEASE_CHANNEL, RELEASE_CHANNEL_NAME},
    http::HttpClient,
    paths::EMBEDDINGS_DIR,
    ResultExt,
};
use workspace::WorkspaceCreated;

const SEMANTIC_INDEX_VERSION: usize = 7;
const EMBEDDINGS_BATCH_SIZE: usize = 80;
const BACKGROUND_INDEXING_DELAY: Duration = Duration::from_secs(600);

pub fn init(
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AppContext,
) {
    settings::register::<SemanticIndexSettings>(cx);

    let db_file_path = EMBEDDINGS_DIR
        .join(Path::new(RELEASE_CHANNEL_NAME.as_str()))
        .join("embeddings_db");

    // This needs to be removed at some point before stable.
    if *RELEASE_CHANNEL == ReleaseChannel::Stable {
        return;
    }

    cx.subscribe_global::<WorkspaceCreated, _>({
        move |event, cx| {
            let Some(semantic_index) = SemanticIndex::global(cx) else {
                return;
            };
            let workspace = &event.0;
            if let Some(workspace) = workspace.upgrade(cx) {
                let project = workspace.read(cx).project().clone();
                if project.read(cx).is_local() {
                    semantic_index.update(cx, |index, cx| {
                        index.initialize_project(project, cx).detach_and_log_err(cx)
                    });
                }
            }
        }
    })
    .detach();

    cx.spawn(move |mut cx| async move {
        let semantic_index = SemanticIndex::new(
            fs,
            db_file_path,
            // Arc::new(embedding::DummyEmbeddings {}),
            Arc::new(OpenAIEmbeddings {
                client: http_client,
                executor: cx.background(),
            }),
            language_registry,
            cx.clone(),
        )
        .await?;

        cx.update(|cx| {
            cx.set_global(semantic_index.clone());
        });

        anyhow::Ok(())
    })
    .detach();
}

pub struct SemanticIndex {
    fs: Arc<dyn Fs>,
    database_url: Arc<PathBuf>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    language_registry: Arc<LanguageRegistry>,
    db_update_tx: channel::Sender<DbOperation>,
    parsing_files_tx: channel::Sender<PendingFile>,
    _db_update_task: Task<()>,
    _embed_batch_tasks: Vec<Task<()>>,
    _batch_files_task: Task<()>,
    _parsing_files_tasks: Vec<Task<()>>,
    projects: HashMap<WeakModelHandle<Project>, ProjectState>,
}

struct ProjectState {
    worktree_db_ids: Vec<(WorktreeId, i64)>,
    _subscription: gpui::Subscription,
    outstanding_job_count_rx: watch::Receiver<usize>,
    outstanding_job_count_tx: Arc<Mutex<watch::Sender<usize>>>,
    changed_paths: BTreeMap<ProjectPath, ChangedPathInfo>,
}

struct ChangedPathInfo {
    changed_at: Instant,
    mtime: SystemTime,
    is_deleted: bool,
}

#[derive(Clone)]
struct JobHandle {
    /// The outer Arc is here to count the clones of a JobHandle instance;
    /// when the last handle to a given job is dropped, we decrement a counter (just once).
    tx: Arc<Weak<Mutex<watch::Sender<usize>>>>,
}

impl JobHandle {
    fn new(tx: &Arc<Mutex<watch::Sender<usize>>>) -> Self {
        *tx.lock().borrow_mut() += 1;
        Self {
            tx: Arc::new(Arc::downgrade(&tx)),
        }
    }
}

impl ProjectState {
    fn new(
        subscription: gpui::Subscription,
        worktree_db_ids: Vec<(WorktreeId, i64)>,
        changed_paths: BTreeMap<ProjectPath, ChangedPathInfo>,
    ) -> Self {
        let (outstanding_job_count_tx, outstanding_job_count_rx) = watch::channel_with(0);
        let outstanding_job_count_tx = Arc::new(Mutex::new(outstanding_job_count_tx));
        Self {
            worktree_db_ids,
            outstanding_job_count_rx,
            outstanding_job_count_tx,
            changed_paths,
            _subscription: subscription,
        }
    }

    pub fn get_outstanding_count(&self) -> usize {
        self.outstanding_job_count_rx.borrow().clone()
    }

    fn db_id_for_worktree_id(&self, id: WorktreeId) -> Option<i64> {
        self.worktree_db_ids
            .iter()
            .find_map(|(worktree_id, db_id)| {
                if *worktree_id == id {
                    Some(*db_id)
                } else {
                    None
                }
            })
    }

    fn worktree_id_for_db_id(&self, id: i64) -> Option<WorktreeId> {
        self.worktree_db_ids
            .iter()
            .find_map(|(worktree_id, db_id)| {
                if *db_id == id {
                    Some(*worktree_id)
                } else {
                    None
                }
            })
    }
}

#[derive(Clone)]
pub struct PendingFile {
    worktree_db_id: i64,
    relative_path: PathBuf,
    absolute_path: PathBuf,
    language: Option<Arc<Language>>,
    modified_time: SystemTime,
    job_handle: JobHandle,
}

pub struct SearchResult {
    pub buffer: ModelHandle<Buffer>,
    pub range: Range<Anchor>,
}

enum DbOperation {
    InsertFile {
        worktree_id: i64,
        documents: Vec<Document>,
        path: PathBuf,
        mtime: SystemTime,
        job_handle: JobHandle,
    },
    Delete {
        worktree_id: i64,
        path: PathBuf,
    },
    FindOrCreateWorktree {
        path: PathBuf,
        sender: oneshot::Sender<Result<i64>>,
    },
    FileMTimes {
        worktree_id: i64,
        sender: oneshot::Sender<Result<HashMap<PathBuf, SystemTime>>>,
    },
    WorktreePreviouslyIndexed {
        path: Arc<Path>,
        sender: oneshot::Sender<Result<bool>>,
    },
}

enum EmbeddingJob {
    Enqueue {
        worktree_id: i64,
        path: PathBuf,
        mtime: SystemTime,
        documents: Vec<Document>,
        job_handle: JobHandle,
    },
    Flush,
}

impl SemanticIndex {
    pub fn global(cx: &AppContext) -> Option<ModelHandle<SemanticIndex>> {
        if cx.has_global::<ModelHandle<Self>>() {
            Some(cx.global::<ModelHandle<SemanticIndex>>().clone())
        } else {
            None
        }
    }

    pub fn enabled(cx: &AppContext) -> bool {
        settings::get::<SemanticIndexSettings>(cx).enabled
            && *RELEASE_CHANNEL != ReleaseChannel::Stable
    }

    async fn new(
        fs: Arc<dyn Fs>,
        database_url: PathBuf,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        language_registry: Arc<LanguageRegistry>,
        mut cx: AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let t0 = Instant::now();
        let database_url = Arc::new(database_url);

        let db = cx
            .background()
            .spawn(VectorDatabase::new(fs.clone(), database_url.clone()))
            .await?;

        log::trace!(
            "db initialization took {:?} milliseconds",
            t0.elapsed().as_millis()
        );

        Ok(cx.add_model(|cx| {
            let t0 = Instant::now();
            // Perform database operations
            let (db_update_tx, db_update_rx) = channel::unbounded();
            let _db_update_task = cx.background().spawn({
                async move {
                    while let Ok(job) = db_update_rx.recv().await {
                        Self::run_db_operation(&db, job)
                    }
                }
            });

            // Group documents into batches and send them to the embedding provider.
            let (embed_batch_tx, embed_batch_rx) =
                channel::unbounded::<Vec<(i64, Vec<Document>, PathBuf, SystemTime, JobHandle)>>();
            let mut _embed_batch_tasks = Vec::new();
            for _ in 0..cx.background().num_cpus() {
                let embed_batch_rx = embed_batch_rx.clone();
                _embed_batch_tasks.push(cx.background().spawn({
                    let db_update_tx = db_update_tx.clone();
                    let embedding_provider = embedding_provider.clone();
                    async move {
                        while let Ok(embeddings_queue) = embed_batch_rx.recv().await {
                            Self::compute_embeddings_for_batch(
                                embeddings_queue,
                                &embedding_provider,
                                &db_update_tx,
                            )
                            .await;
                        }
                    }
                }));
            }

            // Group documents into batches and send them to the embedding provider.
            let (batch_files_tx, batch_files_rx) = channel::unbounded::<EmbeddingJob>();
            let _batch_files_task = cx.background().spawn(async move {
                let mut queue_len = 0;
                let mut embeddings_queue = vec![];
                while let Ok(job) = batch_files_rx.recv().await {
                    Self::enqueue_documents_to_embed(
                        job,
                        &mut queue_len,
                        &mut embeddings_queue,
                        &embed_batch_tx,
                    );
                }
            });

            // Parse files into embeddable documents.
            let (parsing_files_tx, parsing_files_rx) = channel::unbounded::<PendingFile>();
            let mut _parsing_files_tasks = Vec::new();
            for _ in 0..cx.background().num_cpus() {
                let fs = fs.clone();
                let parsing_files_rx = parsing_files_rx.clone();
                let batch_files_tx = batch_files_tx.clone();
                let db_update_tx = db_update_tx.clone();
                _parsing_files_tasks.push(cx.background().spawn(async move {
                    let mut retriever = CodeContextRetriever::new();
                    while let Ok(pending_file) = parsing_files_rx.recv().await {
                        Self::parse_file(
                            &fs,
                            pending_file,
                            &mut retriever,
                            &batch_files_tx,
                            &parsing_files_rx,
                            &db_update_tx,
                        )
                        .await;
                    }
                }));
            }

            log::trace!(
                "semantic index task initialization took {:?} milliseconds",
                t0.elapsed().as_millis()
            );
            Self {
                fs,
                database_url,
                embedding_provider,
                language_registry,
                db_update_tx,
                parsing_files_tx,
                _db_update_task,
                _embed_batch_tasks,
                _batch_files_task,
                _parsing_files_tasks,
                projects: HashMap::new(),
            }
        }))
    }

    fn run_db_operation(db: &VectorDatabase, job: DbOperation) {
        match job {
            DbOperation::InsertFile {
                worktree_id,
                documents,
                path,
                mtime,
                job_handle,
            } => {
                db.insert_file(worktree_id, path, mtime, documents)
                    .log_err();
                drop(job_handle)
            }
            DbOperation::Delete { worktree_id, path } => {
                db.delete_file(worktree_id, path).log_err();
            }
            DbOperation::FindOrCreateWorktree { path, sender } => {
                let id = db.find_or_create_worktree(&path);
                sender.send(id).ok();
            }
            DbOperation::FileMTimes {
                worktree_id: worktree_db_id,
                sender,
            } => {
                let file_mtimes = db.get_file_mtimes(worktree_db_id);
                sender.send(file_mtimes).ok();
            }
            DbOperation::WorktreePreviouslyIndexed { path, sender } => {
                let worktree_indexed = db.worktree_previously_indexed(path.as_ref());
                sender.send(worktree_indexed).ok();
            }
        }
    }

    async fn compute_embeddings_for_batch(
        mut embeddings_queue: Vec<(i64, Vec<Document>, PathBuf, SystemTime, JobHandle)>,
        embedding_provider: &Arc<dyn EmbeddingProvider>,
        db_update_tx: &channel::Sender<DbOperation>,
    ) {
        let mut batch_documents = vec![];
        for (_, documents, _, _, _) in embeddings_queue.iter() {
            batch_documents.extend(documents.iter().map(|document| document.content.as_str()));
        }

        if let Ok(embeddings) = embedding_provider.embed_batch(batch_documents).await {
            log::trace!(
                "created {} embeddings for {} files",
                embeddings.len(),
                embeddings_queue.len(),
            );

            let mut i = 0;
            let mut j = 0;

            for embedding in embeddings.iter() {
                while embeddings_queue[i].1.len() == j {
                    i += 1;
                    j = 0;
                }

                embeddings_queue[i].1[j].embedding = embedding.to_owned();
                j += 1;
            }

            for (worktree_id, documents, path, mtime, job_handle) in embeddings_queue.into_iter() {
                db_update_tx
                    .send(DbOperation::InsertFile {
                        worktree_id,
                        documents,
                        path,
                        mtime,
                        job_handle,
                    })
                    .await
                    .unwrap();
            }
        } else {
            // Insert the file in spite of failure so that future attempts to index it do not take place (unless the file is changed).
            for (worktree_id, _, path, mtime, job_handle) in embeddings_queue.into_iter() {
                db_update_tx
                    .send(DbOperation::InsertFile {
                        worktree_id,
                        documents: vec![],
                        path,
                        mtime,
                        job_handle,
                    })
                    .await
                    .unwrap();
            }
        }
    }

    fn enqueue_documents_to_embed(
        job: EmbeddingJob,
        queue_len: &mut usize,
        embeddings_queue: &mut Vec<(i64, Vec<Document>, PathBuf, SystemTime, JobHandle)>,
        embed_batch_tx: &channel::Sender<Vec<(i64, Vec<Document>, PathBuf, SystemTime, JobHandle)>>,
    ) {
        // Handle edge case where individual file has more documents than max batch size
        let should_flush = match job {
            EmbeddingJob::Enqueue {
                documents,
                worktree_id,
                path,
                mtime,
                job_handle,
            } => {
                // If documents is greater than embeddings batch size, recursively batch existing rows.
                if &documents.len() > &EMBEDDINGS_BATCH_SIZE {
                    let first_job = EmbeddingJob::Enqueue {
                        documents: documents[..EMBEDDINGS_BATCH_SIZE].to_vec(),
                        worktree_id,
                        path: path.clone(),
                        mtime,
                        job_handle: job_handle.clone(),
                    };

                    Self::enqueue_documents_to_embed(
                        first_job,
                        queue_len,
                        embeddings_queue,
                        embed_batch_tx,
                    );

                    let second_job = EmbeddingJob::Enqueue {
                        documents: documents[EMBEDDINGS_BATCH_SIZE..].to_vec(),
                        worktree_id,
                        path: path.clone(),
                        mtime,
                        job_handle: job_handle.clone(),
                    };

                    Self::enqueue_documents_to_embed(
                        second_job,
                        queue_len,
                        embeddings_queue,
                        embed_batch_tx,
                    );
                    return;
                } else {
                    *queue_len += &documents.len();
                    embeddings_queue.push((worktree_id, documents, path, mtime, job_handle));
                    *queue_len >= EMBEDDINGS_BATCH_SIZE
                }
            }
            EmbeddingJob::Flush => true,
        };

        if should_flush {
            embed_batch_tx
                .try_send(mem::take(embeddings_queue))
                .unwrap();
            *queue_len = 0;
        }
    }

    async fn parse_file(
        fs: &Arc<dyn Fs>,
        pending_file: PendingFile,
        retriever: &mut CodeContextRetriever,
        batch_files_tx: &channel::Sender<EmbeddingJob>,
        parsing_files_rx: &channel::Receiver<PendingFile>,
        db_update_tx: &channel::Sender<DbOperation>,
    ) {
        let Some(language) = pending_file.language else {
            return;
        };

        if let Some(content) = fs.load(&pending_file.absolute_path).await.log_err() {
            if let Some(documents) = retriever
                .parse_file_with_template(&pending_file.relative_path, &content, language)
                .log_err()
            {
                log::trace!(
                    "parsed path {:?}: {} documents",
                    pending_file.relative_path,
                    documents.len()
                );

                if documents.len() == 0 {
                    db_update_tx
                        .send(DbOperation::InsertFile {
                            worktree_id: pending_file.worktree_db_id,
                            documents,
                            path: pending_file.relative_path,
                            mtime: pending_file.modified_time,
                            job_handle: pending_file.job_handle,
                        })
                        .await
                        .unwrap();
                } else {
                    batch_files_tx
                        .try_send(EmbeddingJob::Enqueue {
                            worktree_id: pending_file.worktree_db_id,
                            path: pending_file.relative_path,
                            mtime: pending_file.modified_time,
                            job_handle: pending_file.job_handle,
                            documents,
                        })
                        .unwrap();
                }
            }
        }

        if parsing_files_rx.len() == 0 {
            batch_files_tx.try_send(EmbeddingJob::Flush).unwrap();
        }
    }

    fn find_or_create_worktree(&self, path: PathBuf) -> impl Future<Output = Result<i64>> {
        let (tx, rx) = oneshot::channel();
        self.db_update_tx
            .try_send(DbOperation::FindOrCreateWorktree { path, sender: tx })
            .unwrap();
        async move { rx.await? }
    }

    fn get_file_mtimes(
        &self,
        worktree_id: i64,
    ) -> impl Future<Output = Result<HashMap<PathBuf, SystemTime>>> {
        let (tx, rx) = oneshot::channel();
        self.db_update_tx
            .try_send(DbOperation::FileMTimes {
                worktree_id,
                sender: tx,
            })
            .unwrap();
        async move { rx.await? }
    }

    fn worktree_previously_indexed(&self, path: Arc<Path>) -> impl Future<Output = Result<bool>> {
        let (tx, rx) = oneshot::channel();
        self.db_update_tx
            .try_send(DbOperation::WorktreePreviouslyIndexed { path, sender: tx })
            .unwrap();
        async move { rx.await? }
    }

    pub fn project_previously_indexed(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<bool>> {
        let worktrees_indexed_previously = project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| self.worktree_previously_indexed(worktree.read(cx).abs_path()))
            .collect::<Vec<_>>();
        cx.spawn(|_, _cx| async move {
            let worktree_indexed_previously =
                futures::future::join_all(worktrees_indexed_previously).await;

            Ok(worktree_indexed_previously
                .iter()
                .filter(|worktree| worktree.is_ok())
                .all(|v| v.as_ref().log_err().is_some_and(|v| v.to_owned())))
        })
    }

    fn project_entries_changed(
        &mut self,
        project: ModelHandle<Project>,
        changes: Arc<[(Arc<Path>, ProjectEntryId, PathChange)]>,
        cx: &mut ModelContext<'_, SemanticIndex>,
        worktree_id: &WorktreeId,
    ) {
        let Some(worktree) = project.read(cx).worktree_for_id(worktree_id.clone(), cx) else {
            return;
        };
        let project = project.downgrade();
        let Some(project_state) = self.projects.get_mut(&project) else {
            return;
        };

        let worktree = worktree.read(cx);
        let change_time = Instant::now();
        for (path, entry_id, change) in changes.iter() {
            let Some(entry) = worktree.entry_for_id(*entry_id) else {
                continue;
            };
            if entry.is_ignored || entry.is_symlink || entry.is_external {
                continue;
            }
            let project_path = ProjectPath {
                worktree_id: *worktree_id,
                path: path.clone(),
            };
            project_state.changed_paths.insert(
                project_path,
                ChangedPathInfo {
                    changed_at: change_time,
                    mtime: entry.mtime,
                    is_deleted: *change == PathChange::Removed,
                },
            );
        }

        cx.spawn_weak(|this, mut cx| async move {
            cx.background().timer(BACKGROUND_INDEXING_DELAY).await;
            if let Some((this, project)) = this.upgrade(&cx).zip(project.upgrade(&cx)) {
                Self::reindex_changed_paths(this, project, Some(change_time), &mut cx).await;
            }
        })
        .detach();
    }

    pub fn initialize_project(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        log::trace!("Initializing Project for Semantic Index");
        let worktree_scans_complete = project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| {
                let scan_complete = worktree.read(cx).as_local().unwrap().scan_complete();
                async move {
                    scan_complete.await;
                }
            })
            .collect::<Vec<_>>();

        let worktree_db_ids = project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| {
                self.find_or_create_worktree(worktree.read(cx).abs_path().to_path_buf())
            })
            .collect::<Vec<_>>();

        let _subscription = cx.subscribe(&project, |this, project, event, cx| {
            if let project::Event::WorktreeUpdatedEntries(worktree_id, changes) = event {
                this.project_entries_changed(project.clone(), changes.clone(), cx, worktree_id);
            };
        });

        let language_registry = self.language_registry.clone();

        cx.spawn(|this, mut cx| async move {
            futures::future::join_all(worktree_scans_complete).await;

            let worktree_db_ids = futures::future::join_all(worktree_db_ids).await;
            let worktrees = project.read_with(&cx, |project, cx| {
                project
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).snapshot())
                    .collect::<Vec<_>>()
            });

            let mut worktree_file_mtimes = HashMap::new();
            let mut db_ids_by_worktree_id = HashMap::new();

            for (worktree, db_id) in worktrees.iter().zip(worktree_db_ids) {
                let db_id = db_id?;
                db_ids_by_worktree_id.insert(worktree.id(), db_id);
                worktree_file_mtimes.insert(
                    worktree.id(),
                    this.read_with(&cx, |this, _| this.get_file_mtimes(db_id))
                        .await?,
                );
            }

            let worktree_db_ids = db_ids_by_worktree_id
                .iter()
                .map(|(a, b)| (*a, *b))
                .collect();

            let changed_paths = cx
                .background()
                .spawn(async move {
                    let mut changed_paths = BTreeMap::new();
                    let now = Instant::now();
                    for worktree in worktrees.into_iter() {
                        let mut file_mtimes = worktree_file_mtimes.remove(&worktree.id()).unwrap();
                        for file in worktree.files(false, 0) {
                            let absolute_path = worktree.absolutize(&file.path);

                            if file.is_external || file.is_ignored || file.is_symlink {
                                continue;
                            }

                            if let Ok(language) = language_registry
                                .language_for_file(&absolute_path, None)
                                .await
                            {
                                // Test if file is valid parseable file
                                if !PARSEABLE_ENTIRE_FILE_TYPES.contains(&language.name().as_ref())
                                    && &language.name().as_ref() != &"Markdown"
                                    && language
                                        .grammar()
                                        .and_then(|grammar| grammar.embedding_config.as_ref())
                                        .is_none()
                                {
                                    continue;
                                }

                                let stored_mtime = file_mtimes.remove(&file.path.to_path_buf());
                                let already_stored = stored_mtime
                                    .map_or(false, |existing_mtime| existing_mtime == file.mtime);

                                if !already_stored {
                                    changed_paths.insert(
                                        ProjectPath {
                                            worktree_id: worktree.id(),
                                            path: file.path.clone(),
                                        },
                                        ChangedPathInfo {
                                            changed_at: now,
                                            mtime: file.mtime,
                                            is_deleted: false,
                                        },
                                    );
                                }
                            }
                        }

                        // Clean up entries from database that are no longer in the worktree.
                        for (path, mtime) in file_mtimes {
                            changed_paths.insert(
                                ProjectPath {
                                    worktree_id: worktree.id(),
                                    path: path.into(),
                                },
                                ChangedPathInfo {
                                    changed_at: now,
                                    mtime,
                                    is_deleted: true,
                                },
                            );
                        }
                    }

                    anyhow::Ok(changed_paths)
                })
                .await?;

            this.update(&mut cx, |this, _| {
                this.projects.insert(
                    project.downgrade(),
                    ProjectState::new(_subscription, worktree_db_ids, changed_paths),
                );
            });
            Result::<(), _>::Ok(())
        })
    }

    pub fn index_project(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(usize, watch::Receiver<usize>)>> {
        cx.spawn(|this, mut cx| async move {
            Self::reindex_changed_paths(this.clone(), project.clone(), None, &mut cx).await;

            this.update(&mut cx, |this, _cx| {
                let Some(state) = this.projects.get(&project.downgrade()) else {
                    return Err(anyhow!("Project not yet initialized"));
                };
                let job_count_rx = state.outstanding_job_count_rx.clone();
                let count = state.get_outstanding_count();
                Ok((count, job_count_rx))
            })
        })
    }

    pub fn outstanding_job_count_rx(
        &self,
        project: &ModelHandle<Project>,
    ) -> Option<watch::Receiver<usize>> {
        Some(
            self.projects
                .get(&project.downgrade())?
                .outstanding_job_count_rx
                .clone(),
        )
    }

    pub fn search_project(
        &mut self,
        project: ModelHandle<Project>,
        phrase: String,
        limit: usize,
        includes: Vec<PathMatcher>,
        excludes: Vec<PathMatcher>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<SearchResult>>> {
        let project_state = if let Some(state) = self.projects.get(&project.downgrade()) {
            state
        } else {
            return Task::ready(Err(anyhow!("project not added")));
        };

        let worktree_db_ids = project
            .read(cx)
            .worktrees(cx)
            .filter_map(|worktree| {
                let worktree_id = worktree.read(cx).id();
                project_state.db_id_for_worktree_id(worktree_id)
            })
            .collect::<Vec<_>>();

        let embedding_provider = self.embedding_provider.clone();
        let database_url = self.database_url.clone();
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            let t0 = Instant::now();
            let database = VectorDatabase::new(fs.clone(), database_url.clone()).await?;

            let phrase_embedding = embedding_provider
                .embed_batch(vec![&phrase])
                .await?
                .into_iter()
                .next()
                .unwrap();

            log::trace!(
                "Embedding search phrase took: {:?} milliseconds",
                t0.elapsed().as_millis()
            );

            let file_ids =
                database.retrieve_included_file_ids(&worktree_db_ids, &includes, &excludes)?;

            let batch_n = cx.background().num_cpus();
            let ids_len = file_ids.clone().len();
            let batch_size = if ids_len <= batch_n {
                ids_len
            } else {
                ids_len / batch_n
            };

            let mut result_tasks = Vec::new();
            for batch in file_ids.chunks(batch_size) {
                let batch = batch.into_iter().map(|v| *v).collect::<Vec<i64>>();
                let limit = limit.clone();
                let fs = fs.clone();
                let database_url = database_url.clone();
                let phrase_embedding = phrase_embedding.clone();
                let task = cx.background().spawn(async move {
                    let database = VectorDatabase::new(fs, database_url).await.log_err();
                    if database.is_none() {
                        return Err(anyhow!("failed to acquire database connection"));
                    } else {
                        database
                            .unwrap()
                            .top_k_search(&phrase_embedding, limit, batch.as_slice())
                    }
                });
                result_tasks.push(task);
            }

            let batch_results = futures::future::join_all(result_tasks).await;

            let mut results = Vec::new();
            for batch_result in batch_results {
                if batch_result.is_ok() {
                    for (id, similarity) in batch_result.unwrap() {
                        let ix = match results.binary_search_by(|(_, s)| {
                            similarity.partial_cmp(&s).unwrap_or(Ordering::Equal)
                        }) {
                            Ok(ix) => ix,
                            Err(ix) => ix,
                        };
                        results.insert(ix, (id, similarity));
                        results.truncate(limit);
                    }
                }
            }

            let ids = results.into_iter().map(|(id, _)| id).collect::<Vec<i64>>();
            let documents = database.get_documents_by_ids(ids.as_slice())?;

            let mut tasks = Vec::new();
            let mut ranges = Vec::new();
            let weak_project = project.downgrade();
            project.update(&mut cx, |project, cx| {
                for (worktree_db_id, file_path, byte_range) in documents {
                    let project_state =
                        if let Some(state) = this.read(cx).projects.get(&weak_project) {
                            state
                        } else {
                            return Err(anyhow!("project not added"));
                        };
                    if let Some(worktree_id) = project_state.worktree_id_for_db_id(worktree_db_id) {
                        tasks.push(project.open_buffer((worktree_id, file_path), cx));
                        ranges.push(byte_range);
                    }
                }

                Ok(())
            })?;

            let buffers = futures::future::join_all(tasks).await;

            log::trace!(
                "Semantic Searching took: {:?} milliseconds in total",
                t0.elapsed().as_millis()
            );

            Ok(buffers
                .into_iter()
                .zip(ranges)
                .filter_map(|(buffer, range)| {
                    let buffer = buffer.log_err()?;
                    let range = buffer.read_with(&cx, |buffer, _| {
                        buffer.anchor_before(range.start)..buffer.anchor_after(range.end)
                    });
                    Some(SearchResult { buffer, range })
                })
                .collect::<Vec<_>>())
        })
    }

    async fn reindex_changed_paths(
        this: ModelHandle<SemanticIndex>,
        project: ModelHandle<Project>,
        last_changed_before: Option<Instant>,
        cx: &mut AsyncAppContext,
    ) {
        let mut pending_files = Vec::new();
        let (language_registry, parsing_files_tx) = this.update(cx, |this, cx| {
            if let Some(project_state) = this.projects.get_mut(&project.downgrade()) {
                let outstanding_job_count_tx = &project_state.outstanding_job_count_tx;
                let db_ids = &project_state.worktree_db_ids;
                let mut worktree: Option<ModelHandle<Worktree>> = None;

                project_state.changed_paths.retain(|path, info| {
                    if let Some(last_changed_before) = last_changed_before {
                        if info.changed_at > last_changed_before {
                            return true;
                        }
                    }

                    if worktree
                        .as_ref()
                        .map_or(true, |tree| tree.read(cx).id() != path.worktree_id)
                    {
                        worktree = project.read(cx).worktree_for_id(path.worktree_id, cx);
                    }
                    let Some(worktree) = &worktree else {
                        return false;
                    };

                    let Some(worktree_db_id) = db_ids
                        .iter()
                        .find_map(|entry| (entry.0 == path.worktree_id).then_some(entry.1))
                    else {
                        return false;
                    };

                    if info.is_deleted {
                        this.db_update_tx
                            .try_send(DbOperation::Delete {
                                worktree_id: worktree_db_id,
                                path: path.path.to_path_buf(),
                            })
                            .ok();
                    } else {
                        let absolute_path = worktree.read(cx).absolutize(&path.path);
                        let job_handle = JobHandle::new(&outstanding_job_count_tx);
                        pending_files.push(PendingFile {
                            absolute_path,
                            relative_path: path.path.to_path_buf(),
                            language: None,
                            job_handle,
                            modified_time: info.mtime,
                            worktree_db_id,
                        });
                    }

                    false
                });
            }

            (
                this.language_registry.clone(),
                this.parsing_files_tx.clone(),
            )
        });

        for mut pending_file in pending_files {
            if let Ok(language) = language_registry
                .language_for_file(&pending_file.relative_path, None)
                .await
            {
                if !PARSEABLE_ENTIRE_FILE_TYPES.contains(&language.name().as_ref())
                    && &language.name().as_ref() != &"Markdown"
                    && language
                        .grammar()
                        .and_then(|grammar| grammar.embedding_config.as_ref())
                        .is_none()
                {
                    continue;
                }
                pending_file.language = Some(language);
            }
            parsing_files_tx.try_send(pending_file).ok();
        }
    }
}

impl Entity for SemanticIndex {
    type Event = ();
}

impl Drop for JobHandle {
    fn drop(&mut self) {
        if let Some(inner) = Arc::get_mut(&mut self.tx) {
            // This is the last instance of the JobHandle (regardless of it's origin - whether it was cloned or not)
            if let Some(tx) = inner.upgrade() {
                let mut tx = tx.lock();
                *tx.borrow_mut() -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_job_handle() {
        let (job_count_tx, job_count_rx) = watch::channel_with(0);
        let tx = Arc::new(Mutex::new(job_count_tx));
        let job_handle = JobHandle::new(&tx);

        assert_eq!(1, *job_count_rx.borrow());
        let new_job_handle = job_handle.clone();
        assert_eq!(1, *job_count_rx.borrow());
        drop(job_handle);
        assert_eq!(1, *job_count_rx.borrow());
        drop(new_job_handle);
        assert_eq!(0, *job_count_rx.borrow());
    }
}
