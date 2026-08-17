#![forbid(unsafe_code)]

use crate::render::{HeadlessError, LayoutOptions, SvgPipeline, SvgRenderOptions};
use merman_core::{Engine, ParseOptions};

#[derive(Debug, thiserror::Error)]
pub enum RasterError {
    #[error(transparent)]
    Headless(#[from] HeadlessError),
    #[error("failed to parse SVG")]
    SvgParse,
    #[error("failed to allocate pixmap for raster rendering")]
    PixmapAlloc,
    #[error("invalid raster scale; expected a finite positive number")]
    InvalidScale,
    #[error("invalid raster sizing option: {0}")]
    InvalidSizing(&'static str),
    #[error("failed to encode PNG")]
    PngEncode,
    #[error("invalid background color for JPG rendering")]
    JpegBackground,
    #[error("JPG rendering requires an opaque background color (e.g. white)")]
    JpegOpaqueBackgroundRequired,
    #[error("failed to encode JPG")]
    JpegEncode,
    #[error("failed to convert SVG to PDF")]
    PdfConvert,
}

pub type Result<T> = std::result::Result<T, RasterError>;

pub const DEFAULT_MAX_RASTER_SIDE_LENGTH: u32 = 8192;
pub const DEFAULT_MAX_RASTER_PIXELS: u64 =
    (DEFAULT_MAX_RASTER_SIDE_LENGTH as u64) * (DEFAULT_MAX_RASTER_SIDE_LENGTH as u64);

/// Optional display box for target-aware rasterization.
///
/// Browser previews typically draw Mermaid SVG as vector content inside a container. A headless
/// rasterizer has to allocate a full pixmap, so UI hosts should pass the visible container size
/// here and use [`RasterOptions::scale`] for device-pixel ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RasterFitBox {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl RasterFitBox {
    pub const fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self { width, height }
    }

    pub const fn width(width: u32) -> Self {
        Self {
            width: Some(width),
            height: None,
        }
    }

    pub const fn height(height: u32) -> Self {
        Self {
            width: None,
            height: Some(height),
        }
    }

    pub const fn contain(width: u32, height: u32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
        }
    }
}

/// Resource budget applied before allocating the output pixmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RasterSizeLimit {
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub max_pixels: Option<u64>,
}

impl RasterSizeLimit {
    pub const fn new(
        max_width: Option<u32>,
        max_height: Option<u32>,
        max_pixels: Option<u64>,
    ) -> Self {
        Self {
            max_width,
            max_height,
            max_pixels,
        }
    }

    pub const fn max_side_length(max_side_length: u32) -> Self {
        Self {
            max_width: Some(max_side_length),
            max_height: Some(max_side_length),
            max_pixels: None,
        }
    }

    pub const fn default_safe() -> Self {
        Self {
            max_width: Some(DEFAULT_MAX_RASTER_SIDE_LENGTH),
            max_height: Some(DEFAULT_MAX_RASTER_SIDE_LENGTH),
            max_pixels: Some(DEFAULT_MAX_RASTER_PIXELS),
        }
    }

    pub const fn unbounded() -> Self {
        Self {
            max_width: None,
            max_height: None,
            max_pixels: None,
        }
    }
}

impl Default for RasterSizeLimit {
    fn default() -> Self {
        Self::default_safe()
    }
}

#[derive(Debug, Clone)]
pub struct RasterOptions {
    pub scale: f32,
    pub background: Option<String>,
    pub jpeg_quality: u8,
    pub fit_to: Option<RasterFitBox>,
    pub size_limit: RasterSizeLimit,
}

impl Default for RasterOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            background: None,
            jpeg_quality: 90,
            fit_to: None,
            size_limit: RasterSizeLimit::default(),
        }
    }
}

impl RasterOptions {
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_background(mut self, background: impl Into<String>) -> Self {
        self.background = Some(background.into());
        self
    }

    pub fn with_fit_to(mut self, fit_to: RasterFitBox) -> Self {
        self.fit_to = Some(fit_to);
        self
    }

    pub fn with_size_limit(mut self, size_limit: RasterSizeLimit) -> Self {
        self.size_limit = size_limit;
        self
    }

