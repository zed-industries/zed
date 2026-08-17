use crate::fs::{open, FollowSymlinks, OpenOptions, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::{fs, io};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
};

/// *Unsandboxed* function similar to `read_link`, but which does not perform
/// sandboxing.
pub(crate) fn read_link_impl(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Open the link with no access mode, instead of generic read.
    // By default FILE_LIST_DIRECTORY is denied for the junction "C:\Documents and
    // Settings", so this is needed for a common case.
    let mut opts = OpenOptions::new();
    opts.access_mode(0);
    opts.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
    opts.follow(FollowSymlinks::No);
    let file = open(start, path, &opts)?;
    winx::file::read_link(&file)
}
