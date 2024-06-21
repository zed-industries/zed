use gpui::*;

struct WindowContent {
    text: SharedString,
}

impl Render for WindowContent {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x1e2025))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(self.text.clone())
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        // Create several new windows, positioned in the top right corner of each screen

        for screen in cx.displays() {
            let options = {
                let margin_right = px(16.);
                let margin_height = px(-48.);

                let size = Size {
                    width: px(400.),
                    height: px(72.),
                };

                let bounds = gpui::Bounds::<Pixels> {
                    origin: screen.bounds().upper_right()
                        - point(size.width + margin_right, margin_height),
                    size,
                };

                WindowOptions {
                    // Set the bounds of the window in screen coordinates
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    // Specify the display_id to ensure the window is created on the correct screen
                    display_id: Some(screen.id()),

                    titlebar: None,
                    window_background: WindowBackgroundAppearance::default(),
                    focus: false,
                    show: true,
                    kind: WindowKind::PopUp,
                    is_movable: false,
                    app_id: None,
                }
            };

            cx.open_window(options, |cx| {
                cx.new_view(|_| WindowContent {
                    text: format!("{:?}", screen.id()).into(),
                })
            })
            .unwrap();
        }
    });
}
