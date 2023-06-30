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
    time::SystemTime,
};
use tree_sitter::{Parser, QueryCursor};
use util::{
    channel::{ReleaseChannel, RELEASE_CHANNEL, RELEASE_CHANNEL_NAME},
    http::HttpClient,
    paths::EMBEDDINGS_DIR,
    ResultExt,
};
use workspace::{Workspace, WorkspaceCreated};

#[derive(Debug)]
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

#[derive(Debug)]
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
    _db_update_task: Task<()>,
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
            let (db_update_tx, db_update_rx) = channel::unbounded();
            let _db_update_task = cx.background().spawn(async move {
                while let Ok(job) = db_update_rx.recv().await {
                    match job {
                        DbWrite::InsertFile {
                            worktree_id,
                            indexed_file,
                        } => {
                            log::info!("Inserting File: {:?}", &indexed_file.path);
                            db.insert_file(worktree_id, indexed_file).log_err();
                        }
                        DbWrite::Delete { worktree_id, path } => {
                            log::info!("Deleting File: {:?}", &path);
                            db.delete_file(worktree_id, path).log_err();
                        }
                        DbWrite::FindOrCreateWorktree { path, sender } => {
                            let id = db.find_or_create_worktree(&path);
                            sender.send(id).ok();
                        }
                    }
                }
            });

            Self {
                fs,
                database_url,
                db_update_tx,
                embedding_provider,
                language_registry,
                projects: HashMap::new(),
                _db_update_task,
            }
        }))
    }

    async fn index_file(
        cursor: &mut QueryCursor,
        parser: &mut Parser,
        embedding_provider: &dyn EmbeddingProvider,
        fs: &Arc<dyn Fs>,
        language: Arc<Language>,
        file_path: PathBuf,
        mtime: SystemTime,
    ) -> Result<IndexedFile> {
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
                    context_spans.push(item);
                    documents.push(Document {
                        name: name.to_string(),
                        offset: item_range.start,
                        embedding: Vec::new(),
                    });
                }
            }
        }

        if !documents.is_empty() {
            let embeddings = embedding_provider.embed_batch(context_spans).await?;
            for (document, embedding) in documents.iter_mut().zip(embeddings) {
                document.embedding = embedding;
            }
        }

        return Ok(IndexedFile {
            path: file_path,
            mtime,
            documents,
        });
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
        let embedding_provider = self.embedding_provider.clone();
        let database_url = self.database_url.clone();
        let db_update_tx = self.db_update_tx.clone();

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

            let (paths_tx, paths_rx) =
                channel::unbounded::<(i64, PathBuf, Arc<Language>, SystemTime)>();
            cx.background()
                .spawn({
                    let db_ids_by_worktree_id = db_ids_by_worktree_id.clone();
                    let db_update_tx = db_update_tx.clone();
                    async move {
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
                                        paths_tx
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
                    }
                })
                .detach();

            cx.background()
                .scoped(|scope| {
                    for _ in 0..cx.background().num_cpus() {
                        scope.spawn(async {
                            let mut parser = Parser::new();
                            let mut cursor = QueryCursor::new();
                            while let Ok((worktree_id, file_path, language, mtime)) =
                                paths_rx.recv().await
                            {
                                if let Some(indexed_file) = Self::index_file(
                                    &mut cursor,
                                    &mut parser,
                                    embedding_provider.as_ref(),
                                    &fs,
                                    language,
                                    file_path,
                                    mtime,
                                )
                                .await
                                .log_err()
                                {
                                    db_update_tx
                                        .try_send(DbWrite::InsertFile {
                                            worktree_id,
                                            indexed_file,
                                        })
                                        .unwrap();
                                }
                            }
                        });
                    }
                })
                .await;

            this.update(&mut cx, |this, cx| {
                let _subscription = cx.subscribe(&project, |this, project, event, cx| {
                    if let project::Event::WorktreeUpdatedEntries(worktree_id, changes) = event {
                        //
                        log::info!("worktree changes {:?}", changes);
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

            log::info!("Semantic Indexing Complete!");

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

        log::info!("Searching for: {:?}", phrase);

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
