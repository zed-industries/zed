use std::marker::PhantomData;

use gpui3::{px, rgb, Div, Hsla};
use ui::prelude::*;

use crate::story::Story;

/// A reimplementation of the MDN `z-index` example, found here:
/// [https://developer.mozilla.org/en-US/docs/Web/CSS/z-index](https://developer.mozilla.org/en-US/docs/Web/CSS/z-index).
#[derive(Element)]
pub struct ZIndexStory<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> ZIndexStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title(cx, "z-index"))
            .child(
                div()
                    .flex()
                    .child(
                        div()
                            .w(px(250.))
                            .child(Story::label(cx, "z-index: auto"))
                            .child(ZIndexExample::new(0)),
                    )
                    .child(
                        div()
                            .w(px(250.))
                            .child(Story::label(cx, "z-index: 1"))
                            .child(ZIndexExample::new(1)),
                    )
                    .child(
                        div()
                            .w(px(250.))
                            .child(Story::label(cx, "z-index: 3"))
                            .child(ZIndexExample::new(3)),
                    )
                    .child(
                        div()
                            .w(px(250.))
                            .child(Story::label(cx, "z-index: 5"))
                            .child(ZIndexExample::new(5)),
                    )
                    .child(
                        div()
                            .w(px(250.))
                            .child(Story::label(cx, "z-index: 7"))
                            .child(ZIndexExample::new(7)),
                    ),
            )
    }
}

trait Styles: StyleHelpers {
    fn blocks(self) -> Self {
        self.absolute()
            .w(px(150.))
            .h(px(50.))
            .text_color(rgb::<Hsla>(0x000000))
    }

    fn blue(self) -> Self {
        self.fill(rgb::<Hsla>(0xe5e8fc))
            .border_5()
            .border_color(rgb::<Hsla>(0x112382))
            // HACK: Simulate `line-height: 55px`.
            .pt(px(16.))
            // HACK: Simulate `text-align: center`.
            .pl(px(24.))
    }

    fn red(self) -> Self {
        self.fill(rgb::<Hsla>(0xfce5e7))
            .border_5()
            .border_color(rgb::<Hsla>(0xe3a1a7))
            // HACK: Simulate `text-align: center`.
            .pl(px(8.))
    }
}

impl<S> Styles for Div<S> {}

#[derive(Element)]
struct ZIndexExample<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    z_index: u32,
}

impl<S: 'static + Send + Sync> ZIndexExample<S> {
    pub fn new(z_index: u32) -> Self {
        Self {
            state_type: PhantomData,
            z_index,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
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
                    .fill(rgb::<Hsla>(0xfcfbe5))
                    .text_color(rgb::<Hsla>(0x000000))
                    .border_5()
                    .border_color(rgb::<Hsla>(0xe3e0a1))
                    // HACK: Simulate `line-height: 215px`.
                    .pt(px(100.))
                    // HACK: Simulate `text-align: center`.
                    .pl(px(24.))
                    .z_index(self.z_index)
                    .child(format!(
                        "z-index: {}",
                        if self.z_index == 0 {
                            "auto".to_string()
                        } else {
                            self.z_index.to_string()
                        }
                    )),
            )
            // Blue blocks.
            .child(
                div()
                    .blue()
                    .blocks()
                    .top(px(0.))
                    .left(px(0.))
                    .z_index(6)
                    .child("z-index: 6"),
            )
            .child(
                div()
                    .blue()
                    .blocks()
                    .top(px(30.))
                    .left(px(30.))
                    .z_index(4)
                    .child("z-index: 4"),
            )
            .child(
                div()
                    .blue()
                    .blocks()
                    .top(px(60.))
                    .left(px(60.))
                    .z_index(2)
                    .child("z-index: 2"),
            )
            // Red blocks.
            .child(
                div()
                    .red()
                    .blocks()
                    .top(px(150.))
                    .left(px(0.))
                    .child("z-index: auto"),
            )
            .child(
                div()
                    .red()
                    .blocks()
                    .top(px(180.))
                    .left(px(30.))
                    .child("z-index: auto"),
            )
            .child(
                div()
                    .red()
                    .blocks()
                    .top(px(210.))
                    .left(px(60.))
                    .child("z-index: auto"),
            )
    }
}
