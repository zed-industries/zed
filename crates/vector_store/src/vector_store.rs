mod db;
mod embedding;
mod modal;
mod search;

#[cfg(test)]
mod vector_store_tests;

use anyhow::{anyhow, Result};
use db::{FileSha1, VectorDatabase, VECTOR_DB_URL};
use embedding::{EmbeddingProvider, OpenAIEmbeddings};
use gpui::{actions, AppContext, Entity, ModelContext, ModelHandle, Task, ViewContext};
use language::{Language, LanguageRegistry};
use modal::{SemanticSearch, SemanticSearchDelegate, Toggle};
use project::{Fs, Project};
use smol::channel;
use std::{cmp::Ordering, collections::HashMap, path::PathBuf, sync::Arc};
use tree_sitter::{Parser, QueryCursor};
use util::{http::HttpClient, ResultExt, TryFutureExt};
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
    let vector_store = cx.add_model(|cx| {
        VectorStore::new(
            fs,
            VECTOR_DB_URL.to_string(),
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
    // cx.add_action({
    //     let vector_store = vector_store.clone();
    //     move |workspace: &mut Workspace, _: &TestSearch, cx: &mut ViewContext<Workspace>| {
    //         let t0 = std::time::Instant::now();
    //         let task = vector_store.update(cx, |store, cx| {
    //             store.search("compute embeddings for all of the symbols in the codebase and write them to a database".to_string(), 10, cx)
    //         });

    //         cx.spawn(|this, cx| async move {
    //             let results = task.await?;
    //             let duration = t0.elapsed();

    //             println!("search took {:?}", duration);
    //             println!("results {:?}", results);

    //             anyhow::Ok(())
    //         }).detach()
    //     }
    // });
}

#[derive(Debug)]
pub struct IndexedFile {
    path: PathBuf,
    sha1: FileSha1,
    documents: Vec<Document>,
}

pub struct VectorStore {
    fs: Arc<dyn Fs>,
    database_url: Arc<str>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    language_registry: Arc<LanguageRegistry>,
}

#[derive(Debug)]
pub struct SearchResult {
    pub name: String,
    pub offset: usize,
    pub file_path: PathBuf,
}

impl VectorStore {
    fn new(
        fs: Arc<dyn Fs>,
        database_url: String,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            fs,
            database_url: database_url.into(),
            embedding_provider,
            language_registry,
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

        let embeddings = embedding_provider.embed_batch(context_spans).await?;
        for (document, embedding) in documents.iter_mut().zip(embeddings) {
            document.embedding = embedding;
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

        cx.spawn(|_, cx| async move {
            futures::future::join_all(worktree_scans_complete).await;

            // TODO: remove this after fixing the bug in scan_complete
            cx.background()
                .timer(std::time::Duration::from_secs(3))
                .await;

            let db = VectorDatabase::new(&database_url)?;

            let worktrees = project.read_with(&cx, |project, cx| {
                project
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).snapshot())
                    .collect::<Vec<_>>()
            });

            let worktree_root_paths = worktrees
                .iter()
                .map(|worktree| worktree.abs_path().clone())
                .collect::<Vec<_>>();

            // Here we query the worktree ids, and yet we dont have them elsewhere
            // We likely want to clean up these datastructures
            let (db, worktree_hashes, worktree_ids) = cx
                .background()
                .spawn(async move {
                    let mut worktree_ids: HashMap<PathBuf, i64> = HashMap::new();
                    let mut hashes: HashMap<i64, HashMap<PathBuf, FileSha1>> = HashMap::new();
                    for worktree_root_path in worktree_root_paths {
                        let worktree_id =
                            db.find_or_create_worktree(worktree_root_path.as_ref())?;
                        worktree_ids.insert(worktree_root_path.to_path_buf(), worktree_id);
                        hashes.insert(worktree_id, db.get_file_hashes(worktree_id)?);
                    }
                    anyhow::Ok((db, hashes, worktree_ids))
                })
                .await?;

            let (paths_tx, paths_rx) =
                channel::unbounded::<(i64, PathBuf, String, Arc<Language>)>();
            let (indexed_files_tx, indexed_files_rx) = channel::unbounded::<(i64, IndexedFile)>();
            cx.background()
                .spawn({
                    let fs = fs.clone();
                    async move {
                        for worktree in worktrees.into_iter() {
                            let worktree_id = worktree_ids[&worktree.abs_path().to_path_buf()];
                            let file_hashes = &worktree_hashes[&worktree_id];
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
                                                    worktree_id,
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
                    // Initialize Database, creates database and tables if not exists
                    while let Ok((worktree_id, indexed_file)) = indexed_files_rx.recv().await {
                        db.insert_file(worktree_id, indexed_file).log_err();
                    }

                    // ALL OF THE BELOW IS FOR TESTING,
                    // This should be removed as we find and appropriate place for evaluate our search.

                    // let queries = vec![
                    //     "compute embeddings for all of the symbols in the codebase, and write them to a database",
                    //         "compute an outline view of all of the symbols in a buffer",
                    //         "scan a directory on the file system and load all of its children into an in-memory snapshot",
                    // ];
                    // let embeddings = embedding_provider.embed_batch(queries.clone()).await?;

                    // let t2 = Instant::now();
                    // let documents = db.get_documents().unwrap();
                    // let files = db.get_files().unwrap();
                    // println!("Retrieving all documents from Database: {}", t2.elapsed().as_millis());

                    // let t1 = Instant::now();
                    // let mut bfs = BruteForceSearch::load(&db).unwrap();
                    // println!("Loading BFS to Memory: {:?}", t1.elapsed().as_millis());
                    // for (idx, embed) in embeddings.into_iter().enumerate() {
                    //     let t0 = Instant::now();
                    //     println!("\nQuery: {:?}", queries[idx]);
                    //     let results = bfs.top_k_search(&embed, 5).await;
                    //     println!("Search Elapsed: {}", t0.elapsed().as_millis());
                    //     for (id, distance) in results {
                    //         println!("");
                    //         println!("   distance: {:?}", distance);
                    //         println!("   document: {:?}", documents[&id].name);
                    //         println!("   path:     {:?}", files[&documents[&id].file_id].relative_path);
                    //     }

                    // }

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
            anyhow::Ok(())
        })
    }

    pub fn search(
        &mut self,
        phrase: String,
        limit: usize,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<SearchResult>>> {
        let embedding_provider = self.embedding_provider.clone();
        let database_url = self.database_url.clone();
        cx.background().spawn(async move {
            let database = VectorDatabase::new(database_url.as_ref())?;

            let phrase_embedding = embedding_provider
                .embed_batch(vec![&phrase])
                .await?
                .into_iter()
                .next()
                .unwrap();

            let mut results = Vec::<(i64, f32)>::with_capacity(limit + 1);
            database.for_each_document(0, |id, embedding| {
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
            let documents = database.get_documents_by_ids(&ids)?;

            anyhow::Ok(
                documents
                    .into_iter()
                    .map(|(file_path, offset, name)| SearchResult {
                        name,
                        offset,
                        file_path,
                    })
                    .collect(),
            )
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
