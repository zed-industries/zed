use std::time::Duration;

use gpui::*;

struct HelloWorld {
    _task: Option<Task<()>>,
    opacity: f32,
}

impl HelloWorld {
    fn new(_: &mut ViewContext<Self>) -> Self {
        Self {
            _task: None,
            opacity: 0.5,
        }
    }

    fn change_opacity(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.opacity = 0.0;
        cx.notify();

        self._task = Some(cx.spawn(|view, mut cx| async move {
            loop {
                Timer::after(Duration::from_secs_f32(0.05)).await;
                let mut stop = false;
                let _ = cx.update(|cx| {
                    view.update(cx, |view, cx| {
                        if view.opacity >= 1.0 {
                            stop = true;
                            return;
                        }

                        view.opacity += 0.1;
                        cx.notify();
                    })
                });

                if stop {
                    break;
                }
            }

            ()
        }));
    }
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0xE0E0E0))
            .text_xl()
            .child(
                div()
                    .flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .border_1()
                    .text_color(gpui::blue())
                    .child(div().child("This is background text.")),
            )
            .child(
                div()
                    .id("panel")
                    .on_click(cx.listener(Self::change_opacity))
                    .absolute()
                    .top_8()
                    .left_8()
                    .right_8()
                    .bottom_8()
                    .opacity(self.opacity)
                    .flex()
                    .justify_center()
                    .items_center()
                    .bg(gpui::white())
                    .border_3()
                    .border_color(gpui::red())
                    .text_color(gpui::yellow())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .justify_center()
                            .items_center()
                            .size(px(300.))
                            .bg(gpui::blue())
                            .border_3()
                            .border_color(gpui::black())
                            .shadow(smallvec::smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.5),
                                blur_radius: px(1.0),
                                spread_radius: px(5.0),
                                offset: point(px(10.0), px(10.0)),
                            }])
                            .child("Opacity Panel (Click to test)")
                            .child(
                                div()
                                    .id("deep-level-text")
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .p_4()
                                    .bg(gpui::black())
                                    .text_color(gpui::white())
                                    .text_decoration_2()
                                    .text_decoration_wavy()
                                    .text_decoration_color(gpui::red())
                                    .child(format!("opacity: {:.1}", self.opacity)),
                            ),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(HelloWorld::new),
        )
        .unwrap();
    });
}
