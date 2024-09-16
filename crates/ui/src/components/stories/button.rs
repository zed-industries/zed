use gpui::Render;
use story::Story;

use crate::{prelude::*, IconName};
use crate::{Button, ButtonStyle};

pub struct ButtonStory;

impl Render for ButtonStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<Button>(cx))
            .child(Story::label(cx, "Default"))
            .child(Button::new("default_filled", "Click me"))
            .child(Story::label(cx, "Selected"))
            .child(Button::new("selected_filled", "Click me").selected(true))
            .child(Story::label(cx, "Selected with `selected_label`"))
            .child(
                Button::new("selected_label_filled", "Click me")
                    .selected(true)
                    .selected_label("I have been selected"),
            )
            .child(Story::label(cx, "With `label_color`"))
            .child(Button::new("filled_with_label_color", "Click me").color(Color::Created))
            .child(Story::label(cx, "With `icon`"))
            .child(Button::new("filled_with_icon", "Click me").icon(IconName::FileGit))
            .child(Story::label(cx, "Selected with `icon`"))
            .child(
                Button::new("filled_and_selected_with_icon", "Click me")
                    .selected(true)
                    .icon(IconName::FileGit),
            )
            .child(Story::label(cx, "Default (Subtle)"))
            .child(Button::new("default_subtle", "Click me").style(ButtonStyle::Subtle))
            .child(Story::label(cx, "Default (Transparent)"))
            .child(Button::new("default_transparent", "Click me").style(ButtonStyle::Transparent))
    }
}
