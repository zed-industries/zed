//! This defines `open`, the primary entrypoint to sandboxed file and directory
//! opening.

#[cfg(racy_asserts)]
use crate::fs::{file_path, open_unchecked, stat_unchecked, Metadata};
use crate::fs::{open_impl, OpenOptions};
use std::path::Path;
use std::{fs, io};

/// Perform an `openat`-like operation, ensuring that the resolution of the
/// path never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn open(start: &fs::File, path: &Path, options: &OpenOptions) -> io::Result<fs::File> {
    #[cfg(racy_asserts)]
    let stat_before = stat_unchecked(start, path, options.follow);

    // Call the underlying implementation.
    let result = open_impl(start, path, options);

    #[cfg(racy_asserts)]
    let stat_after = stat_unchecked(start, path, options.follow);

    #[cfg(racy_asserts)]
    check_open(start, path, options, &stat_before, &result, &stat_after);

    result
}

#[cfg(racy_asserts)]
fn check_open(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
    _stat_before: &io::Result<Metadata>,
    result: &io::Result<fs::File>,
    _stat_after: &io::Result<Metadata>,
) {
    let unchecked_result = open_unchecked(
        start,
        path,
        options
            .clone()
            .create(false)
            .create_new(false)
            .truncate(false),
    );

    match (&result, &unchecked_result) {
        (Ok(result_file), Ok(unchecked_file)) => {
            assert_same_file!(
                &result_file,
                &unchecked_file,
                "path resolution inconsistency: start='{:?}', path='{}'",
                start,
                path.display(),
            );
        }
        (Ok(result_file), Err(unchecked_error)) => {
            if unchecked_error.kind() == io::ErrorKind::PermissionDenied {
                assert!(options.create || options.create_new);
            } else {
                panic!(
                    "unexpected success opening start='{:?}', path='{}'; expected {:?}; got {:?}",
                    start,
                    path.display(),
                    unchecked_error,
                    result_file
                );
            }
        }
        (Err(result_error), Ok(_unchecked_file)) => match result_error.kind() {
            io::ErrorKind::PermissionDenied | io::ErrorKind::InvalidInput => (),
            io::ErrorKind::AlreadyExists if options.create_new => (),
            _ => panic!(
                "unexpected error opening start='{:?}', path='{}': {:?}",
                start,
                path.display(),
                result_error
            ),
        },
        (Err(result_error), Err(_unchecked_error)) => match result_error.kind() {
            io::ErrorKind::PermissionDenied | io::ErrorKind::InvalidInput => (),
            _ => {
                /* TODO: Check error messages.
                let unchecked_error = unchecked_error.into();
                assert_eq!(result_error.to_string(), unchecked_error.to_string());
                assert_eq!(result_error.kind(), unchecked_error.kind());
                */
            }
        },
    }

    // On operating systems which can tell us the path of a file descriptor,
    // assert that the start path is a parent of the result path.
    if let Ok(result_file) = &result {
        if let Some(result_path) = file_path(result_file) {
            if let Some(start_path) = file_path(start) {
                assert!(
                    result_path.starts_with(start_path),
                    "sandbox escape: start='{:?}' result='{}'",
                    start,
                    result_path.display()
                );
            }
        }
    }
}
