#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(target_os = "wasi")))] // Wasi does not support file operations

use tokio::fs;
use tokio_test::assert_ok;

#[tokio::test]
async fn path_read_write() {
    let temp = tempdir();
    let dir = temp.path();

    assert_ok!(fs::write(dir.join("bar"), b"bytes").await);
    let out = assert_ok!(fs::read(dir.join("bar")).await);

    assert_eq!(out, b"bytes");
}

#[tokio::test]
async fn try_clone_should_preserve_max_buf_size() {
    let buf_size = 128;
    let temp = tempdir();
    let dir = temp.path();

    let mut file = fs::File::create(dir.join("try_clone_should_preserve_max_buf_size"))
        .await
        .unwrap();
    file.set_max_buf_size(buf_size);

    let cloned = file.try_clone().await.unwrap();

    assert_eq!(cloned.max_buf_size(), buf_size);
}

fn tempdir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}
