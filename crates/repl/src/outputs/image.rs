use anyhow::Result;
use base64::{
    Engine as _, alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};
use futures::{FutureExt as _, select_biased};
use gpui::{
    App, ClipboardItem, DevicePixels, Empty, Image, ImageFormat, ParsedSvg, Pixels, RenderImage,
    SharedString, Size, SvgSize, Task, Window, img, size,
};
use settings::Settings as _;
use std::sync::Arc;
use std::time::Duration;
use ui::{IntoElement, Styled, prelude::*};

use crate::outputs::{OutputContent, plain};
use crate::repl_settings::ReplSettings;

/// ImageView renders an image inline in an editor, adapting to the line height to fit the image.
pub struct ImageView {
    clipboard_image: Arc<Image>,
    source: ImageSource,
    task: Option<Task<()>>,
}

enum ImageSource {
    Raster {
        image: Arc<RenderImage>,
        size: Size<Pixels>,
    },
    Svg(SvgImage),
}

enum SvgImage {
    Parsing {
        /// Set once parsing has run long enough to be worth telling the user about.
        slow: bool,
    },
    Ready {
        parsed: Arc<ParsedSvg>,
        /// The raster being displayed, and the device size it was made at.
        raster: Option<(Size<DevicePixels>, Arc<RenderImage>)>,
        /// The device size of a rasterization that is currently running.
        rendering: Option<Size<DevicePixels>>,
    },
    Failed(SharedString),
}

/// Rasters are capped so that a large document cannot ask for a huge texture.
const MAX_RASTER_SIZE: f32 = 2000.;

