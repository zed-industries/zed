//! Demonstrates GPUI's accessibility annotation API.
//!
//! Left panel: interactive UI (buttons + checkboxes + live status region).
//! Right panel: live text dump of `window.accessibility_tree()`.
//!
//! To verify platform accessibility, open **Accessibility Inspector** (ships with
//! Xcode) and point it at this window. The hierarchy shown there should match
//! the tree on the right. Click "Open Accessibility Inspector docs" in the UI to
//! go straight to the Apple developer docs.
#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, FocusHandle, Live, Role, SharedString, Stateful, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, size,
};
use gpui_platform::application;

actions!(accessibility_example, [Quit]);

struct AccessibilityExample {
    count: usize,
    muted: bool,
    option_a: bool,
    option_b: bool,
    status: SharedString,
    increment_focus: FocusHandle,
    mute_focus: FocusHandle,
    option_a_focus: FocusHandle,
    option_b_focus: FocusHandle,
}

impl AccessibilityExample {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            count: 0,
            muted: false,
            option_a: true,
            option_b: false,
            status: SharedString::from("No action yet."),
            increment_focus: cx.focus_handle(),
            mute_focus: cx.focus_handle(),
            option_a_focus: cx.focus_handle(),
            option_b_focus: cx.focus_handle(),
        }
    }

    fn increment(&mut self, _: &gpui::ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        self.status = SharedString::from(format!("Counter updated to {}.", self.count));
        cx.notify();
    }

    fn toggle_mute(&mut self, _: &gpui::ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.muted = !self.muted;
        self.status = SharedString::from(if self.muted { "Muted." } else { "Unmuted." });
        cx.notify();
    }

    fn toggle_a(&mut self, _: &gpui::ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.option_a = !self.option_a;
        self.status = SharedString::from(if self.option_a {
            "Option A enabled."
        } else {
            "Option A disabled."
        });
        cx.notify();
    }

    fn toggle_b(&mut self, _: &gpui::ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.option_b = !self.option_b;
        self.status = SharedString::from(if self.option_b {
            "Option B enabled."
        } else {
            "Option B disabled."
        });
        cx.notify();
    }
}

/// Base button: `bg(black)` + white text. Chain extra methods for variants.
fn button(id: &'static str, label: &'static str) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .role(Role::Button)
        .aria_label(label)
        .px_3()
        .py_1()
        .rounded_md()
        .bg(gpui::black())
        .text_color(gpui::white())
        .cursor_pointer()
        .child(label)
}

fn checkbox(id: &'static str, label: &'static str, checked: bool) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .role(Role::CheckBox)
        .aria_label(label)
        .aria_checked(checked)
        .cursor_pointer()
        .child(
            div()
                .size(px(16.))
                .border_1()
                .border_color(gpui::black())
                .when(checked, |el| el.bg(gpui::black())),
        )
        .child(label)
}

fn section_label(text: &'static str) -> gpui::Div {
    div()
        .text_color(gpui::black().opacity(0.4))
        .child(text)
}

impl Render for AccessibilityExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tree_text = window.accessibility_tree().to_string();

        div()
            .size_full()
            .flex()
            .flex_row()
            .bg(gpui::white())
            .text_color(gpui::black())
            .text_sm()
            // Left panel
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .w(px(240.))
                    // Buttons section
                    .child(section_label("BUTTONS"))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            // Default
                            .child(
                                button("btn-default", "Default")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.status = "Default clicked.".into();
                                        cx.notify();
                                    })),
                            )
                            // Primary (blue)
                            .child(
                                button("btn-primary", "Primary")
                                    .bg(gpui::blue())
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.status = "Primary clicked.".into();
                                        cx.notify();
                                    })),
                            )
                            // Danger (red, destructive)
                            .child(
                                button("btn-danger", "Danger")
                                    .bg(gpui::red())
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.status = "Danger clicked.".into();
                                        cx.notify();
                                    })),
                            )
                            // Disabled — aria_disabled + no on_click
                            .child(
                                button("btn-disabled", "Disabled")
                                    .aria_disabled(true)
                                    .bg(gpui::black().opacity(0.25))
                                    .cursor_not_allowed(),
                            )
                            // Toggle (aria_pressed) — press to mute/unmute
                            .child(
                                button("btn-mute", if self.muted { "Unmute" } else { "Mute" })
                                    .aria_pressed(self.muted)
                                    .track_focus(&self.mute_focus)
                                    .when(self.muted, |el| el.bg(gpui::blue()))
                                    .on_click(cx.listener(Self::toggle_mute)),
                            )
                            // Counter with increment
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .role(Role::Label)
                                            .aria_label("Counter value")
                                            .child(SharedString::from(format!(
                                                "Count: {}",
                                                self.count
                                            ))),
                                    )
                                    .child(
                                        button("btn-increment", "Increment")
                                            .track_focus(&self.increment_focus)
                                            .on_click(cx.listener(Self::increment)),
                                    ),
                            ),
                    )
                    // Checkboxes section
                    .child(section_label("CHECKBOXES"))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                checkbox("option-a", "Option A", self.option_a)
                                    .track_focus(&self.option_a_focus)
                                    .on_click(cx.listener(Self::toggle_a)),
                            )
                            .child(
                                checkbox("option-b", "Option B", self.option_b)
                                    .track_focus(&self.option_b_focus)
                                    .on_click(cx.listener(Self::toggle_b)),
                            ),
                    )
                    // Live status region
                    .child(
                        div()
                            .role(Role::Status)
                            .aria_live(Live::Polite)
                            .text_color(gpui::black().opacity(0.5))
                            .child(self.status.clone()),
                    )
                    // Inspector link
                    .child(
                        div()
                            .id("inspector-link")
                            .mt_2()
                            .cursor_pointer()
                            .text_color(gpui::black().opacity(0.4))
                            .on_click(|_, _, cx| {
                                cx.open_url("https://developer.apple.com/documentation/accessibility/accessibility-inspector");
                            })
                            .child("Open Accessibility Inspector docs ↗"),
                    ),
            )
            // Right panel: live accessibility tree dump
            .child(
                div()
                    .flex_1()
                    .p_3()
                    .bg(gpui::black().opacity(0.05))
                    .text_color(gpui::black())
                    .font_family("monospace")
                    .overflow_hidden()
                    .child(tree_text),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        cx.bind_keys([gpui::KeyBinding::new("cmd-q", Quit, None)]);

        let bounds = Bounds::centered(None, size(px(720.), px(480.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| AccessibilityExample::new(window, cx)),
        )
        .unwrap();

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
