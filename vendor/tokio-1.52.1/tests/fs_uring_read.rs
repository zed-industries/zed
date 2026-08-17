//! Uring file operations tests.

#![cfg(all(
    tokio_unstable,
    feature = "io-uring",
    feature = "rt",
    feature = "fs",
    target_os = "linux"
))]

use futures::future::Future;
use std::future::poll_fn;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use tempfile::NamedTempFile;
use tokio::fs::read;
use tokio::runtime::{Builder, Runtime};
use tokio_test::assert_pending;
use tokio_util::task::TaskTracker;

fn multi_rt(n: usize) -> Box<dyn Fn() -> Runtime> {
    Box::new(move || {
        Builder::new_multi_thread()
            .worker_threads(n)
            .enable_all()
            .build()
            .unwrap()
    })
}

fn current_rt() -> Box<dyn Fn() -> Runtime> {
    Box::new(|| Builder::new_current_thread().enable_all().build().unwrap())
}

fn rt_combinations() -> Vec<Box<dyn Fn() -> Runtime>> {
    vec![
        current_rt(),
        multi_rt(1),
        multi_rt(2),
        multi_rt(8),
        multi_rt(64),
        multi_rt(256),
    ]
}

#[test]
fn shutdown_runtime_while_performing_io_uring_ops() {
    fn run(rt: Runtime) {
        let (done_tx, done_rx) = mpsc::channel();
        let (_tmp, path) = create_tmp_files(1);
        // keep 100 permits
        const N: i32 = 100;
        rt.spawn(async move {
            let path = path[0].clone();

            // spawning a bunch of uring operations.
            let mut futs = vec![];

            // spawning a bunch of uring operations.
            for _ in 0..N {
                let path = path.clone();
                let mut fut = Box::pin(read(path));

                poll_fn(|cx| {
                    assert_pending!(fut.as_mut().poll(cx));
                    Poll::<()>::Pending
                })
                .await;

                futs.push(fut);
            }

            tokio::task::yield_now().await;
        });

        std::thread::spawn(move || {
            rt.shutdown_timeout(Duration::from_millis(300));
            done_tx.send(()).unwrap();
        });

        done_rx.recv().unwrap();
    }

    for rt in rt_combinations() {
        run(rt());
    }
}

#[test]
fn read_many_files() {
    fn run(rt: Runtime) {
        const NUM_FILES: usize = 512;

        let (_tmp_files, paths): (Vec<NamedTempFile>, Vec<PathBuf>) = create_tmp_files(NUM_FILES);

        rt.block_on(async move {
            let tracker = TaskTracker::new();

            for i in 0..10_000 {
                let path = paths.get(i % NUM_FILES).unwrap().clone();
                tracker.spawn(async move {
                    let bytes = read(path).await.unwrap();
                    assert_eq!(bytes, vec![20; 1023]);
                });
            }
            tracker.close();
            tracker.wait().await;
        });
    }

    for rt in rt_combinations() {
        run(rt());
    }
}

#[tokio::test]
async fn read_small_large_files() {
    let (_tmp, path) = create_large_temp_file();

    let bytes = read(path).await.unwrap();

    assert_eq!(bytes, create_buf(5000));

    let (_tmp, path) = create_small_temp_file();

    let bytes = read(path).await.unwrap();

    assert_eq!(bytes, create_buf(20));
}

#[tokio::test]
async fn cancel_op_future() {
    let (_tmp_file, path): (Vec<NamedTempFile>, Vec<PathBuf>) = create_tmp_files(1);
    let path = path[0].clone();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let handle = tokio::spawn(async move {
        let fut = read(path.clone());
        tokio::pin!(fut);

        poll_fn(move |_| {
            // If io_uring is enabled (and not falling back to the thread pool),
            // the first poll should return Pending.
            assert_pending!(fut.as_mut().poll(&mut Context::from_waker(Waker::noop())));
            tx.send(true).unwrap();

            Poll::<()>::Pending
        })
        .await;
    });

    // Wait for the first poll

    let val = rx.recv().await;
    assert!(val.unwrap());

    handle.abort();

    let res = handle.await.unwrap_err();
    assert!(res.is_cancelled());
}

fn create_tmp_files(num_files: usize) -> (Vec<NamedTempFile>, Vec<PathBuf>) {
    let mut files = Vec::with_capacity(num_files);
    for _ in 0..num_files {
        let mut tmp = NamedTempFile::new().unwrap();
        let buf = vec![20; 1023];
        tmp.write_all(&buf).unwrap();
        let path = tmp.path().to_path_buf();
        files.push((tmp, path));
    }

    files.into_iter().unzip()
}

fn create_large_temp_file() -> (NamedTempFile, PathBuf) {
    let mut tmp = NamedTempFile::new().unwrap();
    let buf = create_buf(5000);

    tmp.write_all(&buf).unwrap();
    let path = tmp.path().to_path_buf();

    (tmp, path)
}

fn create_small_temp_file() -> (NamedTempFile, PathBuf) {
    let mut tmp = NamedTempFile::new().unwrap();
    let buf = create_buf(20);

    tmp.write_all(&buf).unwrap();
    let path = tmp.path().to_path_buf();

    (tmp, path)
}

fn create_buf(length: usize) -> Vec<u8> {
    (0..length).map(|i| i as u8).collect()
}
