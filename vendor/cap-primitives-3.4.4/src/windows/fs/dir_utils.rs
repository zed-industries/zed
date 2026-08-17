use crate::fs::OpenOptionsExt;
use crate::fs::{errors, OpenOptions};
use crate::AmbientAuthority;
use std::ffi::OsString;
use std::ops::Deref;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::{fs, io};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_BACKUP_SEMANTICS, FILE_SHARE_READ, FILE_SHARE_WRITE,
};

/// Rust's `Path` implicitly strips redundant slashes, however they aren't
/// redundant in one case: at the end of a path they indicate that a path is
/// expected to name a directory.
pub(crate) fn path_requires_dir(path: &Path) -> bool {
    let wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    wide.ends_with(&['/' as u16])
        || wide.ends_with(&['/' as u16, '.' as _])
        || wide.ends_with(&['\\' as u16])
        || wide.ends_with(&['\\' as u16, '.' as _])
}

/// Windows treats `foo/.` as equivalent to `foo` even if `foo` does not
/// exist or is not a directory. So we don't do the special trailing-dot
/// handling that we do on Posix-ish platforms.
pub(crate) fn path_has_trailing_dot(_path: &Path) -> bool {
    false
}

/// For the purposes of emulating Windows symlink resolution, we sometimes
/// need to know whether a path really does end in a trailing dot though.
pub(crate) fn path_really_has_trailing_dot(path: &Path) -> bool {
    let wide: Vec<u16> = path.as_os_str().encode_wide().collect();

    wide.ends_with(&['/' as u16, '.' as u16]) || wide.ends_with(&['\\' as u16, '.' as u16])
}

/// Rust's `Path` implicitly strips trailing `/`s, however they aren't
/// redundant in one case: at the end of a path they are the final path
/// component, which has different path lookup behavior.
pub(crate) fn path_has_trailing_slash(path: &Path) -> bool {
    let wide: Vec<u16> = path.as_os_str().encode_wide().collect();

    wide.ends_with(&['/' as u16]) || wide.ends_with(&['\\' as u16])
}

/// Strip trailing `/`s, unless this reduces `path` to `/` itself. This is
/// used by `create_dir` and others to prevent paths like `foo/` from
/// canonicalizing to `foo/.` since these syscalls treat these differently.
pub(crate) fn strip_dir_suffix(path: &Path) -> impl Deref<Target = Path> + '_ {
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    while wide.len() > 1
        && (*wide.last().unwrap() == '/' as u16 || *wide.last().unwrap() == '\\' as u16)
    {
        wide.pop();
    }
    PathBuf::from(OsString::from_wide(&wide))
}

