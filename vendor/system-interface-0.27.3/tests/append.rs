#[macro_use]
mod sys_common;

use std::fs::OpenOptions;
use std::io::IoSlice;
#[cfg(any(not(windows), feature = "cap_std_impls"))]
use sys_common::io::tmpdir;
use system_interface::fs::FileIoExt;
use system_interface::io::IoExt;

#[cfg(any(not(windows), feature = "cap_std_impls"))]
#[test]
fn cap_append_all_vectored() {
    let tmpdir = tmpdir();
    let file = check!(tmpdir.open_with(
        "file",
        cap_std::fs::OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
    ));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    check!(file.seek(std::io::SeekFrom::Start(0)));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let mut bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    check!(file.append_all_vectored(&mut bufs));
    assert_eq!(check!(file.stream_position()), 0);
    let mut back = String::new();
    check!(file.read_to_string(&mut back));
    assert_eq!(back, "abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST");
}

#[test]
fn append_all_vectored() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    check!(file.seek(std::io::SeekFrom::Start(0)));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let mut bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    check!(file.append_all_vectored(&mut bufs));
    assert_eq!(check!(file.stream_position()), 0);
    let mut back = String::new();
    check!(file.read_to_string(&mut back));
    assert_eq!(back, "abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST");
}

#[test]
fn append_vectored() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    check!(file.seek(std::io::SeekFrom::Start(0)));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    let nwritten = check!(file.append_vectored(&bufs));
    assert_eq!(check!(file.stream_position()), 0);
    let mut back = String::new();
    check!(file.read_to_string(&mut back));
    assert_eq!(
        &"abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST"[..26 + nwritten],
        &back
    );
}

#[test]
fn append_all() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    check!(file.seek(std::io::SeekFrom::Start(0)));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    check!(file.append_all(&buf0));
    check!(file.append_all(&buf1));
    assert_eq!(check!(file.stream_position()), 0);
    let mut back = String::new();
    check!(file.read_to_string(&mut back));
    assert_eq!(back, "abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST");
}

#[test]
fn append() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    check!(file.seek(std::io::SeekFrom::Start(0)));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let nwritten0 = check!(file.append(&buf0));
    let nwritten1 = check!(file.append(&buf1));
    assert_eq!(check!(file.stream_position()), 0);
    let mut back = String::new();
    check!(file.read_to_string(&mut back));
    assert_eq!(
        &"abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST"[..26 + nwritten0 + nwritten1],
        &back
    );
}
