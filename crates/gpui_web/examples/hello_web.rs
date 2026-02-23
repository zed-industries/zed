#![cfg_attr(target_family = "wasm", no_main)]
//! Minimal "Hello, Web!" demo for gpui_web.
//!
//! This example creates a GPUI application using the web platform and opens
//! a window with a simple greeting. It serves as the integration-test target
//! for Phase 0 of the web platform bring-up.
//!
//! Build with:
//!   cargo build -p gpui_web --example hello_web --target wasm32-unknown-unknown
//!
//! Then generate JS bindings:
//!   wasm-bindgen target/wasm32-unknown-unknown/debug/examples/hello_web.wasm --out-dir web/pkg --target web
//!
//! Serve the `web/` directory with any HTTP server and open `index.html`.

use gpui::{
    AppContext, Application, Context, IntoElement, ParentElement, Render, Styled, Window,
    WindowOptions, div, px, rgb,
};
use wasm_bindgen::prelude::*;

struct HelloWeb;

impl Render for HelloWeb {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .items_center()
                    .child(div().text_xl().child("Hello, Web!"))
                    .child(
                        div()
                            .text_color(rgb(0x6c7086))
                            .child("GPUI is running in the browser."),
                    ),
            )
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    gpui_web::init_logging();

    log::info!("GPUI Web starting...");

    let platform = std::rc::Rc::new(gpui_web::WebPlatform::new());
    let application = Application::with_platform(platform);

    application.run(|cx| {
        match cx.open_window(
            WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds::centered(
                    None,
                    gpui::size(px(800.), px(600.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| HelloWeb),
        ) {
            Ok(_) => log::info!("Window opened successfully"),
            Err(err) => log::error!("Failed to open window: {err:#}"),
        }
    });
}
