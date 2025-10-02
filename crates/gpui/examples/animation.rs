use std::time::Duration;

use anyhow::Result;
use gpui::{
    Animation, AnimationExt as _, App, Application, AssetSource, Bounds, Context, SharedString,
    Transformation, Window, WindowBounds, WindowOptions, black, bounce, div, ease_in_out,
    percentage, prelude::*, px, rgb, size, svg,
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
        div().flex().flex_col().size_full().justify_around().child(
            div().flex().flex_row().w_full().justify_around().child(
                div()
                    .flex()
                    .bg(rgb(0x2e7d32))
                    .size(px(300.0))
                    .justify_center()
                    .items_center()
                    .shadow_lg()
                    .text_xl()
                    .text_color(black())
                    .child("hello")
                    .child(
                        svg()
                            .size_8()
                            .path(ARROW_CIRCLE_SVG)
                            .text_color(black())
                            .with_animation(
                                "image_circle",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(bounce(ease_in_out)),
                                |svg, delta| {
                                    svg.with_transformation(Transformation::rotate(percentage(
                                        delta,
                                    )))
                                },
                            ),
                    ),
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
