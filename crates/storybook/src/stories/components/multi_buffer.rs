use ui::prelude::*;
use ui::{hello_world_rust_buffer_example, MultiBuffer};

use crate::story::Story;

#[derive(Element, Default)]
pub struct MultiBufferStory {}

impl MultiBufferStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        Story::container(cx)
            .child(Story::title_for::<_, MultiBuffer<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(MultiBuffer::new(vec![
                hello_world_rust_buffer_example(&theme),
                hello_world_rust_buffer_example(&theme),
                hello_world_rust_buffer_example(&theme),
                hello_world_rust_buffer_example(&theme),
                hello_world_rust_buffer_example(&theme),
            ]))
    }
}
