mod db;
mod embedding;
mod modal;
mod parsing;
mod vector_store_settings;

#[cfg(test)]
mod vector_store_tests;

use crate::vector_store_settings::VectorStoreSettings;
use anyhow::{anyhow, Result};
use db::VectorDatabase;
use embedding::{EmbeddingProvider, OpenAIEmbeddings};
use futures::{channel::oneshot, Future};
use gpui::{
    AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Subscription, Task,
    ViewContext, WeakModelHandle,
};
use language::{Language, LanguageRegistry};
use modal::{SemanticSearch, SemanticSearchDelegate, Toggle};
use parsing::{CodeContextRetriever, ParsedFile};
use project::{Fs, PathChange, Project, ProjectEntryId, WorktreeId};
use settings::SettingsStore;
use smol::channel;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use tree_sitter::{Parser, QueryCursor};
use util::{
    channel::{ReleaseChannel, RELEASE_CHANNEL, RELEASE_CHANNEL_NAME},
    http::HttpClient,
    paths::EMBEDDINGS_DIR,
    ResultExt,
};
use workspace::{Workspace, WorkspaceCreated};

pub fn init(
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AppContext,
) {
    if *RELEASE_CHANNEL == ReleaseChannel::Stable {
        return;
    }

    settings::register::<VectorStoreSettings>(cx);

    if !settings::get::<VectorStoreSettings>(cx).enable {
        return;
    }

    let db_file_path = EMBEDDINGS_DIR
        .join(Path::new(RELEASE_CHANNEL_NAME.as_str()))
        .join("embeddings_db");

    cx.spawn(move |mut cx| async move {
        let vector_store = VectorStore::new(
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
            cx.subscribe_global::<WorkspaceCreated, _>({
                let vector_store = vector_store.clone();
                move |event, cx| {
                    let workspace = &event.0;
                    if let Some(workspace) = workspace.upgrade(cx) {
                        let project = workspace.read(cx).project().clone();
                        if project.read(cx).is_local() {
                            vector_store.update(cx, |store, cx| {
                                store.add_project(project, cx).detach();
                            });
                        }
                    }
                }
            })
            .detach();

            cx.add_action({
                // "semantic search: Toggle"
                move |workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>| {
                    let vector_store = vector_store.clone();
                    workspace.toggle_modal(cx, |workspace, cx| {
                        let project = workspace.project().clone();
                        let workspace = cx.weak_handle();
                        cx.add_view(|cx| {
                            SemanticSearch::new(
                                SemanticSearchDelegate::new(workspace, project, vector_store),
                                cx,
                            )
                        })
                    })
                }
            });

            SemanticSearch::init(cx);
        });

        anyhow::Ok(())
    })
    .detach();
}

pub struct VectorStore {
    fs: Arc<dyn Fs>,
    database_url: Arc<PathBuf>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    language_registry: Arc<LanguageRegistry>,
    db_update_tx: channel::Sender<DbOperation>,
    parsing_files_tx: channel::Sender<PendingFile>,
    _db_update_task: Task<()>,
    _embed_batch_task: Task<()>,
    _batch_files_task: Task<()>,
    _parsing_files_tasks: Vec<Task<()>>,
    projects: HashMap<WeakModelHandle<Project>, ProjectState>,
}

struct ProjectState {
    worktree_db_ids: Vec<(WorktreeId, i64)>,
    pending_files: HashMap<PathBuf, (PendingFile, SystemTime)>,
    _subscription: gpui::Subscription,
}

impl ProjectState {
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

    fn update_pending_files(&mut self, pending_file: PendingFile, indexing_time: SystemTime) {
        // If Pending File Already Exists, Replace it with the new one
        // but keep the old indexing time
        if let Some(old_file) = self
            .pending_files
            .remove(&pending_file.relative_path.clone())
        {
            self.pending_files.insert(
                pending_file.relative_path.clone(),
                (pending_file, old_file.1),
            );
        } else {
            self.pending_files.insert(
                pending_file.relative_path.clone(),
                (pending_file, indexing_time),
            );
        };
    }

    fn get_outstanding_files(&mut self) -> Vec<PendingFile> {
        let mut outstanding_files = vec![];
        let mut remove_keys = vec![];
        for key in self.pending_files.keys().into_iter() {
            if let Some(pending_details) = self.pending_files.get(key) {
                let (pending_file, index_time) = pending_details;
                if index_time <= &SystemTime::now() {
                    outstanding_files.push(pending_file.clone());
                    remove_keys.push(key.clone());
                }
            }
        }

        for key in remove_keys.iter() {
            self.pending_files.remove(key);
        }

        return outstanding_files;
    }
}

