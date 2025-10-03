use anyhow::Result;
use base64::{
    Engine as _, alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};
use gpui::{App, ClipboardItem, Image, ImageFormat, RenderImage, Window, img};
use std::sync::Arc;
use ui::{IntoElement, Styled, div, prelude::*};

use crate::outputs::OutputContent;

/// ImageView renders an image inline in an editor, adapting to the line height to fit the image.
pub struct ImageView {
    clipboard_image: Arc<Image>,
    height: u32,
    width: u32,
    image: Arc<RenderImage>,
}

pub const STANDARD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new()
        .with_encode_padding(false)
        .with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

impl ImageView {
    pub fn from(base64_encoded_data: &str) -> Result<Self> {
        let filtered =
            base64_encoded_data.replace(&[' ', '\n', '\t', '\r', '\x0b', '\x0c'][..], "");
        let bytes = STANDARD_INDIFFERENT.decode(filtered)?;

        let format = image::guess_format(&bytes)?;

        let mut data = image::load_from_memory_with_format(&bytes, format)?.into_rgba8();

        // Convert from RGBA to BGRA.
        for pixel in data.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }

        let height = data.height();
        let width = data.width();

        let gpui_image_data = RenderImage::new(vec![image::Frame::new(data)]);

        let format = match format {
            image::ImageFormat::Png => ImageFormat::Png,
            image::ImageFormat::Jpeg => ImageFormat::Jpeg,
            image::ImageFormat::Gif => ImageFormat::Gif,
            image::ImageFormat::WebP => ImageFormat::Webp,
            image::ImageFormat::Tiff => ImageFormat::Tiff,
            image::ImageFormat::Bmp => ImageFormat::Bmp,
            format => {
                anyhow::bail!("unsupported image format {format:?}");
            }
        };

        // Convert back to a GPUI image for use with the clipboard
        let clipboard_image = Arc::new(Image::from_bytes(format, bytes));

        Ok(ImageView {
            clipboard_image,
            height,
            width,
            image: Arc::new(gpui_image_data),
        })
    }
}

impl Render for ImageView {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let line_height = window.line_height();

        let (height, width) = if self.height as f32 / f32::from(line_height) == u8::MAX as f32 {
            let height = u8::MAX as f32 * line_height;
            let width = self.width as f32 * height / self.height as f32;
            (height, width)
        } else {
            (self.height.into(), self.width.into())
        };

        let image = self.image.clone();

        div().h(height).w(width).child(img(image))
    }
}

impl OutputContent for ImageView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_image(self.clipboard_image.as_ref()))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }
}
