//! Tests for AsyncRead on tokio::fs::File via io-uring.

#![cfg(all(
    tokio_unstable,
    feature = "io-uring",
    feature = "rt",
    feature = "fs",
    target_os = "linux"
))]

use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tempfile::NamedTempFile;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::runtime::{Builder, Runtime};
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
    vec![current_rt(), multi_rt(1), multi_rt(4), multi_rt(64)]
}

fn create_temp_file(data: &[u8]) -> (NamedTempFile, PathBuf) {
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(data).unwrap();
    tmp.flush().unwrap();
    let path = tmp.path().to_path_buf();
    (tmp, path)
}

#[tokio::test]
async fn test_file_read() {
    let data = b"hello io-uring";
    let (_tmp, path) = create_temp_file(data);

    let mut file = File::open(&path).await.unwrap();
    let mut buf = vec![0u8; data.len()];
    let n = file.read(&mut buf).await.unwrap();
    assert_eq!(n, data.len());
    assert_eq!(&buf[..n], data);
}

#[tokio::test]
async fn test_file_read_exact() {
    let data = b"exact read test data";
    let (_tmp, path) = create_temp_file(data);

    let mut file = File::open(&path).await.unwrap();
    let mut buf = vec![0u8; data.len()];
    file.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, data);
}

#[tokio::test]
async fn test_file_read_to_end() {
    // Empty file
    {
        let (_tmp, path) = create_temp_file(b"");
        let mut file = File::open(&path).await.unwrap();
        let mut buf = Vec::new();
        let n = file.read_to_end(&mut buf).await.unwrap();
        assert_eq!(n, 0);
        assert!(buf.is_empty());
    }

    // Small file
    {
        let data: Vec<u8> = (0..100u8).collect();
        let (_tmp, path) = create_temp_file(&data);
        let mut file = File::open(&path).await.unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.unwrap();
        assert_eq!(buf, data);
    }

    // File larger than typical buffer
    {
        let data: Vec<u8> = (0..100_000u32).map(|i| (i % 256) as u8).collect();
        let (_tmp, path) = create_temp_file(&data);
        let mut file = File::open(&path).await.unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.unwrap();
        assert_eq!(buf, data);
    }
}

#[tokio::test]
async fn test_file_read_to_string() {
    let text = "hello, io-uring world! 🦀";
    let (_tmp, path) = create_temp_file(text.as_bytes());

    let mut file = File::open(&path).await.unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).await.unwrap();
    assert_eq!(s, text);
}

#[tokio::test]
async fn test_file_read_empty() {
    let (_tmp, path) = create_temp_file(b"");

    let mut file = File::open(&path).await.unwrap();
    let mut buf = vec![0u8; 100];
    let n = file.read(&mut buf).await.unwrap();
    assert_eq!(n, 0);
}

#[tokio::test]
async fn test_file_read_large() {
    let data: Vec<u8> = (0..3_000_000u32).map(|i| (i % 256) as u8).collect();
    let (_tmp, path) = create_temp_file(&data);

    let mut file = File::open(&path).await.unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    assert_eq!(buf.len(), data.len());
    assert_eq!(buf, data);
}

#[tokio::test]
async fn test_file_read_custom_buf_size() {
    let data: Vec<u8> = (0..1000u16).map(|i| (i % 256) as u8).collect();
    let (_tmp, path) = create_temp_file(&data);

    let mut file = File::open(&path).await.unwrap();
    file.set_max_buf_size(64);
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    assert_eq!(buf, data);
}

#[tokio::test]
async fn test_file_read_partial_buf() {
    let data: Vec<u8> = (0..100u8).collect();
    let (_tmp, path) = create_temp_file(&data);

    let mut file = File::open(&path).await.unwrap();

    // Read only 5 bytes
    let mut buf = vec![0u8; 5];
    file.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, &data[..5]);

    // Read the rest
    let mut rest = Vec::new();
    file.read_to_end(&mut rest).await.unwrap();
    assert_eq!(rest, data[5..]);
}

#[tokio::test]
async fn test_file_read_seek() {
    let data = b"hello world";
    let (_tmp, path) = create_temp_file(data);

    let mut file = File::open(&path).await.unwrap();
    file.seek(std::io::SeekFrom::Start(6)).await.unwrap();

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    assert_eq!(buf, b"world");
}

