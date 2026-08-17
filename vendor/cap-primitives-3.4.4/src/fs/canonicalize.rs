//! Sandboxed path canonicalization.

use crate::fs::canonicalize_impl;
#[cfg(racy_asserts)]
use crate::fs::{file_path, open, OpenOptions};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Canonicalize the given path, ensuring that the resolution of the path never
/// escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn canonicalize(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Call the underlying implementation.
    let result = canonicalize_impl(start, path);

    #[cfg(racy_asserts)]
    check_canonicalize(start, path, &result);

    result
}

#[cfg(racy_asserts)]
fn check_canonicalize(start: &fs::File, path: &Path, result: &io::Result<PathBuf>) {
    if let Ok(canonical_path) = result {
        let path_result = open(start, path, OpenOptions::new().read(true));
        let canonical_result = open(start, canonical_path, OpenOptions::new().read(true));
        match (path_result, canonical_result) {
            (Ok(path_file), Ok(canonical_file)) => assert_same_file!(
                &path_file,
                &canonical_file,
                "we should be able to stat paths that we just canonicalized"
            ),
            (Err(path_err), Err(canonical_err)) => {
                assert_eq!(path_err.to_string(), canonical_err.to_string())
            }
            other => {
                // TODO: Checking in the case it does end with ".".
                if !path.to_string_lossy().ends_with("/.") {
                    panic!("inconsistent canonicalize checks: {:?}", other);
                }
            }
        }

        // On operating systems which can tell us the path of a file descriptor,
        // assert that the path we computed canonicalizes to the same thing as
        // the input canonicalizes too.
        if let Some(start_abspath) = file_path(start) {
            let check_abspath = start_abspath.join(path);
            let result_abspath = start_abspath.join(canonical_path);
            if let Ok(check_abspath) = fs::canonicalize(check_abspath) {
                let result_abspath =
                    fs::canonicalize(result_abspath).expect("we already canonicalized this");
                assert_eq!(
                    check_abspath,
                    result_abspath,
                    "incorrect canonicalization: start='{:?}' path='{}' result='{}'",
                    start,
                    path.display(),
                    canonical_path.display()
                );
                // TODO: When porting to Windows, check whether `start_abspath` not being
                // a canonicalized path leads to `\\?\` extended path prefix differences.
                assert!(
                    result_abspath.starts_with(start_abspath),
                    "sandbox escape: start='{:?}' path='{}' result='{}'",
                    start,
                    path.display(),
                    canonical_path.display()
                );
            }
        }
    }
}
