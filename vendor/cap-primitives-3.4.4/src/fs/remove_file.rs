//! This defines `remove_file`, the primary entrypoint to sandboxed file
//! removal.

use crate::fs::remove_file_impl;
#[cfg(racy_asserts)]
use crate::fs::{
    manually, map_result, remove_file_unchecked, stat_unchecked, FollowSymlinks, Metadata,
};
use std::path::Path;
use std::{fs, io};

/// Perform a `remove_fileat`-like operation, ensuring that the resolution of
/// the path never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn remove_file(start: &fs::File, path: &Path) -> io::Result<()> {
    #[cfg(racy_asserts)]
    let stat_before = stat_unchecked(start, path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = remove_file_impl(start, path);

    #[cfg(racy_asserts)]
    let stat_after = stat_unchecked(start, path, FollowSymlinks::No);

    #[cfg(racy_asserts)]
    check_remove_file(start, path, &stat_before, &result, &stat_after);

    result
}

#[cfg(racy_asserts)]
#[allow(clippy::enum_glob_use)]
fn check_remove_file(
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
            assert!(!metadata.is_dir());
        }

        (Err((Other, _)), Ok(()), Err((NotFound, _))) => {
            // TODO: Check that the path was inside the sandbox.
        }

        (_, Err((_kind, _message)), _) => {
            match map_result(&manually::canonicalize_with(
                start,
                path,
                FollowSymlinks::No,
            )) {
                Ok(canon) => match map_result(&remove_file_unchecked(start, &canon)) {
                    Err((_unchecked_kind, _unchecked_message)) => {
                        /* TODO: Check error messages.
                        assert_eq!(
                            kind,
                            unchecked_kind,
                            "unexpected error kind from remove_file start='{:?}', \
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
                    _ => panic!("unsandboxed remove_file success"),
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
            "inconsistent remove_file checks: start='{:?}' path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }

    match (result, stat_after) {
        (Ok(()), Ok(_unchecked_metadata)) => panic!(
            "file still exists after remove_file start='{:?}', path='{}'",
            start,
            path.display()
        ),
        (Err(e), Ok(unchecked_metadata)) => match e.kind() {
            #[cfg(io_error_more)]
            io::ErrorKind::IsADirectory => assert!(unchecked_metadata.is_dir()),
            io::ErrorKind::PermissionDenied => (),
            #[cfg(not(io_error_more))]
            io::ErrorKind::Other if unchecked_metadata.is_dir() => (),
            _ => panic!(
                "unexpected error removing file start='{:?}', path='{}': {:?}",
                start,
                path.display(),
                e
            ),
        },
        (Ok(()), Err(_unchecked_error)) => (),
        (Err(result_error), Err(_unchecked_error)) => match result_error.kind() {
            io::ErrorKind::PermissionDenied => (),
            _ => {
                /* TODO: Check error messages.
                assert_eq!(result_error.to_string(), unchecked_error.to_string());
                assert_eq!(result_error.kind(), unchecked_error.kind());
                */
            }
        },
    }
}
