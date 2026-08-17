use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u32, Endian},
    ImageResult, ImageSize,
};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(12))?;
    let height = read_u32(reader, &Endian::Little)? as usize;
    let width = read_u32(reader, &Endian::Little)? as usize;
    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"DDS ")
}
