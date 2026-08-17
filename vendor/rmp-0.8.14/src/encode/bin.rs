use super::RmpWrite;
use crate::encode::{write_marker, ValueWriteError};
use crate::Marker;

/// Encodes and attempts to write the most efficient binary array length implementation to the given
/// write, returning the marker used.
///
/// This function is useful when you want to get full control for writing the data itself, for
/// example, when using non-blocking socket.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_bin_len<W: RmpWrite>(wr: &mut W, len: u32) -> Result<Marker, ValueWriteError<W::Error>> {
    let marker = if len < 256 {
        Marker::Bin8
    } else if len <= u16::MAX as u32 {
        Marker::Bin16
    } else {
        Marker::Bin32
    };
    write_marker(&mut *wr, marker)?;
    if marker == Marker::Bin8 {
        wr.write_data_u8(len as u8)?;
    } else if marker == Marker::Bin16 {
        wr.write_data_u16(len as u16)?;
    } else if marker == Marker::Bin32 {
        wr.write_data_u32(len)?;
    }
    Ok(marker)
}

/// Encodes and attempts to write the most efficient binary implementation to the given `Write`.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
// TODO: Docs, range check, example, visibility.
pub fn write_bin<W: RmpWrite>(wr: &mut W, data: &[u8]) -> Result<(), ValueWriteError<W::Error>> {
    write_bin_len(wr, data.len() as u32)?;
    wr.write_bytes(data)
        .map_err(ValueWriteError::InvalidDataWrite)
}
