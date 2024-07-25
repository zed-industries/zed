use anyhow::{anyhow, Context as _, Result};
use arrayvec::ArrayString;
use completion::CompletionProvider;
use fs::Fs;
use futures::{stream::StreamExt, TryFutureExt};
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{AppContext, Model, Task};
use heed::types::{SerdeBincode, Str};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, Role};
use log;
use project::{Entry, UpdatedEntriesSet, Worktree};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{
    future::Future,
    path::Path,
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use util::ResultExt;
use worktree::Snapshot;

use crate::indexing::{IndexingEntryHandle, IndexingEntrySet};

/// This model should be good for summarizing code - fast, low price, and good at outputting English.
///
/// It's called "preferred" because if the model isn't available (e.g. due to lacking the necessary API key),
/// we fall back on the global CompletionProvider's selected model.
const PREFERRED_SUMMARIZATION_MODEL: LanguageModel =
    LanguageModel::OpenAi(open_ai::Model::FourOmniMini);

#[derive(Debug, Serialize, Deserialize)]
struct UnsummarizedFile {
    // BLAKE3 hash of the source file's contents
    content_hash: String,
    // The source file's contents
    contents: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SummarizedFile {
    // BLAKE3 hash of the source file's contents
    content_hash: String,
    // The LLM's summary of the file's contents
    summary: String,
}

/// This is what blake3's to_hex() method returns - see https://docs.rs/blake3/1.5.3/src/blake3/lib.rs.html#246
type Blake3Digest = ArrayString<{ blake3::OUT_LEN * 2 }>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDigest {
    path: Arc<Path>,
    mtime: Option<SystemTime>,
    digest: Blake3Digest,
}

struct SummarizeFiles {
    files: channel::Receiver<SummarizedFile>,
    task: Task<Result<()>>,
}

struct NeedsSummary {
    files: channel::Receiver<UnsummarizedFile>,
    task: Task<Result<()>>,
}

pub struct SummaryIndex {
    worktree: Model<Worktree>,
    fs: Arc<dyn Fs>,
    db_connection: heed::Env,
    file_digest_db: heed::Database<Str, SerdeBincode<FileDigest>>, // Key: file path. Val: BLAKE3 digest of its contents.
    summary_db: heed::Database<Str, Str>, // Key: BLAKE3 digest of a file's contents. Val: LLM summary of those contents.
    entry_ids_being_indexed: Arc<IndexingEntrySet>,
}

struct ScanEntries {
    updated_entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
    task: Task<Result<()>>,
}

struct MightNeedSummaryFiles {
    files: channel::Receiver<UnsummarizedFile>,
    task: Task<Result<()>>,
}

impl SummaryIndex {
    pub fn new(
        worktree: Model<Worktree>,
        fs: Arc<dyn Fs>,
        db_connection: heed::Env,
        file_digest_db: heed::Database<Str, SerdeBincode<FileDigest>>,
        summary_db: heed::Database<Str, Str>,
        entry_ids_being_indexed: Arc<IndexingEntrySet>,
    ) -> Self {
        Self {
            worktree,
            fs,
            db_connection,
            file_digest_db,
            summary_db,
            entry_ids_being_indexed,
        }
    }

    pub fn index_entries_changed_on_disk(
        &self,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        let start = Instant::now();
        let worktree = self.worktree.read(cx).snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_entries(worktree, cx);
        let digest = self.digest_files(worktree_abs_path, scan.updated_entries, cx);
        let needs_summary = self.check_summary_cache(digest.files, cx);
        let summaries = self.summarize_files(needs_summary.files, cx);
        let persist = self.persist_summaries(summaries.files, cx);

        async move {
            futures::try_join!(
                scan.task,
                digest.task,
                needs_summary.task,
                summaries.task,
                persist
            )?;

            log::info!(
                "Summarizing everything that changed on disk took {:?}",
                start.elapsed()
            );

            Ok(())
        }
    }

    pub fn index_updated_entries(
        &self,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        let start = Instant::now();
        let worktree = self.worktree.read(cx).snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_updated_entries(worktree, updated_entries.clone(), cx);
        let digest = self.digest_files(worktree_abs_path, scan.updated_entries, cx);
        let needs_summary = self.check_summary_cache(digest.files, cx);
        let summaries = self.summarize_files(needs_summary.files, cx);
        let persist = self.persist_summaries(summaries.files, cx);

        async move {
            futures::try_join!(
                scan.task,
                digest.task,
                needs_summary.task,
                summaries.task,
                persist
            )?;

            log::info!("Summarizing updated entries took {:?}", start.elapsed());

            Ok(())
        }
    }

    fn check_summary_cache(
        &self,
        mut might_need_summary: channel::Receiver<UnsummarizedFile>,
        cx: &AppContext,
    ) -> NeedsSummary {
        let db_connection = self.db_connection.clone();
        let db = self.summary_db;
        let (needs_summary_tx, needs_summary_rx) = channel::bounded(512);
        let task = cx.background_executor().spawn(async move {
            while let Some(file) = might_need_summary.next().await {
                let tx = db_connection
                    .read_txn()
                    .context("Failed to create read transaction for checking which hashes are in summary cache")?;

                match db.get(&tx, &file.content_hash) {
                    Ok(opt_answer) => {
                        if opt_answer.is_none() {
                            // It's not in the summary cache db, so we need to summarize it.
                            log::debug!("{:?} was NOT in the db cache and needs to be resummarized.", &file.content_hash);
                            needs_summary_tx.send(file).await?;
                        } else {
                            log::debug!("{:?} was in the db cache and does not need to be resummarized.", &file.content_hash);
                        }
                    }
                    Err(err) => {
                        log::error!("Reading from the summaries database failed: {:?}", err);
                    }
                }
            }

            Ok(())
        });

        NeedsSummary {
            files: needs_summary_rx,
            task,
        }
    }

    fn scan_entries(&self, worktree: Snapshot, cx: &AppContext) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) = channel::bounded(512);
        let db_connection = self.db_connection.clone();
        let digest_db = self.file_digest_db;
        let entries_being_indexed = self.entry_ids_being_indexed.clone();
        let task = cx.background_executor().spawn(async move {
            let txn = db_connection
                .read_txn()
                .context("failed to create read transaction")?;
            // let mut db_entries = digest_db
            //     .iter(&txn)
            //     .context("failed to create iterator")?
            //     .move_between_keys()
            //     .peekable();

            for entry in worktree.files(false, 0) {
                let entry_db_key = db_key_for_path(&entry.path);

                match digest_db.get(&txn, &entry_db_key) {
                    Ok(opt_saved_digest) => {
                        // The file path is the same, but the mtime is different. Update it!
                        if entry.mtime != opt_saved_digest.and_then(|digest| digest.mtime) {
                            let handle = entries_being_indexed.insert(entry.id);
                            updated_entries_tx.send((entry.clone(), handle)).await?;
                        }
                    }
                    Err(err) => {
                        log::error!(
                            "Error trying to get file digest db entry {:?}: {:?}",
                            &entry_db_key,
                            err
                        );
                    }
                }
            }

            // TODO delete db entries for deleted files

            Ok(())
        });

        ScanEntries {
            updated_entries: updated_entries_rx,
            task,
        }
    }

    fn scan_updated_entries(
        &self,
        worktree: Snapshot,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) = channel::bounded(512);
        // let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) = channel::bounded(128);
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
                        let _db_path = db_key_for_path(path);
                        // TODO delete db entries for deleted files
                        // deleted_entry_ranges_tx
                        //     .send((Bound::Included(db_path.clone()), Bound::Included(db_path)))
                        //     .await?;
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
            // deleted_entry_ranges: deleted_entry_ranges_rx,
            task,
        }
    }

    fn digest_files(
        &self,
        worktree_abs_path: Arc<Path>,
        entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
        cx: &AppContext,
    ) -> MightNeedSummaryFiles {
        let fs = self.fs.clone();
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

                                let content_hash = {
                                    let mut hasher = blake3::Hasher::new();

                                    hasher.update(text.as_bytes());

                                    hasher.finalize().to_hex().to_string()
                                };

                                let unsummarized_file = UnsummarizedFile {
                                    content_hash,
                                    contents: text,
                                };

                                if let Err(err) = might_need_summary_tx
                                    .send(unsummarized_file)
                                    .map_err(|error| anyhow!(error))
                                    .await
                                {
                                    log::error!("Error: {:?}", err);

                                    return;
                                }
                            }
                        });
                    }
                })
                .await;
            Ok(())
        });

        MightNeedSummaryFiles {
            files: might_need_summary_rx,
            task,
        }
    }

    fn summarize_files(
        &self,
        mut unsummarized_files: channel::Receiver<UnsummarizedFile>,
        cx: &AppContext,
    ) -> SummarizeFiles {
        let (summarized_tx, summarized_rx) = channel::bounded(512);
        let task = cx.spawn(|cx| async move {
            while let Some(file) = unsummarized_files.next().await {
                log::debug!("Summarizing {:?}", file);
                let summary = cx.update(|cx| Self::summarize_code(&file.contents, cx))?;

                summarized_tx
                    .send(SummarizedFile {
                        content_hash: file.content_hash,
                        summary: summary.await?,
                    })
                    .await?
            }

            Ok(())
        });

        SummarizeFiles {
            files: summarized_rx,
            task,
        }
    }

    fn summarize_code(code: &str, cx: &AppContext) -> impl Future<Output = Result<String>> {
        let start = Instant::now();
        let provider = CompletionProvider::global(cx);
        let model = PREFERRED_SUMMARIZATION_MODEL;
        const PROMPT_BEFORE_CODE: &str = "Summarize this code in 3 sentences, using no newlines or bullet points in the summary:";
        let prompt = format!("{PROMPT_BEFORE_CODE}\n{code}");

        log::debug!(
            "Summarizing code by sending this prompt to {:?}: {:?}",
            &model,
            &prompt
        );

        let request = LanguageModelRequest {
            model,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: prompt,
            }],
            stop: Vec::new(),
            temperature: 1.0,
        };

        let response = provider.stream_completion_bg(request, cx);

        cx.background_executor().spawn(async move {
            let mut chunks = response.await?;
            let mut answer = String::new();

            while let Some(chunk) = chunks.next().await {
                answer.push_str(chunk?.as_str());
            }

            log::info!("Code summarization took {:?}", start.elapsed());
            Ok(answer)
        })
    }

    fn persist_summaries(
        &self,
        summaries: channel::Receiver<SummarizedFile>,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let db_connection = self.db_connection.clone();
        let db = self.summary_db;
        cx.background_executor().spawn(async move {
            let mut summaries = summaries.chunks_timeout(4096, Duration::from_secs(2));
            while let Some(summaries) = summaries.next().await {
                let mut txn = db_connection.write_txn()?;
                for file in &summaries {
                    log::debug!(
                        "Saving {} bytes of summary for content hash {:?}",
                        file.summary.len(),
                        file.content_hash
                    );
                    db.put(&mut txn, &file.content_hash, &file.summary)?;
                }
                txn.commit()?;

                drop(summaries);
                log::debug!("committed summaries");
            }

            Ok(())
        })
    }
}

fn db_key_for_path(path: &Arc<Path>) -> String {
    path.to_string_lossy().replace('/', "\0")
}
