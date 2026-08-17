use crate::error::{Error, Result};
use crate::gen::fs;
use crate::paths;
use std::path::{Component, Path, PathBuf};
use std::{env, io};

pub(crate) fn write(path: impl AsRef<Path>, content: &[u8]) -> Result<()> {
    let path = path.as_ref();

    let mut create_dir_error = None;
    if fs::exists(path) {
        if let Ok(existing) = fs::read(path) {
            if existing == content {
                // Avoid bumping modified time with unchanged contents.
                return Ok(());
            }
        }
        best_effort_remove(path);
    } else {
        let parent = path.parent().unwrap();
        create_dir_error = fs::create_dir_all(parent).err();
    }

    match fs::write(path, content) {
        // As long as write succeeded, ignore any create_dir_all error.
        Ok(()) => Ok(()),
        // If create_dir_all and write both failed, prefer the first error.
        Err(err) => Err(Error::Fs(create_dir_error.unwrap_or(err))),
    }
}

pub(crate) fn relative_symlink_file(
    original: impl AsRef<Path>,
    link: impl AsRef<Path>,
) -> Result<()> {
    let original = original.as_ref();
    let link = link.as_ref();

    let parent_directory_error = prepare_parent_directory_for_symlink(link).err();
    let relativized = best_effort_relativize_symlink(original, link);

    symlink_file(&relativized, original, link, parent_directory_error)
}

pub(crate) fn absolute_symlink_file(
    original: impl AsRef<Path>,
    link: impl AsRef<Path>,
) -> Result<()> {
    let original = original.as_ref();
    let link = link.as_ref();

    let parent_directory_error = prepare_parent_directory_for_symlink(link).err();

    symlink_file(original, original, link, parent_directory_error)
}

pub(crate) fn relative_symlink_dir(
    original: impl AsRef<Path>,
    link: impl AsRef<Path>,
) -> Result<()> {
    let original = original.as_ref();
    let link = link.as_ref();

    let parent_directory_error = prepare_parent_directory_for_symlink(link).err();
    let relativized = best_effort_relativize_symlink(original, link);

    symlink_dir(&relativized, link, parent_directory_error)
}

fn prepare_parent_directory_for_symlink(link: &Path) -> fs::Result<()> {
    if fs::exists(link) {
        best_effort_remove(link);
        Ok(())
    } else {
        let parent = link.parent().unwrap();
        fs::create_dir_all(parent)
    }
}

fn symlink_file(
    path_for_symlink: &Path,
    path_for_copy: &Path,
    link: &Path,
    parent_directory_error: Option<fs::Error>,
) -> Result<()> {
    match paths::symlink_or_copy(path_for_symlink, path_for_copy, link) {
        // As long as symlink_or_copy succeeded, ignore any create_dir_all error.
        Ok(()) => Ok(()),
        Err(err) => {
            if err.kind() == io::ErrorKind::AlreadyExists {
                // This is fine, a different simultaneous build script already
                // created the same link or copy. The cxx_build target directory
                // is laid out such that the same path never refers to two
                // different targets during the same multi-crate build, so if
                // some other build script already created the same path then we
                // know it refers to the identical target that the current build
                // script was trying to create.
                Ok(())
            } else {
                // If create_dir_all and symlink_or_copy both failed, prefer the
                // first error.
                Err(Error::Fs(parent_directory_error.unwrap_or(err)))
            }
        }
    }
}

fn symlink_dir(
    path_for_symlink: &Path,
    link: &Path,
    parent_directory_error: Option<fs::Error>,
) -> Result<()> {
    match fs::symlink_dir(path_for_symlink, link) {
        // As long as symlink_dir succeeded, ignore any create_dir_all error.
        Ok(()) => Ok(()),
        // If create_dir_all and symlink_dir both failed, prefer the first error.
        Err(err) => Err(Error::Fs(parent_directory_error.unwrap_or(err))),
    }
}

fn best_effort_remove(path: &Path) {
    use std::fs;

    if cfg!(windows) {
        // On Windows, the correct choice of remove_file vs remove_dir needs to
        // be used according to what the symlink *points to*. Trying to use
        // remove_file to remove a symlink which points to a directory fails
        // with "Access is denied".
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.is_dir() {
                let _ = fs::remove_dir_all(path);
            } else {
                let _ = fs::remove_file(path);
            }
        } else if fs::symlink_metadata(path).is_ok() {
            // The symlink might exist but be dangling, in which case there is
            // no standard way to determine what "kind" of symlink it is. Try
            // deleting both ways.
            if fs::remove_dir_all(path).is_err() {
                let _ = fs::remove_file(path);
            }
        }
    } else {
        // On non-Windows, we check metadata not following symlinks. All
        // symlinks are removed using remove_file.
        if let Ok(metadata) = fs::symlink_metadata(path) {
            if metadata.is_dir() {
                let _ = fs::remove_dir_all(path);
            } else {
                let _ = fs::remove_file(path);
            }
        }
    }
}

