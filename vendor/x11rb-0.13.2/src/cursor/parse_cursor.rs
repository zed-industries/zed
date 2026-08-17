//! Parse the contents of a cursor file

// This code is loosely based on parse_cursor_file.c from libxcb-cursor, which is:
//   Copyright © 2013 Michael Stapelberg
// and is covered by MIT/X Consortium License

use std::io::{Read, Seek};
use xcursor::parser::{parse_xcursor_stream, Image};

/// An error that occurred while parsing
#[derive(Debug)]
pub(crate) enum Error {
    /// An I/O error occurred
    Io,

    /// The file contains no images
    NoImages,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::Io
    }
}

/// Find the size of the image in the toc with a size as close as possible to the desired size.
fn find_best_size(images: &[Image], desired_size: u32) -> Result<u32, Error> {
    fn distance(a: u32, b: u32) -> u32 {
        a.max(b) - a.min(b)
    }

    fn is_better(desired_size: u32, entry: &Image, result: &Result<u32, Error>) -> bool {
        match result {
            Err(_) => true,
            Ok(size) => distance(entry.size, desired_size) < distance(*size, desired_size),
        }
    }

    let mut result = Err(Error::NoImages);
    for entry in images {
        // If this is better than the best so far, replace best
        if is_better(desired_size, entry, &result) {
            result = Ok(entry.size)
        }
    }
    result
}

/// Parse a complete cursor file
pub(crate) fn parse_cursor<R: Read + Seek>(
    input: &mut R,
    desired_size: u32,
) -> Result<Vec<Image>, Error> {
    let mut cursors = parse_xcursor_stream(input)?;
    let size = find_best_size(&cursors, desired_size)?;
    cursors.retain(|image| image.size == size);
    Ok(cursors)
}

#[cfg(test)]
mod test {
    use super::{find_best_size, Error, Image};

    #[test]
    fn find_best_size_empty_input() {
        let res = find_best_size(&[], 42);
        match res {
            Err(Error::NoImages) => {}
            r => panic!("Unexpected result {r:?}"),
        }
    }

    fn fake_image_with_size(size: u32) -> Image {
        Image {
            size,
            width: 42,
            height: 42,
            xhot: 0,
            yhot: 0,
            delay: 0,
            pixels_rgba: Vec::new(),
            pixels_argb: Vec::new(),
        }
    }

    #[test]
    fn find_best_size_one_input() {
        let input = [fake_image_with_size(42)];
        assert_eq!(42, find_best_size(&input, 10).unwrap());
    }

    #[test]
    fn find_best_size_selects_better() {
        let input = [
            fake_image_with_size(42),
            fake_image_with_size(32),
            fake_image_with_size(3),
            fake_image_with_size(22),
        ];
        assert_eq!(3, find_best_size(&input, 10).unwrap());
    }
}
