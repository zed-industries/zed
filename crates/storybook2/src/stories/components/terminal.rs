use std::marker::PhantomData;

use ui::prelude::*;
use ui::Terminal;

use crate::story::Story;

#[derive(Element)]
pub struct TerminalStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> TerminalStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, Terminal<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Terminal::new())
    }
}
