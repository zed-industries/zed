#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use core::fmt;

// needed to prevent defmt macros from breaking, since they emit code that does `defmt::blahblah`.
#[cfg(feature = "defmt-03")]
use defmt_03 as defmt;

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

/// Enumeration of possible methods to seek within an I/O object.
///
/// This is the `embedded-io` equivalent of [`std::io::SeekFrom`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(u64),
    /// Sets the offset to the size of this object plus the specified number of bytes.
    End(i64),
    /// Sets the offset to the current position plus the specified number of bytes.
    Current(i64),
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<SeekFrom> for std::io::SeekFrom {
    fn from(pos: SeekFrom) -> Self {
        match pos {
            SeekFrom::Start(n) => std::io::SeekFrom::Start(n),
            SeekFrom::End(n) => std::io::SeekFrom::End(n),
            SeekFrom::Current(n) => std::io::SeekFrom::Current(n),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<std::io::SeekFrom> for SeekFrom {
    fn from(pos: std::io::SeekFrom) -> SeekFrom {
        match pos {
            std::io::SeekFrom::Start(n) => SeekFrom::Start(n),
            std::io::SeekFrom::End(n) => SeekFrom::End(n),
            std::io::SeekFrom::Current(n) => SeekFrom::Current(n),
        }
    }
}

/// Possible kinds of errors.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it. In application code, use `match` for the `ErrorKind`
/// values you are expecting; use `_` to match "all other errors".
///
/// This is the `embedded-io` equivalent of [`std::io::ErrorKind`], except with the following changes:
///
/// - `WouldBlock` is removed, since `embedded-io` traits are always blocking. See the [crate-level documentation](crate) for details.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// Unspecified error kind.
    Other,

    /// An entity was not found, often a file.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// The connection was refused by the remote server.
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// A parameter was incorrect.
    InvalidInput,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: ErrorKind::InvalidInput
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,
    /// This operation is unsupported on this platform.
    ///
    /// This means that the operation can never succeed.
    Unsupported,
    /// An operation could not be completed, because it failed
    /// to allocate enough memory.
    OutOfMemory,
    /// An attempted write could not write any data.
    WriteZero,
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<ErrorKind> for std::io::ErrorKind {
    fn from(value: ErrorKind) -> Self {
        match value {
            ErrorKind::NotFound => std::io::ErrorKind::NotFound,
            ErrorKind::PermissionDenied => std::io::ErrorKind::PermissionDenied,
            ErrorKind::ConnectionRefused => std::io::ErrorKind::ConnectionRefused,
            ErrorKind::ConnectionReset => std::io::ErrorKind::ConnectionReset,
            ErrorKind::ConnectionAborted => std::io::ErrorKind::ConnectionAborted,
            ErrorKind::NotConnected => std::io::ErrorKind::NotConnected,
            ErrorKind::AddrInUse => std::io::ErrorKind::AddrInUse,
            ErrorKind::AddrNotAvailable => std::io::ErrorKind::AddrNotAvailable,
            ErrorKind::BrokenPipe => std::io::ErrorKind::BrokenPipe,
            ErrorKind::AlreadyExists => std::io::ErrorKind::AlreadyExists,
            ErrorKind::InvalidInput => std::io::ErrorKind::InvalidInput,
            ErrorKind::InvalidData => std::io::ErrorKind::InvalidData,
            ErrorKind::TimedOut => std::io::ErrorKind::TimedOut,
            ErrorKind::Interrupted => std::io::ErrorKind::Interrupted,
            ErrorKind::Unsupported => std::io::ErrorKind::Unsupported,
            ErrorKind::OutOfMemory => std::io::ErrorKind::OutOfMemory,
            _ => std::io::ErrorKind::Other,
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<std::io::ErrorKind> for ErrorKind {
    fn from(value: std::io::ErrorKind) -> Self {
        match value {
            std::io::ErrorKind::NotFound => ErrorKind::NotFound,
            std::io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            std::io::ErrorKind::ConnectionRefused => ErrorKind::ConnectionRefused,
            std::io::ErrorKind::ConnectionReset => ErrorKind::ConnectionReset,
            std::io::ErrorKind::ConnectionAborted => ErrorKind::ConnectionAborted,
            std::io::ErrorKind::NotConnected => ErrorKind::NotConnected,
            std::io::ErrorKind::AddrInUse => ErrorKind::AddrInUse,
            std::io::ErrorKind::AddrNotAvailable => ErrorKind::AddrNotAvailable,
            std::io::ErrorKind::BrokenPipe => ErrorKind::BrokenPipe,
            std::io::ErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
            std::io::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            std::io::ErrorKind::InvalidData => ErrorKind::InvalidData,
            std::io::ErrorKind::TimedOut => ErrorKind::TimedOut,
            std::io::ErrorKind::Interrupted => ErrorKind::Interrupted,
            std::io::ErrorKind::Unsupported => ErrorKind::Unsupported,
            std::io::ErrorKind::OutOfMemory => ErrorKind::OutOfMemory,
            _ => ErrorKind::Other,
        }
    }
}

/// Error trait.
///
/// This trait allows generic code to do limited inspecting of errors,
/// to react differently to different kinds.
pub trait Error: fmt::Debug {
    /// Get the kind of this error.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

impl Error for ErrorKind {
    fn kind(&self) -> ErrorKind {
        *self
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl Error for std::io::Error {
    fn kind(&self) -> ErrorKind {
        self.kind().into()
    }
}

/// Base trait for all IO traits, defining the error type.
///
/// All IO operations of all traits return the error defined in this trait.
///
/// Having a shared trait instead of having every trait define its own
/// `Error` associated type enforces all impls on the same type use the same error.
/// This is very convenient when writing generic code, it means you have to
/// handle a single error type `T::Error`, instead of `<T as Read>::Error` and `<T as Write>::Error`
/// which might be different types.
pub trait ErrorType {
    /// Error type of all the IO operations on this type.
    type Error: Error;
}

impl<T: ?Sized + ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

/// Error returned by [`Read::read_exact`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ReadExactError<E> {
    /// An EOF error was encountered before reading the exact amount of requested bytes.
    UnexpectedEof,
    /// Error returned by the inner Read.
    Other(E),
}

impl<E> From<E> for ReadExactError<E> {
    fn from(err: E) -> Self {
        Self::Other(err)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<ReadExactError<std::io::Error>> for std::io::Error {
    fn from(err: ReadExactError<std::io::Error>) -> Self {
        match err {
            ReadExactError::UnexpectedEof => std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "UnexpectedEof".to_owned(),
            ),
            ReadExactError::Other(e) => std::io::Error::new(e.kind(), format!("{e:?}")),
        }
    }
}

impl<E: fmt::Debug> fmt::Display for ReadExactError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<E: fmt::Debug> std::error::Error for ReadExactError<E> {}

/// Errors that could be returned by `Write` on `&mut [u8]`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[non_exhaustive]
pub enum SliceWriteError {
    /// The target slice was full and so could not receive any new data.
    Full,
}

/// Error returned by [`Write::write_fmt`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum WriteFmtError<E> {
    /// An error was encountered while formatting.
    FmtError,
    /// Error returned by the inner Write.
    Other(E),
}

impl<E> From<E> for WriteFmtError<E> {
    fn from(err: E) -> Self {
        Self::Other(err)
    }
}

impl<E: fmt::Debug> fmt::Display for WriteFmtError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<E: fmt::Debug> std::error::Error for WriteFmtError<E> {}

/// Blocking reader.
///
/// This trait is the `embedded-io` equivalent of [`std::io::Read`].
pub trait Read: ErrorType {
    /// Read some bytes from this source into the specified buffer, returning how many bytes were read.
    ///
    /// If no bytes are currently available to read, this function blocks until at least one byte is available.
    ///
    /// If bytes are available, a non-zero amount of bytes is read to the beginning of `buf`, and the amount
    /// is returned. It is not guaranteed that *all* available bytes are returned, it is possible for the
    /// implementation to read an amount of bytes less than `buf.len()` while there are more bytes immediately
    /// available.
    ///
    /// If the reader is at end-of-file (EOF), `Ok(0)` is returned. There is no guarantee that a reader at EOF
    /// will always be so in the future, for example a reader can stop being at EOF if another process appends
    /// more bytes to the underlying file.
    ///
    /// If `buf.len() == 0`, `read` returns without blocking, with either `Ok(0)` or an error.
    /// The `Ok(0)` doesn't indicate EOF, unlike when called with a non-empty buffer.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// This function calls `read()` in a loop until exactly `buf.len()` bytes have
    /// been read, blocking if needed.
    ///
    /// If you are using [`ReadReady`] to avoid blocking, you should not use this function.
    /// `ReadReady::read_ready()` returning true only guarantees the first call to `read()` will
    /// not block, so this function may still block in subsequent calls.
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => buf = &mut buf[n..],
                Err(e) => return Err(ReadExactError::Other(e)),
            }
        }
        if buf.is_empty() {
            Ok(())
        } else {
            Err(ReadExactError::UnexpectedEof)
        }
    }
}

/// Blocking buffered reader.
///
/// This trait is the `embedded-io` equivalent of [`std::io::BufRead`].
pub trait BufRead: ErrorType {
    /// Return the contents of the internal buffer, filling it with more data from the inner reader if it is empty.
    ///
    /// If no bytes are currently available to read, this function blocks until at least one byte is available.
    ///
    /// If the reader is at end-of-file (EOF), an empty slice is returned. There is no guarantee that a reader at EOF
    /// will always be so in the future, for example a reader can stop being at EOF if another process appends
    /// more bytes to the underlying file.
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error>;

    /// Tell this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to `fill_buf`.
    fn consume(&mut self, amt: usize);
}

/// Blocking writer.
///
/// This trait is the `embedded-io` equivalent of [`std::io::Write`].
pub trait Write: ErrorType {
    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// If the writer is not currently ready to accept more bytes (for example, its buffer is full),
    /// this function blocks until it is ready to accept least one byte.
    ///
    /// If it's ready to accept bytes, a non-zero amount of bytes is written from the beginning of `buf`, and the amount
    /// is returned. It is not guaranteed that *all* available buffer space is filled, i.e. it is possible for the
    /// implementation to write an amount of bytes less than `buf.len()` while the writer continues to be
    /// ready to accept more bytes immediately.
    ///
    /// Implementations must not return `Ok(0)` unless `buf` is empty. Situations where the
    /// writer is not able to accept more bytes must instead be indicated with an error,
    /// where the `ErrorKind` is `WriteZero`.
    ///
    /// If `buf` is empty, `write` returns without blocking, with either `Ok(0)` or an error.
    /// `Ok(0)` doesn't indicate an error.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    /// Flush this output stream, blocking until all intermediately buffered contents reach their destination.
    fn flush(&mut self) -> Result<(), Self::Error>;

    /// Write an entire buffer into this writer.
    ///
    /// This function calls `write()` in a loop until exactly `buf.len()` bytes have
    /// been written, blocking if needed.
    ///
    /// If you are using [`WriteReady`] to avoid blocking, you should not use this function.
    /// `WriteReady::write_ready()` returning true only guarantees the first call to `write()` will
    /// not block, so this function may still block in subsequent calls.
    ///
    /// This function will panic if `write()` returns `Ok(0)`.
    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Self::Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => panic!("write() returned Ok(0)"),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Write a formatted string into this writer, returning any error encountered.
    ///
    /// This function calls `write()` in a loop until the entire formatted string has
    /// been written, blocking if needed.
    ///
    /// If you are using [`WriteReady`] to avoid blocking, you should not use this function.
    /// `WriteReady::write_ready()` returning true only guarantees the first call to `write()` will
    /// not block, so this function may still block in subsequent calls.
    ///
    /// Unlike [`Write::write`], the number of bytes written is not returned. However, in the case of
    /// writing to an `&mut [u8]` its possible to calculate the number of bytes written by subtracting
    /// the length of the slice after the write, from the initial length of the slice.
    ///
    /// ```rust
    /// # use embedded_io::Write;
    /// let mut buf: &mut [u8] = &mut [0u8; 256];
    /// let start = buf.len();
    /// let len = write!(buf, "{}", "Test").and_then(|_| Ok(start - buf.len()));
    /// ```
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<(), WriteFmtError<Self::Error>> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adapter<'a, T: Write + ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<(), T::Error>,
        }

        impl<T: Write + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adapter {
            inner: self,
            error: Ok(()),
        };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => match output.error {
                // check if the error came from the underlying `Write` or not
                Err(e) => Err(WriteFmtError::Other(e)),
                Ok(()) => Err(WriteFmtError::FmtError),
            },
        }
    }
}

/// Blocking seek within streams.
///
/// This trait is the `embedded-io` equivalent of [`std::io::Seek`].
pub trait Seek: ErrorType {
    /// Seek to an offset, in bytes, in a stream.
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error>;

    /// Rewind to the beginning of a stream.
    fn rewind(&mut self) -> Result<(), Self::Error> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    /// Returns the current seek position from the start of the stream.
    fn stream_position(&mut self) -> Result<u64, Self::Error> {
        self.seek(SeekFrom::Current(0))
    }
}

/// Get whether a reader is ready.
///
/// This allows using a [`Read`] or [`BufRead`] in a nonblocking fashion, i.e. trying to read
/// only when it is ready.
pub trait ReadReady: ErrorType {
    /// Get whether the reader is ready for immediately reading.
    ///
    /// This usually means that there is either some bytes have been received and are buffered and ready to be read,
    /// or that the reader is at EOF.
    ///
    /// If this returns `true`, it's guaranteed that the next call to [`Read::read`] or [`BufRead::fill_buf`] will not block.
    fn read_ready(&mut self) -> Result<bool, Self::Error>;
}

/// Get whether a writer is ready.
///
/// This allows using a [`Write`] in a nonblocking fashion, i.e. trying to write
/// only when it is ready.
pub trait WriteReady: ErrorType {
    /// Get whether the writer is ready for immediately writing.
    ///
    /// This usually means that there is free space in the internal transmit buffer.
    ///
    /// If this returns `true`, it's guaranteed that the next call to [`Write::write`] will not block.
    fn write_ready(&mut self) -> Result<bool, Self::Error>;
}

impl<T: ?Sized + Read> Read for &mut T {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::read(self, buf)
    }
}

impl<T: ?Sized + BufRead> BufRead for &mut T {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        T::fill_buf(self)
    }

    fn consume(&mut self, amt: usize) {
        T::consume(self, amt);
    }
}

impl<T: ?Sized + Write> Write for &mut T {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(self, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        T::flush(self)
    }
}

impl<T: ?Sized + Seek> Seek for &mut T {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        T::seek(self, pos)
    }
}

impl<T: ?Sized + ReadReady> ReadReady for &mut T {
    #[inline]
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        T::read_ready(self)
    }
}

impl<T: ?Sized + WriteReady> WriteReady for &mut T {
    #[inline]
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        T::write_ready(self)
    }
}
