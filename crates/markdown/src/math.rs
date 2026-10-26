use collections::HashMap;
use gpui::{AnyElement, Context, Pixels, RenderImage, StyledText, Task, div, px};
use ratex_types::display_item::{DisplayItem, DisplayList};
use settings::Settings;
use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::{Arc, Mutex, OnceLock};
use theme_settings::ThemeSettings;
use ui::prelude::*;

use super::{Markdown, ParsedMarkdown};
use crate::parser::MarkdownEvent;

#[path = "math_svg.rs"]
mod math_svg;

/// A math expression extracted from parsed markdown events.
#[derive(Clone, Debug)]
pub(crate) struct ParsedMarkdownMathExpression {
    /// Byte range of the full math expression in the source (including delimiters).
    #[allow(dead_code)]
    pub(crate) source_range: Range<usize>,
    /// The raw LaTeX content (without delimiters).
    pub(crate) latex: SharedString,
    /// Whether this is a display math expression (`$$...$$` / `\[...\]`).
    pub(crate) is_display: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MathCacheKey {
    latex: SharedString,
    font_size_bits: u32,
}

impl MathCacheKey {
    fn new(latex: SharedString, font_size: f32) -> Self {
        Self {
            latex,
            font_size_bits: font_size.to_bits(),
        }
    }
}

type MathExpressionCache = HashMap<MathCacheKey, Arc<CachedMathExpression>>;

#[derive(Default, Clone)]
pub(crate) struct MathState {
    cache: MathExpressionCache,
    order: Vec<MathCacheKey>,
    font_size: f32,
    /// Ascent of the buffer font at `font_size`, in pixels.
    /// Used to align inline math baselines with surrounding text.
    text_ascent: f32,
}

struct MathRenderResult {
    image: Arc<RenderImage>,
    baseline_y: f32,
}

struct CachedMathExpression {
    display_tree: Arc<OnceLock<anyhow::Result<DisplayList>>>,
    rendered: Arc<Mutex<Option<anyhow::Result<MathRenderResult>>>>,
    theme_color: Arc<Mutex<gpui::Hsla>>,
    _task: Arc<Mutex<Option<Task<()>>>>,
}

/// Convert a GPUI Hsla color to a ratex Color (RGBA f32).
fn gpui_color_to_ratex(color: gpui::Hsla) -> ratex_types::color::Color {
    let rgba = gpui::Rgba::from(color);
    ratex_types::color::Color::new(rgba.r, rgba.g, rgba.b, rgba.a)
}

/// Helper to identify unstyled default items in ratex layout (black or transparent).
fn is_default_placeholder_color(c: &ratex_types::color::Color) -> bool {
    c.r == 0.0 && c.g == 0.0 && c.b == 0.0
}

/// Replace default placeholder colors in a DisplayList with the given theme color,
/// while preserving explicit custom colors (e.g. \textcolor{red}{x} or \colorbox).
fn recolor_display_list(
    display_list: &DisplayList,
    color: &ratex_types::color::Color,
) -> DisplayList {
    DisplayList {
        items: display_list
            .items
            .iter()
            .map(|item| match item {
                DisplayItem::GlyphPath {
                    x,
                    y,
                    scale,
                    font,
                    char_code,
                    color: item_color,
                } => DisplayItem::GlyphPath {
                    x: *x,
                    y: *y,
                    scale: *scale,
                    font: font.clone(),
                    char_code: *char_code,
                    color: if is_default_placeholder_color(item_color) {
                        *color
                    } else {
                        *item_color
                    },
                },
                DisplayItem::Line {
                    x,
                    y,
                    width,
                    thickness,
                    dashed,
                    color: item_color,
                } => DisplayItem::Line {
                    x: *x,
                    y: *y,
                    width: *width,
                    thickness: *thickness,
                    dashed: *dashed,
                    color: if is_default_placeholder_color(item_color) {
                        *color
                    } else {
                        *item_color
                    },
                },
                DisplayItem::Rect {
                    x,
                    y,
                    width,
                    height,
                    color: item_color,
                } => DisplayItem::Rect {
                    x: *x,
                    y: *y,
                    width: *width,
                    height: *height,
                    color: if is_default_placeholder_color(item_color) {
                        *color
                    } else {
                        *item_color
                    },
                },
                DisplayItem::Path {
                    x,
                    y,
                    commands,
                    fill,
                    color: item_color,
                } => DisplayItem::Path {
                    x: *x,
                    y: *y,
                    commands: commands.clone(),
                    fill: *fill,
                    color: if is_default_placeholder_color(item_color) {
                        *color
                    } else {
                        *item_color
                    },
                },
            })
            .collect(),
        width: display_list.width,
        height: display_list.height,
        depth: display_list.depth,
    }
}

impl MathState {
    pub(crate) fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    pub(crate) fn font_size(&self) -> f32 {
        self.font_size
    }

