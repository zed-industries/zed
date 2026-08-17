use std::io::{Read, Seek, SeekFrom, Write};
use sys_common::io::tmpdir;
use system_interface::fs::{FdFlags, GetSetFdFlags};
#[macro_use]
mod sys_common;

#[test]
fn test_get_set_fd_flags() {
    let tmpdir = tmpdir();
    let mut file = check!(tmpdir.create("file"));

    let flags = check!(file.get_fd_flags());
    assert!(!flags.contains(FdFlags::SYNC));
    assert!(!flags.contains(FdFlags::APPEND));
    assert!(!flags.contains(FdFlags::NONBLOCK));

    let set_fd_flags = check!(file.new_set_fd_flags(FdFlags::APPEND));
    check!(file.set_fd_flags(set_fd_flags));

    let flags = check!(file.get_fd_flags());
    assert!(!flags.contains(FdFlags::SYNC));
    assert!(flags.contains(FdFlags::APPEND));
    assert!(!flags.contains(FdFlags::NONBLOCK));

    // `NONBLOCK` is not supported on Windows yet.
    #[cfg(windows)]
    {
        let set_fd_flags = check!(file.new_set_fd_flags(FdFlags::NONBLOCK));
        check!(file.set_fd_flags(set_fd_flags));
    }
    #[cfg(not(windows))]
    {
        let set_fd_flags = check!(file.new_set_fd_flags(FdFlags::NONBLOCK));
        check!(file.set_fd_flags(set_fd_flags));

        let flags = check!(file.get_fd_flags());
        assert!(!flags.contains(FdFlags::SYNC));
        assert!(!flags.contains(FdFlags::APPEND));
        assert!(flags.contains(FdFlags::NONBLOCK));
    }
}

#[test]
fn test_append() {
    let tmpdir = tmpdir();
    let mut file = check!(tmpdir.create("file"));
    check!(write!(file, "Hello"));

    let mut with_append =
        check!(tmpdir.open_with("file", cap_fs_ext::OpenOptions::new().append(true)));
    check!(write!(with_append, " world"));

    let mut for_read = check!(tmpdir.open("file"));
    let mut s = String::new();
    check!(for_read.read_to_string(&mut s));
    assert_eq!(s, "Hello world");

    check!(with_append.seek(SeekFrom::Start(0)));
    check!(write!(with_append, "The quick brown fox"));

    let mut for_read = check!(tmpdir.open("file"));
    let mut s = String::new();
    check!(for_read.read_to_string(&mut s));
    assert_eq!(s, "Hello worldThe quick brown fox");
}
