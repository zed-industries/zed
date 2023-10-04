use std::marker::PhantomData;

use strum::IntoEnumIterator;

use crate::story::Story;
use crate::story_selector::{ComponentStory, ElementStory};
use crate::ui::prelude::*;

#[derive(Element)]
pub struct KitchenSinkStory<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> KitchenSinkStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let element_stories = ElementStory::iter().map(|selector| selector.story());
        let component_stories = ComponentStory::iter().map(|selector| selector.story());

        Story::container(cx)
            .overflow_y_scroll(ScrollState::default())
            .child(Story::title(cx, "Kitchen Sink"))
            .child(Story::label(cx, "Elements"))
            .child(div().flex().flex_col().children_any(element_stories))
            .child(Story::label(cx, "Components"))
            .child(div().flex().flex_col().children_any(component_stories))
            // Add a bit of space at the bottom of the kitchen sink so elements
            // don't end up squished right up against the bottom of the screen.
            .child(div().p_4())
    }
}
