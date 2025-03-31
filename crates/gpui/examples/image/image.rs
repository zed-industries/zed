use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use gpui::{
    App, AppContext, Application, AssetSource, Bounds, Context, ImageSource, KeyBinding, Menu,
    MenuItem, Point, SharedString, SharedUri, TitlebarOptions, Window, WindowBounds, WindowOptions,
    actions, div, img, prelude::*, px, rgb, size,
};
use reqwest_client::ReqwestClient;

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

#[derive(IntoElement)]
struct ImageContainer {
    text: SharedString,
    src: ImageSource,
}

impl ImageContainer {
    pub fn new(text: impl Into<SharedString>, src: impl Into<ImageSource>) -> Self {
        Self {
            text: text.into(),
            src: src.into(),
        }
    }
}

impl RenderOnce for ImageContainer {
    fn render(self, _window: &mut Window, _: &mut App) -> impl IntoElement {
        div().child(
            div()
                .flex_row()
                .size_full()
                .gap_4()
                .child(self.text)
                .child(img(self.src).size(px(256.0))),
        )
    }
}

struct ImageShowcase {
    local_resource: Arc<std::path::Path>,
    remote_resource: SharedUri,
    asset_resource: SharedString,
}

impl Render for ImageShowcase {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("main")
            .overflow_y_scroll()
            .p_5()
            .size_full()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .gap_8()
            .bg(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .gap_8()
                    .child(ImageContainer::new(
                        "Image loaded from a local file",
                        self.local_resource.clone(),
                    ))
                    .child(ImageContainer::new(
                        "Image loaded from a remote resource",
                        self.remote_resource.clone(),
                    ))
                    .child(ImageContainer::new(
                        "Image loaded from an asset",
                        self.asset_resource.clone(),
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    .child(
                        div()
                            .flex_col()
                            .child("Auto Width")
                            .child(img("https://picsum.photos/800/400").h(px(180.))),
                    )
                    .child(
                        div()
                            .flex_col()
                            .child("Auto Height")
                            .child(img("https://picsum.photos/800/400").w(px(180.))),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .w_full()
                    .border_1()
                    .border_color(rgb(0xC0C0C0))
                    .child("image with max width 100%")
                    .child(img("https://picsum.photos/800/400").max_w_full()),
            )
    }
}

actions!(image, [Quit]);

fn main() {
    env_logger::init();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    Application::new()
        .with_assets(Assets {
            base: manifest_dir.join("examples"),
        })
        .run(move |cx: &mut App| {
            let http_client = ReqwestClient::user_agent("gpui example").unwrap();
            cx.set_http_client(Arc::new(http_client));

            cx.activate(true);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
            cx.set_menus(vec![Menu {
                name: "Image".into(),
                items: vec![MenuItem::action("Quit", Quit)],
            }]);

            let window_options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Image Example")),
                    appears_transparent: false,
                    ..Default::default()
                }),

                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    size: size(px(1100.), px(600.)),
                    origin: Point::new(px(200.), px(200.)),
                })),

                ..Default::default()
            };

            cx.open_window(window_options, |_, cx| {
                cx.new(|_| ImageShowcase {
                    // Relative path to your root project path
                    local_resource: manifest_dir.join("examples/image/app-icon.png").into(),
                    remote_resource: "https://picsum.photos/800/400".into(),
                    asset_resource: "image/color.svg".into(),
                })
            })
            .unwrap();
        });
}
