#[cfg(feature = "std")]
use std::error;
use core::fmt::{self, Display, Formatter};
use core::str::{from_utf8, Utf8Error};

use super::{read_marker, RmpRead, RmpReadErr, ValueReadError};
use crate::Marker;

#[derive(Debug)]
#[allow(deprecated)] // Only for compatibility
pub enum DecodeStringError<'a, E: RmpReadErr = super::Error> {
    InvalidMarkerRead(E),
    InvalidDataRead(E),
    TypeMismatch(Marker),
    /// The given buffer is not large enough to accumulate the specified amount of bytes.
    BufferSizeTooSmall(u32),
    InvalidUtf8(&'a [u8], Utf8Error),
}

#[cfg(feature = "std")]
impl<'a, E: RmpReadErr> error::Error for DecodeStringError<'a, E> {
    #[cold]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DecodeStringError::InvalidMarkerRead(ref err) |
            DecodeStringError::InvalidDataRead(ref err) => Some(err),
            DecodeStringError::TypeMismatch(..) |
            DecodeStringError::BufferSizeTooSmall(..) => None,
            DecodeStringError::InvalidUtf8(_, ref err) => Some(err),
        }
    }
}

impl<'a, E: RmpReadErr> Display for DecodeStringError<'a, E> {
    #[cold]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("error while decoding string")
    }
}

impl<'a, E: RmpReadErr> From<ValueReadError<E>> for DecodeStringError<'a, E> {
    #[cold]
    fn from(err: ValueReadError<E>) -> DecodeStringError<'a, E> {
        match err {
            ValueReadError::InvalidMarkerRead(err) => DecodeStringError::InvalidMarkerRead(err),
            ValueReadError::InvalidDataRead(err) => DecodeStringError::InvalidDataRead(err),
            ValueReadError::TypeMismatch(marker) => DecodeStringError::TypeMismatch(marker),
        }
    }
}

/// Attempts to read up to 9 bytes from the given reader and to decode them as a string `u32` size
/// value.
///
/// According to the MessagePack specification, the string format family stores an byte array in 1,
/// 2, 3, or 5 bytes of extra bytes in addition to the size of the byte array.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading either the marker or
/// the data.
///
/// It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the
/// expected one, indicating you with the actual type.
#[inline]
pub fn read_str_len<R: RmpRead>(rd: &mut R) -> Result<u32, ValueReadError<R::Error>> {
    Ok(read_str_len_with_nread(rd)?.0)
}

fn read_str_len_with_nread<R>(rd: &mut R) -> Result<(u32, usize), ValueReadError<R::Error>>
    where R: RmpRead
{
    match read_marker(rd)? {
        Marker::FixStr(size) => Ok((u32::from(size), 1)),
        Marker::Str8 => Ok((u32::from(rd.read_data_u8()?), 2)),
        Marker::Str16 => Ok((u32::from(rd.read_data_u16()?), 3)),
        Marker::Str32 => Ok((rd.read_data_u32()?, 5)),
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read a string data from the given reader and copy it to the buffer provided.
///
/// On success returns a borrowed string type, allowing to view the copied bytes as properly utf-8
/// string.
/// According to the spec, the string's data must to be encoded using utf-8.
///
/// # Errors
///
/// Returns `Err` in the following cases:
///
///  - if any IO error (including unexpected EOF) occurs, while reading an `rd`, except the EINTR,
///    which is handled internally.
///  - if the `out` buffer size is not large enough to keep all the data copied.
///  - if the data is not utf-8, with a description as to why the provided data is not utf-8 and
///    with a size of bytes actually copied to be able to get them from `out`.
///
/// # Examples
/// ```
/// use rmp::decode::read_str;
///
/// let buf = [0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];
/// let mut out = [0u8; 16];
///
/// assert_eq!("le message", read_str(&mut &buf[..], &mut &mut out[..]).unwrap());
/// ```
///
/// # Unstable
///
/// This function is **unstable**, because it needs review.
// TODO: Stabilize. Mark error values for each error case (in docs).
pub fn read_str<'r, R>(rd: &mut R, buf: &'r mut [u8]) -> Result<&'r str, DecodeStringError<'r, R::Error>>
where
    R: RmpRead,
{
    let len = read_str_len(rd)?;
    let ulen = len as usize;

    if buf.len() < ulen {
        return Err(DecodeStringError::BufferSizeTooSmall(len));
    }

    read_str_data(rd, len, &mut buf[0..ulen])
}

pub fn read_str_data<'r, R>(rd: &mut R,
                            len: u32,
                            buf: &'r mut [u8])
                            -> Result<&'r str, DecodeStringError<'r, R::Error>>
    where R: RmpRead
{
    debug_assert_eq!(len as usize, buf.len());

    // Trying to copy exact `len` bytes.
    match rd.read_exact_buf(buf) {
        Ok(()) => match from_utf8(buf) {
            Ok(decoded) => Ok(decoded),
            Err(err) => Err(DecodeStringError::InvalidUtf8(buf, err)),
        },
        Err(err) => Err(DecodeStringError::InvalidDataRead(err)),
    }
}

/// Attempts to read and decode a string value from the reader, returning a borrowed slice from it.
///
// TODO: Also it's possible to implement all borrowing functions for all `BufRead` implementors.
#[deprecated(since = "0.8.6", note = "useless, use `read_str_from_slice` instead")]
pub fn read_str_ref(rd: &[u8]) -> Result<&[u8], DecodeStringError<'_, super::bytes::BytesReadError>> {
    let mut cur = super::Bytes::new(rd);
    let len = read_str_len(&mut cur)?;
    Ok(&cur.remaining_slice()[..len as usize])
}

/// Attempts to read and decode a string value from the reader, returning a borrowed slice from it.
///
/// # Examples
///
/// ```
/// use rmp::encode::write_str;
/// use rmp::decode::read_str_from_slice;
///
/// let mut buf = Vec::new();
/// write_str(&mut buf, "Unpacking").unwrap();
/// write_str(&mut buf, "multiple").unwrap();
/// write_str(&mut buf, "strings").unwrap();
///
/// let mut chunks = Vec::new();
/// let mut unparsed = &buf[..];
/// while let Ok((chunk, tail)) = read_str_from_slice(unparsed) {
///     chunks.push(chunk);
///     unparsed = tail;
/// }
///
/// assert_eq!(vec!["Unpacking", "multiple", "strings"], chunks);
/// ```
pub fn read_str_from_slice<T: ?Sized + AsRef<[u8]>>(
    buf: &T,
) -> Result<(&str, &[u8]), DecodeStringError<'_, super::bytes::BytesReadError>> {
    let buf = buf.as_ref();
    let (len, nread) = read_str_len_with_nread(&mut super::Bytes::new(buf))?;
    let ulen = len as usize;

    if buf[nread..].len() >= ulen {
        let (head, tail) = buf.split_at(nread + ulen);
        match from_utf8(&head[nread..]) {
            Ok(val) => Ok((val, tail)),
            Err(err) => Err(DecodeStringError::InvalidUtf8(buf, err)),
        }
    } else {
        Err(DecodeStringError::BufferSizeTooSmall(len))
    }
}
