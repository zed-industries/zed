//! This defines `symlink`, the primary entrypoint to sandboxed symlink
//! creation.

use crate::fs::errors;
#[cfg(all(racy_asserts, not(windows)))]
use crate::fs::symlink_unchecked;
#[cfg(racy_asserts)]
use crate::fs::{canonicalize, manually, map_result, stat_unchecked, FollowSymlinks, Metadata};
#[cfg(all(racy_asserts, windows))]
use crate::fs::{symlink_dir_unchecked, symlink_file_unchecked};
use std::path::Path;
use std::{fs, io};

/// Perform a `symlinkat`-like operation, ensuring that the resolution of the
/// path never escapes the directory tree rooted at `start`. An error is
/// returned if the target path is absolute.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[cfg(not(windows))]
#[inline]
pub fn symlink(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    // Don't allow creating symlinks to absolute paths. This isn't strictly
    // necessary to preserve the sandbox, since `open` will refuse to follow
    // absolute symlinks in any case. However, it is useful to enforce this
    // restriction so that a WASI program can't trick some other non-WASI
    // program into following an absolute path.
    if old_path.has_root() {
        return Err(errors::escape_attempt());
    }

    write_symlink_impl(old_path, new_start, new_path)
}

#[cfg(not(windows))]
fn write_symlink_impl(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_impl;

    #[cfg(racy_asserts)]
    let stat_before = stat_unchecked(new_start, new_path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = symlink_impl(old_path, new_start, new_path);

    #[cfg(racy_asserts)]
    let stat_after = stat_unchecked(new_start, new_path, FollowSymlinks::No);

    #[cfg(racy_asserts)]
    check_symlink(
        old_path,
        new_start,
        new_path,
        &stat_before,
        &result,
        &stat_after,
    );

    result
}

/// Perform a `symlinkat`-like operation, ensuring that the resolution of the
/// link path never escapes the directory tree rooted at `start`.
#[cfg(not(windows))]
pub fn symlink_contents<P: AsRef<Path>, Q: AsRef<Path>>(
    old_path: P,
    new_start: &fs::File,
    new_path: Q,
) -> io::Result<()> {
    write_symlink_impl(old_path.as_ref(), new_start, new_path.as_ref())
}

/// Perform a `symlink_file`-like operation, ensuring that the resolution of
/// the path never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[cfg(windows)]
#[inline]
pub fn symlink_file(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_file_impl;

    // As above, don't allow creating symlinks to absolute paths.
    if old_path.has_root() {
        return Err(errors::escape_attempt());
    }

    #[cfg(racy_asserts)]
    let stat_before = stat_unchecked(new_start, new_path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = symlink_file_impl(old_path, new_start, new_path);

    #[cfg(racy_asserts)]
    let stat_after = stat_unchecked(new_start, new_path, FollowSymlinks::No);

    #[cfg(racy_asserts)]
    check_symlink_file(
        old_path,
        new_start,
        new_path,
        &stat_before,
        &result,
        &stat_after,
    );

    result
}

/// Perform a `symlink_dir`-like operation, ensuring that the resolution of the
/// path never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[cfg(windows)]
#[inline]
pub fn symlink_dir(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    use crate::fs::symlink_dir_impl;

    // As above, don't allow creating symlinks to absolute paths.
    if old_path.has_root() {
        return Err(errors::escape_attempt());
    }

    #[cfg(racy_asserts)]
    let stat_before = stat_unchecked(new_start, new_path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = symlink_dir_impl(old_path, new_start, new_path);

    #[cfg(racy_asserts)]
    let stat_after = stat_unchecked(new_start, new_path, FollowSymlinks::No);

    #[cfg(racy_asserts)]
    check_symlink_dir(
        old_path,
        new_start,
        new_path,
        &stat_before,
        &result,
        &stat_after,
    );

    result
}

#[cfg(all(not(windows), racy_asserts))]
#[allow(clippy::enum_glob_use)]
fn check_symlink(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
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
        (Err((NotFound, _)), Ok(()), Ok(metadata)) => {
            assert!(metadata.file_type().is_symlink());
            let canon =
                manually::canonicalize_with(new_start, new_path, FollowSymlinks::No).unwrap();
            assert_same_file_metadata!(
                &stat_unchecked(new_start, &canon, FollowSymlinks::No).unwrap(),
                &metadata
            );
        }

        (Ok(metadata_before), Err((AlreadyExists, _)), Ok(metadata_after)) => {
            assert_same_file_metadata!(&metadata_before, &metadata_after);
        }

        (_, Err((_kind, _message)), _) => match map_result(&canonicalize(new_start, new_path)) {
            Ok(canon) => match map_result(&symlink_unchecked(old_path, new_start, &canon)) {
                Err((_unchecked_kind, _unchecked_message)) => {
                    /* TODO: Check error messages.
                    assert_eq!(
                        kind,
                        unchecked_kind,
                        "unexpected error kind from symlink new_start='{:?}', \
                         new_path='{}':\nstat_before={:#?}\nresult={:#?}\nstat_after={:#?}",
                        new_start,
                        new_path.display(),
                        stat_before,
                        result,
                        stat_after
                    );
                    assert_eq!(message, unchecked_message);
                    */
                }
                _ => panic!("unsandboxed symlink success"),
            },
            Err((_canon_kind, _canon_message)) => {
                /* TODO: Check error messages.
                assert_eq!(kind, canon_kind);
                assert_eq!(message, canon_message);
                */
            }
        },

        _other => {
            /* TODO: Check error messages.
            panic!(
                "inconsistent symlink checks: new_start='{:?}' new_path='{}':\n{:#?}",
                new_start,
                new_path.display(),
                other,
            )
            */
        }
    }
}

#[cfg(all(windows, racy_asserts))]
#[allow(clippy::enum_glob_use)]
fn check_symlink_file(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
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
        (Err((NotFound, _)), Ok(()), Ok(metadata)) => {
            assert!(metadata.file_type().is_symlink());
            let canon =
                manually::canonicalize_with(new_start, new_path, FollowSymlinks::No).unwrap();
            assert_same_file_metadata!(
                &stat_unchecked(new_start, &canon, FollowSymlinks::No).unwrap(),
                &metadata
            );
        }

        (Ok(metadata_before), Err((AlreadyExists, _)), Ok(metadata_after)) => {
            assert_same_file_metadata!(&metadata_before, &metadata_after);
        }

        (_, Err((_kind, _message)), _) => match map_result(&canonicalize(new_start, new_path)) {
            Ok(canon) => match map_result(&symlink_file_unchecked(old_path, new_start, &canon)) {
                Err((_unchecked_kind, _unchecked_message)) => {
                    /* TODO: Check error messages.
                    assert_eq!(
                        kind,
                        unchecked_kind,
                        "unexpected error kind from symlink new_start='{:?}', \
                         new_path='{}':\nstat_before={:#?}\nresult={:#?}\nstat_after={:#?}",
                        new_start,
                        new_path.display(),
                        stat_before,
                        result,
                        stat_after
                    );
                    assert_eq!(message, unchecked_message);
                    */
                }
                _ => panic!("unsandboxed symlink success"),
            },
            Err((_canon_kind, _canon_message)) => {
                /* TODO: Check error messages.
                assert_eq!(kind, canon_kind);
                assert_eq!(message, canon_message);
                */
            }
        },

        _other => {
            /* TODO: Check error messages.
            panic!(
                "inconsistent symlink checks: new_start='{:?}' new_path='{}':\n{:#?}",
                new_start,
                new_path.display(),
                other,
            )
            */
        }
    }
}

#[cfg(all(windows, racy_asserts))]
#[allow(clippy::enum_glob_use)]
fn check_symlink_dir(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
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
        (Err((NotFound, _)), Ok(()), Ok(metadata)) => {
            assert!(metadata.file_type().is_symlink());
            let canon =
                manually::canonicalize_with(new_start, new_path, FollowSymlinks::No).unwrap();
            assert_same_file_metadata!(
                &stat_unchecked(new_start, &canon, FollowSymlinks::No).unwrap(),
                &metadata
            );
        }

        (Ok(metadata_before), Err((AlreadyExists, _)), Ok(metadata_after)) => {
            assert_same_file_metadata!(&metadata_before, &metadata_after);
        }

        (_, Err((_kind, _message)), _) => match map_result(&canonicalize(new_start, new_path)) {
            Ok(canon) => match map_result(&symlink_dir_unchecked(old_path, new_start, &canon)) {
                Err((_unchecked_kind, _unchecked_message)) => {
                    /* TODO: Check error messages.
                    assert_eq!(
                        kind,
                        unchecked_kind,
                        "unexpected error kind from symlink new_start='{:?}', \
                         new_path='{}':\nstat_before={:#?}\nresult={:#?}\nstat_after={:#?}",
                        new_start,
                        new_path.display(),
                        stat_before,
                        result,
                        stat_after
                    );
                    assert_eq!(message, unchecked_message);
                    */
                }
                _ => panic!("unsandboxed symlink success"),
            },
            Err((_canon_kind, _canon_message)) => {
                /* TODO: Check error messages.
                assert_eq!(kind, canon_kind);
                assert_eq!(message, canon_message);
                */
            }
        },

        _other => {
            /* TODO: Check error messages.
            panic!(
                "inconsistent symlink checks: new_start='{:?}' new_path='{}':\n{:#?}",
                new_start,
                new_path.display(),
                other,
            )
            */
        }
    }
}
