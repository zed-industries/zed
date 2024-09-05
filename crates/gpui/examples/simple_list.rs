use gpui::*;

struct SimpleList {}

impl Render for SimpleList {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .border_1()
            .border_color(rgb(0x888888))
            .bg(rgb(0xffffff))
            .child(
                uniform_list(cx.view().clone(), "entries", 50, |this, range, cx| {
                    let mut items = Vec::new();
                    println!("range: {:?}", range);
                    for i in range {
                        items.push(
                            div()
                                .cursor_pointer()
                                .id(i)
                                .on_mouse_down(MouseButton::Left, move |_evt, _ctx| {
                                    println!("clicked {:?}", format!("item {}", i));
                                })
                                .child(format!("item {}", i)),
                        );
                    }
                    items
                })
                .h_full(),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| SimpleList {}),
        )
        .unwrap();
    });
}
