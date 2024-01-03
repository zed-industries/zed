use gpui::{prelude::*, Render, View};
use story::Story;
use strum::IntoEnumIterator;
use ui::prelude::*;

use crate::story_selector::ComponentStory;

pub struct KitchenSinkStory;

impl KitchenSinkStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for KitchenSinkStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let component_stories = ComponentStory::iter()
            .map(|selector| selector.story(cx))
            .collect::<Vec<_>>();

        Story::container()
            .id("kitchen-sink")
            .overflow_y_scroll()
            .child(Story::title("Kitchen Sink"))
            .child(Story::label("Components"))
            .child(div().flex().flex_col().children(component_stories))
            // Add a bit of space at the bottom of the kitchen sink so elements
            // don't end up squished right up against the bottom of the screen.
            .child(div().p_4())
    }
}
