//! Demonstrates injecting the out-of-tree Parley text system and reaching its
//! Parley-native layout features from inside a view by downcasting the shared
//! `TextSystem` trait object.

use gpui::{
    App, Bounds, Context, Render, SharedString, Window, WindowBounds, WindowOptions, application,
    div, prelude::*, px, rgb, size,
};
use gpui_parley::ParleyTextSystem;

struct Text {
    label: SharedString,
    wrap_width: f32,
    indent: f32,
}

impl Render for Text {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The shared `TextSystem` SPI only exposes `layout_line`/`layout_wrapped_line`.
        // Downcast to the concrete type to use Parley's `text-indent` support.
        let line_count = cx
            .text_system()
            .as_any()
            .downcast_ref::<ParleyTextSystem>()
            .map(|parley| {
                parley
                    .layout_indented(&self.label, 16.0, self.indent, self.wrap_width)
                    .lines()
                    .len()
            })
            .unwrap_or(0);

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .rounded_md()
            .bg(rgb(0x202020))
            .text_color(rgb(0xffffff))
            .child(self.label.clone())
            .child(format!(
                "Parley wrapped this into {line_count} lines with a {:.0}px indent",
                self.indent
            ))
    }
}

fn main() {
    application()
        .with_text_system(ParleyTextSystem::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(520.0), px(240.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| Text {
                        label: "The quick brown fox jumps over the lazy dog while the bright \
                                sun shines down on the quiet meadow near the river."
                            .into(),
                        wrap_width: 360.0,
                        indent: 40.0,
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
