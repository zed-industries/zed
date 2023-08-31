use crate::{embedding::EmbeddingProvider, parsing::Document, JobHandle};
use gpui::executor::Background;
use parking_lot::Mutex;
use smol::channel;
use std::{mem, ops::Range, path::PathBuf, sync::Arc, time::SystemTime};

#[derive(Clone)]
pub struct FileToEmbed {
    pub worktree_id: i64,
    pub path: PathBuf,
    pub mtime: SystemTime,
    pub documents: Vec<Document>,
    pub job_handle: JobHandle,
}

impl std::fmt::Debug for FileToEmbed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileToEmbed")
            .field("worktree_id", &self.worktree_id)
            .field("path", &self.path)
            .field("mtime", &self.mtime)
            .field("document", &self.documents)
            .finish_non_exhaustive()
    }
}

impl PartialEq for FileToEmbed {
    fn eq(&self, other: &Self) -> bool {
        self.worktree_id == other.worktree_id
            && self.path == other.path
            && self.mtime == other.mtime
            && self.documents == other.documents
    }
}

pub struct EmbeddingQueue {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    pending_batch: Vec<FileToEmbedFragment>,
    executor: Arc<Background>,
    pending_batch_token_count: usize,
    finished_files_tx: channel::Sender<FileToEmbed>,
    finished_files_rx: channel::Receiver<FileToEmbed>,
}

pub struct FileToEmbedFragment {
    file: Arc<Mutex<FileToEmbed>>,
    document_range: Range<usize>,
}

impl EmbeddingQueue {
    pub fn new(embedding_provider: Arc<dyn EmbeddingProvider>, executor: Arc<Background>) -> Self {
        let (finished_files_tx, finished_files_rx) = channel::unbounded();
        Self {
            embedding_provider,
            executor,
            pending_batch: Vec::new(),
            pending_batch_token_count: 0,
            finished_files_tx,
            finished_files_rx,
        }
    }

    pub fn push(&mut self, file: FileToEmbed) {
        if file.documents.is_empty() {
            self.finished_files_tx.try_send(file).unwrap();
            return;
        }

        let file = Arc::new(Mutex::new(file));

        self.pending_batch.push(FileToEmbedFragment {
            file: file.clone(),
            document_range: 0..0,
        });

        let mut fragment_range = &mut self.pending_batch.last_mut().unwrap().document_range;
        for (ix, document) in file.lock().documents.iter().enumerate() {
            let next_token_count = self.pending_batch_token_count + document.token_count;
            if next_token_count > self.embedding_provider.max_tokens_per_batch() {
                let range_end = fragment_range.end;
                self.flush();
                self.pending_batch.push(FileToEmbedFragment {
                    file: file.clone(),
                    document_range: range_end..range_end,
                });
                fragment_range = &mut self.pending_batch.last_mut().unwrap().document_range;
            }

            fragment_range.end = ix + 1;
            self.pending_batch_token_count += document.token_count;
        }
    }

    pub fn flush(&mut self) {
        let batch = mem::take(&mut self.pending_batch);
        self.pending_batch_token_count = 0;
        if batch.is_empty() {
            return;
        }

        let finished_files_tx = self.finished_files_tx.clone();
        let embedding_provider = self.embedding_provider.clone();
        self.executor.spawn(async move {
            let mut spans = Vec::new();
            for fragment in &batch {
                let file = fragment.file.lock();
                spans.extend(
                    {
                        file.documents[fragment.document_range.clone()]
                            .iter()
                            .map(|d| d.content.clone())
                        }
                );
            }

            match embedding_provider.embed_batch(spans).await {
                Ok(embeddings) => {
                    let mut embeddings = embeddings.into_iter();
                    for fragment in batch {
                        for document in
                            &mut fragment.file.lock().documents[fragment.document_range.clone()]
                        {
                            if let Some(embedding) = embeddings.next() {
                                document.embedding = Some(embedding);
                            } else {
                                //
                                log::error!("number of embeddings returned different from number of documents");
                            }
                        }

                        if let Some(file) = Arc::into_inner(fragment.file) {
                            finished_files_tx.try_send(file.into_inner()).unwrap();
                        }
                    }
                }
                Err(error) => {
                    log::error!("{:?}", error);
                }
            }
        })
        .detach();
    }

    pub fn finished_files(&self) -> channel::Receiver<FileToEmbed> {
        self.finished_files_rx.clone()
    }
}
