use crate::fs::{FileType, Metadata, OpenOptions};
use crate::fs_utf8::{to_utf8, Dir, File};
#[cfg(not(windows))]
use rustix::fs::DirEntryExt;
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
/// There is no `from_std` method, as `std::fs::DirEntry` doesn't provide a way
/// to construct a `DirEntry` without opening directories by ambient paths.
pub struct DirEntry {
    cap_std: crate::fs::DirEntry,
}

impl DirEntry {
    /// Constructs a new instance of `Self` from the given
    /// `cap_std::fs::DirEntry`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::DirEntry) -> Self {
        Self { cap_std }
    }

    /// Open the file for reading.
    #[inline]
    pub fn open(&self) -> io::Result<File> {
        self.cap_std.open().map(File::from_cap_std)
    }

    /// Open the file with the given options.
    #[inline]
    pub fn open_with(&self, options: &OpenOptions) -> io::Result<File> {
        self.cap_std.open_with(options).map(File::from_cap_std)
    }

    /// Open the entry as a directory.
    #[inline]
    pub fn open_dir(&self) -> io::Result<Dir> {
        self.cap_std.open_dir().map(Dir::from_cap_std)
    }

    /// Removes the file from its filesystem.
    #[inline]
    pub fn remove_file(&self) -> io::Result<()> {
        self.cap_std.remove_file()
    }

    /// Removes the directory from its filesystem.
    #[inline]
    pub fn remove_dir(&self) -> io::Result<()> {
        self.cap_std.remove_dir()
    }

    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::metadata`].
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.cap_std.metadata()
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_type`].
    #[inline]
    pub fn file_type(&self) -> io::Result<FileType> {
        self.cap_std.file_type()
    }

    /// Returns the bare file name of this directory entry without any other
    /// leading path component.
    ///
    /// This function returns an `Err` in the case that the file name isn't
    /// encodable as UTF-8.
    ///
    /// If the `arf_strings` feature is enabled, unencodable names are
    /// translated to UTF-8 using `arf-strings`.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_name`].
    #[inline]
    pub fn file_name(&self) -> io::Result<String> {
        Ok(to_utf8(self.cap_std.file_name())?.into())
    }
}

#[cfg(not(windows))]
impl DirEntryExt for DirEntry {
    #[inline]
    fn ino(&self) -> u64 {
        self.cap_std.ino()
    }
}

#[cfg(windows)]
#[doc(hidden)]
impl cap_primitives::fs::_WindowsDirEntryExt for DirEntry {
    #[inline]
    fn full_metadata(&self) -> io::Result<Metadata> {
        self.cap_std.full_metadata()
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cap_std.fmt(f)
    }
}
