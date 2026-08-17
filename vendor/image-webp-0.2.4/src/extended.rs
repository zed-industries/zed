use super::lossless::LosslessDecoder;
use crate::decoder::DecodingError;
use byteorder_lite::ReadBytesExt;
use std::io::{BufRead, Read};

use crate::alpha_blending::do_alpha_blending;

#[derive(Debug, Clone)]
pub(crate) struct WebPExtendedInfo {
    pub(crate) alpha: bool,

    pub(crate) canvas_width: u32,
    pub(crate) canvas_height: u32,

    #[allow(unused)]
    pub(crate) icc_profile: bool,
    pub(crate) exif_metadata: bool,
    pub(crate) xmp_metadata: bool,
    pub(crate) animation: bool,

    pub(crate) background_color: Option<[u8; 4]>,
    pub(crate) background_color_hint: [u8; 4],
}

/// Composites a frame onto a canvas.
///
/// Starts by filling the rectangle occupied by the previous frame with the background
/// color, if provided. Then copies or blends the frame onto the canvas.
#[allow(clippy::too_many_arguments)]
pub(crate) fn composite_frame(
    canvas: &mut [u8],
    canvas_width: u32,
    canvas_height: u32,
    clear_color: Option<[u8; 4]>,
    frame: &[u8],
    frame_offset_x: u32,
    frame_offset_y: u32,
    frame_width: u32,
    frame_height: u32,
    frame_has_alpha: bool,
    frame_use_alpha_blending: bool,
    previous_frame_width: u32,
    previous_frame_height: u32,
    previous_frame_offset_x: u32,
    previous_frame_offset_y: u32,
) {
    let frame_is_full_size = frame_offset_x == 0
        && frame_offset_y == 0
        && frame_width == canvas_width
        && frame_height == canvas_height;

    if frame_is_full_size && !frame_use_alpha_blending {
        if frame_has_alpha {
            canvas.copy_from_slice(frame);
        } else {
            for (input, output) in frame.chunks_exact(3).zip(canvas.chunks_exact_mut(4)) {
                output[..3].copy_from_slice(input);
                output[3] = 255;
            }
        }
        return;
    }

    // clear rectangle occupied by previous frame
    if let Some(clear_color) = clear_color {
        match (frame_is_full_size, frame_has_alpha) {
            (true, true) => {
                for pixel in canvas.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&clear_color);
                }
            }
            (true, false) => {
                for pixel in canvas.chunks_exact_mut(3) {
                    pixel.copy_from_slice(&clear_color[..3]);
                }
            }
            (false, true) => {
                for y in 0..previous_frame_height as usize {
                    for x in 0..previous_frame_width as usize {
                        let canvas_index = ((x + previous_frame_offset_x as usize)
                            + (y + previous_frame_offset_y as usize) * canvas_width as usize)
                            * 4;

                        let output = &mut canvas[canvas_index..][..4];
                        output.copy_from_slice(&clear_color);
                    }
                }
            }
            (false, false) => {
                for y in 0..previous_frame_height as usize {
                    for x in 0..previous_frame_width as usize {
                        // let frame_index = (x + y * frame_width as usize) * 4;
                        let canvas_index = ((x + previous_frame_offset_x as usize)
                            + (y + previous_frame_offset_y as usize) * canvas_width as usize)
                            * 3;

                        let output = &mut canvas[canvas_index..][..3];
                        output.copy_from_slice(&clear_color[..3]);
                    }
                }
            }
        }
    }

    let width = frame_width.min(canvas_width.saturating_sub(frame_offset_x)) as usize;
    let height = frame_height.min(canvas_height.saturating_sub(frame_offset_y)) as usize;

    if frame_has_alpha && frame_use_alpha_blending {
        for y in 0..height {
            for x in 0..width {
                let frame_index = (x + y * frame_width as usize) * 4;
                let canvas_index = ((x + frame_offset_x as usize)
                    + (y + frame_offset_y as usize) * canvas_width as usize)
                    * 4;

                let input = &frame[frame_index..][..4];
                let output = &mut canvas[canvas_index..][..4];

                let blended =
                    do_alpha_blending(input.try_into().unwrap(), output.try_into().unwrap());
                output.copy_from_slice(&blended);
            }
        }
    } else if frame_has_alpha {
        for y in 0..height {
            let frame_index = (y * frame_width as usize) * 4;
            let canvas_index = (frame_offset_x as usize
                + (y + frame_offset_y as usize) * canvas_width as usize)
                * 4;

            canvas[canvas_index..][..width * 4].copy_from_slice(&frame[frame_index..][..width * 4]);
        }
    } else {
        for y in 0..height {
            let index = (y * frame_width as usize) * 3;
            let canvas_index = (frame_offset_x as usize
                + (y + frame_offset_y as usize) * canvas_width as usize)
                * 4;
            let input = &frame[index..][..width * 3];
            let output = &mut canvas[canvas_index..][..width * 4];

            for (input, output) in input.chunks_exact(3).zip(output.chunks_exact_mut(4)) {
                output[..3].copy_from_slice(input);
                output[3] = 255;
            }
        }
    }
}

