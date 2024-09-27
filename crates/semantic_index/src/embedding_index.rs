use crate::{
    chunking::{self, Chunk},
    embedding::{Embedding, EmbeddingProvider, TextToEmbed},
    indexing::{IndexingEntryHandle, IndexingEntrySet},
    tfidf::{
        ChunkTermFrequency, CorpusTermFrequency, SimpleTokenizer, TermFrequency, TfIdfMetadata,
    },
};
use anyhow::{anyhow, Context as _, Result};
use collections::Bound;
use fs::Fs;
use futures::stream::StreamExt;
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{AppContext, Model, Task};
use heed::types::{SerdeBincode, Str};
use language::LanguageRegistry;
use log;
use project::{Entry, UpdatedEntriesSet, Worktree, WorktreeId};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{
    cmp::Ordering,
    collections::HashMap,
    future::Future,
    iter,
    path::Path,
    sync::{Arc, RwLock},
    time::{Duration, SystemTime},
};
use util::ResultExt;
use worktree::Snapshot;

#[derive(Debug, Clone, Copy)]
pub struct EmbeddingIndexSettings {
    pub scan_entries_bound: usize,
    pub deleted_entries_bound: usize,
    pub chunk_files_bound: usize,
    pub embed_files_bound: usize,
}

impl Default for EmbeddingIndexSettings {
    fn default() -> Self {
        Self {
            scan_entries_bound: 512,
            deleted_entries_bound: 128,
            chunk_files_bound: 2048,
            embed_files_bound: 512,
        }
    }
}

pub struct EmbeddingIndex {
    worktree: Model<Worktree>,
    db_connection: heed::Env,
    db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    entry_ids_being_indexed: Arc<IndexingEntrySet>,
    corpus_frequencies: Arc<RwLock<HashMap<WorktreeId, CorpusTermFrequency>>>,
    tokenizer: SimpleTokenizer,
    settings: EmbeddingIndexSettings,
}

impl EmbeddingIndex {
    pub fn new(
        worktree: Model<Worktree>,
        fs: Arc<dyn Fs>,
        db_connection: heed::Env,
        db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
        language_registry: Arc<LanguageRegistry>,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        entry_ids_being_indexed: Arc<IndexingEntrySet>,
    ) -> Self {
        Self {
            worktree,
            fs,
            db_connection,
            db,
            language_registry,
            embedding_provider,
            entry_ids_being_indexed,
            corpus_frequencies: Arc::new(RwLock::new(HashMap::new())),
            tokenizer: SimpleTokenizer::new(),
            settings: EmbeddingIndexSettings::default(),
        }
    }

    pub fn db(&self) -> &heed::Database<Str, SerdeBincode<EmbeddedFile>> {
        &self.db
    }

