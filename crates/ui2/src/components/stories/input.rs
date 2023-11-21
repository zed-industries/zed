use gpui::{Div, Render};

use crate::prelude::*;
use crate::{Input, Story};

pub struct InputStory;

impl Render for InputStory {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container(cx)
            .child(Story::title_for::<Input>(cx))
            .child(Story::label(cx, "Default"))
            .child(div().flex().child(Input::new("Search")))
    }
}
