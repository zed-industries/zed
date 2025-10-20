use std::{
    io::{BufRead, BufReader},
    path::Path,
    pin::pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use anyhow::Context;
use collections::HashSet;
use fs::Fs;
use futures::{SinkExt, StreamExt, select_biased, stream::FuturesOrdered};
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use language::{Buffer, BufferSnapshot};
use parking_lot::Mutex;
use postage::oneshot;
use rpc::{AnyProtoClient, proto};
use smol::{
    channel::{Receiver, Sender, bounded, unbounded},
    future::FutureExt,
};

use text::BufferId;
use util::{ResultExt, maybe};
use worktree::{Entry, ProjectEntryId, Snapshot, Worktree};

use crate::{
    Project, ProjectItem, ProjectPath, RemotelyCreatedModels,
    buffer_store::BufferStore,
    search::{SearchQuery, SearchResult},
    worktree_store::WorktreeStore,
};

pub struct Search {
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    limit: usize,
    kind: SearchKind,
}

enum SearchKind {
    Local {
        fs: Arc<dyn Fs>,
        worktrees: Vec<Entity<Worktree>>,
    },
    Remote {
        client: AnyProtoClient,
        remote_id: u64,
        models: Arc<Mutex<RemotelyCreatedModels>>,
    },
}

/// Represents results of project search and allows one to either obtain match positions OR
/// just the handles to buffers that may match the search.
#[must_use]
pub struct SearchResultsHandle {
    results: Receiver<SearchResult>,
    matching_buffers: Receiver<Entity<Buffer>>,
    trigger_search: Box<dyn FnOnce(&mut App) -> Task<()> + Send + Sync>,
}

impl SearchResultsHandle {
    pub fn results(self, cx: &mut App) -> Receiver<SearchResult> {
        (self.trigger_search)(cx).detach();
        self.results
    }
    pub fn matching_buffers(self, cx: &mut App) -> Receiver<Entity<Buffer>> {
        (self.trigger_search)(cx).detach();
        self.matching_buffers
    }
}

#[derive(Clone)]
enum FindSearchCandidates {
    Local {
        fs: Arc<dyn Fs>,
        /// Start off with all paths in project and filter them based on:
        /// - Include filters
        /// - Exclude filters
        /// - Only open buffers
        /// - Scan ignored files
        /// Put another way: filter out files that can't match (without looking at file contents)
        input_paths_rx: Receiver<InputPath>,
        /// After that, if the buffer is not yet loaded, we'll figure out if it contains at least one match
        /// based on disk contents of a buffer. This step is not performed for buffers we already have in memory.
        confirm_contents_will_match_tx: Sender<MatchingEntry>,
        confirm_contents_will_match_rx: Receiver<MatchingEntry>,
        /// Of those that contain at least one match (or are already in memory), look for rest of matches (and figure out their ranges).
        /// But wait - first, we need to go back to the main thread to open a buffer (& create an entity for it).
        get_buffer_for_full_scan_tx: Sender<ProjectPath>,
    },
    Remote,
}

impl Search {
    pub fn local(
        fs: Arc<dyn Fs>,
        buffer_store: Entity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,

        limit: usize,
        cx: &mut App,
    ) -> Self {
        let worktrees = worktree_store.read(cx).visible_worktrees(cx).collect();
        Self {
            kind: SearchKind::Local { fs, worktrees },
            buffer_store,
            worktree_store,
            limit,
        }
    }

    pub(crate) fn remote(
        buffer_store: Entity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        limit: usize,
        client_state: (AnyProtoClient, u64, Arc<Mutex<RemotelyCreatedModels>>),
    ) -> Self {
        Self {
            kind: SearchKind::Remote {
                client: client_state.0,
                remote_id: client_state.1,
                models: client_state.2,
            },
            buffer_store,
            worktree_store,
            limit,
        }
    }

    pub(crate) const MAX_SEARCH_RESULT_FILES: usize = 5_000;
    pub(crate) const MAX_SEARCH_RESULT_RANGES: usize = 10_000;
    /// Prepares a project search run. The result has to be used to specify whether you're interested in matching buffers
    /// or full search results.
    pub fn into_results(mut self, query: SearchQuery, cx: &mut App) -> SearchResultsHandle {
        let mut open_buffers = HashSet::default();
        let mut unnamed_buffers = Vec::new();
        const MAX_CONCURRENT_BUFFER_OPENS: usize = 64;
        let buffers = self.buffer_store.read(cx);
        for handle in buffers.buffers() {
            let buffer = handle.read(cx);
            if !buffers.is_searchable(&buffer.remote_id()) {
                continue;
            } else if let Some(entry_id) = buffer.entry_id(cx) {
                open_buffers.insert(entry_id);
            } else {
                self.limit -= self.limit.saturating_sub(1);
                unnamed_buffers.push(handle)
            };
        }
        let executor = cx.background_executor().clone();
        let (tx, rx) = unbounded();
        let (grab_buffer_snapshot_tx, grab_buffer_snapshot_rx) = unbounded();
        let matching_buffers = grab_buffer_snapshot_rx.clone();
        let trigger_search = Box::new(move |cx: &mut App| {
            cx.spawn(async move |cx| {
                for buffer in unnamed_buffers {
                    _ = grab_buffer_snapshot_tx.send(buffer).await;
                }

                let (find_all_matches_tx, find_all_matches_rx) =
                    bounded(MAX_CONCURRENT_BUFFER_OPENS);

                let (candidate_searcher, tasks) = match self.kind {
                    SearchKind::Local {
                        fs,
                        ref mut worktrees,
                    } => {
                        let (get_buffer_for_full_scan_tx, get_buffer_for_full_scan_rx) =
                            unbounded();
                        let (confirm_contents_will_match_tx, confirm_contents_will_match_rx) =
                            bounded(64);
                        let (sorted_search_results_tx, sorted_search_results_rx) = unbounded();

                        let (input_paths_tx, input_paths_rx) = unbounded();

                        let tasks = vec![
                            cx.spawn(Self::provide_search_paths(
                                std::mem::take(worktrees),
                                query.include_ignored(),
                                input_paths_tx,
                                sorted_search_results_tx,
                            ))
                            .boxed_local(),
                            Self::open_buffers(
                                &self.buffer_store,
                                get_buffer_for_full_scan_rx,
                                grab_buffer_snapshot_tx,
                                cx.clone(),
                            )
                            .boxed_local(),
                            cx.background_spawn(Self::maintain_sorted_search_results(
                                sorted_search_results_rx,
                                get_buffer_for_full_scan_tx.clone(),
                                self.limit,
                            ))
                            .boxed_local(),
                        ];
                        (
                            FindSearchCandidates::Local {
                                fs,
                                get_buffer_for_full_scan_tx,
                                confirm_contents_will_match_tx,
                                confirm_contents_will_match_rx,
                                input_paths_rx,
                            },
                            tasks,
                        )
                    }
                    SearchKind::Remote {
                        client,
                        remote_id,
                        models,
                    } => {
                        let request = client.request(proto::FindSearchCandidates {
                            project_id: remote_id,
                            query: Some(query.to_proto()),
                            limit: self.limit as _,
                        });
                        let Ok(guard) = cx.update(|cx| {
                            Project::retain_remotely_created_models_impl(
                                &models,
                                &self.buffer_store,
                                &self.worktree_store,
                                cx,
                            )
                        }) else {
                            return;
                        };
                        let buffer_store = self.buffer_store.downgrade();
                        let issue_remote_buffers_request = cx
                            .spawn(async move |cx| {
                                let _ = maybe!(async move {
                                    let response = request.await?;

                                    for buffer_id in response.buffer_ids {
                                        let buffer_id = BufferId::new(buffer_id)?;
                                        let buffer = buffer_store
                                            .update(cx, |buffer_store, cx| {
                                                buffer_store.wait_for_remote_buffer(buffer_id, cx)
                                            })?
                                            .await?;
                                        let _ = grab_buffer_snapshot_tx.send(buffer).await;
                                    }

                                    drop(guard);
                                    anyhow::Ok(())
                                })
                                .await
                                .log_err();
                            })
                            .boxed_local();
                        (
                            FindSearchCandidates::Remote,
                            vec![issue_remote_buffers_request],
                        )
                    }
                };

                let matches_count = AtomicUsize::new(0);
                let matched_buffer_count = AtomicUsize::new(0);

                let worker_pool = executor.scoped(|scope| {
                    let num_cpus = executor.num_cpus();

                    assert!(num_cpus > 0);
                    for _ in 0..executor.num_cpus() - 1 {
                        let worker = Worker {
                            query: &query,
                            open_buffers: &open_buffers,
                            matched_buffer_count: &matched_buffer_count,
                            matches_count: &matches_count,
                            candidates: candidate_searcher.clone(),
                            find_all_matches_rx: find_all_matches_rx.clone(),
                            publish_matches: tx.clone(),
                        };
                        scope.spawn(worker.run());
                    }
                    drop(tx);
                    drop(find_all_matches_rx);
                    drop(candidate_searcher);
                });

                let buffer_snapshots = Self::grab_buffer_snapshots(
                    grab_buffer_snapshot_rx,
                    find_all_matches_tx,
                    cx.clone(),
                );
                futures::future::join_all(
                    [worker_pool.boxed_local(), buffer_snapshots.boxed_local()]
                        .into_iter()
                        .chain(tasks),
                )
                .await;
            })
        });

        SearchResultsHandle {
            results: rx,
            matching_buffers,
            trigger_search,
        }
    }

    fn provide_search_paths(
        worktrees: Vec<Entity<Worktree>>,
        include_ignored: bool,
        tx: Sender<InputPath>,
        results: Sender<oneshot::Receiver<ProjectPath>>,
    ) -> impl AsyncFnOnce(&mut AsyncApp) {
        async move |cx| {
            _ = maybe!(async move {
                for worktree in worktrees {
                    let (mut snapshot, worktree_settings) = worktree
                        .read_with(cx, |this, _| {
                            Some((this.snapshot(), this.as_local()?.settings()))
                        })?
                        .context("The worktree is not local")?;
                    if include_ignored {
                        // Pre-fetch all of the ignored directories as they're going to be searched.
                        let mut entries_to_refresh = vec![];
                        for entry in snapshot.entries(include_ignored, 0) {
                            if entry.is_ignored && entry.kind.is_unloaded() {
                                if !worktree_settings.is_path_excluded(&entry.path) {
                                    entries_to_refresh.push(entry.path.clone());
                                }
                            }
                        }
                        let barrier = worktree.update(cx, |this, _| {
                            let local = this.as_local_mut()?;
                            let barrier = entries_to_refresh
                                .into_iter()
                                .map(|path| local.add_path_prefix_to_scan(path).into_future())
                                .collect::<Vec<_>>();
                            Some(barrier)
                        })?;
                        if let Some(barriers) = barrier {
                            futures::future::join_all(barriers).await;
                        }
                        snapshot = worktree.read_with(cx, |this, _| this.snapshot())?;
                    }
                    cx.background_executor()
                        .scoped(|scope| {
                            scope.spawn(async {
                                for entry in snapshot.files(include_ignored, 0) {
                                    let (should_scan_tx, should_scan_rx) = oneshot::channel();
                                    let Ok(_) = tx
                                        .send(InputPath {
                                            entry: entry.clone(),
                                            snapshot: snapshot.clone(),
                                            should_scan_tx,
                                        })
                                        .await
                                    else {
                                        return;
                                    };
                                    if results.send(should_scan_rx).await.is_err() {
                                        return;
                                    };
                                }
                            })
                        })
                        .await;
                }
                anyhow::Ok(())
            })
            .await;
        }
    }

    async fn maintain_sorted_search_results(
        rx: Receiver<oneshot::Receiver<ProjectPath>>,
        paths_for_full_scan: Sender<ProjectPath>,
        limit: usize,
    ) {
        let mut rx = pin!(rx);
        let mut matched = 0;
        while let Some(mut next_path_result) = rx.next().await {
            let Some(successful_path) = next_path_result.next().await else {
                // This math did not produce a match, hence skip it.
                continue;
            };
            if paths_for_full_scan.send(successful_path).await.is_err() {
                return;
            };
            matched += 1;
            if matched >= limit {
                break;
            }
        }
    }

    /// Background workers cannot open buffers by themselves, hence main thread will do it on their behalf.
    async fn open_buffers(
        buffer_store: &Entity<BufferStore>,
        rx: Receiver<ProjectPath>,
        find_all_matches_tx: Sender<Entity<Buffer>>,
        mut cx: AsyncApp,
    ) {
        let mut rx = pin!(rx.ready_chunks(64));
        _ = maybe!(async move {
            while let Some(requested_paths) = rx.next().await {
                let mut buffers = buffer_store.update(&mut cx, |this, cx| {
                    requested_paths
                        .into_iter()
                        .map(|path| this.open_buffer(path, cx))
                        .collect::<FuturesOrdered<_>>()
                })?;

                while let Some(buffer) = buffers.next().await {
                    if let Some(buffer) = buffer.log_err() {
                        find_all_matches_tx.send(buffer).await?;
                    }
                }
            }
            Result::<_, anyhow::Error>::Ok(())
        })
        .await;
    }

    async fn grab_buffer_snapshots(
        rx: Receiver<Entity<Buffer>>,
        find_all_matches_tx: Sender<(Entity<Buffer>, BufferSnapshot)>,
        mut cx: AsyncApp,
    ) {
        _ = maybe!(async move {
            while let Ok(buffer) = rx.recv().await {
                let snapshot = buffer.read_with(&mut cx, |this, _| this.snapshot())?;
                find_all_matches_tx.send((buffer, snapshot)).await?;
            }
            Result::<_, anyhow::Error>::Ok(())
        })
        .await;
    }
}

struct Worker<'search> {
    query: &'search SearchQuery,
    matched_buffer_count: &'search AtomicUsize,
    matches_count: &'search AtomicUsize,
    open_buffers: &'search HashSet<ProjectEntryId>,
    candidates: FindSearchCandidates,
    /// Ok, we're back in background: run full scan & find all matches in a given buffer snapshot.
    find_all_matches_rx: Receiver<(Entity<Buffer>, BufferSnapshot)>,
    /// Cool, we have results; let's share them with the world.
    publish_matches: Sender<SearchResult>,
}

