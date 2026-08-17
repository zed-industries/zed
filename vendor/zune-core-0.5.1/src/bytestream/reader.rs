use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Formatter;

pub(crate) mod no_std_readers;
pub(crate) mod std_readers;
pub(crate) mod zcursor_no_std;

use crate::bytestream::ZByteReaderTrait;

/// Enumeration of possible methods to seek within an I/O object.
///
/// It is analogous to the [SeekFrom](std::io::SeekFrom) in the std library but
/// it's here to allow this to work in no-std crates
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum ZSeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(u64),

    /// Sets the offset to the size of this object plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it's an error to
    /// seek before byte 0.
    End(i64),

    /// Sets the offset to the current position plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it's an error to
    /// seek before byte 0.
    Current(i64)
}

impl ZSeekFrom {
    /// Convert to [SeekFrom](std::io::SeekFrom) from the `std::io` library
    ///
    /// This is only present when std feature is present
    #[cfg(feature = "std")]
    pub(crate) fn to_std_seek(self) -> std::io::SeekFrom {
        match self {
            ZSeekFrom::Start(pos) => std::io::SeekFrom::Start(pos),
            ZSeekFrom::End(pos) => std::io::SeekFrom::End(pos),
            ZSeekFrom::Current(pos) => std::io::SeekFrom::Current(pos)
        }
    }
}

pub enum ZByteIoError {
    /// A standard library error
    /// Only available with the `std` feature
    #[cfg(feature = "std")]
    StdIoError(std::io::Error),
    /// An error converting from one type to another
    TryFromIntError(core::num::TryFromIntError),
    /// Not enough bytes to satisfy a read
    // requested, read
    NotEnoughBytes(usize, usize),
    /// The output buffer is too small to write the bytes
    NotEnoughBuffer(usize, usize),
    /// An error that may occur randomly
    Generic(&'static str),
    /// An error that occurred during a seek operation
    SeekError(&'static str),
    /// An error that occurred during a seek operation
    SeekErrorOwned(String)
}

impl core::fmt::Debug for ZByteIoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "std")]
            ZByteIoError::StdIoError(err) => {
                writeln!(f, "Underlying I/O error {}", err)
            }
            ZByteIoError::TryFromIntError(err) => {
                writeln!(f, "Cannot convert to int {}", err)
            }
            ZByteIoError::NotEnoughBytes(found, expected) => {
                writeln!(f, "Not enough bytes, expected {expected} but found {found}")
            }
            ZByteIoError::NotEnoughBuffer(expected, found) => {
                writeln!(
                    f,
                    "Not enough buffer to write {expected} bytes, buffer size is {found}"
                )
            }
            ZByteIoError::Generic(err) => {
                writeln!(f, "Generic I/O error: {err}")
            }
            ZByteIoError::SeekError(err) => {
                writeln!(f, "Seek error: {err}")
            }
            ZByteIoError::SeekErrorOwned(err) => {
                writeln!(f, "Seek error {err}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for ZByteIoError {
    fn from(value: std::io::Error) -> Self {
        ZByteIoError::StdIoError(value)
    }
}

impl From<core::num::TryFromIntError> for ZByteIoError {
    fn from(value: core::num::TryFromIntError) -> Self {
        ZByteIoError::TryFromIntError(value)
    }
}

impl From<&'static str> for ZByteIoError {
    fn from(value: &'static str) -> Self {
        ZByteIoError::Generic(value)
    }
}

/// The image reader wrapper
///
/// This wraps anything that implements [ZByteReaderTrait] and
/// extends the ability of the core trait methods by providing
/// utilities like endian aware byte functions.
///
/// This prevents each implementation from providing its own
pub struct ZReader<T> {
    inner:       T,
    temp_buffer: Vec<u8>
}

impl<T: ZByteReaderTrait> ZReader<T> {
    /// Create a new reader from a source
    /// that implements the [ZByteReaderTrait]
    pub fn new(source: T) -> ZReader<T> {
        ZReader {
            inner:       source,
            temp_buffer: vec![]
        }
    }
    /// Destroy this reader returning
    /// the underlying source of the bytes
    /// from which we were decoding
    #[inline(always)]
    pub fn consume(self) -> T {
        self.inner
    }
    /// Skip ahead ignoring `num` bytes
    ///
    /// For more advanced seek methods see [Self::seek] that allows
    /// moving around via more advanced ways
    ///
    /// # Arguments
    ///  - num: The number of bytes to skip.
    ///
    /// # Returns
    ///  - `Ok(u64)`: The new position from the start of the stream.
    ///  - `Error` If something went wrong
    #[inline(always)]
    pub fn skip(&mut self, num: usize) -> Result<u64, ZByteIoError> {
        self.inner.z_seek(ZSeekFrom::Current(num as i64))
    }
    /// Move back from current position to a previous
    /// position
    ///
    /// For more advanced seek methods see [Self::seek] that allows
    /// moving around via more advanced ways
    ///
    /// # Arguments
    /// - `num`: Positions to move before the current cursor
    ///
    /// # Returns
    ///  - `Ok(u64)`: The new position from the start of the stream.
    ///  - `Error` If something went wrong
    #[inline(always)]
    pub fn rewind(&mut self, num: usize) -> Result<u64, ZByteIoError> {
        self.inner.z_seek(ZSeekFrom::Current(-(num as i64)))
    }
    /// Move around a stream of bytes
    ///
    /// This is analogous to the [std::io::Seek] trait with the same ergonomics
    /// only implemented to allow use in a `no_std` environment
    ///
    /// # Arguments
    /// - `from`: The seek operation type.
    ///
    /// # Returns
    ///  - `Ok(u64)`: The new position from the start of the stream.
    ///  -  Error if something went wrong.
    #[inline(always)]
    pub fn seek(&mut self, from: ZSeekFrom) -> Result<u64, ZByteIoError> {
        self.inner.z_seek(from)
    }

