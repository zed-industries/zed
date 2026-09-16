//! A consolidated demonstration of the out-of-tree Parley text system for GPUI.
//!
//! It shows the embedded IBM Plex Sans specimens alongside Parley-native layout
//! features (text-indent, per-line boxes, and character-count breaking) that the
//! shared [`gpui_engine::TextSystem`] SPI does not expose. The native features
//! are reached through the `as_parley` accessor the crate exposes for them.
//!
//! Run with:
//!
//! ```sh
//! cargo run -p gpui_parley --example parley_demo --features demo
//! ```
//!
//! The `demo` feature pulls `gpui` into the example without also pulling in its
//! `test-support` feature. A dev-dependency's features are unified into the
//! example build, and `test-support` turns on `App::flush_effects`'s test draw
//! path, which draws once per `App::update` and bypasses the platform's frame
//! pacing — leaving the example rendering at whatever rate input arrives.

use gpui::{
    App, Bounds, BoundsExt, Context, Render, ScrollHandle, Window, WindowBounds, WindowOptions,
    application, div, prelude::*, px, rgb, size,
};
use gpui_parley::{LineBox, ParleyTextSystem, ParleyTextSystemExt};

/// Sample body copy used by the Parley-native layout cards.
const TEXT: &str = "The quick brown fox jumps over the lazy dog while the bright \
                    sun shines down on the quiet meadow near the river.";

/// The line boxes used to demonstrate flowing text around an excluded region.
const BOXES: [LineBox; 3] = [
    LineBox {
        x: 0.0,
        width: 320.0,
    },
    LineBox {
        x: 80.0,
        width: 160.0,
    },
    LineBox {
        x: 0.0,
        width: 320.0,
    },
];

/// The size, column width, first-line indent, and character limit the native
/// layouts below are computed at. The cards draw at the same size so their
/// offsets and advances line up with the glyphs on screen.
const LAYOUT_SIZE: f32 = 16.0;
const LAYOUT_WIDTH: f32 = 320.0;
const INDENT: f32 = 40.0;
const CHAR_LIMIT: u32 = 16;

/// One line as Parley laid it out.
struct ParleyLine {
    /// The line's text, carved out of the source by Parley's break points.
    text: String,
    /// The x Parley started the line's text at.
    x: f32,
    /// The line's natural advance, which is what the drawn text occupies.
    advance: f32,
    /// The advance limit the line was broken against, when the feature sets one.
    /// The layout does not record it, so it comes from the call site.
    limit: Option<f32>,
}

/// The x position of a line's first positioned glyph.
fn first_glyph_x(line: &parley::Line<'_, [u8; 4]>) -> Option<f32> {
    line.items().find_map(|item| match item {
        parley::PositionedLayoutItem::GlyphRun(run) => {
            run.positioned_glyphs().next().map(|glyph| glyph.x)
        }
        _ => None,
    })
}

/// Reads a layout's lines back out.
///
/// The cards draw from this rather than from a diagram of what Parley is
/// expected to produce, so what is on screen is the geometry that was computed.
fn lines_of(
    layout: &parley::Layout<[u8; 4]>,
    source: &str,
    limit_of: impl Fn(usize) -> Option<f32>,
) -> Vec<ParleyLine> {
    layout
        .lines()
        .enumerate()
        .map(|(index, line)| ParleyLine {
            text: source
                .get(line.text_range())
                .unwrap_or_default()
                .to_string(),
            x: first_glyph_x(&line).unwrap_or(0.0),
            advance: line.metrics().advance,
            limit: limit_of(index),
        })
        .collect()
}

/// A section heading.
fn heading(title: &'static str) -> impl IntoElement {
    div()
        .text_size(px(18.0))
        .font_weight(gpui::FontWeight(600.0))
        .text_color(rgb(0x2dd4bf))
        .child(title)
}

/// A font specimen: a caption plus the pangram in one embedded face.
fn specimen(label: &'static str, weight: gpui::FontWeight, italic: bool) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(0x8a8a93))
                .child(label),
        )
        .child(
            div()
                .font_family("IBM Plex Sans")
                .font_weight(weight)
                .when(italic, |this| this.italic())
                .text_size(px(24.0))
                .text_color(rgb(0xf5f5f5))
                .child("The quick brown fox jumps over the lazy dog"),
        )
}

