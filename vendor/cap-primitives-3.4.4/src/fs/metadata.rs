use crate::fs::{FileType, ImplFileTypeExt, ImplMetadataExt, Permissions};
use crate::time::SystemTime;
use std::{fs, io};

/// Metadata information about a file.
///
/// This corresponds to [`std::fs::Metadata`].
///
/// <details>
/// We need to define our own version because the libstd `Metadata` doesn't
/// have a public constructor that we can use.
/// </details>
#[derive(Debug, Clone)]
pub struct Metadata {
    pub(crate) file_type: FileType,
    pub(crate) len: u64,
    pub(crate) permissions: Permissions,
    pub(crate) modified: Option<SystemTime>,
    pub(crate) accessed: Option<SystemTime>,
    pub(crate) created: Option<SystemTime>,
    pub(crate) ext: ImplMetadataExt,
}

#[allow(clippy::len_without_is_empty)]
impl Metadata {
    /// Constructs a new instance of `Self` from the given [`std::fs::File`].
    #[inline]
    pub fn from_file(file: &fs::File) -> io::Result<Self> {
        let std = file.metadata()?;
        let ext = ImplMetadataExt::from(file, &std)?;
        let file_type = ImplFileTypeExt::from(file, &std)?;
        Ok(Self::from_parts(std, ext, file_type))
    }

    /// Constructs a new instance of `Self` from the given
    /// [`std::fs::Metadata`].
    ///
    /// As with the comments in [`std::fs::Metadata::volume_serial_number`] and
    /// nearby functions, some fields of the resulting metadata will be `None`.
    ///
    /// [`std::fs::Metadata::volume_serial_number`]: https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.volume_serial_number
    #[inline]
    pub fn from_just_metadata(std: fs::Metadata) -> Self {
        let ext = ImplMetadataExt::from_just_metadata(&std);
        let file_type = ImplFileTypeExt::from_just_metadata(&std);
        Self::from_parts(std, ext, file_type)
    }

    #[inline]
    fn from_parts(std: fs::Metadata, ext: ImplMetadataExt, file_type: FileType) -> Self {
        Self {
            file_type,
            len: std.len(),
            permissions: Permissions::from_std(std.permissions()),
            modified: std.modified().ok().map(SystemTime::from_std),
            accessed: std.accessed().ok().map(SystemTime::from_std),
            created: std.created().ok().map(SystemTime::from_std),
            ext,
        }
    }

    /// Returns the file type for this metadata.
    ///
    /// This corresponds to [`std::fs::Metadata::file_type`].
    #[inline]
    pub const fn file_type(&self) -> FileType {
        self.file_type
    }

