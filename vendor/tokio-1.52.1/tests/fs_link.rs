#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(target_os = "wasi")))] // WASI does not support all fs operations

use tokio::fs;

use std::io::Write;
use tempfile::tempdir;

#[tokio::test]
#[cfg_attr(miri, ignore)] // No `linkat` in miri.
async fn test_hard_link() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("dst.txt");

    std::fs::File::create(&src)
        .unwrap()
        .write_all(b"hello")
        .unwrap();

    fs::hard_link(&src, &dst).await.unwrap();

    std::fs::File::create(&src)
        .unwrap()
        .write_all(b"new-data")
        .unwrap();

    let content = fs::read(&dst).await.unwrap();
    assert_eq!(content, b"new-data");

    // test that this is not a symlink:
    assert!(fs::read_link(&dst).await.is_err());
}

#[cfg(unix)]
#[tokio::test]
async fn test_symlink() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("dst.txt");

    std::fs::File::create(&src)
        .unwrap()
        .write_all(b"hello")
        .unwrap();

    fs::symlink(&src, &dst).await.unwrap();

    std::fs::File::create(&src)
        .unwrap()
        .write_all(b"new-data")
        .unwrap();

    let content = fs::read(&dst).await.unwrap();
    assert_eq!(content, b"new-data");

    let read = fs::read_link(dst.clone()).await.unwrap();
    assert!(read == src);

    let symlink_meta = fs::symlink_metadata(dst.clone()).await.unwrap();
    assert!(symlink_meta.file_type().is_symlink());
}

#[tokio::test]
#[cfg_attr(miri, ignore)] // No `linkat` in miri.
async fn test_hard_link_error_source_not_found() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("nonexistent.txt");
    let dst = dir.path().join("dst.txt");

    let err = fs::hard_link(&src, &dst).await.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[tokio::test]
#[cfg_attr(miri, ignore)] // No `linkat` in miri.
async fn test_hard_link_error_destination_already_exists() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("dst.txt");

    // Create source file
    std::fs::write(&src, b"source content").unwrap();

    // Create destination file
    std::fs::write(&dst, b"destination content").unwrap();

    // Attempt to create hard link when destination already exists
    let err = fs::hard_link(&src, &dst).await.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::AlreadyExists);
}

#[tokio::test]
#[cfg_attr(miri, ignore)] // No `linkat` in miri.
async fn test_hard_link_error_source_is_directory() {
    let dir = tempdir().unwrap();
    let src_dir = dir.path().join("src_directory");
    let dst = dir.path().join("dst.txt");

    // Create source directory
    fs::create_dir(&src_dir).await.unwrap();

    // Attempt to create hard link from a directory
    // On most systems, hard linking directories is not allowed
    let err = fs::hard_link(&src_dir, &dst).await.unwrap_err();

    // Different platforms return different error kinds
    #[cfg(unix)]
    assert!(
        err.kind() == std::io::ErrorKind::PermissionDenied
            || err.kind() == std::io::ErrorKind::Other
    );

    #[cfg(windows)]
    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
}
