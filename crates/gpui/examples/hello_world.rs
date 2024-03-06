use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size(Length::Definite(Pixels(300.0).into()))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let options = WindowOptions {
            bounds: WindowBounds::Fixed(Bounds {
                size: size(px(600.0), px(600.0)).into(),
                origin: Default::default(),
            }),
            center: true,
            ..Default::default()
        };
        cx.open_window(options, |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        });
    });
}
