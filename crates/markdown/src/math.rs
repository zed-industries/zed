use collections::HashMap;
use gpui::{
    AnyElement, Bounds, Context, Hsla, Path, Pixels, Point, Size, Task, point, px, quad, size,
};
use rex::font::backend::ttf_parser::ttf_parser_crate as rex_ttf_parser;
use std::sync::{Arc, OnceLock};
use ui::prelude::*;

use crate::parser::MarkdownEvent;
use crate::{Markdown, MarkdownStyle, ParsedMarkdown};

static MATH_FONT_DATA: &[u8] = include_bytes!("../fonts/XITSMath-Regular.otf");

type MathExpressionCache = HashMap<MathCacheKey, Arc<CachedMathExpression>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct MathCacheKey {
    pub(crate) source: String,
    pub(crate) display: bool,
}

#[derive(Default, Clone)]
pub(crate) struct MathState {
    cache: MathExpressionCache,
}

struct CachedMathExpression {
    result: Arc<OnceLock<Result<RenderedMath, String>>>,
    _task: Task<()>,
}

#[derive(Clone)]
struct RenderedMath {
    paths: Vec<(Path<Pixels>, Hsla)>,
    rects: Vec<(Bounds<Pixels>, Hsla)>,
    size: Size<Pixels>,
}

struct TransformationMatrix {
    sx: f64,
    sy: f64,
    tx: f64,
    ty: f64,
}

impl TransformationMatrix {
    fn transform(&self, x: f64, y: f64) -> (f64, f64) {
        (self.sx * x + self.tx, self.sy * y + self.ty)
    }
}

struct GpuiMathBackend {
    paths: Vec<(Path<Pixels>, Hsla)>,
    rects: Vec<(Bounds<Pixels>, Hsla)>,
    layout_to_canvas: TransformationMatrix,
    color: Hsla,
    width: f64,
    height: f64,
}

impl GpuiMathBackend {
    fn new(width: f64, height: f64, depth: f64, scale: f64) -> Self {
        let total_height = height + depth;
        Self {
            paths: Vec::new(),
            rects: Vec::new(),
            layout_to_canvas: TransformationMatrix {
                sx: scale,
                sy: scale,
                tx: 0.0,
                ty: height * scale,
            },
            color: gpui::black(),
            width: width * scale,
            height: total_height * scale,
        }
    }

    fn size(&self) -> Size<Pixels> {
        size(px(self.width as f32), px(self.height as f32))
    }

    fn into_rendered(self) -> RenderedMath {
        RenderedMath {
            size: self.size(),
            paths: self.paths,
            rects: self.rects,
        }
    }
}

impl rex::render::GraphicsBackend for GpuiMathBackend {
    fn rule(&mut self, pos: rex::render::Cursor, width: f64, height: f64) {
        let (x, y) = self.layout_to_canvas.transform(pos.x, pos.y);
        let scaled_height = height * self.layout_to_canvas.sx.abs();
        let scaled_width = width * self.layout_to_canvas.sx.abs();
        let rect_y = y - scaled_height;
        self.rects.push((
            Bounds::new(
                point(px(x as f32), px(rect_y as f32)),
                size(px(scaled_width as f32), px(scaled_height as f32)),
            ),
            self.color,
        ));
    }

    fn begin_color(&mut self, color: rex::render::RGBA) {
        self.color = Hsla::from(gpui::Rgba {
            r: color.0 as f32 / 255.0,
            g: color.1 as f32 / 255.0,
            b: color.2 as f32 / 255.0,
            a: color.3 as f32 / 255.0,
        });
    }

    fn end_color(&mut self) {
        self.color = gpui::black();
    }
}

