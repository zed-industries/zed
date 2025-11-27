//! Example demonstrating the `Input` and `TextArea` components.
//!
//! Run with: `cargo run -p gpui --example text_area`

use gpui::{
    App, Application, Bounds, Context, Entity, FocusHandle, Focusable, InputState, KeyBinding,
    Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, rgba, size, text_area,
};

// todo: move to keymap
use gpui::input::{
    Backspace, Copy, Cut, Delete, Down, End, Enter, Home, Left, MoveToBeginning, MoveToEnd, Paste,
    Right, SelectAll, SelectDown, SelectLeft, SelectRight, SelectToBeginning, SelectToEnd,
    SelectUp, SelectWordLeft, SelectWordRight, Tab, Up, WordLeft, WordRight,
};

struct TextAreaExample {
    input: Entity<InputState>,
}

impl TextAreaExample {
    fn new(cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let mut input = InputState::new(cx);
            input.set_content("Hello, world!\n\nThis is a multi-line text area.\nTry typing, selecting text, and scrolling.", cx);
            input.set_placeholder("Type something...", cx);
            input
        });

        Self { input }
    }
}

impl Focusable for TextAreaExample {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl Render for TextAreaExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.input.focus_handle(cx);
        div()
            .id("text-area-example")
            .key_context("TextAreaExample")
            .track_focus(&focus_handle)
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .child(
                div()
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .child("TextArea Example"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x888888))
                    .child("A multi-line text input component using Input + TextArea"),
            )
            .child(
                div()
                    .flex_1()
                    .border_1()
                    .border_color(rgb(0x444444))
                    .rounded_md()
                    .overflow_hidden()
                    .child(
                        text_area(&self.input)
                            .size_full()
                            .p_2()
                            .bg(rgb(0x2d2d2d))
                            .text_color(rgb(0xffffff))
                            .text_sm()
                            .selection_color(rgba(0x264f7844))
                            .cursor_color(rgb(0xffffff)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(rgb(0x333333))
                            .rounded_sm()
                            .text_xs()
                            .text_color(rgb(0x888888))
                            .child(format!(
                                "Characters: {}",
                                self.input.read(cx).content().len()
                            )),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(rgb(0x333333))
                            .rounded_sm()
                            .text_xs()
                            .text_color(rgb(0x888888))
                            .child(format!(
                                "Selection: {:?}",
                                self.input.read(cx).selected_range()
                            )),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // todo: move to keymap
        cx.bind_keys([
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("up", Up, None),
            KeyBinding::new("down", Down, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("shift-up", SelectUp, None),
            KeyBinding::new("shift-down", SelectDown, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("cmd-up", MoveToBeginning, None),
            KeyBinding::new("cmd-down", MoveToEnd, None),
            KeyBinding::new("cmd-shift-up", SelectToBeginning, None),
            KeyBinding::new("cmd-shift-down", SelectToEnd, None),
            KeyBinding::new("alt-left", WordLeft, None),
            KeyBinding::new("alt-right", WordRight, None),
            KeyBinding::new("alt-shift-left", SelectWordLeft, None),
            KeyBinding::new("alt-shift-right", SelectWordRight, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("enter", Enter, None),
            KeyBinding::new("shift-enter", Enter, None),
            KeyBinding::new("alt-enter", Enter, None),
            KeyBinding::new("ctrl-enter", Enter, None),
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("shift-backspace", Backspace, None),
            KeyBinding::new("alt-backspace", Backspace, None),
            KeyBinding::new("ctrl-backspace", Backspace, None),
        ]);

        let bounds = Bounds::centered(None, size(px(600.), px(400.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(TextAreaExample::new);
                // Focus the input's focus handle so it receives key events
                let focus_handle = view.read(cx).input.focus_handle(cx);
                window.focus(&focus_handle);
                view
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
