use crate::fs::{Dir, File, FileType, Metadata, OpenOptions};
#[cfg(not(windows))]
use rustix::fs::DirEntryExt;
use std::ffi::OsString;
use std::{fmt, io};

/// Entries returned by the `ReadDir` iterator.
///
/// This corresponds to [`std::fs::DirEntry`].
///
/// Unlike `std::fs::DirEntry`, this API has no `DirEntry::path`, because
/// absolute paths don't interoperate well with the capability model.
///
/// There is a `file_name` function, however there are also `open`,
/// `open_with`, `open_dir`, `remove_file`, and `remove_dir` functions for
/// opening or removing the entry directly, which can be more efficient and
/// convenient.
///
/// There is no `from_std` method, as `std::fs::DirEntry` doesn't provide a
/// way to construct a `DirEntry` without opening directories by ambient paths.
pub struct DirEntry {
    pub(crate) inner: cap_primitives::fs::DirEntry,
}

impl DirEntry {
    /// Open the file for reading.
    #[inline]
    pub fn open(&self) -> io::Result<File> {
        let file = self.inner.open()?;
        Ok(File::from_std(file))
    }

    /// Open the file with the given options.
    #[inline]
    pub fn open_with(&self, options: &OpenOptions) -> io::Result<File> {
        let file = self.inner.open_with(options)?;
        Ok(File::from_std(file))
    }

    /// Open the entry as a directory.
    #[inline]
    pub fn open_dir(&self) -> io::Result<Dir> {
        let dir = self.inner.open_dir()?;
        Ok(Dir::from_std_file(dir))
    }

    /// Removes the file from its filesystem.
    #[inline]
    pub fn remove_file(&self) -> io::Result<()> {
        self.inner.remove_file()
    }

    /// Removes the directory from its filesystem.
    #[inline]
    pub fn remove_dir(&self) -> io::Result<()> {
        self.inner.remove_dir()
    }

    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::metadata`].
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.inner.metadata()
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_type`].
    #[inline]
    pub fn file_type(&self) -> io::Result<FileType> {
        self.inner.file_type()
    }

    /// Returns the bare file name of this directory entry without any other
    /// leading path component.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_name`].
    #[inline]
    pub fn file_name(&self) -> OsString {
        self.inner.file_name()
    }
}

#[cfg(not(windows))]
impl DirEntryExt for DirEntry {
    #[inline]
    fn ino(&self) -> u64 {
        self.inner.ino()
    }
}

#[cfg(windows)]
#[doc(hidden)]
impl cap_primitives::fs::_WindowsDirEntryExt for DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        cap_primitives::fs::_WindowsDirEntryExt::full_metadata(&self.inner)
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
