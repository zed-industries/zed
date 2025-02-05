use gpui::{div, prelude::*, px, rgb, size, white, App, Application, Bounds, Window};

struct MainView {}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(rgb(0x202020))
            .font_family("Sans")
            .items_center()
            .justify_between()
            .child(ChildElement { zoom: 1.0 })
            .child(ChildElement { zoom: 0.75 })
            .child(ChildElement { zoom: 0.5 })
            .child(ChildElement { zoom: 0.25 })
    }
}

#[derive(IntoElement)]
struct ChildElement {
    pub zoom: f32,
}

impl RenderOnce for ChildElement {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .min_h(px(120.0))
            .min_w(px(120.0))
            .max_h(px(120.0))
            .max_w(px(120.0))
            .scale(self.zoom)
            .bg(white())
            .items_center()
            .justify_center()
            .child(format!("Zoom: {:.2}x", self.zoom))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.open_window(
            gpui::WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MainView {}),
        )
        .unwrap();
    });
}
