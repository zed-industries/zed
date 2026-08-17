//! This defines `rename`, the primary entrypoint to sandboxed renaming.

#[cfg(all(racy_asserts, not(windows)))]
use crate::fs::append_dir_suffix;
use crate::fs::rename_impl;
use std::path::Path;
use std::{fs, io};
#[cfg(racy_asserts)]
use {
    crate::fs::{
        manually, map_result, path_requires_dir, rename_unchecked, stat_unchecked, FollowSymlinks,
        Metadata,
    },
    std::path::PathBuf,
};

/// Perform a `renameat`-like operation, ensuring that the resolution of both
/// the old and new paths never escape the directory tree rooted at their
/// respective starts.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn rename(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    #[cfg(racy_asserts)]
    let (old_metadata_before, new_metadata_before) = (
        stat_unchecked(old_start, old_path, FollowSymlinks::No),
        stat_unchecked(new_start, new_path, FollowSymlinks::No),
    );

    // Call the underlying implementation.
    let result = rename_impl(old_start, old_path, new_start, new_path);

    #[cfg(racy_asserts)]
    let (old_metadata_after, new_metadata_after) = (
        stat_unchecked(old_start, old_path, FollowSymlinks::No),
        stat_unchecked(new_start, new_path, FollowSymlinks::No),
    );

    #[cfg(racy_asserts)]
    check_rename(
        old_start,
        old_path,
        new_start,
        new_path,
        &old_metadata_before,
        &new_metadata_before,
        &result,
        &old_metadata_after,
        &new_metadata_after,
    );

    result
}

#[cfg(racy_asserts)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::enum_glob_use)]
fn check_rename(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
    old_metadata_before: &io::Result<Metadata>,
    new_metadata_before: &io::Result<Metadata>,
    result: &io::Result<()>,
    old_metadata_after: &io::Result<Metadata>,
    new_metadata_after: &io::Result<Metadata>,
) {
    use io::ErrorKind::*;

    match (
        map_result(old_metadata_before),
        map_result(new_metadata_before),
        map_result(result),
        map_result(old_metadata_after),
        map_result(new_metadata_after),
    ) {
        (
            Ok(old_metadata_before),
            Err((NotFound, _)),
            Ok(()),
            Err((NotFound, _)),
            Ok(new_metadata_after),
        ) => {
            assert_same_file_metadata!(&old_metadata_before, &new_metadata_after);
        }

        (_, Ok(new_metadata_before), Err((AlreadyExists, _)), _, Ok(new_metadata_after)) => {
            assert_same_file_metadata!(&new_metadata_before, &new_metadata_after);
        }

        (_, _, Err((kind, message)), _, _) => match (
            map_result(&canonicalize_for_rename(old_start, old_path)),
            map_result(&canonicalize_for_rename(new_start, new_path)),
        ) {
            (Ok(old_canon), Ok(new_canon)) => match map_result(&rename_unchecked(
                old_start, &old_canon, new_start, &new_canon,
            )) {
                Err((_unchecked_kind, _unchecked_message)) => {
                    /* TODO: Check error messages.
                    assert_eq!(kind, unchecked_kind);
                    assert_eq!(message, unchecked_message);
                    */
                }
                other => panic!(
                    "unsandboxed rename success:\n{:#?}\n{:?} {:?}",
                    other, kind, message
                ),
            },
            (Err((_old_canon_kind, _old_canon_message)), _) => {
                /* TODO: Check error messages.
                assert_eq!(kind, old_canon_kind);
                assert_eq!(message, old_canon_message);
                */
            }
            (_, Err((_new_canon_kind, _new_canon_message))) => {
                /* TODO: Check error messages.
                assert_eq!(kind, new_canon_kind);
                assert_eq!(message, new_canon_message);
                */
            }
        },

        _other => {
            /* TODO: Check error messages.
            panic!(
                "inconsistent rename checks: old_start='{:?}', old_path='{}', new_start='{:?}', \
                 new_path='{}':\n{:#?}",
                old_start,
                old_path.display(),
                new_start,
                new_path.display(),
                other
            )
            */
        }
    }
}

#[cfg(racy_asserts)]
fn canonicalize_for_rename(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    let mut canon = manually::canonicalize_with(start, path, FollowSymlinks::No)?;

    // Rename on paths ending in `.` or `/.` fails due to the directory already
    // being open. Ensure that this happens on the canonical paths too.
    #[cfg(not(windows))]
    if path_requires_dir(path) {
        canon = append_dir_suffix(path.to_path_buf());

        assert!(path_requires_dir(&canon));
    }

    Ok(canon)
}
