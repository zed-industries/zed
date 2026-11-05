use anyhow::{Context as _, Result, anyhow, ensure};
use gpui::{Bounds, DevicePixels};
use std::cell::RefCell;
use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::{OffscreenCanvas, OffscreenCanvasRenderingContext2d};

const MAX_RASTER_DIMENSION: u32 = 4096;
const MAX_RASTER_PIXELS: usize = 4 * 1024 * 1024;

thread_local! {
    static CANVAS: RefCell<Option<TextCanvas>> = const { RefCell::new(None) };
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CanvasTextMetrics {
    pub(crate) advance: f32,
    pub(crate) left: f32,
    pub(crate) right: f32,
    pub(crate) ascent: f32,
    pub(crate) descent: f32,
}

struct TextCanvas {
    canvas: OffscreenCanvas,
    context: OffscreenCanvasRenderingContext2d,
}

impl TextCanvas {
    fn new() -> Result<Self> {
        let canvas = OffscreenCanvas::new(1, 1)
            .map_err(|error| anyhow!("creating text OffscreenCanvas: {error:?}"))?;
        let options = js_sys::Object::new();
        // Every rasterized glyph is read back for upload into GPUI's atlas.
        let assigned = js_sys::Reflect::set(
            &options,
            &JsValue::from_str("willReadFrequently"),
            &JsValue::TRUE,
        )
        .map_err(|error| anyhow!("setting Canvas readback option: {error:?}"))?;
        ensure!(assigned, "Canvas readback option could not be set");
        let context = canvas
            .get_context_with_context_options("2d", &options)
            .map_err(|error| anyhow!("getting OffscreenCanvas 2D context: {error:?}"))?
            .context("OffscreenCanvas 2D text rendering is unavailable")?
            .dyn_into::<OffscreenCanvasRenderingContext2d>()
            .map_err(|error| anyhow!("unexpected OffscreenCanvas 2D context: {error:?}"))?;
        Ok(Self { canvas, context })
    }

    fn configure(&self, css_font: &str) -> Result<()> {
        ensure!(!css_font.trim().is_empty(), "Canvas text font is empty");
        // An invalid CSS font assignment is ignored by Canvas. Reset first so
        // it cannot accidentally inherit the preceding request's font.
        self.context.set_font("10px sans-serif");
        self.context.set_font(css_font);
        self.context.set_text_align("left");
        self.context.set_text_baseline("alphabetic");
        self.context.set_fill_style_str("white");
        // web-sys does not expose direction on the offscreen 2D context.
        let assigned = js_sys::Reflect::set(
            self.context.as_ref(),
            &JsValue::from_str("direction"),
            &JsValue::from_str("ltr"),
        )
        .map_err(|error| anyhow!("setting Canvas text direction: {error:?}"))?;
        ensure!(assigned, "Canvas text direction could not be set");
        Ok(())
    }
}

fn with_canvas<T>(operation: impl FnOnce(&mut TextCanvas) -> Result<T>) -> Result<T> {
    CANVAS.with(|canvas| {
        let mut canvas = canvas
            .try_borrow_mut()
            .context("Canvas text renderer was called reentrantly")?;
        if canvas.is_none() {
            *canvas = Some(TextCanvas::new()?);
        }
        operation(
            canvas
                .as_mut()
                .context("Canvas text renderer is unavailable")?,
        )
    })
}

pub(crate) fn measure(text: &str, css_font: &str) -> Result<CanvasTextMetrics> {
    with_canvas(|canvas| {
        canvas.configure(css_font)?;
        let metrics = canvas
            .context
            .measure_text(text)
            .map_err(|error| anyhow!("measuring Canvas text: {error:?}"))?;
        let metrics = CanvasTextMetrics {
            advance: metrics.width() as f32,
            left: -metrics.actual_bounding_box_left() as f32,
            right: metrics.actual_bounding_box_right() as f32,
            ascent: metrics.actual_bounding_box_ascent() as f32,
            descent: metrics.actual_bounding_box_descent() as f32,
        };
        ensure!(
            [
                metrics.advance,
                metrics.left,
                metrics.right,
                metrics.ascent,
                metrics.descent
            ]
            .into_iter()
            .all(f32::is_finite),
            "Canvas returned non-finite text metrics"
        );
        Ok(metrics)
    })
}

pub(crate) fn rasterize(
    text: &str,
    css_font: &str,
    bounds: Bounds<DevicePixels>,
    subpixel_offset: (f32, f32),
    color: bool,
) -> Result<Vec<u8>> {
    ensure!(
        subpixel_offset.0.is_finite() && subpixel_offset.1.is_finite(),
        "Canvas text subpixel offset must be finite"
    );
    let width = u32::try_from(bounds.size.width.0)
        .context("Canvas text raster width must be nonnegative")?;
    let height = u32::try_from(bounds.size.height.0)
        .context("Canvas text raster height must be nonnegative")?;
    if width == 0 || height == 0 {
        return Ok(Vec::new());
    }
    ensure!(
        width <= MAX_RASTER_DIMENSION && height <= MAX_RASTER_DIMENSION,
        "Canvas text raster exceeds maximum dimension {MAX_RASTER_DIMENSION}: {width}x{height}"
    );
    let pixel_count = usize::try_from(width)?
        .checked_mul(usize::try_from(height)?)
        .context("Canvas text raster pixel count overflow")?;
    ensure!(
        pixel_count <= MAX_RASTER_PIXELS,
        "Canvas text raster exceeds maximum pixel count {MAX_RASTER_PIXELS}: {width}x{height}"
    );
    let byte_count = pixel_count
        .checked_mul(4)
        .context("Canvas text raster byte count overflow")?;

    with_canvas(|canvas| {
        // Shrink before growing so intermediate canvas sizes obey the pixel cap.
        if height < canvas.canvas.height() {
            canvas.canvas.set_height(height);
        }
        if canvas.canvas.width() != width {
            canvas.canvas.set_width(width);
        }
        if canvas.canvas.height() != height {
            canvas.canvas.set_height(height);
        }
        canvas.configure(css_font)?;
        canvas
            .context
            .clear_rect(0.0, 0.0, f64::from(width), f64::from(height));
        canvas
            .context
            .fill_text(
                text,
                -f64::from(bounds.origin.x.0) + f64::from(subpixel_offset.0),
                -f64::from(bounds.origin.y.0) + f64::from(subpixel_offset.1),
            )
            .map_err(|error| anyhow!("drawing Canvas text: {error:?}"))?;
        let image = canvas
            .context
            .get_image_data(0.0, 0.0, f64::from(width), f64::from(height))
            .map_err(|error| anyhow!("reading Canvas text raster pixels: {error:?}"))?;
        let mut pixels = image.data().0;
        ensure!(
            pixels.len() == byte_count,
            "Canvas text raster returned {} bytes, expected {byte_count}",
            pixels.len()
        );
        if color {
            for pixel in pixels.chunks_exact_mut(4) {
                pixel.swap(0, 2);
            }
            Ok(pixels)
        } else {
            let mut alpha = Vec::new();
            alpha
                .try_reserve_exact(pixel_count)
                .context("allocating Canvas text alpha mask")?;
            alpha.extend(pixels.chunks_exact(4).map(|pixel| pixel[3]));
            Ok(alpha)
        }
    })
}
