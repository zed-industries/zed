use std::io::{BufRead, Seek, SeekFrom};

use crate::{util::*, ImageResult, ImageSize};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(12))?;

    let width = read_u16(reader, &Endian::Little)? as usize;
    let height = read_u16(reader, &Endian::Little)? as usize;

    Ok(ImageSize { width, height })
}

pub fn matches<R: BufRead + Seek>(header: &[u8], reader: &mut R) -> bool {
    // Do a naive check first to filter out any obviously non-TGA files
    let colormap_type = header[1];
    let image_type = header[2];

    // Check the image type (byte 2) to be one of the uncompressed or RLE compressed types
    // Note: I've seen mention of types 0, 32, and 33 but have no example files so have omitted them.
    if image_type != 1
        && image_type != 2
        && image_type != 3
        && image_type != 9
        && image_type != 10
        && image_type != 11
    {
        return false;
    }

    // Check that the colormap type (byte 1) is either 0 (no colormap) or 1 (colormap present)
    // Technically 2-127 is reserved and 128-255 are usable by devs, but for simplicity we ignore them
    if colormap_type >= 2 {
        return false;
    }

    is_tga(reader, image_type, colormap_type).unwrap_or(false)
}

fn is_tga<R: BufRead + Seek>(
    reader: &mut R,
    image_type: u8,
    colormap_type: u8,
) -> ImageResult<bool> {
    // Attempt to go to footer section. This also doubles as a size check since
    // if there aren't 18 bytes available it will return an error.
    reader.seek(SeekFrom::End(-18))?;

    // Look for Version 2 TGA footer signature as it's the only concrete data to verify it's a TGA
    let mut signature = [0; 18];
    reader.read_exact(&mut signature)?;

    // If signature is found then we should be confident it's a TGA
    //
    // We do not reset the seek here because size methods should
    // be seeking themselves as a first step anyway.
    if &signature == b"TRUEVISION-XFILE.\0" {
        return Ok(true);
    }

    // Now we're into heuristic territory.
    // With no footer I don't believe there is a 100% way to verify whether given bytes
    // are a TGA or not. To get around this we add a few corroborating byte checks and
    // if they make up a valid TGA configuration we assume that it's a TGA.

    // If image type is color mapped, then color map type must be set to 1
    if (image_type == 1 || image_type == 9) && colormap_type != 1 {
        return Ok(false);
    }

    // Start reading the header information
    reader.seek(SeekFrom::Start(3))?;

    let colormap_offset = read_u32(reader, &Endian::Little)?;
    let colormap_size = read_u8(reader)?;

    // If there is no color map then assume that origin, length, and entry size must be 0
    if colormap_type == 0 {
        if colormap_offset != 0 {
            return Ok(false);
        }

        if colormap_size != 0 {
            return Ok(false);
        }
    }

    // Assume color map sizes must be a multiple of 8
    if colormap_type == 1
        && (colormap_size != 0
            && colormap_size != 8
            && colormap_size != 16
            && colormap_size != 24
            && colormap_size != 32)
    {
        return Ok(false);
    }

    reader.seek(SeekFrom::Start(16))?;
    let pixel_size = read_u8(reader)?;
    let descriptor = read_u8(reader)?;
    let alpha_bits = descriptor & 0x0F;

    // Reserved bit, must be set to 0
    if descriptor & 0x10 != 0 {
        return Ok(false);
    }

    // Assume pixel size must be a multiple of 8
    if pixel_size != 8 && pixel_size != 16 && pixel_size != 24 && pixel_size != 32 {
        return Ok(false);
    }

    // Verify that the alpha bits value makes sense given pixel size
    //
    // 8 and 24 bits have no alpha
    if (pixel_size == 8 || pixel_size == 24) && alpha_bits != 0 {
        return Ok(false);
    }

    // 16 bits can either have 0 or 1 bits of alpha
    if pixel_size == 16 && alpha_bits >= 2 {
        return Ok(false);
    }

    // 32 bits must have 8 bits of alpha, although I've seen one with 0?
    if pixel_size == 32 && (alpha_bits != 8 && alpha_bits != 0) {
        return Ok(false);
    }

    Ok(true)
}
