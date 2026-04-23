use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{BackgroundExecutor, RenderImage};
use image::Frame;
use parking_lot::Mutex;
use regex::Regex;
use smallvec::SmallVec;
use std::sync::{Arc, LazyLock};

const MATH_FONT_DATA: &[u8] = include_bytes!("../fonts/LatinModernMath.otf");

// Rasterize at 1:1 and rely on tiny_skia's built-in antialiasing. A higher
// supersample factor would force GPUI to downscale the resulting pixmap to
// the displayed logical size, which produced visible glyph-edge tearing.
const RENDER_SCALE: f64 = 1.0;

/// Fixed-point multiplier used to hash the floating-point font size.
const FONT_SIZE_HASH_MULTIPLIER: f64 = 100.0;

// Prettier re-formats `_` inside math as `\_`; ReX does not accept that escape.
static UNDERSCORE_ESCAPE: &str = r"\_";

// ReX does not support `\tag{...}`; strip it before layout instead of failing.
static TAG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\tag\{[^}]*\}").expect("valid tag regex literal"));

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LatexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl LatexColor {
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CacheKey {
    source: String,
    font_size: u32,
    color: LatexColor,
    display: bool,
}

#[derive(Clone)]
enum CacheEntry {
    Pending,
    Ready(Result<(Arc<RenderImage>, (f64, f64)), Arc<String>>),
}

/// A callback invoked from a background thread when a previously pending
/// formula transitions to `Ready`. Typically used to notify a UI entity to
/// re-render.
pub type OnRenderComplete = Arc<dyn Fn() + Send + Sync>;

pub struct LatexRenderer {
    cache: Arc<Mutex<HashMap<CacheKey, CacheEntry>>>,
    executor: BackgroundExecutor,
    on_render_complete: Option<OnRenderComplete>,
}

impl LatexRenderer {
    pub fn new(executor: BackgroundExecutor) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::default())),
            executor,
            on_render_complete: None,
        }
    }

    pub fn with_on_render_complete(mut self, callback: OnRenderComplete) -> Self {
        self.on_render_complete = Some(callback);
        self
    }

    /// Returns `None` while the formula is still being rendered in the
    /// background. Returns `Some(Ok((image, (width, height))))` on success
    /// with the rendered image and its logical dimensions. Returns
    /// `Some(Err(...))` if rendering failed.
    ///
    /// `display = true` produces display-style math (`$$...$$`);
    /// `display = false` produces inline/text-style math (`$...$`).
    pub fn render(
        &self,
        source: &str,
        font_size: f64,
        color: LatexColor,
        display: bool,
    ) -> Option<Result<(Arc<RenderImage>, (f64, f64)), String>> {
        let clean_source = sanitize_source(source);

        let key = CacheKey {
            source: clean_source.clone(),
            font_size: (font_size * FONT_SIZE_HASH_MULTIPLIER) as u32,
            color,
            display,
        };

        {
            let cache = self.cache.lock();
            if let Some(entry) = cache.get(&key) {
                return match entry {
                    CacheEntry::Pending => None,
                    CacheEntry::Ready(result) => {
                        Some(result.clone().map_err(|error| error.as_ref().clone()))
                    }
                };
            }
        }

        {
            let mut cache = self.cache.lock();
            cache.insert(key.clone(), CacheEntry::Pending);
        }

        self.executor
            .spawn({
                let cache = self.cache.clone();
                let on_render_complete = self.on_render_complete.clone();
                let source = clean_source;
                async move {
                    let result = render_latex_sync(&source, font_size, color, display);
                    let entry = match result {
                        Ok(data) => CacheEntry::Ready(Ok(data)),
                        Err(error) => CacheEntry::Ready(Err(Arc::new(error.to_string()))),
                    };

                    {
                        let mut cache_guard = cache.lock();
                        cache_guard.insert(key, entry);
                    }

                    if let Some(callback) = on_render_complete {
                        callback();
                    }
                }
            })
            .detach();

        None
    }

    pub fn clear_cache(&self) {
        self.cache.lock().clear();
    }
}