    /// Returns `true` if this metadata is for a directory.
    ///
    /// This corresponds to [`std::fs::Metadata::is_dir`].
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }

    /// Returns `true` if this metadata is for a regular file.
    ///
    /// This corresponds to [`std::fs::Metadata::is_file`].
    #[inline]
    pub fn is_file(&self) -> bool {
        self.file_type.is_file()
    }

    /// Returns `true` if this metadata is for a symbolic link.
    ///
    /// This corresponds to [`std::fs::Metadata::is_symlink`].
    #[inline]
    pub fn is_symlink(&self) -> bool {
        self.file_type.is_symlink()
    }

    /// Returns the size of the file, in bytes, this metadata is for.
    ///
    /// This corresponds to [`std::fs::Metadata::len`].
    #[inline]
    pub const fn len(&self) -> u64 {
        self.len
    }

    /// Returns the permissions of the file this metadata is for.
    ///
    /// This corresponds to [`std::fs::Metadata::permissions`].
    #[inline]
    pub fn permissions(&self) -> Permissions {
        self.permissions.clone()
    }

    /// Returns the last modification time listed in this metadata.
    ///
    /// This corresponds to [`std::fs::Metadata::modified`].
    #[inline]
    pub fn modified(&self) -> io::Result<SystemTime> {
        #[cfg(io_error_uncategorized)]
        {
            self.modified.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Unsupported,
                    "modified time metadata not available on this platform",
                )
            })
        }
        #[cfg(not(io_error_uncategorized))]
        {
            self.modified.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "modified time metadata not available on this platform",
                )
            })
        }
    }

    /// Returns the last access time of this metadata.
    ///
    /// This corresponds to [`std::fs::Metadata::accessed`].
    #[inline]
    pub fn accessed(&self) -> io::Result<SystemTime> {
        #[cfg(io_error_uncategorized)]
        {
            self.accessed.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Unsupported,
                    "accessed time metadata not available on this platform",
                )
            })
        }
        #[cfg(not(io_error_uncategorized))]
        {
            self.accessed.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "accessed time metadata not available on this platform",
                )
            })
        }
    }

    /// Returns the creation time listed in this metadata.
    ///
    /// This corresponds to [`std::fs::Metadata::created`].
    #[inline]
    pub fn created(&self) -> io::Result<SystemTime> {
        #[cfg(io_error_uncategorized)]
        {
            self.created.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Unsupported,
                    "created time metadata not available on this platform",
                )
            })
        }
        #[cfg(not(io_error_uncategorized))]
        {
            self.created.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "created time metadata not available on this platform",
                )
            })
        }
    }

    /// Determine if `self` and `other` refer to the same inode on the same
    /// device.
    #[cfg(any(not(windows), windows_by_handle))]
    pub(crate) fn is_same_file(&self, other: &Self) -> bool {
        self.ext.is_same_file(&other.ext)
    }

    /// `MetadataExt` requires nightly to be implemented, but we sometimes
    /// just need the file attributes.
    #[cfg(windows)]
    #[inline]
    pub(crate) fn file_attributes(&self) -> u32 {
        self.ext.file_attributes()
    }
}

/// Unix-specific extensions for [`MetadataExt`].
///
/// This corresponds to [`std::os::unix::fs::MetadataExt`].
#[cfg(any(unix, target_os = "vxworks"))]
pub trait MetadataExt {
    /// Returns the ID of the device containing the file.
    fn dev(&self) -> u64;
    /// Returns the inode number.
    fn ino(&self) -> u64;
    /// Returns the rights applied to this file.
    fn mode(&self) -> u32;
    /// Returns the number of hard links pointing to this file.
    fn nlink(&self) -> u64;
    /// Returns the user ID of the owner of this file.
    fn uid(&self) -> u32;
    /// Returns the group ID of the owner of this file.
    fn gid(&self) -> u32;
    /// Returns the device ID of this file (if it is a special one).
    fn rdev(&self) -> u64;
    /// Returns the total size of this file in bytes.
    fn size(&self) -> u64;
    /// Returns the last access time of the file, in seconds since Unix Epoch.
    fn atime(&self) -> i64;
    /// Returns the last access time of the file, in nanoseconds since [`atime`].
    fn atime_nsec(&self) -> i64;
    /// Returns the last modification time of the file, in seconds since Unix Epoch.
    fn mtime(&self) -> i64;
    /// Returns the last modification time of the file, in nanoseconds since [`mtime`].
    fn mtime_nsec(&self) -> i64;
    /// Returns the last status change time of the file, in seconds since Unix Epoch.
    fn ctime(&self) -> i64;
    /// Returns the last status change time of the file, in nanoseconds since [`ctime`].
    fn ctime_nsec(&self) -> i64;
    /// Returns the block size for filesystem I/O.
    fn blksize(&self) -> u64;
    /// Returns the number of blocks allocated to the file, in 512-byte units.
    fn blocks(&self) -> u64;
    #[cfg(target_os = "vxworks")]
    fn attrib(&self) -> u8;
}

/// WASI-specific extensions for [`MetadataExt`].
///
/// This corresponds to [`std::os::wasi::fs::MetadataExt`].
#[cfg(target_os = "wasi")]
pub trait MetadataExt {
    /// Returns the ID of the device containing the file.
    fn dev(&self) -> u64;
    /// Returns the inode number.
    fn ino(&self) -> u64;
    /// Returns the number of hard links pointing to this file.
    fn nlink(&self) -> u64;
    /// Returns the total size of this file in bytes.
    fn size(&self) -> u64;
    /// Returns the last access time of the file, in seconds since Unix Epoch.
    fn atim(&self) -> u64;
    /// Returns the last modification time of the file, in seconds since Unix Epoch.
    fn mtim(&self) -> u64;
    /// Returns the last status change time of the file, in seconds since Unix Epoch.
    fn ctim(&self) -> u64;
}

