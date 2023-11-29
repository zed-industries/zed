use gpui::{Div, Render};
use story::Story;

use crate::{prelude::*, Tooltip};
use crate::{Icon, OldIconButton};

pub struct IconButtonStory;

impl Render for IconButtonStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<OldIconButton>())
            .child(Story::label("Default"))
            .child(div().w_8().child(OldIconButton::new("icon_a", Icon::Hash)))
            .child(Story::label("With `on_click`"))
            .child(
                div()
                    .w_8()
                    .child(
                        OldIconButton::new("with_on_click", Icon::Ai).on_click(|_event, _cx| {
                            println!("Clicked!");
                        }),
                    ),
            )
            .child(Story::label("With `tooltip`"))
            .child(
                div().w_8().child(
                    OldIconButton::new("with_tooltip", Icon::MessageBubbles)
                        .tooltip(|cx| Tooltip::text("Open messages", cx)),
                ),
            )
    }
}
