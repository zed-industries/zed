use std::{
    cell::LazyCell,
    collections::BTreeSet,
    future::Future,
    io::{BufRead, BufReader, Cursor, ErrorKind, Read},
    ops::Range,
    path::{Path, PathBuf},
    pin::pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use anyhow::Context;
use async_channel::{Receiver, Sender, bounded, unbounded};
use collections::HashSet;
use fs::Fs;
use futures::FutureExt as _;
use futures::{SinkExt, StreamExt, select_biased, stream::FuturesOrdered};
use gpui::{App, AppContext, AsyncApp, BackgroundExecutor, Entity, Priority, Task};
use language::{Buffer, BufferSnapshot, Point};
use parking_lot::Mutex;
use postage::oneshot;
use rpc::{AnyProtoClient, proto};
use smol::future::yield_now;

use language::ByteContent;
use util::{ResultExt, maybe, rel_path::RelPath};
use worktree::{
    Entry, ProjectEntryId, Snapshot, Worktree, WorktreeSettings, decode_byte_header,
    decode_file_text,
};

use crate::{
    Project, ProjectItem, ProjectPath, RemotelyCreatedModels,
    buffer_store::BufferStore,
    search::{MatchPositionHint, SearchOmission, SearchOmissionReason, SearchQuery, SearchResult},
    worktree_store::WorktreeStore,
};

pub struct Search {
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    limit: usize,
    kind: SearchKind,
    include_private_omissions: bool,
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
    matching_buffers: Receiver<(Entity<Buffer>, MatchPositionHint)>,
    omissions: Receiver<Vec<SearchOmission>>,
    omissions_status: SearchOmissionsStatus,
    trigger_search: Box<dyn FnOnce(&mut App) -> Task<()> + Send + Sync>,
}

pub struct SearchResults<T> {
    pub task_handle: Task<()>,
    pub rx: Receiver<T>,
    pub omissions: Receiver<Vec<SearchOmission>>,
    pub omissions_status: SearchOmissionsStatus,
}

#[derive(Clone, Default)]
pub struct SearchOmissionsStatus(Arc<AtomicBool>);

impl SearchOmissionsStatus {
    pub fn is_complete(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    pub(crate) fn mark_complete(&self) {
        self.0.store(true, Ordering::Release);
    }
}

impl SearchResultsHandle {
    pub fn results(self, cx: &mut App) -> SearchResults<SearchResult> {
        SearchResults {
            task_handle: (self.trigger_search)(cx),
            rx: self.results,
            omissions: self.omissions,
            omissions_status: self.omissions_status,
        }
    }
    pub fn matching_buffers(
        self,
        cx: &mut App,
    ) -> SearchResults<(Entity<Buffer>, MatchPositionHint)> {
        SearchResults {
            task_handle: (self.trigger_search)(cx),
            rx: self.matching_buffers,
            omissions: self.omissions,
            omissions_status: self.omissions_status,
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
        let mut worktrees = worktree_store
            .read(cx)
            .visible_worktrees(cx)
            .collect::<Vec<_>>();
        worktrees.sort_by_key(|worktree| worktree.read(cx).id());
        Self {
            kind: SearchKind::Local { fs, worktrees },
            include_private_omissions: true,
            buffer_store,
            worktree_store,
            limit,
        }
    }

    pub fn include_private_omissions(mut self, include_private_omissions: bool) -> Self {
        self.include_private_omissions = include_private_omissions;
        self
    }

    pub(crate) fn remote(
        buffer_store: Entity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        limit: usize,
        client_state: (AnyProtoClient, u64, Arc<Mutex<RemotelyCreatedModels>>),
    ) -> Self {
        Self {
            include_private_omissions: true,
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
            include_private_omissions: true,
            buffer_store,
            worktree_store,
            limit,
        }
    }

    pub(crate) const MAX_SEARCH_RESULT_FILES: usize = 5_000;
    pub const MAX_SEARCH_RESULT_RANGES: usize = 10_000;
    /// Prepares a project search run. The resulting [`SearchResultsHandle`] has to be used to specify whether you're interested in matching buffers
    /// or full search results.
    pub fn into_handle(mut self, query: SearchQuery, cx: &mut App) -> SearchResultsHandle {
        let mut open_buffers = HashSet::default();
        let mut unnamed_buffers = Vec::new();
        let mut entryless_file_buffers = Vec::new();
        const MAX_CONCURRENT_BUFFER_OPENS: usize = 64;
        let searches_all_unnamed_buffers = !matches!(self.kind, SearchKind::OpenBuffersOnly);
        let buffers = self.buffer_store.read(cx);
        for handle in buffers.buffers() {
            let buffer = handle.read(cx);
            if !buffers.is_searchable(&buffer.remote_id()) {
                continue;
            } else if buffer
                .file()
                .is_some_and(|file| file.disk_state().is_deleted())
            {
                continue;
            } else if let Some(entry_id) = buffer.entry_id(cx) {
                open_buffers.insert(entry_id);
            } else if searches_all_unnamed_buffers {
                match (&self.kind, buffer.file()) {
                    (SearchKind::Local { .. }, Some(file)) => {
                        self.limit = self.limit.saturating_sub(1);
                        let sort_key = (file.worktree_id(cx).to_proto(), file.path().clone());
                        entryless_file_buffers.push((sort_key, handle));
                    }
                    (SearchKind::Remote { .. }, _) => {}
                    _ => {
                        self.limit = self.limit.saturating_sub(1);
                        unnamed_buffers.push(handle);
                    }
                }
            };
        }
        unnamed_buffers.sort_by_cached_key(|buffer| path_key_sort_key(buffer, cx));
        entryless_file_buffers.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b));
        let open_buffers = Arc::new(open_buffers);
        let executor = cx.background_executor().clone();
        let (tx, rx) = unbounded();
        let (omissions_tx, omissions) = unbounded();
        let omissions_status = SearchOmissionsStatus::default();
        let search_omissions_status = omissions_status.clone();
        let (grab_buffer_snapshot_tx, grab_buffer_snapshot_rx) =
            unbounded::<(Entity<Buffer>, MatchPositionHint)>();
        let matching_buffers = grab_buffer_snapshot_rx.clone();
        let trigger_search = Box::new(move |cx: &mut App| {
            cx.spawn(async move |cx| {
                for buffer in unnamed_buffers {
                    _ = grab_buffer_snapshot_tx
                        .send((buffer, MatchPositionHint::default()))
                        .await;
                }

                let (find_all_matches_tx, find_all_matches_rx) =
                    bounded(MAX_CONCURRENT_BUFFER_OPENS);
                let query = Arc::new(query);
                let (candidate_searcher, tasks) = match self.kind {
                    SearchKind::OpenBuffersOnly => {
                        search_omissions_status.mark_complete();
                        drop(omissions_tx);
                        let open_buffers = cx.update(|cx| self.all_loaded_buffers(&query, cx));
                        let fill_requests = cx
                            .background_spawn(async move {
                                for buffer in open_buffers {
                                    if let Err(_) = grab_buffer_snapshot_tx
                                        .send((buffer, MatchPositionHint::default()))
                                        .await
                                    {
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
                                tx.clone(),
                                (omissions_tx, search_omissions_status),
                                self.include_private_omissions,
                            ))
                            .boxed_local(),
                            Self::open_buffers(
                                self.buffer_store,
                                get_buffer_for_full_scan_rx,
                                grab_buffer_snapshot_tx,
                                entryless_file_buffers,
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
                        let report_omissions = !omissions_tx.is_closed();
                        let (handle, rx) = self.buffer_store.update(cx, |this, _| {
                            this.register_project_search_result_handle(
                                omissions_tx,
                                search_omissions_status,
                            )
                        });
                        let unregister_search = util::defer({
                            let buffer_store = self.buffer_store.clone();
                            let mut cx = cx.clone();
                            move || {
                                buffer_store.update(&mut cx, |this, _| {
                                    this.unregister_project_search_result_handle(handle);
                                });
                            }
                        });

                        let cancel_ongoing_search = util::defer({
                            let client = client.clone();
                            move || {
                                _ = client.send(proto::FindSearchCandidatesCancelled {
                                    project_id: remote_id,
                                    handle,
                                });
                            }
                        });
                        let request = client.request(proto::FindSearchCandidates {
                            project_id: remote_id,
                            query: Some(query.to_proto()),
                            limit: self.limit as _,
                            handle,
                            report_omissions,
                            include_private_omissions: self.include_private_omissions,
                        });

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
                                let _unregister_search = unregister_search;
                                let _ = maybe!(async move {
                                    request.await?;

                                    let (buffer_tx, buffer_rx) = bounded(24);

                                    let wait_for_remote_buffers = cx.spawn(async move |cx| {
                                        while let Ok(buffer_id) = rx.recv().await {
                                            let buffer =
                                                buffer_store.update(cx, |buffer_store, cx| {
                                                    buffer_store
                                                        .wait_for_remote_buffer(buffer_id, cx)
                                                });
                                            buffer_tx.send(buffer).await?;
                                        }
                                        anyhow::Ok(())
                                    });

                                    let forward_buffers = cx.background_spawn(async move {
                                        while let Ok(buffer) = buffer_rx.recv().await {
                                            let _ = grab_buffer_snapshot_tx
                                                .send((buffer.await?, MatchPositionHint::default()))
                                                .await;
                                        }
                                        anyhow::Ok(())
                                    });
                                    let (left, right) = futures::future::join(
                                        wait_for_remote_buffers,
                                        forward_buffers,
                                    )
                                    .await;
                                    left?;
                                    right?;

                                    drop(guard);
                                    cancel_ongoing_search.abort();
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
                            let worker_count = (num_cpus - 1).max(1);
                            for _ in 0..worker_count {
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
            omissions,
            omissions_status,
            trigger_search,
        }
    }

    fn provide_search_paths(
        worktrees: Vec<Entity<Worktree>>,
        query: Arc<SearchQuery>,
        tx: Sender<InputPath>,
        results: Sender<oneshot::Receiver<(ProjectPath, MatchPositionHint)>>,
        results_tx: Sender<SearchResult>,
        omissions: (Sender<Vec<SearchOmission>>, SearchOmissionsStatus),
        include_private_omissions: bool,
    ) -> impl AsyncFnOnce(&mut AsyncApp) {
        let (omissions_tx, omissions_status) = omissions;
        async move |cx| {
            _ = maybe!(async move {
                let gitignored_tracker = PathInclusionMatcher::new(query.clone());
                let include_ignored = query.include_ignored();
                for worktree in worktrees {
                    let scan_complete = worktree.read_with(cx, |worktree, _| {
                        worktree.as_local().map(|local| local.scan_complete())
                    });
                    if let Some(scan_complete) = scan_complete {
                        let mut scan_complete = pin!(scan_complete);
                        if scan_complete.as_mut().now_or_never().is_none() {
                            _ = results_tx.send(SearchResult::WaitingForScan).await;
                            scan_complete.await;
                            _ = results_tx.send(SearchResult::Searching).await;
                        }
                    }

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
                    let omissions_tx = omissions_tx.clone();
                    let snapshot = Arc::new(snapshot);

                    cx.background_executor()
                        .spawn(async move {
                            if !omissions_tx.is_closed()
                                && let Some(omissions) = collect_search_omissions(
                                    &snapshot,
                                    &worktree_settings,
                                    include_ignored,
                                    include_private_omissions,
                                    &omissions_tx,
                                )
                                .await
                                && omissions_tx.send(omissions).await.is_err()
                            {
                                omissions_tx.close();
                            }
                            drop(omissions_tx);
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
                if !omissions_tx.is_closed() {
                    omissions_status.mark_complete();
                }
                anyhow::Ok(())
            })
            .await;
        }
    }

    async fn maintain_sorted_search_results(
        rx: Receiver<oneshot::Receiver<(ProjectPath, MatchPositionHint)>>,
        paths_for_full_scan: Sender<(ProjectPath, MatchPositionHint)>,
        limit: usize,
    ) {
        let mut rx = pin!(rx);
        let mut matched = 0;
        while let Some(mut next_path_result) = rx.next().await {
            let Some((successful_path, line_hint)) = next_path_result.next().await else {
                // This file did not produce a match, hence skip it.
                continue;
            };
            if paths_for_full_scan
                .send((successful_path, line_hint))
                .await
                .is_err()
            {
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
        rx: Receiver<(ProjectPath, MatchPositionHint)>,
        find_all_matches_tx: Sender<(Entity<Buffer>, MatchPositionHint)>,
        sorted_entryless_file_buffers: Vec<((u64, Arc<RelPath>), Entity<Buffer>)>,
        mut cx: AsyncApp,
    ) {
        let mut entryless_file_buffers = sorted_entryless_file_buffers.into_iter().peekable();
        let mut rx = pin!(rx.ready_chunks(64));
        _ = maybe!(async move {
            while let Some(requested_paths) = rx.next().await {
                let line_hints: Vec<MatchPositionHint> =
                    requested_paths.iter().map(|(_, line)| *line).collect();
                let sort_keys: Vec<(u64, Arc<RelPath>)> = requested_paths
                    .iter()
                    .map(|(path, _)| (path.worktree_id.to_proto(), path.path.clone()))
                    .collect();
                let mut buffers = buffer_store.update(&mut cx, |this, cx| {
                    requested_paths
                        .into_iter()
                        .map(|(path, _)| this.open_buffer(path, cx))
                        .collect::<FuturesOrdered<_>>()
                });
                let mut line_hints = line_hints.into_iter();
                let mut sort_keys = sort_keys.into_iter();
                while let Some(buffer) = buffers.next().await {
                    let line_hint = line_hints.next().unwrap_or(MatchPositionHint::default());
                    if let Some(sort_key) = sort_keys.next() {
                        while let Some((_, entryless_buffer)) =
                            entryless_file_buffers.next_if(|(key, _)| *key < sort_key)
                        {
                            find_all_matches_tx
                                .send((entryless_buffer, MatchPositionHint::default()))
                                .await?;
                        }
                    }
                    if let Some(buffer) = buffer.log_err() {
                        find_all_matches_tx.send((buffer, line_hint)).await?;
                    }
                }
            }
            for (_, entryless_buffer) in entryless_file_buffers {
                find_all_matches_tx
                    .send((entryless_buffer, MatchPositionHint::default()))
                    .await?;
            }
            Result::<_, anyhow::Error>::Ok(())
        })
        .await;
    }

    async fn grab_buffer_snapshots(
        rx: Receiver<(Entity<Buffer>, MatchPositionHint)>,
        find_all_matches_tx: Sender<FindAllMatchesRequest>,
        results: Sender<oneshot::Receiver<(Entity<Buffer>, Vec<Range<language::Anchor>>)>>,
        mut cx: AsyncApp,
    ) {
        _ = maybe!(async move {
            while let Ok((buffer, line_hint)) = rx.recv().await {
                let snapshot = buffer.read_with(&mut cx, |this, _| this.snapshot());
                let (tx, rx) = oneshot::channel();
                find_all_matches_tx
                    .send(FindAllMatchesRequest {
                        buffer,
                        snapshot,
                        line_hint,
                        report_matches: tx,
                    })
                    .await?;
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
                    if file.disk_state().is_deleted() {
                        return false;
                    }
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
        buffers.sort_by_cached_key(|buffer| path_key_sort_key(buffer, cx));
        buffers.dedup_by_key(|buffer| buffer.entity_id());

        buffers
    }
}

pub async fn forward_search_omissions(
    omissions: Receiver<Vec<SearchOmission>>,
    omissions_status: SearchOmissionsStatus,
    client: AnyProtoClient,
    project_id: u64,
    peer_id: proto::PeerId,
    handle: u64,
) -> anyhow::Result<()> {
    forward_search_omissions_with(
        omissions,
        omissions_status,
        project_id,
        peer_id,
        handle,
        |chunk| client.request(chunk),
    )
    .await
}

pub(crate) const MAX_CONCURRENT_OMISSION_REQUESTS: usize = 16;

async fn forward_search_omissions_with<F>(
    omissions: Receiver<Vec<SearchOmission>>,
    omissions_status: SearchOmissionsStatus,
    project_id: u64,
    peer_id: proto::PeerId,
    handle: u64,
    mut send: impl FnMut(proto::FindSearchCandidatesChunk) -> F,
) -> anyhow::Result<()>
where
    F: Future<Output = anyhow::Result<proto::Ack>>,
{
    let mut requests = FuturesOrdered::<F>::new();
    let mut sequence = 0;
    let mut result = Ok(());
    'batches: while let Ok(omissions) = omissions.recv().await {
        for batch in omissions.chunks(256) {
            if requests.len() == MAX_CONCURRENT_OMISSION_REQUESTS
                && let Some(Err(error)) = requests.next().await
            {
                result = Err(error);
                break 'batches;
            }
            requests.push_back(send(proto::FindSearchCandidatesChunk {
                project_id,
                peer_id: Some(peer_id),
                handle,
                variant: Some(proto::find_search_candidates_chunk::Variant::Matches(
                    proto::FindSearchCandidatesMatches {
                        buffer_ids: Vec::new(),
                    },
                )),
                omissions: batch.iter().map(SearchOmission::to_proto).collect(),
                omissions_done: false,
                omissions_sequence: Some(sequence),
            }));
            sequence += 1;
        }
    }
    while let Some(response) = requests.next().await {
        if let Err(error) = response {
            if result.is_ok() {
                result = Err(error);
            } else {
                log::error!("Failed to forward search omissions: {error:#}");
            }
        }
    }
    result?;
    anyhow::ensure!(
        omissions_status.is_complete(),
        "Search omission report is incomplete"
    );
    send(proto::FindSearchCandidatesChunk {
        project_id,
        peer_id: Some(peer_id),
        handle,
        variant: Some(proto::find_search_candidates_chunk::Variant::Matches(
            proto::FindSearchCandidatesMatches {
                buffer_ids: Vec::new(),
            },
        )),
        omissions: Vec::new(),
        omissions_done: true,
        omissions_sequence: Some(sequence),
    })
    .await?;
    Ok(())
}

async fn collect_search_omissions(
    snapshot: &Snapshot,
    worktree_settings: &WorktreeSettings,
    include_ignored: bool,
    include_private_omissions: bool,
    sender: &Sender<Vec<SearchOmission>>,
) -> Option<Vec<SearchOmission>> {
    let mut omissions = Vec::new();
    let mut not_indexed_root: Option<Arc<RelPath>> = None;
    let mut gitignored_root: Option<Arc<RelPath>> = None;
    for (index, entry) in snapshot
        .search_omission_entries(include_ignored)
        .enumerate()
    {
        if index % 256 == 0 {
            yield_now().await;
        }
        if sender.is_closed() {
            return None;
        }
        let reason = if entry.is_ignored && !entry.is_always_included && !include_ignored {
            SearchOmissionReason::GitIgnored
        } else {
            SearchOmissionReason::NotIndexed
        };
        let previous_root = match reason {
            SearchOmissionReason::NotIndexed => &mut not_indexed_root,
            SearchOmissionReason::GitIgnored => &mut gitignored_root,
        };
        if previous_root
            .as_ref()
            .is_some_and(|root| entry.path.starts_with(root))
        {
            continue;
        }
        if !include_private_omissions
            && (entry.is_private || worktree_settings.is_path_private(&entry.path))
        {
            continue;
        }
        let mut path = entry.path.clone();
        if reason == SearchOmissionReason::GitIgnored {
            for ancestor in entry.path.ancestors().skip(1) {
                if let Some(ancestor) = snapshot.entry_for_path(ancestor)
                    && ancestor.is_dir()
                    && ancestor.is_ignored
                {
                    path = ancestor.path.clone();
                }
            }
        }
        *previous_root = Some(path.clone());
        omissions.push(SearchOmission {
            path: ProjectPath {
                worktree_id: snapshot.id(),
                path,
            },
            reason,
        });
    }
    if sender.is_closed() {
        return None;
    }
    omissions.sort_unstable_by(|left, right| {
        left.path
            .path
            .cmp(&right.path.path)
            .then(left.reason.cmp(&right.reason))
    });
    (!sender.is_closed()).then_some(omissions)
}

fn path_key_sort_key(
    buffer: &Entity<Buffer>,
    cx: &App,
) -> (Option<u64>, Option<Arc<RelPath>>, String) {
    let buffer = buffer.read(cx);
    match buffer.file() {
        Some(file) => (
            Some(file.worktree_id(cx).to_proto()),
            Some(file.path().clone()),
            String::new(),
        ),
        None => (None, None, buffer.remote_id().to_string()),
    }
}

struct Worker {
    query: Arc<SearchQuery>,
    open_buffers: Arc<HashSet<ProjectEntryId>>,
    candidates: FindSearchCandidates,
    /// Ok, we're back in background: run full scan & find all matches in a given buffer snapshot.
    /// Then, when you're done, share them via the channel you were given.
    find_all_matches_rx: Receiver<FindAllMatchesRequest>,
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
    async fn handle_find_all_matches(&self, request: FindAllMatchesRequest) {
        let FindAllMatchesRequest {
            buffer,
            snapshot,
            line_hint,
            mut report_matches,
        } = request;
        let range_offset = match line_hint {
            MatchPositionHint::Line(line_number) if line_number > 0 => {
                snapshot.point_to_offset(Point::new(line_number, 0))
            }
            MatchPositionHint::ByteOffset(offset) => offset,
            _ => 0,
        };

        let subrange = (range_offset > 0).then(|| range_offset..snapshot.len());
        let ranges = self
            .query
            .search(&snapshot, subrange)
            .await
            .iter()
            .map(|range| {
                snapshot.anchor_before(range.start + range_offset)
                    ..snapshot.anchor_after(range.end + range_offset)
            })
            .collect::<Vec<_>>();

        _ = report_matches.send((buffer, ranges)).await;
    }

    async fn handle_find_first_match(&self, mut entry: MatchingEntry) {
        async move {
            let abs_path = entry.worktree_root.join(entry.path.path.as_std_path());
            let fs = self
                .fs
                .context("Trying to query filesystem in remote project search")?;
            let Some(file) = fs.open_sync(&abs_path).await.log_err() else {
                return anyhow::Ok(());
            };

            let mut file = BufReader::new(file);
            let file_start = file.fill_buf()?;
            let (bom_encoding, byte_content) = decode_byte_header(file_start);
            if byte_content == ByteContent::Binary {
                log::debug!("Skipping binary file {abs_path:?}");
                return Ok(());
            }

            let is_plain_utf8 = bom_encoding.is_none()
                && byte_content == ByteContent::Unknown
                && is_utf8_prefix(file_start);

            let line_hint = if is_plain_utf8 {
                match self.query.detect(file).await {
                    Ok(line_hint) => line_hint,
                    Err(error)
                        if error
                            .downcast_ref::<std::io::Error>()
                            .is_some_and(|error| error.kind() == ErrorKind::InvalidData) =>
                    {
                        self.detect_in_decoded_file(fs, &abs_path).await?
                    }
                    Err(error) => return Err(error),
                }
            } else {
                self.detect_in_decoded_file(fs, &abs_path).await?
            };

            if let Some(line_hint) = line_hint {
                // Yes, we should scan the whole file.
                entry.should_scan_tx.send((entry.path, line_hint)).await?;
            }
            Ok(())
        }
        .await
        .ok();
    }

    async fn detect_in_decoded_file(
        &self,
        fs: &dyn Fs,
        abs_path: &Path,
    ) -> anyhow::Result<Option<MatchPositionHint>> {
        let (text, _encoding, _has_bom) = decode_file_text(fs, abs_path).await?;
        let reader: Box<dyn Read + Send + Sync> = Box::new(Cursor::new(text.into_bytes()));
        self.query.detect(BufReader::new(reader)).await
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
                    .send((
                        ProjectPath {
                            worktree_id: snapshot.id(),
                            path: entry.path.clone(),
                        },
                        MatchPositionHint::default(),
                    ))
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

fn is_utf8_prefix(bytes: &[u8]) -> bool {
    match std::str::from_utf8(bytes) {
        Ok(_) => true,
        Err(error) => error.error_len().is_none(),
    }
}

struct InputPath {
    entry: Entry,
    snapshot: Arc<Snapshot>,
    should_scan_tx: oneshot::Sender<(ProjectPath, MatchPositionHint)>,
}

struct MatchingEntry {
    worktree_root: Arc<Path>,
    path: ProjectPath,
    should_scan_tx: oneshot::Sender<(ProjectPath, MatchPositionHint)>,
}

struct FindAllMatchesRequest {
    buffer: Entity<Buffer>,
    snapshot: BufferSnapshot,
    line_hint: MatchPositionHint,
    report_matches: oneshot::Sender<(Entity<Buffer>, Vec<Range<language::Anchor>>)>,
}

/// This struct encapsulates the logic to decide whether a given gitignored directory should be
/// scanned based on include/exclude patterns of a search query (as include/exclude parameters may match paths inside it).
/// It is kind-of doing an inverse of glob. Given a glob pattern like `src/**/` and a parent path like `src`, we need to decide whether the parent
/// may contain glob hits.
pub struct PathInclusionMatcher {
    included: BTreeSet<PathBuf>,
    query: Arc<SearchQuery>,
}

impl PathInclusionMatcher {
    pub fn new(query: Arc<SearchQuery>) -> Self {
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

    pub fn should_scan_gitignored_dir(
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

type IsTerminating = bool;
/// Adaptive batcher that starts eager (small batches) and grows batch size
/// when items arrive quickly, reducing RPC overhead while preserving low latency
/// for slow streams.
pub struct AdaptiveBatcher<T> {
    items: Sender<T>,
    flush_batch: Sender<IsTerminating>,
    _batch_task: Task<()>,
}

impl<T: 'static + Send> AdaptiveBatcher<T> {
    pub fn new(cx: &BackgroundExecutor) -> (Self, Receiver<Vec<T>>) {
        let (items, rx) = unbounded();
        let (batch_tx, batch_rx) = unbounded();
        let (flush_batch_tx, flush_batch_rx) = unbounded();
        let flush_batch = flush_batch_tx.clone();
        let executor = cx.clone();
        let _batch_task = cx.spawn_with_priority(gpui::Priority::High, async move {
            let mut current_batch = vec![];
            let mut items_produced_so_far = 0_u64;

            let mut _schedule_flush_after_delay: Option<Task<()>> = None;
            let _time_elapsed_since_start_of_search = std::time::Instant::now();
            let mut flush = pin!(flush_batch_rx);
            let mut terminating = false;
            loop {
                select_biased! {
                    item = rx.recv().fuse() => {
                        match item {
                            Ok(new_item) => {
                                let is_fresh_batch = current_batch.is_empty();
                                items_produced_so_far += 1;
                                current_batch.push(new_item);
                                if is_fresh_batch {
                                    // Chosen arbitrarily based on some experimentation with plots.
                                    let desired_duration_ms = (20 * (items_produced_so_far + 2).ilog2() as u64).min(300);
                                    let desired_duration = Duration::from_millis(desired_duration_ms);
                                    let _executor = executor.clone();
                                    let _flush = flush_batch_tx.clone();
                                    let new_timer = executor.spawn_with_priority(Priority::High, async move {
                                        _executor.timer(desired_duration).await;
                                        _ = _flush.send(false).await;
                                    });
                                    _schedule_flush_after_delay = Some(new_timer);
                                }
                            }
                            Err(_) => {
                                // Items channel closed - send any remaining batch before exiting
                                if !current_batch.is_empty() {
                                    _ = batch_tx.send(std::mem::take(&mut current_batch)).await;
                                }
                                break;
                            }
                        }
                    }
                    should_break_afterwards = flush.next() => {
                        if !current_batch.is_empty() {
                            _ = batch_tx.send(std::mem::take(&mut current_batch)).await;
                            _schedule_flush_after_delay = None;
                        }
                        if should_break_afterwards.unwrap_or_default() {
                            terminating = true;
                        }
                    }
                    complete => {
                        break;
                    }
                }
                if terminating {
                    // Drain any remaining items before exiting
                    while let Ok(new_item) = rx.try_recv() {
                        current_batch.push(new_item);
                    }
                    if !current_batch.is_empty() {
                        _ = batch_tx.send(std::mem::take(&mut current_batch)).await;
                    }
                    break;
                }
            }
        });
        let this = Self {
            items,
            _batch_task,
            flush_batch,
        };
        (this, batch_rx)
    }

    pub async fn push(&self, item: T) {
        _ = self.items.send(item).await;
    }

    pub async fn flush(self) {
        _ = self.flush_batch.send(true).await;
        self._batch_task.await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use util::{
        path,
        paths::{PathMatcher, PathStyle},
    };
    use worktree::WorktreeId;

    #[gpui::test]
    async fn test_forward_search_omissions_limits_requests_and_orders_completion(
        cx: &mut TestAppContext,
    ) {
        let (sender, omissions) = unbounded();
        sender.send(test_omissions(8193)).await.unwrap();
        drop(sender);
        let status = SearchOmissionsStatus::default();
        status.mark_complete();
        let (requests_sender, requests) = unbounded();
        let task = cx.background_spawn(forward_search_omissions_with(
            omissions,
            status,
            1,
            proto::PeerId::default(),
            1,
            move |chunk| {
                let (sender, receiver) = unbounded();
                requests_sender.try_send((chunk, sender)).unwrap();
                async move { receiver.recv().await? }
            },
        ));
        for first_sequence in [0, 16] {
            let mut responses = Vec::new();
            for sequence in first_sequence..first_sequence + 16 {
                let (chunk, response) = requests.recv().await.unwrap();
                assert_eq!(chunk.omissions_sequence, Some(sequence));
                assert_eq!(chunk.omissions.len(), 256);
                assert!(!chunk.omissions_done);
                responses.push(response);
            }
            cx.run_until_parked();
            assert_eq!(
                requests.try_recv().err(),
                Some(async_channel::TryRecvError::Empty)
            );
            let first = responses.remove(0);
            for response in responses.into_iter().rev() {
                response.send(Ok(proto::Ack {})).await.unwrap();
            }
            cx.run_until_parked();
            assert_eq!(
                requests.try_recv().err(),
                Some(async_channel::TryRecvError::Empty)
            );
            first.send(Ok(proto::Ack {})).await.unwrap();
        }
        let (chunk, response) = requests.recv().await.unwrap();
        assert_eq!(chunk.omissions_sequence, Some(32));
        assert_eq!(chunk.omissions.len(), 1);
        assert!(!chunk.omissions_done);
        cx.run_until_parked();
        assert_eq!(
            requests.try_recv().err(),
            Some(async_channel::TryRecvError::Empty)
        );
        response.send(Ok(proto::Ack {})).await.unwrap();
        let (chunk, response) = requests.recv().await.unwrap();
        assert_eq!(chunk.omissions_sequence, Some(33));
        assert_eq!(chunk.omissions, Vec::new());
        assert!(chunk.omissions_done);
        response.send(Ok(proto::Ack {})).await.unwrap();
        task.await.unwrap();
    }

    #[gpui::test]
    async fn test_forward_search_omissions_drains_in_flight_requests_after_failure(
        cx: &mut TestAppContext,
    ) {
        let (sender, omissions) = unbounded();
        sender.send(test_omissions(8193)).await.unwrap();
        drop(sender);
        let status = SearchOmissionsStatus::default();
        status.mark_complete();
        let (requests_sender, requests) = unbounded();
        let task = cx.background_spawn(forward_search_omissions_with(
            omissions,
            status,
            1,
            proto::PeerId::default(),
            1,
            move |chunk| {
                let (sender, receiver) = unbounded();
                requests_sender.try_send((chunk, sender)).unwrap();
                async move { receiver.recv().await? }
            },
        ));
        let mut responses = Vec::new();
        for sequence in 0..16 {
            let (chunk, response) = requests.recv().await.unwrap();
            assert_eq!(chunk.omissions_sequence, Some(sequence));
            responses.push(response);
        }
        responses
            .remove(0)
            .send(Err(anyhow::anyhow!("Injected omission failure")))
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(
            requests.try_recv().err(),
            Some(async_channel::TryRecvError::Empty)
        );
        for response in responses {
            response.send(Ok(proto::Ack {})).await.unwrap();
        }
        assert_eq!(
            task.await.unwrap_err().to_string(),
            "Injected omission failure"
        );
        assert_eq!(
            requests.try_recv().err(),
            Some(async_channel::TryRecvError::Closed)
        );
    }

    #[gpui::test]
    async fn test_forward_search_omissions_requires_complete_source() {
        for complete in [false, true] {
            let (sender, omissions) = unbounded();
            drop(sender);
            let status = SearchOmissionsStatus::default();
            if complete {
                status.mark_complete();
            }
            let mut completions = Vec::new();
            let result = forward_search_omissions_with(
                omissions,
                status,
                1,
                proto::PeerId::default(),
                1,
                |chunk| {
                    completions.push(chunk.omissions_done);
                    async { Ok(proto::Ack {}) }
                },
            )
            .await;
            if complete {
                result.unwrap();
                assert_eq!(completions, vec![true]);
            } else {
                assert_eq!(
                    result.unwrap_err().to_string(),
                    "Search omission report is incomplete"
                );
                assert_eq!(completions, Vec::<bool>::new());
            }
        }
    }

    #[gpui::test]
    async fn test_collect_search_omissions_compacts_expanded_ignored_roots() {
        let (snapshot, settings) = omission_snapshot();
        let (sender, _receiver) = unbounded();
        let omissions = collect_search_omissions(&snapshot, &settings, false, false, &sender)
            .await
            .unwrap();
        assert_eq!(
            omissions
                .iter()
                .map(|omission| (omission.path.path.as_unix_str(), omission.reason))
                .collect::<Vec<_>>(),
            vec![
                ("ignored", SearchOmissionReason::GitIgnored),
                ("ignored/external", SearchOmissionReason::NotIndexed),
                ("ignored-", SearchOmissionReason::GitIgnored),
            ]
        );
        let omissions = collect_search_omissions(&snapshot, &settings, true, false, &sender)
            .await
            .unwrap();
        assert_eq!(
            omissions
                .iter()
                .map(|omission| (omission.path.path.as_unix_str(), omission.reason))
                .collect::<Vec<_>>(),
            vec![
                ("ignored/external", SearchOmissionReason::NotIndexed),
                ("ignored-", SearchOmissionReason::NotIndexed),
            ]
        );
    }

    #[gpui::test]
    async fn test_collect_search_omissions_cancels_after_traversal_started() {
        let (snapshot, settings) = omission_snapshot();
        let (sender, receiver) = unbounded();
        let mut collect = pin!(collect_search_omissions(
            &snapshot, &settings, false, true, &sender
        ));
        assert!(collect.as_mut().now_or_never().is_none());
        assert!(collect.as_mut().now_or_never().is_none());
        drop(receiver);
        assert_eq!(collect.await, None);
    }

    fn omission_snapshot() -> (Snapshot, WorktreeSettings) {
        let settings = WorktreeSettings {
            prevent_sharing_in_public_channels: false,
            file_scan_exclusions: PathMatcher::default(),
            file_scan_inclusions: PathMatcher::default(),
            parent_dir_scan_inclusions: PathMatcher::default(),
            scan_symlinks: settings::ScanSymlinksSetting::Always,
            file_scan_depth: None,
            private_files: PathMatcher::new(["secret"], PathStyle::Unix).unwrap(),
            hidden_files: PathMatcher::default(),
            read_only_files: PathMatcher::default(),
        };
        let mut snapshot = Snapshot::new(
            WorktreeId::from_proto(1),
            Arc::from(RelPath::from_unix_str("root").unwrap()),
            Arc::from(Path::new(path!("/root"))),
            PathStyle::Unix,
        );
        let mut entries = [
            ("ignored", false),
            ("ignored/external", true),
            ("ignored-", true),
            ("secret", true),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (path, unloaded))| proto::Entry {
            id: index as u64 + 1,
            path: path.to_owned(),
            is_dir: true,
            is_ignored: true,
            is_unloaded: unloaded,
            ..proto::Entry::default()
        })
        .collect::<Vec<_>>();
        entries.extend((0..4096).map(|index| proto::Entry {
            id: index + 5,
            path: format!("ignored/{index:04}.log"),
            is_ignored: true,
            ..proto::Entry::default()
        }));
        snapshot.apply_remote_update(
            proto::UpdateWorktree {
                root_name: "root".to_owned(),
                abs_path: path!("/root").to_owned(),
                updated_entries: entries,
                ..proto::UpdateWorktree::default()
            },
            &PathMatcher::new(["ignored/external"], PathStyle::Unix).unwrap(),
        );
        (snapshot, settings)
    }

    fn test_omissions(count: usize) -> Vec<SearchOmission> {
        (0..count)
            .map(|index| SearchOmission {
                path: ProjectPath {
                    worktree_id: WorktreeId::from_proto(1),
                    path: Arc::from(RelPath::from_unix_str(&format!("{index:05}.log")).unwrap()),
                },
                reason: SearchOmissionReason::GitIgnored,
            })
            .collect()
    }
}
