#![cfg_attr(target_family = "wasm", no_main)]

use std::sync::Arc;

use gpui::{
    App, AppContext, Bounds, Context, ImageFilter, RenderImage, SharedString, TitlebarOptions,
    Window, WindowBounds, WindowOptions, div, img, prelude::*, px, size,
};
use image::Frame;
use smallvec::SmallVec;

const SOURCE_SIZE: u32 = 32;
const CELL_SIZE: u32 = 4;

fn checkerboard() -> Arc<RenderImage> {
    let mut buffer =
        image::RgbaImage::new(SOURCE_SIZE, SOURCE_SIZE);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let dark = ((x / CELL_SIZE) + (y / CELL_SIZE)) % 2 == 0;
        *pixel = if dark {
            image::Rgba([0, 0, 0, 255])
        } else {
            image::Rgba([255, 255, 255, 255])
        };
    }
    Arc::new(RenderImage::new(SmallVec::from_const([Frame::new(buffer)])))
}

struct ImageFilterDemo {
    checker: Arc<RenderImage>,
}

impl Render for ImageFilterDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let upscaled = px(SOURCE_SIZE as f32 * 8.0);
        div()
            .bg(gpui::white())
            .size_full()
            .p_8()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .items_center()
                            .child("Linear")
                            .child(img(self.checker.clone()).w(upscaled).h(upscaled)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .items_center()
                            .child("Nearest")
                            .child(
                                img(self.checker.clone())
                                    .w(upscaled)
                                    .h(upscaled)
                                    .image_filter(ImageFilter::Nearest),
                            ),
                    ),
            )
    }
}

fn run_example() {
    let checker = checkerboard();

    #[cfg(not(target_family = "wasm"))]
    let app = gpui_platform::application();
    #[cfg(target_family = "wasm")]
    let app = gpui_platform::single_threaded_web();

    app.run(move |cx: &mut App| {
        cx.activate(true);
        let window_options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Image Filter")),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(720.0), px(420.0)),
                cx,
            ))),
            ..Default::default()
        };
        cx.open_window(window_options, |_, cx| {
            let checker = checker.clone();
            cx.new(move |_| ImageFilterDemo { checker })
        })
        .unwrap();
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    env_logger::init();
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
