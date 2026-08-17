use byteorder_lite::{LittleEndian, WriteBytesExt};
use std::io::{self, Write};

use crate::error::{
    EncodingError, ImageError, ImageFormatHint, ImageResult, ParameterError, ParameterErrorKind,
    UnsupportedError, UnsupportedErrorKind,
};
use crate::{DynamicImage, ExtendedColorType, ImageEncoder, ImageFormat};

const BITMAPFILEHEADER_SIZE: u32 = 14;
const BITMAPINFOHEADER_SIZE: u32 = 40;
const BITMAPV4HEADER_SIZE: u32 = 108;

/// The representation of a BMP encoder.
pub struct BmpEncoder<'a, W: 'a> {
    writer: &'a mut W,
}

impl<'a, W: Write + 'a> BmpEncoder<'a, W> {
    /// Create a new encoder that writes its output to ```w```.
    pub fn new(w: &'a mut W) -> Self {
        BmpEncoder { writer: w }
    }

    /// Encodes the image `image` that has dimensions `width` and `height` and `ExtendedColorType` `c`.
    ///
    /// # Panics
    ///
    /// Panics if `width * height * c.bytes_per_pixel() != image.len()`.
    #[track_caller]
    pub fn encode(
        &mut self,
        image: &[u8],
        width: u32,
        height: u32,
        c: ExtendedColorType,
    ) -> ImageResult<()> {
        self.encode_with_palette(image, width, height, c, None)
    }

    /// Same as `encode`, but allow a palette to be passed in. The `palette` is ignored for color
    /// types other than Luma/Luma-with-alpha.
    ///
    /// # Panics
    ///
    /// Panics if `width * height * c.bytes_per_pixel() != image.len()`.
    #[track_caller]
    pub fn encode_with_palette(
        &mut self,
        image: &[u8],
        width: u32,
        height: u32,
        color_type: ExtendedColorType,
        palette: Option<&[[u8; 3]]>,
    ) -> ImageResult<()> {
        if palette.is_some()
            && color_type != ExtendedColorType::L8
            && color_type != ExtendedColorType::La8
        {
            return Err(ImageError::Parameter(ParameterError::from_kind(
                ParameterErrorKind::Generic(
                    "Palette given which must only be used with L8 or La8 color types".to_string(),
                ),
            )));
        }

        let expected_buffer_len = color_type.buffer_size(width, height);
        assert_eq!(
            expected_buffer_len,
            image.len() as u64,
            "Invalid buffer length: expected {expected_buffer_len} got {} for {width}x{height} image",
            image.len(),
        );

        let bmp_header_size = BITMAPFILEHEADER_SIZE;

        let (dib_header_size, written_pixel_size, palette_color_count) =
            written_pixel_info(color_type, palette)?;

        let (padded_row, image_size) = width
            .checked_mul(written_pixel_size)
            // each row must be padded to a multiple of 4 bytes
            .and_then(|v| v.checked_next_multiple_of(4))
            .and_then(|v| {
                let image_bytes = v.checked_mul(height)?;
                Some((v, image_bytes))
            })
            .ok_or_else(|| {
                ImageError::Parameter(ParameterError::from_kind(
                    ParameterErrorKind::DimensionMismatch,
                ))
            })?;

        let row_padding = padded_row - width * written_pixel_size;

        // all palette colors are BGRA
        let palette_size = palette_color_count.checked_mul(4).ok_or_else(|| {
            ImageError::Encoding(EncodingError::new(
                ImageFormatHint::Exact(ImageFormat::Bmp),
                "calculated palette size larger than 2^32",
            ))
        })?;

        let file_size = bmp_header_size
            .checked_add(dib_header_size)
            .and_then(|v| v.checked_add(palette_size))
            .and_then(|v| v.checked_add(image_size))
            .ok_or_else(|| {
                ImageError::Encoding(EncodingError::new(
                    ImageFormatHint::Exact(ImageFormat::Bmp),
                    "calculated BMP header size larger than 2^32",
                ))
            })?;

        let image_data_offset = bmp_header_size
            .checked_add(dib_header_size)
            .and_then(|v| v.checked_add(palette_size))
            .ok_or_else(|| {
                ImageError::Encoding(EncodingError::new(
                    ImageFormatHint::Exact(ImageFormat::Bmp),
                    "calculated BMP size larger than 2^32",
                ))
            })?;

        // write BMP header
        self.writer.write_u8(b'B')?;
        self.writer.write_u8(b'M')?;
        self.writer.write_u32::<LittleEndian>(file_size)?; // file size
        self.writer.write_u16::<LittleEndian>(0)?; // reserved 1
        self.writer.write_u16::<LittleEndian>(0)?; // reserved 2
        self.writer.write_u32::<LittleEndian>(image_data_offset)?; // image data offset

        // write DIB header
        self.writer.write_u32::<LittleEndian>(dib_header_size)?;
        self.writer.write_i32::<LittleEndian>(width as i32)?;
        self.writer.write_i32::<LittleEndian>(height as i32)?;
        self.writer.write_u16::<LittleEndian>(1)?; // color planes
        self.writer
            .write_u16::<LittleEndian>((written_pixel_size * 8) as u16)?; // bits per pixel
        if dib_header_size >= BITMAPV4HEADER_SIZE {
            // Assume BGRA32
            self.writer.write_u32::<LittleEndian>(3)?; // compression method - bitfields
        } else {
            self.writer.write_u32::<LittleEndian>(0)?; // compression method - no compression
        }
        self.writer.write_u32::<LittleEndian>(image_size)?;
        self.writer.write_i32::<LittleEndian>(0)?; // horizontal ppm
        self.writer.write_i32::<LittleEndian>(0)?; // vertical ppm
        self.writer.write_u32::<LittleEndian>(palette_color_count)?;
        self.writer.write_u32::<LittleEndian>(0)?; // all colors are important
        if dib_header_size >= BITMAPV4HEADER_SIZE {
            // Assume BGRA32
            self.writer.write_u32::<LittleEndian>(0xff << 16)?; // red mask
            self.writer.write_u32::<LittleEndian>(0xff << 8)?; // green mask
            self.writer.write_u32::<LittleEndian>(0xff)?; // blue mask
            self.writer.write_u32::<LittleEndian>(0xff << 24)?; // alpha mask
            self.writer.write_u32::<LittleEndian>(0x7352_4742)?; // colorspace - sRGB

            // endpoints (3x3) and gamma (3)
            for _ in 0..12 {
                self.writer.write_u32::<LittleEndian>(0)?;
            }
        }

        // write image data
        match color_type {
            ExtendedColorType::Rgb8 => self.encode_rgb(image, width, height, row_padding, 3)?,
            ExtendedColorType::Rgba8 => self.encode_rgba(image, width, height, row_padding, 4)?,
            ExtendedColorType::L8 => {
                self.encode_gray(image, width, height, row_padding, 1, palette)?;
            }
            ExtendedColorType::La8 => {
                self.encode_gray(image, width, height, row_padding, 2, palette)?;
            }
            _ => {
                return Err(ImageError::Unsupported(
                    UnsupportedError::from_format_and_kind(
                        ImageFormat::Bmp.into(),
                        UnsupportedErrorKind::Color(color_type),
                    ),
                ));
            }
        }

        Ok(())
    }

    fn encode_rgb(
        &mut self,
        image: &[u8],
        width: u32,
        height: u32,
        row_padding: u32,
        bytes_per_pixel: u32,
    ) -> io::Result<()> {
        let width = width as usize;
        let height = height as usize;
        let x_stride = bytes_per_pixel as usize;
        let y_stride = width * x_stride;
        for row in (0..height).rev() {
            // from the bottom up
            let row_start = row * y_stride;
            for px in image[row_start..][..y_stride].chunks_exact(x_stride) {
                let r = px[0];
                let g = px[1];
                let b = px[2];
                // written as BGR
                self.writer.write_all(&[b, g, r])?;
            }
            self.write_row_pad(row_padding)?;
        }

        Ok(())
    }

    fn encode_rgba(
        &mut self,
        image: &[u8],
        width: u32,
        height: u32,
        row_padding: u32,
        bytes_per_pixel: u32,
    ) -> io::Result<()> {
        let width = width as usize;
        let height = height as usize;
        let x_stride = bytes_per_pixel as usize;
        let y_stride = width * x_stride;
        for row in (0..height).rev() {
            // from the bottom up
            let row_start = row * y_stride;
            for px in image[row_start..][..y_stride].chunks_exact(x_stride) {
                let r = px[0];
                let g = px[1];
                let b = px[2];
                let a = px[3];
                // written as BGRA
                self.writer.write_all(&[b, g, r, a])?;
            }
            self.write_row_pad(row_padding)?;
        }

        Ok(())
    }

    fn encode_gray(
        &mut self,
        image: &[u8],
        width: u32,
        height: u32,
        row_padding: u32,
        bytes_per_pixel: u32,
        palette: Option<&[[u8; 3]]>,
    ) -> io::Result<()> {
        // write grayscale palette
        if let Some(palette) = palette {
            for item in palette {
                // each color is written as BGRA, where A is always 0
                self.writer.write_all(&[item[2], item[1], item[0], 0])?;
            }
        } else {
            for val in 0u8..=255 {
                // each color is written as BGRA, where A is always 0 and since only grayscale is being written, B = G = R = index
                self.writer.write_all(&[val, val, val, 0])?;
            }
        }

        // write image data
        let x_stride = bytes_per_pixel;
        let y_stride = width * x_stride;
        for row in (0..height).rev() {
            // from the bottom up
            let row_start = row * y_stride;

            // color value is equal to the palette index
            if x_stride == 1 {
                // improve performance by writing the whole row at once
                self.writer
                    .write_all(&image[row_start as usize..][..y_stride as usize])?;
            } else {
                for col in 0..width {
                    let pixel_start = (row_start + (col * x_stride)) as usize;
                    self.writer.write_u8(image[pixel_start])?;
                    // alpha is never written as it's not widely supported
                }
            }

            self.write_row_pad(row_padding)?;
        }

        Ok(())
    }

    fn write_row_pad(&mut self, row_pad_size: u32) -> io::Result<()> {
        for _ in 0..row_pad_size {
            self.writer.write_u8(0)?;
        }

        Ok(())
    }
}

