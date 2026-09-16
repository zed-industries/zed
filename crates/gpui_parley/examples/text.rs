//! Demonstrates injecting the out-of-tree Parley text system into an app.
//!
//! This example is compile-checked; it does not need to be run to prove that
//! `ParleyTextSystem` implements the public `TextSystem` SPI and can be wired in
//! through `Application::with_text_system`.

use gpui::{
    App, Bounds, Context, Render, SharedString, Window, WindowBounds, WindowOptions, application,
    div, prelude::*, px, rgb, size,
};
use gpui_parley::ParleyTextSystem;

struct Text {
    label: SharedString,
}

impl Render for Text {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(0x202020))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(self.label.clone())
    }
}

fn main() {
    application()
        .with_text_system(ParleyTextSystem::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(400.0), px(200.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| Text {
                        label: "Shaped by Parley".into(),
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