impl<'a> rex::render::FontBackend<rex::font::backend::ttf_parser::TtfMathFont<'a>>
    for GpuiMathBackend
{
    fn symbol(
        &mut self,
        pos: rex::render::Cursor,
        gid: rex::font::common::GlyphId,
        scale: f64,
        ctx: &rex::font::backend::ttf_parser::TtfMathFont<'a>,
    ) {
        let (canvas_x, canvas_y) = self.layout_to_canvas.transform(pos.x, pos.y);
        let font_matrix = ctx.font_matrix();
        let glyph_scale = scale * self.layout_to_canvas.sx.abs();

        let ttf_gid: rex_ttf_parser::GlyphId = gid.into();
        let color = self.color;

        let mut outline_builder = GlyphOutlineBuilder {
            path: gpui::PathBuilder::fill(),
            glyph_scale,
            font_matrix,
            canvas_x,
            canvas_y,
        };

        if ctx
            .font()
            .outline_glyph(ttf_gid, &mut outline_builder)
            .is_some()
        {
            if let Ok(path) = outline_builder.path.build() {
                self.paths.push((path, color));
            }
        }
    }
}

impl<'a> rex::render::Backend<rex::font::backend::ttf_parser::TtfMathFont<'a>>
    for GpuiMathBackend
{
}

struct GlyphOutlineBuilder {
    path: gpui::PathBuilder,
    glyph_scale: f64,
    font_matrix: rex_ttf_parser::cff::Matrix,
    canvas_x: f64,
    canvas_y: f64,
}

impl GlyphOutlineBuilder {
    fn transform_point(&self, x: f32, y: f32) -> Point<Pixels> {
        let fx = self.font_matrix.sx as f64 * x as f64 + self.font_matrix.kx as f64 * y as f64
            + self.font_matrix.tx as f64;
        let fy = self.font_matrix.ky as f64 * x as f64 + self.font_matrix.sy as f64 * y as f64
            + self.font_matrix.ty as f64;
        let px_x = self.canvas_x + fx * self.glyph_scale;
        let px_y = self.canvas_y - fy * self.glyph_scale;
        point(px(px_x as f32), px(px_y as f32))
    }
}

impl rex_ttf_parser::OutlineBuilder for GlyphOutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        let pt = self.transform_point(x, y);
        self.path.move_to(pt);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let pt = self.transform_point(x, y);
        self.path.line_to(pt);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let ctrl = self.transform_point(x1, y1);
        let to = self.transform_point(x, y);
        self.path.curve_to(to, ctrl);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let ctrl1 = self.transform_point(x1, y1);
        let ctrl2 = self.transform_point(x2, y2);
        let to = self.transform_point(x, y);
        self.path.cubic_bezier_to(to, ctrl1, ctrl2);
    }

    fn close(&mut self) {
        self.path.close();
    }
}

fn scale_and_offset_path(path: &Path<Pixels>, scale: f32, offset: Point<Pixels>) -> Path<Pixels> {
    let mut new_path = Path::new(point(
        path.bounds.origin.x * scale + offset.x,
        path.bounds.origin.y * scale + offset.y,
    ));
    for vertex in &path.vertices {
        new_path.vertices.push(gpui::PathVertex {
            xy_position: point(
                vertex.xy_position.x * scale + offset.x,
                vertex.xy_position.y * scale + offset.y,
            ),
            st_position: vertex.st_position,
            content_mask: vertex.content_mask.clone(),
        });
    }
    new_path.bounds = Bounds::new(
        point(
            path.bounds.origin.x * scale + offset.x,
            path.bounds.origin.y * scale + offset.y,
        ),
        size(
            path.bounds.size.width * scale,
            path.bounds.size.height * scale,
        ),
    );
    new_path
}

