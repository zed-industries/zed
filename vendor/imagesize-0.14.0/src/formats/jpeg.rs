use crate::util::*;
use crate::{ImageError, ImageResult, ImageSize};

use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    let mut marker = [0; 2];
    let mut depth = 0i32;

    //  Go to the first tag after FF D8
    reader.seek(SeekFrom::Start(2))?;

    loop {
        //  Read current marker (FF XX)
        reader.read_exact(&mut marker)?;

        if marker[0] != 0xFF {
            //  Did not read a marker. Assume image is corrupt.
            return Err(ImageError::CorruptedImage);
        }

        let page = marker[1];

        //  Check for valid SOFn markers. C4, C8, and CC aren't dimension markers.
        if (0xC0..=0xC3).contains(&page)
            || (0xC5..=0xC7).contains(&page)
            || (0xC9..=0xCB).contains(&page)
            || (0xCD..=0xCF).contains(&page)
        {
            //  Only get outside image size
            if depth == 0 {
                //  Correct marker, go forward 3 bytes so we're at height offset
                reader.seek(SeekFrom::Current(3))?;
                break;
            }
        } else if page == 0xD8 {
            depth += 1;
        } else if page == 0xD9 {
            depth -= 1;
            if depth < 0 {
                return Err(ImageError::CorruptedImage);
            }
        }

        //  Read the marker length and skip over it entirely
        let page_size = read_u16(reader, &Endian::Big)? as i64;
        reader.seek(SeekFrom::Current(page_size - 2))?;
    }

    Ok(ImageSize {
        height: read_u16(reader, &Endian::Big)? as usize,
        width: read_u16(reader, &Endian::Big)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"\xFF\xD8\xFF")
}
