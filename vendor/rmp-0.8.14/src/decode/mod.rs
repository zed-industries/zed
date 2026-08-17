//! Provides various functions and structs for MessagePack decoding.
//!
//! Most of the function defined in this module will silently handle interruption error (EINTR)
//! received from the given `Read` to be in consistent state with the `Write::write_all` method in
//! the standard library.
//!
//! Any other error would immediately interrupt the parsing process. If your reader can results in
//! I/O error and simultaneously be a recoverable state (for example, when reading from
//! non-blocking socket and it returns EWOULDBLOCK) be sure that you buffer the data externally
//! to avoid data loss (using `BufRead` readers with manual consuming or some other way).

mod dec;
mod ext;
mod sint;
mod str;
mod uint;

pub use self::dec::{read_f32, read_f64};
pub use self::ext::{
    read_ext_meta, read_fixext1, read_fixext16, read_fixext2, read_fixext4, read_fixext8, ExtMeta,
};
pub use self::sint::{read_i16, read_i32, read_i64, read_i8, read_nfix};
#[allow(deprecated)]
// While we re-export deprecated items, we don't want to trigger warnings while compiling this crate
pub use self::str::{read_str, read_str_from_slice, read_str_len, read_str_ref, DecodeStringError};
pub use self::uint::{read_pfix, read_u16, read_u32, read_u64, read_u8};

use core::fmt::{self, Debug, Display, Formatter};
#[cfg(feature = "std")]
use std::error;

use num_traits::cast::FromPrimitive;

use crate::Marker;

pub mod bytes;
pub use bytes::Bytes;

#[doc(inline)]
#[allow(deprecated)]
pub use crate::errors::Error;

/// The error type for I/O operations on `RmpRead` and associated traits.
///
/// For [`std::io::Read`], this is [`std::io::Error`]
pub trait RmpReadErr: Display + Debug + crate::errors::MaybeErrBound + 'static {}
#[cfg(feature = "std")]
impl RmpReadErr for std::io::Error {}
impl RmpReadErr for core::convert::Infallible {}

macro_rules! read_byteorder_utils {
    ($($name:ident => $tp:ident),* $(,)?) => {
        $(
            #[inline]
            #[doc(hidden)]
            fn $name(&mut self) -> Result<$tp, ValueReadError<Self::Error>> where Self: Sized {
                const SIZE: usize = core::mem::size_of::<$tp>();
                let mut buf: [u8; SIZE] = [0u8; SIZE];
                self.read_exact_buf(&mut buf).map_err(ValueReadError::InvalidDataRead)?;
                Ok(paste::paste! {
                    <byteorder::BigEndian as byteorder::ByteOrder>::[<read_ $tp>](&mut buf)
                })
            }
        )*
    };
}
mod sealed {
    pub trait Sealed {}
    #[cfg(feature = "std")]
    impl<T: ?Sized + std::io::Read> Sealed for T {}
    #[cfg(not(feature = "std"))]
    impl<'a> Sealed for &'a [u8] {}
    impl Sealed for super::Bytes<'_> {}
}

/// A type that `rmp` supports reading from.
///
/// The methods of this trait should be considered an implementation detail (for now).
/// It is currently sealed (can not be implemented by the user).
///
/// See also [`std::io::Read`] and [`byteorder::ReadBytesExt`]
///
/// Its primary implementations are [`std::io::Read`] and [Bytes].
pub trait RmpRead: sealed::Sealed {
    type Error: RmpReadErr;
    /// Read a single (unsigned) byte from this stream
    #[inline]
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0; 1];
        self.read_exact_buf(&mut buf)?;
        Ok(buf[0])
    }

    /// Read the exact number of bytes needed to fill the specified buffer.
    ///
    /// If there are not enough bytes, this will return an error.
    ///
    /// See also [`std::io::Read::read_exact`]
    fn read_exact_buf(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;

    // Internal helper functions to map I/O error into the `InvalidDataRead` error.

    /// Read a single (unsigned) byte from this stream.
    #[inline]
    #[doc(hidden)]
    fn read_data_u8(&mut self) -> Result<u8, ValueReadError<Self::Error>> {
        self.read_u8().map_err(ValueReadError::InvalidDataRead)
    }
    /// Read a single (signed) byte from this stream.
    #[inline]
    #[doc(hidden)]
    fn read_data_i8(&mut self) -> Result<i8, ValueReadError<Self::Error>> {
        self.read_data_u8().map(|b| b as i8)
    }

    read_byteorder_utils!(
        read_data_u16 => u16,
        read_data_u32 => u32,
        read_data_u64 => u64,
        read_data_i16 => i16,
        read_data_i32 => i32,
        read_data_i64 => i64,
        read_data_f32 => f32,
        read_data_f64 => f64
    );
}

