use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::sync::{Arc, OnceLock};

use collections::BTreeMap;
use gpui::{
    AppContext, Bounds, Context, FillOptions, FillRule, Hsla, PathBuilder, PathStyle, Pixels, Rgba,
    SharedString, Task, TextRun, TextSystem, Window, canvas, fill, font, px,
};
use ratex_font::{FontId, katex_ttf_glyph_char};
use ratex_layout::LayoutOptions;
use ratex_types::{Color as RatexColor, DisplayItem, DisplayList, MathStyle, PathCommand};

use anyhow::Result;
use ui::{AnyElement, IntoElement, ParentElement, Styled, div};
use util::ResultExt;

use crate::parser::MarkdownEvent;
use crate::{Markdown, MarkdownStyle, ParsedMarkdown};

type MathExpressionCache = HashMap<ParsedMathExpressionContents, Arc<CachedMathExpression>>;

static FONTS_REGISTERED: OnceLock<()> = OnceLock::new();

const KATEX_FONTS: &[(FontId, &str)] = &[
    (FontId::MainRegular, "KaTeX_Main-Regular.ttf"),
    (FontId::MainBold, "KaTeX_Main-Bold.ttf"),
    (FontId::MainItalic, "KaTeX_Main-Italic.ttf"),
    (FontId::MainBoldItalic, "KaTeX_Main-BoldItalic.ttf"),
    (FontId::MathItalic, "KaTeX_Math-Italic.ttf"),
    (FontId::MathBoldItalic, "KaTeX_Math-BoldItalic.ttf"),
    (FontId::AmsRegular, "KaTeX_AMS-Regular.ttf"),
    (FontId::CaligraphicRegular, "KaTeX_Caligraphic-Regular.ttf"),
    (FontId::FrakturRegular, "KaTeX_Fraktur-Regular.ttf"),
    (FontId::FrakturBold, "KaTeX_Fraktur-Bold.ttf"),
    (FontId::SansSerifRegular, "KaTeX_SansSerif-Regular.ttf"),
    (FontId::SansSerifBold, "KaTeX_SansSerif-Bold.ttf"),
    (FontId::SansSerifItalic, "KaTeX_SansSerif-Italic.ttf"),
    (FontId::ScriptRegular, "KaTeX_Script-Regular.ttf"),
    (FontId::TypewriterRegular, "KaTeX_Typewriter-Regular.ttf"),
    (FontId::Size1Regular, "KaTeX_Size1-Regular.ttf"),
    (FontId::Size2Regular, "KaTeX_Size2-Regular.ttf"),
    (FontId::Size3Regular, "KaTeX_Size3-Regular.ttf"),
    (FontId::Size4Regular, "KaTeX_Size4-Regular.ttf"),
];

fn katex_gpui_font(font_name: &str) -> gpui::Font {
    match font_name {
        "Main-Bold" => font("KaTeX_Main").bold(),
        "Main-Italic" => font("KaTeX_Main").italic(),
        "Main-BoldItalic" => font("KaTeX_Main").bold().italic(),
        "Math-Italic" => font("KaTeX_Math").italic(),
        "Math-BoldItalic" => font("KaTeX_Math").bold().italic(),
        "AMS-Regular" => font("KaTeX_AMS"),
        "Caligraphic-Regular" => font("KaTeX_Caligraphic"),
        "Fraktur-Regular" => font("KaTeX_Fraktur"),
        "Fraktur-Bold" => font("KaTeX_Fraktur").bold(),
        "SansSerif-Regular" => font("KaTeX_SansSerif"),
        "SansSerif-Bold" => font("KaTeX_SansSerif").bold(),
        "SansSerif-Italic" => font("KaTeX_SansSerif").italic(),
        "Script-Regular" => font("KaTeX_Script"),
        "Typewriter-Regular" => font("KaTeX_Typewriter"),
        "Size1-Regular" => font("KaTeX_Size1"),
        "Size2-Regular" => font("KaTeX_Size2"),
        "Size3-Regular" => font("KaTeX_Size3"),
        "Size4-Regular" => font("KaTeX_Size4"),
        _ => font("KaTeX_Main"),
    }
}

