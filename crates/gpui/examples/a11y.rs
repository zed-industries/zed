#![cfg_attr(target_family = "wasm", no_main)]

//! Accessibility (AccessKit) demo app.
//!
//! Run with: `cargo run -p gpui --example a11y`
//!
//! Or on Linux: `cargo run -p gpui --features gpui_platform/wayland,gpui_platform/x11 --example a11y`
//!
//! This app uses GPUI's accessibility APIs to attach structured information to
//! the element tree, which allows assistive technology to see and interact with
//! the UI programmatically.
//!
//! The app behaves as follows:
//! - It opens a single window.
//! - The window's title is "GPUI Accessibility Demo".
//! - The window has a sequence of UI elements, stacked vertically:
//!   - A heading with the text "Accessibility Demo".
//!   - A row containing two elements:
//!     - A spin button (role `SpinButton`) labelled "Counter: <n>", where
//!       `<n>` is the current count. It supports `Increment` and `Decrement`
//!       accessible actions, and also increments on click. The numeric value
//!       is clamped to a minimum of 0.
//!     - A button labelled "Reset counter" that resets the count to 0.
//!   - A row containing two elements:
//!     - A switch, that can be toggled, and starts disabled. Toggling the switch
//!       does nothing.
//!     - The text "Enable feature".
//!   - A "to-do" list, with three items, each represented with a `Text` element:
//!     - "1. Write code"
//!     - "2. Run tests"
//!     - "3. Ship it"

use gpui::{
    AccessibleAction, App, Bounds, Context, FocusHandle, KeyBinding, Role, SharedString, Toggled,
    Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size, text,
};
use gpui_platform::application;

actions!(a11y_example, [Tab, TabPrev]);

struct A11yDemo {
    focus_handle: FocusHandle,
    count: i32,
    enabled: bool,
}

impl A11yDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle, cx);
        Self {
            focus_handle,
            count: 0,
            enabled: false,
        }
    }
}

impl Render for A11yDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("root")
            .role(Role::Application)
            .aria_label("Accessibility Demo")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|_, _: &Tab, window, cx| window.focus_next(cx)))
            .on_action(cx.listener(|_, _: &TabPrev, window, cx| window.focus_prev(cx)))
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            // Heading
            .child(
                div()
                    .id("heading")
                    .role(Role::Heading)
                    .aria_level(1)
                    .aria_label("Accessibility Demo")
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(text!("Accessibility Demo")),
            )
            // Counter — uses a SpinButton role with Increment/Decrement
            // actions so screen readers can adjust the value directly.
            // Click also works via the built-in handler.
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .id("counter")
                            .focusable()
                            .tab_stop(true)
                            .role(Role::SpinButton)
                            .aria_label(SharedString::from(format!("Counter: {}", self.count)))
                            .aria_numeric_value(self.count as f64)
                            .aria_min_numeric_value(0.0)
                            .on_a11y_action(AccessibleAction::Increment, {
                                let this = cx.entity().downgrade();
                                move |_, _, cx| {
                                    this.update(cx, |this, cx| {
                                        this.count += 1;
                                        cx.notify();
                                    })
                                    .ok();
                                }
                            })
                            .on_a11y_action(AccessibleAction::Decrement, {
                                let this = cx.entity().downgrade();
                                move |_, _, cx| {
                                    this.update(cx, |this, cx| {
                                        this.count = (this.count - 1).max(0);
                                        cx.notify();
                                    })
                                    .ok();
                                }
                            })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count += 1;
                                cx.notify();
                            }))
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x89b4fa))
                            .text_color(rgb(0x1e1e2e))
                            .cursor_pointer()
                            .child(text!(format!("Count: {}", self.count))),
                    )
                    .child(
                        div()
                            .id("reset")
                            .focusable()
                            .tab_stop(true)
                            .role(Role::Button)
                            .aria_label("Reset counter")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x585b70))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count = 0;
                                cx.notify();
                            }))
                            .child(text!("Reset")),
                    ),
            )
            // A toggle switch
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .id("toggle")
                            .focusable()
                            .tab_stop(true)
                            .role(Role::Switch)
                            .aria_label("Enable feature")
                            .aria_toggled(if self.enabled {
                                Toggled::True
                            } else {
                                Toggled::False
                            })
                            .w(px(44.))
                            .h(px(24.))
                            .rounded_full()
                            .cursor_pointer()
                            .when(self.enabled, |el| el.bg(rgb(0x89b4fa)))
                            .when(!self.enabled, |el| el.bg(rgb(0x585b70)))
                            .child(
                                div()
                                    .size(px(20.))
                                    .rounded_full()
                                    .bg(gpui::white())
                                    .mt(px(2.))
                                    .when(self.enabled, |el| el.ml(px(22.)))
                                    .when(!self.enabled, |el| el.ml(px(2.))),
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.enabled = !this.enabled;
                                cx.notify();
                            })),
                    )
                    .child(text!("Enable feature")),
            )
            // A short list
            .child(
                div()
                    .id("task-list")
                    .role(Role::List)
                    .aria_label("Tasks")
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(
                        ["Write code", "Run tests", "Ship it"]
                            .iter()
                            .enumerate()
                            .map(|(i, label)| {
                                div()
                                    .id(("task", i))
                                    .role(Role::ListItem)
                                    .aria_label(SharedString::from(*label))
                                    .aria_position_in_set(i + 1)
                                    .aria_size_of_set(3)
                                    .py_1()
                                    .px_2()
                                    // Note: even though this `text!` macro
                                    // produces multiple elements, it doesn't
                                    // need its own unique ID because the parent
                                    // div has different IDs for each string.
                                    .child(text!(format!("{}. {}", i + 1, label)))
                            }),
                    ),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new("shift-tab", TabPrev, None),
        ]);

        let bounds = Bounds::centered(None, size(px(500.), px(400.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("GPUI Accessibility Demo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| A11yDemo::new(window, cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("gpui", log::LevelFilter::Info)
        .init();
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
