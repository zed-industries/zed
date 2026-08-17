use crate::FileTime;
use libc::{time_t, timespec, UTIME_OMIT};
use std::fs;
use std::io;
use std::os::unix::prelude::*;
use std::path::Path;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod utimes;
        mod linux;
        pub use self::linux::*;
    } else if #[cfg(target_os = "android")] {
        mod android;
        pub use self::android::*;
    } else if #[cfg(target_os = "macos")] {
        mod utimes;
        mod macos;
        pub use self::macos::*;
    } else if #[cfg(any(target_os = "aix",
                        target_os = "nto",
                        target_os = "solaris",
                        target_os = "illumos",
                        target_os = "emscripten",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                        target_os = "haiku"))] {
        mod utimensat;
        pub use self::utimensat::*;
    } else {
        mod utimes;
        pub use self::utimes::*;
    }
}

#[allow(dead_code)]
fn to_timespec(ft: &Option<FileTime>) -> timespec {
    let mut ts: timespec = unsafe { std::mem::zeroed() };
    if let &Some(ft) = ft {
        ts.tv_sec = ft.seconds() as time_t;
        ts.tv_nsec = ft.nanoseconds() as _;
    } else {
        ts.tv_sec = 0;
        ts.tv_nsec = UTIME_OMIT as _;
    }

    ts
}

pub fn from_last_modification_time(meta: &fs::Metadata) -> FileTime {
    FileTime {
        seconds: meta.mtime(),
        nanos: meta.mtime_nsec() as u32,
    }
}

pub fn from_last_access_time(meta: &fs::Metadata) -> FileTime {
    FileTime {
        seconds: meta.atime(),
        nanos: meta.atime_nsec() as u32,
    }
}

pub fn from_creation_time(meta: &fs::Metadata) -> Option<FileTime> {
    meta.created().map(|i| i.into()).ok()
}

pub fn open(path: &Path) -> io::Result<fs::File> {
    // Try a read-only open, but if that fails try a write-only open, and if
    // that fails then give up.
    fs::File::open(path).or_else(|_| fs::OpenOptions::new().write(true).open(path))
}
