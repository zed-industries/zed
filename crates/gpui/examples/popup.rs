//! Native parent-anchored popups (macOS and Wayland).
//! Run with `cargo run -p gpui --example popup`.
//!
//! Move the parent near a screen edge to test flipping, resize an open popup, open
//! nested menus, press Escape, click another application, and close the parent.
//! Passive popups should not take focus or dismiss when switching applications.
//! Use Tab / Shift-Tab to move focus and Enter / Space to activate a button.

#![cfg_attr(target_family = "wasm", no_main)]

#[path = "example_support/fonts.rs"]
mod example_support;

use gpui::{
    AccessibleAction, AnyWindowHandle, App, Bounds, Context, Div, FocusHandle, KeyBinding,
    MouseButton, Pixels, Role, SharedString, Stateful, Window, WindowBounds, WindowHandle,
    WindowKind, WindowOptions, actions, div, point, popup::*, prelude::*, px, rgb, size,
};
use gpui_platform::application;
use std::rc::Rc;

actions!(popup_example, [Tab, TabPrev]);

struct PopupExample {
    focus: FocusHandle,
    popup: Option<WindowHandle<PopupContent>>,
    status: SharedString,
}

impl PopupExample {
    fn close_popup(&mut self, cx: &mut App) {
        if let Some(popup) = self.popup.take()
            && cx.windows().contains(&popup.into())
            && let Err(error) = popup.update(cx, |_, window, _| window.remove_window())
        {
            self.status = format!("Could not close popup: {error:#}").into();
        }
    }
}

impl Render for PopupExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let parent = window.window_handle();
        div()
            .track_focus(&self.focus)
            .key_context("PopupExample")
            .on_action(|_: &Tab, window, cx| window.focus_next(cx))
            .on_action(|_: &TabPrev, window, cx| window.focus_prev(cx))
            .size_full()
            .bg(rgb(0x242830))
            .text_color(rgb(0xe6e8ec))
            .p_5()
            .flex()
            .flex_col()
            .gap_3()
            .child(div().text_xl().child("Native anchored popups"))
            .child("Move this window near a screen edge to test flipping.")
            .child("Escape closes a menu; clicking another app closes its menu chain.")
            .child(self.status.clone())
            .children(
                [(false, "Open menu"), (true, "Open passive popup")]
                    .into_iter()
                    .enumerate()
                    .map(|(index, (passive, label))| {
                        let anchor_rect = Bounds::new(
                            point(px(24. + index as f32 * 220.), px(210.)),
                            size(px(200.), px(36.)),
                        );
                        popup_button(
                            label,
                            cx.processor(move |this, (), _, cx| {
                                this.close_popup(cx);
                                match open_popup(parent, anchor_rect, !passive, cx) {
                                    Ok(popup) => {
                                        this.popup = Some(popup);
                                        this.status = if passive {
                                            "Passive popup: parent keeps keyboard focus."
                                        } else {
                                            "Menu open: try resizing or opening a submenu."
                                        }
                                        .into();
                                    }
                                    Err(error) => {
                                        this.status =
                                            format!("Could not open popup: {error:#}").into()
                                    }
                                }
                                cx.notify();
                            }),
                        )
                        .absolute()
                        .left(anchor_rect.origin.x)
                        .top(anchor_rect.origin.y)
                        .w(anchor_rect.size.width)
                        .h(anchor_rect.size.height)
                        .bg(rgb(0x394553))
                        .rounded_md()
                        .flex()
                        .items_center()
                        .justify_center()
                    }),
            )
            // Same-application dismissal is deliberately the caller's responsibility.
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.close_popup(cx);
                    cx.notify();
                }),
            )
    }
}

struct PopupContent {
    focus: FocusHandle,
    grab: bool,
    submenu: Option<WindowHandle<PopupContent>>,
    error: Option<SharedString>,
}

impl Render for PopupContent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let parent = window.window_handle();
        div()
            .track_focus(&self.focus)
            .key_context("PopupExample")
            .on_action(|_: &Tab, window, cx| window.focus_next(cx))
            .on_action(|_: &TabPrev, window, cx| window.focus_prev(cx))
            .size_full()
            .bg(rgb(0x303842))
            .text_color(rgb(0xe6e8ec))
            .border_1()
            .border_color(rgb(0x68798c))
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(if self.grab {
                "Native menu"
            } else {
                "Passive popup"
            })
            .child(format!("Keyboard focus: {}", window.is_window_active()))
            .child(format!("Size: {:?}", window.viewport_size()))
            .child(
                button("Resize to 340 × 320")
                    .on_click(|_, window, _| window.resize(size(px(340.), px(320.)))),
            )
            .child(button("Close popup").on_click(|_, window, _| window.remove_window()))
            .children(self.error.clone())
            .when(self.grab, |this| {
                this.child(
                    popup_button(
                        "Open submenu →",
                        cx.processor(move |this, (), _, cx| {
                            if this
                                .submenu
                                .is_some_and(|submenu| cx.windows().contains(&submenu.into()))
                            {
                                return;
                            }
                            let anchor =
                                Bounds::new(point(px(16.), px(220.)), size(px(220.), px(32.)));
                            match open_popup(parent, anchor, true, cx) {
                                Ok(submenu) => this.submenu = Some(submenu),
                                Err(error) => this.error = Some(format!("{error:#}").into()),
                            }
                            cx.notify();
                        }),
                    )
                    .absolute()
                    .left(px(16.))
                    .top(px(220.))
                    .w(px(220.))
                    .h(px(32.)),
                )
            })
    }
}

