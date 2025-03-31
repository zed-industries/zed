use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use gpui::{
    App, Application, AssetSource, Bounds, Context, SharedString, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size, svg,
};

struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
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
            .map_err(|err| err.into())
    }
}

struct SvgExample;

impl Render for SvgExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .justify_center()
            .items_center()
            .gap_8()
            .bg(rgb(0xffffff))
            .child(
                svg()
                    .path("svg/dragon.svg")
                    .size_8()
                    .text_color(rgb(0xff0000)),
            )
            .child(
                svg()
                    .path("svg/dragon.svg")
                    .size_8()
                    .text_color(rgb(0x00ff00)),
            )
            .child(
                svg()
                    .path("svg/dragon.svg")
                    .size_8()
                    .text_color(rgb(0x0000ff)),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples"),
        })
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| SvgExample),
            )
            .unwrap();
            cx.activate(true);
        });
}