pub(crate) fn get_alpha_predictor(
    x: usize,
    y: usize,
    width: usize,
    filtering_method: FilteringMethod,
    image_slice: &[u8],
) -> u8 {
    match filtering_method {
        FilteringMethod::None => 0,
        FilteringMethod::Horizontal => {
            if x == 0 && y == 0 {
                0
            } else if x == 0 {
                let index = (y - 1) * width + x;
                image_slice[index * 4 + 3]
            } else {
                let index = y * width + x - 1;
                image_slice[index * 4 + 3]
            }
        }
        FilteringMethod::Vertical => {
            if x == 0 && y == 0 {
                0
            } else if y == 0 {
                let index = y * width + x - 1;
                image_slice[index * 4 + 3]
            } else {
                let index = (y - 1) * width + x;
                image_slice[index * 4 + 3]
            }
        }
        FilteringMethod::Gradient => {
            let (left, top, top_left) = match (x, y) {
                (0, 0) => (0, 0, 0),
                (0, y) => {
                    let above_index = (y - 1) * width + x;
                    let val = image_slice[above_index * 4 + 3];
                    (val, val, val)
                }
                (x, 0) => {
                    let before_index = y * width + x - 1;
                    let val = image_slice[before_index * 4 + 3];
                    (val, val, val)
                }
                (x, y) => {
                    let left_index = y * width + x - 1;
                    let left = image_slice[left_index * 4 + 3];
                    let top_index = (y - 1) * width + x;
                    let top = image_slice[top_index * 4 + 3];
                    let top_left_index = (y - 1) * width + x - 1;
                    let top_left = image_slice[top_left_index * 4 + 3];

                    (left, top, top_left)
                }
            };

            let combination = i16::from(left) + i16::from(top) - i16::from(top_left);
            i16::clamp(combination, 0, 255).try_into().unwrap()
        }
    }
}

pub(crate) fn read_extended_header<R: Read>(
    reader: &mut R,
) -> Result<WebPExtendedInfo, DecodingError> {
    let chunk_flags = reader.read_u8()?;

    let icc_profile = chunk_flags & 0b00100000 != 0;
    let alpha = chunk_flags & 0b00010000 != 0;
    let exif_metadata = chunk_flags & 0b00001000 != 0;
    let xmp_metadata = chunk_flags & 0b00000100 != 0;
    let animation = chunk_flags & 0b00000010 != 0;

    // reserved bytes are ignored
    let _reserved_bytes = read_3_bytes(reader)?;

    let canvas_width = read_3_bytes(reader)? + 1;
    let canvas_height = read_3_bytes(reader)? + 1;

    //product of canvas dimensions cannot be larger than u32 max
    if u32::checked_mul(canvas_width, canvas_height).is_none() {
        return Err(DecodingError::ImageTooLarge);
    }

    let info = WebPExtendedInfo {
        icc_profile,
        alpha,
        exif_metadata,
        xmp_metadata,
        animation,
        canvas_width,
        canvas_height,
        background_color_hint: [0; 4],
        background_color: None,
    };

    Ok(info)
}

pub(crate) fn read_3_bytes<R: Read>(reader: &mut R) -> Result<u32, DecodingError> {
    let mut buffer: [u8; 3] = [0; 3];
    reader.read_exact(&mut buffer)?;
    let value: u32 =
        (u32::from(buffer[2]) << 16) | (u32::from(buffer[1]) << 8) | u32::from(buffer[0]);
    Ok(value)
}

#[derive(Debug)]
pub(crate) struct AlphaChunk {
    _preprocessing: bool,
    pub(crate) filtering_method: FilteringMethod,
    pub(crate) data: Vec<u8>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum FilteringMethod {
    None,
    Horizontal,
    Vertical,
    Gradient,
}

pub(crate) fn read_alpha_chunk<R: BufRead>(
    reader: &mut R,
    width: u16,
    height: u16,
) -> Result<AlphaChunk, DecodingError> {
    let info_byte = reader.read_u8()?;

    let preprocessing = (info_byte & 0b00110000) >> 4;
    let filtering = (info_byte & 0b00001100) >> 2;
    let compression = info_byte & 0b00000011;

    let preprocessing = match preprocessing {
        0 => false,
        1 => true,
        _ => return Err(DecodingError::InvalidAlphaPreprocessing),
    };

    let filtering_method = match filtering {
        0 => FilteringMethod::None,
        1 => FilteringMethod::Horizontal,
        2 => FilteringMethod::Vertical,
        3 => FilteringMethod::Gradient,
        _ => unreachable!(),
    };

    let lossless_compression = match compression {
        0 => false,
        1 => true,
        _ => return Err(DecodingError::InvalidCompressionMethod),
    };

    let data = if lossless_compression {
        let mut decoder = LosslessDecoder::new(reader);

        let mut data = vec![0; usize::from(width) * usize::from(height) * 4];
        decoder.decode_frame(u32::from(width), u32::from(height), true, &mut data)?;

        let mut green = vec![0; usize::from(width) * usize::from(height)];
        for (rgba_val, green_val) in data.chunks_exact(4).zip(green.iter_mut()) {
            *green_val = rgba_val[1];
        }
        green
    } else {
        let mut framedata = vec![0; width as usize * height as usize];
        reader.read_exact(&mut framedata)?;
        framedata
    };

    let chunk = AlphaChunk {
        _preprocessing: preprocessing,
        filtering_method,
        data,
    };

    Ok(chunk)
}