/// Windows-specific extensions to [`Metadata`].
///
/// This corresponds to [`std::os::windows::fs::MetadataExt`].
#[cfg(windows)]
pub trait MetadataExt {
    /// Returns the value of the `dwFileAttributes` field of this metadata.
    fn file_attributes(&self) -> u32;
    /// Returns the value of the `ftCreationTime` field of this metadata.
    fn creation_time(&self) -> u64;
    /// Returns the value of the `ftLastAccessTime` field of this metadata.
    fn last_access_time(&self) -> u64;
    /// Returns the value of the `ftLastWriteTime` field of this metadata.
    fn last_write_time(&self) -> u64;
    /// Returns the value of the `nFileSize{High,Low}` fields of this metadata.
    fn file_size(&self) -> u64;
    /// Returns the value of the `dwVolumeSerialNumber` field of this metadata.
    #[cfg(windows_by_handle)]
    fn volume_serial_number(&self) -> Option<u32>;
    /// Returns the value of the `nNumberOfLinks` field of this metadata.
    #[cfg(windows_by_handle)]
    fn number_of_links(&self) -> Option<u32>;
    /// Returns the value of the `nFileIndex{Low,High}` fields of this metadata.
    #[cfg(windows_by_handle)]
    fn file_index(&self) -> Option<u64>;
}

#[cfg(unix)]
impl MetadataExt for Metadata {
    #[inline]
    fn dev(&self) -> u64 {
        crate::fs::MetadataExt::dev(&self.ext)
    }

    #[inline]
    fn ino(&self) -> u64 {
        crate::fs::MetadataExt::ino(&self.ext)
    }

    #[inline]
    fn mode(&self) -> u32 {
        crate::fs::MetadataExt::mode(&self.ext)
    }

    #[inline]
    fn nlink(&self) -> u64 {
        crate::fs::MetadataExt::nlink(&self.ext)
    }

    #[inline]
    fn uid(&self) -> u32 {
        crate::fs::MetadataExt::uid(&self.ext)
    }

    #[inline]
    fn gid(&self) -> u32 {
        crate::fs::MetadataExt::gid(&self.ext)
    }

    #[inline]
    fn rdev(&self) -> u64 {
        crate::fs::MetadataExt::rdev(&self.ext)
    }

    #[inline]
    fn size(&self) -> u64 {
        crate::fs::MetadataExt::size(&self.ext)
    }

    #[inline]
    fn atime(&self) -> i64 {
        crate::fs::MetadataExt::atime(&self.ext)
    }

    #[inline]
    fn atime_nsec(&self) -> i64 {
        crate::fs::MetadataExt::atime_nsec(&self.ext)
    }

    #[inline]
    fn mtime(&self) -> i64 {
        crate::fs::MetadataExt::mtime(&self.ext)
    }

    #[inline]
    fn mtime_nsec(&self) -> i64 {
        crate::fs::MetadataExt::mtime_nsec(&self.ext)
    }

    #[inline]
    fn ctime(&self) -> i64 {
        crate::fs::MetadataExt::ctime(&self.ext)
    }

    #[inline]
    fn ctime_nsec(&self) -> i64 {
        crate::fs::MetadataExt::ctime_nsec(&self.ext)
    }

    #[inline]
    fn blksize(&self) -> u64 {
        crate::fs::MetadataExt::blksize(&self.ext)
    }

    #[inline]
    fn blocks(&self) -> u64 {
        crate::fs::MetadataExt::blocks(&self.ext)
    }
}

#[cfg(target_os = "wasi")]
impl MetadataExt for Metadata {
    #[inline]
    fn dev(&self) -> u64 {
        crate::fs::MetadataExt::dev(&self.ext)
    }

