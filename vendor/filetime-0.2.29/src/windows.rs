use crate::FileTime;
use std::fs::{self, OpenOptions};
use std::io;
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::prelude::*;
use std::path::Path;

const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x2000000;
const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x200000;

pub fn open(p: &Path) -> io::Result<fs::File> {
    OpenOptions::new()
        .write(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(p)
}

pub fn set_symlink_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    let f = OpenOptions::new()
        .write(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS)
        .open(p)?;
    crate::set_file_handle_times(&f, Some(atime), Some(mtime))
}

pub fn from_last_modification_time(meta: &fs::Metadata) -> FileTime {
    from_intervals(meta.last_write_time())
}

pub fn from_last_access_time(meta: &fs::Metadata) -> FileTime {
    from_intervals(meta.last_access_time())
}

pub fn from_creation_time(meta: &fs::Metadata) -> Option<FileTime> {
    Some(from_intervals(meta.creation_time()))
}

fn from_intervals(ticks: u64) -> FileTime {
    // Windows write times are in 100ns intervals, so do a little math to
    // get it into the right representation.
    FileTime {
        seconds: (ticks / (1_000_000_000 / 100)) as i64,
        nanos: ((ticks % (1_000_000_000 / 100)) * 100) as u32,
    }
}
