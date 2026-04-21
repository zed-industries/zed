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
    App, Bounds, Context, FocusHandle, KeyDownEvent, Live, Role, SharedString, Stateful, Window,
    WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
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
    username: String,
    username_focus: FocusHandle,
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
            username: String::new(),
            username_focus: cx.focus_handle(),
        }
    }

    fn handle_username_keydown(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.keystroke.key == "backspace" {
            self.username.pop();
            cx.notify();
        } else if let Some(ch) = &event.keystroke.key_char {
            if !event.keystroke.modifiers.platform
                && !event.keystroke.modifiers.control
                && ch.chars().all(|c| !c.is_control())
            {
                self.username.push_str(ch);
                cx.notify();
            }
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
    div().text_color(gpui::black().opacity(0.4)).child(text)
}

fn text_input_label(text: &'static str) -> gpui::Div {
    div()
        .text_xs()
        .text_color(gpui::black().opacity(0.5))
        .child(text)
}

fn text_input(id: &'static str, value: &str) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .role(Role::TextInput)
        .px_2()
        .py_1()
        .w_full()
        .border_1()
        .border_color(gpui::black().opacity(0.3))
        .rounded_sm()
        .bg(gpui::white())
        .cursor_text()
        .child(SharedString::from(value.to_owned()))
}

impl Render for AccessibilityExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tree_text = window.accessibility_tree().to_string();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(gpui::white())
            .text_color(gpui::black())
            .text_sm()
            // Top row: left panel + right panel
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .child(
                        // Left panel
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .p_4()
                            .w(px(240.))
                            .border_r_1()
                            .border_color(gpui::black().opacity(0.1))
                            // Buttons section
                            .child(section_label("BUTTONS"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    // Default
                                    .child(button("btn-default", "Button 1").on_click(cx.listener(
                                        |this, _, _, cx| {
                                            this.status = "Default clicked.".into();
                                            cx.notify();
                                        },
                                    )))
                                    // Primary (blue)
                                    .child(
                                        button("btn-primary", "OK").bg(gpui::blue()).on_click(
                                            cx.listener(|this, _, _, cx| {
                                                this.status = "Primary clicked.".into();
                                                cx.notify();
                                            }),
                                        ),
                                    )
                                    // Danger (red, destructive)
                                    .child(button("btn-danger", "Delete").bg(gpui::red()).on_click(
                                        cx.listener(|this, _, _, cx| {
                                            this.status = "Danger clicked.".into();
                                            cx.notify();
                                        }),
                                    ))
                                    // Disabled — aria_disabled + no on_click
                                    .child(
                                        button("btn-disabled", "Upload")
                                            .aria_disabled(true)
                                            .bg(gpui::black().opacity(0.25))
                                            .cursor_not_allowed(),
                                    )
                                    // Toggle (aria_pressed) — press to mute/unmute
                                    .child(
                                        button(
                                            "btn-mute",
                                            if self.muted { "Unmute" } else { "Mute" },
                                        )
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
                            // Text inputs section
                            .child(section_label("TEXT INPUTS"))
                            .child(
                                div().flex().flex_col().gap_3()
                                    // Editable input — required
                                    .child(text_input_label("Username (required)"))
                                    .child(
                                        text_input(
                                            "input-username",
                                            if self.username.is_empty() {
                                                "Type here…"
                                            } else {
                                                &self.username
                                            },
                                        )
                                        .aria_label("Username")
                                        .aria_required(true)
                                        .track_focus(&self.username_focus)
                                        .on_key_down(cx.listener(Self::handle_username_keydown))
                                        .when(self.username.is_empty(), |el| {
                                            el.text_color(gpui::black().opacity(0.3))
                                        }),
                                    )
                                    // Read-only input
                                    .child(text_input_label("Version (read-only)"))
                                    .child(
                                        text_input("input-version", "1.0.0")
                                            .aria_label("Version")
                                            .aria_readonly(true)
                                            .text_color(gpui::black().opacity(0.5))
                                            .bg(gpui::black().opacity(0.05)),
                                    ),
                            )
                            // Live status region
                            .child(
                                div()
                                    .role(Role::Status)
                                    .aria_live(Live::Polite)
                                    .text_color(gpui::black().opacity(0.5))
                                    .child(self.status.clone()),
                            ),
                    )
                    .child(
                        div()
                            .id("inspector-tree")
                            .flex_1()
                            .p_3()
                            .bg(gpui::black().opacity(0.05))
                            .text_color(gpui::black())
                            .font_family("monospace")
                            .overflow_hidden()
                            .child(tree_text),
                    ),
            )
            .child(
                div()
                    .id("inspector-link")
                    .py_1()
                    .px_2()
                    .border_t_1()
                    .border_color(gpui::black().opacity(0.1))
                    .text_color(gpui::black().opacity(0.4))
                    .cursor_pointer()
                    .on_click(|_, _, cx| {
                        cx.open_url(
                            "https://developer.apple.com/documentation/accessibility/accessibility-inspector",
                        );
                    })
                    .child("Open Accessibility Inspector docs ↗"),
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
            |window, cx| {
                window.set_window_title("GPUI Accessibility Example");
                cx.new(|cx| AccessibilityExample::new(window, cx))
            },
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
