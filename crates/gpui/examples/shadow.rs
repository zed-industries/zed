use gpui::*;

fn main() {
    struct ShadowExample;

    impl Render for ShadowExample {
        fn render(
            &mut self,
            _model: &Model<Self>,
            _window: &mut Window,
            _cx: &mut AppContext,
        ) -> impl IntoElement {
            div()
                .flex()
                .bg(rgb(0xffffff))
                .size_full()
                .justify_center()
                .items_center()
                .child(div().size_8().shadow_sm())
        }
    }

    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, _, _| ShadowExample,
        )
        .unwrap();
    });
}
