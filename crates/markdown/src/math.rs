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
                sy: -scale,
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

    Ok(backend.into_rendered())
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
                let container = if display {
                    div().w_full().flex().justify_center().py_1()
                } else {
                    div()
                };
                container
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
