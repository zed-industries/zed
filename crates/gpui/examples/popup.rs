//! Native parent-anchored popups (macOS and Wayland).
//! Run with `cargo run -p gpui --example popup`.
//!
//! Move the parent near a screen edge to test flipping, resize an open popup, open
//! nested menus, press Escape, click another application, and close the parent.
//! Passive popups should not take focus or dismiss when switching applications.

#![cfg_attr(target_family = "wasm", no_main)]

#[path = "example_support/fonts.rs"]
mod example_support;

use gpui::{
    AnyWindowHandle, App, Bounds, Context, FocusHandle, MouseButton, Pixels, SharedString, Window,
    WindowBounds, WindowHandle, WindowKind, WindowOptions, div, point, popup::*, prelude::*, px,
    rgb, size,
};
use gpui_platform::application;

struct PopupExample {
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
                        div()
                            .id(label)
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
                            .cursor_pointer()
                            .child(label)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    cx.stop_propagation();
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
                div()
                    .id("resize")
                    .cursor_pointer()
                    .child("Resize to 340 × 320")
                    .on_click(|_, window, _| window.resize(size(px(340.), px(320.)))),
            )
            .child(
                div()
                    .id("close")
                    .cursor_pointer()
                    .child("Close popup")
                    .on_click(|_, window, _| window.remove_window()),
            )
            .children(self.error.clone())
            .when(self.grab, |this| {
                this.child(
                    div()
                        .id("submenu")
                        .absolute()
                        .left(px(16.))
                        .top(px(220.))
                        .w(px(220.))
                        .h(px(32.))
                        .cursor_pointer()
                        .child("Open submenu →")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
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
                        ),
                )
            })
    }
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
                cx.new(|_| PopupExample {
                    popup: None,
                    status: "Open a popup below.".into(),
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
