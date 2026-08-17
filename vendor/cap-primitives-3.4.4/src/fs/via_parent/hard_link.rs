use super::open_parent;
use crate::fs::{hard_link_unchecked, MaybeOwnedFile};
use std::path::Path;
use std::{fs, io};

/// Implement `hard_link` by `open`ing up the parent component of the path and
/// then calling `hard_link_unchecked` on the last component.
pub(crate) fn hard_link(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let old_start = MaybeOwnedFile::borrowed(old_start);
    let new_start = MaybeOwnedFile::borrowed(new_start);

    let (old_dir, old_basename) = open_parent(old_start, old_path)?;
    let (new_dir, new_basename) = open_parent(new_start, new_path)?;

    hard_link_unchecked(
        &old_dir,
        old_basename.as_ref(),
        &new_dir,
        new_basename.as_ref(),
    )
}
