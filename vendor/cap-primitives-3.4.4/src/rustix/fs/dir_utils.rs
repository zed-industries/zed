use crate::fs::OpenOptions;
use ambient_authority::AmbientAuthority;
use rustix::fs::OFlags;
use std::ffi::{OsStr, OsString};
use std::ops::Deref;
#[cfg(unix)]
use std::os::unix::{
    ffi::{OsStrExt, OsStringExt},
    fs::OpenOptionsExt,
};
#[cfg(target_os = "wasi")]
use std::os::wasi::{
    ffi::{OsStrExt, OsStringExt},
    fs::OpenOptionsExt,
};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Rust's `Path` implicitly strips redundant slashes, however they aren't
/// redundant in one case: at the end of a path they indicate that a path is
/// expected to name a directory.
pub(crate) fn path_requires_dir(path: &Path) -> bool {
    let bytes = path.as_os_str().as_bytes();

    // If a path ends with '/' or '.', it's a directory. These aren't the only
    // cases, but they are the only cases that Rust's `Path` implicitly
    // normalizes away.
    bytes.ends_with(b"/") || bytes.ends_with(b"/.")
}

/// Rust's `Path` implicitly strips trailing `.` components, however they
/// aren't redundant in one case: at the end of a path they are the final path
/// component, which has different path lookup behavior.
pub(crate) fn path_has_trailing_dot(path: &Path) -> bool {
    let mut bytes = path.as_os_str().as_bytes();

    // If a path ends with '.' followed by any number of '/'s, it's a trailing dot.
    while let Some((last, rest)) = bytes.split_last() {
        if *last == b'/' {
            bytes = rest;
        } else {
            break;
        }
    }

    bytes.ends_with(b"/.") || bytes == b"."
}

/// Rust's `Path` implicitly strips trailing `/`s, however they aren't
/// redundant in one case: at the end of a path they are the final path
/// component, which has different path lookup behavior.
pub(crate) fn path_has_trailing_slash(path: &Path) -> bool {
    let bytes = path.as_os_str().as_bytes();

    bytes.ends_with(b"/")
}

/// Append a trailing `/`. This can be used to require that the given `path`
/// names a directory.
pub(crate) fn append_dir_suffix(path: PathBuf) -> PathBuf {
    let mut bytes = path.into_os_string().into_vec();
    bytes.push(b'/');
    OsString::from_vec(bytes).into()
}

/// Strip trailing `/`s, unless this reduces `path` to `/` itself. This is
/// used by `create_dir` and others to prevent paths like `foo/` from
/// canonicalizing to `foo/.` since these syscalls treat these differently.
#[allow(clippy::indexing_slicing)]
pub(crate) fn strip_dir_suffix(path: &Path) -> impl Deref<Target = Path> + '_ {
    let mut bytes = path.as_os_str().as_bytes();
    while bytes.len() > 1 && *bytes.last().unwrap() == b'/' {
        bytes = &bytes[..bytes.len() - 1];
    }
    OsStr::from_bytes(bytes).as_ref()
}

/// Return an `OpenOptions` for opening directories.
pub(crate) fn dir_options() -> OpenOptions {
    OpenOptions::new().read(true).dir_required(true).clone()
}

/// Like `dir_options`, but additionally request the ability to read the
/// directory entries.
pub(crate) fn readdir_options() -> OpenOptions {
    OpenOptions::new()
        .read(true)
        .dir_required(true)
        .readdir_required(true)
        .clone()
}

/// Return an `OpenOptions` for canonicalizing paths.
pub(crate) fn canonicalize_options() -> OpenOptions {
    OpenOptions::new().read(true).clone()
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
    let _ = ambient_authority;

    let mut options = fs::OpenOptions::new();
    options.read(true);

    #[cfg(not(target_os = "wasi"))]
    // This is for `std::fs`, so we don't have `dir_required`, so set
    // `O_DIRECTORY` manually.
    options.custom_flags((OFlags::DIRECTORY | target_o_path()).bits() as i32);
    #[cfg(target_os = "wasi")]
    options.directory(true);

    options.open(path)
}

