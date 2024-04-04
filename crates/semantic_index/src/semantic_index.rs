mod chunking;

use anyhow::{anyhow, Context as _, Result};
use chunking::{chunk_text, Chunk};
use client::Client;
use collections::HashMap;
use fs::Fs;
use futures::StreamExt;
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{AppContext, Context, EventEmitter, Global, Model, ModelContext, Task, WeakModel};
use language::{LanguageRegistry, Tree, PARSER};
use project::{Entry, Project, Worktree};
use sha2::{Digest, Sha256};
use smol::channel;
use std::{cmp, ops::Range, path::Path, sync::Arc, time::Duration};
use util::ResultExt;
use worktree::LocalSnapshot;

pub struct SemanticIndex {
    db: lmdb::Database,
    project_indices: HashMap<WeakModel<Project>, Model<ProjectIndex>>,
}

impl Global for SemanticIndex {}

impl SemanticIndex {
    pub fn new(db_path: &Path) -> Result<Self> {
        let env = lmdb::Environment::new()
            .open(db_path)
            .context("failed to open environment")?;
        let db = env
            .create_db(None, lmdb::DatabaseFlags::empty())
            .context("failed to create db")?;

        Ok(SemanticIndex {
            db,
            project_indices: HashMap::default(),
        })
    }

    pub fn project_index(
        &mut self,
        project: Model<Project>,
        cx: &mut AppContext,
    ) -> Model<ProjectIndex> {
        self.project_indices
            .entry(project.downgrade())
            .or_insert_with(|| cx.new_model(|cx| ProjectIndex::new(project, self.db.clone(), cx)))
            .clone()
    }
}

pub struct ProjectIndex {
    db: lmdb::Database,
    project: WeakModel<Project>,
    worktree_scans: HashMap<WeakModel<Worktree>, Task<()>>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
}

impl ProjectIndex {
    fn new(project: Model<Project>, db: lmdb::Database, cx: &mut ModelContext<Self>) -> Self {
        let language_registry = project.read(cx).languages().clone();
        let fs = project.read(cx).fs().clone();
        let client = project.read(cx).client();
        let mut this = ProjectIndex {
            db,
            project: project.downgrade(),
            worktree_scans: HashMap::default(),
            client,
            language_registry,
            fs,
        };

        for worktree in project.read(cx).worktrees().collect::<Vec<_>>() {
            this.add_worktree(worktree, cx);
        }

        this
    }

    fn add_worktree(&mut self, worktree: Model<Worktree>, cx: &mut ModelContext<Self>) {
        let Some(local_worktree) = worktree.read(cx).as_local() else {
            return;
        };
        let local_worktree = local_worktree.snapshot();
        let worktree_abs_path = local_worktree.abs_path().clone();

        let (entries_rx, scan_entries) = self.scan_entries(local_worktree, cx);
        let (chunked_files, chunk_entries) = self.chunk_files(worktree_abs_path, entries_rx, cx);
        let embed_chunks = self.embed_chunks(chunked_files, cx);

        if self.worktree_scans.is_empty() {
            cx.emit(StatusEvent::Scanning);
        }

        let worktree = worktree.downgrade();
        let scan = cx.spawn(|this, mut cx| {
            let worktree = worktree.clone();
            async move {
                futures::join!(scan_entries, chunk_entries, embed_chunks);
                this.update(&mut cx, |this, cx| {
                    this.worktree_scans.remove(&worktree);
                    if this.worktree_scans.is_empty() {
                        cx.emit(StatusEvent::Idle);
                    }
                })
                .ok();
            }
        });
        self.worktree_scans.insert(worktree, scan);
    }

    fn scan_entries(
        &mut self,
        worktree: LocalSnapshot,
        cx: &mut ModelContext<Self>,
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
        &mut self,
        worktree_abs_path: Arc<Path>,
        entries: channel::Receiver<Entry>,
        cx: &mut ModelContext<Self>,
    ) -> (channel::Receiver<ChunkedFile>, Task<()>) {
        let language_registry = self.language_registry.clone();
        let fs = self.fs.clone();
        let (chunked_files_tx, chunked_files_rx) = channel::bounded(2048);
        let chunk_files = cx.spawn(|_this, cx| async move {
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
                                    break;
                                }
                            }
                        });
                    }
                })
                .await;
        });
        (chunked_files_rx, chunk_files)
    }

    fn embed_chunks(
        &mut self,
        chunked_files: channel::Receiver<ChunkedFile>,
        cx: &mut ModelContext<ProjectIndex>,
    ) -> Task<()> {
        let http_client = self.client.http_client();
        cx.spawn(|this, cx| async move {
            let mut chunked_file_batches =
                chunked_files.chunks_timeout(512, Duration::from_secs(2));
            while let Some(batch) = chunked_file_batches.next().await {
                dbg!(batch.len());
            }
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusEvent {
    Idle,
    Scanning,
}

impl EventEmitter<StatusEvent> for ProjectIndex {}

struct ChunkedFile {
    worktree_root: Arc<Path>,
    entry: Entry,
    text: String,
    chunks: Vec<Chunk>,
}
