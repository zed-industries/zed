use super::open_parent;
use crate::fs::{create_dir_unchecked, strip_dir_suffix, DirOptions, MaybeOwnedFile};
use std::path::Path;
use std::{fs, io};

/// Implement `create_dir` by `open`ing up the parent component of the path and
/// then calling `create_dir_unchecked` on the last component.
pub(crate) fn create_dir(start: &fs::File, path: &Path, options: &DirOptions) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    // As a special case, `create_dir` ignores a trailing slash rather than
    // treating it as equivalent to a trailing slash-dot, so strip any trailing
    // slashes.
    let path = strip_dir_suffix(path);

    let (dir, basename) = open_parent(start, &path)?;

    create_dir_unchecked(&dir, basename.as_ref(), options)
}
