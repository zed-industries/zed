use super::{write_marker, RmpWrite};
use crate::encode::ValueWriteError;
use crate::Marker;

/// Encodes and attempts to write an `f32` value as a 5-byte sequence into the given write.
///
/// The first byte becomes the `f32` marker and the others will represent the data itself.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_f32<W: RmpWrite>(wr: &mut W, val: f32) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::F32)?;
    wr.write_data_f32(val)?;
    Ok(())
}

/// Encodes and attempts to write an `f64` value as a 9-byte sequence into the given write.
///
/// The first byte becomes the `f64` marker and the others will represent the data itself.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_f64<W: RmpWrite>(wr: &mut W, val: f64) -> Result<(), ValueWriteError<W::Error>> {
    write_marker(wr, Marker::F64)?;
    wr.write_data_f64(val)?;
    Ok(())
}
