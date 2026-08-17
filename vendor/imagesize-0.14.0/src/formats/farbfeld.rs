use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u32, Endian},
    ImageResult, ImageSize,
};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(8))?;

    Ok(ImageSize {
        width: read_u32(reader, &Endian::Big)? as usize,
        height: read_u32(reader, &Endian::Big)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"farbfeld")
}