    /// Read a single byte from the underlying stream
    ///
    /// If an error occurs, it will return `0` as default output
    /// hence it may be difficult to distinguish a `0` from the underlying source
    /// and a `0` from an error.
    /// For that there is [Self::read_u8_err]
    ///
    /// # Returns.
    /// - The next byte on the stream.
    ///  
    #[inline(always)]
    pub fn read_u8(&mut self) -> u8 {
        self.inner.read_byte_no_error()
    }

    /// Read a single byte returning an error if the read cannot be satisfied
    ///
    /// # Returns
    /// - `Ok(u8)`: The next byte
    /// - Error if the byte read could not be satisfied   
    #[inline(always)]
    pub fn read_u8_err(&mut self) -> Result<u8, ZByteIoError> {
        let mut buf = [0];
        self.inner.read_const_bytes(&mut buf)?;
        Ok(buf[0])
    }

    /// Look ahead position bytes and return a reference
    /// to num_bytes from that position, or an error if the
    /// peek would be out of bounds.
    ///
    /// This doesn't increment the position, bytes would have to be discarded
    /// at a later point.
    #[inline]
    pub fn peek_at(&mut self, position: usize, num_bytes: usize) -> Result<&[u8], ZByteIoError> {
        // short circuit for zero
        // important since implementations like File will
        // cause a syscall on skip
        if position != 0 {
            // skip position bytes from start
            self.skip(position)?;
        }
        if num_bytes > 20 * 1024 * 1024 {
            // resize of 20 MBs, skipping too much, so panic
            return Err(ZByteIoError::Generic("Too many bytes skipped"));
        }
        // resize buffer
        self.temp_buffer.resize(num_bytes, 0);
        // read bytes
        match self.inner.peek_exact_bytes(&mut self.temp_buffer[..]) {
            Ok(_) => {
                // rewind back to where we were
                if position != 0 {
                    self.rewind(position)?;
                }
                Ok(&self.temp_buffer)
            }
            Err(e) => Err(e)
        }
    }
    /// Read a fixed number of known bytes to a buffer and return the bytes or an error
    /// if it occurred.
    ///
    /// The size of the `N` value must be small enough to fit the stack space otherwise
    /// this will cause a stack overflow :)
    ///
    /// If you can ignore errors, you can use [Self::read_fixed_bytes_or_zero]
    ///
    /// # Returns
    ///  - `Ok([u8;N])`: The bytes read from the source
    ///  - An error if it occurred.
    #[inline(always)]
    pub fn read_fixed_bytes_or_error<const N: usize>(&mut self) -> Result<[u8; N], ZByteIoError> {
        let mut byte_store: [u8; N] = [0; N];
        match self.inner.read_const_bytes(&mut byte_store) {
            Ok(_) => Ok(byte_store),
            Err(e) => Err(e)
        }
    }
    /// Read a fixed bytes to an array and if that is impossible, return an array containing
    /// zeros
    ///
    /// If you want to handle errors, use [Self::read_fixed_bytes_or_error]
    #[inline(always)]
    pub fn read_fixed_bytes_or_zero<const N: usize>(&mut self) -> [u8; N] {
        let mut byte_store: [u8; N] = [0; N];
        self.inner.read_const_bytes_no_error(&mut byte_store);
        byte_store
    }

    /// Move the cursor to a fixed position in the stream
    ///
    /// This will move the cursor to exacltly `position` bytes from the start of the buffer
    ///
    /// # Arguments
    /// - `position`: The current position to move the cursor.
    #[inline]
    pub fn set_position(&mut self, position: usize) -> Result<(), ZByteIoError> {
        self.seek(ZSeekFrom::Start(position as u64))?;

        Ok(())
    }

    /// Return true if the underlying buffer can no longer produce bytes
    ///
    /// This call may be expensive depending on the underlying buffer type, e.g if
    /// it's a file, we have to ask the os whether we have more contents, or in other words make a syscall.
    ///
    /// Use that wisely
    ///
    /// # Returns
    ///  - `Ok(bool)`: True if we are in `EOF`, false if we can produce more bytes
    ///  - Error if something went wrong
    #[inline(always)]
    pub fn eof(&mut self) -> Result<bool, ZByteIoError> {
        self.inner.is_eof()
    }

