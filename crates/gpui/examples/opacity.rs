use std::{fs, path::PathBuf, time::Duration};

use anyhow::Result;
use gpui::{
    App, Application, AssetSource, Bounds, BoxShadow, ClickEvent, Context, SharedString, Task,
    Timer, Window, WindowBounds, WindowOptions, div, hsla, img, point, prelude::*, px, rgb, size,
    svg,
};

struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|e| e.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }
}

struct HelloWorld {
    _task: Option<Task<()>>,
    opacity: f32,
}

impl HelloWorld {
    fn new(_window: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            _task: None,
            opacity: 0.5,
        }
    }

    fn change_opacity(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.opacity = 0.0;
        cx.notify();

        self._task = Some(cx.spawn_in(window, async move |view, cx| {
            loop {
                Timer::after(Duration::from_secs_f32(0.05)).await;
                let mut stop = false;
                let _ = cx.update(|_, cx| {
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
        }));
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0xe0e0e0))
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
                            .child(img("image/app-icon.png").size_8())
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
                            )
                            .child(
                                svg()
                                    .path("image/arrow_circle.svg")
                                    .text_color(gpui::black())
                                    .text_2xl()
                                    .size_8(),
                            )
                            .child("🎊✈️🎉🎈🎁🎂")
                            .child(img("image/black-cat-typing.gif").size_12()),
                    ),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples"),
        })
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| HelloWorld::new(window, cx)),
            )
            .unwrap();
            cx.activate(true);
        });
}
