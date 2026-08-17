//! This defines `set_times`, the primary entrypoint to sandboxed
//! filesystem times modification.
//!
//! TODO: `check_set_times` etc.

use crate::fs::{set_times_impl, set_times_nofollow_impl, SystemTimeSpec};
use std::path::Path;
use std::{fs, io};

/// Perform a `utimensat`-like operation, ensuring that the resolution of the
/// path never escapes the directory tree rooted at `start`. This function
/// follows symlinks.
#[inline]
pub fn set_times(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    set_times_impl(start, path, atime, mtime)
}

/// Like `set_times`, but never follows symlinks.
#[inline]
pub fn set_times_nofollow(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    set_times_nofollow_impl(start, path, atime, mtime)
}
