use cap_primitives::fs::is_file_read_write;
use io_lifetimes::AsFilelike;
use std::io;

/// A trait for the `is_file_read_write` function for `File` types.
///
/// This is only implemented for `File` types; for arbitrary I/O handles, use
/// [`system_interface::io::IsReadWrite`] instead.
///
/// [`system_interface::io::IsReadWrite`]: https://docs.rs/system-interface/latest/system_interface/io/trait.ReadReady.html
pub trait IsFileReadWrite {
    /// Test whether the given file is readable and/or writable.
    fn is_file_read_write(&self) -> io::Result<(bool, bool)>;
}

impl IsFileReadWrite for std::fs::File {
    #[inline]
    fn is_file_read_write(&self) -> io::Result<(bool, bool)> {
        is_file_read_write(self)
    }
}

#[cfg(feature = "std")]
impl IsFileReadWrite for cap_std::fs::File {
    #[inline]
    fn is_file_read_write(&self) -> io::Result<(bool, bool)> {
        is_file_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(all(feature = "std", feature = "fs_utf8"))]
impl IsFileReadWrite for cap_std::fs_utf8::File {
    #[inline]
    fn is_file_read_write(&self) -> io::Result<(bool, bool)> {
        is_file_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(feature = "async_std")]
impl IsFileReadWrite for async_std::fs::File {
    #[inline]
    fn is_file_read_write(&self) -> io::Result<(bool, bool)> {
        is_file_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(feature = "async_std")]
impl IsFileReadWrite for cap_async_std::fs::File {
    #[inline]
    fn is_file_read_write(&self) -> io::Result<(bool, bool)> {
        is_file_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(all(feature = "async_std", feature = "fs_utf8"))]
impl IsFileReadWrite for cap_async_std::fs_utf8::File {
    #[inline]
    fn is_file_read_write(&self) -> io::Result<(bool, bool)> {
        is_file_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}
