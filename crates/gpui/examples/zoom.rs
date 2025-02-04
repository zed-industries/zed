use std::{f32::consts::PI, time::Duration};

use gpui::{
    div, prelude::*, px, rgb, size, white, Animation, AnimationExt, App, Application, Bounds,
    Window,
};

struct MainView {}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .p_5()
            .bg(rgb(0x202020))
            .justify_between()
            .items_center()
            .font_family("Sans")
            .with_animation(
                "main-view",
                Animation::new(Duration::from_millis(5000)).repeat(),
                |el, delta| {
                    el.child(ChildElement {
                        zoom: (2.0 * delta * PI).sin() * 0.25 + 0.75,
                    })
                },
            )
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
            .size(px(120.0))
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
