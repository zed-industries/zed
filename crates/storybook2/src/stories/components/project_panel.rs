use std::marker::PhantomData;

use ui::prelude::*;
use ui::{Panel, ProjectPanel};

use crate::story::Story;

#[derive(Element)]
pub struct ProjectPanelStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> ProjectPanelStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, ProjectPanel<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Panel::new(
                ScrollState::default(),
                |_, _| vec![ProjectPanel::new(ScrollState::default()).into_any()],
                Box::new(()),
            ))
    }
}
