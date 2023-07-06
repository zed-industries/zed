mod db;
mod embedding;
mod modal;

#[cfg(test)]
mod vector_store_tests;

use anyhow::{anyhow, Result};
use db::VectorDatabase;
use embedding::{EmbeddingProvider, OpenAIEmbeddings};
use futures::{channel::oneshot, Future};
use gpui::{
    AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task, ViewContext,
    WeakModelHandle,
};
use language::{Language, LanguageRegistry};
use modal::{SemanticSearch, SemanticSearchDelegate, Toggle};
use project::{Fs, Project, WorktreeId};
use smol::channel;
use std::{
    cmp::Ordering,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Instant, SystemTime},
};
use tree_sitter::{Parser, QueryCursor};
use util::{
    channel::{ReleaseChannel, RELEASE_CHANNEL, RELEASE_CHANNEL_NAME},
    http::HttpClient,
    paths::EMBEDDINGS_DIR,
    ResultExt,
};
use workspace::{Workspace, WorkspaceCreated};

const REINDEXING_DELAY: u64 = 30;
const EMBEDDINGS_BATCH_SIZE: usize = 150;

#[derive(Debug, Clone)]
pub struct Document {
    pub offset: usize,
    pub name: String,
    pub embedding: Vec<f32>,
}

pub fn init(
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AppContext,
) {
    if *RELEASE_CHANNEL == ReleaseChannel::Stable {
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

#[derive(Debug, Clone)]
pub struct IndexedFile {
    path: PathBuf,
    mtime: SystemTime,
    documents: Vec<Document>,
}

pub struct VectorStore {
    fs: Arc<dyn Fs>,
    database_url: Arc<PathBuf>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    language_registry: Arc<LanguageRegistry>,
    db_update_tx: channel::Sender<DbWrite>,
    // embed_batch_tx: channel::Sender<Vec<(i64, IndexedFile, Vec<String>)>>,
    parsing_files_tx: channel::Sender<(i64, PathBuf, Arc<Language>, SystemTime)>,
    _db_update_task: Task<()>,
    _embed_batch_task: Vec<Task<()>>,
    _batch_files_task: Task<()>,
    _parsing_files_tasks: Vec<Task<()>>,
    projects: HashMap<WeakModelHandle<Project>, ProjectState>,
}

struct ProjectState {
    worktree_db_ids: Vec<(WorktreeId, i64)>,
    _subscription: gpui::Subscription,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub worktree_id: WorktreeId,
    pub name: String,
    pub offset: usize,
    pub file_path: PathBuf,
}

enum DbWrite {
    InsertFile {
        worktree_id: i64,
        indexed_file: IndexedFile,
    },
    Delete {
        worktree_id: i64,
        path: PathBuf,
    },
    FindOrCreateWorktree {
        path: PathBuf,
        sender: oneshot::Sender<Result<i64>>,
    },
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
                        DbWrite::InsertFile {
                            worktree_id,
                            indexed_file,
                        } => {
                            log::info!("Inserting Data for {:?}", &indexed_file.path);
                            db.insert_file(worktree_id, indexed_file).log_err();
                        }
                        DbWrite::Delete { worktree_id, path } => {
                            db.delete_file(worktree_id, path).log_err();
                        }
                        DbWrite::FindOrCreateWorktree { path, sender } => {
                            let id = db.find_or_create_worktree(&path);
                            sender.send(id).ok();
                        }
                    }
                }
            });

            // embed_tx/rx: Embed Batch and Send to Database
            let (embed_batch_tx, embed_batch_rx) =
                channel::unbounded::<Vec<(i64, IndexedFile, Vec<String>)>>();
            let mut _embed_batch_task = Vec::new();
            for _ in 0..1 {
                //cx.background().num_cpus() {
                let db_update_tx = db_update_tx.clone();
                let embed_batch_rx = embed_batch_rx.clone();
                let embedding_provider = embedding_provider.clone();
                _embed_batch_task.push(cx.background().spawn(async move {
                    while let Ok(embeddings_queue) = embed_batch_rx.recv().await {
                        // Construct Batch
                        let mut embeddings_queue = embeddings_queue.clone();
                        let mut document_spans = vec![];
                        for (_, _, document_span) in embeddings_queue.clone().into_iter() {
                            document_spans.extend(document_span);
                        }

                        if let Ok(embeddings) = embedding_provider
                            .embed_batch(document_spans.iter().map(|x| &**x).collect())
                            .await
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
                                    .send(DbWrite::InsertFile {
                                        worktree_id,
                                        indexed_file,
                                    })
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                }))
            }

            // batch_tx/rx: Batch Files to Send for Embeddings
            let (batch_files_tx, batch_files_rx) =
                channel::unbounded::<(i64, IndexedFile, Vec<String>)>();
            let _batch_files_task = cx.background().spawn(async move {
                let mut queue_len = 0;
                let mut embeddings_queue = vec![];
                while let Ok((worktree_id, indexed_file, document_spans)) =
                    batch_files_rx.recv().await
                {
                    queue_len += &document_spans.len();
                    embeddings_queue.push((worktree_id, indexed_file, document_spans));
                    if queue_len >= EMBEDDINGS_BATCH_SIZE {
                        embed_batch_tx.try_send(embeddings_queue).unwrap();
                        embeddings_queue = vec![];
                        queue_len = 0;
                    }
                }
                if queue_len > 0 {
                    embed_batch_tx.try_send(embeddings_queue).unwrap();
                }
            });

            // parsing_files_tx/rx: Parsing Files to Embeddable Documents
            let (parsing_files_tx, parsing_files_rx) =
                channel::unbounded::<(i64, PathBuf, Arc<Language>, SystemTime)>();

            let mut _parsing_files_tasks = Vec::new();
            for _ in 0..cx.background().num_cpus() {
                let fs = fs.clone();
                let parsing_files_rx = parsing_files_rx.clone();
                let batch_files_tx = batch_files_tx.clone();
                _parsing_files_tasks.push(cx.background().spawn(async move {
                    let mut parser = Parser::new();
                    let mut cursor = QueryCursor::new();
                    while let Ok((worktree_id, file_path, language, mtime)) =
                        parsing_files_rx.recv().await
                    {
                        log::info!("Parsing File: {:?}", &file_path);
                        if let Some((indexed_file, document_spans)) = Self::index_file(
                            &mut cursor,
                            &mut parser,
                            &fs,
                            language,
                            file_path.clone(),
                            mtime,
                        )
                        .await
                        .log_err()
                        {
                            batch_files_tx
                                .try_send((worktree_id, indexed_file, document_spans))
                                .unwrap();
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

    async fn index_file(
        cursor: &mut QueryCursor,
        parser: &mut Parser,
        fs: &Arc<dyn Fs>,
        language: Arc<Language>,
        file_path: PathBuf,
        mtime: SystemTime,
    ) -> Result<(IndexedFile, Vec<String>)> {
        let grammar = language.grammar().ok_or_else(|| anyhow!("no grammar"))?;
        let embedding_config = grammar
            .embedding_config
            .as_ref()
            .ok_or_else(|| anyhow!("no outline query"))?;

        let content = fs.load(&file_path).await?;

        parser.set_language(grammar.ts_language).unwrap();
        let tree = parser
            .parse(&content, None)
            .ok_or_else(|| anyhow!("parsing failed"))?;

        let mut documents = Vec::new();
        let mut context_spans = Vec::new();
        for mat in cursor.matches(
            &embedding_config.query,
            tree.root_node(),
            content.as_bytes(),
        ) {
            let mut item_range = None;
            let mut name_range = None;
            for capture in mat.captures {
                if capture.index == embedding_config.item_capture_ix {
                    item_range = Some(capture.node.byte_range());
                } else if capture.index == embedding_config.name_capture_ix {
                    name_range = Some(capture.node.byte_range());
                }
            }

            if let Some((item_range, name_range)) = item_range.zip(name_range) {
                if let Some((item, name)) =
                    content.get(item_range.clone()).zip(content.get(name_range))
                {
                    context_spans.push(item.to_string());
                    documents.push(Document {
                        name: name.to_string(),
                        offset: item_range.start,
                        embedding: Vec::new(),
                    });
                }
            }
        }

        return Ok((
            IndexedFile {
                path: file_path,
                mtime,
                documents,
            },
            context_spans,
        ));
    }

    fn find_or_create_worktree(&self, path: PathBuf) -> impl Future<Output = Result<i64>> {
        let (tx, rx) = oneshot::channel();
        self.db_update_tx
            .try_send(DbWrite::FindOrCreateWorktree { path, sender: tx })
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
            let t0 = Instant::now();
            futures::future::join_all(worktree_scans_complete).await;

            let worktree_db_ids = futures::future::join_all(worktree_db_ids).await;
            log::info!("Worktree Scanning Done in {:?}", t0.elapsed().as_millis());

            if let Some(db_directory) = database_url.parent() {
                fs.create_dir(db_directory).await.log_err();
            }

            let worktrees = project.read_with(&cx, |project, cx| {
                project
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).snapshot())
                    .collect::<Vec<_>>()
            });

            // Here we query the worktree ids, and yet we dont have them elsewhere
            // We likely want to clean up these datastructures
            let (mut worktree_file_times, db_ids_by_worktree_id) = cx
                .background()
                .spawn({
                    let worktrees = worktrees.clone();
                    async move {
                        let db = VectorDatabase::new(database_url.to_string_lossy().into())?;
                        let mut db_ids_by_worktree_id = HashMap::new();
                        let mut file_times: HashMap<WorktreeId, HashMap<PathBuf, SystemTime>> =
                            HashMap::new();
                        for (worktree, db_id) in worktrees.iter().zip(worktree_db_ids) {
                            let db_id = db_id?;
                            db_ids_by_worktree_id.insert(worktree.id(), db_id);
                            file_times.insert(worktree.id(), db.get_file_mtimes(db_id)?);
                        }
                        anyhow::Ok((file_times, db_ids_by_worktree_id))
                    }
                })
                .await?;

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
                                            .try_send((
                                                db_ids_by_worktree_id[&worktree.id()],
                                                path_buf,
                                                language,
                                                file.mtime,
                                            ))
                                            .unwrap();
                                    }
                                }
                            }
                            for file in file_mtimes.keys() {
                                db_update_tx
                                    .try_send(DbWrite::Delete {
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

            this.update(&mut cx, |this, cx| {
                // The below is managing for updated on save
                // Currently each time a file is saved, this code is run, and for all the files that were changed, if the current time is
                // greater than the previous embedded time by the REINDEXING_DELAY variable, we will send the file off to be indexed.
                let _subscription = cx.subscribe(&project, |this, project, event, _cx| {
                    if let Some(project_state) = this.projects.get(&project.downgrade()) {
                        let worktree_db_ids = project_state.worktree_db_ids.clone();

                        if let project::Event::WorktreeUpdatedEntries(worktree_id, changes) = event
                        {
                            // Iterate through changes
                            let language_registry = this.language_registry.clone();

                            let db =
                                VectorDatabase::new(this.database_url.to_string_lossy().into());
                            if db.is_err() {
                                return;
                            }
                            let db = db.unwrap();

                            let worktree_db_id: Option<i64> = {
                                let mut found_db_id = None;
                                for (w_id, db_id) in worktree_db_ids.into_iter() {
                                    if &w_id == worktree_id {
                                        found_db_id = Some(db_id);
                                    }
                                }

                                found_db_id
                            };

                            if worktree_db_id.is_none() {
                                return;
                            }
                            let worktree_db_id = worktree_db_id.unwrap();

                            let file_mtimes = db.get_file_mtimes(worktree_db_id);
                            if file_mtimes.is_err() {
                                return;
                            }

                            let file_mtimes = file_mtimes.unwrap();
                            let parsing_files_tx = this.parsing_files_tx.clone();

                            smol::block_on(async move {
                                for change in changes.into_iter() {
                                    let change_path = change.0.clone();
                                    log::info!("Change: {:?}", &change_path);
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

                                        // TODO: Make this a bit more defensive
                                        let modified_time =
                                            change_path.metadata().unwrap().modified().unwrap();
                                        let existing_time =
                                            file_mtimes.get(&change_path.to_path_buf());
                                        let already_stored =
                                            existing_time.map_or(false, |existing_time| {
                                                if &modified_time != existing_time
                                                    && existing_time.elapsed().unwrap().as_secs()
                                                        > REINDEXING_DELAY
                                                {
                                                    false
                                                } else {
                                                    true
                                                }
                                            });

                                        if !already_stored {
                                            log::info!("Need to reindex: {:?}", &change_path);
                                            parsing_files_tx
                                                .try_send((
                                                    worktree_db_id,
                                                    change_path.to_path_buf(),
                                                    language,
                                                    modified_time,
                                                ))
                                                .unwrap();
                                        }
                                    }
                                }
                            })
                        }
                    }
                });

                this.projects.insert(
                    project.downgrade(),
                    ProjectState {
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
                project_state
                    .worktree_db_ids
                    .iter()
                    .find_map(|(id, db_id)| {
                        if *id == worktree_id {
                            Some(*db_id)
                        } else {
                            None
                        }
                    })
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

                    let mut results = Vec::<(i64, f32)>::with_capacity(limit + 1);
                    database.for_each_document(&worktree_db_ids, |id, embedding| {
                        let similarity = dot(&embedding.0, &phrase_embedding);
                        let ix = match results.binary_search_by(|(_, s)| {
                            similarity.partial_cmp(&s).unwrap_or(Ordering::Equal)
                        }) {
                            Ok(ix) => ix,
                            Err(ix) => ix,
                        };
                        results.insert(ix, (id, similarity));
                        results.truncate(limit);
                    })?;

                    let ids = results.into_iter().map(|(id, _)| id).collect::<Vec<_>>();
                    database.get_documents_by_ids(&ids)
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
                        let worktree_id =
                            project_state
                                .worktree_db_ids
                                .iter()
                                .find_map(|(id, db_id)| {
                                    if *db_id == worktree_db_id {
                                        Some(*id)
                                    } else {
                                        None
                                    }
                                })?;
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
}

impl Entity for VectorStore {
    type Event = ();
}

fn dot(vec_a: &[f32], vec_b: &[f32]) -> f32 {
    let len = vec_a.len();
    assert_eq!(len, vec_b.len());

    let mut result = 0.0;
    unsafe {
        matrixmultiply::sgemm(
            1,
            len,
            1,
            1.0,
            vec_a.as_ptr(),
            len as isize,
            1,
            vec_b.as_ptr(),
            1,
            len as isize,
            0.0,
            &mut result as *mut f32,
            1,
            1,
        );
    }
    result
}