fn button(label: &'static str) -> Stateful<Div> {
    div()
        .id(label)
        .role(Role::Button)
        .aria_label(label)
        .focusable()
        .tab_stop(true)
        .border_2()
        .border_color(gpui::rgba(0))
        .focus(|style| style.border_color(rgb(0xa6c8ff)))
        .cursor_pointer()
        .child(label)
}

fn popup_button(
    label: &'static str,
    open: impl Fn((), &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let open = Rc::new(open);
    button(label)
        // Wayland needs the opening input to remain pressed when requesting a grab.
        .on_mouse_down(MouseButton::Left, {
            let open = open.clone();
            move |_, window, cx| {
                cx.stop_propagation();
                open((), window, cx);
            }
        })
        .on_key_down({
            let open = open.clone();
            move |event, window, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space")
                    && !event.keystroke.modifiers.modified()
                    && !event.is_held
                {
                    cx.stop_propagation();
                    window.prevent_default();
                    open((), window, cx);
                }
            }
        })
        .on_a11y_action(AccessibleAction::Click, move |_, window, cx| {
            open((), window, cx);
        })
}

fn open_popup(
    parent: AnyWindowHandle,
    anchor_rect: Bounds<Pixels>,
    grab: bool,
    cx: &mut App,
) -> anyhow::Result<WindowHandle<PopupContent>> {
    cx.open_window(
        WindowOptions {
            titlebar: None,
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                point(px(0.), px(0.)),
                size(px(260.), px(280.)),
            ))),
            kind: WindowKind::AnchoredPopup(PopupOptions {
                parent,
                anchor_rect,
                anchor: PopupAnchor::BottomLeft,
                gravity: PopupGravity::BottomRight,
                constraint_adjustment: PopupConstraintAdjustment::FLIP_Y
                    | PopupConstraintAdjustment::SLIDE_X
                    | PopupConstraintAdjustment::RESIZE_Y,
                offset: point(px(0.), px(4.)),
                grab,
            }),
            focus: grab,
            ..Default::default()
        },
        |window, cx| {
            cx.new(|cx| {
                let focus = cx.focus_handle();
                if grab {
                    focus.focus(window, cx);
                }
                PopupContent {
                    focus,
                    grab,
                    submenu: None,
                    error: None,
                }
            })
        },
    )
}

fn run_example() {
    application().run(|cx: &mut App| {
        if !example_support::load_fonts(cx) {
            return;
        }
        cx.bind_keys([
            KeyBinding::new("tab", Tab, Some("PopupExample")),
            KeyBinding::new("shift-tab", TabPrev, Some("PopupExample")),
        ]);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        let result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    point(px(180.), px(160.)),
                    size(px(500.), px(280.)),
                ))),
                ..Default::default()
            },
            |window, cx| {
                window.set_window_title("Native popup example");
                cx.new(|cx| {
                    let focus = cx.focus_handle();
                    focus.focus(window, cx);
                    PopupExample {
                        focus,
                        popup: None,
                        status: "Open a popup below.".into(),
                    }
                })
            },
        );
        if let Err(error) = result {
            eprintln!("Could not open example: {error:#}");
            cx.quit();
        }
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

#[cfg(all(test, feature = "test-support"))]
mod tests {
    use super::*;
    use gpui::{KeyDownEvent, KeyUpEvent, Keystroke, TestAppContext};
    use std::cell::Cell;

    struct Trigger {
        focus: FocusHandle,
        openings: Rc<Cell<usize>>,
    }

    impl Render for Trigger {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let openings = self.openings.clone();
            popup_button("Open menu", move |(), _, _| {
                openings.set(openings.get() + 1)
            })
            .track_focus(&self.focus)
        }
    }

    #[gpui::test]
    fn popup_trigger_opens_on_press_without_repeating_or_using_modifiers(cx: &mut TestAppContext) {
        let openings = Rc::new(Cell::new(0));
        let window = cx.add_window({
            let openings = openings.clone();
            move |window, cx| {
                let focus = cx.focus_handle();
                focus.focus(window, cx);
                Trigger { focus, openings }
            }
        });
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("draw trigger");

        let mut press = |key, is_held| {
            cx.update_window(window.into(), |_, window, cx| {
                window.dispatch_event(
                    gpui::PlatformInput::KeyDown(KeyDownEvent {
                        keystroke: Keystroke::parse(key).expect("valid keystroke"),
                        is_held,
                        prefer_character_input: false,
                    }),
                    cx,
                );
                let openings_on_press = openings.get();
                window.dispatch_event(
                    gpui::PlatformInput::KeyUp(KeyUpEvent {
                        keystroke: Keystroke::parse(key).expect("valid keystroke"),
                    }),
                    cx,
                );
                openings_on_press
            })
            .expect("dispatch trigger key")
        };
        press("shift-enter", false);
        press("cmd-space", false);
        press("enter", true);
        assert_eq!(openings.get(), 0);

        assert_eq!(press("enter", false), 1);
        assert_eq!(openings.get(), 1);
        assert_eq!(press("space", false), 2);
        assert_eq!(openings.get(), 2);
    }
}
