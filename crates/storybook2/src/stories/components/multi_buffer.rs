use std::marker::PhantomData;

use ui::prelude::*;
use ui::{hello_world_rust_buffer_example, MultiBuffer};

use crate::story::Story;

#[derive(Element)]
pub struct MultiBufferStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> MultiBufferStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        Story::container(cx)
            .child(Story::title_for::<_, MultiBuffer<S>>(cx))
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
