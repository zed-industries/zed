use std::marker::PhantomData;

use crate::ui::prelude::*;
use crate::ui::{Label, Panel};

use crate::story::Story;

#[derive(Element)]
pub struct PanelStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> PanelStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, Panel<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Panel::new(
                ScrollState::default(),
                |_, _| {
                    vec![div()
                        .overflow_y_scroll(ScrollState::default())
                        .children((0..100).map(|ix| Label::new(format!("Item {}", ix + 1))))
                        .into_any()]
                },
                Box::new(()),
            ))
    }
}
