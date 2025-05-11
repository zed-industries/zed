use anyhow::{Context as _, Result, anyhow};
use arrayvec::ArrayString;
use fs::{Fs, MTime};
use futures::{TryFutureExt, stream::StreamExt};
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{App, AppContext as _, Entity, Task};
use heed::{
    RoTxn,
    types::{SerdeBincode, Str},
};
use language_model::{
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, Role,
};
use log;
use parking_lot::Mutex;
use project::{Entry, UpdatedEntriesSet, Worktree};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{
    future::Future,
    path::Path,
    pin::pin,
    sync::Arc,
    time::{Duration, Instant},
};
use util::ResultExt;
use worktree::Snapshot;

use crate::{indexing::IndexingEntrySet, summary_backlog::SummaryBacklog};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSummary {
    pub filename: String,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnsummarizedFile {
    // Path to the file on disk
    path: Arc<Path>,
    // The mtime of the file on disk
    mtime: Option<MTime>,
    // BLAKE3 hash of the source file's contents
    digest: Blake3Digest,
    // The source file's contents
    contents: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SummarizedFile {
    // Path to the file on disk
    path: String,
    // The mtime of the file on disk
    mtime: Option<MTime>,
    // BLAKE3 hash of the source file's contents
    digest: Blake3Digest,
    // The LLM's summary of the file's contents
    summary: String,
}

/// This is what blake3's to_hex() method returns - see https://docs.rs/blake3/1.5.3/src/blake3/lib.rs.html#246
pub type Blake3Digest = ArrayString<{ blake3::OUT_LEN * 2 }>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDigest {
    pub mtime: Option<MTime>,
    pub digest: Blake3Digest,
}

struct NeedsSummary {
    files: channel::Receiver<UnsummarizedFile>,
    task: Task<Result<()>>,
}

struct SummarizeFiles {
    files: channel::Receiver<SummarizedFile>,
    task: Task<Result<()>>,
}

pub struct SummaryIndex {
    worktree: Entity<Worktree>,
    fs: Arc<dyn Fs>,
    db_connection: heed::Env,
    file_digest_db: heed::Database<Str, SerdeBincode<FileDigest>>, // Key: file path. Val: BLAKE3 digest of its contents.
    summary_db: heed::Database<SerdeBincode<Blake3Digest>, Str>, // Key: BLAKE3 digest of a file's contents. Val: LLM summary of those contents.
    backlog: Arc<Mutex<SummaryBacklog>>,
    _entry_ids_being_indexed: Arc<IndexingEntrySet>, // TODO can this be removed?
}

struct Backlogged {
    paths_to_digest: channel::Receiver<Vec<(Arc<Path>, Option<MTime>)>>,
    task: Task<Result<()>>,
}

struct MightNeedSummaryFiles {
    files: channel::Receiver<UnsummarizedFile>,
    task: Task<Result<()>>,
}

impl SummaryIndex {
    pub fn new(
        worktree: Entity<Worktree>,
        fs: Arc<dyn Fs>,
        db_connection: heed::Env,
        file_digest_db: heed::Database<Str, SerdeBincode<FileDigest>>,
        summary_db: heed::Database<SerdeBincode<Blake3Digest>, Str>,
        _entry_ids_being_indexed: Arc<IndexingEntrySet>,
    ) -> Self {
        Self {
            worktree,
            fs,
            db_connection,
            file_digest_db,
            summary_db,
            _entry_ids_being_indexed,
            backlog: Default::default(),
        }
    }

    pub fn file_digest_db(&self) -> heed::Database<Str, SerdeBincode<FileDigest>> {
        self.file_digest_db
    }

    pub fn summary_db(&self) -> heed::Database<SerdeBincode<Blake3Digest>, Str> {
        self.summary_db
    }

    pub fn index_entries_changed_on_disk(
        &self,
        is_auto_available: bool,
        cx: &App,
    ) -> impl Future<Output = Result<()>> + use<> {
        let start = Instant::now();
        let backlogged;
        let digest;
        let needs_summary;
        let summaries;
        let persist;

        if is_auto_available {
            let worktree = self.worktree.read(cx).snapshot();
            let worktree_abs_path = worktree.abs_path().clone();

            backlogged = self.scan_entries(worktree, cx);
            digest = self.digest_files(backlogged.paths_to_digest, worktree_abs_path, cx);
            needs_summary = self.check_summary_cache(digest.files, cx);
            summaries = self.summarize_files(needs_summary.files, cx);
            persist = self.persist_summaries(summaries.files, cx);
        } else {
            // This feature is only staff-shipped, so make the rest of these no-ops.
            backlogged = Backlogged {
                paths_to_digest: channel::unbounded().1,
                task: Task::ready(Ok(())),
            };
            digest = MightNeedSummaryFiles {
                files: channel::unbounded().1,
                task: Task::ready(Ok(())),
            };
            needs_summary = NeedsSummary {
                files: channel::unbounded().1,
                task: Task::ready(Ok(())),
            };
            summaries = SummarizeFiles {
                files: channel::unbounded().1,
                task: Task::ready(Ok(())),
            };
            persist = Task::ready(Ok(()));
        }

        async move {
            futures::try_join!(
                backlogged.task,
                digest.task,
                needs_summary.task,
                summaries.task,
                persist
            )?;

            if is_auto_available {
                log::info!(
                    "Summarizing everything that changed on disk took {:?}",
                    start.elapsed()
                );
            }

            Ok(())
        }
    }

    pub fn index_updated_entries(
        &mut self,
        updated_entries: UpdatedEntriesSet,
        is_auto_available: bool,
        cx: &App,
    ) -> impl Future<Output = Result<()>> + use<> {
        let start = Instant::now();
        let backlogged;
        let digest;
        let needs_summary;
        let summaries;
        let persist;

        if is_auto_available {
            let worktree = self.worktree.read(cx).snapshot();
            let worktree_abs_path = worktree.abs_path().clone();

            backlogged = self.scan_updated_entries(worktree, updated_entries.clone(), cx);
            digest = self.digest_files(backlogged.paths_to_digest, worktree_abs_path, cx);
            needs_summary = self.check_summary_cache(digest.files, cx);
            summaries = self.summarize_files(needs_summary.files, cx);
            persist = self.persist_summaries(summaries.files, cx);
        } else {
            // This feature is only staff-shipped, so make the rest of these no-ops.
            backlogged = Backlogged {
                paths_to_digest: channel::unbounded().1,
                task: Task::ready(Ok(())),
            };
            digest = MightNeedSummaryFiles {
                files: channel::unbounded().1,
                task: Task::ready(Ok(())),
            };
            needs_summary = NeedsSummary {
                files: channel::unbounded().1,
                task: Task::ready(Ok(())),
            };
            summaries = SummarizeFiles {
                files: channel::unbounded().1,
                task: Task::ready(Ok(())),
            };
            persist = Task::ready(Ok(()));
        }

        async move {
            futures::try_join!(
                backlogged.task,
                digest.task,
                needs_summary.task,
                summaries.task,
                persist
            )?;

            log::debug!("Summarizing updated entries took {:?}", start.elapsed());

            Ok(())
        }
    }

    fn check_summary_cache(
        &self,
        might_need_summary: channel::Receiver<UnsummarizedFile>,
        cx: &App,
    ) -> NeedsSummary {
        let db_connection = self.db_connection.clone();
        let db = self.summary_db;
        let (needs_summary_tx, needs_summary_rx) = channel::bounded(512);
        let task = cx.background_spawn(async move {
            let mut might_need_summary = pin!(might_need_summary);
            while let Some(file) = might_need_summary.next().await {
                let tx = db_connection
                    .read_txn()
                    .context("Failed to create read transaction for checking which hashes are in summary cache")?;

                match db.get(&tx, &file.digest) {
                    Ok(opt_answer) => {
                        if opt_answer.is_none() {
                            // It's not in the summary cache db, so we need to summarize it.
                            log::debug!("File {:?} (digest {:?}) was NOT in the db cache and needs to be resummarized.", file.path.display(), &file.digest);
                            needs_summary_tx.send(file).await?;
                        } else {
                            log::debug!("File {:?} (digest {:?}) was in the db cache and does not need to be resummarized.", file.path.display(), &file.digest);
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

    fn scan_entries(&self, worktree: Snapshot, cx: &App) -> Backlogged {
        let (tx, rx) = channel::bounded(512);
        let db_connection = self.db_connection.clone();
        let digest_db = self.file_digest_db;
        let backlog = Arc::clone(&self.backlog);
        let task = cx.background_spawn(async move {
            let txn = db_connection
                .read_txn()
                .context("failed to create read transaction")?;

            for entry in worktree.files(false, 0) {
                let needs_summary =
                    Self::add_to_backlog(Arc::clone(&backlog), digest_db, &txn, entry);

                if !needs_summary.is_empty() {
                    tx.send(needs_summary).await?;
                }
            }

            // TODO delete db entries for deleted files

            Ok(())
        });

        Backlogged {
            paths_to_digest: rx,
            task,
        }
    }

    fn add_to_backlog(
        backlog: Arc<Mutex<SummaryBacklog>>,
        digest_db: heed::Database<Str, SerdeBincode<FileDigest>>,
        txn: &RoTxn<'_>,
        entry: &Entry,
    ) -> Vec<(Arc<Path>, Option<MTime>)> {
        let entry_db_key = db_key_for_path(&entry.path);

        match digest_db.get(&txn, &entry_db_key) {
            Ok(opt_saved_digest) => {
                // The file path is the same, but the mtime is different. (Or there was no mtime.)
                // It needs updating, so add it to the backlog! Then, if the backlog is full, drain it and summarize its contents.
                if entry.mtime != opt_saved_digest.and_then(|digest| digest.mtime) {
                    let mut backlog = backlog.lock();

                    log::info!(
                        "Inserting {:?} ({:?} bytes) into backlog",
                        &entry.path,
                        entry.size,
                    );
                    backlog.insert(Arc::clone(&entry.path), entry.size, entry.mtime);

                    if backlog.needs_drain() {
                        log::info!("Draining summary backlog...");
                        return backlog.drain().collect();
                    }
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

        Vec::new()
    }

    fn scan_updated_entries(
        &self,
        worktree: Snapshot,
        updated_entries: UpdatedEntriesSet,
        cx: &App,
    ) -> Backlogged {
        log::info!("Scanning for updated entries that might need summarization...");
        let (tx, rx) = channel::bounded(512);
        // let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) = channel::bounded(128);
        let db_connection = self.db_connection.clone();
        let digest_db = self.file_digest_db;
        let backlog = Arc::clone(&self.backlog);
        let task = cx.background_spawn(async move {
            let txn = db_connection
                .read_txn()
                .context("failed to create read transaction")?;

            for (path, entry_id, status) in updated_entries.iter() {
                match status {
                    project::PathChange::Loaded
                    | project::PathChange::Added
                    | project::PathChange::Updated
                    | project::PathChange::AddedOrUpdated => {
                        if let Some(entry) = worktree.entry_for_id(*entry_id) {
                            if entry.is_file() {
                                let needs_summary = Self::add_to_backlog(
                                    Arc::clone(&backlog),
                                    digest_db,
                                    &txn,
                                    entry,
                                );

                                if !needs_summary.is_empty() {
                                    tx.send(needs_summary).await?;
                                }
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
                }
            }

            Ok(())
        });

        Backlogged {
            paths_to_digest: rx,
            // deleted_entry_ranges: deleted_entry_ranges_rx,
            task,
        }
    }

    fn digest_files(
        &self,
        paths: channel::Receiver<Vec<(Arc<Path>, Option<MTime>)>>,
        worktree_abs_path: Arc<Path>,
        cx: &App,
    ) -> MightNeedSummaryFiles {
        let fs = self.fs.clone();
        let (rx, tx) = channel::bounded(2048);
        let task = cx.spawn(async move |cx| {
            cx.background_executor()
                .scoped(|cx| {
                    for _ in 0..cx.num_cpus() {
                        cx.spawn(async {
                            while let Ok(pairs) = paths.recv().await {
                                // Note: we could process all these files concurrently if desired. Might or might not speed things up.
                                for (path, mtime) in pairs {
                                    let entry_abs_path = worktree_abs_path.join(&path);

                                    // Load the file's contents and compute its hash digest.
                                    let unsummarized_file = {
                                        let Some(contents) = fs
                                            .load(&entry_abs_path)
                                            .await
                                            .with_context(|| {
                                                format!("failed to read path {entry_abs_path:?}")
                                            })
                                            .log_err()
                                        else {
                                            continue;
                                        };

                                        let digest = {
                                            let mut hasher = blake3::Hasher::new();
                                            // Incorporate both the (relative) file path as well as the contents of the file into the hash.
                                            // This is because in some languages and frameworks, identical files can do different things
                                            // depending on their paths (e.g. Rails controllers). It's also why we send the path to the model.
                                            hasher.update(path.display().to_string().as_bytes());
                                            hasher.update(contents.as_bytes());
                                            hasher.finalize().to_hex()
                                        };

                                        UnsummarizedFile {
                                            digest,
                                            contents,
                                            path,
                                            mtime,
                                        }
                                    };

                                    if let Err(err) = rx
                                        .send(unsummarized_file)
                                        .map_err(|error| anyhow!(error))
                                        .await
                                    {
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

        MightNeedSummaryFiles { files: tx, task }
    }

    fn summarize_files(
        &self,
        unsummarized_files: channel::Receiver<UnsummarizedFile>,
        cx: &App,
    ) -> SummarizeFiles {
        let (summarized_tx, summarized_rx) = channel::bounded(512);
        let task = cx.spawn(async move |cx| {
            while let Ok(file) = unsummarized_files.recv().await {
                log::debug!("Summarizing {:?}", file);
                let summary = cx
                    .update(|cx| Self::summarize_code(&file.contents, &file.path, cx))?
                    .await
                    .unwrap_or_else(|err| {
                        // Log a warning because we'll continue anyway.
                        // In the future, we may want to try splitting it up into multiple requests and concatenating the summaries,
                        // but this might give bad summaries due to cutting off source code files in the middle.
                        log::warn!("Failed to summarize {} - {:?}", file.path.display(), err);

                        String::new()
                    });

                // Note that the summary could be empty because of an error talking to a cloud provider,
                // e.g. because the context limit was exceeded. In that case, we return Ok(String::new()).
                if !summary.is_empty() {
                    summarized_tx
                        .send(SummarizedFile {
                            path: file.path.display().to_string(),
                            digest: file.digest,
                            summary,
                            mtime: file.mtime,
                        })
                        .await?
                }
            }

            Ok(())
        });

        SummarizeFiles {
            files: summarized_rx,
            task,
        }
    }

    fn summarize_code(
        code: &str,
        path: &Path,
        cx: &App,
    ) -> impl Future<Output = Result<String>> + use<> {
        let start = Instant::now();
        let (summary_model_id, use_cache): (LanguageModelId, bool) = (
            "Qwen/Qwen2-7B-Instruct".to_string().into(), // TODO read this from the user's settings.
            false, // qwen2 doesn't have a cache, but we should probably infer this from the model
        );
        let Some(model) = LanguageModelRegistry::read_global(cx)
            .available_models(cx)
            .find(|model| &model.id() == &summary_model_id)
        else {
            return cx.background_spawn(async move {
                Err(anyhow!("Couldn't find the preferred summarization model ({:?}) in the language registry's available models", summary_model_id))
            });
        };
        let utf8_path = path.to_string_lossy();
        const PROMPT_BEFORE_CODE: &str = "Summarize what the code in this file does in 3 sentences, using no newlines or bullet points in the summary:";
        let prompt = format!("{PROMPT_BEFORE_CODE}\n{utf8_path}:\n{code}");

        log::debug!(
            "Summarizing code by sending this prompt to {:?}: {:?}",
            model.name(),
            &prompt
        );

        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            mode: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![prompt.into()],
                cache: use_cache,
            }],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
        };

        let code_len = code.len();
        cx.spawn(async move |cx| {
            let stream = model.stream_completion(request, &cx);
            cx.background_spawn(async move {
                let answer: String = stream
                    .await?
                    .filter_map(|event| async {
                        if let Ok(LanguageModelCompletionEvent::Text(text)) = event {
                            Some(text)
                        } else {
                            None
                        }
                    })
                    .collect()
                    .await;

                log::info!(
                    "It took {:?} to summarize {:?} bytes of code.",
                    start.elapsed(),
                    code_len
                );

                log::debug!("Summary was: {:?}", &answer);

                Ok(answer)
            })
            .await

            // TODO if summarization failed, put it back in the backlog!
        })
    }

    fn persist_summaries(
        &self,
        summaries: channel::Receiver<SummarizedFile>,
        cx: &App,
    ) -> Task<Result<()>> {
        let db_connection = self.db_connection.clone();
        let digest_db = self.file_digest_db;
        let summary_db = self.summary_db;
        cx.background_spawn(async move {
            let mut summaries = pin!(summaries.chunks_timeout(4096, Duration::from_secs(2)));
            while let Some(summaries) = summaries.next().await {
                let mut txn = db_connection.write_txn()?;
                for file in &summaries {
                    log::debug!(
                        "Saving summary of {:?} - which is {} bytes of summary for content digest {:?}",
                        &file.path,
                        file.summary.len(),
                        file.digest
                    );
                    digest_db.put(
                        &mut txn,
                        &file.path,
                        &FileDigest {
                            mtime: file.mtime,
                            digest: file.digest,
                        },
                    )?;
                    summary_db.put(&mut txn, &file.digest, &file.summary)?;
                }
                txn.commit()?;

                drop(summaries);
                log::debug!("committed summaries");
            }

            Ok(())
        })
    }

    /// Empty out the backlog of files that haven't been resummarized, and resummarize them immediately.
    pub(crate) fn flush_backlog(
        &self,
        worktree_abs_path: Arc<Path>,
        cx: &App,
    ) -> impl Future<Output = Result<()>> + use<> {
        let start = Instant::now();
        let backlogged = {
            let (tx, rx) = channel::bounded(512);
            let needs_summary: Vec<(Arc<Path>, Option<MTime>)> = {
                let mut backlog = self.backlog.lock();

                backlog.drain().collect()
            };

            let task = cx.background_spawn(async move {
                tx.send(needs_summary).await?;
                Ok(())
            });

            Backlogged {
                paths_to_digest: rx,
                task,
            }
        };

        let digest = self.digest_files(backlogged.paths_to_digest, worktree_abs_path, cx);
        let needs_summary = self.check_summary_cache(digest.files, cx);
        let summaries = self.summarize_files(needs_summary.files, cx);
        let persist = self.persist_summaries(summaries.files, cx);

        async move {
            futures::try_join!(
                backlogged.task,
                digest.task,
                needs_summary.task,
                summaries.task,
                persist
            )?;

            log::info!("Summarizing backlogged entries took {:?}", start.elapsed());

            Ok(())
        }
    }

    pub(crate) fn backlog_len(&self) -> usize {
        self.backlog.lock().len()
    }
}

fn db_key_for_path(path: &Arc<Path>) -> String {
    path.to_string_lossy().replace('/', "\0")
}
