use gpui2::geometry::rems;
use ui::empty_buffer_example;
use ui::hello_world_rust_buffer_example;
use ui::hello_world_rust_buffer_with_status_example;
use ui::prelude::*;
use ui::Buffer;

use crate::story::Story;

#[derive(Element, Default)]
pub struct BufferStory {}

impl BufferStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, Buffer<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(div().w(rems(64.)).h_96().child(empty_buffer_example()))
            .child(Story::label(cx, "Hello World (Rust)"))
            .child(
                div()
                    .w(rems(64.))
                    .h_96()
                    .child(hello_world_rust_buffer_example(cx)),
            )
            .child(Story::label(cx, "Hello World (Rust) with Status"))
            .child(
                div()
                    .w(rems(64.))
                    .h_96()
                    .child(hello_world_rust_buffer_with_status_example(cx)),
            )
    }
}
