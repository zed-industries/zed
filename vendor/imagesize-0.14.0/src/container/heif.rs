use crate::util::*;
use crate::{ImageError, ImageResult, ImageSize};

use std::convert::TryInto;
use std::io::{BufRead, Seek, SeekFrom};

// REFS: https://github.com/strukturag/libheif/blob/f0c1a863cabbccb2d280515b7ecc73e6717702dc/libheif/heif.h#L600
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Compression {
    Av1,
    Hevc,
    Jpeg,
    Unknown,
    // unused(reuse in the future?)
    // Avc,
    // Vvc,
    // Evc,
}

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;
    //  Read the ftyp header size
    let ftyp_size = read_u32(reader, &Endian::Big)?;

    //  Jump to the first actual box offset
    reader.seek(SeekFrom::Start(ftyp_size.into()))?;

    //  Skip to meta tag which contains all the metadata
    skip_to_tag(reader, b"meta")?;
    read_u32(reader, &Endian::Big)?; //  Meta has a junk value after it
    skip_to_tag(reader, b"iprp")?; //  Find iprp tag

    let mut ipco_size = skip_to_tag(reader, b"ipco")? as usize; //  Find ipco tag

    //  Keep track of the max size of ipco tag
    let mut max_width = 0usize;
    let mut max_height = 0usize;
    let mut found_ispe = false;
    let mut rotation = 0u8;

    while let Ok((tag, size)) = read_tag(reader) {
        //  Size of tag length + tag cannot be under 8 (4 bytes each)
        if size < 8 {
            return Err(ImageError::CorruptedImage);
        }

        //  ispe tag has a junk value followed by width and height as u32
        if tag == "ispe" {
            found_ispe = true;
            read_u32(reader, &Endian::Big)?; //  Discard junk value
            let width = read_u32(reader, &Endian::Big)? as usize;
            let height = read_u32(reader, &Endian::Big)? as usize;

            //  Assign new largest size by area
            if width * height > max_width * max_height {
                max_width = width;
                max_height = height;
            }
        } else if tag == "irot" {
            // irot is 9 bytes total: size, tag, 1 byte for rotation (0-3)
            rotation = read_u8(reader)?;
        } else if size >= ipco_size {
            // If we've gone past the ipco boundary, then break
            break;
        } else {
            // If we're still inside ipco, consume all bytes for
            // the current tag, minus the bytes already read in `read_tag`
            ipco_size -= size;
            reader.seek(SeekFrom::Current(size as i64 - 8))?;
        }
    }

    //  If no ispe found, then we have no actual dimension data to use
    if !found_ispe {
        return Err(
            std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data").into(),
        );
    }

    //  Rotation can only be 0-3. 1 and 3 are 90 and 270 degrees respectively (anti-clockwise)
    //  If we have 90 or 270 rotation, flip width and height
    if rotation == 1 || rotation == 3 {
        std::mem::swap(&mut max_width, &mut max_height);
    }

    Ok(ImageSize {
        width: max_width,
        height: max_height,
    })
}

pub fn matches<R: BufRead + Seek>(header: &[u8], reader: &mut R) -> Option<Compression> {
    if header.len() < 12 || &header[4..8] != b"ftyp" {
        return None;
    }

    let brand: [u8; 4] = header[8..12].try_into().unwrap();

    if let Some(compression) = inner_matches(&brand) {
        // case 1: { heic, ... }
        return Some(compression);
    }

    // REFS: https://github.com/nokiatech/heif/blob/be43efdf273ae9cf90e552b99f16ac43983f3d19/srcs/reader/heifreaderimpl.cpp#L738
    let brands = [b"mif1", b"msf1", b"mif2", b"miaf"];

    if brands.contains(&&brand) {
        let mut buf = [0; 12];

        if reader.read_exact(&mut buf).is_err() {
            return Some(Compression::Unknown);
        }

        let brand2: [u8; 4] = buf[4..8].try_into().unwrap();

        if let Some(compression) = inner_matches(&brand2) {
            // case 2: { msf1, version, heic,  msf1, ... }
            //           brand          brand2 brand3
            return Some(compression);
        }

        if brands.contains(&&brand2) {
            // case 3: { msf1, version, msf1,  heic, ... }
            //           brand          brand2 brand3
            let brand3: [u8; 4] = buf[8..12].try_into().unwrap();

            if let Some(compression) = inner_matches(&brand3) {
                return Some(compression);
            }
        }
    }

    Some(Compression::Unknown)
}

fn inner_matches(brand: &[u8; 4]) -> Option<Compression> {
    // Since other non-heif files may contain ftype in the header
    // we try to use brands to distinguish image files specifically.
    // List of brands from here: https://mp4ra.org/#/brands
    let hevc_brands = [
        b"heic", b"heix", b"heis", b"hevs", b"heim", b"hevm", b"hevc", b"hevx",
    ];
    let av1_brands = [
        b"avif", b"avio", b"avis",
        // AVIF only
        // REFS: https://rawcdn.githack.com/AOMediaCodec/av1-avif/67a92add6cd642a8863e386fa4db87954a6735d1/index.html#advanced-profile
        b"MA1A", b"MA1B",
    ];
    let jpeg_brands = [b"jpeg", b"jpgs"];

    // unused
    // REFS: https://github.com/MPEGGroup/FileFormatConformance/blob/6eef4e4c8bc70e2af9aeb1d62e764a6235f9d6a6/data/standard_features/23008-12/brands.json
    // let avc_brands = [b"avci", b"avcs"];
    // let vvc_brands = [b"vvic", b"vvis"];
    // let evc_brands = [b"evbi", b"evbs", b"evmi", b"evms"];

    // Maybe unnecessary
    // REFS: https://github.com/nokiatech/heif/blob/be43efdf273ae9cf90e552b99f16ac43983f3d19/srcs/reader/heifreaderimpl.cpp#L1415
    // REFS: https://github.com/nokiatech/heif/blob/be43efdf273ae9cf90e552b99f16ac43983f3d19/srcs/api-cpp/ImageItem.h#L37
    // let feature_brands = [b"pred", b"auxl", b"thmb", b"base", b"dimg"];
    if hevc_brands.contains(&brand) {
        return Some(Compression::Hevc);
    }

    if av1_brands.contains(&brand) {
        return Some(Compression::Av1);
    }

    if jpeg_brands.contains(&brand) {
        return Some(Compression::Jpeg);
    }

    None
}

fn skip_to_tag<R: BufRead + Seek>(reader: &mut R, tag: &[u8]) -> ImageResult<u32> {
    let mut tag_buf = [0; 4];

    loop {
        let size = read_u32(reader, &Endian::Big)?;
        reader.read_exact(&mut tag_buf)?;

        if tag_buf == tag {
            return Ok(size);
        }

        if size >= 8 {
            reader.seek(SeekFrom::Current(size as i64 - 8))?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid heif box size: {}", size),
            )
            .into());
        }
    }
}
