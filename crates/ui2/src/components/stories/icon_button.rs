use gpui::{Div, Render};
use story::Story;

use crate::{prelude::*, Tooltip};
use crate::{Icon, IconButton};

pub struct IconButtonStory;

impl Render for IconButtonStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<IconButton>())
            .child(Story::label("Default"))
            .child(div().w_8().child(IconButton::new("icon_a", Icon::Hash)))
            .child(Story::label("Selected"))
            .child(
                div()
                    .w_8()
                    .child(IconButton::new("icon_a", Icon::Hash).selected(true)),
            )
            .child(Story::label("Selected with `selected_icon`"))
            .child(
                div().w_8().child(
                    IconButton::new("icon_a", Icon::AudioOn)
                        .selected(true)
                        .selected_icon(Icon::AudioOff),
                ),
            )
            .child(Story::label("Disabled"))
            .child(
                div()
                    .w_8()
                    .child(IconButton::new("icon_a", Icon::Hash).disabled(true)),
            )
            .child(Story::label("With `on_click`"))
            .child(
                div()
                    .w_8()
                    .child(IconButton::new("with_on_click", Icon::Ai).on_click(
                        |_event: &_, _cx: &mut WindowContext| {
                            println!("Clicked!");
                        },
                    )),
            )
            .child(Story::label("With `tooltip`"))
            .child(
                div().w_8().child(
                    IconButton::new("with_tooltip", Icon::MessageBubbles)
                        .tooltip(|cx| Tooltip::text("Open messages", cx)),
                ),
            )
    }
}
