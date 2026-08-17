#[macro_use]
mod sys_common;

use std::fs::OpenOptions;
use system_interface::fs::FileIoExt;
use system_interface::io::IoExt;

#[test]
fn read_to_end_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf = Vec::new();
    check!(file.read_to_end_at(&mut buf, 4));
    assert_eq!(check!(file.stream_position()), 26);
    assert_eq!(&buf, b"efghijklmnopqrstuvwxyz");
}

#[test]
fn read_to_string_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf = String::new();
    check!(file.read_to_string_at(&mut buf, 4));
    assert_eq!(check!(file.stream_position()), 26);
    assert_eq!(buf, "efghijklmnopqrstuvwxyz");
}

#[test]
fn read_to_string_at_error() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(file.write_all(b"abcdefghijklmnopqrstuvwxyz\xc0"));
    let mut buf = String::new();
    assert!(file.read_to_string_at(&mut buf, 4).is_err());
    assert!(buf.is_empty());
}