fn best_effort_relativize_symlink(original: impl AsRef<Path>, link: impl AsRef<Path>) -> PathBuf {
    let original = original.as_ref();
    let link = link.as_ref();

    let Some(relative_path) = abstractly_relativize_symlink(original, link) else {
        return original.to_path_buf();
    };

    // Sometimes "a/b/../c" refers to a different canonical location than "a/c".
    // This can happen if 'b' is a symlink. The '..' canonicalizes to the parent
    // directory of the symlink's target, not back to 'a'. In cxx-build's case
    // someone could be using `--target-dir` with a location containing such
    // symlinks.
    if let Ok(original_canonical) = original.canonicalize() {
        if let Ok(relative_canonical) = link.parent().unwrap().join(&relative_path).canonicalize() {
            if original_canonical == relative_canonical {
                return relative_path;
            }
        }
    }

    original.to_path_buf()
}

fn abstractly_relativize_symlink(
    original: impl AsRef<Path>,
    link: impl AsRef<Path>,
) -> Option<PathBuf> {
    let original = original.as_ref();
    let link = link.as_ref();

    // Relativization only makes sense if there is a semantically meaningful
    // base directory shared between the two paths.
    //
    // For example /Volumes/code/library/src/lib.rs
    //         and /Volumes/code/library/target/path/to/something.a
    // have a meaningful shared base of /Volumes/code/library. The target and
    // source directory only likely ever get relocated as one unit.
    //
    // On the other hand, /Volumes/code/library/src/lib.rs
    //                and /Volumes/shared_target
    // do not, since upon moving library to a different location it should
    // continue referring to the original location of that shared Cargo target
    // directory.
    let likely_no_semantic_prefix = env::var_os("CARGO_TARGET_DIR").is_some();

    if likely_no_semantic_prefix
        || original.is_relative()
        || link.is_relative()
        || path_contains_intermediate_components(original)
        || path_contains_intermediate_components(link)
    {
        return None;
    }

    let (common_prefix, rest_of_original, rest_of_link) = split_after_common_prefix(original, link);

    if common_prefix == Path::new("") {
        return None;
    }

    let mut rest_of_link = rest_of_link.components();
    rest_of_link
        .next_back()
        .expect("original can't be a subdirectory of link");

    let mut path_to_common_prefix = PathBuf::new();
    while rest_of_link.next_back().is_some() {
        path_to_common_prefix.push(Component::ParentDir);
    }

    Some(path_to_common_prefix.join(rest_of_original))
}

fn path_contains_intermediate_components(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .components()
        .any(|component| component == Component::ParentDir)
}

fn split_after_common_prefix<'first, 'second>(
    first: &'first Path,
    second: &'second Path,
) -> (&'first Path, &'first Path, &'second Path) {
    let entire_first = first;
    let mut first = first.components();
    let mut second = second.components();
    loop {
        let rest_of_first = first.as_path();
        let rest_of_second = second.as_path();
        match (first.next(), second.next()) {
            (Some(first_component), Some(second_component))
                if first_component == second_component => {}
            _ => {
                let mut common_prefix = entire_first;
                for _ in rest_of_first.components().rev() {
                    if let Some(parent) = common_prefix.parent() {
                        common_prefix = parent;
                    } else {
                        common_prefix = Path::new("");
                        break;
                    }
                }
                return (common_prefix, rest_of_first, rest_of_second);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::out::abstractly_relativize_symlink;
    use std::path::Path;

    #[cfg(not(windows))]
    #[test]
    fn test_relativize_symlink_unix() {
        assert_eq!(
            abstractly_relativize_symlink("/foo/bar/baz", "/foo/spam/eggs").as_deref(),
            Some(Path::new("../bar/baz")),
        );
        assert_eq!(
            abstractly_relativize_symlink("/foo/bar/../baz", "/foo/spam/eggs"),
            None,
        );
        assert_eq!(
            abstractly_relativize_symlink("/foo/bar/baz", "/foo/spam/./eggs").as_deref(),
            Some(Path::new("../bar/baz")),
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_relativize_symlink_windows() {
        use std::path::PathBuf;

        let windows_target = PathBuf::from_iter(["c:\\", "windows", "foo"]);
        let windows_link = PathBuf::from_iter(["c:\\", "users", "link"]);
        let windows_different_volume_link = PathBuf::from_iter(["d:\\", "users", "link"]);

        assert_eq!(
            abstractly_relativize_symlink(&windows_target, windows_link).as_deref(),
            Some(Path::new("..\\windows\\foo")),
        );
        assert_eq!(
            abstractly_relativize_symlink(&windows_target, windows_different_volume_link),
            None,
        );
    }
}