#[tokio::test]
async fn test_file_read_seek_back() {
    let data = b"hello world";
    let (_tmp, path) = create_temp_file(data);

    let mut file = File::open(&path).await.unwrap();

    // Read first 5 bytes
    let mut buf = vec![0u8; 5];
    file.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"hello");

    // Seek back to start
    file.seek(std::io::SeekFrom::Start(0)).await.unwrap();

    // Read full content
    let mut full = Vec::new();
    file.read_to_end(&mut full).await.unwrap();
    assert_eq!(full, data);
}

#[tokio::test]
async fn test_file_read_after_write() {
    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();

    let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .await
        .unwrap();

    file.write_all(b"hello uring").await.unwrap();
    file.flush().await.unwrap();
    file.seek(std::io::SeekFrom::Start(0)).await.unwrap();

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    assert_eq!(buf, b"hello uring");
}

#[tokio::test]
async fn test_file_read_cancel() {
    let data: Vec<u8> = (0..10_000).map(|i| (i % 256) as u8).collect();
    let (_tmp, path) = create_temp_file(&data);

    let path2 = path.clone();
    let handle = tokio::spawn(async move {
        let mut file = File::open(&path2).await.unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.unwrap();
        buf
    });

    handle.abort();
    let res = handle.await;
    assert!(res.unwrap_err().is_cancelled());

    // Verify runtime still works
    let mut file = File::open(&path).await.unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    assert_eq!(buf, data);
}

#[tokio::test]
async fn test_file_read_concurrent() {
    const NUM_FILES: usize = 100;

    let files: Vec<_> = (0..NUM_FILES)
        .map(|i| {
            let data: Vec<u8> = (0..1024).map(|j| ((i as u16 + j) % 256) as u8).collect();
            create_temp_file(&data)
        })
        .collect();

    let tracker = TaskTracker::new();

    for (i, (_tmp, path)) in files.iter().enumerate() {
        let path = path.clone();
        let expected: Vec<u8> = (0..1024).map(|j| ((i as u16 + j) % 256) as u8).collect();
        tracker.spawn(async move {
            let mut file = File::open(&path).await.unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).await.unwrap();
            assert_eq!(buf, expected);
        });
    }

    tracker.close();
    tracker.wait().await;
}

#[test]
fn test_file_read_multi_runtime() {
    for rt_factory in rt_combinations() {
        let rt = rt_factory();
        let data: Vec<u8> = (0..10_000).map(|i| (i % 256) as u8).collect();
        let (_tmp, path) = create_temp_file(&data);

        let result = rt.block_on(async {
            let mut file = File::open(&path).await.unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).await.unwrap();
            buf
        });

        assert_eq!(result, data);
    }
}

#[test]
fn shutdown_runtime_with_pending_reads() {
    for rt_factory in rt_combinations() {
        let rt = rt_factory();
        let (done_tx, done_rx) = mpsc::channel();

        let data: Vec<u8> = (0..10_000).map(|i| (i % 256) as u8).collect();
        let (_tmp, path) = create_temp_file(&data);

        for _ in 0..50 {
            let path = path.clone();
            rt.spawn(async move {
                let mut file = File::open(&path).await.unwrap();
                let mut buf = Vec::new();
                let _ = file.read_to_end(&mut buf).await;
            });
        }

        std::thread::spawn(move || {
            rt.shutdown_timeout(Duration::from_millis(500));
            done_tx.send(()).unwrap();
        });

        done_rx.recv().unwrap();
    }
}

#[test]
fn test_file_read_from_std() {
    let mut tmp = NamedTempFile::new().unwrap();
    let data = b"hello from_std io-uring";
    tmp.write_all(data).unwrap();
    tmp.flush().unwrap();
    let path = tmp.path().to_owned();

    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let std_file = std::fs::File::open(&path).unwrap();
        let mut file = File::from_std(std_file);
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.unwrap();
        assert_eq!(buf, data);
    });
}