    /// Re-color every cached expression with the current theme color asynchronously
    /// in the background, avoiding UI thread blocking.
    pub(crate) fn retheme(&mut self, cx: &mut Context<Markdown>) {
        let text_color = cx.theme().colors().text;
        let svg_renderer = cx.svg_renderer();
        let font_size = self.font_size;
        for cached in self.cache.values() {
            cached.recolor_and_render(font_size, text_color, svg_renderer.clone(), cx);
        }
    }

    pub(crate) fn update(
        &mut self,
        parsed: &ParsedMarkdown,
        font_size: f32,
        cx: &mut Context<Markdown>,
    ) {
        self.font_size = font_size;

        // Compute the baseline of the surrounding text line using the UI font
        // and standard line padding so inline math baselines align with text.
        let theme_settings = ThemeSettings::get_global(cx);
        let font = gpui::Font {
            family: theme_settings.ui_font.family.clone(),
            fallbacks: theme_settings.ui_font.fallbacks.clone(),
            features: theme_settings.ui_font.features.clone(),
            style: Default::default(),
            weight: theme_settings.ui_font.weight,
        };
        let text_system = cx.text_system();
        let font_id = text_system.resolve_font(&font);
        let base_font_size = font_size / 1.21;
        let ascent = text_system
            .ascent(font_id, Pixels::from(base_font_size))
            .as_f32();
        let descent = text_system
            .descent(font_id, Pixels::from(base_font_size))
            .as_f32();
        let line_height = base_font_size * 1.3;
        let padding_top = (line_height - ascent - descent) / 2.0;
        self.text_ascent = padding_top + ascent;

        let mut new_order = Vec::new();
        for expr in parsed.math_expressions.values() {
            new_order.push(MathCacheKey::new(expr.latex.clone(), font_size));
        }

        for key in &new_order {
            if !self.cache.contains_key(key) {
                self.cache.insert(
                    key.clone(),
                    Arc::new(CachedMathExpression::new(key.latex.clone(), font_size, cx)),
                );
            }
        }

        let new_order_set: std::collections::HashSet<_> = new_order.iter().cloned().collect();
        self.cache.retain(|key, _| new_order_set.contains(key));
        self.order = new_order;
    }
}

impl CachedMathExpression {
    fn new(latex: SharedString, font_size: f32, cx: &mut Context<Markdown>) -> Self {
        let display_tree = Arc::new(OnceLock::new());
        let rendered = Arc::new(Mutex::new(None));
        let theme_color = Arc::new(Mutex::new(cx.theme().colors().text));
        let svg_renderer = cx.svg_renderer();

        let dt = display_tree.clone();
        let rd = rendered.clone();
        let tc = theme_color.clone();
        let parse_task = cx.spawn(async move |this, cx| {
            // Phase 1 — parse + layout in background, store neutral display tree
            let parse_result = cx
                .background_spawn({
                    let latex = latex.clone();
                    async move {
                        let nodes = ratex_parser::parse(&latex)
                            .map_err(|e| anyhow::anyhow!("LaTeX parse error: {}", e))?;
                        let layout = ratex_layout::layout(&nodes, &Default::default());
                        let dl = ratex_layout::to_display_list(&layout);
                        Ok(dl)
                    }
                })
                .await;
            let _ = dt.set(parse_result);

            // Phase 2 — recolor with latest theme color in background and render SVG
            if let Some(Ok(dl)) = dt.get() {
                let current_color = *tc.lock().unwrap();
                let dl = dl.clone();
                let svg_renderer = svg_renderer.clone();

                let result = cx
                    .background_spawn(async move {
                        let ratex_color = gpui_color_to_ratex(current_color);
                        let recolored = recolor_display_list(&dl, &ratex_color);
                        let svg = math_svg::display_list_to_svg(&recolored, font_size);
                        let baseline_y = svg.baseline_y;
                        let image = svg_renderer
                            .render_single_frame(&svg.svg_bytes, 1.0)
                            .map_err(|e| anyhow::anyhow!("SVG render error: {}", e));
                        image.map(|img| MathRenderResult {
                            image: img,
                            baseline_y,
                        })
                    })
                    .await;
                *rd.lock().unwrap() = Some(result);
            }

            this.update(cx, |_, cx| cx.notify()).ok();
        });

        Self {
            display_tree,
            rendered,
            theme_color,
            _task: Arc::new(Mutex::new(Some(parse_task))),
        }
    }

