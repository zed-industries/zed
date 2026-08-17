use crate::fs::{Metadata, MetadataExt};
use std::{fs, io};

/// Determine if `a` and `b` refer to the same inode on the same device.
pub(crate) fn is_same_file(a: &fs::File, b: &fs::File) -> io::Result<bool> {
    let a_metadata = Metadata::from_file(a)?;
    let b_metadata = Metadata::from_file(b)?;
    is_same_file_metadata(&a_metadata, &b_metadata)
}

/// Determine if `a` and `b` are metadata for the same inode on the same
/// device.
pub(crate) fn is_same_file_metadata(a: &Metadata, b: &Metadata) -> io::Result<bool> {
    Ok(a.dev() == b.dev() && a.ino() == b.ino())
}

/// Determine if `a` and `b` definitely refer to different inodes.
///
/// This is similar to `is_same_file`, but is conservative, and doesn't depend
/// on nightly-only features.
#[allow(dead_code)]
pub(crate) fn is_different_file(a: &fs::File, b: &fs::File) -> io::Result<bool> {
    is_same_file(a, b).map(|same| !same)
}

/// Determine if `a` and `b` are metadata for definitely different inodes.
///
/// This is similar to `is_same_file_metadata`, but is conservative, and
/// doesn't depend on nightly-only features.
#[allow(dead_code)]
pub(crate) fn is_different_file_metadata(a: &Metadata, b: &Metadata) -> io::Result<bool> {
    is_same_file_metadata(a, b).map(|same| !same)
}