/// A feature card title with a short subtitle.
fn feature_title(title: &'static str, subtitle: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_size(px(15.0))
                .font_weight(gpui::FontWeight(600.0))
                .text_color(rgb(0x2dd4bf))
                .child(title),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(0x8a8a93))
                .child(subtitle),
        )
}

/// Draws lines where Parley put them, at the widths it gave them.
///
/// The text is drawn at the layout's own size and is never re-wrapped, so the
/// offsets and box widths on screen are the ones Parley computed.
fn line_flow(lines: &[ParleyLine], width: f32) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .w(px(width))
        .children(lines.iter().map(|line| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .child(div().w(px(line.x)))
                .child(
                    div()
                        .w(px(line.limit.unwrap_or(line.advance)))
                        .py_1()
                        .rounded_sm()
                        .bg(rgb(0x1c1c22))
                        .text_size(px(LAYOUT_SIZE))
                        .text_color(rgb(0xf5f5f5))
                        .whitespace_nowrap()
                        .child(line.text.clone()),
                )
        }))
}

/// A feature card: what the feature is, what Parley computed for it, and the
/// lines themselves.
fn card(
    title: &'static str,
    subtitle: &'static str,
    summary: String,
    lines: &[ParleyLine],
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_start()
        .gap_3()
        .p_5()
        .rounded_md()
        .bg(rgb(0x1e1e22))
        .border_1()
        .border_color(rgb(0x2a2a30))
        .child(feature_title(title, subtitle))
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(0x8a8a93))
                .child(summary),
        )
        .child(line_flow(lines, LAYOUT_WIDTH))
}

/// The page header.
fn header() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_size(px(30.0))
                .font_weight(gpui::FontWeight(600.0))
                .text_color(rgb(0xf5f5f5))
                .child("Parley × GPUI"),
        )
        .child(div().text_size(px(14.0)).text_color(rgb(0x8a8a93)).child(
            "An out-of-tree text system: shaping, wrapping, and rasterization \
                     through the Linebender stack (Parley + Skrifa + tiny-skia).",
        ))
}

/// The embedded font specimens.
fn font_section() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(heading("Font specimens"))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_5()
                .p_6()
                .rounded_md()
                .bg(rgb(0x1e1e22))
                .border_1()
                .border_color(rgb(0x2a2a30))
                .child(specimen(
                    "IBM Plex Sans Regular",
                    gpui::FontWeight(400.0),
                    false,
                ))
                .child(specimen(
                    "IBM Plex Sans Italic",
                    gpui::FontWeight(400.0),
                    true,
                ))
                .child(specimen(
                    "IBM Plex Sans SemiBold",
                    gpui::FontWeight(600.0),
                    false,
                ))
                .child(specimen(
                    "IBM Plex Sans SemiBold Italic",
                    gpui::FontWeight(600.0),
                    true,
                )),
        )
}

/// The `text-indent` feature card.
fn indent_card(lines: &[ParleyLine]) -> impl IntoElement {
    card(
        "text-indent",
        "CSS-style first-line indent, computed by Parley.",
        format!(
            "{} lines · the first line starts at x = {:.0}px",
            lines.len(),
            lines.first().map_or(0.0, |line| line.x),
        ),
        lines,
    )
}

/// The per-line boxes feature card.
fn boxes_card(lines: &[ParleyLine]) -> impl IntoElement {
    card(
        "Per-line boxes",
        "Text flows around an excluded region (a narrow second line).",
        format!(
            "{} lines · the second line is confined to a {}px box",
            lines.len(),
            BOXES.get(1).map_or(LAYOUT_WIDTH, |line_box| line_box.width),
        ),
        lines,
    )
}

