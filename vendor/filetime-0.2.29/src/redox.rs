use crate::FileTime;
use std::fs::{self};
use std::io;
use std::os::unix::prelude::*;
use std::path::Path;

pub const O_NOFOLLOW: i32 = 0x20000;

pub fn set_symlink_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(O_NOFOLLOW)
        .open(p)?;
    crate::set_file_handle_times(&file, Some(atime), Some(mtime))
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

pub fn from_creation_time(_meta: &fs::Metadata) -> Option<FileTime> {
    None
}

pub fn open(path: &Path) -> io::Result<fs::File> {
    fs::File::open(path)
}
