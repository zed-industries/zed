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
                let popup_margin_width = DevicePixels::from(16);
                let popup_margin_height = DevicePixels::from(-0) - DevicePixels::from(48);

                let window_size = Size {
                    width: px(400.),
                    height: px(72.),
                };

                let screen_bounds = screen.bounds();
                let size: Size<DevicePixels> = window_size.into();

                let bounds = gpui::Bounds::<DevicePixels> {
                    origin: screen_bounds.upper_right()
                        - point(size.width + popup_margin_width, popup_margin_height),
                    size: window_size.into(),
                };

                WindowOptions {
                    // Set the bounds of the window in screen coordinates
                    bounds: Some(bounds),
                    // Specify the display_id to ensure the window is created on the correct screen
                    display_id: Some(screen.id()),

                    titlebar: None,
                    window_background: WindowBackgroundAppearance::default(),
                    focus: false,
                    show: true,
                    kind: WindowKind::PopUp,
                    is_movable: false,
                    fullscreen: false,
                }
            };

            cx.open_window(options, |cx| {
                cx.new_view(|_| WindowContent {
                    text: format!("{:?}", screen.id()).into(),
                })
            });
        }
    });
}
