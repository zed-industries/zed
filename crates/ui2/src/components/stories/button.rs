use gpui::{Div, Render};
use story::Story;

use crate::prelude::*;
use crate::{h_stack, Button, Icon, IconPosition};

pub struct ButtonStory;

impl Render for ButtonStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<Button>())
            .child(
                div()
                    .flex()
                    .gap_8()
                    .child(
                        div().child(Story::label("Ghost (Default)")).child(
                            h_stack()
                                .gap_2()
                                .child(Button::new("Label").variant(ButtonVariant::Ghost)),
                        ),
                    )
                    .child(Story::label("Ghost – Left Icon"))
                    .child(
                        h_stack().gap_2().child(
                            Button::new("Label")
                                .variant(ButtonVariant::Ghost)
                                .icon(Icon::Plus)
                                .icon_position(IconPosition::Left),
                        ),
                    ),
            )
            .child(Story::label("Ghost – Right Icon"))
            .child(
                h_stack().gap_2().child(
                    Button::new("Label")
                        .variant(ButtonVariant::Ghost)
                        .icon(Icon::Plus)
                        .icon_position(IconPosition::Right),
                ),
            )
            .child(
                div().child(Story::label("Filled")).child(
                    h_stack()
                        .gap_2()
                        .child(Button::new("Label").variant(ButtonVariant::Filled)),
                ),
            )
            .child(Story::label("Filled – Left Button"))
            .child(
                h_stack().gap_2().child(
                    Button::new("Label")
                        .variant(ButtonVariant::Filled)
                        .icon(Icon::Plus)
                        .icon_position(IconPosition::Left),
                ),
            )
            .child(Story::label("Filled – Right Button"))
            .child(
                h_stack().gap_2().child(
                    Button::new("Label")
                        .variant(ButtonVariant::Filled)
                        .icon(Icon::Plus)
                        .icon_position(IconPosition::Right),
                ),
            )
            .child(Story::label("Button with `on_click`"))
            .child(
                Button::new("Label")
                    .variant(ButtonVariant::Ghost)
                    .on_click(|_, _cx| println!("Button clicked.")),
            )
    }
}
