use crate::embedding::EmbeddingProvider;
use crate::embedding_index::EmbeddingIndex;
use crate::indexing::IndexingEntrySet;
use crate::summary_index::SummaryIndex;
use anyhow::Result;
use fs::Fs;
use futures::future::Shared;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, Subscription, Task, WeakEntity};
use language::LanguageRegistry;
use log;
use project::{UpdatedEntriesSet, Worktree};
use smol::channel;
use std::sync::Arc;
use util::ResultExt;

#[derive(Clone)]
pub enum WorktreeIndexHandle {
    Loading {
        index: Shared<Task<Result<Entity<WorktreeIndex>, Arc<anyhow::Error>>>>,
    },
    Loaded {
        index: Entity<WorktreeIndex>,
    },
}

pub struct WorktreeIndex {
    worktree: Entity<Worktree>,
    db_connection: heed::Env,
    embedding_index: EmbeddingIndex,
    summary_index: SummaryIndex,
    entry_ids_being_indexed: Arc<IndexingEntrySet>,
    _index_entries: Task<Result<()>>,
    _subscription: Subscription,
}

impl WorktreeIndex {
    pub fn load(
        worktree: Entity<Worktree>,
        db_connection: heed::Env,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        status_tx: channel::Sender<()>,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let worktree_for_index = worktree.clone();
        let worktree_for_summary = worktree.clone();
        let worktree_abs_path = worktree.read(cx).abs_path();
        let embedding_fs = Arc::clone(&fs);
        let summary_fs = fs;
        cx.spawn(async move |cx| {
            let entries_being_indexed = Arc::new(IndexingEntrySet::new(status_tx));
            let (embedding_index, summary_index) = cx
                .background_spawn({
                    let entries_being_indexed = Arc::clone(&entries_being_indexed);
                    let db_connection = db_connection.clone();
                    async move {
                        let mut txn = db_connection.write_txn()?;
                        let embedding_index = {
                            let db_name = worktree_abs_path.to_string_lossy();
                            let db = db_connection.create_database(&mut txn, Some(&db_name))?;

                            EmbeddingIndex::new(
                                worktree_for_index,
                                embedding_fs,
                                db_connection.clone(),
                                db,
                                language_registry,
                                embedding_provider,
                                Arc::clone(&entries_being_indexed),
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
                            SummaryIndex::new(
                                worktree_for_summary,
                                summary_fs,
                                db_connection.clone(),
                                file_digest_db,
                                summary_db,
                                Arc::clone(&entries_being_indexed),
                            )
                        };
                        txn.commit()?;
                        anyhow::Ok((embedding_index, summary_index))
                    }
                })
                .await?;

            cx.new(|cx| {
                Self::new(
                    worktree,
                    db_connection,
                    embedding_index,
                    summary_index,
                    entries_being_indexed,
                    cx,
                )
            })
        })
    }

    pub fn new(
        worktree: Entity<Worktree>,
        db_connection: heed::Env,
        embedding_index: EmbeddingIndex,
        summary_index: SummaryIndex,
        entry_ids_being_indexed: Arc<IndexingEntrySet>,
        cx: &mut Context<Self>,
    ) -> Self {
        let (updated_entries_tx, updated_entries_rx) = channel::unbounded();
        let _subscription = cx.subscribe(&worktree, move |_this, _worktree, event, _cx| {
            if let worktree::Event::UpdatedEntries(update) = event {
                log::debug!("Updating entries...");
                _ = updated_entries_tx.try_send(update.clone());
            }
        });

        Self {
            db_connection,
            embedding_index,
            summary_index,
            worktree,
            entry_ids_being_indexed,
            _index_entries: cx.spawn(async move |this, cx| {
                Self::index_entries(this, updated_entries_rx, cx).await
            }),
            _subscription,
        }
    }

    pub fn entry_ids_being_indexed(&self) -> &IndexingEntrySet {
        self.entry_ids_being_indexed.as_ref()
    }

    pub fn worktree(&self) -> &Entity<Worktree> {
        &self.worktree
    }

    pub fn db_connection(&self) -> &heed::Env {
        &self.db_connection
    }

    pub fn embedding_index(&self) -> &EmbeddingIndex {
        &self.embedding_index
    }

    pub fn summary_index(&self) -> &SummaryIndex {
        &self.summary_index
    }

    async fn index_entries(
        this: WeakEntity<Self>,
        updated_entries: channel::Receiver<UpdatedEntriesSet>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let index = this.update(cx, |this, cx| {
            futures::future::try_join(
                this.embedding_index.index_entries_changed_on_disk(cx),
                this.summary_index.index_entries_changed_on_disk(false, cx),
            )
        })?;
        index.await.log_err();

        while let Ok(updated_entries) = updated_entries.recv().await {
            let index = this.update(cx, |this, cx| {
                futures::future::try_join(
                    this.embedding_index
                        .index_updated_entries(updated_entries.clone(), cx),
                    this.summary_index
                        .index_updated_entries(updated_entries, false, cx),
                )
            })?;
            index.await.log_err();
        }

        Ok(())
    }

    #[cfg(test)]
    pub fn path_count(&self) -> Result<u64> {
        use anyhow::Context as _;

        let txn = self
            .db_connection
            .read_txn()
            .context("failed to create read transaction")?;
        Ok(self.embedding_index().db().len(&txn)?)
    }
}