/// Return an `OpenOptions` for opening directories.
pub(crate) fn dir_options() -> OpenOptions {
    // Set `FILE_FLAG_BACKUP_SEMANTICS` so that we can open directories. Unset
    // `FILE_SHARE_DELETE` so that directories can't be renamed or deleted
    // underneath us, since we use paths to implement many directory operations.
    OpenOptions::new()
        .read(true)
        .dir_required(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .clone()
}

/// Like `dir_options`, but additionally request the ability to read the
/// directory entries.
pub(crate) fn readdir_options() -> OpenOptions {
    dir_options().readdir_required(true).clone()
}

/// Return an `OpenOptions` for canonicalizing paths.
pub(crate) fn canonicalize_options() -> OpenOptions {
    OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .clone()
}

/// Open a directory named by a bare path, using the host process' ambient
/// authority.
///
/// # Ambient Authority
///
/// This function is not sandboxed and may trivially access any path that the
/// host process has access to.
pub(crate) fn open_ambient_dir_impl(
    path: &Path,
    ambient_authority: AmbientAuthority,
) -> io::Result<fs::File> {
    use std::os::windows::fs::OpenOptionsExt;

    let _ = ambient_authority;

    // Set `FILE_FLAG_BACKUP_SEMANTICS` so that we can open directories. Unset
    // `FILE_SHARE_DELETE` so that directories can't be renamed or deleted
    // underneath us, since we use paths to implement many directory operations.
    let dir = fs::OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .open(path)?;

    // Require a directory. It may seem possible to eliminate this `metadata()`
    // call by appending a slash to the path before opening it so that the OS
    // requires a directory for us, however on Windows in some circumstances
    // this leads to "The filename, directory name, or volume label syntax is
    // incorrect." errors.
    if !dir.metadata()?.is_dir() {
        return Err(errors::is_not_directory());
    }

    Ok(dir)
}

#[test]
fn strip_dir_suffix_tests() {
    assert_eq!(&*strip_dir_suffix(Path::new("/foo//")), Path::new("/foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("/foo/")), Path::new("/foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("foo/")), Path::new("foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("foo")), Path::new("foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("/")), Path::new("/"));
    assert_eq!(&*strip_dir_suffix(Path::new("//")), Path::new("/"));

    assert_eq!(
        &*strip_dir_suffix(Path::new("\\foo\\\\")),
        Path::new("\\foo")
    );
    assert_eq!(&*strip_dir_suffix(Path::new("\\foo\\")), Path::new("\\foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("foo\\")), Path::new("foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("foo")), Path::new("foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("\\")), Path::new("\\"));
    assert_eq!(&*strip_dir_suffix(Path::new("\\\\")), Path::new("\\"));
}

#[test]
fn test_path_requires_dir() {
    assert!(!path_requires_dir(Path::new(".")));
    assert!(path_requires_dir(Path::new("/")));
    assert!(path_requires_dir(Path::new("//")));
    assert!(path_requires_dir(Path::new("/./.")));
    assert!(path_requires_dir(Path::new("foo/")));
    assert!(path_requires_dir(Path::new("foo//")));
    assert!(path_requires_dir(Path::new("foo//.")));
    assert!(path_requires_dir(Path::new("foo/./.")));
    assert!(path_requires_dir(Path::new("foo/./")));
    assert!(path_requires_dir(Path::new("foo/.//")));

    assert!(path_requires_dir(Path::new("\\")));
    assert!(path_requires_dir(Path::new("\\\\")));
    assert!(path_requires_dir(Path::new("\\.\\.")));
    assert!(path_requires_dir(Path::new("foo\\")));
    assert!(path_requires_dir(Path::new("foo\\\\")));
    assert!(path_requires_dir(Path::new("foo\\\\.")));
    assert!(path_requires_dir(Path::new("foo\\.\\.")));
    assert!(path_requires_dir(Path::new("foo\\.\\")));
    assert!(path_requires_dir(Path::new("foo\\.\\\\")));
}

#[test]
fn test_path_has_trailing_slash() {
    assert!(path_has_trailing_slash(Path::new("/")));
    assert!(path_has_trailing_slash(Path::new("//")));
    assert!(path_has_trailing_slash(Path::new("foo/")));
    assert!(path_has_trailing_slash(Path::new("foo//")));
    assert!(path_has_trailing_slash(Path::new("foo/./")));
    assert!(path_has_trailing_slash(Path::new("foo/.//")));

    assert!(path_has_trailing_slash(Path::new("\\")));
    assert!(path_has_trailing_slash(Path::new("\\\\")));
    assert!(path_has_trailing_slash(Path::new("foo\\")));
    assert!(path_has_trailing_slash(Path::new("foo\\\\")));
    assert!(path_has_trailing_slash(Path::new("foo\\.\\")));
    assert!(path_has_trailing_slash(Path::new("foo\\.\\\\")));

    assert!(!path_has_trailing_slash(Path::new("foo")));
    assert!(!path_has_trailing_slash(Path::new("foo.")));

    assert!(!path_has_trailing_slash(Path::new("/./foo")));
    assert!(!path_has_trailing_slash(Path::new("..")));
    assert!(!path_has_trailing_slash(Path::new("/..")));

    assert!(!path_has_trailing_slash(Path::new("\\.\\foo")));
    assert!(!path_has_trailing_slash(Path::new("..")));
    assert!(!path_has_trailing_slash(Path::new("\\..")));

    assert!(!path_has_trailing_slash(Path::new("/./.")));
    assert!(!path_has_trailing_slash(Path::new("foo//.")));
    assert!(!path_has_trailing_slash(Path::new("foo/./.")));

    assert!(!path_has_trailing_slash(Path::new(".")));

    assert!(!path_has_trailing_slash(Path::new("\\.\\.")));
    assert!(!path_has_trailing_slash(Path::new("foo\\\\.")));
    assert!(!path_has_trailing_slash(Path::new("foo\\.\\.")));
}
