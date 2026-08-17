use crate::util::*;
use crate::{ImageResult, ImageSize};

use std::io::{self, BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(2))?;

    // We try to loop until we find a line that does not start with a comment
    // or is empty. After that, we should expect width and height back to back
    // separated by an arbitrary amount of whitespace.
    loop {
        // Lines can be arbitrarily long, but 1k is a good enough cap I think.
        // Anything higher and I blame whoever made the file.
        let line = read_until_whitespace(reader, 1024)?;
        let trimmed_line = line.trim();

        // If it's a comment, skip until newline
        if trimmed_line.starts_with('#') {
            read_until_capped(reader, b'\n', 1024)?;
            continue;
        }

        // If it's just empty skip
        if trimmed_line.is_empty() {
            continue;
        }

        // The first thing we read that isn't empty or a comment should be the width
        let raw_width = line;

        // Read in the next non-whitespace section as the height
        let line = read_until_whitespace(reader, 1024)?;
        let raw_height = line.trim();

        // Try to parse the width and height
        let width_parsed = raw_width.parse::<usize>().ok();
        let height_parsed = raw_height.parse::<usize>().ok();

        // If successful return it
        if let (Some(width), Some(height)) = (width_parsed, height_parsed) {
            return Ok(ImageSize { width, height });
        }

        // If no successful then assume that it cannot be read
        // If this happens we need to gather test files for those cases
        break;
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "PNM dimensions not found").into())
}

pub fn matches(header: &[u8]) -> bool {
    if header[0] != b'P' {
        return false;
    }

    // We only support P1 to P6. Currently ignoring P7, PF, PFM
    if header[1] < b'1' || header[1] > b'6' {
        return false;
    }

    true
}
