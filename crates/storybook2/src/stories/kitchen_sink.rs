use gpui2::{AppContext, Context, View};
use strum::IntoEnumIterator;
use ui::prelude::*;

use crate::story::Story;
use crate::story_selector::{ComponentStory, ElementStory};

pub struct KitchenSinkStory {}

impl KitchenSinkStory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(cx: &mut AppContext) -> View<Self> {
        {
            let state = cx.build_model(|cx| Self::new());
            let render = Self::render;
            View::for_handle(state, render)
        }
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Component<Self> {
        let element_stories = ElementStory::iter()
            .map(|selector| selector.story(cx))
            .collect::<Vec<_>>();
        let component_stories = ComponentStory::iter()
            .map(|selector| selector.story(cx))
            .collect::<Vec<_>>();

        Story::container(cx)
            .id("kitchen-sink")
            .overflow_y_scroll()
            .child(Story::title(cx, "Kitchen Sink"))
            .child(Story::label(cx, "Elements"))
            .child(div().flex().flex_col().children(element_stories))
            .child(Story::label(cx, "Components"))
            .child(div().flex().flex_col().children(component_stories))
            // Add a bit of space at the bottom of the kitchen sink so elements
            // don't end up squished right up against the bottom of the screen.
            .child(div().p_4())
    }
}