fn render_math_expression(source: &str, _display: bool) -> Result<RenderedMath, String> {
    let face = rex_ttf_parser::Face::parse(MATH_FONT_DATA, 0)
        .map_err(|e| format!("font parse: {e}"))?;
    let font = rex::font::backend::ttf_parser::TtfMathFont::new(face)
        .map_err(|e| format!("math font: {e:?}"))?;

    let renderer = rex::render::Renderer::new();
    let layout = renderer
        .layout(source, &font)
        .map_err(|e| format!("layout: {e:?}"))?;

    let dims = layout.size();
    let mut backend = GpuiMathBackend::new(dims.width, dims.height, dims.depth, 1.0);
    renderer.render(&layout, &mut backend);

    let mut rendered = backend.into_rendered();

    // Compute actual bounding box from rendered paths and rects,
    // since the layout dimensions may underestimate for matrices/large delimiters
    let mut min_x: f32 = 0.0;
    let mut min_y: f32 = 0.0;
    let mut max_x: f32 = rendered.size.width.as_f32();
    let mut max_y: f32 = rendered.size.height.as_f32();
    for (path, _) in &rendered.paths {
        let b = &path.bounds;
        min_x = min_x.min(b.origin.x.as_f32());
        min_y = min_y.min(b.origin.y.as_f32());
        max_x = max_x.max((b.origin.x + b.size.width).as_f32());
        max_y = max_y.max((b.origin.y + b.size.height).as_f32());
    }
    for (rect, _) in &rendered.rects {
        min_x = min_x.min(rect.origin.x.as_f32());
        min_y = min_y.min(rect.origin.y.as_f32());
        max_x = max_x.max((rect.origin.x + rect.size.width).as_f32());
        max_y = max_y.max((rect.origin.y + rect.size.height).as_f32());
    }

    // Offset all paths/rects so the bounding box starts at (0, 0)
    let offset_x = -min_x;
    let offset_y = -min_y;
    if offset_x != 0.0 || offset_y != 0.0 {
        let off = point(px(offset_x), px(offset_y));
        for (path, _) in &mut rendered.paths {
            *path = scale_and_offset_path(path, 1.0, off);
        }
        for (rect, _) in &mut rendered.rects {
            rect.origin.x = rect.origin.x + px(offset_x);
            rect.origin.y = rect.origin.y + px(offset_y);
        }
    }

    rendered.size = size(px(max_x - min_x), px(max_y - min_y));
    Ok(rendered)
}

impl MathState {
    pub(crate) fn clear(&mut self) {
        self.cache.clear();
    }

    pub(crate) fn update(&mut self, parsed: &ParsedMarkdown, cx: &mut Context<Markdown>) {
        let mut needed_keys = Vec::new();
        for (_range, event) in parsed.events.iter() {
            match event {
                MarkdownEvent::InlineMath(source) => {
                    needed_keys.push(MathCacheKey {
                        source: source.clone(),
                        display: false,
                    });
                }
                MarkdownEvent::DisplayMath(source) => {
                    needed_keys.push(MathCacheKey {
                        source: source.clone(),
                        display: true,
                    });
                }
                _ => {}
            }
        }

        for key in &needed_keys {
            if !self.cache.contains_key(key) {
                self.cache.insert(
                    key.clone(),
                    Arc::new(CachedMathExpression::new(key.clone(), cx)),
                );
            }
        }

        let needed_set: std::collections::HashSet<_> = needed_keys.into_iter().collect();
        self.cache.retain(|key, _| needed_set.contains(key));
    }
}

impl CachedMathExpression {
    fn new(key: MathCacheKey, cx: &mut Context<Markdown>) -> Self {
        let result = Arc::new(OnceLock::<Result<RenderedMath, String>>::new());
        let result_clone = result.clone();
        let source = key.source.clone();
        let display = key.display;

        let task = cx.spawn(async move |this, cx| {
            let value = cx
                .background_spawn(async move {
                    render_math_expression(&source, display)
                })
                .await;
            let _ = result_clone.set(value);
            this.update(cx, |_, cx| {
                cx.notify();
            })
            .ok();
        });

        Self {
            result,
            _task: task,
        }
    }
}

pub(crate) fn render_display_math(
    source: &str,
    math_state: &MathState,
    _style: &MarkdownStyle,
    text_color: Hsla,
    font_size: f32,
) -> AnyElement {
    let key = MathCacheKey {
        source: source.to_string(),
        display: true,
    };
    render_math_element(&key, math_state, text_color, true, font_size)
}

