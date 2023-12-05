use crate::{parsing::Span, JobHandle};
use ai::embedding::EmbeddingProvider;
use gpui::BackgroundExecutor;
use parking_lot::Mutex;
use smol::channel;
use std::{mem, ops::Range, path::Path, sync::Arc, time::SystemTime};

#[derive(Clone)]
pub struct FileToEmbed {
    pub worktree_id: i64,
    pub path: Arc<Path>,
    pub mtime: SystemTime,
    pub spans: Vec<Span>,
    pub job_handle: JobHandle,
}

impl std::fmt::Debug for FileToEmbed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileToEmbed")
            .field("worktree_id", &self.worktree_id)
            .field("path", &self.path)
            .field("mtime", &self.mtime)
            .field("spans", &self.spans)
            .finish_non_exhaustive()
    }
}

impl PartialEq for FileToEmbed {
    fn eq(&self, other: &Self) -> bool {
        self.worktree_id == other.worktree_id
            && self.path == other.path
            && self.mtime == other.mtime
            && self.spans == other.spans
    }
}

pub struct EmbeddingQueue {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    pending_batch: Vec<FileFragmentToEmbed>,
    executor: BackgroundExecutor,
    pending_batch_token_count: usize,
    finished_files_tx: channel::Sender<FileToEmbed>,
    finished_files_rx: channel::Receiver<FileToEmbed>,
}

#[derive(Clone)]
pub struct FileFragmentToEmbed {
    file: Arc<Mutex<FileToEmbed>>,
    span_range: Range<usize>,
}

impl EmbeddingQueue {
    pub fn new(
        embedding_provider: Arc<dyn EmbeddingProvider>,
        executor: BackgroundExecutor,
    ) -> Self {
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
        if file.spans.is_empty() {
            self.finished_files_tx.try_send(file).unwrap();
            return;
        }

        let file = Arc::new(Mutex::new(file));

        self.pending_batch.push(FileFragmentToEmbed {
            file: file.clone(),
            span_range: 0..0,
        });

        let mut fragment_range = &mut self.pending_batch.last_mut().unwrap().span_range;
        for (ix, span) in file.lock().spans.iter().enumerate() {
            let span_token_count = if span.embedding.is_none() {
                span.token_count
            } else {
                0
            };

            let next_token_count = self.pending_batch_token_count + span_token_count;
            if next_token_count > self.embedding_provider.max_tokens_per_batch() {
                let range_end = fragment_range.end;
                self.flush();
                self.pending_batch.push(FileFragmentToEmbed {
                    file: file.clone(),
                    span_range: range_end..range_end,
                });
                fragment_range = &mut self.pending_batch.last_mut().unwrap().span_range;
            }

            fragment_range.end = ix + 1;
            self.pending_batch_token_count += span_token_count;
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

        self.executor
            .spawn(async move {
                let mut spans = Vec::new();
                for fragment in &batch {
                    let file = fragment.file.lock();
                    spans.extend(
                        file.spans[fragment.span_range.clone()]
                            .iter()
                            .filter(|d| d.embedding.is_none())
                            .map(|d| d.content.clone()),
                    );
                }

                // If spans is 0, just send the fragment to the finished files if its the last one.
                if spans.is_empty() {
                    for fragment in batch.clone() {
                        if let Some(file) = Arc::into_inner(fragment.file) {
                            finished_files_tx.try_send(file.into_inner()).unwrap();
                        }
                    }
                    return;
                };

                match embedding_provider.embed_batch(spans).await {
                    Ok(embeddings) => {
                        let mut embeddings = embeddings.into_iter();
                        for fragment in batch {
                            for span in &mut fragment.file.lock().spans[fragment.span_range.clone()]
                                .iter_mut()
                                .filter(|d| d.embedding.is_none())
                            {
                                if let Some(embedding) = embeddings.next() {
                                    span.embedding = Some(embedding);
                                } else {
                                    log::error!("number of embeddings != number of documents");
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
