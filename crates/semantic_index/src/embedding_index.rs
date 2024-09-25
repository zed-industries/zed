use crate::{
    chunking::{self, Chunk},
    embedding::{Embedding, EmbeddingProvider, TextToEmbed},
    indexing::{IndexingEntryHandle, IndexingEntrySet},
    tfidf::{ChunkTermFrequency, SimpleTokenizer, TermFrequency, TfIdfMetadata},
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
use project::{Entry, UpdatedEntriesSet, Worktree};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{cmp::Ordering, future::Future, path::Path, sync::Arc, time::SystemTime};
use worktree::Snapshot;

#[derive(Debug, Clone, Copy)]
pub struct EmbeddingIndexSettings {
    pub scan_entries_bound: usize,
    pub deleted_entries_bound: usize,
    pub chunk_files_bound: usize,
    pub chunk_files_batch_size: usize,
    pub embed_files_bound: usize,
}

impl Default for EmbeddingIndexSettings {
    fn default() -> Self {
        Self {
            scan_entries_bound: 512,
            deleted_entries_bound: 128,
            chunk_files_bound: 16,
            chunk_files_batch_size: 32,
            embed_files_bound: 16,
        }
    }
}

pub struct EmbeddingIndex {
    worktree: Model<Worktree>,
    db_connection: heed::Env,
    embedding_db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
    tfidf_metadata_db: heed::Database<Str, SerdeBincode<TfIdfMetadata>>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    entry_ids_being_indexed: Arc<IndexingEntrySet>,
    tokenizer: SimpleTokenizer,
    settings: EmbeddingIndexSettings,
}

impl EmbeddingIndex {
    pub fn new(
        worktree: Model<Worktree>,
        fs: Arc<dyn Fs>,
        db_connection: heed::Env,
        embedding_db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
        tfidf_metadata_db: heed::Database<Str, SerdeBincode<TfIdfMetadata>>,
        language_registry: Arc<LanguageRegistry>,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        entry_ids_being_indexed: Arc<IndexingEntrySet>,
    ) -> Self {
        Self {
            worktree,
            fs,
            db_connection,
            embedding_db,
            language_registry,
            embedding_provider,
            entry_ids_being_indexed,
            tokenizer: SimpleTokenizer::new(),
            tfidf_metadata_db,
            settings: EmbeddingIndexSettings::default(),
        }
    }

    pub fn embedding_db(&self) -> &heed::Database<Str, SerdeBincode<EmbeddedFile>> {
        &self.embedding_db
    }

    pub fn index_entries_changed_on_disk(
        &self,
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

    fn scan_entries(&self, worktree: Snapshot, cx: &AppContext) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) =
            channel::bounded(self.settings.scan_entries_bound);
        let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) =
            channel::bounded(self.settings.deleted_entries_bound);
        let db_connection = self.db_connection.clone();
        let db = self.embedding_db;
        let entries_being_indexed = self.entry_ids_being_indexed.clone();
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
        let batch_size = self.settings.chunk_files_batch_size;
        let task = cx.spawn(|cx| {
            cx.background_executor().spawn(async move {
                let mut current_batch = Vec::new();

                while let Ok((entry, handle)) = entries.recv().await {
                    let entry_abs_path = worktree_abs_path.join(&entry.path);
                    if let Some(text) = fs.load(&entry_abs_path).await.ok() {
                        let language = language_registry
                            .language_for_file_path(&entry.path)
                            .await
                            .ok();
                        let chunks = chunking::chunk_text(&text, language.as_ref(), &entry.path);
                        let chunked_file = ChunkedFile {
                            chunks,
                            handle,
                            path: entry.path,
                            mtime: entry.mtime,
                            text,
                        };

                        current_batch.push(chunked_file);

                        if current_batch.len() >= batch_size {
                            chunked_files_tx.send(current_batch).await?;
                            current_batch = Vec::new();
                        }
                    }
                }

                if !current_batch.is_empty() {
                    chunked_files_tx.send(current_batch).await?;
                }

                Ok(())
            })
        });

        ChunkFiles {
            files: chunked_files_rx,
            task,
        }
    }

    pub fn embed_files(
        embedding_provider: Arc<dyn EmbeddingProvider>,
        chunked_files: channel::Receiver<Vec<ChunkedFile>>,
        tokenizer: SimpleTokenizer,
        settings: EmbeddingIndexSettings,
        cx: &AppContext,
    ) -> EmbedFiles {
        let (embedded_files_tx, embedded_files_rx) = channel::bounded(settings.embed_files_bound);
        let task = cx.background_executor().spawn(async move {
            while let Ok(batch) = chunked_files.recv().await {
                let chunks: Vec<TextToEmbed> = batch
                    .iter()
                    .flat_map(|file| {
                        file.chunks.iter().map(|chunk| TextToEmbed {
                            text: &file.text[chunk.range.clone()],
                            digest: chunk.digest,
                        })
                    })
                    .collect();

                let embeddings = embedding_provider.embed(&chunks).await?;

                let embedded_batch: Vec<_> = batch
                    .iter()
                    .zip(chunks.chunks(batch.len()))
                    .zip(embeddings.chunks(batch.len()))
                    .map(|((chunked_file, file_chunks), file_embeddings)| {
                        let embedded_chunks: Vec<_> = chunked_file
                            .chunks
                            .iter()
                            .zip(file_chunks)
                            .zip(file_embeddings)
                            .map(|((chunk, text_to_embed), embedding)| {
                                let term_frequencies =
                                    ChunkTermFrequency::from_text(&text_to_embed.text, &tokenizer);
                                let chunk_length = term_frequencies.total_terms();
                                EmbeddedChunk {
                                    chunk: chunk.clone(),
                                    embedding: embedding.clone(),
                                    term_frequencies,
                                    chunk_length,
                                }
                            })
                            .collect();

                        (
                            EmbeddedFile {
                                path: chunked_file.path.clone(),
                                mtime: chunked_file.mtime,
                                chunks: embedded_chunks,
                            },
                            chunked_file.handle.clone(),
                        )
                    })
                    .collect();

                embedded_files_tx.send(embedded_batch).await?;
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
        mut embedded_files: channel::Receiver<Vec<(EmbeddedFile, IndexingEntryHandle)>>,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let db_connection = self.db_connection.clone();
        let embedding_db = self.embedding_db;
        let tfidf_metadata_db = self.tfidf_metadata_db;

        cx.background_executor().spawn(async move {
            loop {
                futures::select_biased! {
                    deletion_range = deleted_entry_ranges.next() => {
                        if let Some(deletion_range) = deletion_range {
                            Self::apply_deletion(&db_connection, &embedding_db, &tfidf_metadata_db, deletion_range)?;
                        }
                    },
                    batch = embedded_files.next() => {
                        if let Some(batch) = batch {
                            Self::apply_batch(&db_connection, &embedding_db, &tfidf_metadata_db, batch)?;
                        }
                    },
                    complete => break,
                }
            }

            Ok(())
        })
    }

    fn apply_deletion(
        db_connection: &heed::Env,
        embedding_db: &heed::Database<Str, SerdeBincode<EmbeddedFile>>,
        tfidf_metadata_db: &heed::Database<Str, SerdeBincode<TfIdfMetadata>>,
        deletion_range: (Bound<String>, Bound<String>),
    ) -> Result<()> {
        let mut txn = db_connection.write_txn()?;
        let start = deletion_range.0.as_ref().map(|s| s.as_str());
        let end = deletion_range.1.as_ref().map(|s| s.as_str());

        let mut metadata = tfidf_metadata_db
            .get(&txn, "__tfidf_metadata__")?
            .unwrap_or_else(TfIdfMetadata::new);

        let deleted_files = embedding_db.range(&txn, &(start, end))?;
        for result in deleted_files {
            let (_, embedded_file) = result?;
            for chunk in &embedded_file.chunks {
                metadata.remove_chunk(&chunk.term_frequencies);
            }
        }

        embedding_db.delete_range(&mut txn, &(start, end))?;
        tfidf_metadata_db.put(&mut txn, "__tfidf_metadata__", &metadata)?;
        txn.commit()?;
        Ok(())
    }

    fn apply_batch(
        db_connection: &heed::Env,
        embedding_db: &heed::Database<Str, SerdeBincode<EmbeddedFile>>,
        tfidf_metadata_db: &heed::Database<Str, SerdeBincode<TfIdfMetadata>>,
        batch: Vec<(EmbeddedFile, IndexingEntryHandle)>,
    ) -> Result<()> {
        let mut txn = db_connection.write_txn()?;
        let mut metadata = tfidf_metadata_db
            .get(&txn, "__tfidf_metadata__")?
            .unwrap_or_else(TfIdfMetadata::new);

        for (file, _) in batch {
            let key = db_key_for_path(&file.path);
            embedding_db.put(&mut txn, &key, &file)?;
            for chunk in &file.chunks {
                metadata.add_chunk(&chunk.term_frequencies);
            }
        }

        tfidf_metadata_db.put(&mut txn, "__tfidf_metadata__", &metadata)?;
        txn.commit()?;
        Ok(())
    }

    pub fn reconcile_tfidf_metadata(&self, cx: &AppContext) -> Task<Result<()>> {
        let db_connection = self.db_connection.clone();
        let embedding_db = self.embedding_db;
        let tfidf_metadata_db = self.tfidf_metadata_db;

        cx.background_executor().spawn(async move {
            let mut txn = db_connection.write_txn()?;
            let mut metadata = TfIdfMetadata::new();

            let all_embeddings = embedding_db.iter(&txn)?;
            for result in all_embeddings {
                let (_, embedded_file) = result?;
                for chunk in &embedded_file.chunks {
                    metadata.add_chunk(&chunk.term_frequencies);
                }
            }

            tfidf_metadata_db.put(&mut txn, "__tfidf_metadata__", &metadata)?;
            txn.commit()?;

            Ok(())
        })
    }

    pub fn paths(&self, cx: &AppContext) -> Task<Result<Vec<Arc<Path>>>> {
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

    pub fn chunks_for_path(
        &self,
        path: Arc<Path>,
        cx: &AppContext,
    ) -> Task<Result<Vec<EmbeddedChunk>>> {
        let connection = self.db_connection.clone();
        let db = self.embedding_db;
        cx.background_executor().spawn(async move {
            let tx = connection
                .read_txn()
                .context("failed to create read transaction")?;
            Ok(db
                .get(&tx, &db_key_for_path(&path))?
                .ok_or_else(|| anyhow!("no such path"))?
                .chunks
                .clone())
        })
    }
}

struct ScanEntries {
    updated_entries: channel::Receiver<(Entry, IndexingEntryHandle)>,
    deleted_entry_ranges: channel::Receiver<(Bound<String>, Bound<String>)>,
    task: Task<Result<()>>,
}

struct ChunkFiles {
    files: channel::Receiver<Vec<ChunkedFile>>,
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
    pub files: channel::Receiver<Vec<(EmbeddedFile, IndexingEntryHandle)>>,
    pub task: Task<Result<()>>,
}

#[derive(Debug, Serialize, Deserialize)]
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
