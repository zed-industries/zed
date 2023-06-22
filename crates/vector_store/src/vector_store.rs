mod db;
mod embedding;

use anyhow::{anyhow, Result};
use db::VectorDatabase;
use embedding::{EmbeddingProvider, OpenAIEmbeddings};
use gpui::{AppContext, Entity, ModelContext, ModelHandle};
use language::LanguageRegistry;
use project::{Fs, Project};
use smol::channel;
use std::{path::PathBuf, sync::Arc, time::Instant};
use tree_sitter::{Parser, QueryCursor};
use util::{http::HttpClient, ResultExt};
use workspace::WorkspaceCreated;

pub fn init(
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AppContext,
) {
    let vector_store = cx.add_model(|cx| VectorStore::new(fs, http_client, language_registry));

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

#[derive(Debug, sqlx::FromRow)]
struct Document {
    offset: usize,
    name: String,
    embedding: Vec<f32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct IndexedFile {
    path: PathBuf,
    sha1: String,
    documents: Vec<Document>,
}

struct SearchResult {
    path: PathBuf,
    offset: usize,
    name: String,
    distance: f32,
}

struct VectorStore {
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
}

impl VectorStore {
    fn new(
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            fs,
            http_client,
            language_registry,
        }
    }

    async fn index_file(
        cursor: &mut QueryCursor,
        parser: &mut Parser,
        embedding_provider: &dyn EmbeddingProvider,
        fs: &Arc<dyn Fs>,
        language_registry: &Arc<LanguageRegistry>,
        file_path: PathBuf,
    ) -> Result<IndexedFile> {
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

        let content = fs.load(&file_path).await?;
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

    fn add_project(&mut self, project: ModelHandle<Project>, cx: &mut ModelContext<Self>) {
        let worktree_scans_complete = project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).as_local().unwrap().scan_complete())
            .collect::<Vec<_>>();

        let fs = self.fs.clone();
        let language_registry = self.language_registry.clone();
        let client = self.http_client.clone();

        cx.spawn(|_, cx| async move {
            futures::future::join_all(worktree_scans_complete).await;

            let worktrees = project.read_with(&cx, |project, cx| {
                project
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).snapshot())
                    .collect::<Vec<_>>()
            });

            let (paths_tx, paths_rx) = channel::unbounded::<PathBuf>();
            let (indexed_files_tx, indexed_files_rx) = channel::unbounded::<IndexedFile>();
            cx.background()
                .spawn(async move {
                    for worktree in worktrees {
                        for file in worktree.files(false, 0) {
                            paths_tx.try_send(worktree.absolutize(&file.path)).unwrap();
                        }
                    }
                })
                .detach();

            cx.background()
                .spawn(async move {
                    // Initialize Database, creates database and tables if not exists
                    VectorDatabase::initialize_database().await.log_err();
                    while let Ok(indexed_file) = indexed_files_rx.recv().await {
                        VectorDatabase::insert_file(indexed_file).await.log_err();
                    }
                })
                .detach();

            let provider = OpenAIEmbeddings { client };

            let t0 = Instant::now();

            cx.background()
                .scoped(|scope| {
                    for _ in 0..cx.background().num_cpus() {
                        scope.spawn(async {
                            let mut parser = Parser::new();
                            let mut cursor = QueryCursor::new();
                            while let Ok(file_path) = paths_rx.recv().await {
                                if let Some(indexed_file) = Self::index_file(
                                    &mut cursor,
                                    &mut parser,
                                    &provider,
                                    &fs,
                                    &language_registry,
                                    file_path,
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

            let duration = t0.elapsed();
            log::info!("indexed project in {duration:?}");
        })
        .detach();
    }
}

impl Entity for VectorStore {
    type Event = ();
}
