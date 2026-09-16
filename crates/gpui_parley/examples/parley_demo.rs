//! A consolidated demonstration of the out-of-tree Parley text system for GPUI.
//!
//! It shows the embedded IBM Plex Sans specimens alongside Parley-native layout
//! features (text-indent, per-line boxes, and character-count breaking) that the
//! shared [`gpui_engine::TextSystem`] SPI does not expose. The native features
//! are reached by downcasting the injected text system to [`ParleyTextSystem`].
//!
//! Run with:
//!
//! ```sh
//! cargo run -p gpui_parley --example parley_demo
//! ```

use gpui::{
    App, Bounds, BoundsExt, Context, Render, ScrollHandle, Window, WindowBounds, WindowOptions,
    application, div, prelude::*, px, rgb, size,
};
use gpui_parley::{LineBox, ParleyTextSystem};

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

/// Returns the x position of the first glyph on the first line of a layout.
fn first_glyph_x(layout: &parley::Layout<[u8; 4]>) -> Option<f32> {
    layout.lines().next().and_then(|line| {
        line.items().find_map(|item| match item {
            parley::PositionedLayoutItem::GlyphRun(run) => {
                run.positioned_glyphs().next().map(|glyph| glyph.x)
            }
            _ => None,
        })
    })
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

/// The shared sample copy, constrained so it wraps predictably.
fn sample_text() -> impl IntoElement {
    div()
        .w(px(360.0))
        .text_size(px(15.0))
        .text_color(rgb(0xf5f5f5))
        .child(TEXT)
}

/// A schematic line bar with an x offset and width, for layout diagrams.
fn line_bar(x: f32, width: f32, color: impl Into<gpui::Fill>) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .child(div().w(px(x)))
        .child(div().h(px(8.0)).w(px(width)).rounded_md().bg(color))
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
fn indent_card(line_count: usize, first_glyph_x: Option<f32>) -> impl IntoElement {
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
        .child(feature_title(
            "text-indent",
            "CSS-style first-line indent, computed by Parley.",
        ))
        .child(sample_text())
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(0x8a8a93))
                .child(format!(
                    "Parley: {line_count} lines · first glyph lands at x = {:.1}px",
                    first_glyph_x.unwrap_or(0.0)
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(line_bar(40.0, 280.0, rgb(0x2dd4bf)))
                .child(line_bar(0.0, 320.0, rgb(0x3f3f46)))
                .child(line_bar(0.0, 320.0, rgb(0x3f3f46))),
        )
}

/// The per-line boxes feature card.
fn boxes_card(line_count: usize) -> impl IntoElement {
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
        .child(feature_title(
            "Per-line boxes",
            "Text flows around an excluded region (a narrow second line).",
        ))
        .child(sample_text())
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(0x8a8a93))
                .child(format!(
                    "Parley: {line_count} lines · line 2 flows into a 160px box",
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(line_bar(0.0, 320.0, rgb(0x2dd4bf)))
                .child(line_bar(80.0, 160.0, rgb(0x2dd4bf)))
                .child(line_bar(0.0, 320.0, rgb(0x3f3f46))),
        )
}

/// The character-count breaking feature card.
fn char_count_card(line_count: usize) -> impl IntoElement {
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
        .child(feature_title(
            "Character-count breaking",
            "Lines broken by character count (16), not by pixel width.",
        ))
        .child(sample_text())
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(0x8a8a93))
                .child(format!(
                    "Parley: {line_count} lines · max 16 chars per line",
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(line_bar(0.0, 180.0, rgb(0x2dd4bf)))
                .child(line_bar(0.0, 180.0, rgb(0x2dd4bf)))
                .child(line_bar(0.0, 180.0, rgb(0x2dd4bf)))
                .child(line_bar(0.0, 180.0, rgb(0x3f3f46))),
        )
}

/// Precomputed metrics for the Parley-native layout cards.
struct DemoMetrics {
    indent_lines: usize,
    indent_first_x: Option<f32>,
    box_lines: usize,
    char_lines: usize,
}

impl DemoMetrics {
    fn compute(cx: &App) -> Self {
        let parley = cx.text_system().as_any().downcast_ref::<ParleyTextSystem>();

        let indent = parley.map(|parley| parley.layout_indented(TEXT, 16.0, 40.0, 320.0));
        let indent_lines = indent
            .as_ref()
            .map(|layout| layout.lines().len())
            .unwrap_or(0);
        let indent_first_x = indent.as_ref().and_then(first_glyph_x);

        let box_lines = parley
            .map(|parley| parley.layout_with_boxes(TEXT, 16.0, &BOXES).lines().len())
            .unwrap_or(0);

        let char_lines = parley
            .map(|parley| parley.layout_with_char_count(TEXT, 16.0, 16).lines().len())
            .unwrap_or(0);

        Self {
            indent_lines,
            indent_first_x,
            box_lines,
            char_lines,
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
                                    .child(indent_card(
                                        metrics.indent_lines,
                                        metrics.indent_first_x,
                                    ))
                                    .child(boxes_card(metrics.box_lines))
                                    .child(char_count_card(metrics.char_lines)),
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