    pub fn index_entries_changed_on_disk(
        &mut self,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx).snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_entries(worktree, cx);
        let chunk = self.chunk_files(worktree_abs_path, scan.updated_entries, cx);
        let embed = Self::embed_files(
            self.embedding_provider.clone(),
            chunk.files,
            self.tokenizer.clone(),
            self.settings,
            cx,
        );
        let persist = self.persist_embeddings(scan.deleted_entry_ranges, embed.files, cx);
        async move {
            futures::try_join!(scan.task, chunk.task, embed.task, persist)?;
            Ok(())
        }
    }

    pub fn index_updated_entries(
        &self,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx).snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let scan = self.scan_updated_entries(worktree, updated_entries.clone(), cx);
        let chunk = self.chunk_files(worktree_abs_path, scan.updated_entries, cx);
        let embed = Self::embed_files(
            self.embedding_provider.clone(),
            chunk.files,
            self.tokenizer.clone(),
            self.settings,
            cx,
        );
        let persist = self.persist_embeddings(scan.deleted_entry_ranges, embed.files, cx);
        async move {
            futures::try_join!(scan.task, chunk.task, embed.task, persist)?;
            Ok(())
        }
    }

    fn scan_entries(&mut self, worktree: Snapshot, cx: &AppContext) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) =
            channel::bounded(self.settings.scan_entries_bound);
        let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) =
            channel::bounded(self.settings.deleted_entries_bound);
        let db_connection = self.db_connection.clone();
        let db = self.db;
        let entries_being_indexed = self.entry_ids_being_indexed.clone();
        let corpus_frequency_by_worktree = self.corpus_frequencies.clone();

        let task = cx.background_executor().spawn(async move {
            let txn = db_connection
                .read_txn()
                .context("failed to create read transaction")?;
            let mut db_entries = db
                .iter(&txn)
                .context("failed to create iterator")?
                .move_between_keys()
                .peekable();

            let mut deletion_range: Option<(Bound<&str>, Bound<&str>)> = None;
            for entry in worktree.files(false, 0) {
                log::trace!("scanning for embedding index: {:?}", &entry.path);
                let entry_db_key = db_key_for_path(&entry.path);
                if let Some(embedded_file) = db.get(&txn, &entry_db_key)? {
                    // initialize the CorpusTermFrequency for each worktree
                    let mut cf_map = corpus_frequency_by_worktree
                        .write()
                        .map_err(|_| anyhow!("RwLock poisoned"))?;
                    if let Some(cf) = cf_map.get_mut(&worktree.id()) {
                        for chunk in &embedded_file.chunks {
                            cf.add_chunk(&chunk.term_frequencies);
                        }
                    } else {
                        let mut new_cf = CorpusTermFrequency::new();
                        for chunk in &embedded_file.chunks {
                            new_cf.add_chunk(&chunk.term_frequencies);
                        }
                        cf_map.insert(worktree.id(), new_cf);
                    }
                }

                let mut saved_mtime = None;
                while let Some(db_entry) = db_entries.peek() {
                    match db_entry {
                        Ok((db_path, db_embedded_file)) => match (*db_path).cmp(&entry_db_key) {
                            Ordering::Less => {
                                if let Some(deletion_range) = deletion_range.as_mut() {
                                    deletion_range.1 = Bound::Included(db_path);
                                } else {
                                    deletion_range =
                                        Some((Bound::Included(db_path), Bound::Included(db_path)));
                                }

                                db_entries.next();
                            }
                            Ordering::Equal => {
                                if let Some(deletion_range) = deletion_range.take() {
                                    deleted_entry_ranges_tx
                                        .send((
                                            deletion_range.0.map(ToString::to_string),
                                            deletion_range.1.map(ToString::to_string),
                                        ))
                                        .await?;
                                }
                                saved_mtime = db_embedded_file.mtime;
                                db_entries.next();
                                break;
                            }
                            Ordering::Greater => {
                                break;
                            }
                        },
                        Err(_) => return Err(db_entries.next().unwrap().unwrap_err())?,
                    }
                }

                if entry.mtime != saved_mtime {
                    let handle = entries_being_indexed.insert(entry.id);
                    updated_entries_tx.send((entry.clone(), handle)).await?;
                }
            }

            if let Some(db_entry) = db_entries.next() {
                let (db_path, _) = db_entry?;
                deleted_entry_ranges_tx
                    .send((Bound::Included(db_path.to_string()), Bound::Unbounded))
                    .await?;
            }

            Ok(())
        });

        ScanEntries {
            updated_entries: updated_entries_rx,
            deleted_entry_ranges: deleted_entry_ranges_rx,
            task,
        }
    }

    fn scan_updated_entries(
        &self,
        worktree: Snapshot,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) =
            channel::bounded(self.settings.scan_entries_bound);
        let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) =
            channel::bounded(self.settings.deleted_entries_bound);
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

    fn chunk_files(
        &self,
        worktree_abs_path: Arc<Path>,
        entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
        cx: &AppContext,
    ) -> ChunkFiles {
        let language_registry = self.language_registry.clone();
        let fs = self.fs.clone();
        let (chunked_files_tx, chunked_files_rx) =
            channel::bounded(self.settings.chunk_files_bound);
        let task = cx.spawn(|cx| async move {
            cx.background_executor()
                .scoped(|cx| {
                    for _ in 0..cx.num_cpus() {
                        cx.spawn(async {
                            while let Ok((entry, handle)) = entries.recv().await {
                                let entry_abs_path = worktree_abs_path.join(&entry.path);
                                if let Some(text) = fs.load(&entry_abs_path).await.ok() {
                                    let language = language_registry
                                        .language_for_file_path(&entry.path)
                                        .await
                                        .ok();
                                    let chunked_file = ChunkedFile {
                                        chunks: chunking::chunk_text(
                                            &text,
                                            language.as_ref(),
                                            &entry.path,
                                        ),
                                        handle,
                                        path: entry.path,
                                        mtime: entry.mtime,
                                        text,
                                    };

                                    if chunked_files_tx.send(chunked_file).await.is_err() {
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

        ChunkFiles {
            files: chunked_files_rx,
            task,
        }
    }

    pub fn embed_files(
        embedding_provider: Arc<dyn EmbeddingProvider>,
        chunked_files: channel::Receiver<ChunkedFile>,
        tokenizer: SimpleTokenizer,
        settings: EmbeddingIndexSettings,
        cx: &AppContext,
    ) -> EmbedFiles {
        let (embedded_files_tx, embedded_files_rx) = channel::bounded(settings.embed_files_bound);
        let task = cx.background_executor().spawn(async move {
            let mut chunked_file_batches =
                chunked_files.chunks_timeout(512, Duration::from_secs(2));
            while let Some(chunked_files) = chunked_file_batches.next().await {
                // View the batch of files as a vec of chunks
                // Flatten out to a vec of chunks that we can subdivide into batch sized pieces
                // Once those are done, reassemble them back into the files in which they belong
                // If any embeddings fail for a file, the entire file is discarded
                let chunks: Vec<TextToEmbed> = chunked_files
                    .iter()
                    .flat_map(|file| {
                        file.chunks.iter().map(|chunk| TextToEmbed {
                            text: &file.text[chunk.range.clone()],
                            digest: chunk.digest,
                        })
                    })
                    .collect::<Vec<_>>();

                let mut embeddings: Vec<Option<Embedding>> = Vec::new();
                for embedding_batch in chunks.chunks(embedding_provider.batch_size()) {
                    if let Some(batch_embeddings) =
                        embedding_provider.embed(embedding_batch).await.log_err()
                    {
                        if batch_embeddings.len() == embedding_batch.len() {
                            embeddings.extend(batch_embeddings.into_iter().map(Some));
                            continue;
                        }
                        log::error!(
                            "embedding provider returned unexpected embedding count {}, expected {}",
                            batch_embeddings.len(), embedding_batch.len()
                        );
                    }

                    embeddings.extend(iter::repeat(None).take(embedding_batch.len()));
                }
                let mut embeddings = embeddings.into_iter();
                for chunked_file in chunked_files {
                    let mut embedded_file = EmbeddedFile {
                        path: chunked_file.path,
                        mtime: chunked_file.mtime,
                        chunks: Vec::new(),
                    };

                    let mut embedded_all_chunks = true;
                    for (chunk, embedding) in
                        chunked_file.chunks.into_iter().zip(embeddings.by_ref())
                    {
                        if let Some(embedding) = embedding {
                            let term_frequencies =
                                ChunkTermFrequency::from_text(&chunked_file.text, &tokenizer);
                            let chunk_length = term_frequencies.total_terms();
                            embedded_file
                                .chunks
                                .push(EmbeddedChunk { chunk, embedding, term_frequencies, chunk_length });
                        } else {
                            embedded_all_chunks = false;
                        }
                    }

                    if embedded_all_chunks {
                        embedded_files_tx
                            .send((embedded_file, chunked_file.handle))
                            .await?;
                    }
                }
            }
            Ok(())
        });

        EmbedFiles {
            files: embedded_files_rx,
            task,
        }
    }

    fn persist_embeddings(
        &self,
        mut deleted_entry_ranges: channel::Receiver<(Bound<String>, Bound<String>)>,
        mut embedded_files: channel::Receiver<(EmbeddedFile, IndexingEntryHandle)>,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let db_connection = self.db_connection.clone();
        let db = self.db;

        cx.background_executor().spawn(async move {
            loop {
                futures::select_biased! {
                    deletion_range = deleted_entry_ranges.next() => {
                        if let Some(deletion_range) = deletion_range {
                            let mut txn = db_connection.write_txn()?;
                            let start = deletion_range.0.as_ref().map(|s| s.as_str());
                            let end = deletion_range.1.as_ref().map(|s| s.as_str());
                            db.delete_range(&mut txn, &(start, end)).context("failed to delete embedding range")?;
                            txn.commit()?;
                        }
                    },
                    embedded_file = embedded_files.next() => {
                        if let Some((file, _)) = embedded_file {
                            let mut txn = db_connection.write_txn()?;
                            let key = db_key_for_path(&file.path);
                            db.put(&mut txn, &key, &file)
                                .context("failed to write embedded file")?;
                            txn.commit()?;
                        }
                    },
                    complete => break,
                }
            }

            Ok(())
        })
    }

    pub fn paths(&self, cx: &AppContext) -> Task<Result<Vec<Arc<Path>>>> {
        let connection = self.db_connection.clone();
        let db = self.db;
        cx.background_executor().spawn(async move {
            let tx = connection
                .read_txn()
                .context("failed to create read transaction")?;
            let result = db
                .iter(&tx)?
                .filter_map(|entry| {
                    if let Ok((_, file)) = entry {
                        Some(Ok(file.path.clone()))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<Arc<Path>>>>();
            drop(tx);
            result
        })
    }

    pub fn chunks_for_path(
        &self,
        path: Arc<Path>,
        cx: &AppContext,
    ) -> Task<Result<Vec<EmbeddedChunk>>> {
        let connection = self.db_connection.clone();
        let db = self.db;
        cx.background_executor().spawn(async move {
            let tx = connection
                .read_txn()
                .context("failed to create read transaction")?;
            match db.get(&tx, &db_key_for_path(&path))? {
                Some(file) => Ok(file.chunks.clone()),
                None => Err(anyhow!("no such path")),
            }
        })
    }
}

struct ScanEntries {
    updated_entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
    deleted_entry_ranges: channel::Receiver<(Bound<String>, Bound<String>)>,
    task: Task<Result<()>>,
}

struct ChunkFiles {
    files: channel::Receiver<ChunkedFile>,
    task: Task<Result<()>>,
}

pub struct ChunkedFile {
    pub path: Arc<Path>,
    pub mtime: Option<SystemTime>,
    pub handle: IndexingEntryHandle,
    pub text: String,
    pub chunks: Vec<Chunk>,
}

pub struct EmbedFiles {
    pub files: channel::Receiver<(EmbeddedFile, IndexingEntryHandle)>,
    pub task: Task<Result<()>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedFile {
    pub path: Arc<Path>,
    pub mtime: Option<SystemTime>,
    pub chunks: Vec<EmbeddedChunk>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedChunk {
    pub chunk: Chunk,
    pub embedding: Embedding,
    pub term_frequencies: ChunkTermFrequency,
    pub chunk_length: u32,
}

fn db_key_for_path(path: &Arc<Path>) -> String {
    path.to_string_lossy().replace('/', "\0")
}
