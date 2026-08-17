/// `open_parent` is the key building block in all `via_parent` functions.
/// It opens the parent directory of the given path, and returns the basename,
/// so that all the `via_parent` functions need to do is make sure they
/// don't follow symlinks in the basename.
use crate::fs::{errors, open_dir, path_requires_dir, MaybeOwnedFile};
use std::ffi::OsStr;
use std::io;
use std::path::{Component, Path};

/// Open the "parent" of `path`, relative to `start`. The return value on
/// success is a tuple of the newly opened directory and an `OsStr` referencing
/// the single remaining path component. This last component will not be `..`,
/// though it may be `.` or a symbolic link to anywhere (possibly
/// including `..` or an absolute path).
pub(super) fn open_parent<'path, 'borrow>(
    start: MaybeOwnedFile<'borrow>,
    path: &'path Path,
) -> io::Result<(MaybeOwnedFile<'borrow>, &'path OsStr)> {
    let (dirname, basename) = split_parent(path).ok_or_else(errors::no_such_file_or_directory)?;

    let dir = if dirname.as_os_str().is_empty() {
        start
    } else {
        MaybeOwnedFile::owned(open_dir(&start, dirname)?)
    };

    Ok((dir, basename.as_os_str()))
}

/// Split `path` into parent and basename parts. Return `None` if `path`
/// is empty.
///
/// This differs from `path.parent()` and `path.file_name()` in several
/// respects:
///  - Treat paths ending in `/` or `/.` as implying a directory.
///  - Treat the path `.` as a normal component rather than a parent.
///  - Append a `.` to a path with a trailing `..` to avoid requiring our
///    callers to special-case `..`.
///  - Bare absolute paths are ok.
fn split_parent(path: &Path) -> Option<(&Path, Component)> {
    if path.as_os_str().is_empty() {
        return None;
    }

    if !path_requires_dir(path) {
        let mut comps = path.components();
        if let Some(p) = comps.next_back() {
            match p {
                Component::Normal(_) | Component::CurDir => return Some((comps.as_path(), p)),
                _ => (),
            }
        }
    }

    Some((path, Component::CurDir))
}

#[test]
fn split_parent_basics() {
    assert_eq!(
        split_parent(Path::new("foo/bar/qux")).unwrap(),
        (
            Path::new("foo/bar"),
            Component::Normal(Path::new("qux").as_ref())
        )
    );
    assert_eq!(
        split_parent(Path::new("foo/bar")).unwrap(),
        (
            Path::new("foo"),
            Component::Normal(Path::new("bar").as_ref())
        )
    );
    assert_eq!(
        split_parent(Path::new("foo")).unwrap(),
        (Path::new(""), Component::Normal(Path::new("foo").as_ref()))
    );
}

#[test]
fn split_parent_special_cases() {
    assert!(split_parent(Path::new("")).is_none());
    assert_eq!(
        split_parent(Path::new("foo/")).unwrap(),
        (Path::new("foo"), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("foo/.")).unwrap(),
        (Path::new("foo"), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new(".")).unwrap(),
        (Path::new(""), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("..")).unwrap(),
        (Path::new(".."), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("../..")).unwrap(),
        (Path::new("../.."), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("../foo")).unwrap(),
        (
            Path::new(".."),
            Component::Normal(Path::new("foo").as_ref())
        )
    );
    assert_eq!(
        split_parent(Path::new("foo/..")).unwrap(),
        (Path::new("foo/.."), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("/foo")).unwrap(),
        (Path::new("/"), Component::Normal(Path::new("foo").as_ref()))
    );
    assert_eq!(
        split_parent(Path::new("/foo/")).unwrap(),
        (Path::new("/foo"), Component::CurDir)
    );
    assert_eq!(
        split_parent(Path::new("/")).unwrap(),
        (Path::new("/"), Component::CurDir)
    );
}
