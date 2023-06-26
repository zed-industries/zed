mod db;
mod embedding;
mod parsing;
mod search;

#[cfg(test)]
mod vector_store_tests;

use anyhow::{anyhow, Result};
use db::{VectorDatabase, VECTOR_DB_URL};
use embedding::{DummyEmbeddings, EmbeddingProvider, OpenAIEmbeddings};
use gpui::{AppContext, Entity, ModelContext, ModelHandle, Task};
use language::LanguageRegistry;
use parsing::Document;
use project::{Fs, Project};
use search::{BruteForceSearch, VectorSearch};
use smol::channel;
use std::{cmp::Ordering, path::PathBuf, sync::Arc, time::Instant};
use tree_sitter::{Parser, QueryCursor};
use util::{http::HttpClient, ResultExt, TryFutureExt};
use workspace::WorkspaceCreated;

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
                        store.add_project(project, cx);
                    });
                }
            }
        }
    })
    .detach();
}

#[derive(Debug)]
pub struct IndexedFile {
    path: PathBuf,
    sha1: String,
    documents: Vec<Document>,
}

// struct SearchResult {
//     path: PathBuf,
//     offset: usize,
//     name: String,
//     distance: f32,
// }
struct VectorStore {
    fs: Arc<dyn Fs>,
    database_url: Arc<str>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    language_registry: Arc<LanguageRegistry>,
}

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
        language_registry: &Arc<LanguageRegistry>,
        file_path: PathBuf,
        content: String,
    ) -> Result<IndexedFile> {
        dbg!(&file_path, &content);

        let language = language_registry
            .language_for_file(&file_path, None)
            .await?;

        if language.name().as_ref() != "Rust" {
            Err(anyhow!("unsupported language"))?;
        }

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

        return Ok(IndexedFile {
            path: file_path,
            sha1: String::new(),
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
            .map(|worktree| worktree.read(cx).as_local().unwrap().scan_complete())
            .collect::<Vec<_>>();

        let fs = self.fs.clone();
        let language_registry = self.language_registry.clone();
        let embedding_provider = self.embedding_provider.clone();
        let database_url = self.database_url.clone();

        cx.spawn(|_, cx| async move {
            futures::future::join_all(worktree_scans_complete).await;

            let worktrees = project.read_with(&cx, |project, cx| {
                project
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).snapshot())
                    .collect::<Vec<_>>()
            });

            let db = VectorDatabase::new(&database_url)?;
            let worktree_root_paths = worktrees
                .iter()
                .map(|worktree| worktree.abs_path().clone())
                .collect::<Vec<_>>();
            let (db, file_hashes) = cx
                .background()
                .spawn(async move {
                    let mut hashes = Vec::new();
                    for worktree_root_path in worktree_root_paths {
                        let worktree_id =
                            db.find_or_create_worktree(worktree_root_path.as_ref())?;
                        hashes.push((worktree_id, db.get_file_hashes(worktree_id)?));
                    }
                    anyhow::Ok((db, hashes))
                })
                .await?;

            let (paths_tx, paths_rx) = channel::unbounded::<(i64, PathBuf, String)>();
            let (indexed_files_tx, indexed_files_rx) = channel::unbounded::<IndexedFile>();
            cx.background()
                .spawn({
                    let fs = fs.clone();
                    async move {
                        for worktree in worktrees.into_iter() {
                            for file in worktree.files(false, 0) {
                                let absolute_path = worktree.absolutize(&file.path);
                                dbg!(&absolute_path);
                                if let Some(content) = fs.load(&absolute_path).await.log_err() {
                                    dbg!(&content);
                                    paths_tx.try_send((0, absolute_path, content)).unwrap();
                                }
                            }
                        }
                    }
                })
                .detach();

            let db_write_task = cx.background().spawn(
                async move {
                    // Initialize Database, creates database and tables if not exists
                    while let Ok(indexed_file) = indexed_files_rx.recv().await {
                        db.insert_file(indexed_file).log_err();
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

            let provider = DummyEmbeddings {};
            // let provider = OpenAIEmbeddings { client };

            cx.background()
                .scoped(|scope| {
                    for _ in 0..cx.background().num_cpus() {
                        scope.spawn(async {
                            let mut parser = Parser::new();
                            let mut cursor = QueryCursor::new();
                            while let Ok((worktree_id, file_path, content)) = paths_rx.recv().await
                            {
                                if let Some(indexed_file) = Self::index_file(
                                    &mut cursor,
                                    &mut parser,
                                    &provider,
                                    &language_registry,
                                    file_path,
                                    content,
                                )
                                .await
                                .log_err()
                                {
                                    indexed_files_tx.try_send(indexed_file).unwrap();
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
        cx.spawn(|this, cx| async move {
            let database = VectorDatabase::new(database_url.as_ref())?;

            // let embedding = embedding_provider.embed_batch(vec![&phrase]).await?;
            //
            let mut results = Vec::<(i64, f32)>::with_capacity(limit + 1);

            database.for_each_document(0, |id, embedding| {
                dbg!(id, &embedding);

                let similarity = dot(&embedding.0, &embedding.0);
                let ix = match results.binary_search_by(|(_, s)| {
                    s.partial_cmp(&similarity).unwrap_or(Ordering::Equal)
                }) {
                    Ok(ix) => ix,
                    Err(ix) => ix,
                };

                results.insert(ix, (id, similarity));
                results.truncate(limit);
            })?;

            dbg!(&results);

            let ids = results.into_iter().map(|(id, _)| id).collect::<Vec<_>>();
            // let documents = database.get_documents_by_ids(ids)?;

            // let search_provider = cx
            //     .background()
            //     .spawn(async move { BruteForceSearch::load(&database) })
            //     .await?;

            // let results = search_provider.top_k_search(&embedding, limit))

            anyhow::Ok(vec![])
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
