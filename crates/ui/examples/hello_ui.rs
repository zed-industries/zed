use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

use assets::Assets;
use ui::*;

struct HelloUI {
    text: SharedString,
}

impl Render for HelloUI {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div().flex().gap_2().justify_center().child(
                    Button::new("button", "button")
                        .icon(IconName::Close)
                        .on_click(|event, _window, _app| {
                            println!("{:?}", event);
                        }),
                ),
            )
    }
}

fn main() {
    //use with_assets() for icons.
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        //initialize setting and theme before open window.
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);

        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| HelloUI { text: "ui".into() }),
        )
        .unwrap();
        cx.activate(true);
    });
}
