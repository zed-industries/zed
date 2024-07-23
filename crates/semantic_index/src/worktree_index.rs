use crate::chunking::chunk_text;
use crate::embedding::*;
use crate::embedding_index::EmbeddingIndex;
use crate::summary_index::SummaryIndex;
use anyhow::{anyhow, Context as _, Result};
use collections::{Bound, HashSet};
use fs::Fs;
use futures::future::Shared;
use gpui::{
    AppContext, AsyncAppContext, Context, Model, ModelContext, Subscription, Task, WeakModel,
};
use language::LanguageRegistry;
use log;
use parking_lot::Mutex;
use project::{Entry, ProjectEntryId, UpdatedEntriesSet, Worktree};
use smol::channel;
use std::{
    future::Future,
    path::Path,
    sync::{Arc, Weak},
};
use util::ResultExt;
use worktree::Snapshot;

#[derive(Clone)]
pub enum WorktreeIndexHandle {
    Loading {
        index: Shared<Task<Result<Model<WorktreeIndex>, Arc<anyhow::Error>>>>,
    },
    Loaded {
        index: Model<WorktreeIndex>,
    },
}

struct ScanEntries {
    updated_entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
    deleted_entry_ranges: channel::Receiver<(Bound<String>, Bound<String>)>,
    task: Task<Result<()>>,
}

struct ProcessedFiles {
    chunked_files: channel::Receiver<ChunkedFile>,
    might_need_summary: channel::Receiver<UnsummarizedFile>,
    task: Task<Result<()>>,
}

pub struct WorktreeIndex {
    worktree: Model<Worktree>,
    db_connection: heed::Env,
    embedding_index: EmbeddingIndex,
    summary_index: SummaryIndex,
    fs: Arc<dyn Fs>,
    entry_ids_being_indexed: Arc<IndexingEntrySet>,
    _index_entries: Task<Result<()>>,
    _subscription: Subscription,
}

impl WorktreeIndex {
    pub fn load(
        worktree: Model<Worktree>,
        db_connection: heed::Env,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        status_tx: channel::Sender<()>,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        let worktree_abs_path = worktree.read(cx).abs_path();
        cx.spawn(|mut cx| async move {
            let (embedding_index, summary_index) = cx
                .background_executor()
                .spawn({
                    let db_connection = db_connection.clone();
                    async move {
                        let mut txn = db_connection.write_txn()?;
                        let embedding_index = {
                            let db_name = worktree_abs_path.to_string_lossy();
                            let db = db_connection.create_database(&mut txn, Some(&db_name))?;

                            EmbeddingIndex::new(
                                db_connection.clone(),
                                db,
                                language_registry,
                                embedding_provider,
                            )
                        };
                        let summary_index = {
                            let file_digest_db = {
                                let db_name =
                                // Prepend something that wouldn't be found at the beginning of an
                                // absolute path, so we don't get db key namespace conflicts with
                                // embeddings, which use the abs path as a key.
                                format!("digests-{}", worktree_abs_path.to_string_lossy());
                                db_connection.create_database(&mut txn, Some(&db_name))?
                            };
                            let summary_db = {
                                let db_name =
                                // Prepend something that wouldn't be found at the beginning of an
                                // absolute path, so we don't get db key namespace conflicts with
                                // embeddings, which use the abs path as a key.
                                format!("summaries-{}", worktree_abs_path.to_string_lossy());
                                db_connection.create_database(&mut txn, Some(&db_name))?
                            };
                            SummaryIndex::new(db_connection.clone(), file_digest_db, summary_db)
                        };
                        txn.commit()?;
                        anyhow::Ok((embedding_index, summary_index))
                    }
                })
                .await?;

            cx.new_model(|cx| {
                Self::new(
                    worktree,
                    db_connection,
                    embedding_index,
                    summary_index,
                    status_tx,
                    fs,
                    cx,
                )
            })
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        worktree: Model<Worktree>,
        db_connection: heed::Env,
        embedding_index: EmbeddingIndex,
        summary_index: SummaryIndex,
        status: channel::Sender<()>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let (updated_entries_tx, updated_entries_rx) = channel::unbounded();
        let _subscription = cx.subscribe(&worktree, move |_this, _worktree, event, _cx| {
            if let worktree::Event::UpdatedEntries(update) = event {
                dbg!(&update);
                log::debug!("Updating entries...");
                _ = updated_entries_tx.try_send(update.clone());
            } else {
                dbg!("non-update event");
            }
        });

        Self {
            db_connection,
            embedding_index,
            summary_index,
            worktree,
            fs,
            entry_ids_being_indexed: Arc::new(IndexingEntrySet::new(status)),
            _index_entries: cx.spawn(|this, cx| Self::index_entries(this, updated_entries_rx, cx)),
            _subscription,
        }
    }

    pub fn entry_ids_being_indexed(&self) -> &IndexingEntrySet {
        self.entry_ids_being_indexed.as_ref()
    }

    pub fn worktree(&self) -> &Model<Worktree> {
        &self.worktree
    }

    pub fn db_connection(&self) -> &heed::Env {
        &self.db_connection
    }

    pub fn embedding_index(&self) -> &EmbeddingIndex {
        &self.embedding_index
    }

    async fn index_entries(
        this: WeakModel<Self>,
        updated_entries: channel::Receiver<UpdatedEntriesSet>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let index = this.update(&mut cx, |this, cx| this.index_entries_changed_on_disk(cx))?;
        index.await.log_err();

        while let Ok(updated_entries) = updated_entries.recv().await {
            let index = this.update(&mut cx, |this, cx| {
                this.index_updated_entries(updated_entries, cx)
            })?;
            index.await.log_err();
        }

        Ok(())
    }

    fn index_entries_changed_on_disk(&self, cx: &AppContext) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx).snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_entries(worktree, cx);
        let processed = self.process_files(worktree_abs_path, scan.updated_entries, cx);
        let embed = Self::embed_files(self.embedding_provider.clone(), processed.chunked_files, cx);
        let might_need_summary = self.check_summary_cache(processed.might_need_summary, cx);
        let summarized = self.summarize_files(might_need_summary.files, cx);
        let persist_summaries = self.persist_summaries(summarized.files, cx);
        let persist_embeds = self.persist_embeddings(scan.deleted_entry_ranges, embed.files, cx);
        async move {
            futures::try_join!(
                scan.task,
                processed.task,
                embed.task,
                might_need_summary.task,
                summarized.task,
                persist_embeds,
                persist_summaries
            )?;
            Ok(())
        }
    }

