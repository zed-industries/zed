use super::open_parent;
use crate::fs::{set_permissions_unchecked, MaybeOwnedFile, Permissions};
use std::path::Path;
use std::{fs, io};

#[inline]
pub(crate) fn set_permissions(start: &fs::File, path: &Path, perm: Permissions) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, &path)?;

    set_permissions_unchecked(&dir, basename.as_ref(), perm)
}