impl Worker<'_> {
    async fn run(mut self) {
        let (
            input_paths_rx,
            confirm_contents_will_match_rx,
            mut confirm_contents_will_match_tx,
            mut get_buffer_for_full_scan_tx,
            fs,
        ) = match self.candidates {
            FindSearchCandidates::Local {
                fs,
                input_paths_rx,
                confirm_contents_will_match_rx,
                confirm_contents_will_match_tx,
                get_buffer_for_full_scan_tx,
            } => (
                input_paths_rx,
                confirm_contents_will_match_rx,
                confirm_contents_will_match_tx,
                get_buffer_for_full_scan_tx,
                Some(fs),
            ),
            FindSearchCandidates::Remote => (
                unbounded().1,
                unbounded().1,
                unbounded().0,
                unbounded().0,
                None,
            ),
        };
        let mut find_all_matches = pin!(self.find_all_matches_rx.fuse());
        let mut find_first_match = pin!(confirm_contents_will_match_rx.fuse());
        let mut scan_path = pin!(input_paths_rx.fuse());

        loop {
            let handler = RequestHandler {
                query: self.query,
                open_entries: &self.open_buffers,
                fs: fs.as_deref(),
                matched_buffer_count: self.matched_buffer_count,
                matches_count: self.matches_count,
                confirm_contents_will_match_tx: &confirm_contents_will_match_tx,
                get_buffer_for_full_scan_tx: &get_buffer_for_full_scan_tx,
                publish_matches: &self.publish_matches,
            };
            // Whenever we notice that some step of a pipeline is closed, we don't want to close subsequent
            // steps straight away. Another worker might be about to produce a value that will
            // be pushed there, thus we'll replace current worker's pipe with a dummy one.
            // That way, we'll only ever close a next-stage channel when ALL workers do so.
            select_biased! {
                find_all_matches = find_all_matches.next() => {

                    if self.publish_matches.is_closed() {
                        break;
                    }
                    let Some(matches) = find_all_matches else {
                        self.publish_matches = bounded(1).0;
                        continue;
                    };
                    let result = handler.handle_find_all_matches(matches).await;
                    if let Some(_should_bail) = result {

                        self.publish_matches = bounded(1).0;
                        continue;
                    }
                },
                find_first_match = find_first_match.next() => {
                    if let Some(buffer_with_at_least_one_match) = find_first_match {
                        handler.handle_find_first_match(buffer_with_at_least_one_match).await;
                    } else {
                        get_buffer_for_full_scan_tx = bounded(1).0;
                    }

                },
                scan_path = scan_path.next() => {
                    if let Some(path_to_scan) = scan_path {
                        handler.handle_scan_path(path_to_scan).await;
                    } else {
                        // If we're the last worker to notice that this is not producing values, close the upstream.
                        confirm_contents_will_match_tx = bounded(1).0;
                    }

                 }
                 complete => {
                     break
                },

            }
        }
    }
}

