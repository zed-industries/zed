use gpui::*;
use gpui3 as gpui;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, _| {
                ((), |_, _window, _cx| {
                    div()
                        .flex()
                        .bg(rgb(0xffffff))
                        .size_full()
                        .justify_center()
                        .items_center()
                        .child(div().size_8().shadow_sm())
                })
            },
        )
        .unwrap();
    });
}
