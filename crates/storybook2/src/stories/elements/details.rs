use std::marker::PhantomData;

use ui::prelude::*;
use ui::Details;

use crate::story::Story;

#[derive(Element)]
pub struct DetailsStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> DetailsStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, Details<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Details::new("The quick brown fox jumps over the lazy dog"))
            .child(Story::label(cx, "With meta"))
            .child(
                Details::new("The quick brown fox jumps over the lazy dog")
                    .meta_text("Sphinx of black quartz, judge my vow."),
            )
    }
}
