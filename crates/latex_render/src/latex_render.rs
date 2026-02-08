use anyhow::Result;
use collections::HashMap;
use gpui::{BackgroundExecutor, RenderImage};
use image::Frame;
use parking_lot::Mutex;
use regex::Regex;
use smallvec::SmallVec;
use std::sync::Arc;

/// The bundled Latin Modern Math font for rendering LaTeX.
const MATH_FONT_DATA: &[u8] = include_bytes!("../fonts/LatinModernMath.otf");

/// Render scale factor for high-DPI quality (higher = better quality but more memory).
const RENDER_SCALE: f64 = 8.0;

/// Fixed-point multiplier for font size hashing.
const FONT_SIZE_HASH_MULTIPLIER: f64 = 100.0;

/// RGBA color for rendering.
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
    font_size: u32, // Stored as fixed-point to allow hashing
    color: LatexColor,
    display: bool, // true for display style -> $$ , false for inline/text style -> $
}

/// Cached result of LaTeX rendering.
#[derive(Clone)]
enum CacheEntry {
    Pending,
    Ready(Result<(Arc<RenderImage>, (f64, f64)), Arc<String>>),
}

pub struct LatexRenderer {
    cache: Arc<Mutex<HashMap<CacheKey, CacheEntry>>>,
    executor: BackgroundExecutor,
}

impl LatexRenderer {
    pub fn new(executor: BackgroundExecutor) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::default())),
            executor,
        }
    }

    /// Returns `None` if the formula is still being rendered.
    /// Returns `Some(Ok((image, (width, height))))` with the rendered image and logical dimensions on success.
    /// Returns `Some(Err(...))` if rendering failed.
    ///
    /// The `display` parameter controls the rendering style:
    /// - `true` for display math
    /// - `false` for inline math
    pub fn render(
        &self,
        source: &str,
        font_size: f64,
        color: LatexColor,
        display: bool,
    ) -> Option<Result<(Arc<RenderImage>, (f64, f64)), String>> {
        // TODO: Add more sanitization as needed
        // Sanitize source: for some reason by default in Zed, prettier I believe formats underscores as \_ inside $$
        let clean_source = source.replace(r"\_", "_");

        // Remove \tag{...} commands as they are not supported by ReX
        // We need to keep adding these
        let tag_regex = Regex::new(r"\\tag\{[^}]*\}").unwrap_or_else(|_| Regex::new("").unwrap());
        let clean_source = tag_regex.replace_all(&clean_source, "").to_string();

        // Remove lines that are just whitespace (often left over after removing tag)
        let clean_source = clean_source.trim().to_string();

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
                        Some(result.clone().map_err(|e| e.as_ref().clone()))
                    }
                };
            }
        }

        {
            let mut cache = self.cache.lock();
            cache.insert(key.clone(), CacheEntry::Pending);
        }

        let cache = self.cache.clone();
        let source = clean_source;
        self.executor
            .spawn(async move {
                let result = render_latex_sync(&source, font_size, color, display);
                let entry = match result {
                    Ok(data) => CacheEntry::Ready(Ok(data)),
                    Err(e) => CacheEntry::Ready(Err(Arc::new(e.to_string()))),
                };

                {
                    let mut cache_guard = cache.lock();
                    cache_guard.insert(key, entry);
                }
            })
            .detach();

        None
    }

    pub fn clear_cache(&self) {
        self.cache.lock().clear();
    }
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
        .map_err(|e| anyhow::anyhow!("Failed to parse math font: {:?}", e))?;
    let math_font =
        TtfMathFont::new(font).map_err(|e| anyhow::anyhow!("Font lacks MATH table: {:?}", e))?;

    let parse_nodes =
        rex::parser::parse(source).map_err(|e| anyhow::anyhow!("LaTeX parse error: {:?}", e))?;

    let style = if display { Style::Display } else { Style::Text };
    let layout_builder = LayoutBuilder::new(&math_font)
        .font_size(font_size)
        .style(style);
    let layout = layout_builder
        .build()
        .layout(&parse_nodes)
        .map_err(|e| anyhow::anyhow!("LaTeX layout error: {:?}", e))?;

    let dims = layout.size();
    let logical_width = dims.width;
    let logical_height = dims.height;
    let mut backend = TinySkiaBackend::from_dims(dims, RENDER_SCALE)
        .ok_or_else(|| anyhow::anyhow!("Failed to create render backend"))?;

    backend.set_color(tiny_skia::Color::from_rgba8(
        color.r, color.g, color.b, color.a,
    ));

    let renderer = Renderer::new();
    renderer.render(&layout, &mut backend);

    let pixmap = backend.pixmap();
    let width = pixmap.width();
    let height = pixmap.height();

    // tiny_skia uses premultiplied RGBA, convert to BGRA for GPUI
    let mut pixels = pixmap.take();
    for pixel in pixels.chunks_exact_mut(4) {
        pixel.swap(0, 2); // RGBA -> BGRA
    }

    let buffer = image::RgbaImage::from_raw(width, height, pixels)
        .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;

    let frame = Frame::new(buffer);
    let render_image = RenderImage::new(SmallVec::from_elem(frame, 1));

    Ok((Arc::new(render_image), (logical_width, logical_height)))
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
}
