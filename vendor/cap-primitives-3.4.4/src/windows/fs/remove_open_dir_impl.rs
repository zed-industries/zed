use super::get_path::get_path;
use std::{fs, io};

pub(crate) fn remove_open_dir_impl(dir: fs::File) -> io::Result<()> {
    let path = get_path(&dir)?;

    // Drop the directory before removing it, since we open directories without
    // `FILE_SHARE_DELETE`, and removing it requires accessing via its name
    // rather than its handle.
    //
    // There is a window here in which another process could remove or rename
    // a directory with this path after the handle is dropped, however it's
    // unlikely to happen by accident, and unlikely to cause major problems.
    // It may cause spurious failures, or failures with different error codes,
    // but this appears to be unavoidable.
    //
    // Even if we did have `FILE_SHARE_DELETE` and we kept the handle open
    // while doing the `remove_dir, `FILE_SHARE_DELETE` would grant other
    // processes the right to remove or rename the directory. So there
    // doesn't seem to be a race-free way of removing opened directories.
    drop(dir);

    fs::remove_dir(path)
}
