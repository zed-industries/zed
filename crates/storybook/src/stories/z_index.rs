use gpui::{px, rgb, Div, IntoElement, Render, RenderOnce};
use story::Story;
use ui::prelude::*;

/// A reimplementation of the MDN `z-index` example, found here:
/// [https://developer.mozilla.org/en-US/docs/Web/CSS/z-index](https://developer.mozilla.org/en-US/docs/Web/CSS/z-index).
pub struct ZIndexStory;

impl Render for ZIndexStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container().child(Story::title("z-index")).child(
            div()
                .flex()
                .child(
                    div()
                        .w(px(250.))
                        .child(Story::label("z-index: auto"))
                        .child(ZIndexExample::new(0)),
                )
                .child(
                    div()
                        .w(px(250.))
                        .child(Story::label("z-index: 1"))
                        .child(ZIndexExample::new(1)),
                )
                .child(
                    div()
                        .w(px(250.))
                        .child(Story::label("z-index: 3"))
                        .child(ZIndexExample::new(3)),
                )
                .child(
                    div()
                        .w(px(250.))
                        .child(Story::label("z-index: 5"))
                        .child(ZIndexExample::new(5)),
                )
                .child(
                    div()
                        .w(px(250.))
                        .child(Story::label("z-index: 7"))
                        .child(ZIndexExample::new(7)),
                ),
        )
    }
}

trait Styles: Styled + Sized {
    // Trailing `_` is so we don't collide with `block` style `StyleHelpers`.
    fn block_(self) -> Self {
        self.absolute()
            .w(px(150.))
            .h(px(50.))
            .text_color(rgb(0x000000))
    }

    fn blue(self) -> Self {
        self.bg(rgb(0xe5e8fc))
            .border_5()
            .border_color(rgb(0x112382))
            .line_height(px(55.))
            // HACK: Simulate `text-align: center`.
            .pl(px(24.))
    }

    fn red(self) -> Self {
        self.bg(rgb(0xfce5e7))
            .border_5()
            .border_color(rgb(0xe3a1a7))
            // HACK: Simulate `text-align: center`.
            .pl(px(8.))
    }
}

impl Styles for Div {}

#[derive(IntoElement)]
struct ZIndexExample {
    z_index: u16,
}

impl RenderOnce for ZIndexExample {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            // Example element.
            .child(
                div()
                    .absolute()
                    .top(px(15.))
                    .left(px(15.))
                    .w(px(180.))
                    .h(px(230.))
                    .bg(rgb(0xfcfbe5))
                    .text_color(rgb(0x000000))
                    .border_5()
                    .border_color(rgb(0xe3e0a1))
                    .line_height(px(215.))
                    // HACK: Simulate `text-align: center`.
                    .pl(px(24.))
                    .z_index(self.z_index)
                    .child(SharedString::from(format!(
                        "z-index: {}",
                        if self.z_index == 0 {
                            "auto".to_string()
                        } else {
                            self.z_index.to_string()
                        }
                    ))),
            )
            // Blue blocks.
            .child(
                div()
                    .blue()
                    .block_()
                    .top(px(0.))
                    .left(px(0.))
                    .z_index(6)
                    .child("z-index: 6"),
            )
            .child(
                div()
                    .blue()
                    .block_()
                    .top(px(30.))
                    .left(px(30.))
                    .z_index(4)
                    .child("z-index: 4"),
            )
            .child(
                div()
                    .blue()
                    .block_()
                    .top(px(60.))
                    .left(px(60.))
                    .z_index(2)
                    .child("z-index: 2"),
            )
            // Red blocks.
            .child(
                div()
                    .red()
                    .block_()
                    .top(px(150.))
                    .left(px(0.))
                    .child("z-index: auto"),
            )
            .child(
                div()
                    .red()
                    .block_()
                    .top(px(180.))
                    .left(px(30.))
                    .child("z-index: auto"),
            )
            .child(
                div()
                    .red()
                    .block_()
                    .top(px(210.))
                    .left(px(60.))
                    .child("z-index: auto"),
            )
    }
}

impl ZIndexExample {
    pub fn new(z_index: u16) -> Self {
        Self { z_index }
    }
}