/// Use `O_PATH` on platforms which have it, or none otherwise.
#[inline]
pub(crate) const fn target_o_path() -> OFlags {
    #[cfg(any(
        target_os = "android",
        target_os = "emscripten",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "linux",
        target_os = "redox",
    ))]
    {
        OFlags::PATH
    }

    #[cfg(any(
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "visionos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
        target_os = "illumos",
        target_os = "solaris",
    ))]
    {
        OFlags::empty()
    }
}

#[cfg(racy_asserts)]
#[test]
fn append_dir_suffix_tests() {
    assert!(append_dir_suffix(Path::new("foo").to_path_buf())
        .display()
        .to_string()
        .ends_with('/'));
}

#[test]
fn strip_dir_suffix_tests() {
    assert_eq!(&*strip_dir_suffix(Path::new("/foo//")), Path::new("/foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("/foo/")), Path::new("/foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("foo/")), Path::new("foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("foo")), Path::new("foo"));
    assert_eq!(&*strip_dir_suffix(Path::new("/")), Path::new("/"));
    assert_eq!(&*strip_dir_suffix(Path::new("//")), Path::new("/"));
    assert_eq!(&*strip_dir_suffix(Path::new("/.")), Path::new("/."));
    assert_eq!(&*strip_dir_suffix(Path::new("//.")), Path::new("/."));
    assert_eq!(&*strip_dir_suffix(Path::new(".")), Path::new("."));
    assert_eq!(&*strip_dir_suffix(Path::new("foo/.")), Path::new("foo/."));
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
}

#[test]
fn test_path_has_trailing_dot() {
    assert!(!path_has_trailing_dot(Path::new("foo")));
    assert!(!path_has_trailing_dot(Path::new("foo.")));

    assert!(!path_has_trailing_dot(Path::new("/./foo")));
    assert!(!path_has_trailing_dot(Path::new("..")));
    assert!(!path_has_trailing_dot(Path::new("/..")));

    assert!(!path_has_trailing_dot(Path::new("/")));
    assert!(!path_has_trailing_dot(Path::new("//")));
    assert!(!path_has_trailing_dot(Path::new("foo//")));
    assert!(!path_has_trailing_dot(Path::new("foo/")));

    assert!(path_has_trailing_dot(Path::new(".")));

    assert!(path_has_trailing_dot(Path::new("/./.")));
    assert!(path_has_trailing_dot(Path::new("foo//.")));
    assert!(path_has_trailing_dot(Path::new("foo/./.")));
    assert!(path_has_trailing_dot(Path::new("foo/./")));
    assert!(path_has_trailing_dot(Path::new("foo/.//")));
}

#[test]
fn test_path_has_trailing_slash() {
    assert!(path_has_trailing_slash(Path::new("/")));
    assert!(path_has_trailing_slash(Path::new("//")));
    assert!(path_has_trailing_slash(Path::new("foo//")));
    assert!(path_has_trailing_slash(Path::new("foo/")));
    assert!(path_has_trailing_slash(Path::new("foo/./")));
    assert!(path_has_trailing_slash(Path::new("foo/.//")));

    assert!(!path_has_trailing_slash(Path::new("foo")));
    assert!(!path_has_trailing_slash(Path::new("foo.")));
    assert!(!path_has_trailing_slash(Path::new("/./foo")));
    assert!(!path_has_trailing_slash(Path::new("..")));
    assert!(!path_has_trailing_slash(Path::new("/..")));
    assert!(!path_has_trailing_slash(Path::new(".")));
    assert!(!path_has_trailing_slash(Path::new("/./.")));
    assert!(!path_has_trailing_slash(Path::new("foo//.")));
    assert!(!path_has_trailing_slash(Path::new("foo/./.")));
}