/// How long to let an SVG parse before showing a loading state, so that the
/// common case of a small document does not flash one.
const SLOW_PARSE_THRESHOLD: Duration = Duration::from_millis(2);

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
            source: ImageSource::Raster {
                image: Arc::new(gpui_image_data),
                size: size(px(width as f32), px(height as f32)),
            },
            task: None,
        })
    }

    pub fn from_svg(svg: &str, cx: &mut Context<Self>) -> Self {
        let clipboard_image =
            Arc::new(Image::from_bytes(ImageFormat::Svg, svg.as_bytes().to_vec()));
        let svg_renderer = cx.svg_renderer();

        let task = cx.spawn({
            let clipboard_image = clipboard_image.clone();
            async move |this, cx| {
                let mut parse = cx
                    .background_spawn(
                        async move { svg_renderer.parse_svg(clipboard_image.bytes()) },
                    )
                    .fuse();
                let mut slow = cx.background_executor().timer(SLOW_PARSE_THRESHOLD).fuse();

                let parsed = select_biased! {
                    parsed = parse => parsed,
                    _ = slow => {
                        this.update(cx, |this, cx| {
                            if let ImageSource::Svg(SvgImage::Parsing { slow }) = &mut this.source {
                                *slow = true;
                                cx.notify();
                            }
                        })
                        .ok();
                        parse.await
                    }
                };

                this.update(cx, |this, cx| {
                    this.source = ImageSource::Svg(match parsed {
                        Ok(parsed) => SvgImage::Ready {
                            parsed: Arc::new(parsed),
                            raster: None,
                            rendering: None,
                        },
                        Err(error) => SvgImage::Failed(error.to_string().into()),
                    });
                    cx.notify();
                })
                .ok();
            }
        });

        ImageView {
            clipboard_image,
            source: ImageSource::Svg(SvgImage::Parsing { slow: false }),
            task: Some(task),
        }
    }

    /// Rasterizes the SVG at the size it is laid out at, so that it stays sharp
    /// instead of being scaled from a raster made at some other size.
    fn rasterize_svg(
        &mut self,
        display_size: Size<Pixels>,
        scale_factor: f32,
        cx: &mut Context<Self>,
    ) {
        let ImageSource::Svg(SvgImage::Ready {
            parsed,
            raster,
            rendering,
        }) = &mut self.source
        else {
            return;
        };

        let target = raster_size(display_size, scale_factor);
        if target.width.0 <= 0
            || target.height.0 <= 0
            || raster.as_ref().map(|(size, _)| *size) == Some(target)
            || *rendering == Some(target)
        {
            return;
        }

        *rendering = Some(target);
        let parsed = parsed.clone();
        let svg_renderer = cx.svg_renderer();

        self.task = Some(cx.spawn(async move |this, cx| {
            let rendered = cx
                .background_spawn(async move {
                    svg_renderer.render_parsed(&parsed, SvgSize::ExactSize(target))
                })
                .await;

            this.update(cx, |this, cx| {
                let ImageSource::Svg(svg) = &mut this.source else {
                    return;
                };
                match rendered {
                    Ok(image) => {
                        if let SvgImage::Ready {
                            raster, rendering, ..
                        } = svg
                        {
                            *rendering = None;
                            if let Some((_, previous)) = raster.replace((target, image)) {
                                cx.drop_image(previous, None);
                            }
                        }
                    }
                    Err(error) => *svg = SvgImage::Failed(error.to_string().into()),
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn render_pending(&self) -> AnyElement {
        match &self.source {
            ImageSource::Svg(SvgImage::Parsing { slow: false }) => Empty.into_any_element(),
            _ => div().child("Rendering SVG...").into_any_element(),
        }
    }

    fn scaled_size(
        &self,
        line_height: Pixels,
        max_width: Option<Pixels>,
        max_height: Option<Pixels>,
    ) -> Option<Size<Pixels>> {
        let intrinsic_size = self.source.size()?;

        let (mut height, mut width) =
            if f32::from(intrinsic_size.height) / f32::from(line_height) == u8::MAX as f32 {
                let height = u8::MAX as f32 * line_height;
                let width = intrinsic_size.width * (height / intrinsic_size.height);
                (height, width)
            } else {
                (intrinsic_size.height, intrinsic_size.width)
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

        Some(size(width, height))
    }
}

impl ImageSource {
    /// The intrinsic size of the image, once it is known.
    fn size(&self) -> Option<Size<Pixels>> {
        match self {
            ImageSource::Raster { size, .. } => Some(*size),
            ImageSource::Svg(SvgImage::Ready { parsed, .. }) => Some(parsed.size()),
            ImageSource::Svg(_) => None,
        }
    }

    fn image(&self) -> Option<&Arc<RenderImage>> {
        match self {
            ImageSource::Raster { image, .. } => Some(image),
            ImageSource::Svg(SvgImage::Ready { raster, .. }) => {
                raster.as_ref().map(|(_, image)| image)
            }
            ImageSource::Svg(_) => None,
        }
    }
}

/// The device size to rasterize at, capped so that a large document cannot ask
/// for a huge texture.
fn raster_size(display_size: Size<Pixels>, scale_factor: f32) -> Size<DevicePixels> {
    let longest_side = f32::from(display_size.width).max(f32::from(display_size.height));
    let mut scale = scale_factor;
    if longest_side * scale > MAX_RASTER_SIZE {
        scale = MAX_RASTER_SIZE / longest_side;
    }

    display_size.map(|side| DevicePixels((f32::from(side) * scale).round() as i32))
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

        if let ImageSource::Svg(SvgImage::Failed(error)) = &self.source {
            return div()
                .child(format!("Failed to render SVG: {error}"))
                .into_any_element();
        }

        let Some(display_size) = self.scaled_size(line_height, max_width, max_height) else {
            return self.render_pending();
        };

        self.rasterize_svg(display_size, window.scale_factor(), cx);

        match self.source.image() {
            Some(image) => img(image.clone())
                .w(display_size.width)
                .h(display_size.height)
                .into_any_element(),
            None => self.render_pending(),
        }
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

    const TEST_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="80"><rect width="120" height="80" fill="red"/></svg>"#;

    #[gpui::test]
    async fn test_image_view_parses_svg_in_the_background(cx: &mut gpui::TestAppContext) {
        let view = cx.new(|cx| ImageView::from_svg(TEST_SVG, cx));

        view.read_with(cx, |view, _| {
            assert!(matches!(
                view.source,
                ImageSource::Svg(SvgImage::Parsing { .. })
            ));
        });

        cx.run_until_parked();

        view.read_with(cx, |view, _| {
            assert_eq!(view.source.size(), Some(size(px(120.0), px(80.0))));
            assert!(view.source.image().is_none());
        });
    }

    #[gpui::test]
    async fn test_image_view_reports_invalid_svg(cx: &mut gpui::TestAppContext) {
        let view = cx.new(|cx| ImageView::from_svg("not an svg", cx));
        cx.run_until_parked();

        view.read_with(cx, |view, _| {
            assert!(matches!(view.source, ImageSource::Svg(SvgImage::Failed(_))));
        });
    }

    #[gpui::test]
    async fn test_image_view_rasterizes_svg_at_display_size(cx: &mut gpui::TestAppContext) {
        let view = cx.new(|cx| ImageView::from_svg(TEST_SVG, cx));
        cx.run_until_parked();

        view.update(cx, |view, cx| {
            view.rasterize_svg(size(px(60.0), px(40.0)), 2.0, cx)
        });
        cx.run_until_parked();

        let image = view
            .read_with(cx, |view, _| view.source.image().cloned())
            .expect("SVG should have been rasterized");
        assert_eq!(image.size(0), size(DevicePixels(120), DevicePixels(80)));

        // Laying out at the same size again reuses the raster.
        view.update(cx, |view, cx| {
            view.rasterize_svg(size(px(60.0), px(40.0)), 2.0, cx)
        });
        cx.run_until_parked();
        view.read_with(cx, |view, _| {
            assert!(Arc::ptr_eq(
                &image,
                view.source.image().expect("raster should be kept")
            ));
        });

        view.update(cx, |view, cx| {
            view.rasterize_svg(size(px(30.0), px(20.0)), 1.0, cx)
        });
        cx.run_until_parked();
        view.read_with(cx, |view, _| {
            assert_eq!(
                view.source
                    .image()
                    .expect("SVG should have been rasterized")
                    .size(0),
                size(DevicePixels(30), DevicePixels(20))
            );
        });
    }

    #[test]
    fn test_raster_size_is_capped() {
        let capped = raster_size(size(px(4000.0), px(2000.0)), 2.0);

        assert_eq!(capped.width, DevicePixels(MAX_RASTER_SIZE as i32));
        assert_eq!(capped.height, DevicePixels(MAX_RASTER_SIZE as i32 / 2));
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
        let display_size = image_view
            .scaled_size(line_height, Some(max_width), Some(max_height))
            .expect("raster images have a known size");

        assert_eq!(display_size, size(px(50.0), px(30.0)));
    }

    #[test]
    fn test_image_view_scaled_size_unbounded() {
        let encoded = encode_test_image(200, 120);
        let image_view = match ImageView::from(&encoded) {
            Ok(view) => view,
            Err(error) => panic!("failed to decode image view: {error}"),
        };

        let line_height = Pixels::from(10.0);
        let display_size = image_view
            .scaled_size(line_height, None, None)
            .expect("raster images have a known size");

        assert_eq!(display_size, size(px(200.0), px(120.0)));
    }
}
