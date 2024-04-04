mod chunking;

use anyhow::Result;
use chunking::{chunk_text, Chunk};
use collections::HashMap;
use fs::Fs;
use futures::StreamExt;
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{
    AppContext, AsyncAppContext, Context, EntityId, EventEmitter, Global, Model, ModelContext,
    Subscription, Task, WeakModel,
};
use heed::types::{SerdeBincode, Str};
use language::LanguageRegistry;
use project::{Entry, Project, Worktree};
use smol::channel;
use std::{
    path::Path,
    sync::Arc,
    time::{Duration, SystemTime},
};
use util::ResultExt;
use worktree::LocalSnapshot;

pub struct SemanticIndex {
    db_connection: heed::Env,
    project_indices: HashMap<WeakModel<Project>, Model<ProjectIndex>>,
}

impl Global for SemanticIndex {}

impl SemanticIndex {
    pub fn new(db_path: &Path, cx: &mut AppContext) -> Task<Result<Self>> {
        let db_path = db_path.to_path_buf();
        cx.spawn(|cx| async move {
            let db_connection = cx
                .background_executor()
                .spawn(async move {
                    heed::EnvOpenOptions::new()
                        .map_size(10 * 1024 * 1024)
                        .max_dbs(3000)
                        .open(db_path)
                })
                .await?;

            Ok(SemanticIndex {
                db_connection,
                project_indices: HashMap::default(),
            })
        })
    }

    pub fn project_index(
        &mut self,
        project: Model<Project>,
        cx: &mut AppContext,
    ) -> Model<ProjectIndex> {
        self.project_indices
            .entry(project.downgrade())
            .or_insert_with(|| {
                cx.new_model(|cx| ProjectIndex::new(project, self.db_connection.clone(), cx))
            })
            .clone()
    }
}

pub struct ProjectIndex {
    db_connection: heed::Env,
    project: WeakModel<Project>,
    worktree_indices: HashMap<EntityId, WorktreeIndexHandle>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    last_status: Status,
}

enum WorktreeIndexHandle {
    Loading {
        _task: Task<Result<()>>,
    },
    Loaded {
        index: Model<WorktreeIndex>,
        _subscription: Subscription,
    },
}

impl ProjectIndex {
    fn new(project: Model<Project>, db_connection: heed::Env, cx: &mut ModelContext<Self>) -> Self {
        let language_registry = project.read(cx).languages().clone();
        let fs = project.read(cx).fs().clone();
        let mut this = ProjectIndex {
            db_connection,
            project: project.downgrade(),
            worktree_indices: HashMap::default(),
            language_registry,
            fs,
            last_status: Status::Idle,
        };

        for worktree in project.read(cx).worktrees().collect::<Vec<_>>() {
            this.add_worktree(worktree, cx);
        }

        this
    }

    fn add_worktree(&mut self, worktree: Model<Worktree>, cx: &mut ModelContext<Self>) {
        if !worktree.read(cx).is_local() {
            return;
        }

        let worktree_entity_id = worktree.entity_id();
        let worktree_index = WorktreeIndex::load(
            worktree.clone(),
            self.db_connection.clone(),
            self.language_registry.clone(),
            self.fs.clone(),
            cx,
        );
        let load_worktree = cx.spawn(|this, mut cx| async move {
            if let Some(index) = worktree_index.await.log_err() {
                this.update(&mut cx, |this, cx| {
                    this.worktree_indices.insert(
                        worktree_entity_id,
                        WorktreeIndexHandle::Loaded {
                            _subscription: cx.observe(&index, |this, _, cx| this.update_status(cx)),
                            index,
                        },
                    );
                })?;
            } else {
                this.update(&mut cx, |this, cx| {
                    this.worktree_indices.remove(&worktree_entity_id)
                })?;
            }

            this.update(&mut cx, |this, cx| this.update_status(cx))
        });
        self.worktree_indices.insert(
            worktree_entity_id,
            WorktreeIndexHandle::Loading {
                _task: load_worktree,
            },
        );
        self.update_status(cx);
    }

