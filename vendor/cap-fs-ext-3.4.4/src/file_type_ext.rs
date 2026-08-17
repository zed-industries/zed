#[cfg(windows)]
use cap_primitives::fs::_WindowsFileTypeExt;

/// Extension trait for `FileType`.
pub trait FileTypeExt {
    /// Returns `true` if this file type is a block device.
    ///
    /// This corresponds to
    /// [`std::os::unix::fs::FileTypeExt::is_block_device`], except that it's
    /// supported on Windows platforms as well.
    ///
    /// [`std::os::unix::fs::FileTypeExt::is_block_device`]: https://doc.rust-lang.org/std/os/unix/fs/trait.FileTypeExt.html#tymethod.is_block_device
    fn is_block_device(&self) -> bool;

    /// Returns `true` if this file type is a char device.
    ///
    /// This corresponds to
    /// [`std::os::unix::fs::FileTypeExt::is_char_device`], except that it's
    /// supported on Windows platforms as well.
    ///
    /// [`std::os::unix::fs::FileTypeExt::is_char_device`]: https://doc.rust-lang.org/std/os/unix/fs/trait.FileTypeExt.html#tymethod.is_char_device
    fn is_char_device(&self) -> bool;

    /// Returns `true` if this file type is a fifo.
    ///
    /// This corresponds to
    /// [`std::os::unix::fs::FileTypeExt::is_fifo`], except that it's supported
    /// on Windows platforms as well.
    ///
    /// [`std::os::unix::fs::FileTypeExt::is_fifo`]: https://doc.rust-lang.org/std/os/unix/fs/trait.FileTypeExt.html#tymethod.is_fifo
    fn is_fifo(&self) -> bool;

    /// Returns `true` if this file type is a socket.
    ///
    /// This corresponds to
    /// [`std::os::unix::fs::FileTypeExt::is_socket`], except that it's
    /// supported on Windows platforms as well.
    ///
    /// [`std::os::unix::fs::FileTypeExt::is_socket`]: https://doc.rust-lang.org/std/os/unix/fs/trait.FileTypeExt.html#tymethod.is_socket
    fn is_socket(&self) -> bool;
}

#[cfg(not(windows))]
impl FileTypeExt for std::fs::FileType {
    #[inline]
    fn is_block_device(&self) -> bool {
        std::os::unix::fs::FileTypeExt::is_block_device(self)
    }

    #[inline]
    fn is_char_device(&self) -> bool {
        std::os::unix::fs::FileTypeExt::is_char_device(self)
    }

    #[inline]
    fn is_fifo(&self) -> bool {
        std::os::unix::fs::FileTypeExt::is_fifo(self)
    }

    #[inline]
    fn is_socket(&self) -> bool {
        std::os::unix::fs::FileTypeExt::is_socket(self)
    }
}

#[cfg(all(not(windows), any(feature = "std", feature = "async_std")))]
impl FileTypeExt for cap_primitives::fs::FileType {
    #[inline]
    fn is_block_device(&self) -> bool {
        cap_primitives::fs::FileTypeExt::is_block_device(self)
    }

    #[inline]
    fn is_char_device(&self) -> bool {
        cap_primitives::fs::FileTypeExt::is_char_device(self)
    }

    #[inline]
    fn is_fifo(&self) -> bool {
        cap_primitives::fs::FileTypeExt::is_fifo(self)
    }

    #[inline]
    fn is_socket(&self) -> bool {
        cap_primitives::fs::FileTypeExt::is_socket(self)
    }
}

#[cfg(all(windows, any(feature = "std", feature = "async_std")))]
impl FileTypeExt for cap_primitives::fs::FileType {
    #[inline]
    fn is_block_device(&self) -> bool {
        _WindowsFileTypeExt::is_block_device(self)
    }

    #[inline]
    fn is_char_device(&self) -> bool {
        _WindowsFileTypeExt::is_char_device(self)
    }

    #[inline]
    fn is_fifo(&self) -> bool {
        _WindowsFileTypeExt::is_fifo(self)
    }

    #[inline]
    fn is_socket(&self) -> bool {
        _WindowsFileTypeExt::is_socket(self)
    }
}
