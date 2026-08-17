use crate::fs::{
    dir_options, DirEntryInner, FileType, FollowSymlinks, Metadata, OpenOptions, ReadDir,
};
#[cfg(not(windows))]
use rustix::fs::DirEntryExt;
use std::ffi::OsString;
use std::{fmt, fs, io};

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
    pub(crate) inner: DirEntryInner,
}

impl DirEntry {
    /// Open the file for reading.
    #[inline]
    pub fn open(&self) -> io::Result<fs::File> {
        self.open_with(OpenOptions::new().read(true))
    }

    /// Open the file with the given options.
    #[inline]
    pub fn open_with(&self, options: &OpenOptions) -> io::Result<fs::File> {
        self.inner.open(options)
    }

    /// Open the entry as a directory.
    #[inline]
    pub fn open_dir(&self) -> io::Result<fs::File> {
        self.open_with(&dir_options())
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

    /// Returns an iterator over the entries within the subdirectory.
    #[inline]
    pub fn read_dir(&self) -> io::Result<ReadDir> {
        self.inner.read_dir(FollowSymlinks::Yes)
    }

    /// Returns the metadata for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::metadata`].
    ///
    /// # Platform-specific behavior
    ///
    /// On Windows, this produces a `Metadata` object which does not contain
    /// the optional values returned by [`MetadataExt`]. Use
    /// [`cap_fs_ext::DirEntryExt::full_metadata`] to obtain a `Metadata` with
    /// the values filled in.
    ///
    /// [`MetadataExt`]: https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html
    /// [`cap_fs_ext::DirEntryExt::full_metadata`]: https://docs.rs/cap-fs-ext/latest/cap_fs_ext/trait.DirEntryExt.html#tymethod.full_metadata
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.inner.metadata()
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This corresponds to [`std::fs::DirEntry::file_type`].
    ///
    /// # Platform-specific behavior
    ///
    /// On Windows and most Unix platforms this function is free (no extra system calls needed), but
    /// some Unix platforms may require the equivalent call to `metadata` to learn about the target
    /// file type.
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

    #[cfg(any(not(windows), windows_by_handle))]
    #[cfg_attr(windows, allow(dead_code))]
    #[inline]
    pub(crate) fn is_same_file(&self, metadata: &Metadata) -> io::Result<bool> {
        self.inner.is_same_file(metadata)
    }
}

#[cfg(not(windows))]
impl DirEntryExt for DirEntry {
    #[inline]
    fn ino(&self) -> u64 {
        self.inner.ino()
    }
}

impl fmt::Debug for DirEntry {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// Extension trait to allow `full_metadata` etc. to be exposed by
/// the `cap-fs-ext` crate.
///
/// This is hidden from the main API since this functionality isn't present in
/// `std`. Use `cap_fs_ext::DirEntryExt` instead of calling this directly.
#[cfg(windows)]
#[doc(hidden)]
pub trait _WindowsDirEntryExt {
    fn full_metadata(&self) -> io::Result<Metadata>;
}
