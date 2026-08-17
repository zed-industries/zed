use super::open_parent;
use crate::fs::{access_unchecked, AccessType, FollowSymlinks, MaybeOwnedFile};
use std::path::Path;
use std::{fs, io};

/// Implement `access` by `open`ing up the parent component of the path and
/// then calling `access_unchecked` on the last component.
pub(crate) fn access(
    start: &fs::File,
    path: &Path,
    type_: AccessType,
    follow: FollowSymlinks,
) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, path)?;

    access_unchecked(&dir, basename.as_ref(), type_, follow)
}
