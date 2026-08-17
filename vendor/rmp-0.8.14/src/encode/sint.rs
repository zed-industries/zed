use super::{write_marker, RmpWrite};
use crate::encode::{write_pfix, write_u16, write_u32, write_u64, write_u8, ValueWriteError};
use crate::Marker;

/// Encodes and attempts to write a negative small integer value as a negative fixnum into the
/// given write.
///
/// According to the MessagePack specification, a negative fixed integer value is represented using
/// a single byte in `[0xe0; 0xff]` range inclusively, prepended with a special marker mask.
///
/// The function is **strict** with the input arguments - it is the user's responsibility to check
/// if the value fits in the described range, otherwise it will panic.
///
/// If you are not sure if the value fits in the given range use `write_sint` instead, which
/// automatically selects the most compact integer representation.
///
/// # Errors
///
/// This function will return `FixedValueWriteError` on any I/O error occurred while writing the
/// positive integer marker.
///
/// # Panics
///
/// Panics if `val` does not fit in `[-32; 0)` range.
#[inline]
#[track_caller]
pub fn write_nfix<W: RmpWrite>(wr: &mut W, val: i8) -> Result<(), W::Error> {
    assert!(-32 <= val && val < 0);
    write_marker(wr, Marker::FixNeg(val)).map_err(|e| e.0)?;
    Ok(())
}

/// Encodes and attempts to write an `i8` value as a 2-byte sequence into the given write.
///
/// The first byte becomes the marker and the second one will represent the data itself.
///
/// Note, that this function will encode the given value in 2-byte sequence no matter what, even if
/// the value can be represented using single byte as a fixnum. Also note, that the first byte will
/// always be the i8 marker (`0xd0`).
///
/// If you need to fit the given buffer efficiently use `write_sint` instead, which automatically
/// selects the appropriate integer representation.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
///
/// # Examples
///
/// ```
/// let mut buf = [0x00, 0x00];
///
/// rmp::encode::write_i8(&mut &mut buf[..], 42).ok().unwrap();
/// assert_eq!([0xd0, 0x2a], buf);
///
/// // Note, that -18 can be represented simply as `[0xee]`, but the function emits 2-byte sequence.
/// rmp::encode::write_i8(&mut &mut buf[..], -18).ok().unwrap();
/// assert_eq!([0xd0, 0xee], buf);
/// ```
pub fn write_i8<W: RmpWrite>(wr: &mut W, val: i8) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::I8)?;
    wr.write_data_i8(val)?;
    Ok(())
}

/// Encodes and attempts to write an `i16` value as a 3-byte sequence into the given write.
///
/// The first byte becomes the marker and the others will represent the data itself.
///
/// Note, that this function will encode the given value in 3-byte sequence no matter what, even if
/// the value can be represented using single byte as a fixnum. Also note, that the first byte will
/// always be the i16 marker (`0xd1`).
///
/// If you need to fit the given buffer efficiently use `write_sint` instead, which automatically
/// selects the appropriate integer representation.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_i16<W: RmpWrite>(wr: &mut W, val: i16) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::I16)?;
    wr.write_data_i16(val)?;
    Ok(())
}

/// Encodes and attempts to write an `i32` value as a 5-byte sequence into the given write.
///
/// The first byte becomes the marker and the others will represent the data itself.
///
/// Note, that this function will encode the given value in 5-byte sequence no matter what, even if
/// the value can be represented using single byte as a fixnum. Also note, that the first byte will
/// always be the i32 marker (`0xd2`).
///
/// If you need to fit the given buffer efficiently use `write_sint` instead, which automatically
/// selects the appropriate integer representation.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_i32<W: RmpWrite>(wr: &mut W, val: i32) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::I32)?;
    wr.write_data_i32(val)?;
    Ok(())
}

/// Encodes and attempts to write an `i64` value as a 9-byte sequence into the given write.
///
/// The first byte becomes the marker and the others will represent the data itself.
///
/// Note, that this function will encode the given value in 9-byte sequence no matter what, even if
/// the value can be represented using single byte as a fixnum. Also note, that the first byte will
/// always be the i16 marker (`0xd3`).
///
/// If you need to fit the given buffer efficiently use `write_sint` instead, which automatically
/// selects the appropriate integer representation.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_i64<W: RmpWrite>(wr: &mut W, val: i64) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::I64)?;
    wr.write_data_i64(val)?;
    Ok(())
}

/// Encodes and attempts to write an `i64` value into the given write using the most efficient
/// representation, returning the marker used.
///
/// This function obeys the MessagePack specification, which requires that the serializer SHOULD use
/// the format which represents the data in the smallest number of bytes, with the exception of
/// sized/unsized types.
///
/// Note, that the function will **always** use signed integer representation even if the value can
/// be more efficiently represented using unsigned integer encoding.
///
/// The first byte becomes the marker and the others (if present, up to 9) will represent the data
/// itself.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_sint<W: RmpWrite>(wr: &mut W, val: i64) -> Result<Marker, ValueWriteError<W::Error>> {
    match val {
        val if -32 <= val && val < 0 => {
            write_nfix(wr, val as i8)
                .and(Ok(Marker::FixNeg(val as i8)))
                .map_err(ValueWriteError::InvalidMarkerWrite)
        }
        val if -128 <= val && val < -32 => write_i8(wr, val as i8).and(Ok(Marker::I8)),
        val if -32768 <= val && val < -128 => write_i16(wr, val as i16).and(Ok(Marker::I16)),
        val if -2147483648 <= val && val < -32768 => write_i32(wr, val as i32).and(Ok(Marker::I32)),
        val if val < -2147483648 => write_i64(wr, val).and(Ok(Marker::I64)),
        val if 0 <= val && val < 128 => {
            write_pfix(wr, val as u8)
                .and(Ok(Marker::FixPos(val as u8)))
                .map_err(ValueWriteError::InvalidMarkerWrite)
        }
        val if val < 256 => write_u8(wr, val as u8).and(Ok(Marker::U8)),
        val if val < 65536 => write_u16(wr, val as u16).and(Ok(Marker::U16)),
        val if val < 4294967296 => write_u32(wr, val as u32).and(Ok(Marker::U32)),
        val => write_u64(wr, val as u64).and(Ok(Marker::U64)),
    }
}
