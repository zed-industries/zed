use std::{
    io::{BufRead, BufReader},
    path::Path,
    pin::pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use collections::HashSet;
use fs::Fs;
use futures::{SinkExt, StreamExt, select_biased};
use gpui::{App, AsyncApp, Entity, WeakEntity};
use language::{Buffer, BufferSnapshot};
use postage::oneshot;
use smol::channel::{Receiver, Sender, bounded, unbounded};

use util::{ResultExt, maybe};
use worktree::{Entry, ProjectEntryId, Snapshot, WorktreeSettings};

use crate::{
    ProjectPath,
    buffer_store::BufferStore,
    search::{SearchQuery, SearchResult},
};

pub(crate) struct ProjectSearcher {
    pub(crate) fs: Arc<dyn Fs>,
    pub(crate) buffer_store: WeakEntity<BufferStore>,
    pub(crate) snapshots: Vec<(Snapshot, WorktreeSettings)>,
    pub(crate) open_buffers: HashSet<ProjectEntryId>,
}

const MAX_SEARCH_RESULT_FILES: usize = 5_000;
const MAX_SEARCH_RESULT_RANGES: usize = 10_000;

impl ProjectSearcher {
    pub(crate) fn run(self, query: SearchQuery, cx: &mut App) -> Receiver<SearchResult> {
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
                let (confirm_contents_will_match_tx, confirm_contents_will_match_rx) = bounded(64);
                let (sorted_search_results_tx, sorted_search_results_rx) = bounded(64);
                for _ in 0..executor.num_cpus() {
                    let worker = Worker {
                        query: &query,
                        open_buffers: &self.open_buffers,
                        matched_buffer_count: &matched_buffer_count,
                        matches_count: &matches_count,
                        fs: &*self.fs,
                        input_paths_rx: input_paths_rx.clone(),
                        confirm_contents_will_match_rx: confirm_contents_will_match_rx.clone(),
                        confirm_contents_will_match_tx: confirm_contents_will_match_tx.clone(),
                        get_buffer_for_full_scan_tx: get_buffer_for_full_scan_tx.clone(),
                        find_all_matches_rx: find_all_matches_rx.clone(),
                        publish_matches: tx.clone(),
                    };
                    scope.spawn(worker.run());
                }
                scope.spawn(self.provide_search_paths(
                    &query,
                    input_paths_tx,
                    sorted_search_results_tx,
                ));
                scope.spawn(self.maintain_sorted_search_results(
                    sorted_search_results_rx,
                    get_buffer_for_full_scan_tx,
                ))
            });
            self.open_buffers(get_buffer_for_full_scan_rx, find_all_matches_tx, cx)
                .await;
            worker_pool.await;
            let limit_reached = matches_count.load(Ordering::Acquire) > MAX_SEARCH_RESULT_RANGES
                || matched_buffer_count.load(Ordering::Acquire) > MAX_SEARCH_RESULT_FILES;
            if limit_reached {
                _ = tx.send(SearchResult::LimitReached).await;
            }
        })
        .detach();
        rx
    }

    async fn provide_search_paths<'this>(
        &'this self,
        query: &SearchQuery,
        tx: Sender<InputPath<'this>>,
        results: Sender<oneshot::Receiver<ProjectPath>>,
    ) {
        for (snapshot, worktree_settings) in &self.snapshots {
            for entry in snapshot.entries(query.include_ignored(), 0) {
                let (should_scan_tx, should_scan_rx) = oneshot::channel();
                let Ok(_) = tx
                    .send(InputPath {
                        entry,
                        settings: worktree_settings,
                        snapshot: snapshot,
                        should_scan_tx,
                    })
                    .await
                else {
                    return;
                };
                results.send(should_scan_rx).await;
            }
        }
    }

    async fn maintain_sorted_search_results(
        &self,
        rx: Receiver<oneshot::Receiver<ProjectPath>>,
        paths_for_full_scan: Sender<ProjectPath>,
    ) {
        let mut rx = pin!(rx);
        while let Some(mut next_path_result) = rx.next().await {
            let Some(successful_path) = next_path_result.next().await else {
                // This math did not produce a match, hence skip it.
                continue;
            };
            paths_for_full_scan.send(successful_path).await;
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
    open_buffers: &'search HashSet<ProjectEntryId>,
    fs: &'search dyn Fs,
    /// Start off with all paths in project and filter them based on:
    /// - Include filters
    /// - Exclude filters
    /// - Only open buffers
    /// - Scan ignored files
    /// Put another way: filter out files that can't match (without looking at file contents)
    input_paths_rx: Receiver<InputPath<'search>>,

    /// After that, if the buffer is not yet loaded, we'll figure out if it contains at least one match
    /// based on disk contents of a buffer. This step is not performed for buffers we already have in memory.
    confirm_contents_will_match_tx: Sender<MatchingEntry>,
    confirm_contents_will_match_rx: Receiver<MatchingEntry>,
    /// Of those that contain at least one match (or are already in memory), look for rest of matches (and figure out their ranges).
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
        let mut find_first_match = pin!(self.confirm_contents_will_match_rx.fuse());
        let mut scan_path = pin!(self.input_paths_rx.fuse());
        let handler = RequestHandler {
            query: self.query,
            open_entries: &self.open_buffers,
            fs: self.fs,
            matched_buffer_count: self.matched_buffer_count,
            matches_count: self.matches_count,
            confirm_contents_will_match_tx: &self.confirm_contents_will_match_tx,
            get_buffer_for_full_scan_tx: &self.get_buffer_for_full_scan_tx,
            publish_matches: &self.publish_matches,
        };
        loop {
            select_biased! {
                find_all_matches = find_all_matches.next() => {
                    let result = handler.handle_find_all_matches(find_all_matches).await;
                    if let Some(_should_bail) = result {
                        return;
                    }
                },
                find_first_match = find_first_match.next() => {
                    if let Some(buffer_with_at_least_one_match) = find_first_match {
                        handler.handle_find_first_match(buffer_with_at_least_one_match);
                    }

                },
                scan_path = scan_path.next() => {
                    if let Some(path_to_scan) = scan_path {
                        handler.handle_scan_path(path_to_scan).await;
                    }

                 }
                 complete => break,
            }
        }
    }
}

struct RequestHandler<'worker> {
    query: &'worker SearchQuery,
    fs: &'worker dyn Fs,
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
    async fn handle_find_first_match(&self, mut entry: MatchingEntry) {
        _=maybe!(async move {
            let abs_path = entry.worktree_root.join(entry.path.path.as_std_path());
            let Some(file) = self.fs.open_sync(&abs_path).await.log_err() else {
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

    async fn handle_scan_path(&self, req: InputPath<'_>) {
        _ = maybe!(async move {
            let InputPath {
                entry,
                settings,
                snapshot,
                should_scan_tx,
            } = req;
            if entry.is_dir() && entry.is_ignored {
                if !settings.is_path_excluded(&entry.path) {
                    // Self::scan_ignored_dir(
                    //     self.fs,
                    //     &snapshot,
                    //     &entry.path,
                    //     self.query,
                    //     &filter_tx,
                    //     &output_tx,
                    // )
                    // .await?;
                }
                return Ok(());
            }

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

struct InputPath<'worker> {
    entry: &'worker Entry,
    settings: &'worker WorktreeSettings,
    snapshot: &'worker Snapshot,
    should_scan_tx: oneshot::Sender<ProjectPath>,
}

struct MatchingEntry {
    worktree_root: Arc<Path>,
    path: ProjectPath,
    should_scan_tx: oneshot::Sender<ProjectPath>,
}
