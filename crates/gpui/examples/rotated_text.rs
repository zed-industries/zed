#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, Radians, TextAlign, TextRun, TransformationMatrix, Window, WindowBounds,
    WindowOptions, black, canvas, div, point, prelude::*, px, size, white,
};
use gpui_platform::application;
use std::f32::consts::PI;

struct RotatedText;

fn make_run(text: &str) -> TextRun {
    TextRun {
        len: text.len(),
        font: Default::default(),
        color: black(),
        background_color: None,
        underline: None,
        strikethrough: None,
    }
}

// Build a TransformationMatrix that rotates `angle` around `pivot` in device space.
fn rotation_around(
    pivot: gpui::Point<gpui::Pixels>,
    angle: Radians,
    scale_factor: f32,
) -> TransformationMatrix {
    TransformationMatrix::unit()
        .translate(pivot.scale(scale_factor))
        .rotate(angle)
        .translate(pivot.scale(-scale_factor))
}

impl Render for RotatedText {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().bg(white()).size_full().child(
            canvas(
                |_, _, _| {},
                |_, _, window, cx| {
                    let font_size = px(22.0);
                    let line_height = px(30.0);
                    let sf = window.scale_factor();
                    let text_system = window.text_system().clone();

                    // 1. No rotation — verifies existing behavior is unchanged
                    {
                        let text: gpui::SharedString = "No rotation (baseline)".into();
                        let line = text_system.shape_line(text.clone(), font_size, &[make_run(&text)], None);
                        let _ = line.paint(
                            point(px(40.), px(40.)),
                            line_height,
                            TextAlign::Left,
                            None,
                            window,
                            cx,
                        );
                    }

                    // 2. Rotated 30° around the text's start point
                    {
                        let text: gpui::SharedString = "Rotated 30° clockwise".into();
                        let line = text_system.shape_line(text.clone(), font_size, &[make_run(&text)], None);
                        let origin = point(px(40.), px(120.));
                        let t = rotation_around(origin, Radians(PI / 6.), sf);
                        let _ = line.paint_transformed(
                            origin, line_height, TextAlign::Left, None, t, window, cx,
                        );
                    }

                    // 3. Rotated 45°
                    {
                        let text: gpui::SharedString = "Rotated 45°".into();
                        let line = text_system.shape_line(text.clone(), font_size, &[make_run(&text)], None);
                        let origin = point(px(40.), px(220.));
                        let t = rotation_around(origin, Radians(PI / 4.), sf);
                        let _ = line.paint_transformed(
                            origin, line_height, TextAlign::Left, None, t, window, cx,
                        );
                    }

                    // 4. Rotated 90° (vertical text)
                    {
                        let text: gpui::SharedString = "Vertical 90°".into();
                        let line = text_system.shape_line(text.clone(), font_size, &[make_run(&text)], None);
                        let origin = point(px(500.), px(40.));
                        let t = rotation_around(origin, Radians(PI / 2.), sf);
                        let _ = line.paint_transformed(
                            origin, line_height, TextAlign::Left, None, t, window, cx,
                        );
                    }

                    // 5. Counter-clockwise -30°
                    {
                        let text: gpui::SharedString = "Counter-clockwise -30°".into();
                        let line = text_system.shape_line(text.clone(), font_size, &[make_run(&text)], None);
                        let origin = point(px(40.), px(350.));
                        let t = rotation_around(origin, Radians(-PI / 6.), sf);
                        let _ = line.paint_transformed(
                            origin, line_height, TextAlign::Left, None, t, window, cx,
                        );
                    }

                    // 6a. Emoji, NO rotation (baseline check — if emoji missing here too, it's a pre-existing platform issue)
                    {
                        let text: gpui::SharedString = "Emoji no-rotate: 🎉🔥✨".into();
                        let line = text_system.shape_line(text.clone(), font_size, &[make_run(&text)], None);
                        let _ = line.paint(
                            point(px(40.), px(440.)), line_height, TextAlign::Left, None, window, cx,
                        );
                    }

                    // 6b. Emoji rotated 20° — emoji at START of string to reduce y-offset after rotation
                    {
                        let text: gpui::SharedString = "🎉🔥✨ rotated 20°".into();
                        let line = text_system.shape_line(text.clone(), font_size, &[make_run(&text)], None);
                        let origin = point(px(40.), px(490.));
                        let t = rotation_around(origin, Radians(PI / 9.), sf);
                        let _ = line.paint_transformed(
                            origin, line_height, TextAlign::Left, None, t, window, cx,
                        );
                    }
                },
            )
            .size_full(),
        )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(600.)), cx);
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
