use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div, px, rgb, size,
};

struct HelloWeb {
    text: SharedString,
}

impl Render for HelloWeb {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .justify_center()
            .items_center()
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .text_color(rgb(0xcdd6f4))
                    .child(format!("Hello, {}!", &self.text)),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(swatch(0xf38ba8))
                    .child(swatch(0xa6e3a1))
                    .child(swatch(0x89b4fa))
                    .child(swatch(0xf9e2af))
                    .child(swatch(0xcba6f7))
                    .child(swatch(0x94e2d5)),
            )
    }
}

fn swatch(color: u32) -> impl IntoElement {
    div()
        .size_8()
        .bg(rgb(color))
        .rounded_md()
        .border_1()
        .border_color(rgb(0x585b70))
}

fn main() {
    gpui_platform::web_init();
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(400.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWeb {
                    text: "GPUI Web".into(),
                })
            },
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
