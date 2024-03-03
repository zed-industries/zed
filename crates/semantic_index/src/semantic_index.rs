mod db;
mod embedding_queue;
mod parsing;
pub mod semantic_index_settings;

#[cfg(test)]
mod semantic_index_tests;

use crate::semantic_index_settings::SemanticIndexSettings;
use ai::embedding::{Embedding, EmbeddingProvider};
use ai::providers::open_ai::{OpenAiEmbeddingProvider, OPEN_AI_API_URL};
use anyhow::{anyhow, Context as _, Result};
use collections::{BTreeMap, HashMap, HashSet};
use db::VectorDatabase;
use embedding_queue::{EmbeddingQueue, FileToEmbed};
use futures::{future, FutureExt, StreamExt};
use gpui::{
    AppContext, AsyncAppContext, BorrowWindow, Context, Global, Model, ModelContext, Task,
    ViewContext, WeakModel,
};
use language::{Anchor, Bias, Buffer, Language, LanguageRegistry};
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use parking_lot::Mutex;
use parsing::{CodeContextRetriever, Span, SpanDigest, PARSEABLE_ENTIRE_FILE_TYPES};
use postage::watch;
use project::{Fs, PathChange, Project, ProjectEntryId, Worktree, WorktreeId};
use release_channel::ReleaseChannel;
use settings::Settings;
use smol::channel;
use std::{
    cmp::Reverse,
    env,
    future::Future,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::{Arc, Weak},
    time::{Duration, Instant, SystemTime},
};
use util::paths::PathMatcher;
use util::{http::HttpClient, paths::EMBEDDINGS_DIR, ResultExt};
use workspace::Workspace;

const SEMANTIC_INDEX_VERSION: usize = 11;
const BACKGROUND_INDEXING_DELAY: Duration = Duration::from_secs(5 * 60);
const EMBEDDING_QUEUE_FLUSH_TIMEOUT: Duration = Duration::from_millis(250);

lazy_static! {
    static ref OPENAI_API_KEY: Option<String> = env::var("OPENAI_API_KEY").ok();
}

pub fn init(
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AppContext,
) {
    SemanticIndexSettings::register(cx);

    let db_file_path = EMBEDDINGS_DIR
        .join(Path::new(ReleaseChannel::global(cx).dev_name()))
        .join("embeddings_db");

    cx.observe_new_views(
        |workspace: &mut Workspace, cx: &mut ViewContext<Workspace>| {
            let Some(semantic_index) = SemanticIndex::global(cx) else {
                return;
            };
            let project = workspace.project().clone();

            if project.read(cx).is_local() {
                cx.app_mut()
                    .spawn(|mut cx| async move {
                        let previously_indexed = semantic_index
                            .update(&mut cx, |index, cx| {
                                index.project_previously_indexed(&project, cx)
                            })?
                            .await?;
                        if previously_indexed {
                            semantic_index
                                .update(&mut cx, |index, cx| index.index_project(project, cx))?
                                .await?;
                        }
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
            }
        },
    )
    .detach();

    cx.spawn(move |cx| async move {
        let embedding_provider = OpenAiEmbeddingProvider::new(
            // TODO: We should read it from config, but I'm not sure whether to reuse `openai_api_url` in assistant settings or not
            OPEN_AI_API_URL.to_string(),
            http_client,
            cx.background_executor().clone(),
        )
        .await;
        let semantic_index = SemanticIndex::new(
            fs,
            db_file_path,
            Arc::new(embedding_provider),
            language_registry,
            cx.clone(),
        )
        .await?;

        cx.update(|cx| cx.set_global(GlobalSemanticIndex(semantic_index.clone())))?;

        anyhow::Ok(())
    })
    .detach();
}

#[derive(Copy, Clone, Debug)]
pub enum SemanticIndexStatus {
    NotAuthenticated,
    NotIndexed,
    Indexed,
    Indexing {
        remaining_files: usize,
        rate_limit_expiry: Option<Instant>,
    },
}

pub struct SemanticIndex {
    fs: Arc<dyn Fs>,
    db: VectorDatabase,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    language_registry: Arc<LanguageRegistry>,
    parsing_files_tx: channel::Sender<(Arc<HashMap<SpanDigest, Embedding>>, PendingFile)>,
    _embedding_task: Task<()>,
    _parsing_files_tasks: Vec<Task<()>>,
    projects: HashMap<WeakModel<Project>, ProjectState>,
}

struct GlobalSemanticIndex(Model<SemanticIndex>);

impl Global for GlobalSemanticIndex {}

struct ProjectState {
    worktrees: HashMap<WorktreeId, WorktreeState>,
    pending_file_count_rx: watch::Receiver<usize>,
    pending_file_count_tx: Arc<Mutex<watch::Sender<usize>>>,
    pending_index: usize,
    _subscription: gpui::Subscription,
    _observe_pending_file_count: Task<()>,
}

enum WorktreeState {
    Registering(RegisteringWorktreeState),
    Registered(RegisteredWorktreeState),
}

impl WorktreeState {
    fn is_registered(&self) -> bool {
        matches!(self, Self::Registered(_))
    }

    fn paths_changed(
        &mut self,
        changes: Arc<[(Arc<Path>, ProjectEntryId, PathChange)]>,
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
                    mtime: entry.mtime,
                    is_deleted: *change == PathChange::Removed,
                },
            );
        }
    }
}