    fn index_updated_entries(
        &self,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        log::debug!("index_updated_entries({:?})", &updated_entries);
        let worktree = self.worktree.read(cx).snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_updated_entries(worktree, updated_entries.clone(), cx);
        let processed = self.process_files(worktree_abs_path, scan.updated_entries, cx);
        let embed = Self::embed_files(self.embedding_provider.clone(), processed.chunked_files, cx);
        let might_need_summary = self.check_summary_cache(processed.might_need_summary, cx);
        let summarized = self.summarize_files(might_need_summary.files, cx);
        let persist_summaries = self.persist_summaries(summarized.files, cx);
        let persist_embeds = self.persist_embeddings(scan.deleted_entry_ranges, embed.files, cx);
        async move {
            futures::try_join!(
                scan.task,
                processed.task,
                embed.task,
                might_need_summary.task,
                summarized.task,
                persist_embeds,
                persist_summaries
            )?;
            Ok(())
        }
    }

    fn scan_updated_entries(
        &self,
        worktree: Snapshot,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) = channel::bounded(512);
        let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) = channel::bounded(128);
        let entries_being_indexed = self.entry_ids_being_indexed.clone();
        let task = cx.background_executor().spawn(async move {
            for (path, entry_id, status) in updated_entries.iter() {
                match status {
                    project::PathChange::Added
                    | project::PathChange::Updated
                    | project::PathChange::AddedOrUpdated => {
                        if let Some(entry) = worktree.entry_for_id(*entry_id) {
                            if entry.is_file() {
                                let handle = entries_being_indexed.insert(entry.id);
                                updated_entries_tx.send((entry.clone(), handle)).await?;
                            }
                        }
                    }
                    project::PathChange::Removed => {
                        let db_path = db_key_for_path(path);
                        deleted_entry_ranges_tx
                            .send((Bound::Included(db_path.clone()), Bound::Included(db_path)))
                            .await?;
                    }
                    project::PathChange::Loaded => {
                        // Do nothing.
                    }
                }
            }

            Ok(())
        });

        ScanEntries {
            updated_entries: updated_entries_rx,
            deleted_entry_ranges: deleted_entry_ranges_rx,
            task,
        }
    }

    fn process_files(
        &self,
        worktree_abs_path: Arc<Path>,
        entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
        cx: &AppContext,
    ) -> ProcessedFiles {
        let language_registry = self.language_registry.clone();
        let fs = self.fs.clone();
        let (chunked_files_tx, chunked_files_rx) = channel::bounded(2048);
        let (might_need_summary_tx, might_need_summary_rx) = channel::bounded(2048);
        let task = cx.spawn(|cx| async move {
            cx.background_executor()
                .scoped(|cx| {
                    for _ in 0..cx.num_cpus() {
                        cx.spawn(async {
                            while let Ok((entry, handle)) = entries.recv().await {
                                let entry_abs_path = worktree_abs_path.join(&entry.path);
                                let Some(text) = fs
                                    .load(&entry_abs_path)
                                    .await
                                    .with_context(|| {
                                        format!("failed to read path {entry_abs_path:?}")
                                    })
                                    .log_err()
                                else {
                                    continue;
                                };
                                let language = language_registry
                                    .language_for_file_path(&entry.path)
                                    .await
                                    .ok();
                                let chunked_file = ChunkedFile {
                                    chunks: chunk_text(&text, language.as_ref(), &entry.path),
                                    handle,
                                    path: entry.path,
                                    mtime: entry.mtime,
                                    text: text.clone(),
                                };

                                let content_hash = {
                                    let mut hasher = blake3::Hasher::new();

                                    hasher.update(text.as_bytes());

                                    hasher.finalize().to_hex().to_string()
                                };

                                let unsummarized_file = UnsummarizedFile {
                                    content_hash,
                                    contents: text,
                                };

                                match futures::future::try_join(
                                    might_need_summary_tx
                                        .send(unsummarized_file)
                                        .map_err(|error| anyhow!(error)),
                                    chunked_files_tx
                                        .send(chunked_file)
                                        .map_err(|error| anyhow!(error)),
                                )
                                .await
                                {
                                    Ok(_) => {}
                                    Err(err) => {
                                        log::error!("Error: {:?}", err);

                                        return;
                                    }
                                }
                            }
                        });
                    }
                })
                .await;
            Ok(())
        });

        ProcessedFiles {
            chunked_files: chunked_files_rx,
            might_need_summary: might_need_summary_rx,
            task,
        }
    }

    fn paths(&self, cx: &AppContext) -> Task<Result<Vec<Arc<Path>>>> {
        let connection = self.db_connection.clone();
        let db = self.embedding_db;
        cx.background_executor().spawn(async move {
            let tx = connection
                .read_txn()
                .context("failed to create read transaction")?;
            let result = db
                .iter(&tx)?
                .map(|entry| Ok(entry?.1.path.clone()))
                .collect::<Result<Vec<Arc<Path>>>>();
            drop(tx);
            result
        })
    }
}

/// The set of entries that are currently being indexed.
pub struct IndexingEntrySet {
    entry_ids: Mutex<HashSet<ProjectEntryId>>,
    tx: channel::Sender<()>,
}

/// When dropped, removes the entry from the set of entries that are being indexed.
#[derive(Clone)]
struct IndexingEntryHandle {
    entry_id: ProjectEntryId,
    set: Weak<IndexingEntrySet>,
}

impl IndexingEntrySet {
    fn new(tx: channel::Sender<()>) -> Self {
        Self {
            entry_ids: Default::default(),
            tx,
        }
    }

    fn insert(self: &Arc<Self>, entry_id: ProjectEntryId) -> IndexingEntryHandle {
        self.entry_ids.lock().insert(entry_id);
        self.tx.send_blocking(()).ok();
        IndexingEntryHandle {
            entry_id,
            set: Arc::downgrade(self),
        }
    }

    pub fn len(&self) -> usize {
        self.entry_ids.lock().len()
    }
}

impl Drop for IndexingEntryHandle {
    fn drop(&mut self) {
        if let Some(set) = self.set.upgrade() {
            set.tx.send_blocking(()).ok();
            set.entry_ids.lock().remove(&self.entry_id);
        }
    }
}
