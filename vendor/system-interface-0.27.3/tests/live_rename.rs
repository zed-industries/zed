#[macro_use]
mod sys_common;

use std::fs::{rename, File, OpenOptions};
#[cfg(not(racy_asserts))] // racy asserts are racy
use std::thread;
use system_interface::fs::FileIoExt;
use system_interface::io::IoExt;

/// Ensure that `read_at` works even when the underlying file is renamed.
#[test]
fn live_rename() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));

    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file.upper")));
    check!(write!(&file, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

    let mut buf = [0_u8; 8];
    let file = check!(File::open(dir.path().join("file")));
    check!(file.read_exact_at(&mut buf, 8));
    assert_eq!(&buf, b"ijklmnop");

    check!(rename(dir.path().join("file"), dir.path().join("renamed")));

    check!(file.read_exact_at(&mut buf, 16));
    assert_eq!(&buf, b"qrstuvwx");

    check!(rename(
        dir.path().join("file.upper"),
        dir.path().join("file")
    ));

    check!(file.read_exact_at(&mut buf, 12));
    assert_eq!(&buf, b"mnopqrst");

    assert_eq!(check!(file.stream_position()), 0);
}

#[cfg(not(racy_asserts))] // racy asserts are racy
#[test]
fn concurrent_rename() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));

    let mut joins = vec![];

    let path = dir.path().to_path_buf();
    let file = check!(File::open(path.join("file")));
    joins.push(thread::spawn(move || {
        let mut buf = [0_u8; 8];
        for _ in 0..10000 {
            check!(file.read_exact_at(&mut buf, 8));
            assert_eq!(&buf, b"ijklmnop");
        }
    }));

    let path = dir.path().to_path_buf();
    joins.push(thread::spawn(move || {
        let start = path.join("file");
        check!(rename(start, path.join(format!("file.{}", 0))));
        for i in 0..10000 {
            check!(rename(
                path.join(format!("file.{}", i)),
                path.join(format!("file.{}", i + 1))
            ));
        }
    }));

    for join in joins {
        join.join().unwrap();
    }
}
