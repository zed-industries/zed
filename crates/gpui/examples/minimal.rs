use gpui::{
    App, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct Minimal;

// cx: &App
// cx: &mut App
// cx: &mut Context<T>    (often Self)



// make invalid states unrepresentable
// 
// OLD:
// text_link: None,
// image_link: None,
// 
// text_link: Some,
// image_link: Some,
enum MarkdownLink {
    Text(SharedString),
    Image(SharedString),
}

impl Render for Minimal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let data = cx.new(|_| vec![String::from("hello"), String::from("world")]);

        let first = data.read(cx).first().cloned();

        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .when_some(first, |this, first| {
                this.bg(rgb(0xFF0000))
            })
            .child("Hello, World!")
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(400.), px(300.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Minimal),
        )
        .unwrap();
        cx.activate(true);
    });
}
