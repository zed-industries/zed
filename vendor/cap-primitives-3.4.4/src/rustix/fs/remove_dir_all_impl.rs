use crate::fs::{
    read_dir_nofollow, read_dir_unchecked, remove_dir, remove_file, remove_open_dir, stat,
    FollowSymlinks, ReadDir,
};
use std::path::{Component, Path};
use std::{fs, io};

pub(crate) fn remove_dir_all_impl(start: &fs::File, path: &Path) -> io::Result<()> {
    // Code adapted from `remove_dir_all` in Rust's
    // library/std/src/sys_common/fs.rs at revision
    // 108e90ca78f052c0c1c49c42a22c85620be19712.
    let filetype = stat(start, path, FollowSymlinks::No)?.file_type();
    if filetype.is_symlink() {
        remove_file(start, path)
    } else {
        remove_dir_all_recursive(read_dir_nofollow(start, path)?)?;
        remove_dir(start, path)
    }
}

pub(crate) fn remove_open_dir_all_impl(dir: fs::File) -> io::Result<()> {
    remove_dir_all_recursive(read_dir_unchecked(
        &dir,
        Component::CurDir.as_ref(),
        FollowSymlinks::No,
    )?)?;
    remove_open_dir(dir)
}

fn remove_dir_all_recursive(children: ReadDir) -> io::Result<()> {
    for child in children {
        let child = child?;
        if child.file_type()?.is_dir() {
            remove_dir_all_recursive(child.inner.read_dir(FollowSymlinks::No)?)?;
            child.remove_dir()?;
        } else {
            child.remove_file()?;
        }
    }
    Ok(())
}