/// The character-count breaking feature card.
fn char_count_card(lines: &[ParleyLine]) -> impl IntoElement {
    card(
        "Character-count breaking",
        "Lines broken by character count (16), not by pixel width.",
        format!(
            "{} lines · {} characters on the first line",
            lines.len(),
            lines
                .first()
                .map_or(0, |line| line.text.trim_end().chars().count()),
        ),
        lines,
    )
}

/// The three Parley-native layouts, read back as the lines they produced.
struct DemoMetrics {
    indented: Vec<ParleyLine>,
    boxed: Vec<ParleyLine>,
    char_counted: Vec<ParleyLine>,
}

impl DemoMetrics {
    fn compute(cx: &App) -> Self {
        let parley = cx.text_system().as_parley();
        let Some(parley) = parley else {
            return Self {
                indented: Vec::new(),
                boxed: Vec::new(),
                char_counted: Vec::new(),
            };
        };

        Self {
            // The indent moves the first line right and narrows its advance by
            // the same amount; every other line gets the full column.
            indented: lines_of(
                &parley.layout_indented(TEXT, LAYOUT_SIZE, INDENT, LAYOUT_WIDTH),
                TEXT,
                |index| {
                    Some(if index == 0 {
                        LAYOUT_WIDTH - INDENT
                    } else {
                        LAYOUT_WIDTH
                    })
                },
            ),
            // Lines past the end of the boxes use the full width, as
            // `layout_with_boxes` documents.
            boxed: lines_of(
                &parley.layout_with_boxes(TEXT, LAYOUT_SIZE, &BOXES),
                TEXT,
                |index| {
                    Some(
                        BOXES
                            .get(index)
                            .map_or(LAYOUT_WIDTH, |line_box| line_box.width),
                    )
                },
            ),
            // A character-count break sets no advance limit, so each line runs
            // exactly as far as its glyphs do.
            char_counted: lines_of(
                &parley.layout_with_char_count(TEXT, LAYOUT_SIZE, CHAR_LIMIT),
                TEXT,
                |_| None,
            ),
        }
    }
}

struct ParleyDemo {
    metrics: DemoMetrics,
    scroll_handle: ScrollHandle,
}

impl Render for ParleyDemo {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let metrics = &self.metrics;

        div()
            .relative()
            .size_full()
            .bg(rgb(0x141417))
            .text_color(rgb(0xf5f5f5))
            .child(
                div()
                    .id("parley-demo")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_6()
                            .p_8()
                            .child(header())
                            .child(font_section())
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(heading("Parley-native layout"))
                                    .child(
                                        div().text_size(px(13.0)).text_color(rgb(0x8a8a93)).child(
                                            "These cards draw the lines Parley itself laid out, \
                                                 at the offsets and widths it computed — not the \
                                                 shared TextSystem path the specimens above use.",
                                        ),
                                    )
                                    .child(indent_card(&metrics.indented))
                                    .child(boxes_card(&metrics.boxed))
                                    .child(char_count_card(&metrics.char_counted)),
                            ),
                    ),
            )
            .child(scrollbar(
                &self.scroll_handle,
                window.viewport_size().height.0,
            ))
    }
}

/// Draws a vertical scrollbar thumb on the right edge for `handle`.
fn scrollbar(handle: &ScrollHandle, viewport_height: f32) -> impl IntoElement {
    let max_offset = handle.max_offset().y.0;
    let (thumb_top, thumb_height) = if max_offset > 0.0 && viewport_height > 0.0 {
        let content_height = viewport_height + max_offset;
        let thumb_height = (viewport_height / content_height) * viewport_height;
        let scrolled = -handle.offset().y.0;
        let thumb_top = (scrolled / max_offset) * (viewport_height - thumb_height);
        (thumb_top, thumb_height)
    } else {
        (0.0, 0.0)
    };

    div()
        .absolute()
        .top(px(thumb_top))
        .right(px(4.0))
        .w(px(6.0))
        .h(px(thumb_height))
        .rounded_full()
        .bg(rgb(0x4a4a52))
}

fn main() {
    application()
        .with_text_system(ParleyTextSystem::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(800.0), px(900.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| ParleyDemo {
                        metrics: DemoMetrics::compute(cx),
                        scroll_handle: ScrollHandle::new(),
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
