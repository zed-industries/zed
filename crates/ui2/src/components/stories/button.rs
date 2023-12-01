use gpui::{Div, Render};
use story::Story;

use crate::prelude::*;
use crate::{Button, ButtonStyle};

pub struct ButtonStory;

impl Render for ButtonStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<Button>())
            .child(Story::label("Default"))
            .child(Button::new("default_filled", "Click me"))
            .child(Story::label("Selected"))
            .child(Button::new("selected_filled", "Click me").selected(true))
            .child(Story::label("Selected with `selected_label`"))
            .child(
                Button::new("selected_label_filled", "Click me")
                    .selected(true)
                    .selected_label("I have been selected"),
            )
            .child(Story::label("With `label_color`"))
            .child(Button::new("filled_with_label_color", "Click me").color(Color::Created))
            .child(Story::label("Default (Subtle)"))
            .child(Button::new("default_subtle", "Click me").style(ButtonStyle::Subtle))
            .child(Story::label("Default (Transparent)"))
            .child(Button::new("default_transparent", "Click me").style(ButtonStyle::Transparent))
    }
}
