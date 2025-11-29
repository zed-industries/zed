// Minimal example demonstrating IME-friendly text input on Windows.
// This is intentionally small and mirrors gpui examples/input.rs but focuses
// on EntityInputHandler usage so IME composition works correctly.

use gpui::prelude::*;
// Make the example self-contained by importing the concrete types/functions
use gpui::{Application, WindowOptions, div, text};

fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| {
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .child(text("IME input example"))
            })
        })
        .unwrap();
    });
}
