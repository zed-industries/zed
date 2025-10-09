use std::{
    ops::Range,
    pin::{Pin, pin},
};

use futures::{FutureExt, StreamExt, future::BoxFuture, select_biased};
use gpui::{App, AsyncApp, Entity, WeakEntity};
use language::{Buffer, BufferSnapshot};
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

impl ProjectSearcher {
    pub(crate) fn search(self, query: SearchQuery, cx: &mut App) -> Receiver<SearchResult> {
        let executor = cx.background_executor().clone();
        let (tx, rx) = unbounded();
        cx.spawn(async move |cx| {
            const MAX_CONCURRENT_BUFFER_OPENS: usize = 64;
            let (find_all_matches_tx, find_all_matches_rx) = bounded(MAX_CONCURRENT_BUFFER_OPENS);
            let (get_buffer_for_full_scan_tx, get_buffer_for_full_scan_rx) =
                bounded(MAX_CONCURRENT_BUFFER_OPENS);
            let worker_pool = executor.scoped(|scope| {
                let (input_paths_tx, input_paths_rx) = bounded(64);
                let (find_first_match_tx, find_first_match_rx) = bounded(64);

                for _ in 0..executor.num_cpus() {
                    let worker = Worker {
                        query: &query,
                        input_paths_rx: input_paths_rx.clone(),
                        find_first_match_rx: find_first_match_rx.clone(),
                        find_first_match_tx: find_first_match_tx.clone(),
                        get_buffer_for_full_scan_tx: get_buffer_for_full_scan_tx.clone(),
                        find_all_matches_rx: find_all_matches_rx.clone(),
                        publish_matches: todo!(),
                    };
                    scope.spawn(worker.run());
                }
                scope.spawn(self.provide_search_paths(&query, input_paths_tx))
            });
            self.open_buffers(get_buffer_for_full_scan_rx, find_all_matches_tx, cx)
                .await;
            worker_pool.await;
        })
        .detach();
        rx
    }

    async fn provide_search_paths<'a>(&'a self, query: &SearchQuery, tx: Sender<&'a Entry>) {
        for (snapshot, _) in &self.snapshots {
            for entry in snapshot.entries(query.include_ignored(), 0) {
                let Ok(_) = tx.send(entry).await else {
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
    /// Start off with all paths in project and filter them based on:
    /// - Include filters
    /// - Exclude filters
    /// - Only open buffers
    /// - Scan ignored files
    /// Put another way: filter out files that can't match (without looking at file contents)
    input_paths_rx: Receiver<&'search Entry>,
    /// After that, figure out which paths contain at least one match (look at file contents). That's called "partial scan".
    find_first_match_tx: Sender<()>,
    find_first_match_rx: Receiver<()>,
    /// Of those that contain at least one match, look for rest of matches (and figure out their ranges).
    /// But wait - first, we need to go back to the main thread to open a buffer (& create an entity for it).
    get_buffer_for_full_scan_tx: Sender<ProjectPath>,
    /// Ok, we're back in background: run full scan & find all matches in a given buffer snapshot.
    find_all_matches_rx: Receiver<(Entity<Buffer>, BufferSnapshot)>,
    /// Cool, we have results; let's share them with the world.
    publish_matches: Sender<(Entity<Buffer>, Vec<Range<Anchor>>)>,
}

impl Worker<'_> {
    async fn run(self) {
        let mut find_all_matches = pin!(self.find_all_matches_rx.fuse());
        let mut find_first_match = pin!(self.find_first_match_rx.fuse());
        let mut scan_path = pin!(self.input_paths_rx.fuse());
        loop {
            select_biased! {
                find_all_matches = find_all_matches.next() => {

                },
                find_first_match = find_first_match.next() => {

                },
                scan_path = scan_path.next() => {

                 },
                 complete => break,
            }
        }
    }
}
