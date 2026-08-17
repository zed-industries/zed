use crate::util::*;
use crate::{ImageResult, ImageSize};

use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // aseprite header: https://github.com/aseprite/aseprite/blob/main/docs/ase-file-specs.md#header

    reader.seek(SeekFrom::Start(0x8))?;

    Ok(ImageSize {
        width: read_u16(reader, &Endian::Little)? as usize,
        height: read_u16(reader, &Endian::Little)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.len() >= 12 && &header[4..6] == b"\xE0\xA5"
}
