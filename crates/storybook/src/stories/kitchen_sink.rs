use strum::IntoEnumIterator;
use ui::prelude::*;

use crate::story::Story;
use crate::story_selector::{ComponentStory, ElementStory};

#[derive(Element, Default)]
pub struct KitchenSinkStory {}

impl KitchenSinkStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let element_stories = ElementStory::iter().map(|selector| selector.story());
        let component_stories = ComponentStory::iter().map(|selector| selector.story());

        Story::container(cx)
            .overflow_y_scroll(ScrollState::default())
            .child(Story::title(cx, "Kitchen Sink"))
            .child(Story::label(cx, "Elements"))
            .child(div().flex().flex_col().children_any(element_stories))
            .child(Story::label(cx, "Components"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .py_4()
                    .children_any(component_stories),
            )
    }
}
