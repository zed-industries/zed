use backtrace::Backtrace;
use gpui::*;
use prelude::FluentBuilder as _;

struct SubWindow {
    custom_titlebar: bool,
}

fn button(text: &str, on_click: impl Fn(&mut WindowContext) + 'static) -> impl IntoElement {
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
        .on_click(move |_, cx| on_click(cx))
}

impl Render for SubWindow {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0xffffff))
            .size_full()
            .gap_2()
            .when(self.custom_titlebar, |cx| {
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
                    .child(button("Close", |cx| {
                        cx.remove_window();
                    })),
            )
    }
}

struct WindowDemo {}

impl Render for WindowDemo {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
            .child(button("Normal", move |cx| {
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |cx| {
                        cx.new_view(|_cx| SubWindow {
                            custom_titlebar: false,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Popup", move |cx| {
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(window_bounds),
                        kind: WindowKind::PopUp,
                        ..Default::default()
                    },
                    |cx| {
                        cx.new_view(|_cx| SubWindow {
                            custom_titlebar: false,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Custom Titlebar", move |cx| {
                cx.open_window(
                    WindowOptions {
                        titlebar: None,
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |cx| {
                        cx.new_view(|_cx| SubWindow {
                            custom_titlebar: true,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Invisible", move |cx| {
                cx.open_window(
                    WindowOptions {
                        show: false,
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |cx| {
                        cx.new_view(|_cx| SubWindow {
                            custom_titlebar: false,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Unmovable", move |cx| {
                cx.open_window(
                    WindowOptions {
                        is_movable: false,
                        titlebar: None,
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |cx| {
                        cx.new_view(|_cx| SubWindow {
                            custom_titlebar: false,
                        })
                    },
                )
                .unwrap();
            }))
            .child(button("Hide Application", |cx| {
                cx.hide();

                // Restore the application after 3 seconds
                cx.spawn(|mut cx| async move {
                    Timer::after(std::time::Duration::from_secs(3)).await;
                    cx.update(|cx| {
                        cx.activate(false);
                    })
                })
                .detach();
            }))
    }
}

fn main() {
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");

        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "Box<Any>".to_string());

        let location = info.location().unwrap();
        let backtrace = Backtrace::new();
        eprintln!(
            "Thread {:?} panicked with {:?} at {}:{}:{}\n{:?}",
            thread_name,
            payload,
            location.file(),
            location.line(),
            location.column(),
            backtrace,
        );

        let backtrace = Backtrace::new();
        for frame in backtrace.frames() {
            let mut dl_info = libc::Dl_info {
                dli_sname: std::ptr::null_mut(),
                dli_fname: std::ptr::null_mut(),
                dli_fbase: std::ptr::null_mut(),
                dli_saddr: std::ptr::null_mut(),
            };
            let ret = unsafe { libc::dladdr(frame.ip(), &mut dl_info) };
            eprintln!(
                "Base: {:p}, IP: {:p}, Saddr: {:p}, Ret: {}, Symbols: {}",
                // unsafe { CStr::from_ptr(dl_info.dli_fname).to_string_lossy() },
                // unsafe { CStr::from_ptr(dl_info.dli_sname).to_string_lossy() },
                dl_info.dli_fbase,
                frame.ip(),
                dl_info.dli_saddr,
                ret,
                frame
                    .symbols()
                    .iter()
                    .map(|s| format!("{:?}", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        std::process::abort();
    }));

    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| WindowDemo {}),
        )
        .unwrap();
    });
}
