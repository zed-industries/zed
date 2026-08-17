//! This defines `remove_dir`, the primary entrypoint to sandboxed file
//! removal.

use crate::fs::remove_dir_impl;
#[cfg(racy_asserts)]
use crate::fs::{
    manually, map_result, remove_dir_unchecked, stat_unchecked, FollowSymlinks, Metadata,
};
use std::path::Path;
use std::{fs, io};

/// Perform a `rmdirat`-like operation, ensuring that the resolution of the
/// path never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn remove_dir(start: &fs::File, path: &Path) -> io::Result<()> {
    #[cfg(racy_asserts)]
    let stat_before = stat_unchecked(start, path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = remove_dir_impl(start, path);

    #[cfg(racy_asserts)]
    let stat_after = stat_unchecked(start, path, FollowSymlinks::No);

    #[cfg(racy_asserts)]
    check_remove_dir(start, path, &stat_before, &result, &stat_after);

    result
}

#[cfg(racy_asserts)]
#[allow(clippy::enum_glob_use)]
fn check_remove_dir(
    start: &fs::File,
    path: &Path,
    stat_before: &io::Result<Metadata>,
    result: &io::Result<()>,
    stat_after: &io::Result<Metadata>,
) {
    use io::ErrorKind::*;

    match (
        map_result(stat_before),
        map_result(result),
        map_result(stat_after),
    ) {
        (Ok(metadata), Ok(()), Err((NotFound, _))) => {
            // TODO: Check that the path was inside the sandbox.
            assert!(metadata.is_dir());
        }

        (Err((Other, _)), Ok(()), Err((NotFound, _))) => {
            // TODO: Check that the path was inside the sandbox.
        }

        (_, Err((InvalidInput, _)), _) => {
            // `remove_dir(".")` apparently returns `EINVAL`
        }

        (_, Err((kind, message)), _) => {
            match map_result(&manually::canonicalize_with(
                start,
                path,
                FollowSymlinks::No,
            )) {
                Ok(canon) => match map_result(&remove_dir_unchecked(start, &canon)) {
                    Err((_unchecked_kind, _unchecked_message)) => {
                        /* TODO: Check error messages.
                        assert_eq!(
                            kind,
                            unchecked_kind,
                            "unexpected error kind from remove_dir start='{:?}', \
                             path='{}':\nstat_before={:#?}\nresult={:#?}\nstat_after={:#?}",
                            start,
                            path.display(),
                            stat_before,
                            result,
                            stat_after
                        );
                        assert_eq!(message, unchecked_message);
                        */
                    }
                    _ => {
                        // TODO: Checking in the case it does end with ".".
                        if !path.to_string_lossy().ends_with(".") {
                            panic!(
                                "unsandboxed remove_dir success on start={:?} path={:?}; expected \
                                 {:?}: {}",
                                start, path, kind, message
                            );
                        }
                    }
                },
                Err((_canon_kind, _canon_message)) => {
                    /* TODO: Check error messages.
                    assert_eq!(kind, canon_kind, "'{}' vs '{}'", message, canon_message);
                    assert_eq!(message, canon_message);
                    */
                }
            }
        }

        other => panic!(
            "inconsistent remove_dir checks: start='{:?}' path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }

    match stat_after {
        Ok(unchecked_metadata) => match &result {
            Ok(()) => panic!(
                "file still exists after remove_dir start='{:?}', path='{}'",
                start,
                path.display()
            ),
            Err(e) => match e.kind() {
                #[cfg(io_error_more)]
                io::ErrorKind::NotADirectory => assert!(!unchecked_metadata.is_dir()),
                #[cfg(io_error_more)]
                io::ErrorKind::DirectoryNotEmpty => (),
                io::ErrorKind::PermissionDenied
                | io::ErrorKind::InvalidInput // `remove_dir(".")` apparently returns `EINVAL`
                | io::ErrorKind::Other => (),        // directory not empty, among other things
                _ => panic!(
                    "unexpected error remove_dir'ing start='{:?}', path='{}': {:?}",
                    start,
                    path.display(),
                    e
                ),
            },
        },
        Err(_unchecked_error) => match &result {
            Ok(()) => (),
            Err(result_error) => match result_error.kind() {
                io::ErrorKind::PermissionDenied => (),
                _ => {
                    /* TODO: Check error messages.
                    assert_eq!(result_error.to_string(), unchecked_error.to_string());
                    assert_eq!(result_error.kind(), unchecked_error.kind());
                    */
                }
            },
        },
    }
}