struct RequestHandler<'worker> {
    query: &'worker SearchQuery,
    fs: Option<&'worker dyn Fs>,
    open_entries: &'worker HashSet<ProjectEntryId>,
    matched_buffer_count: &'worker AtomicUsize,
    matches_count: &'worker AtomicUsize,

    confirm_contents_will_match_tx: &'worker Sender<MatchingEntry>,
    get_buffer_for_full_scan_tx: &'worker Sender<ProjectPath>,
    publish_matches: &'worker Sender<SearchResult>,
}

struct LimitReached;

impl RequestHandler<'_> {
    async fn handle_find_all_matches(
        &self,
        (buffer, snapshot): (Entity<Buffer>, BufferSnapshot),
    ) -> Option<LimitReached> {
        let ranges = self
            .query
            .search(&snapshot, None)
            .await
            .iter()
            .map(|range| snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end))
            .collect::<Vec<_>>();

        let matched_ranges = ranges.len();
        if self.matched_buffer_count.fetch_add(1, Ordering::Release)
            > Search::MAX_SEARCH_RESULT_FILES
            || self
                .matches_count
                .fetch_add(matched_ranges, Ordering::Release)
                > Search::MAX_SEARCH_RESULT_RANGES
        {
            _ = self.publish_matches.send(SearchResult::LimitReached).await;
            Some(LimitReached)
        } else {
            _ = self
                .publish_matches
                .send(SearchResult::Buffer { buffer, ranges })
                .await;
            None
        }
    }
    async fn handle_find_first_match(&self, mut entry: MatchingEntry) {
        _=maybe!(async move {
            let abs_path = entry.worktree_root.join(entry.path.path.as_std_path());
            let Some(file) = self.fs.context("Trying to query filesystem in remote project search")?.open_sync(&abs_path).await.log_err() else {
                return anyhow::Ok(());
            };

            let mut file = BufReader::new(file);
            let file_start = file.fill_buf()?;

            if let Err(Some(starting_position)) =
            std::str::from_utf8(file_start).map_err(|e| e.error_len())
            {
                // Before attempting to match the file content, throw away files that have invalid UTF-8 sequences early on;
                // That way we can still match files in a streaming fashion without having look at "obviously binary" files.
                log::debug!(
                    "Invalid UTF-8 sequence in file {abs_path:?} at byte position {starting_position}"
                );
                return Ok(());
            }

            if self.query.detect(file).unwrap_or(false) {
                // Yes, we should scan the whole file.
                entry.should_scan_tx.send(entry.path).await?;
            }
            Ok(())
        }).await;
    }

    async fn handle_scan_path(&self, req: InputPath) {
        _ = maybe!(async move {
            let InputPath {
                entry,

                snapshot,
                should_scan_tx,
            } = req;

            if entry.is_fifo || !entry.is_file() {
                return Ok(());
            }

            if self.query.filters_path() {
                let matched_path = if self.query.match_full_paths() {
                    let mut full_path = snapshot.root_name().as_std_path().to_owned();
                    full_path.push(entry.path.as_std_path());
                    self.query.match_path(&full_path)
                } else {
                    self.query.match_path(entry.path.as_std_path())
                };
                if !matched_path {
                    return Ok(());
                }
            }

            if self.open_entries.contains(&entry.id) {
                // The buffer is already in memory and that's the version we want to scan;
                // hence skip the dilly-dally and look for all matches straight away.
                self.get_buffer_for_full_scan_tx
                    .send(ProjectPath {
                        worktree_id: snapshot.id(),
                        path: entry.path.clone(),
                    })
                    .await?;
            } else {
                self.confirm_contents_will_match_tx
                    .send(MatchingEntry {
                        should_scan_tx: should_scan_tx,
                        worktree_root: snapshot.abs_path().clone(),
                        path: ProjectPath {
                            worktree_id: snapshot.id(),
                            path: entry.path.clone(),
                        },
                    })
                    .await?;
            }

            anyhow::Ok(())
        })
        .await;
    }
}

struct InputPath {
    entry: Entry,
    snapshot: Snapshot,
    should_scan_tx: oneshot::Sender<ProjectPath>,
}

struct MatchingEntry {
    worktree_root: Arc<Path>,
    path: ProjectPath,
    should_scan_tx: oneshot::Sender<ProjectPath>,
}
