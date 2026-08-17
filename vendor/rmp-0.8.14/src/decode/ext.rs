use super::{read_marker, RmpRead, ValueReadError};
use crate::Marker;

/// Attempts to read exactly 3 bytes from the given reader and interpret them as a fixext1 type
/// with data attached.
///
/// According to the MessagePack specification, a fixext1 stores an integer and a byte array whose
/// length is 1 byte. Its marker byte is `0xd4`.
///
/// Note, that this function copies a byte array from the reader to the output `u8` variable.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading either the marker or
/// the data.
///
/// # Note
///
/// This function will silently retry on every EINTR received from the underlying `Read` until
/// successful read.
pub fn read_fixext1<R: RmpRead>(rd: &mut R) -> Result<(i8, u8), ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::FixExt1 => {
            let ty = rd.read_data_i8()?;
            let data = rd.read_data_u8()?;
            Ok((ty, data))
        }
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read exactly 4 bytes from the given reader and interpret them as a fixext2 type
/// with data attached.
///
/// According to the MessagePack specification, a fixext2 stores an integer and a byte array whose
/// length is 2 bytes. Its marker byte is `0xd5`.
///
/// Note, that this function copies a byte array from the reader to the output buffer, which is
/// unlikely if you want zero-copy functionality.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading either the marker or
/// the data.
pub fn read_fixext2<R: RmpRead>(rd: &mut R) -> Result<(i8, [u8; 2]), ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::FixExt2 => {
            let mut buf = [0; 2];
            read_fixext_data(rd, &mut buf).map(|ty| (ty, buf))
        }
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read exactly 6 bytes from the given reader and interpret them as a fixext4 type
/// with data attached.
///
/// According to the MessagePack specification, a fixext4 stores an integer and a byte array whose
/// length is 4 bytes. Its marker byte is `0xd6`.
///
/// Note, that this function copies a byte array from the reader to the output buffer, which is
/// unlikely if you want zero-copy functionality.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading either the marker or
/// the data.
pub fn read_fixext4<R: RmpRead>(rd: &mut R) -> Result<(i8, [u8; 4]), ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::FixExt4 => {
            let mut buf = [0; 4];
            read_fixext_data(rd, &mut buf).map(|ty| (ty, buf))
        }
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read exactly 10 bytes from the given reader and interpret them as a fixext8 type
/// with data attached.
///
/// According to the MessagePack specification, a fixext8 stores an integer and a byte array whose
/// length is 8 bytes. Its marker byte is `0xd7`.
///
/// Note, that this function copies a byte array from the reader to the output buffer, which is
/// unlikely if you want zero-copy functionality.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading either the marker or
/// the data.
pub fn read_fixext8<R: RmpRead>(rd: &mut R) -> Result<(i8, [u8; 8]), ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::FixExt8 => {
            let mut buf = [0; 8];
            read_fixext_data(rd, &mut buf).map(|ty| (ty, buf))
        }
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

/// Attempts to read exactly 18 bytes from the given reader and interpret them as a fixext16 type
/// with data attached.
///
/// According to the MessagePack specification, a fixext16 stores an integer and a byte array whose
/// length is 16 bytes. Its marker byte is `0xd8`.
///
/// Note, that this function copies a byte array from the reader to the output buffer, which is
/// unlikely if you want zero-copy functionality.
///
/// # Errors
///
/// This function will return `ValueReadError` on any I/O error while reading either the marker or
/// the data.
pub fn read_fixext16<R: RmpRead>(rd: &mut R) -> Result<(i8, [u8; 16]), ValueReadError<R::Error>> {
    match read_marker(rd)? {
        Marker::FixExt16 => {
            let mut buf = [0; 16];
            read_fixext_data(rd, &mut buf).map(|ty| (ty, buf))
        }
        marker => Err(ValueReadError::TypeMismatch(marker)),
    }
}

fn read_fixext_data<R: RmpRead>(rd: &mut R, buf: &mut [u8]) -> Result<i8, ValueReadError<R::Error>> {
    let id = rd.read_data_i8()?;
    match rd.read_exact_buf(buf) {
        Ok(()) => Ok(id),
        Err(err) => Err(ValueReadError::InvalidDataRead(err)),
    }
}

/// Extension type meta information.
///
/// Extension represents a tuple of type information and a byte array where type information is an
/// integer whose meaning is defined by applications.
///
/// Applications can assign 0 to 127 to store application-specific type information.
///
/// # Note
///
/// MessagePack reserves -1 to -128 for future extension to add predefined types which will be
/// described in separated documents.
#[derive(Debug, PartialEq)]
pub struct ExtMeta {
    /// Type information.
    pub typeid: i8,
    /// Byte array size.
    pub size: u32,
}

pub fn read_ext_meta<R: RmpRead>(rd: &mut R) -> Result<ExtMeta, ValueReadError<R::Error>> {
    let size = match read_marker(rd)? {
        Marker::FixExt1 => 1,
        Marker::FixExt2 => 2,
        Marker::FixExt4 => 4,
        Marker::FixExt8 => 8,
        Marker::FixExt16 => 16,
        Marker::Ext8 => u32::from(rd.read_data_u8()?),
        Marker::Ext16 => u32::from(rd.read_data_u16()?),
        Marker::Ext32 => rd.read_data_u32()?,
        marker => return Err(ValueReadError::TypeMismatch(marker)),
    };

    let ty = rd.read_data_i8()?;
    let meta = ExtMeta { typeid: ty, size };

    Ok(meta)
}
