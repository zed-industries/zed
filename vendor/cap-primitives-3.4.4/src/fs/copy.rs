use crate::fs::copy_impl;
use std::path::Path;
use std::{fs, io};

/// Copies the contents of one file to another.
#[inline]
pub fn copy(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    // In theory we could do extra sanity checks here, but `copy_impl`
    // implementations use other sandboxed routines to open the files,
    // so it'd be mostly redundant.
    copy_impl(from_start, from_path, to_start, to_path)
}
