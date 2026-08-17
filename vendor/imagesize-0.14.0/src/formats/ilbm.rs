use crate::{
    util::{read_u16, read_u32, Endian},
    ImageResult, ImageSize,
};
use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // skip the IFF header
    reader.seek(SeekFrom::Start(12))?;

    let mut chunk_id = [0; 4];

    loop {
        reader.read_exact(&mut chunk_id)?;
        let chunk_length = read_u32(reader, &Endian::Big)? as i64;

        if &chunk_id == b"BMHD" {
            return Ok(ImageSize {
                width: read_u16(reader, &Endian::Big)? as usize,
                height: read_u16(reader, &Endian::Big)? as usize,
            });
        }

        // the BMHD chunk must occur before the BODY chunk
        if &chunk_id == b"BODY" {
            return Err(crate::ImageError::CorruptedImage);
        }

        // skip over the chunk; chunks of odd length have a padding byte
        reader.seek(SeekFrom::Current(chunk_length + chunk_length % 2))?;
    }
}

pub fn matches(header: &[u8]) -> bool {
    header.len() >= 12
        && &header[0..4] == b"FORM"
        && (&header[8..12] == b"ILBM" || &header[8..12] == b"PBM ")
}
