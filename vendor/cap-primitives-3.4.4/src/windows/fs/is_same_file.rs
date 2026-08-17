use crate::fs::ImplMetadataExt;
#[cfg(windows_by_handle)]
use crate::fs::Metadata;
use std::{fs, io};

/// Determine if `a` and `b` refer to the same inode on the same device.
pub(crate) fn is_same_file(a: &fs::File, b: &fs::File) -> io::Result<bool> {
    let a_metadata = ImplMetadataExt::from(a, &a.metadata()?)?;
    let b_metadata = ImplMetadataExt::from(b, &b.metadata()?)?;
    Ok(a_metadata.is_same_file(&b_metadata))
}

/// Determine if `a` and `b` are metadata for the same inode on the same
/// device.
#[cfg(windows_by_handle)]
#[allow(dead_code)]
pub(crate) fn is_same_file_metadata(a: &Metadata, b: &Metadata) -> io::Result<bool> {
    use crate::fs::MetadataExt;
    Ok(a.volume_serial_number() == b.volume_serial_number() && a.file_index() == b.file_index())
}

/// Determine if `a` and `b` definitely refer to different inodes.
///
/// This is similar to `is_same_file`, but is conservative, and doesn't depend
/// on nightly-only features.
#[cfg(racy_asserts)]
pub(crate) fn is_different_file(a: &fs::File, b: &fs::File) -> io::Result<bool> {
    #[cfg(windows_by_handle)]
    {
        is_same_file(a, b).map(|same| !same)
    }

    #[cfg(not(windows_by_handle))]
    {
        let a_metadata = Metadata::from_std(a.metadata()?);
        let b_metadata = Metadata::from_std(b.metadata()?);
        is_different_file_metadata(&a_metadata, &b_metadata)
    }
}

/// Determine if `a` and `b` are metadata for definitely different inodes.
///
/// This is similar to `is_same_file_metadata`, but is conservative, and
/// doesn't depend on nightly-only features.
#[cfg(racy_asserts)]
pub(crate) fn is_different_file_metadata(a: &Metadata, b: &Metadata) -> io::Result<bool> {
    #[cfg(windows_by_handle)]
    {
        is_same_file_metadata(a, b).map(|same| !same)
    }

    #[cfg(not(windows_by_handle))]
    {
        // Conservatively just compare creation times.
        Ok(a.created()? != b.created()?)
    }
}
