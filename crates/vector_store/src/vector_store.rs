mod db;
use anyhow::Result;
use db::VectorDatabase;
use gpui::{AppContext, Entity, ModelContext, ModelHandle};
use language::LanguageRegistry;
use project::{Fs, Project};
use rand::Rng;
use smol::channel;
use std::{path::PathBuf, sync::Arc, time::Instant};
use util::ResultExt;
use workspace::WorkspaceCreated;

pub fn init(fs: Arc<dyn Fs>, language_registry: Arc<LanguageRegistry>, cx: &mut AppContext) {
    let vector_store = cx.add_model(|cx| VectorStore::new(fs, language_registry));

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
    language_registry: Arc<LanguageRegistry>,
}

impl VectorStore {
    fn new(fs: Arc<dyn Fs>, language_registry: Arc<LanguageRegistry>) -> Self {
        Self {
            fs,
            language_registry,
        }
    }

    async fn index_file(
        fs: &Arc<dyn Fs>,
        language_registry: &Arc<LanguageRegistry>,
        file_path: PathBuf,
    ) -> Result<IndexedFile> {
        // This is creating dummy documents to test the database writes.
        let mut documents = vec![];
        let mut rng = rand::thread_rng();
        let rand_num_of_documents: u8 = rng.gen_range(0..200);
        for _ in 0..rand_num_of_documents {
            let doc = Document {
                offset: 0,
                name: "test symbol".to_string(),
                embedding: vec![0.32 as f32; 768],
            };
            documents.push(doc);
        }

        return Ok(IndexedFile {
            path: file_path,
            sha1: "asdfasdfasdf".to_string(),
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

        cx.spawn(|this, cx| async move {
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

            cx.background()
                .scoped(|scope| {
                    for _ in 0..cx.background().num_cpus() {
                        scope.spawn(async {
                            while let Ok(file_path) = paths_rx.recv().await {
                                if let Some(indexed_file) =
                                    Self::index_file(&fs, &language_registry, file_path)
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
        })
        .detach();
    }
}

impl Entity for VectorStore {
    type Event = ();
}
