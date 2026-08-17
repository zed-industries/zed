use crate::fs::{Metadata, OpenOptions, Permissions};
use crate::fs_utf8::from_utf8;
use camino::Utf8Path;
use cap_primitives::AmbientAuthority;
#[cfg(not(windows))]
use io_extras::os::rustix::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsHandle, BorrowedHandle, OwnedHandle};
use std::io::{self, IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write};
#[cfg(target_os = "wasi")]
use std::path::Path;
use std::{fmt, fs, process};
#[cfg(windows)]
use {
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, IntoRawHandleOrSocket,
        OwnedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle},
};

/// A reference to an open file on a filesystem.
///
/// This corresponds to [`std::fs::File`].
///
/// This `File` has no `open` or `create` methods. To open or create a file,
/// first obtain a [`Dir`] containing the path, and then call [`Dir::open`] or
/// [`Dir::create`].
///
/// [`Dir`]: crate::fs::Dir
/// [`Dir::open`]: crate::fs::Dir::open
/// [`Dir::create`]: crate::fs::Dir::create
pub struct File {
    cap_std: crate::fs::File,
}

impl File {
    /// Constructs a new instance of `Self` from the given [`std::fs::File`].
    ///
    /// This grants access the resources the `std::fs::File` instance
    /// already has access to.
    #[inline]
    pub fn from_std(std: fs::File) -> Self {
        Self::from_cap_std(crate::fs::File::from_std(std))
    }

    /// Constructs a new instance of `Self` from the given `cap_std::fs::File`.
    #[inline]
    pub fn from_cap_std(cap_std: crate::fs::File) -> Self {
        Self { cap_std }
    }

    /// Consumes `self` and returns a `std::fs::File`.
    #[inline]
    pub fn into_std(self) -> fs::File {
        self.cap_std.into_std()
    }

    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// This corresponds to [`std::fs::File::sync_all`].
    #[inline]
    pub fn sync_all(&self) -> io::Result<()> {
        self.cap_std.sync_all()
    }

    /// This function is similar to `sync_all`, except that it may not
    /// synchronize file metadata to a filesystem.
    ///
    /// This corresponds to [`std::fs::File::sync_data`].
    #[inline]
    pub fn sync_data(&self) -> io::Result<()> {
        self.cap_std.sync_data()
    }

    /// Truncates or extends the underlying file, updating the size of this
    /// file to become size.
    ///
    /// This corresponds to [`std::fs::File::set_len`].
    #[inline]
    pub fn set_len(&self, size: u64) -> io::Result<()> {
        self.cap_std.set_len(size)
    }

    /// Queries metadata about the underlying file.
    ///
    /// This corresponds to [`std::fs::File::metadata`].
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.cap_std.metadata()
    }

    /// Creates a new `File` instance that shares the same underlying file
    /// handle as the existing `File` instance.
    ///
    /// This corresponds to [`std::fs::File::try_clone`].
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self::from_cap_std(self.cap_std.try_clone()?))
    }

    /// Changes the permissions on the underlying file.
    ///
    /// This corresponds to [`std::fs::File::set_permissions`].
    #[inline]
    pub fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        self.cap_std.set_permissions(perm)
    }

    /// Constructs a new instance of `Self` in read-only mode by opening the
    /// given path as a file using the host process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub fn open_ambient<P: AsRef<Utf8Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = from_utf8(path.as_ref())?;
        Ok(Self::from_cap_std(crate::fs::File::open_ambient(
            path,
            ambient_authority,
        )?))
    }

    /// Constructs a new instance of `Self` in write-only mode by opening,
    /// creating or truncating, the given path as a file using the host
    /// process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub fn create_ambient<P: AsRef<Utf8Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = from_utf8(path.as_ref())?;
        Ok(Self::from_cap_std(crate::fs::File::create_ambient(
            path,
            ambient_authority,
        )?))
    }

    /// Constructs a new instance of `Self` with the options specified by
    /// `options` by opening the given path as a file using the host process'
    /// ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub fn open_ambient_with<P: AsRef<Utf8Path>>(
        path: P,
        options: &OpenOptions,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let path = from_utf8(path.as_ref())?;
        Ok(Self::from_cap_std(crate::fs::File::open_ambient_with(
            path,
            options,
            ambient_authority,
        )?))
    }

    /// Returns a new `OpenOptions` object.
    ///
    /// This corresponds to [`std::fs::File::options`].
    #[must_use]
    #[inline]
    pub fn options() -> OpenOptions {
        OpenOptions::new()
    }
}

// Safety: `FilelikeViewType` is implemented for `std::fs::File`.
unsafe impl io_lifetimes::views::FilelikeViewType for File {}

