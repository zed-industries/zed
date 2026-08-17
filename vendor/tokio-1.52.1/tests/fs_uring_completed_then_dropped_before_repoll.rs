#![cfg(all(
    tokio_unstable,
    feature = "io-uring",
    feature = "rt",
    feature = "fs",
    target_os = "linux"
))]

use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use tempfile::NamedTempFile;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::time::timeout;

/// Count currently-open fds in this process.
fn fd_count() -> usize {
    fs::read_dir("/proc/self/fd").unwrap().count()
}

/// First poll:
/// - polls the inner `tokio::fs::OpenOptions::open()` future once,
/// - expects `Pending` so we know we took the io_uring path,
/// - registers the task waker with Tokio's uring machinery.
///
/// Second poll:
/// - happens after the kernel completes the open and Tokio stores the CQE as
///   `Lifecycle::Completed(cqe)` and wakes the task,
/// - **intentionally does not poll the inner open future again**,
/// - stays pending forever so the task can be aborted.
///
/// Aborting the task here drops the inner `open()` future while Tokio still has
/// a completed CQE sitting in the slab.
struct PollOpenOnceThenNeverRepoll<F> {
    inner: Pin<Box<F>>,
    first_poll_tx: Option<UnboundedSender<()>>,
    second_poll_tx: Option<UnboundedSender<()>>,
    polled_once: bool,
}

impl<F: Future> Future for PollOpenOnceThenNeverRepoll<F> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.polled_once {
            // We don't check if the result is actually pending because
            // we run some checks on old kernel that do not support uring.
            let _pending = self.inner.as_mut().poll(cx);

            self.polled_once = true;
            self.first_poll_tx.take().unwrap().send(()).unwrap();
            return Poll::Pending;
        }

        // We were polled again after the inner open completed and woke the task.
        // Crucially, we do *not* re-poll the inner future here.
        if let Some(tx) = self.second_poll_tx.take() {
            tx.send(()).unwrap();
        }

        Poll::Pending
    }
}

async fn completed_then_dropped_before_repoll(path: PathBuf) {
    let (first_tx, mut first_rx) = unbounded_channel();
    let (second_tx, mut second_rx) = unbounded_channel();

    let handle = tokio::spawn(async move {
        let mut opt = tokio::fs::OpenOptions::new();
        opt.read(true);

        PollOpenOnceThenNeverRepoll {
            inner: Box::pin(opt.open(&path)),
            first_poll_tx: Some(first_tx),
            second_poll_tx: Some(second_tx),
            polled_once: false,
        }
        .await;
    });

    // Wait until the inner open has been polled once and registered with io_uring.
    first_rx.recv().await.unwrap();

    // Wait until Tokio wakes the task because the open completed. At this point
    // the CQE should already be stored as `Lifecycle::Completed(cqe)`.
    let _ = timeout(Duration::from_secs(2), second_rx.recv()).await;

    // Abort now, before the inner open future gets re-polled and consumes the CQE.
    handle.abort();
    let err = handle.await.unwrap_err();
    assert!(err.is_cancelled(), "task was not cancelled as expected");
}

#[test]
fn uring_completed_then_dropped() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let before = fd_count();
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        for _ in 0..128 {
            completed_then_dropped_before_repoll(path.clone()).await;
        }

        // Give completions a moment to settle before counting fds.
        tokio::time::sleep(Duration::from_millis(250)).await;

        let after = fd_count();
        let leaked = after.saturating_sub(before);

        // Since we are opening 128 files, we expect that the related fds
        // related to this operation will be closed. Since some other fds
        // can be opened in the meantime, we expect this number to be higher
        // than the counter before opening the files. This number could be
        // lower, but to avoid test flakiness we check that this is at most
        // half the number of the file we opened to check if there's a leak.
        assert!(leaked <= 64);
    });
}
