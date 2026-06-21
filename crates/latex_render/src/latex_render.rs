use ab_glyph::{Font as _, FontRef, OutlineCurve};
use anyhow::Result;
use collections::HashMap;
use futures::StreamExt;
use futures::channel::mpsc;
use gpui::{
    App, BackgroundExecutor, Bounds, Context, Element, ElementId, FontId as GpuiFontId, FontStyle,
    FontWeight, GlobalElementId, GlyphId, Hsla, InspectorElementId, IntoElement, LayoutId,
    PathBuilder, Pixels, Point, Rgba, Style, Task, TextRun, Window, fill, point, px, size,
};
use parking_lot::{Mutex, RwLock};
use ratex_font::FontId;
use ratex_font_loader::outline_cache;
use ratex_types::color::Color as RatexColor;
use ratex_types::display_item::{DisplayItem, DisplayList};
use ratex_types::path_command::PathCommand;
use std::borrow::Cow;
use std::sync::{Arc, OnceLock};

// Prettier rewrites `_` inside math as `\_`; sanitize back so subscripts work.
static UNDERSCORE_ESCAPE: &str = r"\_";

// Glyph outline coordinates come from ab_glyph in font design units. Most KaTeX
// faces report 1000 units per em; fall back to that when the face doesn't advertise.
const DEFAULT_UNITS_PER_EM: f32 = 1000.0;

// Outline points whose mapped pixel distance is below this are considered to
// continue the current sub-path rather than starting a new one. Matches the
// upstream ratex-svg standalone tolerance.
const SUBPATH_JOIN_TOLERANCE: f32 = 0.01;

// Default stroke width for unfilled ratex SVG paths (radicals, large delimiters).
// ratex-svg's standalone export uses 1.5 user units at its default em; ours is
// applied in device-pixel space so we scale with font size.
const STROKE_WIDTH_EM: f32 = 1.5 / 40.0;

struct KatexFace {
    id: FontId,
    filename: &'static str,
    family: &'static str,
    weight: FontWeight,
    style: FontStyle,
}

