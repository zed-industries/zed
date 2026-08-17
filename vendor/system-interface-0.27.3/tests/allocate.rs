#[macro_use]
mod sys_common;

#[cfg(not(target_os = "openbsd"))]
use std::fs::OpenOptions;
#[cfg(not(target_os = "openbsd"))]
use system_interface::fs::FileIoExt;

#[test]
#[cfg(not(any(windows, target_os = "openbsd")))]
fn allocate() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));

    assert_eq!(check!(file.metadata()).len(), 0);

    check!(file.allocate(1024, 1024));

    assert_eq!(check!(file.metadata()).len(), 1024 + 1024);

    check!(file.allocate(1024, 1024));

    assert_eq!(check!(file.metadata()).len(), 1024 + 1024);

    check!(file.allocate(4096, 4096));

    assert_eq!(check!(file.metadata()).len(), 4096 + 4096);
}

#[test]
#[cfg(windows)]
fn allocate() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));

    assert_eq!(check!(file.metadata()).len(), 0);

    file.allocate(1024, 1024)
        .expect_err("allocate should fail on windows");
}
