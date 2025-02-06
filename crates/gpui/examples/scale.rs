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
            .bg(rgb(0x202020))
            .font_family("Sans")
            .items_center()
            .justify_between()
            .child(ChildElement { scale: 1.0 })
            .child(ChildElement { scale: 0.75 })
            .child(ChildElement { scale: 0.5 })
            .with_animation(
                "animation",
                Animation::new(Duration::from_millis(5000)).repeat(),
                |el, delta| {
                    el.child(ChildElement {
                        scale: (2.0 * delta * PI).sin() * 0.5 + 1.0,
                    })
                },
            )
    }
}

#[derive(IntoElement)]
struct ChildElement {
    pub scale: f32,
}

impl RenderOnce for ChildElement {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .min_h(px(120.0))
            .min_w(px(120.0))
            .max_h(px(120.0))
            .max_w(px(120.0))
            .scale(self.scale)
            .bg(white())
            .items_center()
            .justify_center()
            .child(format!("Scale: {:.2}x", self.scale))
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
