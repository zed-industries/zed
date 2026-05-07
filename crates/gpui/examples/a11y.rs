//! Accessibility (AccessKit) demo app.
//!
//! Run with: `cargo run -p gpui --example a11y`
//!
//! Demonstrates core ARIA roles and properties using the GPUI accessibility
//! integration. Use with a screen reader (e.g. Orca on Linux) or an
//! accessibility inspector (e.g. Accerciser) to verify the a11y tree.

use gpui::{
    App, Bounds, Context, FocusHandle, KeyBinding, Role, SharedString, Toggled, Window,
    WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size, text,
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
            // Buttons — screen readers use the built-in Click action,
            // so plain on_click is all that's needed.
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .id("increment")
                            .role(Role::Button)
                            .aria_label(SharedString::from(format!(
                                "Count is {}. Click to increment.",
                                self.count
                            )))
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x89b4fa))
                            .text_color(rgb(0x1e1e2e))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count += 1;
                                cx.notify();
                            }))
                            .child(text!(format!("Count: {}", self.count))),
                    )
                    .child(
                        div()
                            .id("reset")
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
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
