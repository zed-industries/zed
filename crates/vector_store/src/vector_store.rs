mod db;
mod embedding;
mod modal;

#[cfg(test)]
mod vector_store_tests;

use anyhow::{anyhow, Result};
use db::{FileSha1, VectorDatabase};
use embedding::{EmbeddingProvider, OpenAIEmbeddings};
use gpui::{AppContext, Entity, ModelContext, ModelHandle, Task, ViewContext};
use language::{Language, LanguageRegistry};
use modal::{SemanticSearch, SemanticSearchDelegate, Toggle};
use project::{Fs, Project, WorktreeId};
use smol::channel;
use std::{
    cmp::Ordering,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tree_sitter::{Parser, QueryCursor};
use util::{
    channel::RELEASE_CHANNEL_NAME, http::HttpClient, paths::EMBEDDINGS_DIR, ResultExt, TryFutureExt,
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
    let db_file_path = EMBEDDINGS_DIR
        .join(Path::new(RELEASE_CHANNEL_NAME.as_str()))
        .join("embeddings_db");

    let vector_store = cx.add_model(|_| {
        VectorStore::new(
            fs,
            db_file_path,
            Arc::new(OpenAIEmbeddings {
                client: http_client,
            }),
            language_registry,
        )
    });

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
}

#[derive(Debug)]
pub struct IndexedFile {
    path: PathBuf,
    sha1: FileSha1,
    documents: Vec<Document>,
}

pub struct VectorStore {
    fs: Arc<dyn Fs>,
    database_url: Arc<PathBuf>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    language_registry: Arc<LanguageRegistry>,
    worktree_db_ids: Vec<(WorktreeId, i64)>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub worktree_id: WorktreeId,
    pub name: String,
    pub offset: usize,
    pub file_path: PathBuf,
}

impl VectorStore {
    fn new(
        fs: Arc<dyn Fs>,
        database_url: PathBuf,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            fs,
            database_url: Arc::new(database_url),
            embedding_provider,
            language_registry,
            worktree_db_ids: Vec::new(),
        }
    }

    async fn index_file(
        cursor: &mut QueryCursor,
        parser: &mut Parser,
        embedding_provider: &dyn EmbeddingProvider,
        language: Arc<Language>,
        file_path: PathBuf,
        content: String,
    ) -> Result<IndexedFile> {
        let grammar = language.grammar().ok_or_else(|| anyhow!("no grammar"))?;
        let outline_config = grammar
            .outline_config
            .as_ref()
            .ok_or_else(|| anyhow!("no outline query"))?;

        parser.set_language(grammar.ts_language).unwrap();
        let tree = parser
            .parse(&content, None)
            .ok_or_else(|| anyhow!("parsing failed"))?;

        let mut documents = Vec::new();
        let mut context_spans = Vec::new();
        for mat in cursor.matches(&outline_config.query, tree.root_node(), content.as_bytes()) {
            let mut item_range = None;
            let mut name_range = None;
            for capture in mat.captures {
                if capture.index == outline_config.item_capture_ix {
                    item_range = Some(capture.node.byte_range());
                } else if capture.index == outline_config.name_capture_ix {
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

        let sha1 = FileSha1::from_str(content);

        return Ok(IndexedFile {
            path: file_path,
            sha1,
            documents,
        });
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
                    log::info!("worktree scan completed");
                }
            })
            .collect::<Vec<_>>();

        let fs = self.fs.clone();
        let language_registry = self.language_registry.clone();
        let embedding_provider = self.embedding_provider.clone();
        let database_url = self.database_url.clone();

        cx.spawn(|this, mut cx| async move {
            futures::future::join_all(worktree_scans_complete).await;

            // TODO: remove this after fixing the bug in scan_complete
            cx.background()
                .timer(std::time::Duration::from_secs(3))
                .await;

            if let Some(db_directory) = database_url.parent() {
                fs.create_dir(db_directory).await.log_err();
            }
            let db = VectorDatabase::new(database_url.to_string_lossy().into())?;

            let worktrees = project.read_with(&cx, |project, cx| {
                project
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).snapshot())
                    .collect::<Vec<_>>()
            });

            // Here we query the worktree ids, and yet we dont have them elsewhere
            // We likely want to clean up these datastructures
            let (db, worktree_hashes, worktree_db_ids) = cx
                .background()
                .spawn({
                    let worktrees = worktrees.clone();
                    async move {
                        let mut worktree_db_ids: HashMap<WorktreeId, i64> = HashMap::new();
                        let mut hashes: HashMap<WorktreeId, HashMap<PathBuf, FileSha1>> =
                            HashMap::new();
                        for worktree in worktrees {
                            let worktree_db_id =
                                db.find_or_create_worktree(worktree.abs_path().as_ref())?;
                            worktree_db_ids.insert(worktree.id(), worktree_db_id);
                            hashes.insert(worktree.id(), db.get_file_hashes(worktree_db_id)?);
                        }
                        anyhow::Ok((db, hashes, worktree_db_ids))
                    }
                })
                .await?;

            let (paths_tx, paths_rx) =
                channel::unbounded::<(i64, PathBuf, String, Arc<Language>)>();
            let (indexed_files_tx, indexed_files_rx) = channel::unbounded::<(i64, IndexedFile)>();
            cx.background()
                .spawn({
                    let fs = fs.clone();
                    let worktree_db_ids = worktree_db_ids.clone();
                    async move {
                        for worktree in worktrees.into_iter() {
                            let file_hashes = &worktree_hashes[&worktree.id()];
                            for file in worktree.files(false, 0) {
                                let absolute_path = worktree.absolutize(&file.path);

                                if let Ok(language) = language_registry
                                    .language_for_file(&absolute_path, None)
                                    .await
                                {
                                    if language.name().as_ref() != "Rust" {
                                        continue;
                                    }

                                    if let Some(content) = fs.load(&absolute_path).await.log_err() {
                                        log::info!("loaded file: {absolute_path:?}");

                                        let path_buf = file.path.to_path_buf();
                                        let already_stored = file_hashes
                                            .get(&path_buf)
                                            .map_or(false, |existing_hash| {
                                                existing_hash.equals(&content)
                                            });

                                        if !already_stored {
                                            log::info!(
                                                "File Changed (Sending to Parse): {:?}",
                                                &path_buf
                                            );
                                            paths_tx
                                                .try_send((
                                                    worktree_db_ids[&worktree.id()],
                                                    path_buf,
                                                    content,
                                                    language,
                                                ))
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                })
                .detach();

            let db_write_task = cx.background().spawn(
                async move {
                    while let Ok((worktree_id, indexed_file)) = indexed_files_rx.recv().await {
                        db.insert_file(worktree_id, indexed_file).log_err();
                    }

                    anyhow::Ok(())
                }
                .log_err(),
            );

            cx.background()
                .scoped(|scope| {
                    for _ in 0..cx.background().num_cpus() {
                        scope.spawn(async {
                            let mut parser = Parser::new();
                            let mut cursor = QueryCursor::new();
                            while let Ok((worktree_id, file_path, content, language)) =
                                paths_rx.recv().await
                            {
                                if let Some(indexed_file) = Self::index_file(
                                    &mut cursor,
                                    &mut parser,
                                    embedding_provider.as_ref(),
                                    language,
                                    file_path,
                                    content,
                                )
                                .await
                                .log_err()
                                {
                                    indexed_files_tx
                                        .try_send((worktree_id, indexed_file))
                                        .unwrap();
                                }
                            }
                        });
                    }
                })
                .await;
            drop(indexed_files_tx);

            db_write_task.await;

            this.update(&mut cx, |this, _| {
                this.worktree_db_ids.extend(worktree_db_ids);
            });

            anyhow::Ok(())
        })
    }

    pub fn search(
        &mut self,
        project: &ModelHandle<Project>,
        phrase: String,
        limit: usize,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<SearchResult>>> {
        let project = project.read(cx);
        let worktree_db_ids = project
            .worktrees(cx)
            .filter_map(|worktree| {
                let worktree_id = worktree.read(cx).id();
                self.worktree_db_ids.iter().find_map(|(id, db_id)| {
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

            let results = this.read_with(&cx, |this, _| {
                documents
                    .into_iter()
                    .filter_map(|(worktree_db_id, file_path, offset, name)| {
                        let worktree_id = this.worktree_db_ids.iter().find_map(|(id, db_id)| {
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
                    .collect()
            });

            anyhow::Ok(results)
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
