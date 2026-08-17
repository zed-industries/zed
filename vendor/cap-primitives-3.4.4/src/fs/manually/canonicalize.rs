//! Manual path canonicalization, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use super::internal_open;
use crate::fs::{canonicalize_options, FollowSymlinks, MaybeOwnedFile};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Implement `canonicalize` by breaking up the path into components and
/// resolving each component individually, and resolving symbolic links
/// manually.
pub(crate) fn canonicalize(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    canonicalize_with(start, path, FollowSymlinks::Yes)
}

/// The main body of `canonicalize`, which takes an extra `follow` flag
/// allowing the caller to disable following symlinks in the last component.
pub(crate) fn canonicalize_with(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<PathBuf> {
    let mut symlink_count = 0;
    let mut canonical_path = PathBuf::new();
    let start = MaybeOwnedFile::borrowed(start);

    match internal_open(
        start,
        path,
        canonicalize_options().follow(follow),
        &mut symlink_count,
        Some(&mut canonical_path),
    ) {
        // If the open succeeded, we got our path.
        Ok(_) => (),

        // If it failed due to an invalid argument or filename, report it.
        Err(err) if err.kind() == io::ErrorKind::InvalidInput => {
            return Err(err);
        }
        #[cfg(io_error_more)]
        Err(err) if err.kind() == io::ErrorKind::InvalidFilename => {
            return Err(err);
        }
        #[cfg(windows)]
        Err(err)
            if err.raw_os_error()
                == Some(windows_sys::Win32::Foundation::ERROR_INVALID_NAME as _)
                || err.raw_os_error()
                    == Some(windows_sys::Win32::Foundation::ERROR_DIRECTORY as _) =>
        {
            return Err(err);
        }

        // For any other error, like permission denied, it's ok as long as
        // we got our path.
        Err(err) => {
            if canonical_path.as_os_str().is_empty() {
                return Err(err);
            }
        }
    }

    Ok(canonical_path)
}
