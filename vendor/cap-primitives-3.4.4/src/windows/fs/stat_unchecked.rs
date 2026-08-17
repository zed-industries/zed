use crate::fs::OpenOptionsExt;
use crate::fs::{open_unchecked, FollowSymlinks, Metadata, OpenOptions};
use std::path::Path;
use std::{fs, io};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
};

/// *Unsandboxed* function similar to `stat`, but which does not perform
/// sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    // Attempt to open the file to get the metadata that way, as that gives
    // us all the info.
    let mut opts = OpenOptions::new();

    // Explicitly request no access, because we're just querying metadata.
    opts.access_mode(0);

    match follow {
        FollowSymlinks::Yes => {
            opts.custom_flags(FILE_FLAG_BACKUP_SEMANTICS);
            opts.follow(FollowSymlinks::Yes);
        }
        FollowSymlinks::No => {
            opts.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
            opts.follow(FollowSymlinks::No);
        }
    }

    let file = open_unchecked(start, path, &opts)?;
    Metadata::from_file(&file)
}
