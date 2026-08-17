#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(target_os = "wasi")))] // WASI does not support all fs operations

use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn write() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.txt");

    fs::write(&path, "Hello, World!").await.unwrap();

    let contents = fs::read_to_string(&path).await.unwrap();
    assert_eq!(contents, "Hello, World!");
}