pub(crate) fn latex_to_unicode(source: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '\\' {
            let cmd_start = i + 1;
            let mut cmd_end = cmd_start;
            while cmd_end < len && chars[cmd_end].is_ascii_alphabetic() {
                cmd_end += 1;
            }
            let cmd = &source[cmd_start..cmd_end];
            if let Some(sym) = latex_command_to_unicode(cmd) {
                result.push_str(sym);
                i = cmd_end;
                if i < len && chars[i] == ' ' {
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else if chars[i] == '^' {
            i += 1;
            let content = extract_group(&chars, &mut i);
            for ch in content.chars() {
                result.push(to_superscript(ch));
            }
        } else if chars[i] == '_' {
            i += 1;
            let content = extract_group(&chars, &mut i);
            for ch in content.chars() {
                result.push(to_subscript(ch));
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

fn extract_group(chars: &[char], i: &mut usize) -> String {
    if *i < chars.len() && chars[*i] == '{' {
        *i += 1;
        let mut depth = 1;
        let mut content = String::new();
        while *i < chars.len() && depth > 0 {
            if chars[*i] == '{' {
                depth += 1;
            } else if chars[*i] == '}' {
                depth -= 1;
                if depth == 0 {
                    *i += 1;
                    return content;
                }
            }
            content.push(chars[*i]);
            *i += 1;
        }
        content
    } else if *i < chars.len() {
        let ch = chars[*i];
        *i += 1;
        ch.to_string()
    } else {
        String::new()
    }
}

fn to_superscript(ch: char) -> char {
    match ch {
        '0' => '⁰', '1' => '¹', '2' => '²', '3' => '³', '4' => '⁴',
        '5' => '⁵', '6' => '⁶', '7' => '⁷', '8' => '⁸', '9' => '⁹',
        '+' => '⁺', '-' => '⁻', '=' => '⁼', '(' => '⁽', ')' => '⁾',
        'n' => 'ⁿ', 'i' => 'ⁱ',
        'T' => 'ᵀ',
        _ => ch,
    }
}

fn to_subscript(ch: char) -> char {
    match ch {
        '0' => '₀', '1' => '₁', '2' => '₂', '3' => '₃', '4' => '₄',
        '5' => '₅', '6' => '₆', '7' => '₇', '8' => '₈', '9' => '₉',
        '+' => '₊', '-' => '₋', '=' => '₌', '(' => '₍', ')' => '₎',
        'a' => 'ₐ', 'e' => 'ₑ', 'h' => 'ₕ', 'i' => 'ᵢ', 'j' => 'ⱼ',
        'k' => 'ₖ', 'l' => 'ₗ', 'm' => 'ₘ', 'n' => 'ₙ', 'o' => 'ₒ',
        'p' => 'ₚ', 'r' => 'ᵣ', 's' => 'ₛ', 't' => 'ₜ', 'u' => 'ᵤ',
        'v' => 'ᵥ', 'x' => 'ₓ',
        _ => ch,
    }
}

fn latex_command_to_unicode(cmd: &str) -> Option<&'static str> {
    Some(match cmd {
        "alpha" => "α", "beta" => "β", "gamma" => "γ", "delta" => "δ",
        "epsilon" => "ε", "zeta" => "ζ", "eta" => "η", "theta" => "θ",
        "iota" => "ι", "kappa" => "κ", "lambda" => "λ", "mu" => "μ",
        "nu" => "ν", "xi" => "ξ", "pi" => "π", "rho" => "ρ",
        "sigma" => "σ", "tau" => "τ", "upsilon" => "υ", "phi" => "φ",
        "chi" => "χ", "psi" => "ψ", "omega" => "ω",
        "Alpha" => "Α", "Beta" => "Β", "Gamma" => "Γ", "Delta" => "Δ",
        "Theta" => "Θ", "Lambda" => "Λ", "Xi" => "Ξ", "Pi" => "Π",
        "Sigma" => "Σ", "Phi" => "Φ", "Psi" => "Ψ", "Omega" => "Ω",
        "Rightarrow" => "⇒", "Leftarrow" => "⇐", "Leftrightarrow" => "⇔",
        "rightarrow" => "→", "leftarrow" => "←", "leftrightarrow" => "↔",
        "cap" => "∩", "cup" => "∪", "in" => "∈", "notin" => "∉",
        "subset" => "⊂", "supset" => "⊃", "subseteq" => "⊆", "supseteq" => "⊇",
        "perp" => "⊥", "parallel" => "∥",
        "cdot" => "·", "times" => "×", "div" => "÷", "pm" => "±", "mp" => "∓",
        "le" | "leq" => "≤", "ge" | "geq" => "≥", "neq" | "ne" => "≠",
        "approx" => "≈", "equiv" => "≡", "sim" => "∼", "propto" => "∝",
        "infty" => "∞", "partial" => "∂", "nabla" => "∇",
        "forall" => "∀", "exists" => "∃",
        "sum" => "∑", "prod" => "∏", "int" => "∫",
        "sqrt" => "√", "cbrt" => "∛",
        "langle" => "⟨", "rangle" => "⟩",
        "ldots" | "dots" | "cdots" => "…",
        "neg" | "lnot" => "¬", "land" | "wedge" => "∧", "lor" | "vee" => "∨",
        "oplus" => "⊕", "otimes" => "⊗",
        "emptyset" | "varnothing" => "∅",
        "mathbb" => "", "mathbf" => "", "mathrm" => "", "mathcal" => "",
        "text" => "", "textbf" => "", "textrm" => "",
        "left" | "right" | "bigl" | "bigr" => "",
        _ => return None,
    })
}

pub(crate) fn render_inline_math(
    source: &str,
    math_state: &MathState,
    _style: &MarkdownStyle,
    text_color: Hsla,
    font_size: f32,
) -> AnyElement {
    let key = MathCacheKey {
        source: source.to_string(),
        display: false,
    };
    render_math_element(&key, math_state, text_color, false, font_size)
}

fn render_math_element(
    key: &MathCacheKey,
    math_state: &MathState,
    text_color: Hsla,
    display: bool,
    font_size: f32,
) -> AnyElement {
    let cached = math_state.cache.get(key);
    let scale = font_size / 10.0;

    if let Some(result) = cached.and_then(|c| c.result.get()) {
        match result {
            Ok(rendered) => {
                let rendered = rendered.clone();
                let scaled_w = rendered.size.width * scale;
                let scaled_h = rendered.size.height * scale;
                div()
                    .child(
                        gpui::canvas(
                            {
                                move |_bounds, _window, _cx| {
                                    size(scaled_w, scaled_h)
                                }
                            },
                            {
                                let rendered = rendered.clone();
                                move |bounds, _size, window, _cx| {
                                    let offset = bounds.origin;
                                    for (rect, color) in &rendered.rects {
                                        let offset_rect = Bounds::new(
                                            point(
                                                offset.x + rect.origin.x * scale,
                                                offset.y + rect.origin.y * scale,
                                            ),
                                            size(
                                                rect.size.width * scale,
                                                rect.size.height * scale,
                                            ),
                                        );
                                        let paint_color = if *color == gpui::black() {
                                            text_color
                                        } else {
                                            *color
                                        };
                                        window.paint_quad(quad(
                                            offset_rect,
                                            gpui::Corners::default(),
                                            paint_color,
                                            gpui::Edges::default(),
                                            gpui::Hsla::transparent_black(),
                                            gpui::BorderStyle::default(),
                                        ));
                                    }
                                    for (path, color) in &rendered.paths {
                                        let paint_color = if *color == gpui::black() {
                                            text_color
                                        } else {
                                            *color
                                        };
                                        let scaled_path = scale_and_offset_path(path, scale, offset);
                                        window.paint_path(scaled_path, paint_color);
                                    }
                                }
                            },
                        )
                        .w(scaled_w)
                        .h(scaled_h),
                    )
                    .into_any_element()
            }
            Err(_error) => {
                let fallback_text = if display {
                    format!("$${}$$", key.source)
                } else {
                    format!("${}$", key.source)
                };
                div()
                    .child(SharedString::from(fallback_text))
                    .into_any_element()
            }
        }
    } else {
        let fallback_text = if display {
            format!("$${}$$", key.source)
        } else {
            format!("${}$", key.source)
        };
        div()
            .child(SharedString::from(fallback_text))
            .into_any_element()
    }
}
