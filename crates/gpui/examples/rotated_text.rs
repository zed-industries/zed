#![cfg_attr(target_family = "wasm", no_main)]

#[path = "example_support/fonts.rs"]
mod example_support;

use gpui::{
    App, Bounds, ContentMask, Context, FontWeight, Hsla, Pixels, Point, SharedString, TextAlign,
    TextRun, Transformation, Window, WindowBounds, WindowOptions, canvas, div, hsla, point,
    prelude::*, px, radians, rgb, size,
};
use gpui_platform::application;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

struct RotatedText;

fn shape(
    text: &str,
    font_size: Pixels,
    weight: FontWeight,
    color: Hsla,
    window: &Window,
) -> gpui::ShapedLine {
    let mut font = window.text_style().font();
    font.weight = weight;
    let text: SharedString = text.to_string().into();
    window.text_system().shape_line(
        text.clone(),
        font_size,
        &[TextRun {
            len: text.len(),
            font,
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        }],
        None,
    )
}

/// Paints `line` rotated by `angle` with its centre at `center`.
fn paint_centered(
    line: &gpui::ShapedLine,
    center: Point<Pixels>,
    line_height: Pixels,
    angle: f32,
    window: &mut Window,
    cx: &mut App,
) {
    let origin = center - point(line.width / 2., line_height / 2.);
    line.paint_transformed(
        origin,
        line_height,
        TextAlign::Left,
        None,
        Transformation::rotate(radians(angle)),
        window,
        cx,
    )
    .unwrap();
}

/// Text spokes around a centre, one every 30°.
fn fan() -> impl IntoElement {
    canvas(
        |_, _, _| {},
        |bounds, _, window, cx| {
            let center = bounds.center();
            for i in 0..12 {
                let angle = TAU * i as f32 / 12.;
                let hue = i as f32 / 12.;
                let line = shape(
                    "rotated text",
                    px(15.),
                    FontWeight::MEDIUM,
                    hsla(hue, 0.75, 0.72, 1.),
                    window,
                );
                // Push each line out along its own direction so they radiate, and
                // turn the left half around so every spoke reads outward.
                let reach = px(42.) + line.width / 2.;
                let offset = point(reach * angle.cos(), reach * angle.sin());
                let upright = if angle.cos() < -1e-3 {
                    angle + PI
                } else {
                    angle
                };
                paint_centered(&line, center + offset, px(20.), upright, window, cx);
            }
            let hub = shape(
                "gpui",
                px(22.),
                FontWeight::BOLD,
                hsla(0., 0., 0.95, 1.),
                window,
            );
            paint_centered(&hub, center, px(28.), 0., window, cx);
        },
    )
    .size(px(340.))
}

/// A row tag: a colored strip with its label running bottom-to-top, clipped
/// to the strip like any other content.
fn vertical_tag(label: &'static str, color: Hsla) -> impl IntoElement {
    div().w(px(22.)).h_full().flex_none().bg(color).child(
        canvas(
            |_, _, _| {},
            move |bounds, _, window, cx| {
                let line = shape(
                    label,
                    px(10.5),
                    FontWeight::BOLD,
                    hsla(0., 0., 1., 0.92),
                    window,
                );
                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    paint_centered(&line, bounds.center(), px(14.), -FRAC_PI_2, window, cx);
                });
            },
        )
        .size_full(),
    )
}

fn row(
    tag: &'static str,
    color: Hsla,
    title: &'static str,
    meta: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .h(px(64.))
        .border_b_1()
        .border_color(rgb(0x313244))
        .child(vertical_tag(tag, color))
        .child(
            div()
                .flex()
                .flex_col()
                .justify_center()
                .gap(px(4.))
                .px(px(14.))
                .child(
                    div()
                        .text_size(px(14.))
                        .text_color(rgb(0xcdd6f4))
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(0x7f849c))
                        .child(meta),
                ),
        )
}

impl Render for RotatedText {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x11111b))
            .flex()
            .items_center()
            .justify_center()
            .gap(px(36.))
            .child(fan())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.))
                    .child(
                        div()
                            .w(px(380.))
                            .rounded(px(10.))
                            .overflow_hidden()
                            .bg(rgb(0x1e1e2e))
                            .border_1()
                            .border_color(rgb(0x313244))
                            .child(row(
                                "CONTOSO",
                                hsla(0.6, 0.85, 0.6, 1.),
                                "Add Apple Pay to the payment step",
                                "Storefront.Web #4182 · Mia Larsen",
                            ))
                            .child(row(
                                "FABRIKAM",
                                hsla(0.4, 0.6, 0.42, 1.),
                                "Cluster markers above 500 vehicles",
                                "Fleet.Tracker #915 · Tom Hansen",
                            ))
                            .child(row(
                                "LITWARE",
                                hsla(0.08, 0.8, 0.55, 1.),
                                "Import the spring catalogue",
                                "Catalog.Sync #77 · Sara Kim",
                            )),
                    )
                    .child(
                        canvas(
                            |_, _, _| {},
                            |bounds, _, window, cx| {
                                let line = shape(
                                    "Emoji rotate too 🎉 🔥 ✨",
                                    px(16.),
                                    FontWeight::NORMAL,
                                    hsla(0., 0., 0.85, 1.),
                                    window,
                                );
                                paint_centered(&line, bounds.center(), px(22.), -0.12, window, cx);
                            },
                        )
                        .w(px(380.))
                        .h(px(56.)),
                    ),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        if !example_support::load_fonts(cx) {
            return;
        }
        let bounds = Bounds::centered(None, size(px(860.), px(460.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| RotatedText),
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
