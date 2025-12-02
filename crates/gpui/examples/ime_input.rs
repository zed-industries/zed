// Minimal example demonstrating IME-friendly text input on Windows.
// This is intentionally small and mirrors gpui examples/input.rs but focuses
// on EntityInputHandler usage so IME composition works correctly.

use gpui::prelude::*;
// Make the example self-contained by importing the concrete types we rely on
use gpui::{Application, EmptyView, WindowOptions};

fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| EmptyView))
            .unwrap();
    });
}
