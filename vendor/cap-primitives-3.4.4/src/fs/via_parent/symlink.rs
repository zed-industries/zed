use super::open_parent;
use crate::fs::MaybeOwnedFile;
use std::path::Path;
use std::{fs, io};

/// Implement `symlink` by `open`ing up the parent component of the path and
/// then calling `symlink_unchecked` on the last component.
#[cfg(not(windows))]
pub(crate) fn symlink(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_unchecked;
    let new_start = MaybeOwnedFile::borrowed(new_start);

    let (new_dir, new_basename) = open_parent(new_start, new_path)?;

    symlink_unchecked(old_path, &new_dir, new_basename.as_ref())
}

/// Implement `symlink_file` by `open`ing up the parent component of the path
/// and then calling `symlink_file` on the last component.
#[cfg(windows)]
pub(crate) fn symlink_file(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    use crate::fs::symlink_file_unchecked;
    let new_start = MaybeOwnedFile::borrowed(new_start);

    let (new_dir, new_basename) = open_parent(new_start, new_path)?;

    symlink_file_unchecked(old_path, &new_dir, new_basename.as_ref())
}

/// Implement `symlink_dir` by `open`ing up the parent component of the path
/// and then calling `symlink_dir` on the last component.
#[cfg(windows)]
pub(crate) fn symlink_dir(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    use crate::fs::symlink_dir_unchecked;
    let new_start = MaybeOwnedFile::borrowed(new_start);

    let (new_dir, new_basename) = open_parent(new_start, new_path)?;

    symlink_dir_unchecked(old_path, &new_dir, new_basename.as_ref())
}
