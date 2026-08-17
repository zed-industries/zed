#[cfg(windows)]
use cap_primitives::fs::_WindowsByHandle;

/// Extension trait for `Metadata`.
pub trait MetadataExt {
    /// Returns the ID of the device containing the file.
    ///
    /// This corresponds to [`std::os::unix::fs::MetadataExt::dev`], except
    /// that it's supported on Windows platforms as well.
    ///
    /// [`std::os::unix::fs::MetadataExt::dev`]: https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.dev
    fn dev(&self) -> u64;

    /// Returns the inode number.
    ///
    /// This corresponds to [`std::os::unix::fs::MetadataExt::ino`], except
    /// that it's supported on Windows platforms as well.
    ///
    /// FIXME: On Windows' `ReFS`, file identifiers are 128-bit.
    ///
    /// [`std::os::unix::fs::MetadataExt::ino`]: https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.ino
    fn ino(&self) -> u64;

    /// Returns the number of hard links pointing to this file.
    ///
    /// This corresponds to [`std::os::unix::fs::MetadataExt::nlink`], except
    /// that it's supported on Windows platforms as well.
    ///
    /// [`std::os::unix::fs::MetadataExt::nlink`]: https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html#tymethod.nlink
    fn nlink(&self) -> u64;
}

#[cfg(not(windows))]
impl MetadataExt for std::fs::Metadata {
    #[inline]
    fn dev(&self) -> u64 {
        std::os::unix::fs::MetadataExt::dev(self)
    }

    #[inline]
    fn ino(&self) -> u64 {
        std::os::unix::fs::MetadataExt::ino(self)
    }

    #[inline]
    fn nlink(&self) -> u64 {
        std::os::unix::fs::MetadataExt::nlink(self)
    }
}

#[cfg(all(windows, windows_by_handle))]
impl MetadataExt for std::fs::Metadata {
    #[inline]
    fn dev(&self) -> u64 {
        std::os::windows::fs::MetadataExt::volume_serial_number(self)
            .expect("`dev` depends on a Metadata constructed from an open `File`")
            .into()
    }

    #[inline]
    fn ino(&self) -> u64 {
        std::os::windows::fs::MetadataExt::file_index(self)
            .expect("`ino` depends on a Metadata constructed from an open `File`")
    }

    #[inline]
    fn nlink(&self) -> u64 {
        std::os::windows::fs::MetadataExt::number_of_links(self)
            .expect("`nlink` depends on a Metadata constructed from an open `File`")
            .into()
    }
}

#[cfg(all(not(windows), any(feature = "std", feature = "async_std")))]
impl MetadataExt for cap_primitives::fs::Metadata {
    #[inline]
    fn dev(&self) -> u64 {
        cap_primitives::fs::MetadataExt::dev(self)
    }

    #[inline]
    fn ino(&self) -> u64 {
        cap_primitives::fs::MetadataExt::ino(self)
    }

    #[inline]
    fn nlink(&self) -> u64 {
        cap_primitives::fs::MetadataExt::nlink(self)
    }
}

#[cfg(all(windows, any(feature = "std", feature = "async_std")))]
impl MetadataExt for cap_primitives::fs::Metadata {
    fn dev(&self) -> u64 {
        _WindowsByHandle::volume_serial_number(self)
            .expect("`dev` depends on a Metadata constructed from an open `File`")
            .into()
    }

    #[inline]
    fn ino(&self) -> u64 {
        _WindowsByHandle::file_index(self)
            .expect("`ino` depends on a Metadata constructed from an open `File`")
    }

    #[inline]
    fn nlink(&self) -> u64 {
        _WindowsByHandle::number_of_links(self)
            .expect("`nlink` depends on a Metadata constructed from an open `File`")
            .into()
    }
}
