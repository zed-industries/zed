use gpui::{Div, Render};
use story::Story;

use crate::prelude::*;
use crate::{Button, ButtonStyle2};

pub struct ButtonStory;

impl Render for ButtonStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<Button>())
            .child(Story::label("Default"))
            .child(Button::new("default_filled", "Click me"))
            .child(Story::label("Default (Subtle)"))
            .child(Button::new("default_subtle", "Click me").style(ButtonStyle2::Subtle))
            .child(Story::label("Default (Transparent)"))
            .child(Button::new("default_transparent", "Click me").style(ButtonStyle2::Transparent))
    }
}
