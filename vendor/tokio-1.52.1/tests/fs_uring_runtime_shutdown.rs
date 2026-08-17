//! Uring file operations tests.

#![cfg(all(
    tokio_unstable,
    feature = "io-uring",
    feature = "rt",
    feature = "fs",
    target_os = "linux"
))]

use futures::FutureExt;
use std::fs;
use std::future::poll_fn;
use std::task::Poll;
use tempfile::NamedTempFile;
use tokio::runtime::Builder;

// see: https://github.com/tokio-rs/tokio/issues/7979
#[test]
fn shutdown_runtime_while_performing_io_uring_ops() {
    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();

    let fd_count_before_opens = fs::read_dir("/proc/self/fd").unwrap().count();

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    rt.block_on(async {
        for _ in 0..128 {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

            let path = path.clone();
            let handle = tokio::spawn(async move {
                poll_fn(|cx| {
                    let opt = {
                        let mut opt = tokio::fs::OpenOptions::new();
                        opt.read(true);
                        opt
                    };

                    let fut = opt.open(&path);

                    // If io_uring is enabled (and not falling back to the thread pool),
                    // the first poll should return Pending. We don't check if the result
                    // is actually a pending because we run some CI checks based on old
                    // kernels that don't support uring.
                    let _pending = Box::pin(fut).poll_unpin(cx);

                    tx.send(()).unwrap();

                    Poll::<()>::Pending
                })
                .await;
            });

            // Wait for the first poll
            rx.recv().await.unwrap();

            handle.abort();

            let res = handle.await.unwrap_err();
            assert!(res.is_cancelled());
        }
    });

    rt.shutdown_background();

    let fd_count_after_cancel = fs::read_dir("/proc/self/fd").unwrap().count();
    let leaked = fd_count_after_cancel.saturating_sub(fd_count_before_opens);

    // Since we are opening 128 files, we expect that the related fds
    // related to this operation will be closed. Since some other fds
    // can be opened in the meantime, we expect this number to be higher
    // than the counter before opening the files. This number could be
    // lower, but to avoid test flakiness we check that this is at most
    // half the number of the file we opened to check if there's a leak.
    assert!(leaked <= 64);
}
