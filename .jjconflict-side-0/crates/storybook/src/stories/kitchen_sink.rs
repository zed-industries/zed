use gpui::{Entity, Render, prelude::*};
use story::Story;
use strum::IntoEnumIterator;
use ui::prelude::*;

use crate::story_selector::ComponentStory;

pub struct KitchenSinkStory;

impl KitchenSinkStory {
    pub fn model(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for KitchenSinkStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let component_stories = ComponentStory::iter()
            .map(|selector| selector.story(window, cx))
            .collect::<Vec<_>>();

        Story::container(cx)
            .id("kitchen-sink")
            .overflow_y_scroll()
            .child(Story::title("Kitchen Sink", cx))
            .child(Story::label("Components", cx))
            .child(div().flex().flex_col().children(component_stories))
            // Add a bit of space at the bottom of the kitchen sink so elements
            // don't end up squished right up against the bottom of the screen.
            .child(div().p_4())
    }
}
