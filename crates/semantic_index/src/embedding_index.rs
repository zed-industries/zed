use crate::chunking::Chunk;
use crate::embedding::*;
use anyhow::{anyhow, Context as _, Result};
use collections::Bound;
use futures::stream::StreamExt;
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{AppContext, Task};
use heed::types::{SerdeBincode, Str};
use language::LanguageRegistry;
use log;
use serde::{Deserialize, Serialize};
use smol::channel::{self, Sender};
use std::{
    cmp::Ordering,
    future::Future,
    iter,
    path::Path,
    sync::Arc,
    time::{Duration, SystemTime},
};
use util::ResultExt;
use worktree::Snapshot;

fn db_key_for_path(path: &Arc<Path>) -> String {
    path.to_string_lossy().replace('/', "\0")
}

pub struct EmbeddingIndex {
    db_connection: heed::Env,
    embedding_db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
    language_registry: Arc<LanguageRegistry>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddedFile {
    pub path: Arc<Path>,
    pub mtime: Option<SystemTime>,
    pub chunks: Vec<EmbeddedChunk>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EmbeddedChunk {
    pub chunk: Chunk,
    pub embedding: Embedding,
}

struct ChunkedFile {
    pub path: Arc<Path>,
    pub mtime: Option<SystemTime>,
    pub handle: IndexingEntryHandle,
    pub text: String,
    pub chunks: Vec<Chunk>,
}

struct EmbedFiles {
    files: channel::Receiver<(EmbeddedFile, IndexingEntryHandle)>,
    task: Task<Result<()>>,
}

impl EmbeddingIndex {
    pub fn new(
        db_connection: heed::Env,
        embedding_db: heed::Database<Str, SerdeBincode<EmbeddedFile>>,
        language_registry: Arc<LanguageRegistry>,
        embedding_provider: Arc<dyn EmbeddingProvider>,
    ) -> Self {
        Self {
            db_connection,
            embedding_db,
            language_registry,
            embedding_provider,
        }
    }

    pub async fn read_chunks<'a, Id: Send + Sync + Clone + 'a>(
        &'a self,
        tx: Sender<(Id, Arc<Path>, EmbeddedChunk)>,
        id: Id,
    ) -> Result<()> {
        let db_connection = self.db_connection.clone();
        let db = self.embedding_db;
        let txn = db_connection
            .read_txn()
            .context("failed to create read transaction")?;
        let db_entries = db.iter(&txn).context("failed to iterate database")?;
        for db_entry in db_entries {
            let (_key, db_embedded_file) = db_entry?;
            for chunk in db_embedded_file.chunks {
                let path = Arc::clone(&db_embedded_file.path);

                tx.send((id, path, chunk)).await?;
            }
        }
        anyhow::Ok(())
    }

    pub fn db(&self) -> &heed::Database<Str, SerdeBincode<EmbeddedFile>> {
        &self.embedding_db
    }

    pub fn scan_entries(
        &self,
        db_conn: heed::Env,
        worktree: Snapshot,
        cx: &AppContext,
    ) -> ScanEntries {
        let (updated_entries_tx, updated_entries_rx) = channel::bounded(512);
        let (deleted_entry_ranges_tx, deleted_entry_ranges_rx) = channel::bounded(128);
        let db = self.embedding_db;
        let entries_being_indexed = self.entry_ids_being_indexed.clone();
        let task = cx.background_executor().spawn(async move {
            let txn = db_conn
                .read_txn()
                .context("failed to create read transaction")?;
            let mut db_entries = db
                .iter(&txn)
                .context("failed to create iterator")?
                .move_between_keys()
                .peekable();

            let mut deletion_range: Option<(Bound<&str>, Bound<&str>)> = None;
            for entry in worktree.files(false, 0) {
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

    fn embed_files(
        embedding_provider: Arc<dyn EmbeddingProvider>,
        chunked_files: channel::Receiver<ChunkedFile>,
        cx: &AppContext,
    ) -> EmbedFiles {
        let embedding_provider = embedding_provider.clone();
        let (embedded_files_tx, embedded_files_rx) = channel::bounded(512);
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
                            embedded_file
                                .chunks
                                .push(EmbeddedChunk { chunk, embedding });
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
        embedded_files: channel::Receiver<(EmbeddedFile, IndexingEntryHandle)>,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let db_connection = self.db_connection.clone();
        let db = self.embedding_db;
        cx.background_executor().spawn(async move {
            while let Some(deletion_range) = deleted_entry_ranges.next().await {
                let mut txn = db_connection.write_txn()?;
                let start = deletion_range.0.as_ref().map(|start| start.as_str());
                let end = deletion_range.1.as_ref().map(|end| end.as_str());
                log::debug!("deleting embeddings in range {:?}", &(start, end));
                db.delete_range(&mut txn, &(start, end))?;
                txn.commit()?;
            }

            let mut embedded_files = embedded_files.chunks_timeout(4096, Duration::from_secs(2));
            while let Some(embedded_files) = embedded_files.next().await {
                let mut txn = db_connection.write_txn()?;
                for (file, _) in &embedded_files {
                    log::debug!("saving embedding for file {:?}", file.path);
                    let key = db_key_for_path(&file.path);
                    db.put(&mut txn, &key, file)?;
                }
                txn.commit()?;

                drop(embedded_files);
                log::debug!("committed embeddings");
            }

            Ok(())
        })
    }

    fn chunks_for_path(
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

    #[cfg(test)]
    fn path_count(&self) -> Result<u64> {
        let txn = self
            .db_connection
            .read_txn()
            .context("failed to create read transaction")?;
        Ok(self.embedding_db.len(&txn)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::worktree_index::WorktreeIndex;

    use super::*;
    use futures::{future::BoxFuture, FutureExt};
    use gpui::TestAppContext;
    use language::language_settings::AllLanguageSettings;
    use project::Project;
    use settings::SettingsStore;
    use std::{future, path::Path, sync::Arc};

    fn init_test(cx: &mut TestAppContext) {
        _ = cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            language::init(cx);
            Project::init_settings(cx);
            SettingsStore::update(cx, |store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
            });
        });
    }

    pub struct TestEmbeddingProvider {
        batch_size: usize,
        compute_embedding: Box<dyn Fn(&str) -> Result<Embedding> + Send + Sync>,
    }

    impl TestEmbeddingProvider {
        pub fn new(
            batch_size: usize,
            compute_embedding: impl 'static + Fn(&str) -> Result<Embedding> + Send + Sync,
        ) -> Self {
            return Self {
                batch_size,
                compute_embedding: Box::new(compute_embedding),
            };
        }
    }

    impl EmbeddingProvider for TestEmbeddingProvider {
        fn embed<'a>(
            &'a self,
            texts: &'a [TextToEmbed<'a>],
        ) -> BoxFuture<'a, Result<Vec<Embedding>>> {
            let embeddings = texts
                .iter()
                .map(|to_embed| (self.compute_embedding)(to_embed.text))
                .collect();
            future::ready(embeddings).boxed()
        }

        fn batch_size(&self) -> usize {
            self.batch_size
        }
    }

    #[gpui::test]
    async fn test_search(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        init_test(cx);

        let temp_dir = tempfile::tempdir().unwrap();

        let mut semantic_index = SemanticIndex::new(
            temp_dir.path().into(),
            Arc::new(TestEmbeddingProvider::new(16, |text| {
                let mut embedding = vec![0f32; 2];
                // if the text contains garbage, give it a 1 in the first dimension
                if text.contains("garbage in") {
                    embedding[0] = 0.9;
                } else {
                    embedding[0] = -0.9;
                }

                if text.contains("garbage out") {
                    embedding[1] = 0.9;
                } else {
                    embedding[1] = -0.9;
                }

                Ok(Embedding::new(embedding))
            })),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let project_path = Path::new("./fixture");

        let project = cx
            .spawn(|mut cx| async move { Project::example([project_path], &mut cx).await })
            .await;

        cx.update(|cx| {
            let language_registry = project.read(cx).languages().clone();
            let node_runtime = project.read(cx).node_runtime().unwrap().clone();
            languages::init(language_registry, node_runtime, cx);
        });

        let project_index = cx.update(|cx| semantic_index.project_index(project.clone(), cx));

        while project_index
            .read_with(cx, |index, cx| index.path_count(cx))
            .unwrap()
            == 0
        {
            project_index.next_event(cx).await;
        }

        let results = cx
            .update(|cx| {
                let project_index = project_index.read(cx);
                let query = "garbage in, garbage out";
                project_index.search(query.into(), 4, cx)
            })
            .await
            .unwrap();

        assert!(results.len() > 1, "should have found some results");

        for result in &results {
            println!("result: {:?}", result.path);
            println!("score: {:?}", result.score);
        }

        // Find result that is greater than 0.5
        let search_result = results.iter().find(|result| result.score > 0.9).unwrap();

        assert_eq!(search_result.path.to_string_lossy(), "needle.md");

        let content = cx
            .update(|cx| {
                let worktree = search_result.worktree.read(cx);
                let entry_abs_path = worktree.abs_path().join(&search_result.path);
                let fs = project.read(cx).fs().clone();
                cx.background_executor()
                    .spawn(async move { fs.load(&entry_abs_path).await.unwrap() })
            })
            .await;

        let range = search_result.range.clone();
        let content = content[range.clone()].to_owned();

        assert!(content.contains("garbage in, garbage out"));
    }

    #[gpui::test]
    async fn test_embed_files(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let provider = Arc::new(TestEmbeddingProvider::new(3, |text| {
            if text.contains('g') {
                Err(anyhow!("cannot embed text containing a 'g' character"))
            } else {
                Ok(Embedding::new(
                    ('a'..'z')
                        .map(|char| text.chars().filter(|c| *c == char).count() as f32)
                        .collect(),
                ))
            }
        }));

        let (indexing_progress_tx, _) = channel::unbounded();
        let indexing_entries = Arc::new(IndexingEntrySet::new(indexing_progress_tx));

        let (chunked_files_tx, chunked_files_rx) = channel::unbounded::<ChunkedFile>();
        chunked_files_tx
            .send_blocking(ChunkedFile {
                path: Path::new("test1.md").into(),
                mtime: None,
                handle: indexing_entries.insert(ProjectEntryId::from_proto(0)),
                text: "abcdefghijklmnop".to_string(),
                chunks: [0..4, 4..8, 8..12, 12..16]
                    .into_iter()
                    .map(|range| Chunk {
                        range,
                        digest: Default::default(),
                    })
                    .collect(),
            })
            .unwrap();
        chunked_files_tx
            .send_blocking(ChunkedFile {
                path: Path::new("test2.md").into(),
                mtime: None,
                handle: indexing_entries.insert(ProjectEntryId::from_proto(1)),
                text: "qrstuvwxyz".to_string(),
                chunks: [0..4, 4..8, 8..10]
                    .into_iter()
                    .map(|range| Chunk {
                        range,
                        digest: Default::default(),
                    })
                    .collect(),
            })
            .unwrap();
        chunked_files_tx.close();

        let embed_files_task =
            cx.update(|cx| WorktreeIndex::embed_files(provider.clone(), chunked_files_rx, cx));
        embed_files_task.task.await.unwrap();

        let mut embedded_files_rx = embed_files_task.files;
        let mut embedded_files = Vec::new();
        while let Some((embedded_file, _)) = embedded_files_rx.next().await {
            embedded_files.push(embedded_file);
        }

        assert_eq!(embedded_files.len(), 1);
        assert_eq!(embedded_files[0].path.as_ref(), Path::new("test2.md"));
        assert_eq!(
            embedded_files[0]
                .chunks
                .iter()
                .map(|embedded_chunk| { embedded_chunk.embedding.clone() })
                .collect::<Vec<Embedding>>(),
            vec![
                (provider.compute_embedding)("qrst").unwrap(),
                (provider.compute_embedding)("uvwx").unwrap(),
                (provider.compute_embedding)("yz").unwrap(),
            ],
        );
    }
}
