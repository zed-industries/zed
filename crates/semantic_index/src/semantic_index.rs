mod db;
mod embedding;
mod embedding_queue;
mod parsing;
pub mod semantic_index_settings;

#[cfg(test)]
mod semantic_index_tests;

use crate::semantic_index_settings::SemanticIndexSettings;
use anyhow::{anyhow, Result};
use collections::{BTreeMap, HashMap, HashSet};
use db::VectorDatabase;
use embedding::{Embedding, EmbeddingProvider, OpenAIEmbeddings};
use embedding_queue::{EmbeddingQueue, FileToEmbed};
use futures::{FutureExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task, WeakModelHandle};
use language::{Anchor, Buffer, Language, LanguageRegistry};
use parking_lot::Mutex;
use parsing::{CodeContextRetriever, DocumentDigest, PARSEABLE_ENTIRE_FILE_TYPES};
use postage::watch;
use project::{search::PathMatcher, Fs, PathChange, Project, ProjectEntryId, Worktree, WorktreeId};
use smol::channel;
use std::{
    cmp::Ordering,
    ops::Range,
    path::{Path, PathBuf},
    sync::{Arc, Weak},
    time::{Duration, Instant, SystemTime},
};
use util::{
    channel::{ReleaseChannel, RELEASE_CHANNEL, RELEASE_CHANNEL_NAME},
    http::HttpClient,
    paths::EMBEDDINGS_DIR,
    ResultExt, TryFutureExt,
};
use workspace::WorkspaceCreated;

