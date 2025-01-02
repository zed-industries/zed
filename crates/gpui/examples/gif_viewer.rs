use gpui::{div, img, prelude::*, App, AppContext, ModelContext, Render, Window, WindowOptions};
use std::path::PathBuf;

struct GifViewer {
    gif_path: PathBuf,
}

impl GifViewer {
    fn new(gif_path: PathBuf) -> Self {
        Self { gif_path }
    }
}

impl Render for GifViewer {
    fn render(&mut self, _window: &mut Window, _cx: &mut ModelContext<Self>) -> impl IntoElement {
        div().size_full().child(
            img(self.gif_path.clone())
                .size_full()
                .object_fit(gpui::ObjectFit::Contain)
                .id("gif"),
        )
    }
}

fn main() {
    env_logger::init();
    App::new().run(|cx: &mut AppContext| {
        let cwd = std::env::current_dir().expect("Failed to get current working directory");
        let gif_path = cwd.join("crates/gpui/examples/image/black-cat-typing.gif");

        if !gif_path.exists() {
            eprintln!("Image file not found at {:?}", gif_path);
            eprintln!("Make sure you're running this example from the root of the gpui crate");
            cx.quit();
            return;
        }

        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |window, window, cx| window.new_view(cx, |_window, _cx| GifViewer::new(gif_path)),
        )
        .unwrap();
        cx.activate(true);
    });
}