    fn update_status(&mut self, cx: &mut ModelContext<Self>) {
        let mut status = Status::Idle;
        for index in self.worktree_indices.values() {
            match index {
                WorktreeIndexHandle::Loading { .. } => {
                    status = Status::Scanning;
                    break;
                }
                WorktreeIndexHandle::Loaded { index, .. } => {
                    if index.read(cx).status == Status::Scanning {
                        status = Status::Scanning;
                        break;
                    }
                }
            }
        }

        if status != self.last_status {
            self.last_status = status;
            cx.emit(status);
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Idle,
    Scanning,
}

impl EventEmitter<Status> for ProjectIndex {}

struct WorktreeIndex {
    worktree: Model<Worktree>,
    db_connection: heed::Env,
    db: heed::Database<Str, SerdeBincode<ChunkedFile>>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    status: Status,
    index_entries: Task<Result<()>>,
}

impl WorktreeIndex {
    pub fn load(
        worktree: Model<Worktree>,
        db_connection: heed::Env,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        let worktree_abs_path = worktree.read(cx).abs_path();
        cx.spawn(|mut cx| async move {
            let db = cx
                .background_executor()
                .spawn({
                    let db_connection = db_connection.clone();
                    async move {
                        let mut txn = db_connection.write_txn()?;
                        let db_name = worktree_abs_path.to_string_lossy();
                        db_connection.create_database(&mut txn, Some(&db_name))
                    }
                })
                .await?;
            cx.new_model(|cx| Self::new(worktree, db_connection, db, language_registry, fs, cx))
        })
    }

    fn new(
        worktree: Model<Worktree>,
        db_connection: heed::Env,
        db: heed::Database<Str, SerdeBincode<ChunkedFile>>,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            db_connection,
            db,
            worktree,
            language_registry,
            fs,
            status: Status::Idle,
            index_entries: cx.spawn(Self::index_entries),
        }
    }

    async fn index_entries(this: WeakModel<Self>, mut cx: AsyncAppContext) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.status = Status::Scanning;
            cx.notify();
        })?;

        let worktree = this.read_with(&cx, |this, cx| {
            this.worktree.read(cx).as_local().unwrap().snapshot()
        })?;
        let worktree_abs_path = worktree.abs_path().clone();
        let (entries, scan_entries) = Self::scan_entries(worktree, &mut cx);
        let (chunked_files, chunk_files) =
            Self::chunk_files(&this, worktree_abs_path, entries, &mut cx)?;
        let (embedded_files, embed_chunks) = Self::embed_files(&this, chunked_files, &mut cx);
        let save_embedded_files = Self::save_embedded_files(&this, embedded_files, &mut cx);

        futures::join!(scan_entries, chunk_files, embed_chunks, save_embedded_files);
        this.update(&mut cx, |this, cx| {
            this.status = Status::Idle;
            cx.notify();
        })?;

        Ok(())
    }

    fn scan_entries(
        worktree: LocalSnapshot,
        cx: &mut AsyncAppContext,
    ) -> (channel::Receiver<Entry>, Task<()>) {
        let (entries_tx, entries_rx) = channel::bounded(512);
        let scan_entries = cx.background_executor().spawn(async move {
            for entry in worktree.files(false, 0) {
                if entries_tx.send(entry.clone()).await.is_err() {
                    break;
                }
            }
        });
        (entries_rx, scan_entries)
    }

    fn chunk_files(
        this: &WeakModel<Self>,
        worktree_abs_path: Arc<Path>,
        entries: channel::Receiver<Entry>,
        cx: &mut AsyncAppContext,
    ) -> Result<(channel::Receiver<ChunkedFile>, Task<()>)> {
        let language_registry = this.read_with(cx, |this, _| this.language_registry.clone())?;
        let fs = this.read_with(cx, |this, _| this.fs.clone())?;
        let (chunked_files_tx, chunked_files_rx) = channel::bounded(2048);
        let chunk_files = cx.spawn(|cx| async move {
            cx.background_executor()
                .scoped(|cx| {
                    for _ in 0..cx.num_cpus() {
                        cx.spawn(async {
                            while let Ok(entry) = entries.recv().await {
                                let entry_abs_path = worktree_abs_path.join(&entry.path);
                                let Some(text) = fs.load(&entry_abs_path).await.log_err() else {
                                    continue;
                                };
                                let language = language_registry
                                    .language_for_file_path(&entry.path)
                                    .await
                                    .ok();
                                let grammar =
                                    language.as_ref().and_then(|language| language.grammar());
                                let chunked_file = ChunkedFile {
                                    worktree_root: worktree_abs_path.clone(),
                                    chunks: chunk_text(&text, grammar),
                                    entry,
                                    text,
                                };

                                if chunked_files_tx.send(chunked_file).await.is_err() {
                                    return;
                                }
                            }
                        });
                    }
                })
                .await;
        });
        Ok((chunked_files_rx, chunk_files))
    }

    fn embed_files(
        this: &WeakModel<Self>,
        chunked_files: channel::Receiver<ChunkedFile>,
        cx: &mut AsyncAppContext,
    ) -> (channel::Receiver<EmbeddedFile>, Task<()>) {
        let (embedded_files_tx, embedded_files_rx) = channel::bounded(512);
        let embed_chunks = cx.background_executor().spawn(async move {
            let mut chunked_file_batches =
                chunked_files.chunks_timeout(512, Duration::from_secs(2));
            while let Some(batch) = chunked_file_batches.next().await {
                // todo!("actually embed the batch")
                for chunked_file in batch {
                    let embedded_file = EmbeddedFile {
                        path: chunked_file.entry.path.clone(),
                        mtime: chunked_file.entry.mtime,
                        chunks: chunked_file
                            .chunks
                            .into_iter()
                            .map(|chunk| EmbeddedChunk {
                                chunk,
                                embedding: [0.0; 1536],
                            })
                            .collect(),
                    };
                    if embedded_files_tx.send(embedded_file).await.is_err() {
                        return;
                    }
                }
            }
        });
        (embedded_files_rx, embed_chunks)
    }

    fn save_embedded_files(
        this: &WeakModel<Self>,
        embedded_files: channel::Receiver<EmbeddedFile>,
        cx: &mut AsyncAppContext,
    ) -> Task<()> {
        todo!()
    }
}

struct ChunkedFile {
    worktree_root: Arc<Path>,
    entry: Entry,
    text: String,
    chunks: Vec<Chunk>,
}

struct EmbeddedFile {
    path: Arc<Path>,
    mtime: Option<SystemTime>,
    chunks: Vec<EmbeddedChunk>,
}

struct EmbeddedChunk {
    chunk: Chunk,
    embedding: [f32; 1536],
}