    #[inline]
    fn ino(&self) -> u64 {
        crate::fs::MetadataExt::ino(&self.ext)
    }

    #[inline]
    fn nlink(&self) -> u64 {
        crate::fs::MetadataExt::nlink(&self.ext)
    }

    #[inline]
    fn size(&self) -> u64 {
        crate::fs::MetadataExt::size(&self.ext)
    }

    #[inline]
    fn atim(&self) -> u64 {
        crate::fs::MetadataExt::atim(&self.ext)
    }

    #[inline]
    fn mtim(&self) -> u64 {
        crate::fs::MetadataExt::mtim(&self.ext)
    }

    #[inline]
    fn ctim(&self) -> u64 {
        crate::fs::MetadataExt::ctim(&self.ext)
    }
}

#[cfg(target_os = "vxworks")]
impl MetadataExt for Metadata {
    #[inline]
    fn dev(&self) -> u64 {
        self.ext.dev()
    }

    #[inline]
    fn ino(&self) -> u64 {
        self.ext.ino()
    }

    #[inline]
    fn mode(&self) -> u32 {
        self.ext.mode()
    }

    #[inline]
    fn nlink(&self) -> u64 {
        self.ext.nlink()
    }

    #[inline]
    fn uid(&self) -> u32 {
        self.ext.uid()
    }

    #[inline]
    fn gid(&self) -> u32 {
        self.ext.gid()
    }

    #[inline]
    fn rdev(&self) -> u64 {
        self.ext.rdev()
    }

    #[inline]
    fn size(&self) -> u64 {
        self.ext.size()
    }

    #[inline]
    fn atime(&self) -> i64 {
        self.ext.atime()
    }

    #[inline]
    fn atime_nsec(&self) -> i64 {
        self.ext.atime_nsec()
    }

    #[inline]
    fn mtime(&self) -> i64 {
        self.ext.mtime()
    }

    #[inline]
    fn mtime_nsec(&self) -> i64 {
        self.ext.mtime_nsec()
    }

    #[inline]
    fn ctime(&self) -> i64 {
        self.ext.ctime()
    }

    #[inline]
    fn ctime_nsec(&self) -> i64 {
        self.ext.ctime_nsec()
    }

    #[inline]
    fn blksize(&self) -> u64 {
        self.ext.blksize()
    }

    #[inline]
    fn blocks(&self) -> u64 {
        self.ext.blocks()
    }
}

#[cfg(windows)]
impl MetadataExt for Metadata {
    #[inline]
    fn file_attributes(&self) -> u32 {
        self.ext.file_attributes()
    }

    #[inline]
    fn creation_time(&self) -> u64 {
        self.ext.creation_time()
    }

    #[inline]
    fn last_access_time(&self) -> u64 {
        self.ext.last_access_time()
    }

    #[inline]
    fn last_write_time(&self) -> u64 {
        self.ext.last_write_time()
    }

    #[inline]
    fn file_size(&self) -> u64 {
        self.ext.file_size()
    }

    #[inline]
    #[cfg(windows_by_handle)]
    fn volume_serial_number(&self) -> Option<u32> {
        self.ext.volume_serial_number()
    }

    #[inline]
    #[cfg(windows_by_handle)]
    fn number_of_links(&self) -> Option<u32> {
        self.ext.number_of_links()
    }

    #[inline]
    #[cfg(windows_by_handle)]
    fn file_index(&self) -> Option<u64> {
        self.ext.file_index()
    }
}

/// Extension trait to allow `volume_serial_number` etc. to be exposed by
/// the `cap-fs-ext` crate.
///
/// This is hidden from the main API since this functionality isn't present in
/// `std`. Use `cap_fs_ext::MetadataExt` instead of calling this directly.
#[cfg(windows)]
#[doc(hidden)]
pub trait _WindowsByHandle {
    fn file_attributes(&self) -> u32;
    fn volume_serial_number(&self) -> Option<u32>;
    fn number_of_links(&self) -> Option<u32>;
    fn file_index(&self) -> Option<u64>;
}
