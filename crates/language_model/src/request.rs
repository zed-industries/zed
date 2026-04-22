use std::io::{Cursor, Write};
use std::sync::Arc;

use anyhow::Result;
use base64::write::EncoderWriter;
use gpui::{
    App, AppContext as _, DevicePixels, Image, ImageFormat, ObjectFit, Size, Task, point, px, size,
};
use image::GenericImageView as _;
use image::codecs::png::PngEncoder;
use util::ResultExt;

use language_model_core::{ImageSize, LanguageModelImage};

/// Anthropic wants uploaded images to be smaller than this in both dimensions.
const ANTHROPIC_SIZE_LIMIT: f32 = 1568.;

/// Default per-image hard limit (in bytes) for the encoded image payload we send upstream.
///
/// NOTE: `LanguageModelImage.source` is base64-encoded PNG bytes (without the `data:` prefix).
/// This limit is enforced on the encoded PNG bytes *before* base64 encoding.
const DEFAULT_IMAGE_MAX_BYTES: usize = 5 * 1024 * 1024;

/// Conservative cap on how many times we'll attempt to shrink/re-encode an image to fit
/// `DEFAULT_IMAGE_MAX_BYTES`.
const MAX_IMAGE_DOWNSCALE_PASSES: usize = 8;

/// Extension trait for `LanguageModelImage` that provides GPUI-dependent functionality.
pub trait LanguageModelImageExt {
    const FORMAT: ImageFormat;
    fn from_image(data: Arc<Image>, cx: &mut App) -> Task<Option<LanguageModelImage>>;
}

impl LanguageModelImageExt for LanguageModelImage {
    const FORMAT: ImageFormat = ImageFormat::Png;

    fn from_image(data: Arc<Image>, cx: &mut App) -> Task<Option<LanguageModelImage>> {
        cx.background_spawn(async move {
            let image_bytes = Cursor::new(data.bytes());
            let dynamic_image = match data.format() {
                ImageFormat::Png => image::codecs::png::PngDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Jpeg => image::codecs::jpeg::JpegDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Webp => image::codecs::webp::WebPDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Gif => image::codecs::gif::GifDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Bmp => image::codecs::bmp::BmpDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Tiff => image::codecs::tiff::TiffDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                _ => return None,
            }
            .log_err()?;

            let width = dynamic_image.width();
            let height = dynamic_image.height();
            let image_size = size(DevicePixels(width as i32), DevicePixels(height as i32));

            // First apply any provider-specific dimension constraints we know about (Anthropic).
            let mut processed_image = if image_size.width.0 > ANTHROPIC_SIZE_LIMIT as i32
                || image_size.height.0 > ANTHROPIC_SIZE_LIMIT as i32
            {
                let new_bounds = ObjectFit::ScaleDown.get_bounds(
                    gpui::Bounds {
                        origin: point(px(0.0), px(0.0)),
                        size: size(px(ANTHROPIC_SIZE_LIMIT), px(ANTHROPIC_SIZE_LIMIT)),
                    },
                    image_size,
                );
                dynamic_image.resize(
                    new_bounds.size.width.into(),
                    new_bounds.size.height.into(),
                    image::imageops::FilterType::Triangle,
                )
            } else {
                dynamic_image
            };

            // Then enforce a default per-image size cap on the encoded PNG bytes.
            //
            // We always send PNG bytes (either original PNG bytes, or re-encoded PNG) base64'd.
            // The upstream provider limit we want to respect is effectively on the binary image
            // payload size, so we enforce against the encoded PNG bytes before base64 encoding.
            let mut encoded_png = encode_png_bytes(&processed_image).log_err()?;
            for _pass in 0..MAX_IMAGE_DOWNSCALE_PASSES {
                if encoded_png.len() <= DEFAULT_IMAGE_MAX_BYTES {
                    break;
                }

                // Scale down geometrically to converge quickly. We don't know the final PNG size
                // as a function of pixels, so we iteratively shrink.
                let (w, h) = processed_image.dimensions();
                if w <= 1 || h <= 1 {
                    break;
                }

                // Shrink by ~15% each pass (0.85). This is a compromise between speed and
                // preserving image detail.
                let new_w = ((w as f32) * 0.85).round().max(1.0) as u32;
                let new_h = ((h as f32) * 0.85).round().max(1.0) as u32;

                processed_image =
                    processed_image.resize(new_w, new_h, image::imageops::FilterType::Triangle);
                encoded_png = encode_png_bytes(&processed_image).log_err()?;
            }

            if encoded_png.len() > DEFAULT_IMAGE_MAX_BYTES {
                // Still too large after multiple passes; treat as non-convertible for now.
                // (Provider-specific handling can be introduced later.)
                return None;
            }

            // Now base64 encode the PNG bytes.
            let base64_image = encode_bytes_as_base64(encoded_png.as_slice()).log_err()?;

            // SAFETY: The base64 encoder should not produce non-UTF8.
            let source = unsafe { String::from_utf8_unchecked(base64_image) };

            let (final_width, final_height) = processed_image.dimensions();

            Some(LanguageModelImage {
                size: Some(ImageSize {
                    width: final_width as i32,
                    height: final_height as i32,
                }),
                source: source.into(),
            })
        })
    }
}

