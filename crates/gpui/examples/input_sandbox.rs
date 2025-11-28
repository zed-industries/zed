//! Input Sandbox - A simple example for testing single-line and multi-line inputs.
//!
//! Run with: `cargo run -p gpui --example input_sandbox`

use gpui::{
    App, Application, Bounds, Context, Entity, FocusHandle, Focusable, InputState, KeyBinding,
    Window, WindowBounds, WindowOptions, div, input, prelude::*, px, rgb, size, text_area,
};

use gpui::input::{
    Backspace, Copy, Cut, Delete, Down, End, Enter, Home, Left, MoveToBeginning, MoveToEnd, Paste,
    Right, SelectAll, SelectDown, SelectLeft, SelectRight, SelectToBeginning, SelectToEnd,
    SelectUp, SelectWordLeft, SelectWordRight, Tab, Up, WordLeft, WordRight,
};

struct InputSandbox {
    multiline_input: Entity<InputState>,
    singleline_input: Entity<InputState>,
    use_multiline: bool,
}

impl InputSandbox {
    fn new(cx: &mut Context<Self>) -> Self {
        let multiline_input = cx.new(|cx| {
            let mut input = InputState::new_multiline(cx);
            input.set_content("Multi-line text.\nLine 2.\nLine 3.", cx);
            input
        });

        let singleline_input = cx.new(|cx| {
            let mut input = InputState::new_singleline(cx);
            input.set_content("Single-line text", cx);
            input
        });

        Self {
            multiline_input,
            singleline_input,
            use_multiline: true,
        }
    }

    fn toggle_mode(&mut self, _: &ToggleMode, _window: &mut Window, cx: &mut Context<Self>) {
        self.use_multiline = !self.use_multiline;
        cx.notify();
    }

    fn active_input(&self) -> &Entity<InputState> {
        if self.use_multiline {
            &self.multiline_input
        } else {
            &self.singleline_input
        }
    }
}

impl Focusable for InputSandbox {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_input().focus_handle(cx)
    }
}

impl Render for InputSandbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_input = self.active_input().clone();
        let input_state = active_input.read(cx);
        let content = input_state.content().to_string();
        let selected_range = input_state.selected_range().clone();
        let cursor_offset = input_state.cursor_offset();
        let char_count = content.chars().count();
        let line_count = content.lines().count().max(1);

        let focus_handle = active_input.focus_handle(cx);
        let mode_label = if self.use_multiline {
            "Multi-line"
        } else {
            "Single-line"
        };

        div()
            .id("input-sandbox")
            .key_context("InputSandbox")
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::toggle_mode))
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .bg(rgb(0x1a1a1a))
            .text_color(rgb(0xcccccc))
            .text_sm()
            .size_full()
            // Header
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Input Sandbox"),
                    )
                    .child(
                        div()
                            .id("toggle-btn")
                            .px_2()
                            .py_1()
                            .bg(rgb(0x333333))
                            .rounded_sm()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x444444)))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.toggle_mode(&ToggleMode, window, cx);
                            }))
                            .child(format!("Mode: {} (click to toggle)", mode_label)),
                    ),
            )
            // Input area
            .child(
                div()
                    .flex_1()
                    .min_h(px(100.))
                    .border_1()
                    .border_color(rgb(0x444444))
                    .rounded_sm()
                    .overflow_hidden()
                    .when(self.use_multiline, |this| {
                        this.child(
                            text_area(&self.multiline_input)
                                .size_full()
                                .p_2()
                                .bg(rgb(0x2a2a2a))
                                .text_color(rgb(0xffffff))
                                .selection_color(gpui::rgba(0x3388ff44))
                                .cursor_color(rgb(0xffffff)),
                        )
                    })
                    .when(!self.use_multiline, |this| {
                        this.child(
                            div().h(px(36.)).child(
                                input(&self.singleline_input)
                                    .size_full()
                                    .px_2()
                                    .bg(rgb(0x2a2a2a))
                                    .text_color(rgb(0xffffff))
                                    .selection_color(gpui::rgba(0x3388ff44))
                                    .cursor_color(rgb(0xffffff)),
                            ),
                        )
                    }),
            )
            // Info panel
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .p_2()
                    .bg(rgb(0x222222))
                    .rounded_sm()
                    .text_xs()
                    .text_color(rgb(0x888888))
                    .child(format!("Cursor: {}", cursor_offset))
                    .child(format!(
                        "Selection: {}..{}",
                        selected_range.start, selected_range.end
                    ))
                    .child(format!("Chars: {} | Lines: {}", char_count, line_count))
                    .child(format!("Content length (bytes): {}", content.len())),
            )
            // Keybinding hints
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .text_xs()
                    .text_color(rgb(0x666666))
                    .child(key_hint("Ctrl+T", "Toggle mode"))
                    .child(key_hint("Cmd+A", "Select all"))
                    .child(key_hint("Cmd+C/X/V", "Copy/Cut/Paste"))
                    .child(key_hint("Alt+←/→", "Word nav"))
                    .child(key_hint("Cmd+↑/↓", "Doc start/end")),
            )
    }
}

fn key_hint(key: &str, desc: &str) -> impl IntoElement {
    div()
        .px_1()
        .bg(rgb(0x2a2a2a))
        .rounded_sm()
        .child(format!("{}: {}", key, desc))
}

gpui::actions!(input_sandbox, [ToggleMode]);

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("ctrl-t", ToggleMode, None),
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
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new("backspace", Backspace, None),
        ]);

        let bounds = Bounds::centered(None, size(px(500.), px(400.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(InputSandbox::new);
                let focus_handle = view.read(cx).active_input().focus_handle(cx);
                window.focus(&focus_handle);
                view
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
