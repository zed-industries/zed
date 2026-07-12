//! Demonstrates rounded-corner clipping: children of a container with rounded
//! corners and hidden overflow are clipped to the container's corner arcs, not
//! just its rectangular bounds.
#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb,
    size,
};
use gpui_platform::application;

struct RoundedClippingDemo;

impl Render for RoundedClippingDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .gap_8()
            .items_center()
            .justify_center()
            .bg(rgb(0x1e1e2e))
            .child(
                // A rounded container whose child would poke out of the corners
                // without rounded clipping.
                div()
                    .size(px(200.))
                    .rounded(px(48.))
                    .overflow_hidden()
                    .bg(rgb(0x89b4fa))
                    .child(div().size(px(200.)).bg(rgb(0xf38ba8)).rounded(px(0.))),
            )
            .child(
                // A rounded scroll container: the scrolled content (text lines)
                // stays inside the corner arcs while scrolling.
                div()
                    .id("scroller")
                    .size(px(200.))
                    .rounded(px(48.))
                    .overflow_y_scroll()
                    .bg(rgb(0xa6e3a1))
                    .text_color(rgb(0x11111b))
                    .children((0..50).map(|ix| div().px_2().child(format!("Line {ix}")))),
            )
            .child(
                // Nested rounded clips: the inner clip's corners apply within
                // the outer clip's corners.
                div()
                    .size(px(200.))
                    .rounded(px(64.))
                    .overflow_hidden()
                    .bg(rgb(0xf9e2af))
                    .child(
                        div()
                            .ml(px(-20.))
                            .mt(px(-20.))
                            .size(px(120.))
                            .rounded(px(32.))
                            .overflow_hidden()
                            .bg(rgb(0xcba6f7))
                            .child(div().size(px(120.)).bg(rgb(0x94e2d5))),
                    ),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.), px(400.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| cx.new(|_| RoundedClippingDemo),
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