    /// Return the current position of the inner reader or an error
    /// if that occurred when reading.
    ///
    /// Like [eof](Self::eof), the perf characteristics may vary depending on underlying reader
    ///
    /// # Returns
    /// - `Ok(u64)`: The current position of the inner reader
    #[inline(always)]
    pub fn position(&mut self) -> Result<u64, ZByteIoError> {
        self.inner.z_position()
    }

    /// Read a fixed number of bytes from the underlying reader returning
    /// an error if that can't be satisfied
    ///
    /// Similar to [std::io::Read::read_exact]
    ///
    /// # Returns
    ///  - `Ok(())`: If the read was successful
    ///  - An error if the read was unsuccessful including failure to fill the whole bytes
    pub fn read_exact_bytes(&mut self, buf: &mut [u8]) -> Result<(), ZByteIoError> {
        self.inner.read_exact_bytes(buf)
    }

    /// Read some bytes from the inner reader, and return number of bytes read
    ///
    /// The implementation may not read bytes enough to fill the buffer
    ///
    /// Similar to [std::io::Read::read]
    ///
    /// # Returns
    /// - `Ok(usize)`: Number of bytes actually read to the buffer
    /// - An error if something went wrong
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ZByteIoError> {
        self.inner.read_bytes(buf)
    }
}

enum Mode {
    // Big endian
    BE,
    // Little Endian
    LE
}
macro_rules! get_single_type {
    ($name:tt,$name2:tt,$name3:tt,$name4:tt,$name5:tt,$name6:tt,$int_type:tt) => {
        impl<T:ZByteReaderTrait> ZReader<T>
        {
            #[inline(always)]
            fn $name(&mut self, mode: Mode) -> $int_type
            {
                const SIZE_OF_VAL: usize = core::mem::size_of::<$int_type>();

                let mut space = [0; SIZE_OF_VAL];

                self.inner.read_const_bytes_no_error(&mut space);

                match mode {
                    Mode::BE => $int_type::from_be_bytes(space),
                    Mode::LE => $int_type::from_le_bytes(space)
                }
            }

            #[inline(always)]
            fn $name2(&mut self, mode: Mode) -> Result<$int_type, ZByteIoError>
            {
                const SIZE_OF_VAL: usize = core::mem::size_of::<$int_type>();

                let mut space = [0; SIZE_OF_VAL];

                match self.inner.read_const_bytes(&mut space)
                {
                    Ok(_) => match mode {
                        Mode::BE => Ok($int_type::from_be_bytes(space)),
                        Mode::LE => Ok($int_type::from_le_bytes(space))
                    },
                     Err(e) =>  Err(e)
                }
            }
            #[doc=concat!("Read ",stringify!($int_type)," as a big endian integer")]
            #[doc=concat!("Returning an error if the underlying buffer cannot support a ",stringify!($int_type)," read.")]
            #[inline]
            pub fn $name3(&mut self) -> Result<$int_type, ZByteIoError>
            {
                self.$name2(Mode::BE)
            }

            #[doc=concat!("Read ",stringify!($int_type)," as a little endian integer")]
            #[doc=concat!("Returning an error if the underlying buffer cannot support a ",stringify!($int_type)," read.")]
            #[inline]
            pub fn $name4(&mut self) -> Result<$int_type, ZByteIoError>
            {
                self.$name2(Mode::LE)
            }
            #[doc=concat!("Read ",stringify!($int_type)," as a big endian integer")]
            #[doc=concat!("Returning 0 if the underlying  buffer does not have enough bytes for a ",stringify!($int_type)," read.")]
            #[inline(always)]
            pub fn $name5(&mut self) -> $int_type
            {
                self.$name(Mode::BE)
            }
            #[doc=concat!("Read ",stringify!($int_type)," as a little endian integer")]
            #[doc=concat!("Returning 0 if the underlying buffer does not have enough bytes for a ",stringify!($int_type)," read.")]
            #[inline(always)]
            pub fn $name6(&mut self) -> $int_type
            {
                self.$name(Mode::LE)
            }
        }
    };
}

get_single_type!(
    get_u16_inner_or_default,
    get_u16_inner_or_die,
    get_u16_be_err,
    get_u16_le_err,
    get_u16_be,
    get_u16_le,
    u16
);
get_single_type!(
    get_u32_inner_or_default,
    get_u32_inner_or_die,
    get_u32_be_err,
    get_u32_le_err,
    get_u32_be,
    get_u32_le,
    u32
);
get_single_type!(
    get_u64_inner_or_default,
    get_u64_inner_or_die,
    get_u64_be_err,
    get_u64_le_err,
    get_u64_be,
    get_u64_le,
    u64
);

#[cfg(feature = "std")]
impl<T> std::io::Read for ZReader<T>
where
    T: ZByteReaderTrait
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        use std::io::ErrorKind;
        self.read_bytes(buf)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("{:?}", e)))
    }
}
