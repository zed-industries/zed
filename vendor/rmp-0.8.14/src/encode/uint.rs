use super::{write_marker, RmpWrite};
use crate::encode::ValueWriteError;
use crate::Marker;

/// Encodes and attempts to write an unsigned small integer value as a positive fixint into the
/// given write.
///
/// According to the MessagePack specification, a positive fixed integer value is represented using
/// a single byte in `[0x00; 0x7f]` range inclusively, prepended with a special marker mask.
///
/// The function is **strict** with the input arguments - it is the user's responsibility to check
/// if the value fits in the described range, otherwise it will panic.
///
/// If you are not sure if the value fits in the given range use `write_uint` instead, which
/// automatically selects the most compact integer representation.
///
/// # Errors
///
/// This function will return `FixedValueWriteError` on any I/O error occurred while writing the
/// positive integer marker.
///
/// # Panics
///
/// Panics if `val` is greater than 127.
#[inline]
pub fn write_pfix<W: RmpWrite>(wr: &mut W, val: u8) -> Result<(), W::Error> {
    assert!(val < 128);
    write_marker(wr, Marker::FixPos(val)).map_err(|e| e.0)?;
    Ok(())
}

/// Encodes and attempts to write an `u8` value as a 2-byte sequence into the given write.
///
/// The first byte becomes the marker and the second one will represent the data itself.
///
/// Note, that this function will encode the given value in 2-byte sequence no matter what, even if
/// the value can be represented using single byte as a positive fixnum.
///
/// If you need to fit the given buffer efficiently use `write_uint` instead, which automatically
/// selects the appropriate integer representation.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
///
/// # Examples
/// ```
/// let mut buf = [0x00, 0x00];
///
/// rmp::encode::write_u8(&mut &mut buf[..], 146).ok().unwrap();
/// assert_eq!([0xcc, 0x92], buf);
///
/// // Note, that 42 can be represented simply as `[0x2a]`, but the function emits 2-byte sequence.
/// rmp::encode::write_u8(&mut &mut buf[..], 42).ok().unwrap();
/// assert_eq!([0xcc, 0x2a], buf);
/// ```
pub fn write_u8<W: RmpWrite>(wr: &mut W, val: u8) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::U8)?;
    wr.write_data_u8(val)?;
    Ok(())
}

/// Encodes and attempts to write an `u16` value strictly as a 3-byte sequence into the given write.
///
/// The first byte becomes the marker and the others will represent the data itself.
///
/// Note, that this function will encode the given value in 3-byte sequence no matter what, even if
/// the value can be represented using single byte as a positive fixnum.
///
/// If you need to fit the given buffer efficiently use `write_uint` instead, which automatically
/// selects the appropriate integer representation.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_u16<W: RmpWrite>(wr: &mut W, val: u16) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::U16)?;
    wr.write_data_u16(val)?;
    Ok(())
}

/// Encodes and attempts to write an `u32` value strictly as a 5-byte sequence into the given write.
///
/// The first byte becomes the marker and the others will represent the data itself.
///
/// Note, that this function will encode the given value in 5-byte sequence no matter what, even if
/// the value can be represented using single byte as a positive fixnum.
///
/// If you need to fit the given buffer efficiently use `write_uint` instead, which automatically
/// selects the appropriate integer representation.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_u32<W: RmpWrite>(wr: &mut W, val: u32) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::U32)?;
    wr.write_data_u32(val)?;
    Ok(())
}

/// Encodes and attempts to write an `u64` value strictly as a 9-byte sequence into the given write.
///
/// The first byte becomes the marker and the others will represent the data itself.
///
/// Note, that this function will encode the given value in 9-byte sequence no matter what, even if
/// the value can be represented using single byte as a positive fixnum.
///
/// If you need to fit the given buffer efficiently use `write_uint` instead, which automatically
/// selects the appropriate integer representation.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_u64<W: RmpWrite>(wr: &mut W, val: u64) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::U64)?;
    wr.write_data_u64(val)?;
    Ok(())
}

/// Encodes and attempts to write an `u8` value into the given write using the most efficient
/// representation, returning the marker used.
///
/// See [`write_uint`] for more info.
pub fn write_uint8<W: RmpWrite>(wr: &mut W, val: u8) -> Result<Marker, ValueWriteError<W::Error>> {
    if val < 128 {
        write_pfix(wr, val)
            .and(Ok(Marker::FixPos(val)))
            .map_err(ValueWriteError::InvalidMarkerWrite)
    } else {
        write_u8(wr, val).and(Ok(Marker::U8))
    }
}

/// Encodes and attempts to write an `u64` value into the given write using the most efficient
/// representation, returning the marker used.
///
/// This function obeys the MessagePack specification, which requires that the serializer SHOULD use
/// the format which represents the data in the smallest number of bytes.
///
/// The first byte becomes the marker and the others (if present, up to 9) will represent the data
/// itself.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_uint<W: RmpWrite>(wr: &mut W, val: u64) -> Result<Marker, ValueWriteError<W::Error>> {
    if val < 256 {
        write_uint8(wr, val as u8)
    } else if val < 65536 {
        write_u16(wr, val as u16).and(Ok(Marker::U16))
    } else if val < 4294967296 {
        write_u32(wr, val as u32).and(Ok(Marker::U32))
    } else {
        write_u64(wr, val).and(Ok(Marker::U64))
    }
}
