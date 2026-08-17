use rustix::fs::{linkat, AtFlags};
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `hard_link`, but which does not perform
/// sandboxing.
///
/// Even though POSIX `linkat` has the ability to follow symlinks in
/// `old_path`, using `AT_SYMLINK_FOLLOW`, Rust's `hard_link` doesn't need
/// that, so we don't expose it here.
pub(crate) fn hard_link_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    Ok(linkat(
        old_start,
        old_path,
        new_start,
        new_path,
        AtFlags::empty(),
    )?)
}
