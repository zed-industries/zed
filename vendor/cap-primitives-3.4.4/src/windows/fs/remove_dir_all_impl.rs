use super::get_path::get_path;
use crate::fs::{open_dir, open_dir_nofollow, remove_dir, stat, FollowSymlinks};
use std::path::Path;
use std::{fs, io};

pub(crate) fn remove_dir_all_impl(start: &fs::File, path: &Path) -> io::Result<()> {
    // Open the directory, following symlinks, to make sure it is a directory.
    let file = open_dir(start, path)?;
    // Test whether the path is a symlink.
    let md = stat(start, path, FollowSymlinks::No)?;
    drop(file);
    if md.is_symlink() {
        // If so, just remove the link.
        remove_dir(start, path)
    } else {
        // Otherwise, remove the tree.
        let dir = open_dir_nofollow(start, path)?;
        remove_open_dir_all_impl(dir)
    }
}

pub(crate) fn remove_open_dir_all_impl(dir: fs::File) -> io::Result<()> {
    // Close the directory so that we can delete it. This is racy; see the
    // comments in `remove_open_dir_impl` for details.
    let path = get_path(&dir)?;
    drop(dir);
    fs::remove_dir_all(&path)
}
