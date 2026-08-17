//! Linux has an `O_PATH` flag which allows opening a file without necessary
//! having read or write access to it; we can use that with `openat2` and
//! `fstat` to perform a fast sandboxed `stat`.

use super::file_metadata::file_metadata;
use crate::fs::{manually, open_beneath, FollowSymlinks, Metadata, OpenOptions};
use rustix::fs::OFlags;
use std::path::Path;
use std::{fs, io};

/// Use `openat2` with `O_PATH` and `fstat`. If that's not available, fallback
/// to `manually::stat`.
pub(crate) fn stat_impl(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    use crate::fs::{stat_unchecked, OpenOptionsExt};
    use std::path::Component;

    // Optimization: if path has exactly one component and it's not ".." and
    // we're not following symlinks we can go straight to `stat_unchecked`,
    // which is faster than doing an open with a separate fstat.
    if follow == FollowSymlinks::No {
        let mut components = path.components();
        if let Some(component) = components.next() {
            if components.next().is_none() && component != Component::ParentDir {
                return stat_unchecked(start, component.as_ref(), FollowSymlinks::No);
            }
        }
    }

    // Open the path with `O_PATH`. Use `read(true)` even though we don't need
    // `read` permissions, because Rust's libstd requires an access mode, and
    // Linux ignores `O_RDONLY` with `O_PATH`.
    let result = open_beneath(
        start,
        path,
        OpenOptions::new()
            .read(true)
            .follow(follow)
            .custom_flags(OFlags::PATH.bits() as i32),
    );

    // If that worked, call `fstat`.
    match result {
        Ok(file) => file_metadata(&file),
        Err(err) => match rustix::io::Errno::from_io_error(&err) {
            // `ENOSYS` from `open_beneath` means `openat2` is unavailable
            // and we should use a fallback.
            Some(rustix::io::Errno::NOSYS) => manually::stat(start, path, follow),
            _ => Err(err),
        },
    }
}