const SEMANTIC_INDEX_VERSION: usize = 9;
const BACKGROUND_INDEXING_DELAY: Duration = Duration::from_secs(5 * 60);
const EMBEDDING_QUEUE_FLUSH_TIMEOUT: Duration = Duration::from_millis(250);

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
                    semantic_index.update(cx, |index, cx| index.register_project(project, cx));
                }
            }
        }
    })
    .detach();

    cx.spawn(move |mut cx| async move {
        let semantic_index = SemanticIndex::new(
            fs,
            db_file_path,
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
    db: VectorDatabase,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    language_registry: Arc<LanguageRegistry>,
    parsing_files_tx: channel::Sender<(Arc<HashMap<DocumentDigest, Embedding>>, PendingFile)>,
    _embedding_task: Task<()>,
    _parsing_files_tasks: Vec<Task<()>>,
    projects: HashMap<WeakModelHandle<Project>, ProjectState>,
}

struct ProjectState {
    worktrees: HashMap<WorktreeId, WorktreeState>,
    outstanding_job_count_rx: watch::Receiver<usize>,
    outstanding_job_count_tx: Arc<Mutex<watch::Sender<usize>>>,
    _subscription: gpui::Subscription,
}

enum WorktreeState {
    Registering(RegisteringWorktreeState),
    Registered(RegisteredWorktreeState),
}

impl WorktreeState {
    fn paths_changed(
        &mut self,
        changes: Arc<[(Arc<Path>, ProjectEntryId, PathChange)]>,
        changed_at: Instant,
        worktree: &Worktree,
    ) {
        let changed_paths = match self {
            Self::Registering(state) => &mut state.changed_paths,
            Self::Registered(state) => &mut state.changed_paths,
        };

        for (path, entry_id, change) in changes.iter() {
            let Some(entry) = worktree.entry_for_id(*entry_id) else {
                continue;
            };
            if entry.is_ignored || entry.is_symlink || entry.is_external || entry.is_dir() {
                continue;
            }
            changed_paths.insert(
                path.clone(),
                ChangedPathInfo {
                    changed_at,
                    mtime: entry.mtime,
                    is_deleted: *change == PathChange::Removed,
                },
            );
        }
    }
}

struct RegisteringWorktreeState {
    changed_paths: BTreeMap<Arc<Path>, ChangedPathInfo>,
    _registration: Task<Option<()>>,
}

struct RegisteredWorktreeState {
    db_id: i64,
    changed_paths: BTreeMap<Arc<Path>, ChangedPathInfo>,
}

struct ChangedPathInfo {
    changed_at: Instant,
    mtime: SystemTime,
    is_deleted: bool,
}

#[derive(Clone)]
pub struct JobHandle {
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
    fn new(subscription: gpui::Subscription) -> Self {
        let (outstanding_job_count_tx, outstanding_job_count_rx) = watch::channel_with(0);
        let outstanding_job_count_tx = Arc::new(Mutex::new(outstanding_job_count_tx));
        Self {
            worktrees: Default::default(),
            outstanding_job_count_rx,
            outstanding_job_count_tx,
            _subscription: subscription,
        }
    }

    fn db_id_for_worktree_id(&self, id: WorktreeId) -> Option<i64> {
        match self.worktrees.get(&id)? {
            WorktreeState::Registering(_) => None,
            WorktreeState::Registered(state) => Some(state.db_id),
        }
    }

    fn worktree_id_for_db_id(&self, id: i64) -> Option<WorktreeId> {
        self.worktrees
            .iter()
            .find_map(|(worktree_id, worktree_state)| match worktree_state {
                WorktreeState::Registered(state) if state.db_id == id => Some(*worktree_id),
                _ => None,
            })
    }

    fn worktree(&mut self, id: WorktreeId) -> Option<&mut WorktreeState> {
        self.worktrees.get_mut(&id)
    }
}

#[derive(Clone)]
pub struct PendingFile {
    worktree_db_id: i64,
    relative_path: Arc<Path>,
    absolute_path: PathBuf,
    language: Option<Arc<Language>>,
    modified_time: SystemTime,
    job_handle: JobHandle,
}

pub struct SearchResult {
    pub buffer: ModelHandle<Buffer>,
    pub range: Range<Anchor>,
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
        database_path: PathBuf,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        language_registry: Arc<LanguageRegistry>,
        mut cx: AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let t0 = Instant::now();
        let database_path = Arc::from(database_path);
        let db = VectorDatabase::new(fs.clone(), database_path, cx.background()).await?;

        log::trace!(
            "db initialization took {:?} milliseconds",
            t0.elapsed().as_millis()
        );

        Ok(cx.add_model(|cx| {
            let t0 = Instant::now();
            let embedding_queue =
                EmbeddingQueue::new(embedding_provider.clone(), cx.background().clone());
            let _embedding_task = cx.background().spawn({
                let embedded_files = embedding_queue.finished_files();
                let db = db.clone();
                async move {
                    while let Ok(file) = embedded_files.recv().await {
                        db.insert_file(file.worktree_id, file.path, file.mtime, file.documents)
                            .await
                            .log_err();
                    }
                }
            });

            // Parse files into embeddable documents.
            let (parsing_files_tx, parsing_files_rx) =
                channel::unbounded::<(Arc<HashMap<DocumentDigest, Embedding>>, PendingFile)>();
            let embedding_queue = Arc::new(Mutex::new(embedding_queue));
            let mut _parsing_files_tasks = Vec::new();
            for _ in 0..cx.background().num_cpus() {
                let fs = fs.clone();
                let mut parsing_files_rx = parsing_files_rx.clone();
                let embedding_provider = embedding_provider.clone();
                let embedding_queue = embedding_queue.clone();
                let background = cx.background().clone();
                _parsing_files_tasks.push(cx.background().spawn(async move {
                    let mut retriever = CodeContextRetriever::new(embedding_provider.clone());
                    loop {
                        let mut timer = background.timer(EMBEDDING_QUEUE_FLUSH_TIMEOUT).fuse();
                        let mut next_file_to_parse = parsing_files_rx.next().fuse();
                        futures::select_biased! {
                            next_file_to_parse = next_file_to_parse => {
                                if let Some((embeddings_for_digest, pending_file)) = next_file_to_parse {
                                    Self::parse_file(
                                        &fs,
                                        pending_file,
                                        &mut retriever,
                                        &embedding_queue,
                                        &embeddings_for_digest,
                                    )
                                    .await
                                } else {
                                    break;
                                }
                            },
                            _ = timer => {
                                embedding_queue.lock().flush();
                            }
                        }
                    }
                }));
            }

            log::trace!(
                "semantic index task initialization took {:?} milliseconds",
                t0.elapsed().as_millis()
            );
            Self {
                fs,
                db,
                embedding_provider,
                language_registry,
                parsing_files_tx,
                _embedding_task,
                _parsing_files_tasks,
                projects: Default::default(),
            }
        }))
    }

    async fn parse_file(
        fs: &Arc<dyn Fs>,
        pending_file: PendingFile,
        retriever: &mut CodeContextRetriever,
        embedding_queue: &Arc<Mutex<EmbeddingQueue>>,
        embeddings_for_digest: &HashMap<DocumentDigest, Embedding>,
    ) {
        let Some(language) = pending_file.language else {
            return;
        };

        if let Some(content) = fs.load(&pending_file.absolute_path).await.log_err() {
            if let Some(mut documents) = retriever
                .parse_file_with_template(&pending_file.relative_path, &content, language)
                .log_err()
            {
                log::trace!(
                    "parsed path {:?}: {} documents",
                    pending_file.relative_path,
                    documents.len()
                );

                for document in documents.iter_mut() {
                    if let Some(embedding) = embeddings_for_digest.get(&document.digest) {
                        document.embedding = Some(embedding.to_owned());
                    }
                }

                embedding_queue.lock().push(FileToEmbed {
                    worktree_id: pending_file.worktree_db_id,
                    path: pending_file.relative_path,
                    mtime: pending_file.modified_time,
                    job_handle: pending_file.job_handle,
                    documents,
                });
            }
        }
    }

    pub fn project_previously_indexed(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<bool>> {
        let worktrees_indexed_previously = project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| {
                self.db
                    .worktree_previously_indexed(&worktree.read(cx).abs_path())
            })
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
        worktree_id: WorktreeId,
        changes: Arc<[(Arc<Path>, ProjectEntryId, PathChange)]>,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(worktree) = project.read(cx).worktree_for_id(worktree_id.clone(), cx) else {
            return;
        };
        let project = project.downgrade();
        let Some(project_state) = self.projects.get_mut(&project) else {
            return;
        };

        let change_time = Instant::now();
        let worktree = worktree.read(cx);
        let worktree_state = if let Some(worktree_state) = project_state.worktree(worktree_id) {
            worktree_state
        } else {
            return;
        };
        worktree_state.paths_changed(changes, Instant::now(), worktree);
        if let WorktreeState::Registered(worktree_state) = worktree_state {
            let embeddings_for_digest = {
                let worktree_paths = worktree_state
                    .changed_paths
                    .iter()
                    .map(|(path, _)| path.clone())
                    .collect::<Vec<_>>();
                let mut worktree_id_file_paths = HashMap::default();
                worktree_id_file_paths.insert(worktree_state.db_id, worktree_paths);
                self.db.embeddings_for_files(worktree_id_file_paths)
            };

            cx.spawn_weak(|this, mut cx| async move {
                let embeddings_for_digest =
                    embeddings_for_digest.await.log_err().unwrap_or_default();

                cx.background().timer(BACKGROUND_INDEXING_DELAY).await;
                if let Some((this, project)) = this.upgrade(&cx).zip(project.upgrade(&cx)) {
                    Self::reindex_changed_paths(
                        this,
                        project,
                        Some(change_time),
                        &mut cx,
                        Arc::new(embeddings_for_digest),
                    )
                    .await;
                }
            })
            .detach();
        }
    }

    pub fn register_project(&mut self, project: ModelHandle<Project>, cx: &mut ModelContext<Self>) {
        log::trace!("Registering Project for Semantic Index");

        let subscription = cx.subscribe(&project, |this, project, event, cx| match event {
            project::Event::WorktreeAdded | project::Event::WorktreeRemoved(_) => {
                this.project_worktrees_changed(project.clone(), cx);
            }
            project::Event::WorktreeUpdatedEntries(worktree_id, changes) => {
                this.project_entries_changed(project, *worktree_id, changes.clone(), cx);
            }
            _ => {}
        });
        self.projects
            .insert(project.downgrade(), ProjectState::new(subscription));
        self.project_worktrees_changed(project, cx);
    }

    fn register_worktree(
        &mut self,
        project: ModelHandle<Project>,
        worktree: ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) {
        let project = project.downgrade();
        let project_state = if let Some(project_state) = self.projects.get_mut(&project) {
            project_state
        } else {
            return;
        };
        let worktree = if let Some(worktree) = worktree.read(cx).as_local() {
            worktree
        } else {
            return;
        };
        let worktree_abs_path = worktree.abs_path().clone();
        let scan_complete = worktree.scan_complete();
        let worktree_id = worktree.id();
        let db = self.db.clone();
        let language_registry = self.language_registry.clone();
        let registration = cx.spawn(|this, mut cx| {
            async move {
                scan_complete.await;
                let db_id = db.find_or_create_worktree(worktree_abs_path).await?;
                let mut file_mtimes = db.get_file_mtimes(db_id).await?;
                let worktree = if let Some(project) = project.upgrade(&cx) {
                    project
                        .read_with(&cx, |project, cx| project.worktree_for_id(worktree_id, cx))
                        .ok_or_else(|| anyhow!("worktree not found"))?
                } else {
                    return anyhow::Ok(());
                };
                let worktree = worktree.read_with(&cx, |worktree, _| worktree.snapshot());
                let mut changed_paths = cx
                    .background()
                    .spawn(async move {
                        let mut changed_paths = BTreeMap::new();
                        let now = Instant::now();
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
                                        file.path.clone(),
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
                                path.into(),
                                ChangedPathInfo {
                                    changed_at: now,
                                    mtime,
                                    is_deleted: true,
                                },
                            );
                        }

                        anyhow::Ok(changed_paths)
                    })
                    .await?;
                this.update(&mut cx, |this, _| {
                    let project_state = this
                        .projects
                        .get_mut(&project)
                        .ok_or_else(|| anyhow!("project not registered"))?;

                    if let Some(WorktreeState::Registering(state)) =
                        project_state.worktrees.remove(&worktree_id)
                    {
                        changed_paths.extend(state.changed_paths);
                    }
                    project_state.worktrees.insert(
                        worktree_id,
                        WorktreeState::Registered(RegisteredWorktreeState {
                            db_id,
                            changed_paths,
                        }),
                    );

                    anyhow::Ok(())
                })?;

                anyhow::Ok(())
            }
            .log_err()
        });
        project_state.worktrees.insert(
            worktree_id,
            WorktreeState::Registering(RegisteringWorktreeState {
                changed_paths: Default::default(),
                _registration: registration,
            }),
        );
    }

    fn project_worktrees_changed(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) {
        let project_state = if let Some(project_state) = self.projects.get_mut(&project.downgrade())
        {
            project_state
        } else {
            return;
        };

        let mut worktrees = project
            .read(cx)
            .worktrees(cx)
            .filter(|worktree| worktree.read(cx).is_local())
            .collect::<Vec<_>>();
        let worktree_ids = worktrees
            .iter()
            .map(|worktree| worktree.read(cx).id())
            .collect::<HashSet<_>>();

        // Remove worktrees that are no longer present
        project_state
            .worktrees
            .retain(|worktree_id, _| worktree_ids.contains(worktree_id));

        // Register new worktrees
        worktrees.retain(|worktree| {
            let worktree_id = worktree.read(cx).id();
            project_state.worktree(worktree_id).is_none()
        });
        for worktree in worktrees {
            self.register_worktree(project.clone(), worktree, cx);
        }
    }

    pub fn index_project(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(usize, watch::Receiver<usize>)>> {
        let project_state = if let Some(project_state) = self.projects.get_mut(&project.downgrade())
        {
            project_state
        } else {
            return Task::ready(Err(anyhow!("project was not registered")));
        };
        let outstanding_job_count_rx = project_state.outstanding_job_count_rx.clone();

        let mut worktree_id_file_paths = HashMap::default();
        for worktree in project_state.worktrees.values() {
            if let WorktreeState::Registered(worktree_state) = worktree {
                for (path, _) in &worktree_state.changed_paths {
                    worktree_id_file_paths
                        .entry(worktree_state.db_id)
                        .or_insert(Vec::new())
                        .push(path.clone());
                }
            }
        }

        cx.spawn(|this, mut cx| async move {
            let embeddings_for_digest = this.read_with(&cx, |this, _| {
                this.db.embeddings_for_files(worktree_id_file_paths)
            });
            let embeddings_for_digest = Arc::new(embeddings_for_digest.await?);
            Self::reindex_changed_paths(
                this.clone(),
                project.clone(),
                None,
                &mut cx,
                embeddings_for_digest,
            )
            .await;
            let count = *outstanding_job_count_rx.borrow();
            Ok((count, outstanding_job_count_rx))
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
        let db_path = self.db.path().clone();
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            let t0 = Instant::now();
            let database =
                VectorDatabase::new(fs.clone(), db_path.clone(), cx.background()).await?;

            let phrase_embedding = embedding_provider
                .embed_batch(vec![phrase])
                .await?
                .into_iter()
                .next()
                .unwrap();

            log::trace!(
                "Embedding search phrase took: {:?} milliseconds",
                t0.elapsed().as_millis()
            );

            let file_ids = database
                .retrieve_included_file_ids(&worktree_db_ids, &includes, &excludes)
                .await?;

            let batch_n = cx.background().num_cpus();
            let ids_len = file_ids.clone().len();
            let batch_size = if ids_len <= batch_n {
                ids_len
            } else {
                ids_len / batch_n
            };

            let mut batch_results = Vec::new();
            for batch in file_ids.chunks(batch_size) {
                let batch = batch.into_iter().map(|v| *v).collect::<Vec<i64>>();
                let limit = limit.clone();
                let fs = fs.clone();
                let db_path = db_path.clone();
                let phrase_embedding = phrase_embedding.clone();
                if let Some(db) = VectorDatabase::new(fs, db_path.clone(), cx.background())
                    .await
                    .log_err()
                {
                    batch_results.push(async move {
                        db.top_k_search(&phrase_embedding, limit, batch.as_slice())
                            .await
                    });
                }
            }
            let batch_results = futures::future::join_all(batch_results).await;

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
            let documents = database.get_documents_by_ids(ids.as_slice()).await?;

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
        embeddings_for_digest: Arc<HashMap<DocumentDigest, Embedding>>,
    ) {
        let mut pending_files = Vec::new();
        let mut files_to_delete = Vec::new();
        let (db, language_registry, parsing_files_tx) = this.update(cx, |this, cx| {
            if let Some(project_state) = this.projects.get_mut(&project.downgrade()) {
                let outstanding_job_count_tx = &project_state.outstanding_job_count_tx;
                project_state
                    .worktrees
                    .retain(|worktree_id, worktree_state| {
                        let worktree = if let Some(worktree) =
                            project.read(cx).worktree_for_id(*worktree_id, cx)
                        {
                            worktree
                        } else {
                            return false;
                        };
                        let worktree_state =
                            if let WorktreeState::Registered(worktree_state) = worktree_state {
                                worktree_state
                            } else {
                                return true;
                            };

                        worktree_state.changed_paths.retain(|path, info| {
                            if let Some(last_changed_before) = last_changed_before {
                                if info.changed_at > last_changed_before {
                                    return true;
                                }
                            }

                            if info.is_deleted {
                                files_to_delete.push((worktree_state.db_id, path.clone()));
                            } else {
                                let absolute_path = worktree.read(cx).absolutize(path);
                                let job_handle = JobHandle::new(&outstanding_job_count_tx);
                                pending_files.push(PendingFile {
                                    absolute_path,
                                    relative_path: path.clone(),
                                    language: None,
                                    job_handle,
                                    modified_time: info.mtime,
                                    worktree_db_id: worktree_state.db_id,
                                });
                            }

                            false
                        });
                        true
                    });
            }

            (
                this.db.clone(),
                this.language_registry.clone(),
                this.parsing_files_tx.clone(),
            )
        });

        for (worktree_db_id, path) in files_to_delete {
            db.delete_file(worktree_db_id, path).await.log_err();
        }

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
            parsing_files_tx
                .try_send((embeddings_for_digest.clone(), pending_file))
                .ok();
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