struct RegisteringWorktreeState {
    changed_paths: BTreeMap<Arc<Path>, ChangedPathInfo>,
    done_rx: watch::Receiver<Option<()>>,
    _registration: Task<()>,
}

impl RegisteringWorktreeState {
    fn done(&self) -> impl Future<Output = ()> {
        let mut done_rx = self.done_rx.clone();
        async move {
            while let Some(result) = done_rx.next().await {
                if result.is_some() {
                    break;
                }
            }
        }
    }
}

struct RegisteredWorktreeState {
    db_id: i64,
    changed_paths: BTreeMap<Arc<Path>, ChangedPathInfo>,
}

struct ChangedPathInfo {
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
    fn new(subscription: gpui::Subscription, cx: &mut ModelContext<SemanticIndex>) -> Self {
        let (pending_file_count_tx, pending_file_count_rx) = watch::channel_with(0);
        let pending_file_count_tx = Arc::new(Mutex::new(pending_file_count_tx));
        Self {
            worktrees: Default::default(),
            pending_file_count_rx: pending_file_count_rx.clone(),
            pending_file_count_tx,
            pending_index: 0,
            _subscription: subscription,
            _observe_pending_file_count: cx.spawn({
                let mut pending_file_count_rx = pending_file_count_rx.clone();
                |this, mut cx| async move {
                    while let Some(_) = pending_file_count_rx.next().await {
                        if this.update(&mut cx, |_, cx| cx.notify()).is_err() {
                            break;
                        }
                    }
                }
            }),
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

#[derive(Clone)]
pub struct SearchResult {
    pub buffer: Model<Buffer>,
    pub range: Range<Anchor>,
    pub similarity: OrderedFloat<f32>,
}

impl SemanticIndex {
    pub fn global(cx: &mut AppContext) -> Option<Model<SemanticIndex>> {
        cx.try_global::<GlobalSemanticIndex>()
            .map(|semantic_index| semantic_index.0.clone())
    }

    pub fn authenticate(&mut self, cx: &mut AppContext) -> Task<bool> {
        if !self.embedding_provider.has_credentials() {
            let embedding_provider = self.embedding_provider.clone();
            cx.spawn(|cx| async move {
                if let Some(retrieve_credentials) = cx
                    .update(|cx| embedding_provider.retrieve_credentials(cx))
                    .log_err()
                {
                    retrieve_credentials.await;
                }

                embedding_provider.has_credentials()
            })
        } else {
            Task::ready(true)
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.embedding_provider.has_credentials()
    }

    pub fn enabled(cx: &AppContext) -> bool {
        SemanticIndexSettings::get_global(cx).enabled
    }

    pub fn status(&self, project: &Model<Project>) -> SemanticIndexStatus {
        if !self.is_authenticated() {
            return SemanticIndexStatus::NotAuthenticated;
        }

        if let Some(project_state) = self.projects.get(&project.downgrade()) {
            if project_state
                .worktrees
                .values()
                .all(|worktree| worktree.is_registered())
                && project_state.pending_index == 0
            {
                SemanticIndexStatus::Indexed
            } else {
                SemanticIndexStatus::Indexing {
                    remaining_files: *project_state.pending_file_count_rx.borrow(),
                    rate_limit_expiry: self.embedding_provider.rate_limit_expiration(),
                }
            }
        } else {
            SemanticIndexStatus::NotIndexed
        }
    }

    pub async fn new(
        fs: Arc<dyn Fs>,
        database_path: PathBuf,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        language_registry: Arc<LanguageRegistry>,
        mut cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        let t0 = Instant::now();
        let database_path = Arc::from(database_path);
        let db = VectorDatabase::new(fs.clone(), database_path, cx.background_executor().clone())
            .await?;

        log::trace!(
            "db initialization took {:?} milliseconds",
            t0.elapsed().as_millis()
        );

        cx.new_model(|cx| {
            let t0 = Instant::now();
            let embedding_queue =
                EmbeddingQueue::new(embedding_provider.clone(), cx.background_executor().clone());
            let _embedding_task = cx.background_executor().spawn({
                let embedded_files = embedding_queue.finished_files();
                let db = db.clone();
                async move {
                    while let Ok(file) = embedded_files.recv().await {
                        db.insert_file(file.worktree_id, file.path, file.mtime, file.spans)
                            .await
                            .log_err();
                    }
                }
            });

            // Parse files into embeddable spans.
            let (parsing_files_tx, parsing_files_rx) =
                channel::unbounded::<(Arc<HashMap<SpanDigest, Embedding>>, PendingFile)>();
            let embedding_queue = Arc::new(Mutex::new(embedding_queue));
            let mut _parsing_files_tasks = Vec::new();
            for _ in 0..cx.background_executor().num_cpus() {
                let fs = fs.clone();
                let mut parsing_files_rx = parsing_files_rx.clone();
                let embedding_provider = embedding_provider.clone();
                let embedding_queue = embedding_queue.clone();
                let background = cx.background_executor().clone();
                _parsing_files_tasks.push(cx.background_executor().spawn(async move {
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
        })
    }

    async fn parse_file(
        fs: &Arc<dyn Fs>,
        pending_file: PendingFile,
        retriever: &mut CodeContextRetriever,
        embedding_queue: &Arc<Mutex<EmbeddingQueue>>,
        embeddings_for_digest: &HashMap<SpanDigest, Embedding>,
    ) {
        let Some(language) = pending_file.language else {
            return;
        };

        if let Some(content) = fs.load(&pending_file.absolute_path).await.log_err() {
            if let Some(mut spans) = retriever
                .parse_file_with_template(Some(&pending_file.relative_path), &content, language)
                .log_err()
            {
                log::trace!(
                    "parsed path {:?}: {} spans",
                    pending_file.relative_path,
                    spans.len()
                );

                for span in &mut spans {
                    if let Some(embedding) = embeddings_for_digest.get(&span.digest) {
                        span.embedding = Some(embedding.to_owned());
                    }
                }

                embedding_queue.lock().push(FileToEmbed {
                    worktree_id: pending_file.worktree_db_id,
                    path: pending_file.relative_path,
                    mtime: pending_file.modified_time,
                    job_handle: pending_file.job_handle,
                    spans,
                });
            }
        }
    }

    pub fn project_previously_indexed(
        &mut self,
        project: &Model<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<bool>> {
        let worktrees_indexed_previously = project
            .read(cx)
            .worktrees()
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
        project: Model<Project>,
        worktree_id: WorktreeId,
        changes: Arc<[(Arc<Path>, ProjectEntryId, PathChange)]>,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(worktree) = project.read(cx).worktree_for_id(worktree_id, cx) else {
            return;
        };
        let project = project.downgrade();
        let Some(project_state) = self.projects.get_mut(&project) else {
            return;
        };

        let worktree = worktree.read(cx);
        let worktree_state =
            if let Some(worktree_state) = project_state.worktrees.get_mut(&worktree_id) {
                worktree_state
            } else {
                return;
            };
        worktree_state.paths_changed(changes, worktree);
        if let WorktreeState::Registered(_) = worktree_state {
            cx.spawn(|this, mut cx| async move {
                cx.background_executor()
                    .timer(BACKGROUND_INDEXING_DELAY)
                    .await;
                if let Some((this, project)) = this.upgrade().zip(project.upgrade()) {
                    this.update(&mut cx, |this, cx| {
                        this.index_project(project, cx).detach_and_log_err(cx)
                    })?;
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn register_worktree(
        &mut self,
        project: Model<Project>,
        worktree: Model<Worktree>,
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
        let (mut done_tx, done_rx) = watch::channel();
        let registration = cx.spawn(|this, mut cx| {
            async move {
                let register = async {
                    scan_complete.await;
                    let db_id = db.find_or_create_worktree(worktree_abs_path).await?;
                    let mut file_mtimes = db.get_file_mtimes(db_id).await?;
                    let worktree = if let Some(project) = project.upgrade() {
                        project
                            .read_with(&cx, |project, cx| project.worktree_for_id(worktree_id, cx))
                            .ok()
                            .flatten()
                            .context("worktree not found")?
                    } else {
                        return anyhow::Ok(());
                    };
                    let worktree = worktree.read_with(&cx, |worktree, _| worktree.snapshot())?;
                    let mut changed_paths = cx
                        .background_executor()
                        .spawn(async move {
                            let mut changed_paths = BTreeMap::new();
                            for file in worktree.files(false, 0) {
                                let absolute_path = worktree.absolutize(&file.path)?;

                                if file.is_external || file.is_ignored || file.is_symlink {
                                    continue;
                                }

                                if let Ok(language) = language_registry
                                    .language_for_file(&absolute_path, None)
                                    .await
                                {
                                    // Test if file is valid parseable file
                                    if !PARSEABLE_ENTIRE_FILE_TYPES
                                        .contains(&language.name().as_ref())
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
                                        .map_or(false, |existing_mtime| {
                                            existing_mtime == file.mtime
                                        });

                                    if !already_stored {
                                        changed_paths.insert(
                                            file.path.clone(),
                                            ChangedPathInfo {
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
                                        mtime,
                                        is_deleted: true,
                                    },
                                );
                            }

                            anyhow::Ok(changed_paths)
                        })
                        .await?;
                    this.update(&mut cx, |this, cx| {
                        let project_state = this
                            .projects
                            .get_mut(&project)
                            .context("project not registered")?;
                        let project = project.upgrade().context("project was dropped")?;

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
                        this.index_project(project, cx).detach_and_log_err(cx);

                        anyhow::Ok(())
                    })??;

                    anyhow::Ok(())
                };

                if register.await.log_err().is_none() {
                    // Stop tracking this worktree if the registration failed.
                    this.update(&mut cx, |this, _| {
                        if let Some(project_state) = this.projects.get_mut(&project) {
                            project_state.worktrees.remove(&worktree_id);
                        }
                    })
                    .ok();
                }

                *done_tx.borrow_mut() = Some(());
            }
        });
        project_state.worktrees.insert(
            worktree_id,
            WorktreeState::Registering(RegisteringWorktreeState {
                changed_paths: Default::default(),
                done_rx,
                _registration: registration,
            }),
        );
    }

    fn project_worktrees_changed(&mut self, project: Model<Project>, cx: &mut ModelContext<Self>) {
        let project_state = if let Some(project_state) = self.projects.get_mut(&project.downgrade())
        {
            project_state
        } else {
            return;
        };

        let mut worktrees = project
            .read(cx)
            .worktrees()
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
            !project_state.worktrees.contains_key(&worktree_id)
        });
        for worktree in worktrees {
            self.register_worktree(project.clone(), worktree, cx);
        }
    }

    pub fn pending_file_count(&self, project: &Model<Project>) -> Option<watch::Receiver<usize>> {
        Some(
            self.projects
                .get(&project.downgrade())?
                .pending_file_count_rx
                .clone(),
        )
    }

    pub fn search_project(
        &mut self,
        project: Model<Project>,
        query: String,
        limit: usize,
        includes: Vec<PathMatcher>,
        excludes: Vec<PathMatcher>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<SearchResult>>> {
        if query.is_empty() {
            return Task::ready(Ok(Vec::new()));
        }

        let index = self.index_project(project.clone(), cx);
        let embedding_provider = self.embedding_provider.clone();

        cx.spawn(|this, mut cx| async move {
            index.await?;
            let t0 = Instant::now();

            let query = embedding_provider
                .embed_batch(vec![query])
                .await?
                .pop()
                .context("could not embed query")?;
            log::trace!("Embedding Search Query: {:?}ms", t0.elapsed().as_millis());

            let search_start = Instant::now();
            let modified_buffer_results = this.update(&mut cx, |this, cx| {
                this.search_modified_buffers(
                    &project,
                    query.clone(),
                    limit,
                    &includes,
                    &excludes,
                    cx,
                )
            })?;
            let file_results = this.update(&mut cx, |this, cx| {
                this.search_files(project, query, limit, includes, excludes, cx)
            })?;
            let (modified_buffer_results, file_results) =
                futures::join!(modified_buffer_results, file_results);

            // Weave together the results from modified buffers and files.
            let mut results = Vec::new();
            let mut modified_buffers = HashSet::default();
            for result in modified_buffer_results.log_err().unwrap_or_default() {
                modified_buffers.insert(result.buffer.clone());
                results.push(result);
            }
            for result in file_results.log_err().unwrap_or_default() {
                if !modified_buffers.contains(&result.buffer) {
                    results.push(result);
                }
            }
            results.sort_by_key(|result| Reverse(result.similarity));
            results.truncate(limit);
            log::trace!("Semantic search took {:?}", search_start.elapsed());
            Ok(results)
        })
    }

    pub fn search_files(
        &mut self,
        project: Model<Project>,
        query: Embedding,
        limit: usize,
        includes: Vec<PathMatcher>,
        excludes: Vec<PathMatcher>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<SearchResult>>> {
        let db_path = self.db.path().clone();
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            let database = VectorDatabase::new(
                fs.clone(),
                db_path.clone(),
                cx.background_executor().clone(),
            )
            .await?;

            let worktree_db_ids = this.read_with(&cx, |this, _| {
                let project_state = this
                    .projects
                    .get(&project.downgrade())
                    .context("project was not indexed")?;
                let worktree_db_ids = project_state
                    .worktrees
                    .values()
                    .filter_map(|worktree| {
                        if let WorktreeState::Registered(worktree) = worktree {
                            Some(worktree.db_id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<i64>>();
                anyhow::Ok(worktree_db_ids)
            })??;

            let file_ids = database
                .retrieve_included_file_ids(&worktree_db_ids, &includes, &excludes)
                .await?;

            let batch_n = cx.background_executor().num_cpus();
            let ids_len = file_ids.clone().len();
            let minimum_batch_size = 50;

            let batch_size = {
                let size = ids_len / batch_n;
                if size < minimum_batch_size {
                    minimum_batch_size
                } else {
                    size
                }
            };

            let mut batch_results = Vec::new();
            for batch in file_ids.chunks(batch_size) {
                let batch = batch.into_iter().map(|v| *v).collect::<Vec<i64>>();
                let fs = fs.clone();
                let db_path = db_path.clone();
                let query = query.clone();
                if let Some(db) =
                    VectorDatabase::new(fs, db_path.clone(), cx.background_executor().clone())
                        .await
                        .log_err()
                {
                    batch_results.push(async move {
                        db.top_k_search(&query, limit, batch.as_slice()).await
                    });
                }
            }

            let batch_results = futures::future::join_all(batch_results).await;

            let mut results = Vec::new();
            for batch_result in batch_results {
                if batch_result.is_ok() {
                    for (id, similarity) in batch_result.unwrap() {
                        let ix = match results
                            .binary_search_by_key(&Reverse(similarity), |(_, s)| Reverse(*s))
                        {
                            Ok(ix) => ix,
                            Err(ix) => ix,
                        };

                        results.insert(ix, (id, similarity));
                        results.truncate(limit);
                    }
                }
            }

            let ids = results.iter().map(|(id, _)| *id).collect::<Vec<i64>>();
            let scores = results
                .into_iter()
                .map(|(_, score)| score)
                .collect::<Vec<_>>();
            let spans = database.spans_for_ids(ids.as_slice()).await?;

            let mut tasks = Vec::new();
            let mut ranges = Vec::new();
            let weak_project = project.downgrade();
            project.update(&mut cx, |project, cx| {
                let this = this.upgrade().context("index was dropped")?;
                for (worktree_db_id, file_path, byte_range) in spans {
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
            })??;

            let buffers = futures::future::join_all(tasks).await;
            Ok(buffers
                .into_iter()
                .zip(ranges)
                .zip(scores)
                .filter_map(|((buffer, range), similarity)| {
                    let buffer = buffer.log_err()?;
                    let range = buffer
                        .read_with(&cx, |buffer, _| {
                            let start = buffer.clip_offset(range.start, Bias::Left);
                            let end = buffer.clip_offset(range.end, Bias::Right);
                            buffer.anchor_before(start)..buffer.anchor_after(end)
                        })
                        .log_err()?;
                    Some(SearchResult {
                        buffer,
                        range,
                        similarity,
                    })
                })
                .collect())
        })
    }

    fn search_modified_buffers(
        &self,
        project: &Model<Project>,
        query: Embedding,
        limit: usize,
        includes: &[PathMatcher],
        excludes: &[PathMatcher],
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<SearchResult>>> {
        let modified_buffers = project
            .read(cx)
            .opened_buffers()
            .into_iter()
            .filter_map(|buffer_handle| {
                let buffer = buffer_handle.read(cx);
                let snapshot = buffer.snapshot();
                let excluded = snapshot.resolve_file_path(cx, false).map_or(false, |path| {
                    excludes.iter().any(|matcher| matcher.is_match(&path))
                });

                let included = if includes.len() == 0 {
                    true
                } else {
                    snapshot.resolve_file_path(cx, false).map_or(false, |path| {
                        includes.iter().any(|matcher| matcher.is_match(&path))
                    })
                };

                if buffer.is_dirty() && !excluded && included {
                    Some((buffer_handle, snapshot))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        let embedding_provider = self.embedding_provider.clone();
        let fs = self.fs.clone();
        let db_path = self.db.path().clone();
        let background = cx.background_executor().clone();
        cx.background_executor().spawn(async move {
            let db = VectorDatabase::new(fs, db_path.clone(), background).await?;
            let mut results = Vec::<SearchResult>::new();

            let mut retriever = CodeContextRetriever::new(embedding_provider.clone());
            for (buffer, snapshot) in modified_buffers {
                let language = snapshot
                    .language_at(0)
                    .cloned()
                    .unwrap_or_else(|| language::PLAIN_TEXT.clone());
                let mut spans = retriever
                    .parse_file_with_template(None, &snapshot.text(), language)
                    .log_err()
                    .unwrap_or_default();
                if Self::embed_spans(&mut spans, embedding_provider.as_ref(), &db)
                    .await
                    .log_err()
                    .is_some()
                {
                    for span in spans {
                        let similarity = span.embedding.unwrap().similarity(&query);
                        let ix = match results
                            .binary_search_by_key(&Reverse(similarity), |result| {
                                Reverse(result.similarity)
                            }) {
                            Ok(ix) => ix,
                            Err(ix) => ix,
                        };

                        let range = {
                            let start = snapshot.clip_offset(span.range.start, Bias::Left);
                            let end = snapshot.clip_offset(span.range.end, Bias::Right);
                            snapshot.anchor_before(start)..snapshot.anchor_after(end)
                        };

                        results.insert(
                            ix,
                            SearchResult {
                                buffer: buffer.clone(),
                                range,
                                similarity,
                            },
                        );
                        results.truncate(limit);
                    }
                }
            }

            Ok(results)
        })
    }

    pub fn index_project(
        &mut self,
        project: Model<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if self.is_authenticated() {
            self.index_project_internal(project, cx)
        } else {
            let authenticate = self.authenticate(cx);
            cx.spawn(|this, mut cx| async move {
                if authenticate.await {
                    this.update(&mut cx, |this, cx| this.index_project_internal(project, cx))?
                        .await
                } else {
                    Err(anyhow!("user is not authenticated"))
                }
            })
        }
    }

    fn index_project_internal(
        &mut self,
        project: Model<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.projects.contains_key(&project.downgrade()) {
            let subscription = cx.subscribe(&project, |this, project, event, cx| match event {
                project::Event::WorktreeAdded | project::Event::WorktreeRemoved(_) => {
                    this.project_worktrees_changed(project.clone(), cx);
                }
                project::Event::WorktreeUpdatedEntries(worktree_id, changes) => {
                    this.project_entries_changed(project, *worktree_id, changes.clone(), cx);
                }
                _ => {}
            });
            let project_state = ProjectState::new(subscription, cx);
            self.projects.insert(project.downgrade(), project_state);
            self.project_worktrees_changed(project.clone(), cx);
        }
        let project_state = self.projects.get_mut(&project.downgrade()).unwrap();
        project_state.pending_index += 1;
        cx.notify();

        let mut pending_file_count_rx = project_state.pending_file_count_rx.clone();
        let db = self.db.clone();
        let language_registry = self.language_registry.clone();
        let parsing_files_tx = self.parsing_files_tx.clone();
        let worktree_registration = self.wait_for_worktree_registration(&project, cx);

        cx.spawn(|this, mut cx| async move {
            worktree_registration.await?;

            let mut pending_files = Vec::new();
            let mut files_to_delete = Vec::new();
            this.update(&mut cx, |this, cx| {
                let project_state = this
                    .projects
                    .get_mut(&project.downgrade())
                    .context("project was dropped")?;
                let pending_file_count_tx = &project_state.pending_file_count_tx;

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

                        for (path, info) in &worktree_state.changed_paths {
                            if info.is_deleted {
                                files_to_delete.push((worktree_state.db_id, path.clone()));
                            } else if let Ok(absolute_path) = worktree.read(cx).absolutize(path) {
                                let job_handle = JobHandle::new(pending_file_count_tx);
                                pending_files.push(PendingFile {
                                    absolute_path,
                                    relative_path: path.clone(),
                                    language: None,
                                    job_handle,
                                    modified_time: info.mtime,
                                    worktree_db_id: worktree_state.db_id,
                                });
                            }
                        }
                        worktree_state.changed_paths.clear();
                        true
                    });

                anyhow::Ok(())
            })??;

            cx.background_executor()
                .spawn(async move {
                    for (worktree_db_id, path) in files_to_delete {
                        db.delete_file(worktree_db_id, path).await.log_err();
                    }

                    let embeddings_for_digest = {
                        let mut files = HashMap::default();
                        for pending_file in &pending_files {
                            files
                                .entry(pending_file.worktree_db_id)
                                .or_insert(Vec::new())
                                .push(pending_file.relative_path.clone());
                        }
                        Arc::new(
                            db.embeddings_for_files(files)
                                .await
                                .log_err()
                                .unwrap_or_default(),
                        )
                    };

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

                    // Wait until we're done indexing.
                    while let Some(count) = pending_file_count_rx.next().await {
                        if count == 0 {
                            break;
                        }
                    }
                })
                .await;

            this.update(&mut cx, |this, cx| {
                let project_state = this
                    .projects
                    .get_mut(&project.downgrade())
                    .context("project was dropped")?;
                project_state.pending_index -= 1;
                cx.notify();
                anyhow::Ok(())
            })??;

            Ok(())
        })
    }

    fn wait_for_worktree_registration(
        &self,
        project: &Model<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let project = project.downgrade();
        cx.spawn(|this, cx| async move {
            loop {
                let mut pending_worktrees = Vec::new();
                this.upgrade()
                    .context("semantic index dropped")?
                    .read_with(&cx, |this, _| {
                        if let Some(project) = this.projects.get(&project) {
                            for worktree in project.worktrees.values() {
                                if let WorktreeState::Registering(worktree) = worktree {
                                    pending_worktrees.push(worktree.done());
                                }
                            }
                        }
                    })?;

                if pending_worktrees.is_empty() {
                    break;
                } else {
                    future::join_all(pending_worktrees).await;
                }
            }
            Ok(())
        })
    }

    async fn embed_spans(
        spans: &mut [Span],
        embedding_provider: &dyn EmbeddingProvider,
        db: &VectorDatabase,
    ) -> Result<()> {
        let mut batch = Vec::new();
        let mut batch_tokens = 0;
        let mut embeddings = Vec::new();

        let digests = spans
            .iter()
            .map(|span| span.digest.clone())
            .collect::<Vec<_>>();
        let embeddings_for_digests = db
            .embeddings_for_digests(digests)
            .await
            .log_err()
            .unwrap_or_default();

        for span in &*spans {
            if embeddings_for_digests.contains_key(&span.digest) {
                continue;
            };

            if batch_tokens + span.token_count > embedding_provider.max_tokens_per_batch() {
                let batch_embeddings = embedding_provider
                    .embed_batch(mem::take(&mut batch))
                    .await?;
                embeddings.extend(batch_embeddings);
                batch_tokens = 0;
            }

            batch_tokens += span.token_count;
            batch.push(span.content.clone());
        }

        if !batch.is_empty() {
            let batch_embeddings = embedding_provider
                .embed_batch(mem::take(&mut batch))
                .await?;

            embeddings.extend(batch_embeddings);
        }

        let mut embeddings = embeddings.into_iter();
        for span in spans {
            let embedding = if let Some(embedding) = embeddings_for_digests.get(&span.digest) {
                Some(embedding.clone())
            } else {
                embeddings.next()
            };
            let embedding = embedding.context("failed to embed spans")?;
            span.embedding = Some(embedding);
        }
        Ok(())
    }
}

impl Drop for JobHandle {
    fn drop(&mut self) {
        if let Some(inner) = Arc::get_mut(&mut self.tx) {
            // This is the last instance of the JobHandle (regardless of its origin - whether it was cloned or not)
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
