use crate::{
    embedding::{EmbeddingProvider, TextToEmbed},
    summary_index::FileSummary,
    worktree_index::{WorktreeIndex, WorktreeIndexHandle},
};
use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use fs::Fs;
use futures::{stream::StreamExt, FutureExt};
use gpui::{
    AppContext, Entity, EntityId, EventEmitter, Model, ModelContext, Subscription, Task, WeakModel,
};
use language::LanguageRegistry;
use log;
use project::{Project, Worktree, WorktreeId};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{
    cmp::Ordering,
    future::Future,
    num::NonZeroUsize,
    ops::{Range, RangeInclusive},
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;

#[derive(Debug)]
pub struct SearchResult {
    pub worktree: Model<Worktree>,
    pub path: Arc<Path>,
    pub range: Range<usize>,
    pub score: f32,
}

pub struct LoadedSearchResult {
    pub path: Arc<Path>,
    pub range: Range<usize>,
    pub full_path: PathBuf,
    pub file_content: String,
    pub row_range: RangeInclusive<u32>,
}

pub struct WorktreeSearchResult {
    pub worktree_id: WorktreeId,
    pub path: Arc<Path>,
    pub range: Range<usize>,
    pub score: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Idle,
    Loading,
    Scanning { remaining_count: NonZeroUsize },
}

pub struct ProjectIndex {
    db_connection: heed::Env,
    project: WeakModel<Project>,
    worktree_indices: HashMap<EntityId, WorktreeIndexHandle>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    last_status: Status,
    status_tx: channel::Sender<()>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    _maintain_status: Task<()>,
    _subscription: Subscription,
}

impl ProjectIndex {
    pub fn new(
        project: Model<Project>,
        db_connection: heed::Env,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let language_registry = project.read(cx).languages().clone();
        let fs = project.read(cx).fs().clone();
        let (status_tx, mut status_rx) = channel::unbounded();
        let mut this = ProjectIndex {
            db_connection,
            project: project.downgrade(),
            worktree_indices: HashMap::default(),
            language_registry,
            fs,
            status_tx,
            last_status: Status::Idle,
            embedding_provider,
            _subscription: cx.subscribe(&project, Self::handle_project_event),
            _maintain_status: cx.spawn(|this, mut cx| async move {
                while status_rx.next().await.is_some() {
                    if this
                        .update(&mut cx, |this, cx| this.update_status(cx))
                        .is_err()
                    {
                        break;
                    }
                }
            }),
        };
        this.update_worktree_indices(cx);
        this
    }

    pub fn status(&self) -> Status {
        self.last_status
    }

    pub fn project(&self) -> WeakModel<Project> {
        self.project.clone()
    }

    pub fn fs(&self) -> Arc<dyn Fs> {
        self.fs.clone()
    }

    fn handle_project_event(
        &mut self,
        _: Model<Project>,
        event: &project::Event,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            project::Event::WorktreeAdded | project::Event::WorktreeRemoved(_) => {
                self.update_worktree_indices(cx);
            }
            _ => {}
        }
    }

    fn update_worktree_indices(&mut self, cx: &mut ModelContext<Self>) {
        let Some(project) = self.project.upgrade() else {
            return;
        };

        let worktrees = project
            .read(cx)
            .visible_worktrees(cx)
            .filter_map(|worktree| {
                if worktree.read(cx).is_local() {
                    Some((worktree.entity_id(), worktree))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        self.worktree_indices
            .retain(|worktree_id, _| worktrees.contains_key(worktree_id));
        for (worktree_id, worktree) in worktrees {
            self.worktree_indices.entry(worktree_id).or_insert_with(|| {
                let worktree_index = WorktreeIndex::load(
                    worktree.clone(),
                    self.db_connection.clone(),
                    self.language_registry.clone(),
                    self.fs.clone(),
                    self.status_tx.clone(),
                    self.embedding_provider.clone(),
                    cx,
                );

                let load_worktree = cx.spawn(|this, mut cx| async move {
                    let result = match worktree_index.await {
                        Ok(worktree_index) => {
                            this.update(&mut cx, |this, _| {
                                this.worktree_indices.insert(
                                    worktree_id,
                                    WorktreeIndexHandle::Loaded {
                                        index: worktree_index.clone(),
                                    },
                                );
                            })?;
                            Ok(worktree_index)
                        }
                        Err(error) => {
                            this.update(&mut cx, |this, _cx| {
                                this.worktree_indices.remove(&worktree_id)
                            })?;
                            Err(Arc::new(error))
                        }
                    };

                    this.update(&mut cx, |this, cx| this.update_status(cx))?;

                    result
                });

                WorktreeIndexHandle::Loading {
                    index: load_worktree.shared(),
                }
            });
        }

        self.update_status(cx);
    }

    fn update_status(&mut self, cx: &mut ModelContext<Self>) {
        let mut indexing_count = 0;
        let mut any_loading = false;

        for index in self.worktree_indices.values_mut() {
            match index {
                WorktreeIndexHandle::Loading { .. } => {
                    any_loading = true;
                    break;
                }
                WorktreeIndexHandle::Loaded { index, .. } => {
                    indexing_count += index.read(cx).entry_ids_being_indexed().len();
                }
            }
        }

        let status = if any_loading {
            Status::Loading
        } else if let Some(remaining_count) = NonZeroUsize::new(indexing_count) {
            Status::Scanning { remaining_count }
        } else {
            Status::Idle
        };

        if status != self.last_status {
            self.last_status = status;
            cx.emit(status);
        }
    }

    pub fn search(
        &self,
        query: String,
        limit: usize,
        cx: &AppContext,
    ) -> Task<Result<Vec<SearchResult>>> {
        let (chunks_tx, chunks_rx) = channel::bounded(1024);
        let mut worktree_scan_tasks = Vec::new();
        for worktree_index in self.worktree_indices.values() {
            let worktree_index = worktree_index.clone();
            let chunks_tx = chunks_tx.clone();
            worktree_scan_tasks.push(cx.spawn(|cx| async move {
                let index = match worktree_index {
                    WorktreeIndexHandle::Loading { index } => {
                        index.clone().await.map_err(|error| anyhow!(error))?
                    }
                    WorktreeIndexHandle::Loaded { index } => index.clone(),
                };

                index
                    .read_with(&cx, |index, cx| {
                        let worktree_id = index.worktree().read(cx).id();
                        let db_connection = index.db_connection().clone();
                        let db = *index.embedding_index().db();
                        cx.background_executor().spawn(async move {
                            let txn = db_connection
                                .read_txn()
                                .context("failed to create read transaction")?;
                            let db_entries = db.iter(&txn).context("failed to iterate database")?;
                            for db_entry in db_entries {
                                let (_key, db_embedded_file) = db_entry?;
                                for chunk in db_embedded_file.chunks {
                                    chunks_tx
                                        .send((worktree_id, db_embedded_file.path.clone(), chunk))
                                        .await?;
                                }
                            }
                            anyhow::Ok(())
                        })
                    })?
                    .await
            }));
        }
        drop(chunks_tx);

        let project = self.project.clone();
        let embedding_provider = self.embedding_provider.clone();
        cx.spawn(|cx| async move {
            #[cfg(debug_assertions)]
            let embedding_query_start = std::time::Instant::now();
            log::info!("Searching for {query}");

            let query_embeddings = embedding_provider
                .embed(&[TextToEmbed::new(&query)])
                .await?;
            let query_embedding = query_embeddings
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("no embedding for query"))?;

            let mut results_by_worker = Vec::new();
            for _ in 0..cx.background_executor().num_cpus() {
                results_by_worker.push(Vec::<WorktreeSearchResult>::new());
            }

            #[cfg(debug_assertions)]
            let search_start = std::time::Instant::now();

            cx.background_executor()
                .scoped(|cx| {
                    for results in results_by_worker.iter_mut() {
                        cx.spawn(async {
                            while let Ok((worktree_id, path, chunk)) = chunks_rx.recv().await {
                                let score = chunk.embedding.similarity(&query_embedding);
                                let ix = match results.binary_search_by(|probe| {
                                    score.partial_cmp(&probe.score).unwrap_or(Ordering::Equal)
                                }) {
                                    Ok(ix) | Err(ix) => ix,
                                };
                                results.insert(
                                    ix,
                                    WorktreeSearchResult {
                                        worktree_id,
                                        path: path.clone(),
                                        range: chunk.chunk.range.clone(),
                                        score,
                                    },
                                );
                                results.truncate(limit);
                            }
                        });
                    }
                })
                .await;

            for scan_task in futures::future::join_all(worktree_scan_tasks).await {
                scan_task.log_err();
            }

            project.read_with(&cx, |project, cx| {
                let mut search_results = Vec::with_capacity(results_by_worker.len() * limit);
                for worker_results in results_by_worker {
                    search_results.extend(worker_results.into_iter().filter_map(|result| {
                        Some(SearchResult {
                            worktree: project.worktree_for_id(result.worktree_id, cx)?,
                            path: result.path,
                            range: result.range,
                            score: result.score,
                        })
                    }));
                }
                search_results.sort_unstable_by(|a, b| {
                    b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal)
                });
                search_results.truncate(limit);

                #[cfg(debug_assertions)]
                {
                    let search_elapsed = search_start.elapsed();
                    log::debug!(
                        "searched {} entries in {:?}",
                        search_results.len(),
                        search_elapsed
                    );
                    let embedding_query_elapsed = embedding_query_start.elapsed();
                    log::debug!("embedding query took {:?}", embedding_query_elapsed);
                }

                search_results
            })
        })
    }

    #[cfg(test)]
    pub fn path_count(&self, cx: &AppContext) -> Result<u64> {
        let mut result = 0;
        for worktree_index in self.worktree_indices.values() {
            if let WorktreeIndexHandle::Loaded { index, .. } = worktree_index {
                result += index.read(cx).path_count()?;
            }
        }
        Ok(result)
    }

    pub(crate) fn worktree_index(
        &self,
        worktree_id: WorktreeId,
        cx: &AppContext,
    ) -> Option<Model<WorktreeIndex>> {
        for index in self.worktree_indices.values() {
            if let WorktreeIndexHandle::Loaded { index, .. } = index {
                if index.read(cx).worktree().read(cx).id() == worktree_id {
                    return Some(index.clone());
                }
            }
        }
        None
    }

    pub(crate) fn worktree_indices(&self, cx: &AppContext) -> Vec<Model<WorktreeIndex>> {
        let mut result = self
            .worktree_indices
            .values()
            .filter_map(|index| {
                if let WorktreeIndexHandle::Loaded { index, .. } = index {
                    Some(index.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        result.sort_by_key(|index| index.read(cx).worktree().read(cx).id());
        result
    }

    pub fn all_summaries(&self, cx: &AppContext) -> Task<Result<Vec<FileSummary>>> {
        let (summaries_tx, summaries_rx) = channel::bounded(1024);
        let mut worktree_scan_tasks = Vec::new();
        for worktree_index in self.worktree_indices.values() {
            let worktree_index = worktree_index.clone();
            let summaries_tx: channel::Sender<(String, String)> = summaries_tx.clone();
            worktree_scan_tasks.push(cx.spawn(|cx| async move {
                let index = match worktree_index {
                    WorktreeIndexHandle::Loading { index } => {
                        index.clone().await.map_err(|error| anyhow!(error))?
                    }
                    WorktreeIndexHandle::Loaded { index } => index.clone(),
                };

                index
                    .read_with(&cx, |index, cx| {
                        let db_connection = index.db_connection().clone();
                        let summary_index = index.summary_index();
                        let file_digest_db = summary_index.file_digest_db();
                        let summary_db = summary_index.summary_db();

                        cx.background_executor().spawn(async move {
                            let txn = db_connection
                                .read_txn()
                                .context("failed to create db read transaction")?;
                            let db_entries = file_digest_db
                                .iter(&txn)
                                .context("failed to iterate database")?;
                            for db_entry in db_entries {
                                let (file_path, db_file) = db_entry?;

                                match summary_db.get(&txn, &db_file.digest) {
                                    Ok(opt_summary) => {
                                        // Currently, we only use summaries we already have. If the file hasn't been
                                        // summarized yet, then we skip it and don't include it in the inferred context.
                                        // If we want to do just-in-time summarization, this would be the place to do it!
                                        if let Some(summary) = opt_summary {
                                            summaries_tx
                                                .send((file_path.to_string(), summary.to_string()))
                                                .await?;
                                        } else {
                                            log::warn!("No summary found for {:?}", &db_file);
                                        }
                                    }
                                    Err(err) => {
                                        log::error!(
                                            "Error reading from summary database: {:?}",
                                            err
                                        );
                                    }
                                }
                            }
                            anyhow::Ok(())
                        })
                    })?
                    .await
            }));
        }
        drop(summaries_tx);

        let project = self.project.clone();
        cx.spawn(|cx| async move {
            let mut results_by_worker = Vec::new();
            for _ in 0..cx.background_executor().num_cpus() {
                results_by_worker.push(Vec::<FileSummary>::new());
            }

            cx.background_executor()
                .scoped(|cx| {
                    for results in results_by_worker.iter_mut() {
                        cx.spawn(async {
                            while let Ok((filename, summary)) = summaries_rx.recv().await {
                                results.push(FileSummary { filename, summary });
                            }
                        });
                    }
                })
                .await;

            for scan_task in futures::future::join_all(worktree_scan_tasks).await {
                scan_task.log_err();
            }

            project.read_with(&cx, |_project, _cx| {
                results_by_worker.into_iter().flatten().collect()
            })
        })
    }

    /// Empty out the backlogs of all the worktrees in the project
    pub fn flush_summary_backlogs(&self, cx: &AppContext) -> impl Future<Output = ()> {
        let flush_start = std::time::Instant::now();

        futures::future::join_all(self.worktree_indices.values().map(|worktree_index| {
            let worktree_index = worktree_index.clone();

            cx.spawn(|cx| async move {
                let index = match worktree_index {
                    WorktreeIndexHandle::Loading { index } => {
                        index.clone().await.map_err(|error| anyhow!(error))?
                    }
                    WorktreeIndexHandle::Loaded { index } => index.clone(),
                };
                let worktree_abs_path =
                    cx.update(|cx| index.read(cx).worktree().read(cx).abs_path())?;

                index
                    .read_with(&cx, |index, cx| {
                        cx.background_executor()
                            .spawn(index.summary_index().flush_backlog(worktree_abs_path, cx))
                    })?
                    .await
            })
        }))
        .map(move |results| {
            // Log any errors, but don't block the user. These summaries are supposed to
            // improve quality by providing extra context, but they aren't hard requirements!
            for result in results {
                if let Err(err) = result {
                    log::error!("Error flushing summary backlog: {:?}", err);
                }
            }

            log::info!("Summary backlog flushed in {:?}", flush_start.elapsed());
        })
    }

    pub fn remaining_summaries(&self, cx: &mut ModelContext<Self>) -> usize {
        self.worktree_indices(cx)
            .iter()
            .map(|index| index.read(cx).summary_index().backlog_len())
            .sum()
    }
}

impl EventEmitter<Status> for ProjectIndex {}