impl<W: Write> ImageEncoder for BmpEncoder<'_, W> {
    #[track_caller]
    fn write_image(
        mut self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: ExtendedColorType,
    ) -> ImageResult<()> {
        self.encode(buf, width, height, color_type)
    }

    fn make_compatible_img(
        &self,
        _: crate::io::encoder::MethodSealedToImage,
        img: &DynamicImage,
    ) -> Option<DynamicImage> {
        crate::io::encoder::dynimage_conversion_8bit(img)
    }
}

/// Returns a tuple representing: (dib header size, written pixel size, palette color count).
fn written_pixel_info(
    c: ExtendedColorType,
    palette: Option<&[[u8; 3]]>,
) -> Result<(u32, u32, u32), ImageError> {
    let (header, color_bytes, palette_count) = match c {
        ExtendedColorType::Rgb8 => (BITMAPINFOHEADER_SIZE, 3, Some(0)),
        ExtendedColorType::Rgba8 => (BITMAPV4HEADER_SIZE, 4, Some(0)),
        ExtendedColorType::L8 => (
            BITMAPINFOHEADER_SIZE,
            1,
            u32::try_from(palette.map(|p| p.len()).unwrap_or(256)).ok(),
        ),
        ExtendedColorType::La8 => (
            BITMAPINFOHEADER_SIZE,
            1,
            u32::try_from(palette.map(|p| p.len()).unwrap_or(256)).ok(),
        ),
        _ => {
            return Err(ImageError::Unsupported(
                UnsupportedError::from_format_and_kind(
                    ImageFormat::Bmp.into(),
                    UnsupportedErrorKind::Color(c),
                ),
            ));
        }
    };

    let palette_count = palette_count.ok_or_else(|| {
        ImageError::Encoding(EncodingError::new(
            ImageFormatHint::Exact(ImageFormat::Bmp),
            "calculated palette size larger than 2^32",
        ))
    })?;

    Ok((header, color_bytes, palette_count))
}