fn sanitize_source(source: &str) -> String {
    let replaced = source.replace(UNDERSCORE_ESCAPE, "_");
    TAG_REGEX.replace_all(&replaced, "").trim().to_string()
}

fn render_latex_sync(
    source: &str,
    font_size: f64,
    color: LatexColor,
    display: bool,
) -> Result<(Arc<RenderImage>, (f64, f64))> {
    use rex::Renderer;
    use rex::font::backend::ttf_parser::TtfMathFont;
    use rex::layout::Style;
    use rex::layout::engine::LayoutBuilder;
    use rex::render::tinyskia::TinySkiaBackend;

    let font = ttf_parser::Face::parse(MATH_FONT_DATA, 0)
        .map_err(|error| anyhow::anyhow!("Failed to parse math font: {error:?}"))?;
    let math_font = TtfMathFont::new(font)
        .map_err(|error| anyhow::anyhow!("Font lacks MATH table: {error:?}"))?;

    let parse_nodes = rex::parser::parse(source)
        .map_err(|error| anyhow::anyhow!("LaTeX parse error: {error:?}"))?;

    let style = if display { Style::Display } else { Style::Text };
    let layout = LayoutBuilder::new(&math_font)
        .font_size(font_size)
        .style(style)
        .build()
        .layout(&parse_nodes)
        .map_err(|error| anyhow::anyhow!("LaTeX layout error: {error:?}"))?;

    let dims = layout.size();
    let mut backend = TinySkiaBackend::from_dims(dims, RENDER_SCALE)
        .context("Failed to create render backend")?;

    backend.set_color(tiny_skia::Color::from_rgba8(
        color.r, color.g, color.b, color.a,
    ));

    Renderer::new().render(&layout, &mut backend);

    let pixmap = backend.pixmap();
    let width = pixmap.width();
    let height = pixmap.height();

    // tiny_skia outputs premultiplied RGBA. GPUI's image pipeline expects
    // straight (unassociated) BGRA, so we swap R↔B and divide color channels
    // by alpha. Skipping the unpremultiply leaves dark halos around glyph
    // edges that read as "font tearing".
    let mut pixels = pixmap.take();
    for pixel in pixels.chunks_exact_mut(4) {
        pixel.swap(0, 2);
        let alpha = pixel[3];
        if alpha > 0 && alpha < 255 {
            let alpha_f = alpha as f32 / 255.0;
            pixel[0] = ((pixel[0] as f32 / alpha_f).min(255.0)) as u8;
            pixel[1] = ((pixel[1] as f32 / alpha_f).min(255.0)) as u8;
            pixel[2] = ((pixel[2] as f32 / alpha_f).min(255.0)) as u8;
        }
    }

    let buffer = image::RgbaImage::from_raw(width, height, pixels)
        .context("Failed to create image buffer")?;

    let frame = Frame::new(buffer);
    let render_image = RenderImage::new(SmallVec::from_elem(frame, 1));

    // Report the pixmap's native pixel dimensions as the logical display size.
    // This lets GPUI render 1:1 instead of resampling a supersampled image
    // down to a smaller logical box, which was producing visible edge tearing.
    let display_width = width as f64 / RENDER_SCALE;
    let display_height = height as f64 / RENDER_SCALE;
    Ok((Arc::new(render_image), (display_width, display_height)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple_latex() {
        let result = render_latex_sync("x^2", 16.0, LatexColor::BLACK, true);
        assert!(
            result.is_ok(),
            "Failed to render simple LaTeX: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_render_fraction() {
        let result = render_latex_sync(r"\frac{a}{b}", 16.0, LatexColor::BLACK, true);
        assert!(
            result.is_ok(),
            "Failed to render fraction: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_render_integral() {
        let result = render_latex_sync(r"\int_0^1 x\,dx", 16.0, LatexColor::BLACK, true);
        assert!(
            result.is_ok(),
            "Failed to render integral: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_sanitize_source_strips_tag_and_underscore() {
        assert_eq!(
            sanitize_source(r"x\_{1,2} = \frac{-b}{2a} \tag{1}"),
            r"x_{1,2} = \frac{-b}{2a}".to_string()
        );
    }
}
