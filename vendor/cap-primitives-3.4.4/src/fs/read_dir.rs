use crate::fs::{DirEntry, FollowSymlinks, ReadDirInner};
use std::path::Path;
use std::{fmt, fs, io};

/// Construct a `ReadDir` to iterate over the contents of a directory,
/// ensuring that the resolution of the path never escapes the directory
/// tree rooted at `start`.
#[inline]
pub fn read_dir(start: &fs::File, path: &Path) -> io::Result<ReadDir> {
    Ok(ReadDir {
        inner: ReadDirInner::new(start, path, FollowSymlinks::Yes)?,
    })
}

/// Like `read_dir`, but fails if `path` names a symlink.
#[inline]
#[cfg(not(windows))]
pub(crate) fn read_dir_nofollow(start: &fs::File, path: &Path) -> io::Result<ReadDir> {
    Ok(ReadDir {
        inner: ReadDirInner::new(start, path, FollowSymlinks::No)?,
    })
}

/// Like `read_dir` but operates on the base directory itself, rather than
/// on a path based on it.
#[inline]
pub fn read_base_dir(start: &fs::File) -> io::Result<ReadDir> {
    Ok(ReadDir {
        inner: ReadDirInner::read_base_dir(start)?,
    })
}

/// Like `read_dir`, but doesn't perform sandboxing.
#[inline]
#[cfg(not(windows))]
pub(crate) fn read_dir_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<ReadDir> {
    Ok(ReadDir {
        inner: ReadDirInner::new_unchecked(start, path, follow)?,
    })
}

/// Iterator over the entries in a directory.
///
/// This corresponds to [`std::fs::ReadDir`].
///
/// There is no `from_std` method, as `std::fs::ReadDir` doesn't provide a way
/// to construct a `ReadDir` without opening directories by ambient paths.
pub struct ReadDir {
    pub(crate) inner: ReadDirInner,
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|inner| inner.map(|inner| DirEntry { inner }))
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
