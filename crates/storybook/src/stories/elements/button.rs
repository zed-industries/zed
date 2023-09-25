use gpui2::elements::div;
use gpui2::geometry::rems;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use strum::IntoEnumIterator;
use ui::{h_stack, prelude::*, v_stack, Label};
use ui::{Button, IconAsset, IconPosition};

use crate::story::Story;

#[derive(Element, Default)]
pub struct ButtonStory {}

impl ButtonStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let states = InteractionState::iter();

        Story::container(cx)
            .child(Story::title_for::<_, Button>(cx))
            .child(
                div()
                    .flex()
                    .gap_8()
                    .child(
                        div()
                            .child(Story::label(cx, "Ghost (Default)"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Ghost)
                                            .state(state),
                                    )
                            })))
                            .child(Story::label(cx, "Ghost – Left Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Ghost)
                                            .icon(IconAsset::Plus)
                                            .icon_position(IconPosition::Left)
                                            .state(state),
                                    )
                            })))
                            .child(Story::label(cx, "Ghost – Right Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Ghost)
                                            .icon(IconAsset::Plus)
                                            .icon_position(IconPosition::Right)
                                            .state(state),
                                    )
                            }))),
                    )
                    .child(
                        div()
                            .child(Story::label(cx, "Filled"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .state(state),
                                    )
                            })))
                            .child(Story::label(cx, "Filled – Left Button"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .icon(IconAsset::Plus)
                                            .icon_position(IconPosition::Left)
                                            .state(state),
                                    )
                            })))
                            .child(Story::label(cx, "Filled – Right Button"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .icon(IconAsset::Plus)
                                            .icon_position(IconPosition::Right)
                                            .state(state),
                                    )
                            }))),
                    )
                    .child(
                        div()
                            .child(Story::label(cx, "Fixed With"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .state(state)
                                            .width(Some(rems(6.).into())),
                                    )
                            })))
                            .child(Story::label(cx, "Fixed With – Left Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .state(state)
                                            .icon(IconAsset::Plus)
                                            .icon_position(IconPosition::Left)
                                            .width(Some(rems(6.).into())),
                                    )
                            })))
                            .child(Story::label(cx, "Fixed With – Right Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .state(state)
                                            .icon(IconAsset::Plus)
                                            .icon_position(IconPosition::Right)
                                            .width(Some(rems(6.).into())),
                                    )
                            }))),
                    ),
            )
    }
}
