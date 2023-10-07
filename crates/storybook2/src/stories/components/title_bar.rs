use std::marker::PhantomData;

use ui::prelude::*;
use ui::TitleBar;

use crate::story::Story;

#[derive(Element)]
pub struct TitleBarStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> TitleBarStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, TitleBar<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(TitleBar::new(cx))
    }
}
