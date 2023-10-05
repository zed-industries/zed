use ui::prelude::*;
use ui::{Buffer, MultiBuffer};

use crate::story::Story;

#[derive(Element, Default)]
pub struct MultiBufferStory {}

impl MultiBufferStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, MultiBuffer<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(MultiBuffer::new(vec![
                Buffer::new(),
                Buffer::new(),
                Buffer::new(),
                Buffer::new(),
            ]))
    }
}
