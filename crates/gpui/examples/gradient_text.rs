#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Background, Bounds, ColorSpace, Context, FontWeight, IntoElement, Render, SharedString,
    TextRun, Window, WindowBounds, WindowOptions, canvas, checkerboard, div, font,
    linear_color_stop, linear_gradient, pattern_slash, prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct GradientText;

#[derive(Clone)]
struct Sample {
    label: SharedString,
    text: SharedString,
    background: Background,
    font_size: f32,
    weight: FontWeight,
}

fn samples() -> Vec<Sample> {
    vec![
        Sample {
            label: "Linear gradient (sRGB)".into(),
            text: "The quick brown fox jumps over the lazy dog".into(),
            background: linear_gradient(
                90.,
                linear_color_stop(gpui::red(), 0.),
                linear_color_stop(gpui::blue(), 1.),
            )
            .color_space(ColorSpace::Srgb),
            font_size: 48.,
            weight: FontWeight::BOLD,
        },
        Sample {
            label: "Linear gradient (Oklab)".into(),
            text: "The quick brown fox jumps over the lazy dog".into(),
            background: linear_gradient(
                90.,
                linear_color_stop(gpui::red(), 0.),
                linear_color_stop(gpui::blue(), 1.),
            )
            .color_space(ColorSpace::Oklab),
            font_size: 48.,
            weight: FontWeight::BOLD,
        },
        Sample {
            label: "Hash pattern fill".into(),
            text: "Patterns work too!".into(),
            background: pattern_slash(gpui::black(), 2.0, 4.0),
            font_size: 56.,
            weight: FontWeight::BOLD,
        },
        Sample {
            label: "Checkerboard fill".into(),
            text: "Checkers".into(),
            background: checkerboard(gpui::black(), 6.0),
            font_size: 56.,
            weight: FontWeight::BOLD,
        },
    ]
}

impl Render for GradientText {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut root = div()
            .bg(gpui::white())
            .size_full()
            .p_6()
            .flex()
            .flex_col()
            .gap_4();

        for sample in samples() {
            root = root.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_color(rgb(0x555555))
                            .text_size(px(11.))
                            .child(sample.label.clone()),
                    )
                    .child(sample_canvas(sample)),
            );
        }

        root
    }
}

fn sample_canvas(sample: Sample) -> impl IntoElement {
    div().h(px(sample.font_size * 1.4)).child(canvas(
        |_, _, _| {},
        move |bounds: Bounds<gpui::Pixels>, _, window, cx| {
            let font_size = px(sample.font_size);
            let line_height = font_size * 1.25;
            let run = TextRun {
                len: sample.text.len(),
                font: gpui::Font {
                    weight: sample.weight,
                    ..font(".SystemUIFont")
                },
                color: gpui::black(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };

            let shaped =
                window
                    .text_system()
                    .shape_line(sample.text.clone(), font_size, &[run], None);

            let origin = bounds.origin;
            shaped
                .paint_with_clip_background(
                    origin,
                    line_height,
                    gpui::TextAlign::Left,
                    None,
                    sample.background,
                    window,
                    cx,
                )
                .ok();
        },
    ))
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(720.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| GradientText),
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
