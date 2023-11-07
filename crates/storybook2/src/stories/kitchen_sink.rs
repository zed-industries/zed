use crate::{story::Story, story_selector::ComponentStory};
use gpui::{Div, Render, StatefulInteraction, View, VisualContext};
use strum::IntoEnumIterator;
use ui::prelude::*;

pub struct KitchenSinkStory;

impl KitchenSinkStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.build_view(|cx| Self)
    }
}

impl Render for KitchenSinkStory {
    type Element = Div<Self, StatefulInteraction<Self>>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let component_stories = ComponentStory::iter()
            .map(|selector| selector.story(cx))
            .collect::<Vec<_>>();

        Story::container(cx)
            .id("kitchen-sink")
            .overflow_y_scroll()
            .child(Story::title(cx, "Kitchen Sink"))
            .child(Story::label(cx, "Components"))
            .child(div().flex().flex_col().children(component_stories))
            // Add a bit of space at the bottom of the kitchen sink so elements
            // don't end up squished right up against the bottom of the screen.
            .child(div().p_4())
    }
}