#[derive(Clone, Debug)]
pub struct PendingFile {
    worktree_db_id: i64,
    relative_path: PathBuf,
    absolute_path: PathBuf,
    language: Arc<Language>,
    modified_time: SystemTime,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub worktree_id: WorktreeId,
    pub name: String,
    pub offset: usize,
    pub file_path: PathBuf,
}

enum DbOperation {
    InsertFile {
        worktree_id: i64,
        indexed_file: ParsedFile,
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
}

enum EmbeddingJob {
    Enqueue {
        worktree_id: i64,
        parsed_file: ParsedFile,
        document_spans: Vec<String>,
    },
    Flush,
}

impl VectorStore {
    async fn new(
        fs: Arc<dyn Fs>,
        database_url: PathBuf,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        language_registry: Arc<LanguageRegistry>,
        mut cx: AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let database_url = Arc::new(database_url);

        let db = cx
            .background()
            .spawn({
                let fs = fs.clone();
                let database_url = database_url.clone();
                async move {
                    if let Some(db_directory) = database_url.parent() {
                        fs.create_dir(db_directory).await.log_err();
                    }

                    let db = VectorDatabase::new(database_url.to_string_lossy().to_string())?;
                    anyhow::Ok(db)
                }
            })
            .await?;

        Ok(cx.add_model(|cx| {
            // paths_tx -> embeddings_tx -> db_update_tx

            //db_update_tx/rx: Updating Database
            let (db_update_tx, db_update_rx) = channel::unbounded();
            let _db_update_task = cx.background().spawn(async move {
                while let Ok(job) = db_update_rx.recv().await {
                    match job {
                        DbOperation::InsertFile {
                            worktree_id,
                            indexed_file,
                        } => {
                            db.insert_file(worktree_id, indexed_file).log_err();
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
                    }
                }
            });

            // embed_tx/rx: Embed Batch and Send to Database
            let (embed_batch_tx, embed_batch_rx) =
                channel::unbounded::<Vec<(i64, ParsedFile, Vec<String>)>>();
            let _embed_batch_task = cx.background().spawn({
                let db_update_tx = db_update_tx.clone();
                let embedding_provider = embedding_provider.clone();
                async move {
                    while let Ok(mut embeddings_queue) = embed_batch_rx.recv().await {
                        // Construct Batch
                        let mut document_spans = vec![];
                        for (_, _, document_span) in embeddings_queue.iter() {
                            document_spans.extend(document_span.iter().map(|s| s.as_str()));
                        }

                        if let Ok(embeddings) = embedding_provider.embed_batch(document_spans).await
                        {
                            let mut i = 0;
                            let mut j = 0;

                            for embedding in embeddings.iter() {
                                while embeddings_queue[i].1.documents.len() == j {
                                    i += 1;
                                    j = 0;
                                }

                                embeddings_queue[i].1.documents[j].embedding = embedding.to_owned();
                                j += 1;
                            }

                            for (worktree_id, indexed_file, _) in embeddings_queue.into_iter() {
                                for document in indexed_file.documents.iter() {
                                    // TODO: Update this so it doesn't panic
                                    assert!(
                                        document.embedding.len() > 0,
                                        "Document Embedding Not Complete"
                                    );
                                }

                                db_update_tx
                                    .send(DbOperation::InsertFile {
                                        worktree_id,
                                        indexed_file,
                                    })
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                }
            });

            // batch_tx/rx: Batch Files to Send for Embeddings
            let batch_size = settings::get::<VectorStoreSettings>(cx).embedding_batch_size;
            let (batch_files_tx, batch_files_rx) = channel::unbounded::<EmbeddingJob>();
            let _batch_files_task = cx.background().spawn(async move {
                let mut queue_len = 0;
                let mut embeddings_queue = vec![];

                while let Ok(job) = batch_files_rx.recv().await {
                    let should_flush = match job {
                        EmbeddingJob::Enqueue {
                            document_spans,
                            worktree_id,
                            parsed_file,
                        } => {
                            queue_len += &document_spans.len();
                            embeddings_queue.push((worktree_id, parsed_file, document_spans));
                            queue_len >= batch_size
                        }
                        EmbeddingJob::Flush => true,
                    };

                    if should_flush {
                        embed_batch_tx.try_send(embeddings_queue).unwrap();
                        embeddings_queue = vec![];
                        queue_len = 0;
                    }
                }
            });

            // parsing_files_tx/rx: Parsing Files to Embeddable Documents
            let (parsing_files_tx, parsing_files_rx) = channel::unbounded::<PendingFile>();

            let mut _parsing_files_tasks = Vec::new();
            for _ in 0..cx.background().num_cpus() {
                let fs = fs.clone();
                let parsing_files_rx = parsing_files_rx.clone();
                let batch_files_tx = batch_files_tx.clone();
                _parsing_files_tasks.push(cx.background().spawn(async move {
                    let parser = Parser::new();
                    let cursor = QueryCursor::new();
                    let mut retriever = CodeContextRetriever { parser, cursor, fs };
                    while let Ok(pending_file) = parsing_files_rx.recv().await {
                        if let Some((indexed_file, document_spans)) =
                            retriever.parse_file(pending_file.clone()).await.log_err()
                        {
                            batch_files_tx
                                .try_send(EmbeddingJob::Enqueue {
                                    worktree_id: pending_file.worktree_db_id,
                                    parsed_file: indexed_file,
                                    document_spans,
                                })
                                .unwrap();
                        }

                        if parsing_files_rx.len() == 0 {
                            batch_files_tx.try_send(EmbeddingJob::Flush).unwrap();
                        }
                    }
                }));
            }

            Self {
                fs,
                database_url,
                embedding_provider,
                language_registry,
                db_update_tx,
                parsing_files_tx,
                _db_update_task,
                _embed_batch_task,
                _batch_files_task,
                _parsing_files_tasks,
                projects: HashMap::new(),
            }
        }))
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

    fn add_project(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
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

        let fs = self.fs.clone();
        let language_registry = self.language_registry.clone();
        let database_url = self.database_url.clone();
        let db_update_tx = self.db_update_tx.clone();
        let parsing_files_tx = self.parsing_files_tx.clone();

        cx.spawn(|this, mut cx| async move {
            futures::future::join_all(worktree_scans_complete).await;

            let worktree_db_ids = futures::future::join_all(worktree_db_ids).await;

            if let Some(db_directory) = database_url.parent() {
                fs.create_dir(db_directory).await.log_err();
            }

            let worktrees = project.read_with(&cx, |project, cx| {
                project
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).snapshot())
                    .collect::<Vec<_>>()
            });

            let mut worktree_file_times = HashMap::new();
            let mut db_ids_by_worktree_id = HashMap::new();
            for (worktree, db_id) in worktrees.iter().zip(worktree_db_ids) {
                let db_id = db_id?;
                db_ids_by_worktree_id.insert(worktree.id(), db_id);
                worktree_file_times.insert(
                    worktree.id(),
                    this.read_with(&cx, |this, _| this.get_file_mtimes(db_id))
                        .await?,
                );
            }

            cx.background()
                .spawn({
                    let db_ids_by_worktree_id = db_ids_by_worktree_id.clone();
                    let db_update_tx = db_update_tx.clone();
                    let language_registry = language_registry.clone();
                    let parsing_files_tx = parsing_files_tx.clone();
                    async move {
                        let t0 = Instant::now();
                        for worktree in worktrees.into_iter() {
                            let mut file_mtimes =
                                worktree_file_times.remove(&worktree.id()).unwrap();
                            for file in worktree.files(false, 0) {
                                let absolute_path = worktree.absolutize(&file.path);

                                if let Ok(language) = language_registry
                                    .language_for_file(&absolute_path, None)
                                    .await
                                {
                                    if language
                                        .grammar()
                                        .and_then(|grammar| grammar.embedding_config.as_ref())
                                        .is_none()
                                    {
                                        continue;
                                    }

                                    let path_buf = file.path.to_path_buf();
                                    let stored_mtime = file_mtimes.remove(&file.path.to_path_buf());
                                    let already_stored = stored_mtime
                                        .map_or(false, |existing_mtime| {
                                            existing_mtime == file.mtime
                                        });

                                    if !already_stored {
                                        parsing_files_tx
                                            .try_send(PendingFile {
                                                worktree_db_id: db_ids_by_worktree_id
                                                    [&worktree.id()],
                                                relative_path: path_buf,
                                                absolute_path,
                                                language,
                                                modified_time: file.mtime,
                                            })
                                            .unwrap();
                                    }
                                }
                            }
                            for file in file_mtimes.keys() {
                                db_update_tx
                                    .try_send(DbOperation::Delete {
                                        worktree_id: db_ids_by_worktree_id[&worktree.id()],
                                        path: file.to_owned(),
                                    })
                                    .unwrap();
                            }
                        }
                        log::info!(
                            "Parsing Worktree Completed in {:?}",
                            t0.elapsed().as_millis()
                        );
                    }
                })
                .detach();

            // let mut pending_files: Vec<(PathBuf, ((i64, PathBuf, Arc<Language>, SystemTime), SystemTime))> = vec![];
            this.update(&mut cx, |this, cx| {
                // The below is managing for updated on save
                // Currently each time a file is saved, this code is run, and for all the files that were changed, if the current time is
                // greater than the previous embedded time by the REINDEXING_DELAY variable, we will send the file off to be indexed.
                let _subscription = cx.subscribe(&project, |this, project, event, cx| {
                    if let project::Event::WorktreeUpdatedEntries(worktree_id, changes) = event {
                        this.project_entries_changed(project, changes.clone(), cx, worktree_id);
                    }
                });

                this.projects.insert(
                    project.downgrade(),
                    ProjectState {
                        pending_files: HashMap::new(),
                        worktree_db_ids: db_ids_by_worktree_id.into_iter().collect(),
                        _subscription,
                    },
                );
            });

            anyhow::Ok(())
        })
    }

    pub fn search(
        &mut self,
        project: ModelHandle<Project>,
        phrase: String,
        limit: usize,
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
        cx.spawn(|this, cx| async move {
            let documents = cx
                .background()
                .spawn(async move {
                    let database = VectorDatabase::new(database_url.to_string_lossy().into())?;

                    let phrase_embedding = embedding_provider
                        .embed_batch(vec![&phrase])
                        .await?
                        .into_iter()
                        .next()
                        .unwrap();

                    database.top_k_search(&worktree_db_ids, &phrase_embedding, limit)
                })
                .await?;

            this.read_with(&cx, |this, _| {
                let project_state = if let Some(state) = this.projects.get(&project.downgrade()) {
                    state
                } else {
                    return Err(anyhow!("project not added"));
                };

                Ok(documents
                    .into_iter()
                    .filter_map(|(worktree_db_id, file_path, offset, name)| {
                        let worktree_id = project_state.worktree_id_for_db_id(worktree_db_id)?;
                        Some(SearchResult {
                            worktree_id,
                            name,
                            offset,
                            file_path,
                        })
                    })
                    .collect())
            })
        })
    }

    fn project_entries_changed(
        &mut self,
        project: ModelHandle<Project>,
        changes: Arc<[(Arc<Path>, ProjectEntryId, PathChange)]>,
        cx: &mut ModelContext<'_, VectorStore>,
        worktree_id: &WorktreeId,
    ) -> Option<()> {
        let reindexing_delay = settings::get::<VectorStoreSettings>(cx).reindexing_delay_seconds;

        let worktree = project
            .read(cx)
            .worktree_for_id(worktree_id.clone(), cx)?
            .read(cx)
            .snapshot();

        let worktree_db_id = self
            .projects
            .get(&project.downgrade())?
            .db_id_for_worktree_id(worktree.id())?;
        let file_mtimes = self.get_file_mtimes(worktree_db_id);

        let language_registry = self.language_registry.clone();

        cx.spawn(|this, mut cx| async move {
            let file_mtimes = file_mtimes.await.log_err()?;

            for change in changes.into_iter() {
                let change_path = change.0.clone();
                let absolute_path = worktree.absolutize(&change_path);

                // Skip if git ignored or symlink
                if let Some(entry) = worktree.entry_for_id(change.1) {
                    if entry.is_ignored || entry.is_symlink || entry.is_external {
                        continue;
                    }
                }

                match change.2 {
                    PathChange::Removed => this.update(&mut cx, |this, _| {
                        this.db_update_tx
                            .try_send(DbOperation::Delete {
                                worktree_id: worktree_db_id,
                                path: absolute_path,
                            })
                            .unwrap();
                    }),
                    _ => {
                        if let Ok(language) = language_registry
                            .language_for_file(&change_path.to_path_buf(), None)
                            .await
                        {
                            if language
                                .grammar()
                                .and_then(|grammar| grammar.embedding_config.as_ref())
                                .is_none()
                            {
                                continue;
                            }

                            let modified_time =
                                change_path.metadata().log_err()?.modified().log_err()?;

                            let existing_time = file_mtimes.get(&change_path.to_path_buf());
                            let already_stored = existing_time
                                .map_or(false, |existing_time| &modified_time != existing_time);

                            if !already_stored {
                                this.update(&mut cx, |this, _| {
                                    let reindex_time = modified_time
                                        + Duration::from_secs(reindexing_delay as u64);

                                    let project_state =
                                        this.projects.get_mut(&project.downgrade())?;
                                    project_state.update_pending_files(
                                        PendingFile {
                                            relative_path: change_path.to_path_buf(),
                                            absolute_path,
                                            modified_time,
                                            worktree_db_id,
                                            language: language.clone(),
                                        },
                                        reindex_time,
                                    );

                                    for file in project_state.get_outstanding_files() {
                                        this.parsing_files_tx.try_send(file).unwrap();
                                    }
                                    Some(())
                                });
                            }
                        }
                    }
                }
            }

            Some(())
        })
        .detach();

        Some(())
    }
}

impl Entity for VectorStore {
    type Event = ();
}
