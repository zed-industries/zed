use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u32, Endian},
    ImageResult, ImageSize,
};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(16))?;

    let width = read_u32(reader, &Endian::Little)? as usize;
    let height = read_u32(reader, &Endian::Little)? as usize;

    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    let ktx2_identifier = [
        0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
    ];
    header.starts_with(&ktx2_identifier)
}
