//! This defines `create_dir`, the primary entrypoint to sandboxed directory
//! creation.

#[cfg(racy_asserts)]
use crate::fs::{
    canonicalize, create_dir_unchecked, map_result, stat_unchecked, FollowSymlinks, Metadata,
};
use crate::fs::{create_dir_impl, DirOptions};
use std::path::Path;
use std::{fs, io};

/// Perform a `mkdirat`-like operation, ensuring that the resolution of the
/// path never escapes the directory tree rooted at `start`.
#[cfg_attr(not(racy_asserts), allow(clippy::let_and_return))]
#[inline]
pub fn create_dir(start: &fs::File, path: &Path, options: &DirOptions) -> io::Result<()> {
    #[cfg(racy_asserts)]
    let stat_before = stat_unchecked(start, path, FollowSymlinks::No);

    // Call the underlying implementation.
    let result = create_dir_impl(start, path, options);

    #[cfg(racy_asserts)]
    let stat_after = stat_unchecked(start, path, FollowSymlinks::No);

    #[cfg(racy_asserts)]
    check_create_dir(start, path, options, &stat_before, &result, &stat_after);

    result
}

#[cfg(racy_asserts)]
#[allow(clippy::enum_glob_use)]
fn check_create_dir(
    start: &fs::File,
    path: &Path,
    options: &DirOptions,
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
            assert!(metadata.is_dir());
            assert_same_file_metadata!(
                &stat_unchecked(
                    start,
                    &canonicalize(start, path).unwrap(),
                    FollowSymlinks::No
                )
                .unwrap(),
                &metadata
            );
        }

        (Ok(metadata_before), Err((AlreadyExists, _)), Ok(metadata_after)) => {
            assert_same_file_metadata!(&metadata_before, &metadata_after);
        }

        (_, Err((kind, message)), _) => {
            // TODO: Checking in the case it does end with ".".
            if !path.to_string_lossy().ends_with(".") {
                match map_result(&canonicalize(start, path)) {
                    Ok(canon) => match map_result(&create_dir_unchecked(start, &canon, options)) {
                        Err((unchecked_kind, unchecked_message)) => {
                            assert_eq!(
                                kind,
                                unchecked_kind,
                                "unexpected error kind from create_dir start='{:?}', \
                                 path='{}':\nstat_before={:#?}\nresult={:#?}\nstat_after={:#?}",
                                start,
                                path.display(),
                                stat_before,
                                result,
                                stat_after
                            );
                            assert_eq!(message, unchecked_message);
                        }
                        _ => panic!("unsandboxed create_dir success"),
                    },
                    Err((_canon_kind, _canon_message)) => {
                        /* TODO: Check error messages
                        assert_eq!(kind, canon_kind);
                        assert_eq!(message, canon_message);
                        */
                    }
                }
            }
        }

        other => panic!(
            "inconsistent create_dir checks: start='{:?}' path='{}':\n{:#?}",
            start,
            path.display(),
            other,
        ),
    }
}
