#![cfg_attr(target_family = "wasm", no_main)]

use std::time::Duration;

use gpui::{
    App, Bounds, Context, DurationWithEasing as _, Window, WindowBounds, WindowOptions, div,
    ease_in_out, prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct StyleTransitionsExample;

impl Render for StyleTransitionsExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x110f15))
            .child(
                div()
                    .id("transition-button")
                    .cursor_pointer()
                    .rounded_full()
                    .px(px(18.0))
                    .py(px(12.0))
                    .bg(rgb(0x663399))
                    .text_color(rgb(0xffffff))
                    .child("Click me!")
                    .transitions(|transitions| {
                        transitions
                            .bg(Duration::from_millis(200).with_easing(ease_in_out))
                            .rounded(Duration::from_millis(200).with_easing(ease_in_out))
                    })
                    .hover(|refinement| refinement.bg(rgb(0x4F207E)).rounded(px(10.0)))
                    .active(|refinement| refinement.bg(rgb(0x3F1965)).rounded(px(0.))),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        let result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| StyleTransitionsExample),
        );
        if let Err(error) = result {
            eprintln!("failed to open style transitions example: {error:#}");
            cx.quit();
            return;
        }
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