/*
 * HACK: rmpv & rmp-erde used the internal read_data_* functions.
 *
 * Since adding no_std support moved these functions to the RmpRead trait,
 * this broke compatiblity  (despite changing no public APIs).
 *
 * In theory, we could update rmpv and rmp-serde to use the new APIS,
 * but that would be needless churn (and might surprise users who just want to update rmp proper).
 *
 * Instead, we emulate these internal APIs for now,
 * so that rmpv and rmp-serde continue to compile without issue.
 *
 *
 * TODO: Remove this hack once we release a new version of rmp proper
 */

macro_rules! wrap_data_funcs_for_compatibility {
    ($($tp:ident),* $(,)?) => {
        $(paste::paste! {
            #[cfg(feature = "std")]
            #[doc(hidden)]
            #[deprecated(note = "internal function. rmpv & rmp-serde need to switch to RmpRead")]
            pub fn [<read_data_ $tp>] <R: std::io::Read>(buf: &mut R) -> Result<$tp, ValueReadError> {
                buf.[<read_data_ $tp>]()
            }
        })*
    };
}
wrap_data_funcs_for_compatibility!(
    u8, u16, u32, u64,
    i8, i16, i32, i64,
    f32, f64
);

#[cfg(feature = "std")]
impl<T: std::io::Read> RmpRead for T {
    type Error = std::io::Error;

    #[inline]
    fn read_exact_buf(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        std::io::Read::read_exact(self, buf)
    }
}

// An error returned from the `write_marker` and `write_fixval` functions.
struct MarkerWriteError<E: RmpReadErr>(E);

impl<E: RmpReadErr> From<E> for MarkerWriteError<E> {
    #[cold]
    fn from(err: E) -> Self {
        MarkerWriteError(err)
    }
}

/// An error that can occur when attempting to read a MessagePack marker from the reader.
#[derive(Debug)]
#[allow(deprecated)] // Needed for backwards compat
pub struct MarkerReadError<E: RmpReadErr = Error>(pub E);

/// An error which can occur when attempting to read a MessagePack value from the reader.
#[derive(Debug)]
#[allow(deprecated)] // Needed for backwards compat
pub enum ValueReadError<E: RmpReadErr = Error> {
    /// Failed to read the marker.
    InvalidMarkerRead(E),
    /// Failed to read the data.
    InvalidDataRead(E),
    /// The type decoded isn't match with the expected one.
    TypeMismatch(Marker),
}

#[cfg(feature = "std")]
impl error::Error for ValueReadError {
    #[cold]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ValueReadError::InvalidMarkerRead(ref err) |
            ValueReadError::InvalidDataRead(ref err) => Some(err),
            ValueReadError::TypeMismatch(..) => None,
        }
    }
}

impl Display for ValueReadError {
    #[cold]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        // TODO: This should probably use formatting
        f.write_str(match *self {
            ValueReadError::InvalidMarkerRead(..) => "failed to read MessagePack marker",
            ValueReadError::InvalidDataRead(..) => "failed to read MessagePack data",
            ValueReadError::TypeMismatch(..) => {
                "the type decoded isn't match with the expected one"
            }
        })
    }
}

impl<E: RmpReadErr> From<MarkerReadError<E>> for ValueReadError<E> {
    #[cold]
    fn from(err: MarkerReadError<E>) -> ValueReadError<E> {
        match err {
            MarkerReadError(err) => ValueReadError::InvalidMarkerRead(err),
        }
    }
}

impl<E: RmpReadErr> From<E> for MarkerReadError<E> {
    #[cold]
    fn from(err: E) -> MarkerReadError<E> {
        MarkerReadError(err)
    }
}

