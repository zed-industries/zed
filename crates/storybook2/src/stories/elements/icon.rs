use std::marker::PhantomData;

use strum::IntoEnumIterator;
use ui::prelude::*;
use ui::{Icon, IconElement};

use crate::story::Story;

#[derive(Element, Default)]
pub struct IconStory<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> IconStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let icons = Icon::iter();

        Story::container(cx)
            .child(Story::title_for::<_, IconElement<S>>(cx))
            .child(Story::label(cx, "All Icons"))
            .child(div().flex().gap_3().children(icons.map(IconElement::new)))
    }
}
