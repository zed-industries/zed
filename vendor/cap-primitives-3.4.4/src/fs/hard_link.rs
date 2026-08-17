//! This defines `hard_link`, the primary entrypoint to sandboxed hard-link
//! creation.

use crate::fs::hard_link_impl;
#[cfg(racy_asserts)]
use crate::fs::{
    canonicalize, hard_link_unchecked, map_result, stat_unchecked, FollowSymlinks, Metadata,
};
use std::path::Path;
use std::{fs, io};

/// Perform a `linkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn hard_link(
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
    let result = hard_link_impl(old_start, old_path, new_start, new_path);

    #[cfg(racy_asserts)]
    let (old_metadata_after, new_metadata_after) = (
        stat_unchecked(old_start, old_path, FollowSymlinks::No),
        stat_unchecked(new_start, new_path, FollowSymlinks::No),
    );

    #[cfg(racy_asserts)]
    check_hard_link(
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
fn check_hard_link(
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
            Ok(old_metadata_after),
            Ok(new_metadata_after),
        ) => {
            assert_same_file_metadata!(old_metadata_before, old_metadata_after);
            assert_same_file_metadata!(old_metadata_before, new_metadata_after);
        }

        (_, Ok(new_metadata_before), Err((AlreadyExists, _)), _, Ok(new_metadata_after)) => {
            assert_same_file_metadata!(&new_metadata_before, &new_metadata_after);
        }

        (_, _, Err((_kind, _message)), _, _) => match (
            map_result(&canonicalize(old_start, old_path)),
            map_result(&canonicalize(new_start, new_path)),
        ) {
            (Ok(old_canon), Ok(new_canon)) => match map_result(&hard_link_unchecked(
                old_start, &old_canon, new_start, &new_canon,
            )) {
                Err((_unchecked_kind, _unchecked_message)) => {
                    /* TODO: Check error messages.
                    assert_eq!(kind, unchecked_kind);
                    assert_eq!(message, unchecked_message);
                    */
                }
                _ => panic!("unsandboxed link success"),
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

        other => panic!(
            "inconsistent link checks: old_start='{:?}', old_path='{}', new_start='{:?}', \
             new_path='{}':\n{:#?}",
            old_start,
            old_path.display(),
            new_start,
            new_path.display(),
            other
        ),
    }
}
