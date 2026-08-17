use crate::fs::FileType;
use rustix::fs::RawMode;
use std::{fs, io};

/// A type that implements `FileTypeExt` for this platform.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum ImplFileTypeExt {
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
}

impl ImplFileTypeExt {
    /// Constructs a new instance of `FileType` from the given
    /// [`std::fs::File`] and [`std::fs::FileType`].
    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn from(_file: &fs::File, metadata: &fs::Metadata) -> io::Result<FileType> {
        // On `rustix`-style platforms, the `Metadata` has everything we need.
        Ok(Self::from_just_metadata(metadata))
    }

    /// Constructs a new instance of `FileType` from the given
    /// [`std::fs::Metadata`].
    #[inline]
    pub(crate) fn from_just_metadata(metadata: &fs::Metadata) -> FileType {
        let std = metadata.file_type();
        Self::from_std(std)
    }

    /// Constructs a new instance of `Self` from the given
    /// [`std::fs::FileType`].
    #[inline]
    pub(crate) fn from_std(std: fs::FileType) -> FileType {
        use rustix::fs::FileTypeExt;
        if std.is_file() {
            FileType::file()
        } else if std.is_dir() {
            FileType::dir()
        } else if std.is_symlink() {
            FileType::ext(Self::Symlink)
        } else if std.is_block_device() {
            FileType::ext(Self::BlockDevice)
        } else if std.is_char_device() {
            FileType::ext(Self::CharDevice)
        } else {
            #[cfg(not(target_os = "wasi"))]
            if std.is_fifo() {
                return FileType::ext(Self::Fifo);
            }
            #[cfg(not(target_os = "wasi"))]
            if std.is_socket() {
                return FileType::ext(Self::Socket);
            }
            FileType::unknown()
        }
    }

    /// Constructs a new instance of `FileType` from the given
    /// [`RawMode`].
    #[inline]
    pub(crate) const fn from_raw_mode(mode: RawMode) -> FileType {
        match rustix::fs::FileType::from_raw_mode(mode) {
            rustix::fs::FileType::RegularFile => FileType::file(),
            rustix::fs::FileType::Directory => FileType::dir(),
            rustix::fs::FileType::Symlink => FileType::ext(Self::symlink()),
            #[cfg(not(target_os = "wasi"))]
            rustix::fs::FileType::Fifo => FileType::ext(Self::fifo()),
            rustix::fs::FileType::CharacterDevice => FileType::ext(Self::char_device()),
            rustix::fs::FileType::BlockDevice => FileType::ext(Self::block_device()),
            #[cfg(not(target_os = "wasi"))]
            rustix::fs::FileType::Socket => FileType::ext(Self::socket()),
            _ => FileType::unknown(),
        }
    }

    /// Creates a `FileType` for which `is_symlink()` returns `true`.
    #[inline]
    pub(crate) const fn symlink() -> Self {
        Self::Symlink
    }

    /// Creates a `FileType` for which `is_block_device()` returns `true`.
    #[inline]
    pub(crate) const fn block_device() -> Self {
        Self::BlockDevice
    }

    /// Creates a `FileType` for which `is_char_device()` returns `true`.
    #[inline]
    pub(crate) const fn char_device() -> Self {
        Self::CharDevice
    }

    /// Creates a `FileType` for which `is_fifo()` returns `true`.
    #[inline]
    pub(crate) const fn fifo() -> Self {
        Self::Fifo
    }

    /// Creates a `FileType` for which `is_socket()` returns `true`.
    #[inline]
    pub(crate) const fn socket() -> Self {
        Self::Socket
    }

    /// Tests whether this file type represents a symbolic link.
    #[inline]
    pub(crate) fn is_symlink(&self) -> bool {
        *self == Self::Symlink
    }
}
