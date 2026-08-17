use super::open_parent;
use crate::fs::{remove_dir_unchecked, MaybeOwnedFile};
use std::path::Path;
use std::{fs, io};

/// Implement `remove_dir` by `open`ing up the parent component of the path and
/// then calling `remove_dir_unchecked` on the last component.
pub(crate) fn remove_dir(start: &fs::File, path: &Path) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, path)?;

    remove_dir_unchecked(&dir, basename.as_ref())
}