/// Read with a buffer smaller than the file content. Verifies internal
/// buffering serves subsequent small reads without issuing new underlying read
/// operations.
#[tokio::test]
async fn test_file_read_with_smaller_buf() {
    let data: Vec<u8> = (0..1024u16).map(|i| (i % 256) as u8).collect();
    let (_tmp, path) = create_temp_file(&data);

    let mut file = File::open(&path).await.unwrap();

    // triggers an underlying read that fills the internal buffer with more data
    // than we consume here
    let mut buf = vec![0u8; 4];
    let n = file.read(&mut buf).await.unwrap();
    assert_eq!(n, 4);
    assert_eq!(&buf, &data[..4]);

    // Second read: still smaller than what's buffered internally
    let mut buf = vec![0u8; 32];
    let n = file.read(&mut buf).await.unwrap();
    assert!(n > 0);
    assert_eq!(&buf[..n], &data[4..4 + n]);

    // Read the rest
    let mut rest = Vec::new();
    file.read_to_end(&mut rest).await.unwrap();
    let total = 4 + n + rest.len();
    assert_eq!(total, data.len());
}

#[tokio::test]
async fn test_file_read_with_bigger_buf() {
    let data = b"hello io-uring";
    let (_tmp, path) = create_temp_file(data);

    let mut file = File::open(&path).await.unwrap();

    // Read with buffer larger than file's contents
    let mut buf = vec![0u8; 1024];
    let n = file.read(&mut buf).await.unwrap();
    assert!(n > 0 && n <= data.len());
    assert_eq!(&buf[..n], &data[..n]);

    if n < data.len() {
        let mut rest = vec![0u8; 1024];
        let n2 = file.read(&mut rest).await.unwrap();
        assert_eq!(&rest[..n2], &data[n..n + n2]);
    }
}

/// Read a file larger than DEFAULT_MAX_BUF_SIZE (2 MiB). Verifies that
/// chunked reads across multiple underlying operations produce correct data.
#[tokio::test]
async fn test_file_read_buffer_larger_than_max() {
    // 4 MiB + 1000 bytes to cross multiple chunk boundaries.
    let size = (4 << 20) + 1000;
    let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
    let (_tmp, path) = create_temp_file(&data);

    let mut file = File::open(&path).await.unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    assert_eq!(buf.len(), data.len());
    assert_eq!(buf, data);
}

/// Read some bytes from a file, then write, verifying the implicit seek-back
/// works correctly.
#[tokio::test]
async fn test_file_read_then_write() {
    let original = b"hello world, io-uring!";
    let (_tmp, path) = create_temp_file(original);

    let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .await
        .unwrap();

    let mut buf = vec![0u8; 5];
    file.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"hello");

    // Write at the current position
    file.write_all(b" REPLACED").await.unwrap();
    file.flush().await.unwrap();

    // Re-read the full file and verify the write landed correctly
    file.seek(std::io::SeekFrom::Start(0)).await.unwrap();
    let mut result = Vec::new();
    file.read_to_end(&mut result).await.unwrap();
    assert_eq!(&result[..5], b"hello");
    assert_eq!(&result[5..14], b" REPLACED");
}

/// Partial read followed by write at a different position. Verifies the
/// seek-back accounts for partially consumed internal buffer.
#[tokio::test]
async fn test_file_partial_read_then_write() {
    let data = b"abcdefghijklmnopqrstuvwxyz";
    let (_tmp, path) = create_temp_file(data);

    let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .await
        .unwrap();

    // First read fills internal buffer.
    let mut buf = vec![0u8; 10];
    file.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"abcdefghij");

    // Second read served from internal buffer
    let mut buf2 = vec![0u8; 3];
    file.read_exact(&mut buf2).await.unwrap();
    assert_eq!(&buf2, b"klm");

    // Write should seek back past unconsumed buffered bytes, then write
    file.write_all(b"NOP").await.unwrap();
    file.flush().await.unwrap();

    file.seek(std::io::SeekFrom::Start(0)).await.unwrap();
    let mut result = Vec::new();
    file.read_to_end(&mut result).await.unwrap();
    assert_eq!(&result[..13], b"abcdefghijklm");
    assert_eq!(&result[13..16], b"NOP");
    assert_eq!(&result[16..], &data[16..]);
}
