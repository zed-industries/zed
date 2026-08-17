use super::{read_marker, RmpRead, ValueReadError};
use crate::Marker;

/// Attempts to read exactly 5 bytes from the given reader and to decode them as `f32` value.
///
/// The first byte should be the marker and the others should represent the data itself.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading either the marker or
/// the data.
///
/// It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the
/// expected one, indicating you with the actual type.
///
/// # Note
///
/// This function will silently retry on every EINTR received from the underlying `Read` until
/// successful read.
pub fn read_f32<R: RmpRead>(rd: &mut R) -> Result<f32, ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::F32 => Ok(rd.read_data_f32()?),
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read exactly 9 bytes from the given reader and to decode them as `f64` value.
///
/// The first byte should be the marker and the others should represent the data itself.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading either the marker or
/// the data.
///
/// It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the
/// expected one, indicating you with the actual type.
///
/// # Note
///
/// This function will silently retry on every EINTR received from the underlying `Read` until
/// successful read.
pub fn read_f64<R: RmpRead>(rd: &mut R) -> Result<f64, ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::F64 => Ok(rd.read_data_f64()?),
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}
