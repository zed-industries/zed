use std::{
    cell::LazyCell,
    collections::BTreeSet,
    io::{BufRead, BufReader},
    ops::Range,
    path::{Path, PathBuf},
    pin::pin,
    sync::Arc,
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
use util::{ResultExt, maybe, paths::compare_rel_paths, rel_path::RelPath};
use worktree::{Entry, ProjectEntryId, Snapshot, Worktree, WorktreeSettings};

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

/// Represents search setup, before it is actually kicked off with Search::into_results
enum SearchKind {
    /// Search for candidates by inspecting file contents on file system, avoiding loading the buffer unless we know that a given file contains a match.
    Local {
        fs: Arc<dyn Fs>,
        worktrees: Vec<Entity<Worktree>>,
    },
    /// Query remote host for candidates. As of writing, the host runs a local search in "buffers with matches only" mode.
    Remote {
        client: AnyProtoClient,
        remote_id: u64,
        models: Arc<Mutex<RemotelyCreatedModels>>,
    },
    /// Run search against a known set of candidates. Even when working with a remote host, this won't round-trip to host.
    OpenBuffersOnly,
}

/// Represents results of project search and allows one to either obtain match positions OR
/// just the handles to buffers that may match the search. Grabbing the handles is cheaper than obtaining full match positions, because in that case we'll look for
/// at most one match in each file.
#[must_use]
pub struct SearchResultsHandle {
    results: Receiver<SearchResult>,
    matching_buffers: Receiver<Entity<Buffer>>,
    trigger_search: Box<dyn FnOnce(&mut App) -> Task<()> + Send + Sync>,
}

pub struct SearchResults<T> {
    pub _task_handle: Task<()>,
    pub rx: Receiver<T>,
}
impl SearchResultsHandle {
    pub fn results(self, cx: &mut App) -> SearchResults<SearchResult> {
        SearchResults {
            _task_handle: (self.trigger_search)(cx),
            rx: self.results,
        }
    }
    pub fn matching_buffers(self, cx: &mut App) -> SearchResults<Entity<Buffer>> {
        SearchResults {
            _task_handle: (self.trigger_search)(cx),
            rx: self.matching_buffers,
        }
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
    },
    Remote,
    OpenBuffersOnly,
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
    pub(crate) fn open_buffers_only(
        buffer_store: Entity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        limit: usize,
    ) -> Self {
        Self {
            kind: SearchKind::OpenBuffersOnly,
            buffer_store,
            worktree_store,
            limit,
        }
    }

    pub(crate) const MAX_SEARCH_RESULT_FILES: usize = 5_000;
    pub(crate) const MAX_SEARCH_RESULT_RANGES: usize = 10_000;
    /// Prepares a project search run. The resulting [`SearchResultsHandle`] has to be used to specify whether you're interested in matching buffers
    /// or full search results.
    pub fn into_handle(mut self, query: SearchQuery, cx: &mut App) -> SearchResultsHandle {
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
                self.limit = self.limit.saturating_sub(1);
                unnamed_buffers.push(handle)
            };
        }
        let open_buffers = Arc::new(open_buffers);
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
                let query = Arc::new(query);
                let (candidate_searcher, tasks) = match self.kind {
                    SearchKind::OpenBuffersOnly => {
                        let open_buffers = cx.update(|cx| self.all_loaded_buffers(&query, cx));
                        let fill_requests = cx
                            .background_spawn(async move {
                                for buffer in open_buffers {
                                    if let Err(_) = grab_buffer_snapshot_tx.send(buffer).await {
                                        return;
                                    }
                                }
                            })
                            .boxed_local();
                        (FindSearchCandidates::OpenBuffersOnly, vec![fill_requests])
                    }
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
                                query.clone(),
                                input_paths_tx,
                                sorted_search_results_tx,
                            ))
                            .boxed_local(),
                            Self::open_buffers(
                                self.buffer_store,
                                get_buffer_for_full_scan_rx,
                                grab_buffer_snapshot_tx,
                                cx.clone(),
                            )
                            .boxed_local(),
                            cx.background_spawn(Self::maintain_sorted_search_results(
                                sorted_search_results_rx,
                                get_buffer_for_full_scan_tx,
                                self.limit,
                            ))
                            .boxed_local(),
                        ];
                        (
                            FindSearchCandidates::Local {
                                fs,
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
                        let weak_buffer_store = self.buffer_store.downgrade();
                        let buffer_store = self.buffer_store;
                        let guard = cx.update(|cx| {
                            Project::retain_remotely_created_models_impl(
                                &models,
                                &buffer_store,
                                &self.worktree_store,
                                cx,
                            )
                        });

                        let issue_remote_buffers_request = cx
                            .spawn(async move |cx| {
                                let _ = maybe!(async move {
                                    let response = request.await?;
                                    for buffer_id in response.buffer_ids {
                                        let buffer_id = BufferId::new(buffer_id)?;
                                        let buffer = weak_buffer_store
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

                let should_find_all_matches = !tx.is_closed();

                let _executor = executor.clone();
                let worker_pool = executor.spawn(async move {
                    let num_cpus = _executor.num_cpus();

                    assert!(num_cpus > 0);
                    _executor
                        .scoped(|scope| {
                            for _ in 0..num_cpus - 1 {
                                let worker = Worker {
                                    query: query.clone(),
                                    open_buffers: open_buffers.clone(),
                                    candidates: candidate_searcher.clone(),
                                    find_all_matches_rx: find_all_matches_rx.clone(),
                                };
                                scope.spawn(worker.run());
                            }

                            drop(find_all_matches_rx);
                            drop(candidate_searcher);
                        })
                        .await;
                });

                let (sorted_matches_tx, sorted_matches_rx) = unbounded();
                // The caller of `into_handle` decides whether they're interested in all matches (files that matched + all matching ranges) or
                // just the files. *They are using the same stream as the guts of the project search do*.
                // This means that we cannot grab values off of that stream unless it's strictly needed for making a progress in project search.
                //
                // Grabbing buffer snapshots is only necessary when we're looking for all matches. If the caller decided that they're not interested
                // in all matches, running that task unconditionally would hinder caller's ability to observe all matching file paths.
                let buffer_snapshots = if should_find_all_matches {
                    Some(
                        Self::grab_buffer_snapshots(
                            grab_buffer_snapshot_rx,
                            find_all_matches_tx,
                            sorted_matches_tx,
                            cx.clone(),
                        )
                        .boxed_local(),
                    )
                } else {
                    drop(find_all_matches_tx);

                    None
                };
                let ensure_matches_are_reported_in_order = if should_find_all_matches {
                    Some(
                        Self::ensure_matched_ranges_are_reported_in_order(sorted_matches_rx, tx)
                            .boxed_local(),
                    )
                } else {
                    drop(tx);
                    None
                };

                futures::future::join_all(
                    [worker_pool.boxed_local()]
                        .into_iter()
                        .chain(buffer_snapshots)
                        .chain(ensure_matches_are_reported_in_order)
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
        query: Arc<SearchQuery>,
        tx: Sender<InputPath>,
        results: Sender<oneshot::Receiver<ProjectPath>>,
    ) -> impl AsyncFnOnce(&mut AsyncApp) {
        async move |cx| {
            _ = maybe!(async move {
                let gitignored_tracker = PathInclusionMatcher::new(query.clone());
                let include_ignored = query.include_ignored();
                for worktree in worktrees {
                    let (mut snapshot, worktree_settings) = worktree
                        .read_with(cx, |this, _| {
                            Some((this.snapshot(), this.as_local()?.settings()))
                        })
                        .context("The worktree is not local")?;
                    if query.include_ignored() {
                        // Pre-fetch all of the ignored directories as they're going to be searched.
                        let mut entries_to_refresh = vec![];

                        for entry in snapshot.entries(query.include_ignored(), 0) {
                            if gitignored_tracker.should_scan_gitignored_dir(
                                entry,
                                &snapshot,
                                &worktree_settings,
                            ) {
                                entries_to_refresh.push(entry.path.clone());
                            }
                        }
                        let barrier = worktree.update(cx, |this, _| {
                            let local = this.as_local_mut()?;
                            let barrier = entries_to_refresh
                                .into_iter()
                                .map(|path| local.add_path_prefix_to_scan(path).into_future())
                                .collect::<Vec<_>>();
                            Some(barrier)
                        });
                        if let Some(barriers) = barrier {
                            futures::future::join_all(barriers).await;
                        }
                        snapshot = worktree.read_with(cx, |this, _| this.snapshot());
                    }
                    let tx = tx.clone();
                    let results = results.clone();

                    cx.background_executor()
                        .spawn(async move {
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
                // This file did not produce a match, hence skip it.
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
        buffer_store: Entity<BufferStore>,
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
                });

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
        find_all_matches_tx: Sender<(
            Entity<Buffer>,
            BufferSnapshot,
            oneshot::Sender<(Entity<Buffer>, Vec<Range<language::Anchor>>)>,
        )>,
        results: Sender<oneshot::Receiver<(Entity<Buffer>, Vec<Range<language::Anchor>>)>>,
        mut cx: AsyncApp,
    ) {
        _ = maybe!(async move {
            while let Ok(buffer) = rx.recv().await {
                let snapshot = buffer.read_with(&mut cx, |this, _| this.snapshot());
                let (tx, rx) = oneshot::channel();
                find_all_matches_tx.send((buffer, snapshot, tx)).await?;
                results.send(rx).await?;
            }
            debug_assert!(rx.is_empty());
            Result::<_, anyhow::Error>::Ok(())
        })
        .await;
    }

    async fn ensure_matched_ranges_are_reported_in_order(
        rx: Receiver<oneshot::Receiver<(Entity<Buffer>, Vec<Range<language::Anchor>>)>>,
        tx: Sender<SearchResult>,
    ) {
        use postage::stream::Stream;
        _ = maybe!(async move {
            let mut matched_buffers = 0;
            let mut matches = 0;
            while let Ok(mut next_buffer_matches) = rx.recv().await {
                let Some((buffer, ranges)) = next_buffer_matches.recv().await else {
                    continue;
                };

                if matched_buffers > Search::MAX_SEARCH_RESULT_FILES
                    || matches > Search::MAX_SEARCH_RESULT_RANGES
                {
                    _ = tx.send(SearchResult::LimitReached).await;
                    break;
                }
                matched_buffers += 1;
                matches += ranges.len();

                _ = tx.send(SearchResult::Buffer { buffer, ranges }).await?;
            }
            anyhow::Ok(())
        })
        .await;
    }

    fn all_loaded_buffers(&self, search_query: &SearchQuery, cx: &App) -> Vec<Entity<Buffer>> {
        let worktree_store = self.worktree_store.read(cx);
        let mut buffers = search_query
            .buffers()
            .into_iter()
            .flatten()
            .filter(|buffer| {
                let b = buffer.read(cx);
                if let Some(file) = b.file() {
                    if !search_query.match_path(file.path()) {
                        return false;
                    }
                    if !search_query.include_ignored()
                        && let Some(entry) = b
                            .entry_id(cx)
                            .and_then(|entry_id| worktree_store.entry_for_id(entry_id, cx))
                        && entry.is_ignored
                    {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect::<Vec<_>>();
        buffers.sort_by(|a, b| {
            let a = a.read(cx);
            let b = b.read(cx);
            match (a.file(), b.file()) {
                (None, None) => a.remote_id().cmp(&b.remote_id()),
                (None, Some(_)) => std::cmp::Ordering::Less,
                (Some(_), None) => std::cmp::Ordering::Greater,
                (Some(a), Some(b)) => compare_rel_paths((a.path(), true), (b.path(), true)),
            }
        });

        buffers
    }
}

struct Worker {
    query: Arc<SearchQuery>,
    open_buffers: Arc<HashSet<ProjectEntryId>>,
    candidates: FindSearchCandidates,
    /// Ok, we're back in background: run full scan & find all matches in a given buffer snapshot.
    /// Then, when you're done, share them via the channel you were given.
    find_all_matches_rx: Receiver<(
        Entity<Buffer>,
        BufferSnapshot,
        oneshot::Sender<(Entity<Buffer>, Vec<Range<language::Anchor>>)>,
    )>,
}

impl Worker {
    async fn run(self) {
        let (
            input_paths_rx,
            confirm_contents_will_match_rx,
            mut confirm_contents_will_match_tx,
            fs,
        ) = match self.candidates {
            FindSearchCandidates::Local {
                fs,
                input_paths_rx,
                confirm_contents_will_match_rx,
                confirm_contents_will_match_tx,
            } => (
                input_paths_rx,
                confirm_contents_will_match_rx,
                confirm_contents_will_match_tx,
                Some(fs),
            ),
            FindSearchCandidates::Remote | FindSearchCandidates::OpenBuffersOnly => {
                (unbounded().1, unbounded().1, unbounded().0, None)
            }
        };
        // WorkerA: grabs a request for "find all matches in file/a" <- takes 5 minutes
        // right after: WorkerB: grabs a request for "find all matches in file/b" <- takes 5 seconds
        let mut find_all_matches = pin!(self.find_all_matches_rx.fuse());
        let mut find_first_match = pin!(confirm_contents_will_match_rx.fuse());
        let mut scan_path = pin!(input_paths_rx.fuse());

        loop {
            let handler = RequestHandler {
                query: &self.query,
                open_entries: &self.open_buffers,
                fs: fs.as_deref(),
                confirm_contents_will_match_tx: &confirm_contents_will_match_tx,
            };
            // Whenever we notice that some step of a pipeline is closed, we don't want to close subsequent
            // steps straight away. Another worker might be about to produce a value that will
            // be pushed there, thus we'll replace current worker's pipe with a dummy one.
            // That way, we'll only ever close a next-stage channel when ALL workers do so.
            select_biased! {
                find_all_matches = find_all_matches.next() => {
                    let Some(matches) = find_all_matches else {
                        continue;
                    };
                    handler.handle_find_all_matches(matches).await;
                },
                find_first_match = find_first_match.next() => {
                    if let Some(buffer_with_at_least_one_match) = find_first_match {
                        handler.handle_find_first_match(buffer_with_at_least_one_match).await;
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
    confirm_contents_will_match_tx: &'worker Sender<MatchingEntry>,
}

impl RequestHandler<'_> {
    async fn handle_find_all_matches(
        &self,
        (buffer, snapshot, mut report_matches): (
            Entity<Buffer>,
            BufferSnapshot,
            oneshot::Sender<(Entity<Buffer>, Vec<Range<language::Anchor>>)>,
        ),
    ) {
        let ranges = self
            .query
            .search(&snapshot, None)
            .await
            .iter()
            .map(|range| snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end))
            .collect::<Vec<_>>();

        _ = report_matches.send((buffer, ranges)).await;
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

            if self.query.detect(file).await.unwrap_or(false) {
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
                mut should_scan_tx,
            } = req;

            if entry.is_fifo || !entry.is_file() {
                return Ok(());
            }

            if self.query.filters_path() {
                let matched_path = if self.query.match_full_paths() {
                    let mut full_path = snapshot.root_name().to_owned();
                    full_path.push(&entry.path);
                    self.query.match_path(&full_path)
                } else {
                    self.query.match_path(&entry.path)
                };
                if !matched_path {
                    return Ok(());
                }
            }

            if self.open_entries.contains(&entry.id) {
                // The buffer is already in memory and that's the version we want to scan;
                // hence skip the dilly-dally and look for all matches straight away.
                should_scan_tx
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

/// This struct encapsulates the logic to decide whether a given gitignored directory should be
/// scanned based on include/exclude patterns of a search query (as include/exclude parameters may match paths inside it).
/// It is kind-of doing an inverse of glob. Given a glob pattern like `src/**/` and a parent path like `src`, we need to decide whether the parent
/// may contain glob hits.
struct PathInclusionMatcher {
    included: BTreeSet<PathBuf>,
    query: Arc<SearchQuery>,
}

impl PathInclusionMatcher {
    fn new(query: Arc<SearchQuery>) -> Self {
        let mut included = BTreeSet::new();
        // To do an inverse glob match, we split each glob into it's prefix and the glob part.
        // For example, `src/**/*.rs` becomes `src/` and `**/*.rs`. The glob part gets dropped.
        // Then, when checking whether a given directory should be scanned, we check whether it is a non-empty substring of any glob prefix.
        if query.filters_path() {
            included.extend(
                query
                    .files_to_include()
                    .sources()
                    .flat_map(|glob| Some(wax::Glob::new(glob).ok()?.partition().0)),
            );
        }
        Self { included, query }
    }

    fn should_scan_gitignored_dir(
        &self,
        entry: &Entry,
        snapshot: &Snapshot,
        worktree_settings: &WorktreeSettings,
    ) -> bool {
        if !entry.is_ignored || !entry.kind.is_unloaded() {
            return false;
        }
        if !self.query.include_ignored() {
            return false;
        }
        if worktree_settings.is_path_excluded(&entry.path) {
            return false;
        }
        if !self.query.filters_path() {
            return true;
        }

        let as_abs_path = LazyCell::new(move || snapshot.absolutize(&entry.path));
        let entry_path = &entry.path;
        // 3. Check Exclusions (Pruning)
        // If the current path is a child of an excluded path, we stop.
        let is_excluded = self.path_is_definitely_excluded(&entry_path, snapshot);

        if is_excluded {
            return false;
        }

        // 4. Check Inclusions (Traversal)
        if self.included.is_empty() {
            return true;
        }

        // We scan if the current path is a descendant of an include prefix
        // OR if the current path is an ancestor of an include prefix (we need to go deeper to find it).
        let is_included = self.included.iter().any(|prefix| {
            let (prefix_matches_entry, entry_matches_prefix) = if prefix.is_absolute() {
                (
                    prefix.starts_with(&**as_abs_path),
                    as_abs_path.starts_with(prefix),
                )
            } else {
                RelPath::new(prefix, snapshot.path_style()).map_or((false, false), |prefix| {
                    (
                        prefix.starts_with(entry_path),
                        entry_path.starts_with(&prefix),
                    )
                })
            };

            // Logic:
            // 1. entry_matches_prefix: We are inside the target zone (e.g. glob: src/, current: src/lib/). Keep scanning.
            // 2. prefix_matches_entry: We are above the target zone (e.g. glob: src/foo/, current: src/). Keep scanning to reach foo.
            prefix_matches_entry || entry_matches_prefix
        });

        is_included
    }
    fn path_is_definitely_excluded(&self, path: &RelPath, snapshot: &Snapshot) -> bool {
        if !self.query.files_to_exclude().sources().next().is_none() {
            let mut path = if self.query.match_full_paths() {
                let mut full_path = snapshot.root_name().to_owned();
                full_path.push(path);
                full_path
            } else {
                path.to_owned()
            };
            loop {
                if self.query.files_to_exclude().is_match(&path) {
                    return true;
                } else if !path.pop() {
                    return false;
                }
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use serde_json::json;
    use settings::Settings;
    use util::{
        path,
        paths::{PathMatcher, PathStyle},
        rel_path::RelPath,
    };
    use worktree::{Entry, EntryKind, WorktreeSettings};

    use crate::{
        Project, project_search::PathInclusionMatcher, project_tests::init_test,
        search::SearchQuery,
    };

    #[gpui::test]
    async fn test_path_inclusion_matcher(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                ".gitignore": "src/data/\n",
                "src": {
                    "data": {
                        "main.csv": "field_1,field_2,field_3",
                    },
                    "lib": {
                        "main.txt": "Are you familiar with fields?",
                    },
                },
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        let (worktree_settings, worktree_snapshot) = worktree.update(cx, |worktree, cx| {
            let settings_location = worktree.settings_location(cx);
            return (
                WorktreeSettings::get(Some(settings_location), cx).clone(),
                worktree.snapshot(),
            );
        });

        // Manually create a test entry for the gitignored directory since it won't
        // be loaded by the worktree
        let entry = Entry {
            id: ProjectEntryId::from_proto(1),
            kind: EntryKind::UnloadedDir,
            path: Arc::from(RelPath::unix(Path::new("src/data")).unwrap()),
            inode: 0,
            mtime: None,
            canonical_path: None,
            is_ignored: true,
            is_hidden: false,
            is_always_included: false,
            is_external: false,
            is_private: false,
            size: 0,
            char_bag: Default::default(),
            is_fifo: false,
        };

        // 1. Test searching for `field`, including ignored files without any
        // inclusion and exclusion filters.
        let include_ignored = true;
        let files_to_include = PathMatcher::default();
        let files_to_exclude = PathMatcher::default();
        let match_full_paths = false;
        let search_query = SearchQuery::text(
            "field",
            false,
            false,
            include_ignored,
            files_to_include,
            files_to_exclude,
            match_full_paths,
            None,
        )
        .unwrap();

        let path_matcher = PathInclusionMatcher::new(Arc::new(search_query));
        assert!(path_matcher.should_scan_gitignored_dir(
            &entry,
            &worktree_snapshot,
            &worktree_settings
        ));

        // 2. Test searching for `field`, including ignored files but updating
        // `files_to_include` to only include files under `src/lib`.
        let include_ignored = true;
        let files_to_include = PathMatcher::new(vec!["src/lib"], PathStyle::Posix).unwrap();
        let files_to_exclude = PathMatcher::default();
        let match_full_paths = false;
        let search_query = SearchQuery::text(
            "field",
            false,
            false,
            include_ignored,
            files_to_include,
            files_to_exclude,
            match_full_paths,
            None,
        )
        .unwrap();

        let path_matcher = PathInclusionMatcher::new(Arc::new(search_query));
        assert!(!path_matcher.should_scan_gitignored_dir(
            &entry,
            &worktree_snapshot,
            &worktree_settings
        ));
    }
}
