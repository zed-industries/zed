use std::io::{self, BufRead, Seek, SeekFrom};

use crate::{
    util::{read_i32, read_null_terminated_string, read_u32, Endian},
    ImageResult, ImageSize,
};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(4))?;

    // If long names flag is set then max attribute name and type name is 255, otherwise it's only 31
    let flags = read_u32(reader, &Endian::Little)?;
    let long_names = flags & 0x400 != 0;
    let max_name_size = if long_names { 255 } else { 31 };

    // Read header attributes until we find the dataWindow attribute
    loop {
        let attr_name = read_null_terminated_string(reader, max_name_size)?;
        if attr_name.is_empty() {
            break; // End of the header
        }

        let attr_type = read_null_terminated_string(reader, max_name_size)?;

        // Skip attr_size
        let attr_size = read_u32(reader, &Endian::Little)?;

        if attr_name == "dataWindow" && attr_type == "box2i" {
            // Read the data window values
            let x_min = read_i32(reader, &Endian::Little)? as i64;
            let y_min = read_i32(reader, &Endian::Little)? as i64;
            let x_max = read_i32(reader, &Endian::Little)? as i64;
            let y_max = read_i32(reader, &Endian::Little)? as i64;

            if x_min > x_max || y_min > y_max {
                continue;
            }

            let width = (x_max - x_min + 1) as usize;
            let height = (y_max - y_min + 1) as usize;

            return Ok(ImageSize { width, height });
        } else {
            // Skip the attribute value
            reader.seek(SeekFrom::Current(attr_size as i64))?;
        }
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "Data window not found").into())
}

pub fn matches(header: &[u8]) -> bool {
    let exr_magic_number = [0x76, 0x2f, 0x31, 0x01];
    header.starts_with(&exr_magic_number)
}
