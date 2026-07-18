#![cfg_attr(target_family = "wasm", no_main)]

//! View example — composing a text input from the `View` primitives.
//!
//! The whole point: a text input is deceptively complicated, and `View` makes it
//! easy to compose one. Three pieces, each shown in its own section:
//!
//!   * `Editor`  — the workhorse entity: cursor, blink, focus, keyboard, and a
//!                 specialized text renderer. All the hard parts live here.
//!   * `String`  — the data plane. `editor.text(cx)` / `value.read(cx)` get it out.
//!   * `Input` / `TextArea` — the shaping layer. Each takes a `String` (and grows
//!                 the editor internally) OR an `Editor` (so you can read the cursor).
//!
//! Run: `cargo run -p gpui --example view_example`

mod example_editor;
mod example_input;
mod example_text_area;

#[cfg(test)]
mod example_tests;

use example_editor::Editor;
use example_input::Input;
use example_text_area::TextArea;

use gpui::{
    App, Bounds, Context, Div, Entity, IntoElement, KeyBinding, Render, SharedString, Window,
    WindowBounds, WindowOptions, actions, div, hsla, prelude::*, px, rgb, size,
};
use gpui_platform::application;

actions!(
    view_example,
    [Backspace, Delete, Left, Right, Home, End, Enter, Quit]
);

/// A tiny stateless view that reads an editor's cursor and is composed *beside*
/// the thing editing it — two views over one entity, zero wiring.
#[derive(IntoElement)]
struct CursorReadout {
    editor: Entity<Editor>,
}

impl CursorReadout {
    fn new(editor: Entity<Editor>) -> Self {
        Self { editor }
    }
}

impl gpui::RenderOnce for CursorReadout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let cursor = self.editor.read(cx).cursor;
        div()
            .text_sm()
            .text_color(hsla(0., 0., 0.45, 1.))
            .child(SharedString::from(format!("cursor @ {cursor}")))
    }
}

struct ViewExample;

impl ViewExample {
    fn new() -> Self {
        Self
    }
}

impl Render for ViewExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The data plane: plain strings, allocated at the top by the hook.
        let name = window.use_state(cx, |_, _| String::new());
        let email = window.use_state(cx, |_, _| String::from("me@example.com"));
        let bio = window.use_state(cx, |_, _| String::new());
        // Editors that own their own string internally — no extra wiring up top.
        let notes = window.use_state(cx, |window, cx| Editor::new("multi\nline", window, cx));
        let owned = window.use_state(cx, |window, cx| Editor::new("editable", window, cx));

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf0f0f0))
            .p(px(24.))
            .gap(px(24.))
            .child(
                section("Inputs — from a String (cursor stays internal)")
                    .child(Input::new(name).width(px(320.)))
                    .child(
                        Input::new(email)
                            .width(px(320.))
                            .color(hsla(0., 0., 0.3, 1.)),
                    ),
            )
            .child(
                section("Input — from an Editor (read its cursor beside it)").child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(12.))
                        .child(Input::editor(owned.clone()).width(px(320.)))
                        .child(CursorReadout::new(owned)),
                ),
            )
            .child(
                section("Text areas — from a String, or from an Editor")
                    .child(TextArea::new(bio, 3))
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap(px(12.))
                            .child(TextArea::editor(notes.clone(), 3).color(hsla(
                                250. / 360.,
                                0.7,
                                0.4,
                                1.,
                            )))
                            .child(CursorReadout::new(notes)),
                    ),
            )
    }
}

/// A labeled vertical section.
fn section(title: &str) -> Div {
    div().flex().flex_col().gap(px(8.)).child(
        div()
            .text_sm()
            .text_color(hsla(0., 0., 0.3, 1.))
            .child(SharedString::from(title.to_string())),
    )
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(560.0), px(480.0)), cx);
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
            |_, cx| cx.new(|_| ViewExample::new()),
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
