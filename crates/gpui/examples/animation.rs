use std::time::Duration;

use gpui::*;

struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<std::borrow::Cow<'static, [u8]>> {
        std::fs::read(path).map(Into::into).map_err(Into::into)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().to_string(),
                ))
            })
            .collect::<Vec<_>>())
    }
}

struct AnimationExample {}

impl Render for AnimationExample {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size(Length::Definite(Pixels(300.0).into()))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border()
            .border_color(rgb(0xff0000))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child("hello")
            .child(
                svg()
                    .size_8()
                    .path("examples/image/arrow_circle.svg")
                    .text_color(rgb(0xff0000))
                    .with_animation(
                        "image_circle",
                        Animation::new(Duration::from_secs(1)).repeat(),
                        |svg, delta| {
                            svg.with_transformation(Transformation::rotate(dbg!(
                                delta * 2.0 * std::f64::consts::PI as f32
                            )))
                        },
                    ),
            )
    }
}

fn main() {
    App::new()
        .with_assets(Assets {})
        .run(|cx: &mut AppContext| {
            let options = WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    size: size(px(600.0), px(600.0)).into(),
                    origin: Default::default(),
                }),
                center: true,
                ..Default::default()
            };
            cx.open_window(options, |cx| cx.new_view(|_cx| AnimationExample {}));
        });
}
