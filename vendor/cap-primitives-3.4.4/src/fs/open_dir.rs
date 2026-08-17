//! This defines `open_dir`, a wrapper around `open` which can be used to open
//! path as a directory.

#[allow(unused_imports)]
use crate::fs::open_unchecked;
use crate::fs::{dir_options, open, open_ambient_dir_impl, readdir_options, FollowSymlinks};
use ambient_authority::AmbientAuthority;
use std::path::{Component, Path};
use std::{fs, io};

/// Open a directory by performing an `openat`-like operation,
/// ensuring that the resolution of the path never escapes
/// the directory tree rooted at `start`.
#[inline]
pub fn open_dir(start: &fs::File, path: &Path) -> io::Result<fs::File> {
    open(start, path, &dir_options())
}

/// Like `open_dir`, but additionally request the ability to read the directory
/// entries.
#[cfg(not(windows))]
#[inline]
pub(crate) fn open_dir_for_reading(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<fs::File> {
    open(start, path, readdir_options().follow(follow))
}

/// Similar to `open_dir`, but fails if the path names a symlink.
#[inline]
pub fn open_dir_nofollow(start: &fs::File, path: &Path) -> io::Result<fs::File> {
    open(start, path, dir_options().follow(FollowSymlinks::No))
}

/// Open a directory by performing an unsandboxed `openat`-like operation.
#[inline]
#[allow(dead_code)]
pub(crate) fn open_dir_unchecked(start: &fs::File, path: &Path) -> io::Result<fs::File> {
    open_unchecked(start, path, &dir_options()).map_err(Into::into)
}

/// Like `open_dir_unchecked`, but additionally request the ability to read the
/// directory entries.
#[inline]
#[allow(dead_code)]
pub(crate) fn open_dir_for_reading_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<fs::File> {
    open_unchecked(start, path, readdir_options().follow(follow)).map_err(Into::into)
}

/// Open a directory named by a bare path, using the host process' ambient
/// authority.
///
/// # Ambient Authority
///
/// This function is not sandboxed and may trivially access any path that the
/// host process has access to.
#[inline]
pub fn open_ambient_dir(path: &Path, ambient_authority: AmbientAuthority) -> io::Result<fs::File> {
    open_ambient_dir_impl(path, ambient_authority)
}

/// Open the parent directory of a given open directory, using the host
/// process' ambient authority.
///
/// # Ambient Authority
///
/// This function accesses a path outside of the `start` directory subtree.
#[inline]
pub fn open_parent_dir(
    start: &fs::File,
    ambient_authority: AmbientAuthority,
) -> io::Result<fs::File> {
    let _ = ambient_authority;
    open_dir_unchecked(start, Component::ParentDir.as_ref())
}