    fn recolor_and_render(
        &self,
        font_size: f32,
        new_color: gpui::Hsla,
        svg_renderer: gpui::SvgRenderer,
        cx: &mut Context<Markdown>,
    ) {
        *self.theme_color.lock().unwrap() = new_color;
        let dt = self.display_tree.clone();
        let rd = self.rendered.clone();
        let tc = self.theme_color.clone();

        let task = cx.spawn(async move |this, cx| {
            let Some(Ok(display_list)) = dt.get() else {
                return;
            };
            let current_color = *tc.lock().unwrap();
            let dl = display_list.clone();
            let svg_renderer = svg_renderer.clone();

            let result = cx
                .background_spawn(async move {
                    let ratex_color = gpui_color_to_ratex(current_color);
                    let recolored = recolor_display_list(&dl, &ratex_color);
                    let svg = math_svg::display_list_to_svg(&recolored, font_size);
                    let baseline_y = svg.baseline_y;
                    let image = svg_renderer
                        .render_single_frame(&svg.svg_bytes, 1.0)
                        .map_err(|e| anyhow::anyhow!("SVG render error: {}", e));
                    image.map(|img| MathRenderResult {
                        image: img,
                        baseline_y,
                    })
                })
                .await;
            *rd.lock().unwrap() = Some(result);
            this.update(cx, |_, cx| cx.notify()).ok();
        });

        *self._task.lock().unwrap() = Some(task);
    }

