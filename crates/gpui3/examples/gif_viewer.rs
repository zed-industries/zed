use gpui::{div, img, prelude::*, App, AppContext, WindowOptions};
use gpui3 as gpui;

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
            move |_window, _cx| {
                div().size_full().child(
                    img(gif_path.clone())
                        .size_full()
                        .object_fit(gpui::ObjectFit::Contain)
                        .id("gif"),
                )
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
