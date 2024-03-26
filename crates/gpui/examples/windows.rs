use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
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
        // Create several new windows, positioned where the notification windows should be
        //
        for screen in cx.displays() {
            let options = {
                let notification_margin_width = GlobalPixels::from(16.);
                let notification_margin_height = GlobalPixels::from(-0.) - GlobalPixels::from(48.);

                let window_size = Size {
                    width: px(400.),
                    height: px(72.),
                };

                let screen_bounds = screen.bounds();
                let size: Size<GlobalPixels> = window_size.into();

                let bounds = gpui::Bounds::<GlobalPixels> {
                    origin: screen_bounds.upper_right()
                        - point(
                            size.width + notification_margin_width,
                            notification_margin_height,
                        ),
                    size: window_size.into(),
                };

                WindowOptions {
                    bounds: Some(bounds),
                    titlebar: None,
                    focus: false,
                    show: true,
                    kind: WindowKind::PopUp,
                    is_movable: false,
                    display_id: Some(screen.id()),
                    fullscreen: false,
                }
            };

            cx.open_window(options, |cx| {
                cx.new_view(|_| HelloWorld {
                    text: "World".into(),
                })
            });
        }
    });
}