#[cfg(test)]
mod tests {
    use super::super::BmpDecoder;
    use super::BmpEncoder;

    use crate::ExtendedColorType;
    use crate::ImageDecoder as _;
    use std::io::Cursor;

    fn round_trip_image(image: &[u8], width: u32, height: u32, c: ExtendedColorType) -> Vec<u8> {
        let mut encoded_data = Vec::new();
        {
            let mut encoder = BmpEncoder::new(&mut encoded_data);
            encoder
                .encode(image, width, height, c)
                .expect("could not encode image");
        }

        let decoder = BmpDecoder::new(Cursor::new(&encoded_data)).expect("failed to decode");

        let mut buf = vec![0; decoder.total_bytes() as usize];
        decoder.read_image(&mut buf).expect("failed to decode");
        buf
    }

    #[test]
    fn round_trip_single_pixel_rgb() {
        let image = [255u8, 0, 0]; // single red pixel
        let decoded = round_trip_image(&image, 1, 1, ExtendedColorType::Rgb8);
        assert_eq!(3, decoded.len());
        assert_eq!(255, decoded[0]);
        assert_eq!(0, decoded[1]);
        assert_eq!(0, decoded[2]);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn huge_files_return_error() {
        let mut encoded_data = Vec::new();
        let image = vec![0u8; 3 * 40_000 * 40_000]; // 40_000x40_000 pixels, 3 bytes per pixel, allocated on the heap
        let mut encoder = BmpEncoder::new(&mut encoded_data);
        let result = encoder.encode(&image, 40_000, 40_000, ExtendedColorType::Rgb8);
        assert!(result.is_err());
    }

    #[test]
    fn round_trip_single_pixel_rgba() {
        let image = [1, 2, 3, 4];
        let decoded = round_trip_image(&image, 1, 1, ExtendedColorType::Rgba8);
        assert_eq!(&decoded[..], &image[..]);
    }

    #[test]
    fn round_trip_3px_rgb() {
        let image = [0u8; 3 * 3 * 3]; // 3x3 pixels, 3 bytes per pixel
        let _decoded = round_trip_image(&image, 3, 3, ExtendedColorType::Rgb8);
    }

    #[test]
    fn round_trip_gray() {
        let image = [0u8, 1, 2]; // 3 pixels
        let decoded = round_trip_image(&image, 3, 1, ExtendedColorType::L8);
        // should be read back as 3 RGB pixels
        assert_eq!(9, decoded.len());
        assert_eq!(0, decoded[0]);
        assert_eq!(0, decoded[1]);
        assert_eq!(0, decoded[2]);
        assert_eq!(1, decoded[3]);
        assert_eq!(1, decoded[4]);
        assert_eq!(1, decoded[5]);
        assert_eq!(2, decoded[6]);
        assert_eq!(2, decoded[7]);
        assert_eq!(2, decoded[8]);
    }

    #[test]
    fn round_trip_graya() {
        let image = [0u8, 0, 1, 0, 2, 0]; // 3 pixels, each with an alpha channel
        let decoded = round_trip_image(&image, 1, 3, ExtendedColorType::La8);
        // should be read back as 3 RGB pixels
        assert_eq!(9, decoded.len());
        assert_eq!(0, decoded[0]);
        assert_eq!(0, decoded[1]);
        assert_eq!(0, decoded[2]);
        assert_eq!(1, decoded[3]);
        assert_eq!(1, decoded[4]);
        assert_eq!(1, decoded[5]);
        assert_eq!(2, decoded[6]);
        assert_eq!(2, decoded[7]);
        assert_eq!(2, decoded[8]);
    }

    #[test]
    fn regression_issue_2604() {
        let mut image = vec![];
        let mut encoder = BmpEncoder::new(&mut image);
        encoder
            .encode(&[], 1 << 31, 0, ExtendedColorType::Rgb8)
            .unwrap_err();
    }
}
