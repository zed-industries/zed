//! iOS Hello World example.
//!
//! This example demonstrates a basic GPUI app that works on iOS.
//! On iOS, windows are always fullscreen and touch-based.
//!
//! To build for iOS simulator:
//! ```
//! cargo build --target aarch64-apple-ios-sim --example ios_hello --features font-kit
//! ```

use gpui::{App, Application, Context, SharedString, Window, WindowOptions, div, prelude::*, rgb};

struct IosHelloWorld {
    text: SharedString,
    tap_count: u32,
}

impl Render for IosHelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x1e1e2e)) // Dark background
            .size_full() // Full screen on iOS
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xcdd6f4)) // Light text
            .child(format!("Hello, {}!", &self.text))
            .child(format!("Taps: {}", self.tap_count))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .size_16()
                            .bg(rgb(0xf38ba8)) // Rosewater
                            .rounded_md(),
                    )
                    .child(
                        div()
                            .size_16()
                            .bg(rgb(0xa6e3a1)) // Green
                            .rounded_md(),
                    )
                    .child(
                        div()
                            .size_16()
                            .bg(rgb(0x89b4fa)) // Blue
                            .rounded_md(),
                    )
                    .child(
                        div()
                            .size_16()
                            .bg(rgb(0xf9e2af)) // Yellow
                            .rounded_md(),
                    ),
            )
            .child(
                div()
                    .mt_8()
                    .px_6()
                    .py_3()
                    .bg(rgb(0x89b4fa))
                    .rounded_lg()
                    .text_color(rgb(0x1e1e2e))
                    .child("Tap to increment")
                    .on_mouse_down(gpui::MouseButton::Left, |_, _window, _cx| {
                        // This will be triggered by touch on iOS
                        // Note: proper tap handling would increment tap_count
                    }),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // On iOS, we use fullscreen bounds
        cx.open_window(
            WindowOptions {
                // iOS windows are always fullscreen
                window_bounds: None,
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| IosHelloWorld {
                    text: "iOS".into(),
                    tap_count: 0,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
