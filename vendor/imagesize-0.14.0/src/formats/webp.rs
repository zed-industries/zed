use crate::util::*;
use crate::{ImageResult, ImageSize};

use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;

    if buffer[3] == b' ' {
        webp_vp8_size(reader)
    } else if buffer[3] == b'L' {
        webp_vp8l_size(reader)
    } else if buffer[3] == b'X' {
        webp_vp8x_size(reader)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid VP8 Tag").into())
    }
}

pub fn matches(header: &[u8]) -> bool {
    header.len() >= 12 && &header[0..4] == b"RIFF" && &header[8..12] == b"WEBP"
}

fn webp_vp8x_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x18))?;

    Ok(ImageSize {
        width: read_u24(reader, &Endian::Little)? as usize + 1,
        height: read_u24(reader, &Endian::Little)? as usize + 1,
    })
}

fn webp_vp8l_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x15))?;

    let dims = read_u32(reader, &Endian::Little)?;

    Ok(ImageSize {
        width: (dims & 0x3FFF) as usize + 1,
        height: ((dims >> 14) & 0x3FFF) as usize + 1,
    })
}

fn webp_vp8_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x1A))?;

    Ok(ImageSize {
        width: read_u16(reader, &Endian::Little)? as usize,
        height: read_u16(reader, &Endian::Little)? as usize,
    })
}
