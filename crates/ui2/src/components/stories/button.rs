use gpui::{rems, Div, Render};
use story::Story;
use strum::IntoEnumIterator;

use crate::prelude::*;
use crate::{h_stack, v_stack, Button, Icon, IconPosition, Label};

pub struct ButtonStory;

impl Render for ButtonStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        let states = InteractionState::iter();

        Story::container()
            .child(Story::title_for::<Button>())
            .child(
                div()
                    .flex()
                    .gap_8()
                    .child(
                        div()
                            .child(Story::label("Ghost (Default)"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label").variant(ButtonVariant::Ghost), // .state(state),
                                    )
                            })))
                            .child(Story::label("Ghost – Left Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Ghost)
                                            .icon(Icon::Plus)
                                            .icon_position(IconPosition::Left), // .state(state),
                                    )
                            })))
                            .child(Story::label("Ghost – Right Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Ghost)
                                            .icon(Icon::Plus)
                                            .icon_position(IconPosition::Right), // .state(state),
                                    )
                            }))),
                    )
                    .child(
                        div()
                            .child(Story::label("Filled"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label").variant(ButtonVariant::Filled), // .state(state),
                                    )
                            })))
                            .child(Story::label("Filled – Left Button"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .icon(Icon::Plus)
                                            .icon_position(IconPosition::Left), // .state(state),
                                    )
                            })))
                            .child(Story::label("Filled – Right Button"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .icon(Icon::Plus)
                                            .icon_position(IconPosition::Right), // .state(state),
                                    )
                            }))),
                    )
                    .child(
                        div()
                            .child(Story::label("Fixed With"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            // .state(state)
                                            .width(Some(rems(6.).into())),
                                    )
                            })))
                            .child(Story::label("Fixed With – Left Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            // .state(state)
                                            .icon(Icon::Plus)
                                            .icon_position(IconPosition::Left)
                                            .width(Some(rems(6.).into())),
                                    )
                            })))
                            .child(Story::label("Fixed With – Right Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(Label::new(state.to_string()).color(Color::Muted))
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            // .state(state)
                                            .icon(Icon::Plus)
                                            .icon_position(IconPosition::Right)
                                            .width(Some(rems(6.).into())),
                                    )
                            }))),
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
