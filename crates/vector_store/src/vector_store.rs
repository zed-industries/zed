use anyhow::{anyhow, Result};
use gpui::{AppContext, Entity, ModelContext, ModelHandle};
use language::LanguageRegistry;
use project::{Fs, Project};
use smol::channel;
use std::{path::PathBuf, sync::Arc};
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

struct Document {
    offset: usize,
    name: String,
    embedding: Vec<f32>,
}

struct IndexedFile {
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
        eprintln!("indexing file {file_path:?}");
        Err(anyhow!("not implemented"))
        // todo!();
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
                    while let Ok(indexed_file) = indexed_files_rx.recv().await {
                        // write document to database
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
