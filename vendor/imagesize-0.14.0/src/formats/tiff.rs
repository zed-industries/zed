use crate::util::*;
use crate::{ImageResult, ImageSize};

use std::io::{BufRead, Cursor, Read, Seek, SeekFrom};

#[derive(Debug, PartialEq)]
enum Type {
    Tiff,
    BigTiff,
}

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;

    let mut endian_marker = [0; 2];
    reader.read_exact(&mut endian_marker)?;

    //  Get the endianness which determines how we read the input
    let endianness = if &endian_marker[0..2] == b"II" {
        Endian::Little
    } else if &endian_marker[0..2] == b"MM" {
        Endian::Big
    } else {
        //  Shouldn't get here by normal means, but handle invalid header anyway
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid TIFF header").into(),
        );
    };
    let type_marker = read_u16(reader, &endianness)?;
    let tiff_type = if type_marker == 42 {
        Type::Tiff
    } else if type_marker == 43 {
        Type::BigTiff
    } else {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid TIFF header").into(),
        );
    };

    if tiff_type == Type::BigTiff {
        // http://bigtiff.org/ describes the BigTIFF header additions as constants 8 and 0.
        let offset_bytesize = read_u16(reader, &endianness)?;
        if offset_bytesize != 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unrecognised BigTiff offset size",
            )
            .into());
        }
        let extra_field = read_u16(reader, &endianness)?;
        if extra_field != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid BigTiff header",
            )
            .into());
        }
    }

    //  Read the IFD offset from the header
    let ifd_offset = if tiff_type == Type::Tiff {
        read_u32(reader, &endianness)? as u64
    } else {
        read_u64(reader, &endianness)?
    };

    //  IFD offset cannot be 0
    if ifd_offset == 0 {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid IFD offset").into(),
        );
    }

    //  Jump to the IFD offset
    reader.seek(SeekFrom::Start(ifd_offset))?;

    //  Read how many IFD records there are
    let ifd_count = if tiff_type == Type::Tiff {
        read_u16(reader, &endianness)? as u64
    } else {
        read_u64(reader, &endianness)?
    };

    let mut width = None;
    let mut height = None;

    for _ifd in 0..ifd_count {
        let tag = read_u16(reader, &endianness)?;
        let kind = read_u16(reader, &endianness)?;
        let _count = if tiff_type == Type::Tiff {
            read_u32(reader, &endianness)? as u64
        } else {
            read_u64(reader, &endianness)?
        };

        let value_bytes = match kind {
            // BYTE | ASCII | SBYTE | UNDEFINED
            1 | 2 | 6 | 7 => 1,
            // SHORT | SSHORT
            3 | 8 => 2,
            // LONG | SLONG | FLOAT | IFD
            4 | 9 | 11 | 13 => 4,
            // RATIONAL | SRATIONAL
            5 | 10 => 4 * 2,
            // DOUBLE
            12 => 8,
            // BigTiff only: LONG8 | SLONG8 | IFD8
            16..=18 => {
                if tiff_type == Type::Tiff {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid IFD type for standard TIFF",
                    )
                    .into());
                }
                8
            }
            // Anything else is invalid
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid IFD type",
                )
                .into())
            }
        };

        let mut value_buffer = [0; 8];
        let ifd_value_length = if tiff_type == Type::Tiff { 4 } else { 8 };
        let mut handle = reader.take(ifd_value_length);
        let bytes_loaded = handle.read(&mut value_buffer)?;
        if bytes_loaded != ifd_value_length as usize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid IFD value length",
            )
            .into());
        }

        let mut r = Cursor::new(&value_buffer[..]);
        let value = match value_bytes {
            2 => Some(read_u16(&mut r, &endianness)? as u32),
            4 => Some(read_u32(&mut r, &endianness)?),
            _ => None,
        };

        //  Tag 0x100 is the image width, 0x101 is image height
        if tag == 0x100 {
            width = value;
        } else if tag == 0x101 {
            height = value;
        }

        //  If we've read both values we need, return the data
        if let (Some(width), Some(height)) = (width, height) {
            return Ok(ImageSize {
                width: width as usize,
                height: height as usize,
            });
        }
    }

    //  If no width/height pair was found return invalid data
    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "No dimensions in IFD tags").into())
}

pub fn matches(header: &[u8]) -> bool {
    const TYPE_MARKERS: [u8; 2] = [b'\x2A', b'\x2B'];
    (header.starts_with(b"II") && TYPE_MARKERS.contains(&header[2]) && header[3] == 0)
        || (header.starts_with(b"MM\x00") && TYPE_MARKERS.contains(&header[3]))
}
