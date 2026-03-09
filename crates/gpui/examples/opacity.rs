#![cfg_attr(target_family = "wasm", no_main)]

use std::{fs, path::PathBuf};

use anyhow::Result;
use gpui::{
    App, AssetSource, Bounds, BoxShadow, ClickEvent, Context, SharedString, Task, Window,
    WindowBounds, WindowOptions, div, hsla, img, point, prelude::*, px, rgb, size, svg,
};
use gpui_platform::application;

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
    animating: bool,
}

impl HelloWorld {
    fn new(_window: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            _task: None,
            opacity: 0.5,
            animating: false,
        }
    }

    fn start_animation(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.opacity = 0.0;
        self.animating = true;
        cx.notify();
    }
}

impl Render for HelloWorld {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.animating {
            self.opacity += 0.005;
            if self.opacity >= 1.0 {
                self.animating = false;
                self.opacity = 1.0;
            } else {
                window.request_animation_frame();
            }
        }

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
                    .on_click(cx.listener(Self::start_animation))
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
                            .shadow(vec![BoxShadow {
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
                            .child(
                                div()
                                    .flex()
                                    .children(["🎊", "✈️", "🎉", "🎈", "🎁", "🎂"].map(|emoji| {
                                        div()
                                            .child(emoji.to_string())
                                            .hover(|style| style.opacity(0.5))
                                    })),
                            )
                            .child(img("image/black-cat-typing.gif").size_12()),
                    ),
            )
    }
}

fn run_example() {
    application()
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
