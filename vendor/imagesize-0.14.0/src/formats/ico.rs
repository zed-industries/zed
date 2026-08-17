use crate::util::*;
use crate::{ImageError, ImageResult, ImageSize};

use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(4))?;
    let img_count = read_u16(reader, &Endian::Little)?;
    let mut sizes = Vec::with_capacity(img_count as usize);

    for _ in 0..img_count {
        if let Ok(size) = ico_image_size(reader) {
            sizes.push(size)
        } else {
            // if we don't have all the bytes of the headers, just
            // return the largest one found so far
            break;
        }
        // each ICONDIRENTRY (image header) is 16 bytes, skip the last 14
        reader.seek(SeekFrom::Current(14))?;
    }
    sizes.into_iter().max().ok_or(ImageError::CorruptedImage)
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(&[0, 0, 1, 0])
}

/// Reads two bytes to determine an individual image's size within an ICO
fn ico_image_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // ICO dimensions are 1-256 pixels, with a byte value of 0 representing 256
    Ok(ImageSize {
        width: read_u8(reader)?.wrapping_sub(1) as usize + 1,
        height: read_u8(reader)?.wrapping_sub(1) as usize + 1,
    })
}
