use crate::fs::remove_dir_all_impl;
use std::path::Path;
use std::{fs, io};

/// Removes a directory and all of its contents.
#[inline]
pub fn remove_dir_all(start: &fs::File, path: &Path) -> io::Result<()> {
    remove_dir_all_impl(start, path)
}