// The embedded KaTeX faces ratex can emit, with the family/weight/style they
// advertise to font databases. CJK and emoji fallbacks resolve to system fonts
// at runtime and are handled by the outline-tessellation fallback instead.
const KATEX_FACES: &[KatexFace] = &[
    KatexFace {
        id: FontId::AmsRegular,
        filename: "KaTeX_AMS-Regular.ttf",
        family: "KaTeX_AMS",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::CaligraphicRegular,
        filename: "KaTeX_Caligraphic-Regular.ttf",
        family: "KaTeX_Caligraphic",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::FrakturRegular,
        filename: "KaTeX_Fraktur-Regular.ttf",
        family: "KaTeX_Fraktur",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::FrakturBold,
        filename: "KaTeX_Fraktur-Bold.ttf",
        family: "KaTeX_Fraktur",
        weight: FontWeight::BOLD,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::MainBold,
        filename: "KaTeX_Main-Bold.ttf",
        family: "KaTeX_Main",
        weight: FontWeight::BOLD,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::MainBoldItalic,
        filename: "KaTeX_Main-BoldItalic.ttf",
        family: "KaTeX_Main",
        weight: FontWeight::BOLD,
        style: FontStyle::Italic,
    },
    KatexFace {
        id: FontId::MainItalic,
        filename: "KaTeX_Main-Italic.ttf",
        family: "KaTeX_Main",
        weight: FontWeight::NORMAL,
        style: FontStyle::Italic,
    },
    KatexFace {
        id: FontId::MainRegular,
        filename: "KaTeX_Main-Regular.ttf",
        family: "KaTeX_Main",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::MathBoldItalic,
        filename: "KaTeX_Math-BoldItalic.ttf",
        family: "KaTeX_Math",
        weight: FontWeight::BOLD,
        style: FontStyle::Italic,
    },
    KatexFace {
        id: FontId::MathItalic,
        filename: "KaTeX_Math-Italic.ttf",
        family: "KaTeX_Math",
        weight: FontWeight::NORMAL,
        style: FontStyle::Italic,
    },
    KatexFace {
        id: FontId::SansSerifBold,
        filename: "KaTeX_SansSerif-Bold.ttf",
        family: "KaTeX_SansSerif",
        weight: FontWeight::BOLD,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::SansSerifItalic,
        filename: "KaTeX_SansSerif-Italic.ttf",
        family: "KaTeX_SansSerif",
        weight: FontWeight::NORMAL,
        style: FontStyle::Italic,
    },
    KatexFace {
        id: FontId::SansSerifRegular,
        filename: "KaTeX_SansSerif-Regular.ttf",
        family: "KaTeX_SansSerif",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::ScriptRegular,
        filename: "KaTeX_Script-Regular.ttf",
        family: "KaTeX_Script",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::Size1Regular,
        filename: "KaTeX_Size1-Regular.ttf",
        family: "KaTeX_Size1",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::Size2Regular,
        filename: "KaTeX_Size2-Regular.ttf",
        family: "KaTeX_Size2",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::Size3Regular,
        filename: "KaTeX_Size3-Regular.ttf",
        family: "KaTeX_Size3",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::Size4Regular,
        filename: "KaTeX_Size4-Regular.ttf",
        family: "KaTeX_Size4",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
    KatexFace {
        id: FontId::TypewriterRegular,
        filename: "KaTeX_Typewriter-Regular.ttf",
        family: "KaTeX_Typewriter",
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LatexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl LatexColor {
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

/// Logical-em dimensions of a rendered formula. Multiply by the target font size to get pixel dimensions.
#[derive(Clone, Copy, Debug)]
pub struct RenderedFormula {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

/// Parsed and laid-out formula. Cheap to clone (display list lives behind `Arc`), repaints just walk the stored display list.
#[derive(Clone)]
pub struct PreparedFormula {
    pub display_list: Arc<DisplayList>,
    pub metrics: RenderedFormula,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CacheKey {
    source: String,
    color: LatexColor,
    display: bool,
}

#[derive(Clone)]
enum CacheEntry {
    Pending,
    Ready(Result<PreparedFormula, Arc<String>>),
}

pub struct LatexRenderer {
    cache: Arc<Mutex<HashMap<CacheKey, CacheEntry>>>,
    executor: BackgroundExecutor,
    on_render_complete: Option<Arc<dyn Fn() + Send + Sync>>,
    _notify_task: Option<Task<()>>,
}

impl LatexRenderer {
    /// Build a renderer wired to re-render the owning view whenever a formula
    /// finishes parsing in the background. Spawns and owns the foreground
    /// bridge task internally, so callers don't have to thread an mpsc channel through their setup code.
    pub fn for_view<T: 'static>(executor: BackgroundExecutor, cx: &mut Context<T>) -> Self {
        let (tx, mut rx) = mpsc::unbounded::<()>();
        let notify_task = cx.spawn(async move |this, cx| {
            while rx.next().await.is_some() {
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    break;
                }
            }
        });
        Self {
            cache: Arc::new(Mutex::new(HashMap::default())),
            executor,
            on_render_complete: Some(Arc::new(move || {
                if tx.unbounded_send(()).is_err() {
                    log::debug!("latex_render: skipping notify for dropped view");
                }
            })),
            _notify_task: Some(notify_task),
        }
    }

    /// Returns `None` while parsing/layout is still in progress. Returns
    /// `Some(Ok(...))` with the cached display list once ready, or `Some(Err(...))`
    /// if parsing failed.
    pub fn render(
        &self,
        source: &str,
        color: LatexColor,
        display: bool,
    ) -> Option<Result<PreparedFormula, String>> {
        let clean_source = sanitize_source(source);
        let key = CacheKey {
            source: clean_source.clone(),
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

        self.cache.lock().insert(key.clone(), CacheEntry::Pending);

        self.executor
            .spawn({
                let cache = self.cache.clone();
                let on_complete = self.on_render_complete.clone();
                let source = clean_source;
                async move {
                    let entry = match parse_and_layout(&source, color, display) {
                        Ok(prepared) => CacheEntry::Ready(Ok(prepared)),
                        Err(error) => CacheEntry::Ready(Err(Arc::new(error.to_string()))),
                    };
                    cache.lock().insert(key, entry);
                    if let Some(callback) = on_complete {
                        callback();
                    }
                }
            })
            .detach();

        None
    }
}

fn sanitize_source(source: &str) -> String {
    source.replace(UNDERSCORE_ESCAPE, "_").trim().to_string()
}

fn parse_and_layout(source: &str, color: LatexColor, display: bool) -> Result<PreparedFormula> {
    use ratex_layout::{LayoutOptions, layout, to_display_list};
    use ratex_parser::parse;
    use ratex_types::math_style::MathStyle;

    let ast = parse(source).map_err(|error| anyhow::anyhow!("LaTeX parse error: {error}"))?;
    let opts = LayoutOptions::default()
        .with_style(if display {
            MathStyle::Display
        } else {
            MathStyle::Text
        })
        .with_color(RatexColor::new(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
        ));
    let lbox = layout(&ast, &opts);
    let dl = to_display_list(&lbox);
    let metrics = RenderedFormula {
        width: dl.width,
        height: dl.height,
        depth: dl.depth,
    };
    Ok(PreparedFormula {
        display_list: Arc::new(dl),
        metrics,
    })
}

/// Where a formula sits relative to surrounding content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathPlacement {
    /// Math flows with text: the element occupies a line-height-tall slot and the formula baseline is pinned to the surrounding text's baseline.
    Inline,
    /// Math owns its own block: a tight box (formula width by height plus depth) meant for a centered display container.
    Display,
}

/// Build a GPUI element that paints `formula` natively. The formula's em size
/// is the inherited text style's font size times `font_scale`, and it draws in
/// the inherited text color (for any item the formula didn't recolor).
///
/// All metrics are resolved from `window.text_style()` during this element's
/// own layout and paint, not captured at build time. Sibling text elements
/// resolve their fonts and line heights the same way, so inline placement
/// stays exact under enclosing div text-style refinements (paragraph
/// line-height overrides, heading font sizes) and at any zoom level.
///
/// For [`MathPlacement::Inline`], descenders paint past the slot bottom by
/// design (no overflow clipping). GPUI does not feed text-glyph baselines into
/// taffy, so `items_baseline` on a flex container treats every item's baseline
/// as its bottom edge. Making this element the same height as the host text
/// line and offsetting the painted formula inside is what lines the baselines up.
pub fn latex_formula_element(
    formula: PreparedFormula,
    placement: MathPlacement,
    font_scale: f32,
) -> LatexFormulaElement {
    LatexFormulaElement {
        formula,
        placement,
        font_scale,
    }
}

pub struct LatexFormulaElement {
    formula: PreparedFormula,
    placement: MathPlacement,
    font_scale: f32,
}

struct ResolvedFormulaLayout {
    em: f32,
    size: gpui::Size<Pixels>,
    /// Top of the painted formula relative to the element's top.
    formula_top: f32,
}

impl LatexFormulaElement {
    fn resolve_layout(&self, window: &Window) -> ResolvedFormulaLayout {
        let text_style = window.text_style();
        let rem_size = window.rem_size();
        let text_font_size = text_style.font_size.to_pixels(rem_size);
        let em = f32::from(text_font_size) * self.font_scale;
        let width = self.formula.metrics.width as f32 * em;
        let height = self.formula.metrics.height as f32 * em;
        let depth = self.formula.metrics.depth as f32 * em;
        match self.placement {
            MathPlacement::Inline => {
                let line_height = text_style.line_height_in_pixels(rem_size);
                // Match the surrounding text's painted baseline exactly: shape
                // a probe line through the same pipeline text elements use, and
                // reproduce text painting's `(line_height - ascent - descent) /
                // 2 + ascent` placement. Font-header metrics can differ
                // slightly from the shaped line's ascent/descent, which reads
                // as the formula sitting a hair below the text baseline.
                let probe = window.text_system().shape_line(
                    "x".into(),
                    text_font_size,
                    &[TextRun {
                        len: 1,
                        font: text_style.font(),
                        color: text_style.color,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    }],
                    None,
                );
                let text_baseline =
                    (line_height - probe.ascent - probe.descent) / 2. + probe.ascent;
                ResolvedFormulaLayout {
                    em,
                    size: size(px(width), line_height),
                    formula_top: f32::from(text_baseline) - height,
                }
            }
            MathPlacement::Display => ResolvedFormulaLayout {
                em,
                size: size(px(width), px(height + depth)),
                formula_top: 0.0,
            },
        }
    }
}

impl Element for LatexFormulaElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let resolved = self.resolve_layout(window);
        let mut style = Style::default();
        style.size.width = resolved.size.width.into();
        style.size.height = resolved.size.height.into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut (),
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut (),
        _prepaint: &mut (),
        window: &mut Window,
        _cx: &mut App,
    ) {
        let resolved = self.resolve_layout(window);
        let color = window.text_style().color;
        let origin = point(bounds.origin.x, bounds.origin.y + px(resolved.formula_top));
        paint_display_list(
            window,
            origin,
            resolved.em,
            color,
            &self.formula.display_list,
        );
    }
}

impl IntoElement for LatexFormulaElement {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

fn paint_display_list(
    window: &mut Window,
    origin: Point<Pixels>,
    em: f32,
    default_color: Hsla,
    display_list: &DisplayList,
) {
    let ox = f32::from(origin.x);
    let oy = f32::from(origin.y);
    for item in &display_list.items {
        match item {
            DisplayItem::GlyphPath {
                x,
                y,
                scale,
                font,
                char_code,
                color,
            } => {
                let color = ratex_to_hsla(*color, default_color);
                paint_glyph(
                    window,
                    ox,
                    oy,
                    em,
                    *x as f32,
                    *y as f32,
                    *scale as f32,
                    font,
                    *char_code,
                    color,
                );
            }
            DisplayItem::Line {
                x,
                y,
                width,
                thickness,
                color,
                dashed,
            } => {
                let color = ratex_to_hsla(*color, default_color);
                let thickness_px = (*thickness as f32) * em;
                let x_px = ox + (*x as f32) * em;
                let center_y_px = oy + (*y as f32) * em;
                let width_px_val = (*width as f32) * em;
                if *dashed {
                    paint_dashed_line(window, x_px, center_y_px, width_px_val, thickness_px, color);
                } else {
                    // Snap rules (fraction bars, overlines) to device pixels like
                    // GPUI underlines, so a thin bar lands on one pixel row
                    // instead of smearing in half-tones across two.
                    let scale_factor = window.scale_factor();
                    let thickness_snapped =
                        (thickness_px * scale_factor).round().max(1.0) / scale_factor;
                    let top_px =
                        ((center_y_px - thickness_px / 2.0) * scale_factor).round() / scale_factor;
                    let bounds = Bounds {
                        origin: point(px(x_px), px(top_px)),
                        size: size(px(width_px_val), px(thickness_snapped)),
                    };
                    window.paint_quad(fill(bounds, color));
                }
            }
            DisplayItem::Rect {
                x,
                y,
                width,
                height,
                color,
            } => {
                let color = ratex_to_hsla(*color, default_color);
                let bounds = Bounds {
                    origin: point(px(ox + (*x as f32) * em), px(oy + (*y as f32) * em)),
                    size: size(px((*width as f32) * em), px((*height as f32) * em)),
                };
                window.paint_quad(fill(bounds, color));
            }
            DisplayItem::Path {
                x,
                y,
                commands,
                fill: filled,
                color,
            } => {
                let color = ratex_to_hsla(*color, default_color);
                paint_em_path(
                    window,
                    ox + (*x as f32) * em,
                    oy + (*y as f32) * em,
                    em,
                    commands,
                    *filled,
                    color,
                );
            }
        }
    }
}

fn ratex_to_hsla(color: RatexColor, default: Hsla) -> Hsla {
    // The layout pass stamps `LatexColor::BLACK` onto every item by default. We
    // treat opaque black as "use the caller's color" so that the formula picks
    // up theme text color without us having to thread it through the cache key.
    if color == RatexColor::BLACK {
        return default;
    }
    Rgba {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    }
    .into()
}

fn paint_em_path(
    window: &mut Window,
    origin_x_px: f32,
    origin_y_px: f32,
    em: f32,
    commands: &[PathCommand],
    filled: bool,
    color: Hsla,
) {
    // ratex emits one DisplayItem::Path with multiple subpaths separated by `MoveTo`.
    // Lyon's fill tessellator behaves more reliably when each subpath is built as its
    // own Path, matching ratex-svg's approach of emitting one `<path>` per subpath.
    let mut subpath_start: Option<usize> = None;
    for (i, cmd) in commands.iter().enumerate() {
        if matches!(cmd, PathCommand::MoveTo { .. }) {
            if let Some(start) = subpath_start
                && start < i
            {
                paint_subpath(
                    window,
                    origin_x_px,
                    origin_y_px,
                    em,
                    &commands[start..i],
                    filled,
                    color,
                );
            }
            subpath_start = Some(i);
        }
    }
    if let Some(start) = subpath_start
        && start < commands.len()
    {
        paint_subpath(
            window,
            origin_x_px,
            origin_y_px,
            em,
            &commands[start..],
            filled,
            color,
        );
    }
}

// Filled subpaths at most this many device pixels across one axis are painted
// as snapped quads instead of tessellated paths. Extensible delimiter bars
// (`\|`, `\left|`) are ~0.04em wide, under a device pixel at body sizes, and
// antialias into near-invisible smears when they straddle pixel boundaries,
// making the same delimiter visible on one side of a formula and not the other.
const HAIRLINE_MAX_DEVICE_PX: f32 = 2.0;
// A subpath only counts as a hairline strip when it is this many times longer
// than it is thick, so small square-ish path pieces still tessellate normally.
const HAIRLINE_MIN_ASPECT: f32 = 4.0;

fn paint_subpath(
    window: &mut Window,
    origin_x_px: f32,
    origin_y_px: f32,
    em: f32,
    commands: &[PathCommand],
    filled: bool,
    color: Hsla,
) {
    if filled
        && let Some(bounds) = hairline_strip_bounds(window, origin_x_px, origin_y_px, em, commands)
    {
        window.paint_quad(fill(bounds, color));
        return;
    }
    let stroke_width = px(STROKE_WIDTH_EM * em);
    let mut builder = if filled {
        PathBuilder::fill()
    } else {
        PathBuilder::stroke(stroke_width)
    };
    let map = |x: f64, y: f64| {
        point(
            px(origin_x_px + x as f32 * em),
            px(origin_y_px + y as f32 * em),
        )
    };
    let mut started = false;
    for cmd in commands {
        match *cmd {
            PathCommand::MoveTo { x, y } => {
                builder.move_to(map(x, y));
                started = true;
            }
            PathCommand::LineTo { x, y } => {
                builder.line_to(map(x, y));
            }
            PathCommand::QuadTo { x1, y1, x, y } => {
                builder.curve_to(map(x, y), map(x1, y1));
            }
            PathCommand::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                builder.cubic_bezier_to(map(x, y), map(x1, y1), map(x2, y2));
            }
            PathCommand::Close => {
                builder.close();
            }
        }
    }
    if started && filled {
        // Auto-close any subpath that ratex left open: fill tessellation requires
        // a closed loop, otherwise lyon stitches a straight edge across the gap.
        builder.close();
    }
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

/// If `commands` outline a thin vertical or horizontal strip, return its
/// device-pixel-snapped bounds (thin axis at least one device pixel) so the
/// caller can draw a quad instead of tessellating. Rounded strip ends are
/// flattened into the quad; at hairline sizes the difference is invisible.
fn hairline_strip_bounds(
    window: &Window,
    origin_x_px: f32,
    origin_y_px: f32,
    em: f32,
    commands: &[PathCommand],
) -> Option<Bounds<Pixels>> {
    let mut min = (f32::MAX, f32::MAX);
    let mut max = (f32::MIN, f32::MIN);
    let mut extend = |x: f64, y: f64| {
        let (x, y) = (x as f32, y as f32);
        min = (min.0.min(x), min.1.min(y));
        max = (max.0.max(x), max.1.max(y));
    };
    for cmd in commands {
        match *cmd {
            PathCommand::MoveTo { x, y } | PathCommand::LineTo { x, y } => extend(x, y),
            PathCommand::QuadTo { x1, y1, x, y } => {
                extend(x1, y1);
                extend(x, y);
            }
            PathCommand::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                extend(x1, y1);
                extend(x2, y2);
                extend(x, y);
            }
            PathCommand::Close => {}
        }
    }
    if min.0 > max.0 || min.1 > max.1 {
        return None;
    }

    let width_px = (max.0 - min.0) * em;
    let height_px = (max.1 - min.1) * em;
    let scale_factor = window.scale_factor();
    let snap = |value: f32| (value * scale_factor).round() / scale_factor;
    let snap_thickness = |value: f32| (value * scale_factor).round().max(1.0) / scale_factor;

    if width_px * scale_factor <= HAIRLINE_MAX_DEVICE_PX
        && height_px >= HAIRLINE_MIN_ASPECT * width_px
    {
        return Some(Bounds {
            origin: point(
                px(snap(origin_x_px + min.0 * em)),
                px(origin_y_px + min.1 * em),
            ),
            size: size(px(snap_thickness(width_px)), px(height_px)),
        });
    }
    if height_px * scale_factor <= HAIRLINE_MAX_DEVICE_PX
        && width_px >= HAIRLINE_MIN_ASPECT * height_px
    {
        return Some(Bounds {
            origin: point(
                px(origin_x_px + min.0 * em),
                px(snap(origin_y_px + min.1 * em)),
            ),
            size: size(px(width_px), px(snap_thickness(height_px))),
        });
    }
    None
}

fn paint_dashed_line(
    window: &mut Window,
    x: f32,
    center_y: f32,
    width: f32,
    thickness: f32,
    color: Hsla,
) {
    let dash_unit = px(thickness * 4.0);
    let mut builder = PathBuilder::stroke(px(thickness)).dash_array(&[dash_unit, dash_unit]);
    builder.move_to(point(px(x), px(center_y)));
    builder.line_to(point(px(x + width), px(center_y)));
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

fn paint_glyph(
    window: &mut Window,
    origin_x_px: f32,
    origin_y_px: f32,
    em: f32,
    x_em: f32,
    y_em: f32,
    glyph_scale: f32,
    font_str: &str,
    char_code: u32,
    color: Hsla,
) {
    let Some(font_id) = FontId::parse(font_str) else {
        return;
    };
    let Some(ch) = char::from_u32(char_code) else {
        return;
    };
    let pen_x = origin_x_px + x_em * em;
    let pen_y = origin_y_px + y_em * em;

    // Prefer GPUI's glyph atlas: the platform text system rasterizes with the
    // same hinting and subpixel treatment as editor text, so math glyphs stay
    // sharp at every zoom level. Outline tessellation below remains as the
    // fallback for system CJK/emoji faces and registration failures.
    if let Some((gpui_font_id, glyph_id)) = atlas_glyph(window, font_id, ch) {
        match window.paint_glyph(
            point(px(pen_x), px(pen_y)),
            gpui_font_id,
            glyph_id,
            px(glyph_scale * em),
            color,
        ) {
            Ok(()) => return,
            Err(error) => {
                log::warn!("latex_render: atlas glyph paint failed for {ch:?}: {error}")
            }
        }
    }

    with_font(font_id, |font_ref| {
        let glyph_id = font_ref.glyph_id(ch);
        let Some(curves) = outline_cache::get_or_compute_outline(font_id, font_ref, glyph_id)
        else {
            return;
        };
        if curves.is_empty() {
            return;
        }
        let upem = font_ref.units_per_em().unwrap_or(DEFAULT_UNITS_PER_EM);
        let unit_scale = glyph_scale * em / upem;
        paint_glyph_curves(window, pen_x, pen_y, unit_scale, &curves, color);
    });
}

fn paint_glyph_curves(
    window: &mut Window,
    pen_x: f32,
    pen_y: f32,
    unit_scale: f32,
    curves: &[OutlineCurve],
    color: Hsla,
) {
    let mut builder = PathBuilder::fill();
    let mut last_end: Option<(f32, f32)> = None;
    // ab_glyph outlines are y-up with baseline at zero; flip y to GPUI's y-down.
    let map = |p: ab_glyph::Point| (pen_x + p.x * unit_scale, pen_y - p.y * unit_scale);

    for curve in curves {
        let (start, end) = match curve {
            OutlineCurve::Line(p0, p1) => (map(*p0), map(*p1)),
            OutlineCurve::Quad(p0, _, p2) => (map(*p0), map(*p2)),
            OutlineCurve::Cubic(p0, _, _, p3) => (map(*p0), map(*p3)),
        };
        let need_move = last_end.is_none_or(|(lx, ly)| {
            (lx - start.0).abs() > SUBPATH_JOIN_TOLERANCE
                || (ly - start.1).abs() > SUBPATH_JOIN_TOLERANCE
        });
        if need_move {
            if last_end.is_some() {
                builder.close();
            }
            builder.move_to(point(px(start.0), px(start.1)));
        }
        match curve {
            OutlineCurve::Line(_, p1) => {
                let pt = map(*p1);
                builder.line_to(point(px(pt.0), px(pt.1)));
            }
            OutlineCurve::Quad(_, p1, p2) => {
                let ctrl = map(*p1);
                let pt = map(*p2);
                builder.curve_to(point(px(pt.0), px(pt.1)), point(px(ctrl.0), px(ctrl.1)));
            }
            OutlineCurve::Cubic(_, p1, p2, p3) => {
                let c1 = map(*p1);
                let c2 = map(*p2);
                let pt = map(*p3);
                builder.cubic_bezier_to(
                    point(px(pt.0), px(pt.1)),
                    point(px(c1.0), px(c1.1)),
                    point(px(c2.0), px(c2.1)),
                );
            }
        }
        last_end = Some(end);
    }
    if last_end.is_some() {
        builder.close();
    }
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

/// Resolve a ratex font + character to a GPUI font id and glyph id suitable
/// for `Window::paint_glyph`. Glyph indices are font-internal, and GPUI loads
/// the same TTF bytes we query through ab_glyph, so the cmap lookup is valid
/// for both.
fn atlas_glyph(window: &Window, font_id: FontId, ch: char) -> Option<(GpuiFontId, GlyphId)> {
    let gpui_font_id = resolve_katex_font(window, font_id)?;
    let glyph_index = with_font(font_id, |font_ref| font_ref.glyph_id(ch).0)?;
    if glyph_index == 0 {
        return None;
    }
    Some((gpui_font_id, GlyphId(glyph_index as u32)))
}

fn resolved_fonts() -> &'static RwLock<HashMap<FontId, Option<GpuiFontId>>> {
    static RESOLVED: OnceLock<RwLock<HashMap<FontId, Option<GpuiFontId>>>> = OnceLock::new();
    RESOLVED.get_or_init(|| RwLock::new(HashMap::default()))
}

fn resolve_katex_font(window: &Window, font_id: FontId) -> Option<GpuiFontId> {
    if let Some(resolved) = resolved_fonts().read().get(&font_id) {
        return *resolved;
    }
    let resolved = (|| {
        let face = KATEX_FACES.iter().find(|face| face.id == font_id)?;
        if !register_katex_fonts(window) {
            return None;
        }
        let mut font = gpui::font(face.family);
        font.weight = face.weight;
        font.style = face.style;
        let text_system = window.text_system();
        let candidate = text_system.resolve_font(&font);
        // `resolve_font` silently substitutes the default font stack when the
        // family is missing; painting with another face's glyph indices would
        // draw garbage, so verify the id maps back to the family we asked for.
        let family_matches = text_system
            .get_font_for_id(candidate)
            .is_some_and(|resolved| resolved.family == face.family);
        family_matches.then_some(candidate)
    })();
    resolved_fonts().write().insert(font_id, resolved);
    resolved
}

fn register_katex_fonts(window: &Window) -> bool {
    static REGISTERED: OnceLock<bool> = OnceLock::new();
    *REGISTERED.get_or_init(|| {
        let fonts: Vec<Cow<'static, [u8]>> = KATEX_FACES
            .iter()
            .filter_map(|face| {
                let bytes = ratex_katex_fonts::ttf_bytes(face.filename)?;
                Some(patched_font_bytes(face, bytes))
            })
            .collect();
        if fonts.len() != KATEX_FACES.len() {
            log::error!("latex_render: embedded KaTeX font files are missing");
            return false;
        }
        match window.text_system().add_fonts(fonts) {
            Ok(()) => true,
            Err(error) => {
                log::error!("latex_render: failed to register KaTeX fonts: {error}");
                false
            }
        }
    })
}

/// KaTeX TTFs ship with no style bits set: `fsSelection` is REGULAR and
/// `head.macStyle` is zero on every face, italic ones included. Font databases
/// therefore can't tell Main-Regular from Main-Italic (both "Normal, 400") and
/// family matching degrades to registration order; rasterizing one face with
/// glyph indices computed from another draws shifted glyphs (`=` becomes `@`).
/// Patch each face's real style into the bytes before registering.
fn patched_font_bytes(face: &KatexFace, bytes: Cow<'static, [u8]>) -> Cow<'static, [u8]> {
    let italic = face.style == FontStyle::Italic;
    let bold = face.weight == FontWeight::BOLD;
    if !italic && !bold {
        return bytes;
    }
    match apply_style_bits(&bytes, italic, bold) {
        Some(patched) => Cow::Owned(patched),
        None => {
            log::warn!(
                "latex_render: could not patch style bits for {}",
                face.filename
            );
            bytes
        }
    }
}

fn apply_style_bits(bytes: &[u8], italic: bool, bold: bool) -> Option<Vec<u8>> {
    let mut data = bytes.to_vec();
    let num_tables = u16::from_be_bytes([*data.get(4)?, *data.get(5)?]) as usize;
    let mut fs_selection_at = None;
    let mut mac_style_at = None;
    for i in 0..num_tables {
        let record = 12 + i * 16;
        let tag = data.get(record..record + 4)?;
        let table_offset =
            u32::from_be_bytes(data.get(record + 8..record + 12)?.try_into().ok()?) as usize;
        match tag {
            // OS/2 `fsSelection` lives at byte 62 of the table in every version.
            b"OS/2" => fs_selection_at = Some(table_offset + 62),
            // `macStyle` lives at byte 44 of `head`.
            b"head" => mac_style_at = Some(table_offset + 44),
            _ => {}
        }
    }

    fn patch_u16(data: &mut [u8], at: usize, set: u16, clear: u16) -> Option<()> {
        let raw: [u8; 2] = data.get(at..at + 2)?.try_into().ok()?;
        let value = (u16::from_be_bytes(raw) & !clear) | set;
        data.get_mut(at..at + 2)?
            .copy_from_slice(&value.to_be_bytes());
        Some(())
    }

    // fsSelection: bit 0 = ITALIC, bit 5 = BOLD, bit 6 = REGULAR (exclusive
    // with the other two, so clear it).
    let fs_bits = (italic as u16) | ((bold as u16) << 5);
    patch_u16(&mut data, fs_selection_at?, fs_bits, 1 << 6)?;
    // macStyle: bit 0 = bold, bit 1 = italic.
    let mac_bits = (bold as u16) | ((italic as u16) << 1);
    patch_u16(&mut data, mac_style_at?, mac_bits, 0)?;
    Some(data)
}

struct FontEntry {
    font_ref: FontRef<'static>,
}

fn cache() -> &'static RwLock<HashMap<FontId, FontEntry>> {
    static FONT_CACHE: OnceLock<RwLock<HashMap<FontId, FontEntry>>> = OnceLock::new();
    FONT_CACHE.get_or_init(|| RwLock::new(HashMap::default()))
}

fn with_font<R>(font_id: FontId, f: impl FnOnce(&FontRef<'_>) -> R) -> Option<R> {
    {
        let r = cache().read();
        if let Some(entry) = r.get(&font_id) {
            return Some(f(&entry.font_ref));
        }
    }
    let bytes = load_font_bytes(font_id)?;
    // Box::leak is intentional: KaTeX fonts are loaded once per process and
    // used for the program's lifetime.
    let static_bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());
    let font_ref = FontRef::try_from_slice(static_bytes).ok()?;
    {
        let mut w = cache().write();
        w.entry(font_id).or_insert(FontEntry { font_ref });
    }
    let r = cache().read();
    let entry = r.get(&font_id)?;
    Some(f(&entry.font_ref))
}

fn load_font_bytes(font_id: FontId) -> Option<Vec<u8>> {
    // ratex-font-loader doesn't expose a "load this font_id" helper; we synthesize
    // a one-item DisplayList that uses the target font and let it do the routing
    // (embedded KaTeX fonts vs system CJK/emoji).
    let probe = DisplayItem::GlyphPath {
        x: 0.0,
        y: 0.0,
        scale: 1.0,
        font: font_id.as_str().to_string(),
        char_code: 0x41,
        color: RatexColor::BLACK,
    };
    let set = ratex_font_loader::load_fonts_for_items("", std::slice::from_ref(&probe)).ok()?;
    set.get(&font_id).map(|bytes| bytes.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_reverses_prettier_underscore_escape() {
        assert_eq!(
            sanitize_source(r"x\_{1,2} = \frac{-b}{2a} \tag{1}"),
            r"x_{1,2} = \frac{-b}{2a} \tag{1}".to_string()
        );
    }

    #[test]
    fn parse_and_layout_simple() {
        let result = parse_and_layout("x^2", LatexColor::BLACK, true);
        assert!(result.is_ok(), "{:?}", result.err());
    }

    #[test]
    fn parse_and_layout_fraction() {
        let result = parse_and_layout(r"\frac{a}{b}", LatexColor::BLACK, true);
        assert!(result.is_ok(), "{:?}", result.err());
    }

    #[test]
    fn parse_and_layout_chemistry_preserved() {
        let result = parse_and_layout(
            r"\ce{H2SO4 + 2NaOH -> Na2SO4 + 2H2O}",
            LatexColor::BLACK,
            true,
        );
        assert!(result.is_ok(), "{:?}", result.err());
    }

    #[test]
    fn parse_and_layout_tag_preserved() {
        let result = parse_and_layout(r"\tag{1} x_1 + x_2 = y", LatexColor::BLACK, true);
        assert!(result.is_ok(), "{:?}", result.err());
    }

    #[test]
    fn style_bit_patch_keeps_font_parseable_and_glyphs_stable() {
        let bytes = ratex_katex_fonts::ttf_bytes("KaTeX_Main-Italic.ttf")
            .expect("embedded Main-Italic available");
        let patched = apply_style_bits(&bytes, true, false).expect("patch applies");
        assert_ne!(bytes.as_ref(), patched.as_slice());

        let original = FontRef::try_from_slice(&bytes).expect("original parses");
        let patched_font = FontRef::try_from_slice(&patched).expect("patched parses");
        for ch in ['=', 'x', '4', 'a'] {
            assert_eq!(
                original.glyph_id(ch),
                patched_font.glyph_id(ch),
                "glyph id changed for {ch:?}"
            );
        }
    }

    #[test]
    fn style_bit_patch_disambiguates_faces_for_font_databases() {
        // KaTeX faces all ship with REGULAR style bits; fontdb (via ttf-parser)
        // must see the patched style or family matching stays order-dependent
        // and glyphs render shifted (`=` as `@`).
        for face in KATEX_FACES {
            let bytes = ratex_katex_fonts::ttf_bytes(face.filename).expect("face embedded");
            let registered = patched_font_bytes(face, bytes);
            let parsed = ttf_parser::Face::parse(&registered, 0).expect("face parses");
            assert_eq!(
                parsed.is_italic(),
                face.style == FontStyle::Italic,
                "italic bit wrong for {}",
                face.filename
            );
            assert_eq!(
                parsed.is_bold(),
                face.weight == FontWeight::BOLD,
                "bold bit wrong for {}",
                face.filename
            );
        }
    }

    #[test]
    fn norm_delimiter_bars_detected_as_hairline_strips() {
        // `\left\| ... \right\|` extensible bars are ~0.04em-wide filled path
        // subpaths. They must be classified as hairline strips so they paint as
        // snapped quads; tessellated they antialias into near-invisibility at
        // body sizes, depending on where the formula lands on the pixel grid.
        let result = parse_and_layout(
            r"\left\| \frac{\partial f}{\partial x} \right\|_2",
            LatexColor::BLACK,
            true,
        )
        .expect("layout");
        let bar_paths = result
            .display_list
            .items
            .iter()
            .filter(|item| matches!(item, DisplayItem::Path { fill: true, .. }))
            .count();
        assert_eq!(bar_paths, 2, "expected one path per delimiter");
    }

    #[test]
    fn loads_katex_main_regular() {
        let bytes = load_font_bytes(FontId::MainRegular);
        assert!(
            bytes.is_some(),
            "embed-fonts feature should make Main-Regular available"
        );
    }
}