/// Attempts to read a single byte from the given reader and to decode it as a MessagePack marker.
#[inline]
pub fn read_marker<R: RmpRead>(rd: &mut R) -> Result<Marker, MarkerReadError<R::Error>> {
    Ok(Marker::from_u8(rd.read_u8()?))
}

/// Attempts to read a single byte from the given reader and to decode it as a nil value.
///
/// According to the MessagePack specification, a nil value is represented as a single `0xc0` byte.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading the nil marker,
/// except the EINTR, which is handled internally.
///
/// It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the
/// expected one, indicating you with the actual type.
///
/// # Note
///
/// This function will silently retry on every EINTR received from the underlying `Read` until
/// successful read.
pub fn read_nil<R: RmpRead>(rd: &mut R) -> Result<(), ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::Null => Ok(()),
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read a single byte from the given reader and to decode it as a boolean value.
///
/// According to the MessagePack specification, an encoded boolean value is represented as a single
/// byte.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading the bool marker,
/// except the EINTR, which is handled internally.
///
/// It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the
/// expected one, indicating you with the actual type.
///
/// # Note
///
/// This function will silently retry on every EINTR received from the underlying `Read` until
/// successful read.
pub fn read_bool<R: RmpRead>(rd: &mut R) -> Result<bool, ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::True => Ok(true),
        Marker::False => Ok(false),
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// An error which can occur when attempting to read a MessagePack numeric value from the reader.
#[derive(Debug)]
#[allow(deprecated)] // Used for compatibility
pub enum NumValueReadError<E: RmpReadErr = Error> {
    /// Failed to read the marker.
    InvalidMarkerRead(E),
    /// Failed to read the data.
    InvalidDataRead(E),
    /// The type decoded isn't match with the expected one.
    TypeMismatch(Marker),
    /// Out of range integral type conversion attempted.
    OutOfRange,
}

#[cfg(feature = "std")]
impl error::Error for NumValueReadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            NumValueReadError::InvalidMarkerRead(ref err) |
            NumValueReadError::InvalidDataRead(ref err) => Some(err),
            NumValueReadError::TypeMismatch(..) |
            NumValueReadError::OutOfRange => None,
        }
    }
}

impl<E: RmpReadErr> Display for NumValueReadError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(match *self {
            NumValueReadError::InvalidMarkerRead(..) => "failed to read MessagePack marker",
            NumValueReadError::InvalidDataRead(..) => "failed to read MessagePack data",
            NumValueReadError::TypeMismatch(..) => {
                "the type decoded isn't match with the expected one"
            }
            NumValueReadError::OutOfRange => "out of range integral type conversion attempted",
        })
    }
}

impl<E: RmpReadErr> From<MarkerReadError<E>> for NumValueReadError<E> {
    #[cold]
    fn from(err: MarkerReadError<E>) -> NumValueReadError<E> {
        match err {
            MarkerReadError(err) => NumValueReadError::InvalidMarkerRead(err),
        }
    }
}

impl<E: RmpReadErr> From<ValueReadError<E>> for NumValueReadError<E> {
    #[cold]
    fn from(err: ValueReadError<E>) -> NumValueReadError<E> {
        match err {
            ValueReadError::InvalidMarkerRead(err) => NumValueReadError::InvalidMarkerRead(err),
            ValueReadError::InvalidDataRead(err) => NumValueReadError::InvalidDataRead(err),
            ValueReadError::TypeMismatch(err) => NumValueReadError::TypeMismatch(err),
        }
    }
}

