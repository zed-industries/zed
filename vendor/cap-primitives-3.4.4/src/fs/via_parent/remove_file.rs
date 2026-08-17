use super::open_parent;
use crate::fs::{remove_file_unchecked, MaybeOwnedFile};
use std::path::Path;
use std::{fs, io};

/// Implement `remove_file` by `open`ing up the parent component of the path
/// and then calling `remove_file_unchecked` on the last component.
pub(crate) fn remove_file(start: &fs::File, path: &Path) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, path)?;

    remove_file_unchecked(&dir, basename.as_ref())
}
