#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, Hsla, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

// https://en.wikipedia.org/wiki/Holy_grail_(web_design)
struct HolyGrailExample {}

impl Render for HolyGrailExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let block = |color: Hsla| {
            div()
                .size_full()
                .bg(color)
                .border_1()
                .border_dashed()
                .rounded_md()
                .border_color(gpui::white())
                .items_center()
        };

        div()
            .gap_1()
            .grid()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .shadow_lg()
            .border_1()
            .size_full()
            .grid_cols(5)
            .grid_rows(5)
            .child(
                block(gpui::white())
                    .row_span(1)
                    .col_span_full()
                    .child("Header"),
            )
            .child(
                block(gpui::red())
                    .col_span(1)
                    .h_56()
                    .child("Table of contents"),
            )
            .child(
                block(gpui::green())
                    .col_span(3)
                    .row_span(3)
                    .child("Content"),
            )
            .child(
                block(gpui::blue())
                    .col_span(1)
                    .row_span(3)
                    .child("AD :(")
                    .text_color(gpui::white()),
            )
            .child(
                block(gpui::black())
                    .row_span(1)
                    .col_span_full()
                    .text_color(gpui::white())
                    .child("Footer"),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| HolyGrailExample {}),
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
