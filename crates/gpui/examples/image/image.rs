use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use gpui::*;

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
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        div().child(
            div()
                .flex_row()
                .size_full()
                .gap_4()
                .child(self.text)
                .child(img(self.src).w(px(512.0)).h(px(512.0))),
        )
    }
}

struct ImageShowcase {
    local_resource: Arc<PathBuf>,
    remote_resource: SharedUri,
}

impl Render for ImageShowcase {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .justify_center()
            .items_center()
            .gap_8()
            .bg(rgb(0xFFFFFF))
            .child(ImageContainer::new(
                "Image loaded from a local file",
                self.local_resource.clone(),
            ))
            .child(ImageContainer::new(
                "Image loaded from a remote resource",
                self.remote_resource.clone(),
            ))
    }
}

actions!(image, [Quit]);

fn main() {
    env_logger::init();

    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Image",
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

        cx.open_window(window_options, |cx| {
            cx.new_view(|_cx| ImageShowcase {
                // Relative path to your root project path
                local_resource: Arc::new(PathBuf::from_str("examples/image/app-icon.png").unwrap()),
                remote_resource: "https://picsum.photos/512/512".into(),
            })
        })
        .unwrap();
    });
}