    fn rendered_data(&self) -> Option<anyhow::Result<(Arc<RenderImage>, f32)>> {
        self.rendered.lock().unwrap().as_ref().map(|r| match r {
            Ok(mrr) => Ok((mrr.image.clone(), mrr.baseline_y)),
            Err(e) => Err(anyhow::anyhow!("{:#}", e)),
        })
    }
}

/// Renders a math expression as a GPUI element.
///
/// Returns the rendered image if available, or a fallback element showing
/// the raw LaTeX source styled as inline code.
pub(crate) fn render_math_expression(
    parsed: &ParsedMarkdownMathExpression,
    math_state: &MathState,
    text_baseline: Option<f32>,
) -> AnyElement {
    let key = MathCacheKey::new(parsed.latex.clone(), math_state.font_size);
    let render_result = math_state
        .cache
        .get(&key)
        .and_then(|cached| cached.rendered_data());

    match render_result {
        Some(Ok((image, baseline_y))) => {
            if parsed.is_display {
                div()
                    .flex()
                    .w_full()
                    .justify_center()
                    .my_2()
                    .child(gpui::img(image))
                    .into_any_element()
            } else {
                let baseline = text_baseline.unwrap_or(math_state.text_ascent);
                let shift = baseline - baseline_y;
                div()
                    .child(gpui::img(image).mt(px(shift)))
                    .into_any_element()
            }
        }
        Some(Err(_)) | None => {
            let label = if parsed.is_display {
                format!("$${}$$", parsed.latex)
            } else {
                format!("${}$", parsed.latex)
            };
            div().child(StyledText::new(label)).into_any_element()
        }
    }
}

/// Extracts math expressions from parsed markdown events.
///
/// Walks the event list looking for `MarkdownEvent::InlineMath` and
/// `MarkdownEvent::DisplayMath` events, collecting them into a map keyed
/// by source offset for O(log n) lookup during rendering.
pub(crate) fn extract_math_expressions(
    events: &[(Range<usize>, MarkdownEvent)],
) -> BTreeMap<usize, ParsedMarkdownMathExpression> {
    let mut expressions = BTreeMap::default();

    for (source_range, event) in events {
        match event {
            MarkdownEvent::InlineMath(latex) => {
                expressions.insert(
                    source_range.start,
                    ParsedMarkdownMathExpression {
                        source_range: source_range.clone(),
                        latex: SharedString::from(latex.trim()),
                        is_display: false,
                    },
                );
            }
            MarkdownEvent::DisplayMath(latex) => {
                expressions.insert(
                    source_range.start,
                    ParsedMarkdownMathExpression {
                        source_range: source_range.clone(),
                        latex: SharedString::from(latex.trim()),
                        is_display: true,
                    },
                );
            }
            _ => {}
        }
    }

    expressions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{MarkdownEvent, parse_markdown_with_options};

    #[test]
    fn test_extract_inline_math() {
        let input = "Hello $x^2$ world";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        assert_eq!(expressions.len(), 1);
        let expr = expressions.values().next().unwrap();
        assert_eq!(expr.latex.as_ref(), "x^2");
        assert!(!expr.is_display);
    }

    #[test]
    fn test_extract_display_math() {
        let input = "$$\\frac{a}{b}$$";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        assert_eq!(expressions.len(), 1);
        let expr = expressions.values().next().unwrap();
        assert_eq!(expr.latex.as_ref(), "\\frac{a}{b}");
        assert!(expr.is_display);
    }

    #[test]
    fn test_extract_multiple_expressions() {
        let input = "Let $a$ and $b$ be variables. Then $$a + b = c$$";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        assert_eq!(expressions.len(), 3);
    }

    #[test]
    fn test_no_math_in_code_blocks() {
        let input = "```\n$x^2$\n```";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        assert!(expressions.is_empty());
    }

    #[test]
    fn test_no_math_in_inline_code() {
        let input = "Use `$x^2$` to represent a parabola";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        assert!(expressions.is_empty());
    }

    #[test]
    fn test_math_with_complex_latex() {
        let input = "$\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}$";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        assert_eq!(expressions.len(), 1);
        let expr = expressions.values().next().unwrap();
        assert_eq!(
            expr.latex.as_ref(),
            "\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}"
        );
    }

    #[test]
    fn test_math_source_range_includes_delimiters() {
        let input = "Hello $x^2$ world";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        let expr = expressions.values().next().unwrap();
        // Source range should include the $ delimiters
        assert_eq!(&input[expr.source_range.clone()], "$x^2$");
    }

    #[test]
    fn test_math_in_paragraph_with_text() {
        let input = "The equation $E = mc^2$ is famous.";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let events: Vec<_> = parsed.events.iter().map(|(_, e)| e.clone()).collect();
        // Should have: RootStart, Paragraph, Text, InlineMath, Text, Paragraph, RootEnd
        let has_inline_math = events
            .iter()
            .any(|e| matches!(e, MarkdownEvent::InlineMath(_)));
        let has_text = events.iter().any(|e| matches!(e, MarkdownEvent::Text));
        assert!(has_inline_math, "should have inline math");
        assert!(has_text, "should have surrounding text");
    }

    #[test]
    fn test_display_math_standalone() {
        let input = "$$\n\\sum_{i=1}^n i = \\frac{n(n+1)}{2}\n$$";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        assert_eq!(expressions.len(), 1);
        let expr = expressions.values().next().unwrap();
        assert!(expr.is_display);
        assert!(expr.latex.as_ref().contains("\\sum"));
    }

    #[test]
    fn test_display_math_multiline_trimmed() {
        let input = "$$\n\\int_0^\\infty e^{-x^2}dx=\\frac{\\sqrt\\pi}{2}\n$$";
        let parsed = parse_markdown_with_options(input, false, false, false);
        let expressions = extract_math_expressions(&parsed.events);
        assert_eq!(expressions.len(), 1);
        let expr = expressions.values().next().unwrap();
        assert!(expr.is_display);
        assert_eq!(
            expr.latex.as_ref(),
            "\\int_0^\\infty e^{-x^2}dx=\\frac{\\sqrt\\pi}{2}"
        );
    }

    #[test]
    fn test_extract_math_from_empty_events() {
        let events: &[(Range<usize>, MarkdownEvent)] = &[];
        let expressions = extract_math_expressions(events);
        assert!(expressions.is_empty());
    }

    #[test]
    fn test_cache_key_includes_font_size() {
        let a = MathCacheKey::new("x^2".into(), 14.0);
        let b = MathCacheKey::new("x^2".into(), 18.0);
        assert_ne!(a, b);

        let c = MathCacheKey::new("x^2".into(), 14.0);
        assert_eq!(a, c);
    }
}
