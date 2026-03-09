#![cfg_attr(target_family = "wasm", no_main)]

//! **text_views** — an end-to-end GPUI example demonstrating how Entity,
//! Element, View, and Render compose together to build rich text components.
//!
//! ## Architecture
//!
//! Each module has a focused job:
//!
//! | Module          | Layer   | Job                                                      |
//! |-----------------|---------|----------------------------------------------------------|
//! | `editor`        | Entity  | Owns text, cursor, blink task, `EntityInputHandler`      |
//! | `editor_text`   | Element | Shapes text, paints cursor, wires `handle_input`         |
//! | `input`         | View    | Single-line input — composes `EditorText` with styling   |
//! | `text_area`     | View    | Multi-line text area — same entity, different layout      |
//! | `main` (here)   | Render  | Root view — creates entities with `use_state`, assembles  |
//!
//! ## Running
//!
//! ```sh
//! cargo run --example text_views -p gpui
//! ```
//!
//! ## Testing
//!
//! ```sh
//! cargo test --example text_views -p gpui
//! ```

mod editor;
mod input;
mod text_area;

#[cfg(test)]
mod editor_test;

use gpui::{
    App, Bounds, Context, Hsla, KeyBinding, Window, WindowBounds, WindowOptions, actions, div,
    hsla, prelude::*, px, rgb, size,
};
use gpui_platform::application;

use editor::Editor;
use input::Input;
use text_area::TextArea;

actions!(
    text_views,
    [Backspace, Delete, Left, Right, Home, End, Enter, Quit,]
);

// ---------------------------------------------------------------------------
// Example — the root view using `Render` and `window.use_state()`
// ---------------------------------------------------------------------------

struct Example {
    input_color: Hsla,
    textarea_color: Hsla,
}

impl Example {
    fn new() -> Self {
        Self {
            input_color: hsla(0., 0., 0.1, 1.),
            textarea_color: hsla(250. / 360., 0.7, 0.4, 1.),
        }
    }
}

impl Render for Example {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input_editor = window.use_state(cx, |_window, cx| Editor::new(cx));
        let textarea_editor = window.use_state(cx, |_window, cx| Editor::new(cx));
        let input_color = self.input_color;
        let textarea_color = self.textarea_color;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf0f0f0))
            .p(px(24.))
            .gap(px(20.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .child(
                        div()
                            .text_sm()
                            .text_color(hsla(0., 0., 0.3, 1.))
                            .child("Single-line input (Input — View with cached EditorText)"),
                    )
                    .child(Input::new(input_editor).width(px(320.)).color(input_color)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .child(div().text_sm().text_color(hsla(0., 0., 0.3, 1.)).child(
                        "Multi-line text area (TextArea — same entity type, different View)",
                    ))
                    .child(TextArea::new(textarea_editor, 5).color(textarea_color)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .mt(px(12.))
                    .text_xs()
                    .text_color(hsla(0., 0., 0.5, 1.))
                    .child("• Editor entity owns state, blink task, EntityInputHandler")
                    .child("• EditorText element shapes text, paints cursor, wires handle_input")
                    .child("• Input / TextArea views compose EditorText with container styling")
                    .child("• ViewElement::cached() enables render caching via #[derive(Hash)]")
                    .child("• Entities created via window.use_state()"),
            )
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("enter", Enter, None),
            KeyBinding::new("cmd-q", Quit, None),
        ]);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Example::new()),
        )
        .unwrap();

        cx.on_action(|_: &Quit, cx| cx.quit());
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