/// Attempts to read up to 9 bytes from the given reader and to decode them as integral `T` value.
///
/// This function will try to read up to 9 bytes from the reader (1 for marker and up to 8 for data)
/// and interpret them as a big-endian `T`.
///
/// Unlike `read_*`, this function weakens type restrictions, allowing you to safely decode packed
/// values even if you aren't sure about the actual integral type.
///
/// # Errors
///
/// This function will return `NumValueReadError` on any I/O error while reading either the marker
/// or the data.
///
/// It also returns `NumValueReadError::OutOfRange` if the actual type is not an integer or it does
/// not fit in the given numeric range.
///
/// # Examples
///
/// ```
/// let buf = [0xcd, 0x1, 0x2c];
///
/// assert_eq!(300u16, rmp::decode::read_int(&mut &buf[..]).unwrap());
/// assert_eq!(300i16, rmp::decode::read_int(&mut &buf[..]).unwrap());
/// assert_eq!(300u32, rmp::decode::read_int(&mut &buf[..]).unwrap());
/// assert_eq!(300i32, rmp::decode::read_int(&mut &buf[..]).unwrap());
/// assert_eq!(300u64, rmp::decode::read_int(&mut &buf[..]).unwrap());
/// assert_eq!(300i64, rmp::decode::read_int(&mut &buf[..]).unwrap());
/// assert_eq!(300usize, rmp::decode::read_int(&mut &buf[..]).unwrap());
/// assert_eq!(300isize, rmp::decode::read_int(&mut &buf[..]).unwrap());
/// ```
pub fn read_int<T: FromPrimitive, R: RmpRead>(rd: &mut R) -> Result<T, NumValueReadError<R::Error>> {
    let val = match read_marker(rd)? {
        Marker::FixPos(val) => T::from_u8(val),
        Marker::FixNeg(val) => T::from_i8(val),
        Marker::U8 => T::from_u8(rd.read_data_u8()?),
        Marker::U16 => T::from_u16(rd.read_data_u16()?),
        Marker::U32 => T::from_u32(rd.read_data_u32()?),
        Marker::U64 => T::from_u64(rd.read_data_u64()?),
        Marker::I8 => T::from_i8(rd.read_data_i8()?),
        Marker::I16 => T::from_i16(rd.read_data_i16()?),
        Marker::I32 => T::from_i32(rd.read_data_i32()?),
        Marker::I64 => T::from_i64(rd.read_data_i64()?),
        marker => return Err(NumValueReadError::TypeMismatch(marker)),
    };

    val.ok_or(NumValueReadError::OutOfRange)
}

/// Attempts to read up to 5 bytes from the given reader and to decode them as a big-endian u32
/// array size.
///
/// Array format family stores a sequence of elements in 1, 3, or 5 bytes of extra bytes in addition
/// to the elements.
///
/// # Note
///
/// This function will silently retry on every EINTR received from the underlying `Read` until
/// successful read.
// TODO: Docs.
// NOTE: EINTR is managed internally.
pub fn read_array_len<R>(rd: &mut R) -> Result<u32, ValueReadError<R::Error>>
where
    R: RmpRead,
{
    match read_marker(rd)? {
        Marker::FixArray(size) => Ok(u32::from(size)),
        Marker::Array16 => Ok(u32::from(rd.read_data_u16()?)),
        Marker::Array32 => Ok(rd.read_data_u32()?),
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read up to 5 bytes from the given reader and to decode them as a big-endian u32
/// map size.
///
/// Map format family stores a sequence of elements in 1, 3, or 5 bytes of extra bytes in addition
/// to the elements.
///
/// # Note
///
/// This function will silently retry on every EINTR received from the underlying `Read` until
/// successful read.
// TODO: Docs.
pub fn read_map_len<R: RmpRead>(rd: &mut R) -> Result<u32, ValueReadError<R::Error>> {
    let marker = read_marker(rd)?;
    marker_to_len(rd, marker)
}

pub fn marker_to_len<R: RmpRead>(rd: &mut R, marker: Marker) -> Result<u32, ValueReadError<R::Error>> {
    match marker {
        Marker::FixMap(size) => Ok(u32::from(size)),
        Marker::Map16 => Ok(u32::from(rd.read_data_u16()?)),
        Marker::Map32 => Ok(rd.read_data_u32()?),
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read up to 5 bytes from the given reader and to decode them as Binary array length.
///
/// # Note
///
/// This function will silently retry on every EINTR received from the underlying `Read` until
/// successful read.
// TODO: Docs.
pub fn read_bin_len<R: RmpRead>(rd: &mut R) -> Result<u32, ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::Bin8 => Ok(u32::from(rd.read_data_u8()?)),
        Marker::Bin16 => Ok(u32::from(rd.read_data_u16()?)),
        Marker::Bin32 => Ok(rd.read_data_u32()?),
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}
