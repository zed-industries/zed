use crate::fs::{remove_open_dir_all_impl, remove_open_dir_impl};
use std::{fs, io};

/// Given an open directory handle, delete the directory.
#[inline]
pub fn remove_open_dir(dir: fs::File) -> io::Result<()> {
    remove_open_dir_impl(dir)
}

/// Given an open directory handle, recursively delete the contents of the
/// directory plus the directory itself.
#[allow(clippy::module_name_repetitions)]
#[inline]
pub fn remove_open_dir_all(dir: fs::File) -> io::Result<()> {
    remove_open_dir_all_impl(dir)
}
