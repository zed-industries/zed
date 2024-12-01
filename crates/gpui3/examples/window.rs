use gpui::*;
use gpui3 as gpui;
use prelude::FluentBuilder as _;

struct SubWindow {
    custom_titlebar: bool,
}

fn button(
    text: &str,
    on_click: impl Fn(&mut Window, &mut AppContext) + 'static,
) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_2()
        .bg(rgb(0xf7f7f7))
        .active(|this| this.opacity(0.85))
        .border_1()
        .border_color(rgb(0xe0e0e0))
        .rounded_md()
        .cursor_pointer()
        .child(text.to_string())
        .on_click(move |_, window, cx| on_click(window, cx))
}

impl SubWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut ModelContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .size_full()
            .gap_2()
            .when(self.custom_titlebar.clone(), |cx| {
                cx.child(
                    div()
                        .flex()
                        .h(px(32.))
                        .px_4()
                        .bg(gpui::blue())
                        .text_color(gpui::white())
                        .w_full()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .size_full()
                                .child("Custom Titlebar"),
                        ),
                )
            })
            .child(
                div()
                    .p_8()
                    .gap_2()
                    .child("SubWindow")
                    .child(button("Close", |window, _cx| {
                        window.remove_window();
                    })),
            )
    }
}

fn render_window_demo(
    _: &mut (),
    _window: &mut Window,
    cx: &mut ModelContext<()>,
) -> impl IntoElement {
    let window_bounds =
        WindowBounds::Windowed(Bounds::centered(None, size(px(300.0), px(300.0)), cx));

    div()
        .p_4()
        .flex()
        .flex_wrap()
        .bg(rgb(0xffffff))
        .size_full()
        .justify_center()
        .items_center()
        .gap_2()
        .child(button("Normal", move |_window, cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(window_bounds),
                    ..Default::default()
                },
                |_, _| {
                    (
                        SubWindow {
                            custom_titlebar: false,
                        },
                        SubWindow::render,
                    )
                },
            )
            .unwrap();
        }))
        .child(button("Popup", move |_window, cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(window_bounds),
                    kind: WindowKind::PopUp,
                    ..Default::default()
                },
                |_, _| {
                    (
                        SubWindow {
                            custom_titlebar: false,
                        },
                        SubWindow::render,
                    )
                },
            )
            .unwrap();
        }))
        .child(button("Custom Titlebar", move |_window, cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: None,
                    window_bounds: Some(window_bounds),
                    ..Default::default()
                },
                |_, _| {
                    (
                        SubWindow {
                            custom_titlebar: true,
                        },
                        SubWindow::render,
                    )
                },
            )
            .unwrap();
        }))
        .child(button("Invisible", move |_window, cx| {
            cx.open_window(
                WindowOptions {
                    show: false,
                    window_bounds: Some(window_bounds),
                    ..Default::default()
                },
                |_, _| {
                    (
                        SubWindow {
                            custom_titlebar: false,
                        },
                        SubWindow::render,
                    )
                },
            )
            .unwrap();
        }))
        .child(button("Unmovable", move |_window, cx| {
            cx.open_window(
                WindowOptions {
                    is_movable: false,
                    titlebar: None,
                    window_bounds: Some(window_bounds),
                    ..Default::default()
                },
                |_, _| {
                    (
                        SubWindow {
                            custom_titlebar: false,
                        },
                        SubWindow::render,
                    )
                },
            )
            .unwrap();
        }))
        .child(button("Hide Application", |_window, cx| {
            cx.hide();

            // Restore the application after 3 seconds
            cx.spawn(|cx| async move {
                Timer::after(std::time::Duration::from_secs(3)).await;
                cx.update(|cx| {
                    cx.activate(false);
                })
            })
            .detach();
        }))
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, _| ((), render_window_demo),
        )
        .unwrap();
    });
}
