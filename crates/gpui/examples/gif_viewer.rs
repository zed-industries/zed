use gpui::{div, img, prelude::*, App, AppContext, Window, WindowOptions};

struct GifViewerExample {
    gif_path: std::path::PathBuf,
}

impl Render for GifViewerExample {
    fn render(
        &mut self,
        _model: &Model<Self>,
        _window: &mut Window,
        _cx: &mut AppContext,
    ) -> impl IntoElement {
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
            |_, _, _| GifViewerExample { gif_path },
        )
        .unwrap();
        cx.activate(true);
    });
}
