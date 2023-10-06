use std::marker::PhantomData;

use gpui3::rems;
use ui::prelude::*;
use ui::{
    empty_buffer_example, hello_world_rust_buffer_example,
    hello_world_rust_buffer_with_status_example, Buffer,
};

use crate::story::Story;

#[derive(Element)]
pub struct BufferStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> BufferStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        Story::container(cx)
            .child(Story::title_for::<_, Buffer<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(div().w(rems(64.)).h_96().child(empty_buffer_example()))
            .child(Story::label(cx, "Hello World (Rust)"))
            .child(
                div()
                    .w(rems(64.))
                    .h_96()
                    .child(hello_world_rust_buffer_example(&theme)),
            )
            .child(Story::label(cx, "Hello World (Rust) with Status"))
            .child(
                div()
                    .w(rems(64.))
                    .h_96()
                    .child(hello_world_rust_buffer_with_status_example(&theme)),
            )
    }
}
