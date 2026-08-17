use super::open_parent;
use crate::fs::{read_link_unchecked, MaybeOwnedFile};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Implement `read_link` by `open`ing up the parent component of the path and
/// then calling `read_link_unchecked` on the last component.
///
/// This technique doesn't work in all cases on Windows. In particular, a
/// directory symlink such as `C:\Documents and Settings` may not grant any
/// access other than what is needed to resolve the symlink, so `open_parent`'s
/// technique of returning a relative path of `.` from that point doesn't work,
/// because opening `.` within such a directory is denied. Consequently, we use
/// a different implementation on Windows.
pub(crate) fn read_link(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, path)?;

    read_link_unchecked(&dir, basename.as_ref(), PathBuf::new())
}
