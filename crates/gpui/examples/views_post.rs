use gpui::*;

struct Counter {
    count: usize,
}

impl Counter {
    fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { count: 0 })
    }
}

impl Render for Counter {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .text_xl()
            .bg(rgb(0x2d004b))
            .text_color(rgb(0xffffff))
            .child(self.count.to_string())
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), Counter::new);
        cx.activate(true);
    });
}
