use std::marker::PhantomData;

use ui::prelude::*;
use ui::Label;

use crate::story::Story;

#[derive(Element)]
pub struct LabelStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> LabelStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, Label<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Label::new("Hello, world!"))
            .child(Story::label(cx, "Highlighted"))
            .child(Label::new("Hello, world!").with_highlights(vec![0, 1, 2, 7, 8, 12]))
    }
}
