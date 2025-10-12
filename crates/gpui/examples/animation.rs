use std::time::Duration;

use anyhow::Result;
use gpui::{
    Animation, AnimationExt as _, App, Application, AssetSource, Bounds, Context, SharedString,
    Transformation, Window, WindowBounds, WindowOptions, bounce, div, ease_in_out, percentage,
    prelude::*, px, size, svg,
};

struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().into_owned(),
                ))
            })
            .collect::<Vec<_>>())
    }
}

const ARROW_CIRCLE_SVG: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/image/arrow_circle.svg"
);

struct AnimationExample {}

impl Render for AnimationExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::white())
            .text_color(gpui::black())
            .justify_around()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .justify_around()
                    .child(
                        div()
                            .id("content")
                            .flex()
                            .flex_col()
                            .h(px(150.))
                            .overflow_y_scroll()
                            .w_full()
                            .flex_1()
                            .justify_center()
                            .items_center()
                            .text_xl()
                            .gap_4()
                            .child("Hello Animation")
                            .child(
                                svg()
                                    .size_20()
                                    .overflow_hidden()
                                    .path(ARROW_CIRCLE_SVG)
                                    .text_color(gpui::black())
                                    .with_animation(
                                        "image_circle",
                                        Animation::new(Duration::from_secs(2))
                                            .repeat()
                                            .with_easing(bounce(ease_in_out)),
                                        |svg, delta| {
                                            svg.with_transformation(Transformation::rotate(
                                                percentage(delta),
                                            ))
                                        },
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .h(px(64.))
                            .w_full()
                            .p_2()
                            .justify_center()
                            .items_center()
                            .border_t_1()
                            .border_color(gpui::black().opacity(0.1))
                            .bg(gpui::black().opacity(0.05))
                            .child("Other Panel"),
                    ),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {})
        .run(|cx: &mut App| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(300.), px(300.)),
                    cx,
                ))),
                ..Default::default()
            };
            cx.open_window(options, |_, cx| {
                cx.activate(false);
                cx.new(|_| AnimationExample {})
            })
            .unwrap();
        });
}