    pub fn with_unbounded_size(mut self) -> Self {
        self.size_limit = RasterSizeLimit::unbounded();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RasterPlan {
    pub requested_width_px: u32,
    pub requested_height_px: u32,
    pub width_px: u32,
    pub height_px: u32,
    pub requested_scale: f64,
    pub effective_scale: f64,
    pub limited: bool,
}

pub fn render_png_sync(
    engine: &Engine,
    text: &str,
    parse_options: ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
    raster: &RasterOptions,
) -> Result<Option<Vec<u8>>> {
    let Some(svg) = super::render_svg_with_pipeline_sync(
        engine,
        text,
        parse_options,
        layout_options,
        svg_options,
        &SvgPipeline::resvg_safe(),
    )?
    else {
        return Ok(None);
    };
    Ok(Some(svg_to_png(&svg, raster)?))
}

pub fn render_jpeg_sync(
    engine: &Engine,
    text: &str,
    parse_options: ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
    raster: &RasterOptions,
) -> Result<Option<Vec<u8>>> {
    let Some(svg) = super::render_svg_with_pipeline_sync(
        engine,
        text,
        parse_options,
        layout_options,
        svg_options,
        &SvgPipeline::resvg_safe(),
    )?
    else {
        return Ok(None);
    };
    Ok(Some(svg_to_jpeg(&svg, raster)?))
}

pub fn render_pdf_sync(
    engine: &Engine,
    text: &str,
    parse_options: ParseOptions,
    layout_options: &LayoutOptions,
    svg_options: &SvgRenderOptions,
) -> Result<Option<Vec<u8>>> {
    let Some(svg) = super::render_svg_with_pipeline_sync(
        engine,
        text,
        parse_options,
        layout_options,
        svg_options,
        &SvgPipeline::resvg_safe(),
    )?
    else {
        return Ok(None);
    };
    Ok(Some(svg_to_pdf(&svg)?))
}

pub fn svg_to_png(svg: &str, options: &RasterOptions) -> Result<Vec<u8>> {
    let pixmap = svg_to_pixmap(svg, options)?;
    pixmap.encode_png().map_err(|_| RasterError::PngEncode)
}

pub fn svg_to_jpeg(svg: &str, options: &RasterOptions) -> Result<Vec<u8>> {
    let bg = options.background.as_deref().unwrap_or("white");
    let Some(color) = parse_tiny_skia_color(bg) else {
        return Err(RasterError::JpegBackground);
    };
    if color.alpha() != 1.0 {
        return Err(RasterError::JpegOpaqueBackgroundRequired);
    }

    let mut raster_options = options.clone();
    raster_options.background = Some(bg.to_string());
    let pixmap = svg_to_pixmap(svg, &raster_options)?;
    let (w, h) = (pixmap.width(), pixmap.height());

    // tiny-skia renders into an RGBA8 buffer. When the destination is opaque (we always fill a
    // solid background for JPG), the alpha channel is always 255 and can be dropped safely.
    let rgba = pixmap.data();
    let mut rgb = vec![0u8; (w as usize) * (h as usize) * 3];
    for (src, dst) in rgba.chunks_exact(4).zip(rgb.chunks_exact_mut(3)) {
        dst[0] = src[0];
        dst[1] = src[1];
        dst[2] = src[2];
    }

    let mut out = Vec::new();
    let mut enc =
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, options.jpeg_quality);
    enc.encode(&rgb, w, h, image::ExtendedColorType::Rgb8)
        .map_err(|_| RasterError::JpegEncode)?;
    Ok(out)
}

pub fn svg_raster_plan(svg: &str, options: &RasterOptions) -> Result<RasterPlan> {
    let mut opt = usvg::Options::default();
    configure_usvg_options_for_raster(&mut opt, svg);

    let tree = usvg::Tree::from_str(svg, &opt).map_err(|_| RasterError::SvgParse)?;
    let (geo, _) = raster_geometry_for_svg(svg, &tree);
    raster_plan_for_geometry(geo, options)
}

pub fn svg_to_pdf(svg: &str) -> Result<Vec<u8>> {
    svg_to_pdf_with_options(svg, &RasterOptions::default())
}

pub fn svg_to_pdf_with_options(svg: &str, options: &RasterOptions) -> Result<Vec<u8>> {
    validate_svg_pdf_size(svg, options)?;
    svg_to_pdf_unchecked(svg)
}

pub fn validate_svg_pdf_size(svg: &str, options: &RasterOptions) -> Result<RasterPlan> {
    let mut pdf_options = options.clone();
    pdf_options.scale = 1.0;
    pdf_options.fit_to = None;

    let plan = svg_raster_plan(svg, &pdf_options)?;
    if plan.limited {
        return Err(RasterError::InvalidSizing(
            "PDF output exceeds configured size_limit",
        ));
    }
    Ok(plan)
}

fn svg_to_pdf_unchecked(svg: &str) -> Result<Vec<u8>> {
    let mut opt = svg2pdf::usvg::Options::default();
    configure_usvg_options_for_pdf(&mut opt);

    let tree = svg2pdf::usvg::Tree::from_str(svg, &opt).map_err(|_| RasterError::SvgParse)?;

    svg2pdf::to_pdf(
        &tree,
        svg2pdf::ConversionOptions::default(),
        svg2pdf::PageOptions::default(),
    )
    .map_err(|_| RasterError::PdfConvert)
}

#[derive(Debug, Clone, Copy)]
struct ParsedViewBox {
    width: f32,
    height: f32,
}

fn parse_svg_viewbox(svg: &str) -> Option<ParsedViewBox> {
    // Cheap, non-validating parse for root viewBox: `viewBox="minX minY w h"`.
    // This is sufficient for Mermaid-like SVG output.
    let i = svg.find("viewBox=\"")?;
    let rest = &svg[i + "viewBox=\"".len()..];
    let end = rest.find('"')?;
    let raw = &rest[..end];
    let mut it = raw.split_whitespace();
    let _min_x = it.next()?.parse::<f32>().ok()?;
    let _min_y = it.next()?.parse::<f32>().ok()?;
    let width = it.next()?.parse::<f32>().ok()?;
    let height = it.next()?.parse::<f32>().ok()?;
    if width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0 {
        Some(ParsedViewBox { width, height })
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct RasterGeometry {
    min_x: f32,
    min_y: f32,
    width: f32,
    height: f32,
}

fn svg_to_pixmap(svg: &str, options: &RasterOptions) -> Result<tiny_skia::Pixmap> {
    let mut opt = usvg::Options::default();
    configure_usvg_options_for_raster(&mut opt, svg);

    let tree = usvg::Tree::from_str(svg, &opt).map_err(|_| RasterError::SvgParse)?;
    let (geo, translate_min_to_origin) = raster_geometry_for_svg(svg, &tree);
    let plan = raster_plan_for_geometry(geo, options)?;

    let mut pixmap =
        tiny_skia::Pixmap::new(plan.width_px, plan.height_px).ok_or(RasterError::PixmapAlloc)?;

    if let Some(bg) = options.background.as_deref() {
        if let Some(color) = parse_tiny_skia_color(bg) {
            pixmap.fill(color);
        }
    }

    let scale = plan.effective_scale as f32;
    let transform = if translate_min_to_origin {
        // Render at `scale`, translating so min_x/min_y map to (0,0).
        tiny_skia::Transform::from_row(
            scale,
            0.0,
            0.0,
            scale,
            -geo.min_x * scale,
            -geo.min_y * scale,
        )
    } else {
        tiny_skia::Transform::from_scale(scale, scale)
    };

    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Ok(pixmap)
}

fn raster_geometry_for_svg(svg: &str, tree: &usvg::Tree) -> (RasterGeometry, bool) {
    if let Some(vb) = parse_svg_viewbox(svg) {
        // `usvg`/`resvg` already apply the root viewBox transform (including translating the
        // viewBox min corner to (0,0)) when building/rendering the tree. If we also translate
        // by `-min_x/-min_y` here, diagrams with negative viewBox mins (e.g. kanban, gitGraph)
        // get shifted fully out of the viewport and render as a blank/transparent pixmap.
        return (
            RasterGeometry {
                min_x: 0.0,
                min_y: 0.0,
                width: vb.width,
                height: vb.height,
            },
            false,
        );
    }

    // Some Mermaid diagrams (e.g. `info`) don't emit a viewBox upstream.
    // For raster formats, fall back to the rendered content bounds as computed by usvg.
    let bbox = tree.root().abs_stroke_bounding_box();
    let w = bbox.width().max(1.0);
    let h = bbox.height().max(1.0);
    if w.is_finite() && h.is_finite() && w > 0.0 && h > 0.0 {
        (
            RasterGeometry {
                min_x: bbox.x(),
                min_y: bbox.y(),
                width: w,
                height: h,
            },
            true,
        )
    } else {
        let size = tree.size();
        (
            RasterGeometry {
                min_x: 0.0,
                min_y: 0.0,
                width: size.width(),
                height: size.height(),
            },
            false,
        )
    }
}

fn raster_plan_for_geometry(geo: RasterGeometry, options: &RasterOptions) -> Result<RasterPlan> {
    if !(options.scale.is_finite() && options.scale > 0.0) {
        return Err(RasterError::InvalidScale);
    }

    validate_fit_box(options.fit_to)?;
    validate_size_limit(options.size_limit)?;

    // Make scaling more intuitive/stable: at scale=1 we already round up to whole pixels, so for
    // scale>1 prefer scaling the *rounded* base size. This avoids surprising off-by-one shrinkage
    // when the viewBox/bounds are fractional (e.g. 342.36 * 2 = 684.72 -> ceil = 685, while
    // ceil(342.36) * 2 = 686).
    let base_width_px = f64::from(geo.width).ceil().max(1.0);
    let base_height_px = f64::from(geo.height).ceil().max(1.0);

    let fit_scale = fit_scale_for_base_size(base_width_px, base_height_px, options.fit_to);
    let requested_scale = fit_scale * f64::from(options.scale);
    let requested_width_px = raster_dim_px(base_width_px * requested_scale, None)?;
    let requested_height_px = raster_dim_px(base_height_px * requested_scale, None)?;

    let limit_scale = size_limit_scale(
        base_width_px * requested_scale,
        base_height_px * requested_scale,
        options.size_limit,
    );
    let mut effective_scale = requested_scale * limit_scale;
    let (mut width_px, mut height_px) = raster_limited_dims(
        base_width_px,
        base_height_px,
        effective_scale,
        options.size_limit,
    )?;

    if let Some(max_pixels) = options.size_limit.max_pixels {
        for _ in 0..8 {
            if u64::from(width_px) * u64::from(height_px) <= max_pixels {
                break;
            }
            let pixels = f64::from(width_px) * f64::from(height_px);
            let shrink = ((max_pixels as f64) / pixels).sqrt() * 0.999_999;
            effective_scale *= shrink;
            (width_px, height_px) = raster_limited_dims(
                base_width_px,
                base_height_px,
                effective_scale,
                options.size_limit,
            )?;
        }
    }

    Ok(RasterPlan {
        requested_width_px,
        requested_height_px,
        width_px,
        height_px,
        requested_scale,
        effective_scale,
        limited: width_px != requested_width_px || height_px != requested_height_px,
    })
}

fn validate_fit_box(fit: Option<RasterFitBox>) -> Result<()> {
    let Some(fit) = fit else {
        return Ok(());
    };

    if fit.width.is_none() && fit.height.is_none() {
        return Err(RasterError::InvalidSizing(
            "fit_to must include a positive width or height",
        ));
    }
    if fit.width == Some(0) || fit.height == Some(0) {
        return Err(RasterError::InvalidSizing(
            "fit_to width and height must be positive",
        ));
    }
    Ok(())
}

fn validate_size_limit(limit: RasterSizeLimit) -> Result<()> {
    if limit.max_width == Some(0) || limit.max_height == Some(0) {
        return Err(RasterError::InvalidSizing(
            "size_limit max_width and max_height must be positive",
        ));
    }
    if limit.max_pixels == Some(0) {
        return Err(RasterError::InvalidSizing(
            "size_limit max_pixels must be positive",
        ));
    }
    Ok(())
}

fn fit_scale_for_base_size(width: f64, height: f64, fit: Option<RasterFitBox>) -> f64 {
    let Some(fit) = fit else {
        return 1.0;
    };

    let mut scale: f64 = 1.0;
    if let Some(target_width) = fit.width {
        scale = scale.min(f64::from(target_width) / width);
    }
    if let Some(target_height) = fit.height {
        scale = scale.min(f64::from(target_height) / height);
    }
    scale.min(1.0).max(0.0)
}

fn size_limit_scale(width: f64, height: f64, limit: RasterSizeLimit) -> f64 {
    let mut scale: f64 = 1.0;
    if let Some(max_width) = limit.max_width {
        scale = scale.min(f64::from(max_width) / width);
    }
    if let Some(max_height) = limit.max_height {
        scale = scale.min(f64::from(max_height) / height);
    }
    if let Some(max_pixels) = limit.max_pixels {
        let pixels = width * height * scale * scale;
        if pixels > max_pixels as f64 {
            scale *= ((max_pixels as f64) / pixels).sqrt();
        }
    }
    scale.min(1.0).max(0.0)
}

fn raster_limited_dims(
    base_width_px: f64,
    base_height_px: f64,
    scale: f64,
    limit: RasterSizeLimit,
) -> Result<(u32, u32)> {
    Ok((
        raster_dim_px(base_width_px * scale, limit.max_width)?,
        raster_dim_px(base_height_px * scale, limit.max_height)?,
    ))
}

fn raster_dim_px(value: f64, max: Option<u32>) -> Result<u32> {
    if !(value.is_finite() && value > 0.0) {
        return Err(RasterError::InvalidSizing(
            "computed raster dimension must be finite and positive",
        ));
    }
    let value = value.ceil().max(1.0);
    if value > f64::from(u32::MAX) {
        return Err(RasterError::InvalidSizing(
            "computed raster dimension exceeds u32",
        ));
    }
    let px = value as u32;
    Ok(max.map_or(px, |max| px.min(max)))
}

fn configure_usvg_options_for_raster(opt: &mut usvg::Options<'_>, svg: &str) {
    opt.fontdb_mut().load_system_fonts();

    if parse_svg_viewbox(svg).is_none() {
        if let Some(max_width) = parse_svg_max_width_px(svg) {
            if max_width.is_finite() && max_width > 0.0 {
                if let Some(size) = usvg::Size::from_wh(max_width, opt.default_size.height()) {
                    opt.default_size = size;
                }
            }
        }
    }

    configure_fontdb_generic_families(opt.fontdb_mut());
    opt.font_family =
        raster_default_font_family(opt.fontdb.as_ref()).unwrap_or_else(|| "Arial".to_string());
    opt.font_resolver = browser_like_font_resolver();
}

fn configure_usvg_options_for_pdf(opt: &mut svg2pdf::usvg::Options<'_>) {
    opt.fontdb_mut().load_system_fonts();
    configure_fontdb_generic_families(opt.fontdb_mut());
    opt.font_family =
        raster_default_font_family(opt.fontdb.as_ref()).unwrap_or_else(|| "Arial".to_string());
    opt.font_resolver = browser_like_pdf_font_resolver();
}

fn browser_like_font_resolver() -> usvg::FontResolver<'static> {
    let default_select = usvg::FontResolver::default_font_selector();

    usvg::FontResolver {
        select_font: Box::new(move |font, fontdb| {
            default_select(font, fontdb)
                .or_else(|| query_browser_like_fallback_font(font, fontdb.as_ref()))
                .or_else(|| fontdb.faces().next().map(|face| face.id))
        }),
        select_fallback: usvg::FontResolver::default_fallback_selector(),
    }
}

fn browser_like_pdf_font_resolver() -> svg2pdf::usvg::FontResolver<'static> {
    let default_select = svg2pdf::usvg::FontResolver::default_font_selector();

    svg2pdf::usvg::FontResolver {
        select_font: Box::new(move |font, fontdb| {
            default_select(font, fontdb)
                .or_else(|| query_browser_like_pdf_fallback_font(font, fontdb.as_ref()))
                .or_else(|| fontdb.faces().next().map(|face| face.id))
        }),
        select_fallback: svg2pdf::usvg::FontResolver::default_fallback_selector(),
    }
}

fn configure_fontdb_generic_families(fontdb: &mut usvg::fontdb::Database) {
    let sans = first_font_family(fontdb, |face| !face.monospaced)
        .or_else(|| first_font_family(fontdb, |_| true));
    let mono = first_font_family(fontdb, |face| face.monospaced).or_else(|| sans.clone());

    if query_normal_font_family(fontdb, usvg::fontdb::Family::SansSerif).is_none() {
        if let Some(family) = sans.as_ref() {
            fontdb.set_sans_serif_family(family.clone());
        }
    }
    if query_normal_font_family(fontdb, usvg::fontdb::Family::Serif).is_none() {
        if let Some(family) = sans.as_ref() {
            fontdb.set_serif_family(family.clone());
        }
    }
    if query_normal_font_family(fontdb, usvg::fontdb::Family::Monospace).is_none() {
        if let Some(family) = mono.as_ref() {
            fontdb.set_monospace_family(family.clone());
        }
    }
}

fn raster_default_font_family(fontdb: &usvg::fontdb::Database) -> Option<String> {
    query_normal_font_family(fontdb, usvg::fontdb::Family::SansSerif)
        .or_else(|| query_normal_font_family(fontdb, usvg::fontdb::Family::Serif))
        .or_else(|| first_font_family(fontdb, |_| true))
}

fn query_browser_like_fallback_font(
    font: &usvg::Font,
    fontdb: &usvg::fontdb::Database,
) -> Option<usvg::fontdb::ID> {
    let mut families = Vec::with_capacity(3);
    if font_requests_monospace(font) {
        families.push(usvg::fontdb::Family::Monospace);
        families.push(usvg::fontdb::Family::SansSerif);
        families.push(usvg::fontdb::Family::Serif);
    } else {
        families.push(usvg::fontdb::Family::SansSerif);
        families.push(usvg::fontdb::Family::Serif);
        families.push(usvg::fontdb::Family::Monospace);
    }

    let query = usvg::fontdb::Query {
        families: &families,
        weight: usvg::fontdb::Weight(font.weight()),
        stretch: font.stretch().into(),
        style: font.style().into(),
    };
    fontdb.query(&query)
}

fn query_browser_like_pdf_fallback_font(
    font: &svg2pdf::usvg::Font,
    fontdb: &svg2pdf::usvg::fontdb::Database,
) -> Option<svg2pdf::usvg::fontdb::ID> {
    let mut families = Vec::with_capacity(3);
    if pdf_font_requests_monospace(font) {
        families.push(svg2pdf::usvg::fontdb::Family::Monospace);
        families.push(svg2pdf::usvg::fontdb::Family::SansSerif);
        families.push(svg2pdf::usvg::fontdb::Family::Serif);
    } else {
        families.push(svg2pdf::usvg::fontdb::Family::SansSerif);
        families.push(svg2pdf::usvg::fontdb::Family::Serif);
        families.push(svg2pdf::usvg::fontdb::Family::Monospace);
    }

    let query = svg2pdf::usvg::fontdb::Query {
        families: &families,
        weight: svg2pdf::usvg::fontdb::Weight(font.weight()),
        stretch: font.stretch().into(),
        style: font.style().into(),
    };
    fontdb.query(&query)
}

fn query_normal_font_family(
    fontdb: &usvg::fontdb::Database,
    family: usvg::fontdb::Family<'_>,
) -> Option<String> {
    let families = [family];
    let query = usvg::fontdb::Query {
        families: &families,
        weight: usvg::fontdb::Weight::NORMAL,
        stretch: usvg::fontdb::Stretch::Normal,
        style: usvg::fontdb::Style::Normal,
    };
    fontdb
        .query(&query)
        .and_then(|id| fontdb.face(id))
        .and_then(face_family_name)
}

fn first_font_family<F>(fontdb: &usvg::fontdb::Database, mut predicate: F) -> Option<String>
where
    F: FnMut(&usvg::fontdb::FaceInfo) -> bool,
{
    fontdb
        .faces()
        .find(|face| predicate(face))
        .and_then(face_family_name)
}

fn face_family_name(face: &usvg::fontdb::FaceInfo) -> Option<String> {
    face.families
        .iter()
        .find(|(_, lang)| *lang == usvg::fontdb::Language::English_UnitedStates)
        .or_else(|| face.families.first())
        .map(|(family, _)| family.clone())
}

fn font_requests_monospace(font: &usvg::Font) -> bool {
    font.families().iter().any(|family| match family {
        usvg::FontFamily::Monospace => true,
        usvg::FontFamily::Named(name) => {
            let name = name.to_ascii_lowercase();
            name.contains("mono")
                || name.contains("courier")
                || name.contains("consolas")
                || name.contains("menlo")
        }
        _ => false,
    })
}

fn pdf_font_requests_monospace(font: &svg2pdf::usvg::Font) -> bool {
    font.families().iter().any(|family| match family {
        svg2pdf::usvg::FontFamily::Monospace => true,
        svg2pdf::usvg::FontFamily::Named(name) => {
            let name = name.to_ascii_lowercase();
            name.contains("mono")
                || name.contains("courier")
                || name.contains("consolas")
                || name.contains("menlo")
        }
        _ => false,
    })
}

fn parse_svg_max_width_px(svg: &str) -> Option<f32> {
    let start = svg.find("max-width:")? + "max-width:".len();
    let rest = svg[start..].trim_start();
    let px_end = rest.find("px")?;
    rest[..px_end].trim().parse::<f32>().ok()
}

fn parse_tiny_skia_color(text: &str) -> Option<tiny_skia::Color> {
    let s = text.trim().to_ascii_lowercase();
    match s.as_str() {
        "transparent" => return Some(tiny_skia::Color::from_rgba8(0, 0, 0, 0)),
        "white" => return Some(tiny_skia::Color::from_rgba8(255, 255, 255, 255)),
        "black" => return Some(tiny_skia::Color::from_rgba8(0, 0, 0, 255)),
        _ => {}
    }

    let hex = s.strip_prefix('#')?;
    fn hex2(b: &[u8]) -> Option<u8> {
        let hi = (*b.first()? as char).to_digit(16)? as u8;
        let lo = (*b.get(1)? as char).to_digit(16)? as u8;
        Some((hi << 4) | lo)
    }
    fn hex1(c: u8) -> Option<u8> {
        let v = (c as char).to_digit(16)? as u8;
        Some((v << 4) | v)
    }

    let bytes = hex.as_bytes();
    match bytes.len() {
        3 => Some(tiny_skia::Color::from_rgba8(
            hex1(bytes[0])?,
            hex1(bytes[1])?,
            hex1(bytes[2])?,
            255,
        )),
        4 => Some(tiny_skia::Color::from_rgba8(
            hex1(bytes[0])?,
            hex1(bytes[1])?,
            hex1(bytes[2])?,
            hex1(bytes[3])?,
        )),
        6 => Some(tiny_skia::Color::from_rgba8(
            hex2(&bytes[0..2])?,
            hex2(&bytes[2..4])?,
            hex2(&bytes[4..6])?,
            255,
        )),
        8 => Some(tiny_skia::Color::from_rgba8(
            hex2(&bytes[0..2])?,
            hex2(&bytes[2..4])?,
            hex2(&bytes[4..6])?,
            hex2(&bytes[6..8])?,
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_to_png_produces_png_signature() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10" fill="black"/></svg>"#;
        let bytes = svg_to_png(svg, &RasterOptions::default()).unwrap();
        assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    }

    #[test]
    fn svg_to_pdf_produces_pdf_signature() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10" fill="black"/></svg>"#;
        let bytes = svg_to_pdf(svg).unwrap();
        assert!(bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn svg_to_pdf_rejects_large_intrinsic_svg_by_default() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 9000 9000"><rect width="9000" height="9000" fill="black"/></svg>"#;
        let err = svg_to_pdf(svg).unwrap_err();

        assert!(
            err.to_string()
                .contains("PDF output exceeds configured size_limit"),
            "{err}"
        );
    }

    #[test]
    fn svg_to_pdf_with_unbounded_size_allows_large_intrinsic_svg() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 9000 9000"><rect width="9000" height="9000" fill="black"/></svg>"#;
        let options = RasterOptions::default().with_unbounded_size();
        let bytes = svg_to_pdf_with_options(svg, &options).unwrap();

        assert!(bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn svg_to_jpeg_defaults_to_white_background() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"></svg>"#;
        let bytes = svg_to_jpeg(svg, &RasterOptions::default()).unwrap();
        let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Jpeg)
            .unwrap()
            .to_rgb8();
        let px = img.get_pixel(0, 0);

        assert!(
            px[0] > 240 && px[1] > 240 && px[2] > 240,
            "expected default JPG background to be white-ish, got {px:?}"
        );
    }

    #[test]
    fn svg_to_png_keeps_text_visible_when_requested_font_is_missing() {
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="100%" style="max-width: 400px; background-color: white;"><text x="100" y="40" fill="#333333" font-size="32" style="font-family: '__merman_missing_font__'; text-anchor: middle;">v{}</text></svg>"##,
            merman_core::baseline::PINNED_MERMAID_BASELINE_VERSION
        );
        let bytes = svg_to_png(&svg, &RasterOptions::default()).unwrap();
        assert_png_has_visible_non_background_ink(&bytes);
    }

    #[test]
    fn default_plan_downscales_large_intrinsic_svg_without_allocating() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14544.4375 6565.5"><rect width="14544.4375" height="6565.5" fill="white"/></svg>"#;
        let plan = svg_raster_plan(svg, &RasterOptions::default()).unwrap();

        assert_eq!(plan.requested_width_px, 14545);
        assert_eq!(plan.requested_height_px, 6566);
        assert_eq!(plan.width_px, DEFAULT_MAX_RASTER_SIDE_LENGTH);
        assert!(plan.height_px < DEFAULT_MAX_RASTER_SIDE_LENGTH);
        assert!(plan.limited);
        assert!(plan.effective_scale < plan.requested_scale);
    }

