//! This defines `read_link`, the primary entrypoint to sandboxed symbolic link
//! dereferencing.

use crate::fs::{errors, read_link_impl};
#[cfg(racy_asserts)]
use crate::fs::{map_result, read_link_unchecked, stat, FollowSymlinks};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Perform a `readlinkat`-like operation, ensuring that the resolution of the
/// link path never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn read_link_contents(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Call the underlying implementation.
    let result = read_link_impl(start, path);

    #[cfg(racy_asserts)]
    let unchecked = read_link_unchecked(start, path, PathBuf::new());

    #[cfg(racy_asserts)]
    check_read_link(start, path, &result, &unchecked);

    result
}

/// Perform a `readlinkat`-like operation, ensuring that the resolution of the
/// path never escapes the directory tree rooted at `start`, and also verifies
/// that the link target is not absolute.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn read_link(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // Call the underlying implementation.
    let result = read_link_contents(start, path);

    // Don't allow reading symlinks to absolute paths. This isn't strictly
    // necessary to preserve the sandbox, since `open` will refuse to follow
    // absolute paths in any case. However, it is useful to enforce this
    // restriction to avoid leaking information about the host filesystem
    // outside the sandbox.
    if let Ok(path) = &result {
        if path.has_root() {
            return Err(errors::escape_attempt());
        }
    }

    result
}

#[cfg(racy_asserts)]
#[allow(clippy::enum_glob_use)]
fn check_read_link(
    start: &fs::File,
    path: &Path,
    result: &io::Result<PathBuf>,
    unchecked: &io::Result<PathBuf>,
) {
    use io::ErrorKind::*;

    match (map_result(result), map_result(unchecked)) {
        (Ok(target), Ok(unchecked_target)) => {
            assert_eq!(target, unchecked_target);
        }

        (Err((PermissionDenied, message)), _) => {
            match map_result(&stat(start, path, FollowSymlinks::No)) {
                Err((PermissionDenied, canon_message)) => {
                    assert_eq!(message, canon_message);
                }
                _ => panic!("read_link failed where canonicalize succeeded"),
            }
        }

        (Err((_kind, _message)), Err((_unchecked_kind, _unchecked_message))) => {
            /* TODO: Check error messages.
            assert_eq!(kind, unchecked_kind);
            assert_eq!(message, unchecked_message);
            */
        }

        other => panic!(
            "unexpected result from read_link start='{:?}', path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }
}

#[cfg(not(windows))]
#[test]
fn test_read_link_contents() {
    use io_lifetimes::AsFilelike;
    let td = cap_tempfile::tempdir(cap_tempfile::ambient_authority()).unwrap();
    let td_view = &td.as_filelike_view::<std::fs::File>();
    let valid = [
        "relative/path",
        "/some/absolute/path",
        "/",
        "../",
        "basepath",
    ];
    for case in valid {
        let linkname = Path::new("linkname");
        crate::fs::symlink_contents(case, td_view, linkname).unwrap();
        let contents = crate::fs::read_link_contents(td_view, linkname).unwrap();
        assert_eq!(contents.to_str().unwrap(), case);
        crate::fs::remove_file(td_view, linkname).unwrap();
    }
}
