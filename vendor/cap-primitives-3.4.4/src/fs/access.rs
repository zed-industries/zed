//! Access test functions.

use crate::fs::{access_impl, FollowSymlinks};
#[cfg(racy_asserts)]
use crate::fs::{access_unchecked, file_path};
use std::path::Path;
use std::{fs, io};

/// Access modes for use with [`DirExt::access`].
#[derive(Clone, Copy, Debug)]
pub struct AccessModes {
    /// Is the object readable?
    pub readable: bool,
    /// Is the object writable?
    pub writable: bool,
    /// Is the object executable?
    pub executable: bool,
}

/// Access modes for use with [`DirExt::access`].
#[derive(Clone, Copy, Debug)]
pub enum AccessType {
    /// Test whether the named object is accessible in the given modes.
    Access(AccessModes),

    /// Test whether the named object exists.
    Exists,
}

/// Canonicalize the given path, ensuring that the resolution of the path never
/// escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
pub fn access(
    start: &fs::File,
    path: &Path,
    type_: AccessType,
    follow: FollowSymlinks,
) -> io::Result<()> {
    // Call the underlying implementation.
    let result = access_impl(start, path, type_, follow);

    #[cfg(racy_asserts)]
    let unchecked = access_unchecked(start, path, type_, follow);

    #[cfg(racy_asserts)]
    check_access(start, path, type_, follow, &result, &unchecked);

    result
}

#[cfg(racy_asserts)]
#[allow(clippy::enum_glob_use)]
fn check_access(
    start: &fs::File,
    path: &Path,
    _type_: AccessType,
    _follow: FollowSymlinks,
    result: &io::Result<()>,
    unchecked: &io::Result<()>,
) {
    use io::ErrorKind::*;

    match (map_result(result), map_result(stat)) {
        (Ok(()), Ok(())) => {}

        (Err((PermissionDenied, message)), _) => {
            // TODO: Check that access in the no-follow case got the right
            // error.
        }

        (Err((kind, message)), Err((unchecked_kind, unchecked_message))) => {
            assert_eq!(kind, unchecked_kind);
            assert_eq!(
                message,
                unchecked_message,
                "start='{:?}', path='{:?}'",
                start,
                path.display()
            );
        }

        other => panic!(
            "unexpected result from access start='{:?}', path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }
}
