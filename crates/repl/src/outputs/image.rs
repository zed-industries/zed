use anyhow::Result;
use base64::{
    Engine as _, alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};
use gpui::{App, ClipboardItem, Image, ImageFormat, Pixels, RenderImage, Window, img};
use settings::Settings as _;
use std::sync::Arc;
use ui::{IntoElement, Styled, prelude::*};

use crate::outputs::{OutputContent, plain};
use crate::repl_settings::ReplSettings;

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
            image::ImageFormat::Ico => ImageFormat::Ico,
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

    fn scaled_size(
        &self,
        line_height: Pixels,
        max_width: Option<Pixels>,
        max_height: Option<Pixels>,
    ) -> (Pixels, Pixels) {
        let (mut height, mut width) = if self.height as f32 / f32::from(line_height)
            == u8::MAX as f32
        {
            let height = u8::MAX as f32 * line_height;
            let width = Pixels::from(self.width as f32 * f32::from(height) / self.height as f32);
            (height, width)
        } else {
            (self.height.into(), self.width.into())
        };

        let mut scale: f32 = 1.0;
        if let Some(max_width) = max_width {
            if width > max_width {
                scale = scale.min(max_width / width);
            }
        }

        if let Some(max_height) = max_height {
            if height > max_height {
                scale = scale.min(max_height / height);
            }
        }

        if scale < 1.0 {
            width *= scale;
            height *= scale;
        }

        (height, width)
    }
}

impl Render for ImageView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ReplSettings::get_global(cx);
        let line_height = window.line_height();

        let max_width = plain::max_width_for_columns(settings.max_columns, window, cx);

        let max_height = if settings.output_max_height_lines > 0 {
            Some(line_height * settings.output_max_height_lines as f32)
        } else {
            None
        };

        let (height, width) = self.scaled_size(line_height, max_width, max_height);

        let image = self.image.clone();

        // `img` turns an auto height into the image's natural height, so the
        // aspect ratio lives on a wrapper that can shrink with the output area.
        div()
            .w_full()
            .max_w(width)
            .aspect_ratio(f32::from(width) / f32::from(height))
            .child(img(image).size_full())
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AvailableSpace, TestAppContext, VisualTestContext, point, px, size};
    use settings::SettingsStore;

    fn encode_test_image(width: u32, height: u32) -> String {
        let image_buffer =
            image::ImageBuffer::from_pixel(width, height, image::Rgba([0, 0, 0, 255]));
        let image = image::DynamicImage::ImageRgba8(image_buffer);

        let mut bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut bytes);
        if let Err(error) = image.write_to(&mut cursor, image::ImageFormat::Png) {
            panic!("failed to encode test image: {error}");
        }

        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    #[test]
    fn test_image_view_scaled_size_respects_limits() {
        let encoded = encode_test_image(200, 120);
        let image_view = match ImageView::from(&encoded) {
            Ok(view) => view,
            Err(error) => panic!("failed to decode image view: {error}"),
        };

        let line_height = Pixels::from(10.0);
        let max_width = Pixels::from(50.0);
        let max_height = Pixels::from(40.0);
        let (height, width) =
            image_view.scaled_size(line_height, Some(max_width), Some(max_height));

        assert_eq!(f32::from(width), 50.0);
        assert_eq!(f32::from(height), 30.0);
    }

    #[test]
    fn test_image_view_scaled_size_unbounded() {
        let encoded = encode_test_image(200, 120);
        let image_view = match ImageView::from(&encoded) {
            Ok(view) => view,
            Err(error) => panic!("failed to decode image view: {error}"),
        };

        let line_height = Pixels::from(10.0);
        let (height, width) = image_view.scaled_size(line_height, None, None);

        assert_eq!(f32::from(width), 200.0);
        assert_eq!(f32::from(height), 120.0);
    }

    fn laid_out_height(
        image_width: u32,
        image_height: u32,
        container_width: f32,
        cx: &mut TestAppContext,
    ) -> f32 {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let cx: &mut VisualTestContext = cx.add_empty_window();
        let encoded = encode_test_image(image_width, image_height);
        let image_view = cx.update(|_, cx| {
            cx.new(|_| match ImageView::from(&encoded) {
                Ok(view) => view,
                Err(error) => panic!("failed to decode image view: {error}"),
            })
        });
        cx.draw(
            point(px(0.), px(0.)),
            size(
                AvailableSpace::Definite(px(container_width)),
                AvailableSpace::MinContent,
            ),
            |_, _| {
                div()
                    .w(px(container_width))
                    .debug_selector(|| "image-output".into())
                    .child(image_view)
            },
        );
        match cx.debug_bounds("image-output") {
            Some(bounds) => f32::from(bounds.size.height),
            None => panic!("image output was not laid out"),
        }
    }

    #[gpui::test]
    fn test_image_view_shrinks_to_fit_narrow_output(cx: &mut TestAppContext) {
        assert_eq!(laid_out_height(800, 400, 300.0, cx), 150.0);
    }

    #[gpui::test]
    fn test_image_view_does_not_upscale_small_images(cx: &mut TestAppContext) {
        assert_eq!(laid_out_height(100, 50, 300.0, cx), 50.0);
    }
}
