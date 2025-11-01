fn main() {
    #[cfg(all(target_os = "linux", feature = "wayland"))]
    example::main();

    #[cfg(not(all(target_os = "linux", feature = "wayland")))]
    panic!("This example requires the `wayland` feature and a linux system.");
}

#[cfg(all(target_os = "linux", feature = "wayland"))]
mod example {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use gpui::{
        App, Application, Bounds, Context, FontWeight, Size, Window, WindowBackgroundAppearance,
        WindowBounds, WindowKind, WindowOptions, div, layer_shell::*, point, prelude::*, px, rems,
        rgba, white,
    };

    struct LayerShellExample;

    impl LayerShellExample {
        fn new(cx: &mut Context<Self>) -> Self {
            cx.spawn(async move |this, cx| {
                loop {
                    let _ = this.update(cx, |_, cx| cx.notify());
                    cx.background_executor()
                        .timer(Duration::from_millis(500))
                        .await;
                }
            })
            .detach();

            LayerShellExample
        }
    }

    impl Render for LayerShellExample {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let hours = (now / 3600) % 24;
            let minutes = (now / 60) % 60;
            let seconds = now % 60;

            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_size(rems(4.5))
                .font_weight(FontWeight::EXTRA_BOLD)
                .text_color(white())
                .bg(rgba(0x0000044))
                .rounded_xl()
                .child(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
        }
    }

    pub fn main() {
        Application::new().run(|cx: &mut App| {
            cx.open_window(
                WindowOptions {
                    titlebar: None,
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(0.), px(0.)),
                        size: Size::new(px(500.), px(200.)),
                    })),
                    app_id: Some("gpui-layer-shell-example".to_string()),
                    window_background: WindowBackgroundAppearance::Transparent,
                    kind: WindowKind::LayerShell(LayerShellOptions {
                        namespace: "gpui".to_string(),
                        anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::BOTTOM,
                        margin: Some((px(0.), px(0.), px(40.), px(0.))),
                        keyboard_interactivity: KeyboardInteractivity::None,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_, cx| cx.new(LayerShellExample::new),
            )
            .unwrap();
        });
    }
}
