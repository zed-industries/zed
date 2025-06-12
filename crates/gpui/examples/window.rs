use gpui::{
    App, Application, Bounds, Context, KeyBinding, PromptButton, PromptLevel, SharedString, Timer,
    Window, WindowBounds, WindowControlArea, WindowKind, WindowOptions, actions, div, prelude::*,
    px, rgb, size,
};

struct SubWindow {
    custom_titlebar: bool,
}

fn button(text: &str, on_click: impl Fn(&mut Window, &mut App) + 'static) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_2()
        .bg(rgb(0xf7f7f7))
        .active(|this| this.opacity(0.85))
        .border_1()
        .border_color(rgb(0xe0e0e0))
        .rounded_sm()
        .cursor_pointer()
        .child(text.to_string())
        .on_click(move |_, window, cx| on_click(window, cx))
}

impl Render for SubWindow {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .size_full()
            .gap_2()
            .when(self.custom_titlebar, |cx| {
                cx.child(
                    div()
                        .id("titlebar")
                        .flex()
                        .h(px(32.))
                        .text_color(gpui::white())
                        .w_full()
                        .justify_between()
                        .child(
                            div()
                                .px_3()
                                .bg(gpui::black())
                                .window_control_area(WindowControlArea::Drag)
                                .flex()
                                .items_center()
                                .flex_1()
                                .child("Custom Titlebar"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_shrink_0()
                                .child(
                                    div()
                                        .id("minimize")
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .size(px(32.))
                                        .window_control_area(WindowControlArea::Min)
                                        .bg(gpui::black())
                                        .hover(|this| this.bg(gpui::black().opacity(0.95)))
                                        .text_color(gpui::white())
                                        .child("_"),
                                )
                                .child(
                                    div()
                                        .id("maximize")
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .size(px(32.))
                                        .window_control_area(WindowControlArea::Max)
                                        .bg(gpui::black())
                                        .hover(|this| this.bg(gpui::black().opacity(0.95)))
                                        .text_color(gpui::white())
                                        .child("+"),
                                )
                                .child(
                                    div()
                                        .id("close")
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .size(px(32.))
                                        .window_control_area(WindowControlArea::Close)
                                        .bg(gpui::black())
                                        .hover(|this| this.bg(gpui::red().opacity(0.95)))
                                        .text_color(gpui::white())
                                        .child("x"),
                                ),
                        ),
                )
            })
            .child(div().p_8().gap_2().child("SubWindow with custom titlebar"))
    }
}

struct WindowDemo {}

impl Render for WindowDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(300.0), px(300.0)), cx));

        div()
            .p_4()
            .flex()
            .flex_wrap()
            .bg(rgb(0xffffff))
            .size_full()
            .justify_center()
            .content_center()
            .gap_2()
            .child(button("Normal", move |_, cx| {
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |_, cx| {
                        cx.new(|_| SubWindow {
                            custom_titlebar: false,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Popup", move |_, cx| {
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(window_bounds),
                        kind: WindowKind::PopUp,
                        ..Default::default()
                    },
                    |_, cx| {
                        cx.new(|_| SubWindow {
                            custom_titlebar: false,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Custom Titlebar", move |_, cx| {
                cx.open_window(
                    WindowOptions {
                        titlebar: None,
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |_, cx| {
                        cx.new(|_| SubWindow {
                            custom_titlebar: true,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Invisible", move |_, cx| {
                cx.open_window(
                    WindowOptions {
                        show: false,
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |_, cx| {
                        cx.new(|_| SubWindow {
                            custom_titlebar: false,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Unmovable", move |_, cx| {
                cx.open_window(
                    WindowOptions {
                        is_movable: false,
                        titlebar: None,
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |_, cx| {
                        cx.new(|_| SubWindow {
                            custom_titlebar: false,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Hide Application", |window, cx| {
                cx.hide();

                // Restore the application after 3 seconds
                window
                    .spawn(cx, async move |cx| {
                        Timer::after(std::time::Duration::from_secs(3)).await;
                        cx.update(|_, cx| {
                            cx.activate(false);
                        })
                    })
                    .detach();
            }))
            .child(button("Resize", |window, _| {
                let content_size = window.bounds().size;
                window.resize(size(content_size.height, content_size.width));
            }))
            .child(button("Prompt", |window, cx| {
                let answer = window.prompt(
                    PromptLevel::Info,
                    "Are you sure?",
                    None,
                    &["Ok", "Cancel"],
                    cx,
                );

                cx.spawn(async move |_| {
                    if answer.await.unwrap() == 0 {
                        println!("You have clicked Ok");
                    } else {
                        println!("You have clicked Cancel");
                    }
                })
                .detach();
            }))
            .child(button("Prompt (non-English)", |window, cx| {
                let answer = window.prompt(
                    PromptLevel::Info,
                    "Are you sure?",
                    None,
                    &[PromptButton::ok("确定"), PromptButton::cancel("取消")],
                    cx,
                );

                cx.spawn(async move |_| {
                    if answer.await.unwrap() == 0 {
                        println!("You have clicked Ok");
                    } else {
                        println!("You have clicked Cancel");
                    }
                })
                .detach();
            }))
    }
}

actions!(window, [Quit]);

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    cx.observe_window_bounds(window, move |_, window, _| {
                        println!("Window bounds changed: {:?}", window.bounds());
                    })
                    .detach();

                    WindowDemo {}
                })
            },
        )
        .unwrap();

        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    });
}
