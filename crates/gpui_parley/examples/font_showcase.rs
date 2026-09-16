//! A font showcase rendering the classic pangram in the embedded IBM Plex Sans
//! faces, shaped through the out-of-tree Parley text system.

use gpui::{
    App, Bounds, BoundsExt, Context, Render, Window, WindowBounds, WindowOptions, application, div,
    prelude::*, px, rgb, size,
};
use gpui_parley::ParleyTextSystem;

fn specimen(label: &'static str, weight: gpui::FontWeight, italic: bool) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_color(rgb(0x9a9a9a))
                .text_size(px(12.0))
                .child(label),
        )
        .child(
            div()
                .font_family("IBM Plex Sans")
                .font_weight(weight)
                .when(italic, |this| this.italic())
                .text_size(px(26.0))
                .text_color(rgb(0xffffff))
                .child("The quick brown fox jumps over the lazy dog"),
        )
}

struct FontShowcase;

impl Render for FontShowcase {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_5()
            .p_6()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .child(
                div()
                    .text_color(rgb(0xffffff))
                    .text_size(px(16.0))
                    .font_weight(gpui::FontWeight(600.0))
                    .child("Parley Font Showcase"),
            )
            .child(specimen(
                "IBM Plex Sans Regular",
                gpui::FontWeight(400.0),
                false,
            ))
            .child(specimen(
                "IBM Plex Sans Italic",
                gpui::FontWeight(400.0),
                true,
            ))
            .child(specimen(
                "IBM Plex Sans SemiBold",
                gpui::FontWeight(600.0),
                false,
            ))
            .child(specimen(
                "IBM Plex Sans SemiBold Italic",
                gpui::FontWeight(600.0),
                true,
            ))
    }
}

fn main() {
    application()
        .with_text_system(ParleyTextSystem::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(720.0), px(400.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| FontShowcase),
            )
            .unwrap();
            cx.activate(true);
        });
}