#[cfg(not(windows))]
impl FromRawFd for File {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(fs::File::from_raw_fd(fd))
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for File {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(fs::File::from(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for File {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std(fs::File::from_raw_handle(handle))
    }
}

#[cfg(windows)]
impl From<OwnedHandle> for File {
    #[inline]
    fn from(handle: OwnedHandle) -> Self {
        Self::from_std(fs::File::from(handle))
    }
}

#[cfg(not(windows))]
impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.cap_std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for File {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.cap_std.as_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.cap_std.as_raw_handle()
    }
}

#[cfg(windows)]
impl AsHandle for File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.cap_std.as_handle()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for File {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.cap_std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for File {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.cap_std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for File {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.cap_std.into_raw_fd()
    }
}

#[cfg(not(windows))]
impl From<File> for OwnedFd {
    #[inline]
    fn from(file: File) -> OwnedFd {
        file.cap_std.into()
    }
}

#[cfg(windows)]
impl IntoRawHandle for File {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.cap_std.into_raw_handle()
    }
}

#[cfg(windows)]
impl From<File> for OwnedHandle {
    #[inline]
    fn from(file: File) -> OwnedHandle {
        file.cap_std.into()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for File {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.cap_std.into_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl From<File> for OwnedHandleOrSocket {
    #[inline]
    fn from(file: File) -> Self {
        file.cap_std.into()
    }
}

impl Read for File {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cap_std.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        self.cap_std.read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.cap_std.read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.cap_std.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.cap_std.read_to_string(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.cap_std.is_read_vectored()
    }
}

impl Read for &File {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&mut &self.cap_std).read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        (&mut &self.cap_std).read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (&mut &self.cap_std).read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        (&mut &self.cap_std).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        (&mut &self.cap_std).read_to_string(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.cap_std.is_read_vectored()
    }
}

impl Write for File {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.cap_std.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.cap_std.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        self.cap_std.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.cap_std.write_all(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.cap_std.is_write_vectored()
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice]) -> io::Result<()> {
        self.cap_std.write_all_vectored(bufs)
    }
}

impl Write for &File {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&mut &self.cap_std).write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        (&mut &self.cap_std).flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        (&mut &self.cap_std).write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&mut &self.cap_std).write_all(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.cap_std.is_write_vectored()
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice]) -> io::Result<()> {
        (&mut &self.cap_std).write_all_vectored(bufs)
    }
}

impl Seek for File {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cap_std.seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        self.cap_std.stream_position()
    }
}

impl Seek for &File {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        (&mut &self.cap_std).seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        (&mut &self.cap_std).stream_position()
    }
}

impl From<File> for process::Stdio {
    #[inline]
    fn from(file: File) -> Self {
        From::<crate::fs::File>::from(file.cap_std)
    }
}

#[cfg(unix)]
impl crate::fs::FileExt for File {
    #[inline]
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.cap_std.read_at(buf, offset)
    }

    #[inline]
    fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.cap_std.write_at(buf, offset)
    }

    #[inline]
    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()> {
        self.cap_std.read_exact_at(buf, offset)
    }

    #[inline]
    fn write_all_at(&self, buf: &[u8], offset: u64) -> io::Result<()> {
        self.cap_std.write_all_at(buf, offset)
    }
}

#[cfg(target_os = "wasi")]
impl crate::fs::FileExt for File {
    #[inline]
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.cap_std.read_at(buf, offset)
    }

    #[inline]
    fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.cap_std.write_at(buf, offset)
    }

    #[inline]
    fn read_vectored_at(&self, bufs: &mut [IoSliceMut], offset: u64) -> io::Result<usize> {
        self.cap_std.read_vectored_at(bufs, offset)
    }

    #[inline]
    fn write_vectored_at(&self, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
        self.cap_std.write_vectored_at(bufs, offset)
    }

    #[inline]
    fn fdstat_set_flags(&self, flags: u16) -> std::result::Result<(), io::Error> {
        self.cap_std.fdstat_set_flags(flags)
    }

    #[inline]
    fn fdstat_set_rights(
        &self,
        rights: u64,
        inheriting: u64,
    ) -> std::result::Result<(), io::Error> {
        self.cap_std.fdstat_set_rights(rights, inheriting)
    }

    #[inline]
    fn advise(&self, offset: u64, len: u64, advice: u8) -> std::result::Result<(), io::Error> {
        self.cap_std.advise(offset, len, advice)
    }

    #[inline]
    fn allocate(&self, offset: u64, len: u64) -> std::result::Result<(), io::Error> {
        self.cap_std.allocate(offset, len)
    }

    #[inline]
    fn create_directory<P: AsRef<Path>>(&self, path: P) -> std::result::Result<(), io::Error> {
        let path = path.as_ref();
        self.cap_std.create_directory(path)
    }

    #[inline]
    fn read_link<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> std::result::Result<std::path::PathBuf, io::Error> {
        let path = path.as_ref();
        self.cap_std.read_link(path)
    }

    #[inline]
    fn metadata_at<P: AsRef<Path>>(
        &self,
        lookup_flags: u32,
        path: P,
    ) -> std::result::Result<std::fs::Metadata, io::Error> {
        let path = path.as_ref();
        self.cap_std.metadata_at(lookup_flags, path)
    }

    #[inline]
    fn remove_file<P: AsRef<Path>>(&self, path: P) -> std::result::Result<(), io::Error> {
        let path = path.as_ref();
        self.cap_std.remove_file(path)
    }

    #[inline]
    fn remove_directory<P: AsRef<Path>>(&self, path: P) -> std::result::Result<(), io::Error> {
        let path = path.as_ref();
        self.cap_std.remove_directory(path)
    }
}

#[cfg(windows)]
impl crate::fs::FileExt for File {
    #[inline]
    fn seek_read(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.cap_std.seek_read(buf, offset)
    }

    #[inline]
    fn seek_write(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.cap_std.seek_write(buf, offset)
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cap_std.fmt(f)
    }
}
