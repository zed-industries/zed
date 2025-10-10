use std::{
    ops::{ControlFlow, Range},
    path::Path,
    pin::{Pin, pin},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use futures::{FutureExt, StreamExt, future::BoxFuture, select_biased};
use gpui::{App, AsyncApp, Entity, WeakEntity};
use language::{Buffer, BufferSnapshot};
use postage::oneshot;
use smol::channel::{Receiver, Sender, bounded, unbounded};
use text::Anchor;
use util::{ResultExt, maybe};
use worktree::{Entry, Snapshot, WorktreeSettings};

use crate::{
    ProjectPath,
    buffer_store::BufferStore,
    search::{SearchQuery, SearchResult},
};

pub(crate) struct ProjectSearcher {
    buffer_store: WeakEntity<BufferStore>,
    pub(crate) snapshots: Vec<(Snapshot, WorktreeSettings)>,
}

const MAX_SEARCH_RESULT_FILES: usize = 5_000;
const MAX_SEARCH_RESULT_RANGES: usize = 10_000;

impl ProjectSearcher {
    pub(crate) fn search(self, query: SearchQuery, cx: &mut App) -> Receiver<SearchResult> {
        let executor = cx.background_executor().clone();
        let (tx, rx) = unbounded();
        cx.spawn(async move |cx| {
            const MAX_CONCURRENT_BUFFER_OPENS: usize = 64;
            let (find_all_matches_tx, find_all_matches_rx) = bounded(MAX_CONCURRENT_BUFFER_OPENS);
            let (get_buffer_for_full_scan_tx, get_buffer_for_full_scan_rx) =
                bounded(MAX_CONCURRENT_BUFFER_OPENS);
            let matches_count = AtomicUsize::new(0);
            let matched_buffer_count = AtomicUsize::new(0);
            let worker_pool = executor.scoped(|scope| {
                let (input_paths_tx, input_paths_rx) = bounded(64);
                let (find_first_match_tx, find_first_match_rx) = bounded(64);

                for _ in 0..executor.num_cpus() {
                    let worker = Worker {
                        query: &query,
                        matched_buffer_count: &matched_buffer_count,
                        matches_count: &matches_count,
                        input_paths_rx: input_paths_rx.clone(),
                        find_first_match_rx: find_first_match_rx.clone(),
                        find_first_match_tx: find_first_match_tx.clone(),
                        get_buffer_for_full_scan_tx: get_buffer_for_full_scan_tx.clone(),
                        find_all_matches_rx: find_all_matches_rx.clone(),
                        publish_matches: tx.clone(),
                    };
                    scope.spawn(worker.run());
                }
                scope.spawn(self.provide_search_paths(&query, input_paths_tx))
            });
            self.open_buffers(get_buffer_for_full_scan_rx, find_all_matches_tx, cx)
                .await;
            worker_pool.await;
            let limit_reached = matches_count.load(Ordering::Release) > MAX_SEARCH_RESULT_RANGES
                || matched_buffer_count.load(Ordering::Release) > MAX_SEARCH_RESULT_FILES;
            if limit_reached {
                _ = tx.send(SearchResult::LimitReached).await;
            }
        })
        .detach();
        rx
    }

    async fn provide_search_paths<'a>(
        &'a self,
        query: &SearchQuery,
        tx: Sender<(&'a Entry, &'a WorktreeSettings)>,
    ) {
        for (snapshot, worktree_settings) in &self.snapshots {
            for entry in snapshot.entries(query.include_ignored(), 0) {
                let Ok(_) = tx.send((entry, worktree_settings)).await else {
                    return;
                };
            }
        }
    }

    /// Background workers cannot open buffers by themselves, hence main thread will do it on their behalf.
    async fn open_buffers<'a>(
        &'a self,
        rx: Receiver<ProjectPath>,
        find_all_matches_tx: Sender<(Entity<Buffer>, BufferSnapshot)>,
        cx: &mut AsyncApp,
    ) {
        _ = maybe!(async move {
            while let Ok(requested_path) = rx.recv().await {
                let Some(buffer) = self
                    .buffer_store
                    .update(cx, |this, cx| this.open_buffer(requested_path, cx))?
                    .await
                    .log_err()
                else {
                    continue;
                };
                let snapshot = buffer.read_with(cx, |this, _| this.snapshot())?;
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
    /// Start off with all paths in project and filter them based on:
    /// - Include filters
    /// - Exclude filters
    /// - Only open buffers
    /// - Scan ignored files
    /// Put another way: filter out files that can't match (without looking at file contents)
    input_paths_rx: Receiver<InputPath<'search>>,
    /// After that, figure out which paths contain at least one match (look at file contents). That's called "partial scan".
    find_first_match_tx: Sender<MatchingEntry>,
    find_first_match_rx: Receiver<MatchingEntry>,
    /// Of those that contain at least one match, look for rest of matches (and figure out their ranges).
    /// But wait - first, we need to go back to the main thread to open a buffer (& create an entity for it).
    get_buffer_for_full_scan_tx: Sender<ProjectPath>,
    /// Ok, we're back in background: run full scan & find all matches in a given buffer snapshot.
    find_all_matches_rx: Receiver<(Entity<Buffer>, BufferSnapshot)>,
    /// Cool, we have results; let's share them with the world.
    publish_matches: Sender<SearchResult>,
}

impl Worker<'_> {
    async fn run(self) {
        let mut find_all_matches = pin!(self.find_all_matches_rx.fuse());
        let mut find_first_match = pin!(self.find_first_match_rx.fuse());
        let mut scan_path = pin!(self.input_paths_rx.fuse());
        let handler = RequestHandler {
            query: self.query,
            matched_buffer_count: self.matched_buffer_count,
            matches_count: self.matches_count,
            find_first_match_tx: &self.find_first_match_tx,
            get_buffer_for_full_scan_tx: &self.get_buffer_for_full_scan_tx,
            publish_matches: &self.publish_matches,
        };
        loop {
            select_biased! {
                find_all_matches = find_all_matches.next() => {
                    let result = handler.handle_find_all_matches(find_all_matches).await;
                    if let Some(should_bail) = result {
                        return;
                    }
                },
                find_first_match = find_first_match.next() => {

                },
                scan_path = scan_path.next() => {
                    handler.handle_scan_path(scan_path).await;
                 }
                 complete => break,
            }
        }
    }
}

struct RequestHandler<'worker> {
    query: &'worker SearchQuery,
    matched_buffer_count: &'worker AtomicUsize,
    matches_count: &'worker AtomicUsize,

    find_first_match_tx: &'worker Sender<MatchingEntry>,
    get_buffer_for_full_scan_tx: &'worker Sender<ProjectPath>,
    publish_matches: &'worker Sender<SearchResult>,
}

struct LimitReached;

impl RequestHandler<'_> {
    async fn handle_find_all_matches(
        &self,
        req: Option<(Entity<Buffer>, BufferSnapshot)>,
    ) -> Option<LimitReached> {
        let Some((buffer, snapshot)) = req else {
            unreachable!()
        };
        let ranges = self
            .query
            .search(&snapshot, None)
            .await
            .iter()
            .map(|range| snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end))
            .collect::<Vec<_>>();

        let matched_ranges = ranges.len();
        if self.matched_buffer_count.fetch_add(1, Ordering::Release) > MAX_SEARCH_RESULT_FILES
            || self
                .matches_count
                .fetch_add(matched_ranges, Ordering::Release)
                > MAX_SEARCH_RESULT_RANGES
        {
            Some(LimitReached)
        } else {
            self.publish_matches
                .send(SearchResult::Buffer { buffer, ranges })
                .await;
            None
        }
    }
    async fn handle_scan_path(&self, req: InputPath<'_>) {
        let InputPath {
            entry,
            settings,
            snapshot,
        } = req;
        if entry.is_dir() && entry.is_ignored {
            if !settings.is_path_excluded(&entry.path) {
                Self::scan_ignored_dir(&fs, &snapshot, &entry.path, &query, &filter_tx, &output_tx)
                    .await?;
            }
            return None;
            // continue;
        }

        if entry.is_fifo || !entry.is_file() {
            return None;
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
                return None;
                // continue;
            }
        }

        let (mut tx, rx) = oneshot::channel();

        if open_entries.contains(&entry.id) {
            tx.send(ProjectPath {
                worktree_id: snapshot.id(),
                path: entry.path.clone(),
            })
            .await?;
        } else {
            filter_tx
                .send(MatchingEntry {
                    respond: tx,
                    worktree_root: snapshot.abs_path().clone(),
                    path: ProjectPath {
                        worktree_id: snapshot.id(),
                        path: entry.path.clone(),
                    },
                })
                .await?;
        }

        output_tx.send(rx).await?;
    }
}

struct InputPath<'worker> {
    entry: &'worker Entry,
    settings: &'worker WorktreeSettings,
    snapshot: &'worker Snapshot,
}

struct MatchingEntry {
    worktree_root: Arc<Path>,
    path: ProjectPath,
    respond: oneshot::Sender<ProjectPath>,
}
