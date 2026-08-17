use crate::util::*;
use crate::{ImageResult, ImageSize};

use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x12))?;

    Ok(ImageSize {
        width: read_u32(reader, &Endian::Little)? as usize,
        height: read_u32(reader, &Endian::Little)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"\x42\x4D")
}