fn is_system_fallback_font(font_id: FontId) -> bool {
    matches!(
        font_id,
        FontId::CjkRegular | FontId::CjkFallback | FontId::EmojiFallback
    )
}

fn register_katex_fonts(text_system: &TextSystem) {
    let fonts = KATEX_FONTS
        .iter()
        .filter_map(|(_, filename)| ratex_katex_fonts::ttf_bytes(filename))
        .collect::<Vec<_>>();

    if fonts.is_empty() {
        return;
    }

    if let Err(error) = text_system.add_fonts(fonts) {
        log::debug!("failed to register KaTeX fonts: {error}");
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedMathExpression {
    pub(crate) contents: ParsedMathExpressionContents,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ParsedMathExpressionContents {
    pub(crate) contents: SharedString,
    pub(crate) display_mode: bool,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct MathLayoutMetrics {
    pub(crate) width: Pixels,
    pub(crate) ascent: Pixels,
    pub(crate) descent: Pixels,
}

impl MathLayoutMetrics {
    fn height(self) -> Pixels {
        self.ascent + self.descent
    }
}

#[derive(Default, Clone)]
pub(crate) struct MathState {
    cache: MathExpressionCache,
    order: Vec<ParsedMathExpressionContents>,
}

impl MathState {
    pub(crate) fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    fn get_fallback(
        idx: usize,
        old_order: &[ParsedMathExpressionContents],
        new_order_len: usize,
        cache: &MathExpressionCache,
    ) -> Option<Arc<DisplayList>> {
        if old_order.len() != new_order_len {
            return None;
        }

        old_order.get(idx).and_then(|old_content| {
            cache.get(old_content).and_then(|old_cached| {
                old_cached
                    .display_list
                    .get()
                    .and_then(|result| result.as_ref().ok().cloned())
                    .or_else(|| old_cached.fallback.clone())
            })
        })
    }

    pub(crate) fn update(&mut self, parsed: &ParsedMarkdown, cx: &mut Context<Markdown>) {
        FONTS_REGISTERED.get_or_init(|| {
            register_katex_fonts(cx.text_system());
        });
        let mut new_order = Vec::new();
        for math_expression in parsed.math_expressions.values() {
            new_order.push(math_expression.contents.clone());
        }

        for (idx, new_content) in new_order.iter().enumerate() {
            if !self.cache.contains_key(new_content) {
                let fallback = Self::get_fallback(idx, &self.order, new_order.len(), &self.cache);
                self.cache.insert(
                    new_content.clone(),
                    Arc::new(CachedMathExpression::new(new_content.clone(), fallback, cx)),
                );
            }
        }

        let new_order_set: HashSet<_> = new_order.iter().cloned().collect();
        self.cache
            .retain(|content, _| new_order_set.contains(content));
        self.order = new_order;
    }
}

fn parse_and_layout(latex: &str, display_mode: bool) -> Result<Arc<DisplayList>> {
    let ast = ratex_parser::parse(latex)?;
    let options = LayoutOptions {
        style: if display_mode {
            MathStyle::Display
        } else {
            MathStyle::Text
        },
        color: RatexColor::BLACK,
        ..LayoutOptions::default()
    };
    let layout_box = ratex_layout::layout(&ast, &options);
    let display_list = ratex_layout::to_display_list(&layout_box);
    Ok(Arc::new(display_list))
}

struct CachedMathExpression {
    display_list: Arc<OnceLock<anyhow::Result<Arc<DisplayList>>>>,
    fallback: Option<Arc<DisplayList>>,
    _task: Task<()>,
}

impl CachedMathExpression {
    fn new(
        contents: ParsedMathExpressionContents,
        fallback: Option<Arc<DisplayList>>,
        cx: &mut Context<Markdown>,
    ) -> Self {
        let display_list = Arc::new(OnceLock::<anyhow::Result<Arc<DisplayList>>>::new());
        let display_list_clone = display_list.clone();

        let task = cx.spawn(async move |this, cx| {
            let value = cx
                .background_spawn(async move {
                    parse_and_layout(&contents.contents, contents.display_mode)
                })
                .await;
            // `set` returns `Result<(), anyhow::Result<...>>` — the
            // outer Err carries the value we tried to set, so
            // `log_err`'s Display bound doesn't apply directly.
            if let Err(value) = display_list_clone.set(value) {
                log::error!("display list set failed: {value:?}");
            }
            this.update(cx, |_, cx| {
                cx.notify();
            })
            .ok();
        });

        Self {
            display_list,
            fallback,
            _task: task,
        }
    }
}

pub(crate) fn extract_math_expressions(
    source: &str,
    events: &[(Range<usize>, MarkdownEvent)],
) -> BTreeMap<usize, ParsedMathExpression> {
    let mut math_expressions = BTreeMap::default();

    for (source_range, event) in events {
        let display_mode = match event {
            MarkdownEvent::InlineMath => false,
            MarkdownEvent::DisplayMath => true,
            _ => continue,
        };

        let contents = &source[source_range.clone()];
        let inner = strip_math_delimiters(contents, display_mode);
        if inner.trim().is_empty() {
            continue;
        }
        math_expressions.insert(
            source_range.start,
            ParsedMathExpression {
                contents: ParsedMathExpressionContents {
                    contents: inner.into(),
                    display_mode,
                },
            },
        );
    }

    math_expressions
}

fn strip_math_delimiters(latex: &str, display_mode: bool) -> String {
    let s = latex.trim();
    if display_mode {
        s.strip_prefix("$$")
            .and_then(|s| s.strip_suffix("$$"))
            .unwrap_or(s)
    } else {
        s.strip_prefix('$')
            .and_then(|s| s.strip_suffix('$'))
            .unwrap_or(s)
    }
    .to_string()
}

fn math_color_to_hsla(color: &RatexColor, default_color: Hsla) -> Hsla {
    if *color == RatexColor::BLACK {
        default_color
    } else {
        Hsla::from(Rgba {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        })
    }
}

fn resolved_font_matches(window: &Window, font_id: gpui::FontId, expected: &gpui::Font) -> bool {
    window
        .text_system()
        .get_font_for_id(font_id)
        .map(|resolved| {
            resolved.family == expected.family
                && resolved.weight == expected.weight
                && resolved.style == expected.style
        })
        .unwrap_or(false)
}

fn paint_display_item(
    item: &DisplayItem,
    origin: gpui::Point<Pixels>,
    font_size: Pixels,
    default_color: Hsla,
    window: &mut Window,
) {
    let display_offset = |value: f64| px(value as f32 * font_size.as_f32());
    let display_point =
        |x: f64, y: f64| gpui::point(origin.x + display_offset(x), origin.y + display_offset(y));

    match item {
        DisplayItem::GlyphPath {
            x,
            y,
            scale,
            font,
            char_code,
            color,
        } => {
            let mut origin = display_point(*x, *y);
            let em = display_offset(*scale);

            let font_id = FontId::parse(font).unwrap_or(FontId::MainRegular);
            let (ch, font, require_katex_font) = if is_system_fallback_font(font_id) {
                let Some(ch) = char::from_u32(*char_code) else {
                    return;
                };
                (ch, window.text_style().font(), false)
            } else {
                let ch = katex_ttf_glyph_char(font_id, *char_code);
                (ch, katex_gpui_font(font), true)
            };

            let text = ch.to_string();
            let run = TextRun {
                len: text.len(),
                font: font.clone(),
                color: math_color_to_hsla(color, default_color),
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let line = window
                .text_system()
                .shape_line(text.into(), em, &[run], None);
            let Some(run) = line.runs.first() else {
                return;
            };
            if require_katex_font && !resolved_font_matches(window, run.font_id, &font) {
                return;
            }
            let Some(glyph) = run.glyphs.first() else {
                return;
            };
            origin = origin + gpui::point(glyph.position.x, px(0.));
            //NOTE: We ignore glyph painting errors here, as they can be caused by invalid font glyphs, and we don't want to crash the entire rendering because of that.
            if glyph.is_emoji {
                window
                    .paint_emoji(origin, run.font_id, glyph.id, em)
                    .log_with_level(log::Level::Debug);
            } else {
                window
                    .paint_glyph(
                        origin,
                        run.font_id,
                        glyph.id,
                        em,
                        math_color_to_hsla(color, default_color),
                    )
                    .log_with_level(log::Level::Debug);
            }
        }

        DisplayItem::Line {
            x,
            y,
            width,
            thickness,
            color,
            dashed,
        } => {
            let thickness_px = px((*thickness as f32 * font_size.as_f32()).max(1.0));
            let line_center = display_point(*x, *y);
            let line_top_left = gpui::point(
                line_center.x,
                line_center.y - px(thickness_px.as_f32() / 2.0),
            );
            if *dashed {
                let end = display_point(*x + *width, *y);
                let mut builder = PathBuilder::stroke(thickness_px).dash_array(&[px(4.), px(2.)]);
                builder.move_to(line_center);
                builder.line_to(end);
                if let Ok(path) = builder.build() {
                    window.paint_path(path, math_color_to_hsla(color, default_color));
                }
            } else {
                let bounds = Bounds::new(
                    line_top_left,
                    gpui::size(display_offset(*width), thickness_px),
                );
                window.paint_quad(fill(bounds, math_color_to_hsla(color, default_color)));
            }
        }
        DisplayItem::Rect {
            x,
            y,
            width,
            height,
            color,
        } => {
            let rect_origin = display_point(*x, *y);
            let width = px(display_offset(*width).as_f32().max(1.0));
            let height = px(display_offset(*height).as_f32().max(1.0));
            let bounds = Bounds::new(rect_origin, gpui::size(width, height));
            window.paint_quad(fill(bounds, math_color_to_hsla(color, default_color)));
        }
        DisplayItem::Path {
            x,
            y,
            commands,
            fill,
            color,
        } => {
            let path_point =
                |command_x: f64, command_y: f64| display_point(*x + command_x, *y + command_y);
            let mut paint_path_commands = |commands: &[PathCommand]| {
                let mut builder = if *fill {
                    PathBuilder::fill().with_style(PathStyle::Fill(
                        FillOptions::default().with_fill_rule(FillRule::EvenOdd),
                    ))
                } else {
                    // NOTE: define const
                    PathBuilder::stroke(px(1.5))
                };
                for cmd in commands {
                    match cmd {
                        PathCommand::MoveTo { x, y } => {
                            builder.move_to(path_point(*x, *y));
                        }
                        PathCommand::LineTo { x, y } => {
                            builder.line_to(path_point(*x, *y));
                        }
                        PathCommand::QuadTo { x1, y1, x, y } => {
                            builder.curve_to(path_point(*x, *y), path_point(*x1, *y1));
                        }
                        PathCommand::CubicTo {
                            x1,
                            y1,
                            x2,
                            y2,
                            x,
                            y,
                        } => {
                            builder.cubic_bezier_to(
                                path_point(*x, *y),
                                path_point(*x1, *y1),
                                path_point(*x2, *y2),
                            );
                        }
                        PathCommand::Close => builder.close(),
                    }
                }
                if let Ok(path) = builder.build() {
                    window.paint_path(path, math_color_to_hsla(color, default_color));
                }
            };

            if *fill {
                let mut start = 0;
                for index in 1..commands.len() {
                    if matches!(commands[index], PathCommand::MoveTo { .. }) {
                        paint_path_commands(&commands[start..index]);
                        start = index;
                    }
                }
                if start < commands.len() {
                    paint_path_commands(&commands[start..]);
                }
            } else {
                paint_path_commands(commands);
            }
        }
    }
}

fn display_list_metrics(display_list: &DisplayList, font_size: Pixels) -> MathLayoutMetrics {
    MathLayoutMetrics {
        width: px(display_list.width as f32 * font_size.as_f32()),
        ascent: px(display_list.height as f32 * font_size.as_f32()),
        descent: px(display_list.depth as f32 * font_size.as_f32()),
    }
}

pub(crate) fn math_layout_metrics(
    expr: &ParsedMathExpression,
    math_state: &MathState,
    font_size: Pixels,
) -> Option<MathLayoutMetrics> {
    let cached = math_state.cache.get(&expr.contents)?;
    let display_list = cached
        .display_list
        .get()
        .and_then(|result| result.as_ref().ok().cloned())
        .or_else(|| cached.fallback.clone())?;

    Some(display_list_metrics(&display_list, font_size))
}

pub(crate) fn paint_math_expression_at(
    contents: &ParsedMathExpressionContents,
    math_state: &MathState,
    bounds: Bounds<Pixels>,
    font_size: Pixels,
    default_color: Hsla,
    window: &mut Window,
) {
    let Some(cached) = math_state.cache.get(contents) else {
        return;
    };
    let display_list = cached
        .display_list
        .get()
        .and_then(|result| result.as_ref().ok().cloned())
        .or_else(|| cached.fallback.clone());
    let Some(display_list) = display_list else {
        return;
    };

    for item in &display_list.items {
        paint_display_item(item, bounds.origin, font_size, default_color, window);
    }
}

pub(crate) fn render_math_expression(
    expr: &ParsedMathExpression,
    math_state: &MathState,
    font_size: Pixels,
    style: &MarkdownStyle,
) -> AnyElement {
    let cached = math_state.cache.get(&expr.contents);
    let display_list = cached.and_then(|c| c.display_list.get()?.as_ref().ok());

    match display_list {
        Some(dl) => {
            let metrics = display_list_metrics(dl, font_size);
            let dl = dl.clone();
            let default_color = style.base_text_style.color;

            canvas(
                move |_bounds, _window, _cx| (dl, font_size, default_color),
                move |bounds, (dl, font_size, default_color), window, _cx| {
                    for item in &dl.items {
                        paint_display_item(item, bounds.origin, font_size, default_color, window);
                    }
                },
            )
            .w(metrics.width)
            .h(metrics.height())
            .into_any_element()
        }
        None => div()
            .child(SharedString::from(format!(
                "{}{}{}",
                if expr.contents.display_mode {
                    "$$"
                } else {
                    "$"
                },
                expr.contents.contents,
                if expr.contents.display_mode {
                    "$$"
                } else {
                    "$"
                }
            )))
            .into_any_element(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::MarkdownEvent;

    #[test]
    fn strip_math_delimiters_inline() {
        assert_eq!(strip_math_delimiters("$x + y$", false), "x + y");
        // Surrounding whitespace is trimmed first.
        assert_eq!(strip_math_delimiters("  $x$  ", false), "x");
        // Missing one side of the delimiter leaves the original `s` intact
        // (the function is defensive — pulldown-cmark only emits matched pairs).
        assert_eq!(strip_math_delimiters("x + y", false), "x + y");
        assert_eq!(strip_math_delimiters("$x + y", false), "$x + y");
        assert_eq!(strip_math_delimiters("x + y$", false), "x + y$");
    }

    #[test]
    fn strip_math_delimiters_display() {
        assert_eq!(strip_math_delimiters("$$x + y$$", true), "x + y");
        // Trims surrounding whitespace before stripping `$$`.
        assert_eq!(strip_math_delimiters("\n  $$x$$  \n", true), "x");
        // Single `$` boundary doesn't match the display delimiter.
        assert_eq!(strip_math_delimiters("$x$", true), "$x$");
        assert_eq!(strip_math_delimiters("$$x$", true), "$$x$");
    }

    #[test]
    fn extract_math_expressions_collects_inline_and_display() {
        use crate::parser::parse_markdown_with_options;
        let source = "before $a + b$ middle\n\n$$
foo
 + bar
$$\n\nafter";
        let parsed = parse_markdown_with_options(source, true, false, false);
        let math_events: Vec<_> = parsed
            .events
            .iter()
            .filter(|(_, ev)| matches!(ev, MarkdownEvent::InlineMath | MarkdownEvent::DisplayMath))
            .cloned()
            .collect();
        assert_eq!(math_events.len(), 2, "got events: {:?}", parsed.events);

        let extracted = extract_math_expressions(source, &math_events);
        assert_eq!(extracted.len(), 2, "got: {extracted:?}");

        // Inline math: `$a + b$` with surrounding spaces/newlines.
        let inline = extracted
            .values()
            .find(|m| !m.contents.display_mode)
            .expect("inline math should be extracted");
        assert_eq!(inline.contents.contents.to_string(), "a + b");

        // Display math: the multi-line block.
        let display = extracted
            .values()
            .find(|m| m.contents.display_mode)
            .expect("display math should be extracted");
        assert!(display.contents.contents.to_string().contains("foo"));
        assert!(display.contents.contents.to_string().contains("+ bar"));
        // The display event's source range must slice back to the original `$$...$$` block.
        let display_event_range = math_events
            .iter()
            .find(|(_, ev)| matches!(ev, MarkdownEvent::DisplayMath))
            .expect("display math event")
            .0
            .clone();
        let slice = &source[display_event_range];
        assert!(slice.starts_with("$$"));
        assert!(slice.ends_with("$$"));
    }

    #[test]
    fn extract_math_expressions_skips_empty_and_non_math_events() {
        use crate::parser::parse_markdown_with_options;
        // A display-math block consisting only of whitespace, and an
        // inline math that's just a stray `$` (which pulldown-cmark would
        // not normally emit, so this also covers "all events are not math").
        let source = "no math here\n\n$$  $$\n\njust text $ $ more";
        let parsed = parse_markdown_with_options(source, true, false, false);
        let extracted = extract_math_expressions(source, &parsed.events);
        // Whatever pulldown-cmark produces for `$$  $$`, the extracted
        // content must never be empty.
        for m in extracted.values() {
            assert!(
                !m.contents.contents.trim().is_empty(),
                "extracted math content should not be empty: {m:?}"
            );
        }
    }

    #[test]
    fn extract_math_expressions_ignores_other_events() {
        let source = "text";
        let events = vec![
            (0..4, MarkdownEvent::Text),
            (
                0..4,
                MarkdownEvent::Start(crate::parser::MarkdownTag::Paragraph),
            ),
        ];
        let extracted = extract_math_expressions(source, &events);
        assert!(extracted.is_empty());
    }

    #[test]
    fn is_system_fallback_font_recognises_cjk_and_emoji() {
        assert!(is_system_fallback_font(FontId::CjkRegular));
        assert!(is_system_fallback_font(FontId::CjkFallback));
        assert!(is_system_fallback_font(FontId::EmojiFallback));
        assert!(!is_system_fallback_font(FontId::MainRegular));
        assert!(!is_system_fallback_font(FontId::MathItalic));
    }

    #[test]
    fn display_list_metrics_zero_for_empty() {
        let display_list = DisplayList {
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            items: vec![],
        };
        let metrics = display_list_metrics(&display_list, px(16.0));
        assert_eq!(metrics.width, Pixels::ZERO);
        assert_eq!(metrics.ascent, Pixels::ZERO);
        assert_eq!(metrics.descent, Pixels::ZERO);
        assert_eq!(metrics.height(), Pixels::ZERO);

        // Non-zero display list: ascent/depth are scaled by font_size in em.
        let display_list = DisplayList {
            width: 0.5,
            height: 0.8,
            depth: 0.2,
            items: vec![],
        };
        let metrics = display_list_metrics(&display_list, px(10.0));
        assert_eq!(metrics.width, px(5.0));
        assert_eq!(metrics.ascent, px(8.0));
        assert_eq!(metrics.descent, px(2.0));
        assert_eq!(metrics.height(), px(10.0));
    }

    #[test]
    fn katex_gpui_font_all_variants_map_to_correct_family() {
        use gpui::{FontStyle, FontWeight};

        // (input name, expected family, expected weight, expected style)
        let cases: &[(&str, &str, FontWeight, FontStyle)] = &[
            (
                "Main-Bold",
                "KaTeX_Main",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Main-Italic",
                "KaTeX_Main",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Main-BoldItalic",
                "KaTeX_Main",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            (
                "Math-Italic",
                "KaTeX_Math",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Math-BoldItalic",
                "KaTeX_Math",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            (
                "AMS-Regular",
                "KaTeX_AMS",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Caligraphic-Regular",
                "KaTeX_Caligraphic",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Fraktur-Regular",
                "KaTeX_Fraktur",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Fraktur-Bold",
                "KaTeX_Fraktur",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "SansSerif-Regular",
                "KaTeX_SansSerif",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "SansSerif-Bold",
                "KaTeX_SansSerif",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "SansSerif-Italic",
                "KaTeX_SansSerif",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Script-Regular",
                "KaTeX_Script",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Typewriter-Regular",
                "KaTeX_Typewriter",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Size1-Regular",
                "KaTeX_Size1",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Size2-Regular",
                "KaTeX_Size2",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Size3-Regular",
                "KaTeX_Size3",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Size4-Regular",
                "KaTeX_Size4",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
        ];

        for (input, expected_family, expected_weight, expected_style) in cases {
            let font = katex_gpui_font(input);
            assert_eq!(font.family.as_ref(), *expected_family, "family for {input}");
            assert_eq!(font.weight, *expected_weight, "weight for {input}");
            assert_eq!(font.style, *expected_style, "style for {input}");
        }
    }

    #[test]
    fn katex_gpui_font_unknown_name_falls_back_to_main_regular() {
        use gpui::{FontStyle, FontWeight};
        // An unrecognized name (e.g. a future KaTeX face we haven't wired
        // up) should fall back to KaTeX_Main Regular, never crash and
        // never produce a font with the raw name as family.
        let font = katex_gpui_font("NotARealKaTeXFface");
        assert_eq!(font.family.as_ref(), "KaTeX_Main");
        assert_eq!(font.weight, FontWeight::NORMAL);
        assert_eq!(font.style, FontStyle::Normal);
    }

    #[test]
    fn math_color_to_hsla_uses_default_for_black() {
        use gpui::hsla;
        let default = hsla(0.5, 0.5, 0.5, 1.0);
        // RatexColor::BLACK must short-circuit to the default — `BLACK`
        // is the sentinel that means "follow the surrounding text color"
        // and a black hsla would visually mask the math.
        assert_eq!(math_color_to_hsla(&RatexColor::BLACK, default), default,);
    }

    #[test]
    fn math_color_to_hsla_converts_rgba() {
        use gpui::{Rgba, hsla};
        let default = hsla(0.0, 0.0, 0.0, 1.0);
        let red = RatexColor {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        let result = math_color_to_hsla(&red, default);
        let expected = Hsla::from(Rgba {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });
        assert_eq!(result, expected);
        // Sanity: must not be the default (we passed a non-black color).
        assert_ne!(result, default);
    }

    fn make_cached(
        display_list_result: Option<anyhow::Result<Arc<DisplayList>>>,
        fallback: Option<Arc<DisplayList>>,
    ) -> CachedMathExpression {
        let cell = Arc::new(OnceLock::<anyhow::Result<Arc<DisplayList>>>::new());
        if let Some(value) = display_list_result {
            // OnceLock can only be set once; the test that exercises the
            // "pending" path leaves this empty.
            if let Err(value) = cell.set(value) {
                panic!("test OnceLock already set: {value:?}");
            }
        }
        CachedMathExpression {
            display_list: cell,
            fallback,
            _task: Task::ready(()),
        }
    }

    fn make_cache_with(
        contents: ParsedMathExpressionContents,
        display_list: Option<anyhow::Result<Arc<DisplayList>>>,
        fallback: Option<Arc<DisplayList>>,
    ) -> (MathExpressionCache, Vec<ParsedMathExpressionContents>) {
        let mut cache = MathExpressionCache::default();
        let order = vec![contents.clone()];
        cache.insert(contents, Arc::new(make_cached(display_list, fallback)));
        (cache, order)
    }

    fn empty_display_list() -> Arc<DisplayList> {
        Arc::new(DisplayList {
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            items: vec![],
        })
    }

    fn sample_contents() -> ParsedMathExpressionContents {
        ParsedMathExpressionContents {
            contents: SharedString::from("a + b"),
            display_mode: false,
        }
    }

    #[test]
    fn get_fallback_returns_none_on_length_mismatch() {
        let (cache, order) = make_cache_with(
            sample_contents(),
            Some(Ok(empty_display_list())),
            Some(empty_display_list()),
        );
        // new_order_len differs from old order length — must short-circuit.
        assert!(MathState::get_fallback(0, &order, order.len() + 1, &cache).is_none());
        // And in the other direction (shorter new order).
        assert!(MathState::get_fallback(0, &order, order.len() - 1, &cache).is_none());
    }

    #[test]
    fn get_fallback_returns_none_when_idx_out_of_bounds() {
        let (cache, order) = make_cache_with(
            sample_contents(),
            Some(Ok(empty_display_list())),
            Some(empty_display_list()),
        );
        assert!(MathState::get_fallback(99, &order, order.len(), &cache).is_none());
    }

    #[test]
    fn get_fallback_returns_none_when_cache_missing() {
        // Build a cache whose key doesn't match what `old_order` refers to.
        let contents_a = sample_contents();
        let contents_b = ParsedMathExpressionContents {
            contents: SharedString::from("x + y"),
            display_mode: false,
        };
        let mut cache = MathExpressionCache::default();
        cache.insert(
            contents_a,
            Arc::new(make_cached(Some(Ok(empty_display_list())), None)),
        );
        // order references contents_b which is not in the cache.
        let order = vec![contents_b];
        assert!(MathState::get_fallback(0, &order, order.len(), &cache).is_none());
    }

    #[test]
    fn get_fallback_returns_display_list_when_ready() {
        let dl = empty_display_list();
        let (cache, order) = make_cache_with(
            sample_contents(),
            Some(Ok(dl.clone())),
            // fallback present but must NOT be used when display_list is ready.
            Some(Arc::new(DisplayList {
                width: 99.0,
                height: 0.0,
                depth: 0.0,
                items: vec![],
            })),
        );
        let result = MathState::get_fallback(0, &order, order.len(), &cache);
        assert_eq!(result.as_ref().map(|d| d.width), Some(0.0));
    }

    #[test]
    fn get_fallback_uses_fallback_when_display_list_errored() {
        let fallback = empty_display_list();
        let (cache, order) = make_cache_with(
            sample_contents(),
            Some(Err(anyhow::anyhow!("ratex parse error"))),
            Some(fallback.clone()),
        );
        let result = MathState::get_fallback(0, &order, order.len(), &cache);
        // Width 0.0 == empty_display_list().width.
        assert_eq!(result.as_ref().map(|d| d.width), Some(fallback.width));
    }

    #[test]
    fn get_fallback_uses_fallback_when_display_list_pending() {
        let fallback = empty_display_list();
        let (cache, order) = make_cache_with(
            sample_contents(),
            None, // OnceLock never set → pending
            Some(fallback.clone()),
        );
        let result = MathState::get_fallback(0, &order, order.len(), &cache);
        assert_eq!(result.as_ref().map(|d| d.width), Some(fallback.width));
    }

    #[test]
    fn get_fallback_returns_none_when_nothing_available() {
        // Pending display list, no fallback.
        let (cache, order) = make_cache_with(sample_contents(), None, None);
        assert!(MathState::get_fallback(0, &order, order.len(), &cache).is_none());

        // Errored display list, no fallback.
        let (cache, order) =
            make_cache_with(sample_contents(), Some(Err(anyhow::anyhow!("boom"))), None);
        assert!(MathState::get_fallback(0, &order, order.len(), &cache).is_none());
    }

    #[test]
    fn math_state_clear_empties_cache_and_order() {
        let contents = sample_contents();
        let mut state = MathState::default();
        // Seed by going through the public cache (state.cache is private).
        state.cache.insert(
            contents.clone(),
            Arc::new(make_cached(Some(Ok(empty_display_list())), None)),
        );
        state.order.push(contents);
        assert_eq!(state.cache.len(), 1);
        assert_eq!(state.order.len(), 1);

        state.clear();
        assert!(state.cache.is_empty());
        assert!(state.order.is_empty());
    }
}
