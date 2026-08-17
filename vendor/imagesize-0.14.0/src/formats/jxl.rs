use crate::util::*;
use crate::{ImageError, ImageResult, ImageSize};

use std::io::{BufRead, Read, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    let mut file_header = [0; 16]; // The size is variable, but doesn't exceed 16 bytes
    let mut header_size = 0;

    reader.seek(SeekFrom::Start(0))?;
    reader.read_exact(&mut file_header[..2])?;

    if &file_header[..2] == b"\xFF\x0A" {
        // Raw data: Read header directly
        header_size = reader.read(&mut file_header[2..])? + 2;
    } else {
        // Container format: Read from a single jxlc box or multiple jxlp boxes
        reader.seek(SeekFrom::Start(12))?;

        loop {
            let (box_type, box_size) = read_tag(reader)?;
            let box_start = reader.stream_position()? - 8;

            // If box_size is 1, the real size is stored in the first 8 bytes of content.
            // If box_size is 0, the box ends at EOF.

            let box_size = match box_size {
                1 => {
                    let mut box_size = [0; 8];
                    reader.read_exact(&mut box_size)?;
                    u64::from_be_bytes(box_size)
                }
                _ => box_size as u64,
            };

            let box_end = box_start
                .checked_add(box_size)
                .ok_or(ImageError::CorruptedImage)?;
            let box_header_size = reader.stream_position()? - box_start;

            if box_size != 0 && box_size < box_header_size {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid size for {} box: {}", box_type, box_size),
                )
                .into());
            }

            let mut box_reader = match box_size {
                0 => reader.take(file_header.len() as u64),
                _ => reader.take(box_size - box_header_size),
            };

            // The jxlc box must contain the complete codestream

            if box_type == "jxlc" {
                header_size = box_reader.read(&mut file_header)?;
                break;
            }

            // Or it could be stored as part of multiple jxlp boxes

            if box_type == "jxlp" {
                let mut jxlp_index = [0; 4];
                box_reader.read_exact(&mut jxlp_index)?;

                header_size += box_reader.read(&mut file_header[header_size..])?;

                // If jxlp_index has the high bit set to 1, this is the final jxlp box
                if header_size == file_header.len() || (jxlp_index[0] & 0x80) != 0 {
                    break;
                }
            }

            if box_size == 0 {
                break;
            }

            reader.seek(SeekFrom::Start(box_end))?;
        }
    }

    if header_size < 2 {
        return Err(ImageError::CorruptedImage);
    }

    if &file_header[0..2] != b"\xFF\x0A" {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JXL signature").into(),
        );
    }

    // Parse the header data

    let file_header = u128::from_le_bytes(file_header);
    let header_size = 8 * header_size;

    let is_small = read_bits(file_header, 1, 16, header_size)? != 0;

    // Extract image height:
    //     For small images, the height is stored in the next 5 bits
    //     For non-small images, the next two bits are used to determine the number of bits to read

    let height_selector = read_bits(file_header, 2, 17, header_size)?;

    let (height_bits, height_offset, height_shift) = match (is_small, height_selector) {
        (true, _) => (5, 17, 3),
        (false, 0) => (9, 19, 0),
        (false, 1) => (13, 19, 0),
        (false, 2) => (18, 19, 0),
        (false, 3) => (30, 19, 0),
        (false, _) => (0, 0, 0),
    };

    let height =
        (read_bits(file_header, height_bits, height_offset, header_size)? + 1) << height_shift;

    // Extract image width:
    //     If ratio is 0, use the same logic as before
    //     Otherwise, the width is calculated using a predefined aspect ratio

    let ratio = read_bits(file_header, 3, height_bits + height_offset, header_size)?;
    let width_selector = read_bits(file_header, 2, height_bits + height_offset + 3, 128)?;

    let (width_bits, width_offset, width_shift) = match (is_small, width_selector) {
        (true, _) => (5, 25, 3),
        (false, 0) => (9, height_bits + height_offset + 5, 0),
        (false, 1) => (13, height_bits + height_offset + 5, 0),
        (false, 2) => (18, height_bits + height_offset + 5, 0),
        (false, 3) => (30, height_bits + height_offset + 5, 0),
        (false, _) => (0, 0, 0),
    };

    let width = match ratio {
        1 => height,             // 1:1
        2 => (height / 10) * 12, // 12:10
        3 => (height / 3) * 4,   // 4:3
        4 => (height / 2) * 3,   // 3:2
        5 => (height / 9) * 16,  // 16:9
        6 => (height / 4) * 5,   // 5:4
        7 => height * 2,         // 2:1
        _ => (read_bits(file_header, width_bits, width_offset, header_size)? + 1) << width_shift,
    };

    // Extract orientation:
    //     This value overrides the orientation in EXIF metadata

    let metadata_offset = match ratio {
        0 => width_bits + width_offset,
        _ => height_bits + height_offset + 3,
    };

    let all_default = read_bits(file_header, 1, metadata_offset, header_size)? != 0;

    let orientation = match all_default {
        true => 0,
        false => {
            let extra_fields = read_bits(file_header, 1, metadata_offset + 1, header_size)? != 0;

            match extra_fields {
                false => 0,
                true => read_bits(file_header, 3, metadata_offset + 2, header_size)?,
            }
        }
    };

    if orientation < 4 {
        Ok(ImageSize { width, height })
    } else {
        Ok(ImageSize {
            width: height,
            height: width,
        })
    }
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"\xFF\x0A") || header.starts_with(b"\x00\x00\x00\x0CJXL \x0D\x0A\x87\x0A")
}