fn encode_png_bytes(image: &image::DynamicImage) -> Result<Vec<u8>> {
    let mut png = Vec::new();
    image.write_with_encoder(PngEncoder::new(&mut png))?;
    Ok(png)
}

fn encode_bytes_as_base64(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut base64_image = Vec::new();
    {
        let mut base64_encoder = EncoderWriter::new(
            Cursor::new(&mut base64_image),
            &base64::engine::general_purpose::STANDARD,
        );
        base64_encoder.write_all(bytes)?;
    }
    Ok(base64_image)
}

/// Convert a core `ImageSize` to a gpui `Size<DevicePixels>`.
pub fn image_size_to_gpui(size: ImageSize) -> Size<DevicePixels> {
    Size {
        width: DevicePixels(size.width),
        height: DevicePixels(size.height),
    }
}

/// Convert a gpui `Size<DevicePixels>` to a core `ImageSize`.
pub fn gpui_size_to_image_size(size: Size<DevicePixels>) -> ImageSize {
    ImageSize {
        width: size.width.0,
        height: size.height.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine as _;
    use gpui::TestAppContext;

    fn base64_to_png_bytes(base64: &str) -> Vec<u8> {
        base64::engine::general_purpose::STANDARD
            .decode(base64)
            .expect("valid base64")
    }

    fn png_dimensions(png_bytes: &[u8]) -> (u32, u32) {
        let img = image::load_from_memory(png_bytes).expect("valid png");
        (img.width(), img.height())
    }

    fn make_noisy_png_bytes(width: u32, height: u32) -> Vec<u8> {
        use image::{ImageBuffer, Rgba};
        use std::hash::{Hash, Hasher};

        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let mut hasher = std::hash::DefaultHasher::new();
            (x, y, width, height).hash(&mut hasher);
            let h = hasher.finish();
            Rgba([h as u8, (h >> 8) as u8, (h >> 16) as u8, 255])
        });

        let mut buf = Cursor::new(Vec::new());
        img.write_with_encoder(PngEncoder::new(&mut buf))
            .expect("encode");
        buf.into_inner()
    }

    #[gpui::test]
    async fn test_from_image_downscales_to_default_5mb_limit(cx: &mut TestAppContext) {
        let raw_png = make_noisy_png_bytes(4096, 4096);
        assert!(
            raw_png.len() > DEFAULT_IMAGE_MAX_BYTES,
            "Test image should exceed the 5 MB limit (actual: {} bytes)",
            raw_png.len()
        );

        let image = Arc::new(gpui::Image::from_bytes(ImageFormat::Png, raw_png));
        let lm_image = cx
            .update(|cx| LanguageModelImage::from_image(Arc::clone(&image), cx))
            .await
            .expect("from_image should succeed");

        let decoded_png = base64_to_png_bytes(lm_image.source.as_ref());
        assert!(
            decoded_png.len() <= DEFAULT_IMAGE_MAX_BYTES,
            "Encoded PNG should be ≤ {} bytes after downscale, but was {} bytes",
            DEFAULT_IMAGE_MAX_BYTES,
            decoded_png.len()
        );

        let (w, h) = png_dimensions(&decoded_png);
        assert!(
            w < 4096 && h < 4096,
            "Dimensions should have shrunk: got {}×{}",
            w,
            h
        );

        let size = lm_image.size.expect("ImageSize should be present");
        assert_eq!(
            size.width, w as i32,
            "ImageSize.width should match the encoded PNG width after downscaling"
        );
        assert_eq!(
            size.height, h as i32,
            "ImageSize.height should match the encoded PNG height after downscaling"
        );
    }
}