    #[test]
    fn fit_to_models_browser_preview_container_before_scale() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 500"><rect width="1000" height="500" fill="black"/></svg>"#;
        let options = RasterOptions::default()
            .with_fit_to(RasterFitBox::width(250))
            .with_scale(2.0);
        let plan = svg_raster_plan(svg, &options).unwrap();

        assert_eq!(plan.requested_width_px, 500);
        assert_eq!(plan.requested_height_px, 250);
        assert_eq!(plan.width_px, 500);
        assert_eq!(plan.height_px, 250);
        assert!(!plan.limited);
    }

    #[test]
    fn size_limit_caps_actual_png_dimensions() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 500"><rect width="1000" height="500" fill="black"/></svg>"#;
        let options = RasterOptions::default()
            .with_size_limit(RasterSizeLimit::max_side_length(128))
            .with_background("white");
        let bytes = svg_to_png(svg, &options).unwrap();
        let (width, height) = png_size(&bytes);

        assert_eq!((width, height), (128, 64));
    }

    #[test]
    fn size_limit_caps_by_total_pixels() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 1000"><rect width="1000" height="1000" fill="black"/></svg>"#;
        let options = RasterOptions::default().with_size_limit(RasterSizeLimit::new(
            None,
            None,
            Some(10_000),
        ));
        let plan = svg_raster_plan(svg, &options).unwrap();

        assert_eq!((plan.width_px, plan.height_px), (100, 100));
        assert!(plan.limited);
    }

    #[test]
    fn unbounded_size_keeps_requested_dimensions() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 9000 4500"><rect width="9000" height="4500" fill="black"/></svg>"#;
        let plan = svg_raster_plan(svg, &RasterOptions::default().with_unbounded_size()).unwrap();

        assert_eq!((plan.width_px, plan.height_px), (9000, 4500));
        assert!(!plan.limited);
    }

    fn png_size(bytes: &[u8]) -> (u32, u32) {
        let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        img.dimensions()
    }

    fn assert_png_has_visible_non_background_ink(bytes: &[u8]) {
        let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let pixels = img.as_raw();
        let background = &pixels[..4];
        let differing_pixels = pixels
            .chunks_exact(4)
            .filter(|px| {
                let alpha_delta = px[3].abs_diff(background[3]) as u16;
                let rgb_delta = px[0].abs_diff(background[0]) as u16
                    + px[1].abs_diff(background[1]) as u16
                    + px[2].abs_diff(background[2]) as u16;
                alpha_delta > 3 || (px[3] > 0 && rgb_delta > 8)
            })
            .take(16)
            .count();
        assert!(
            differing_pixels >= 8,
            "expected visible text ink in rasterized PNG"
        );
    }
}
