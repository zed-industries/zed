use gpui2::elements::div;
use gpui2::geometry::rems;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use strum::IntoEnumIterator;
use ui::{h_stack, label, prelude::*, v_stack};
use ui::{theme, Button, IconAsset, IconPosition};

use crate::story::Story;

#[derive(Element, Default)]
pub struct ButtonStory {}

impl ButtonStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let states = InteractionState::iter();

        Story::container()
            .child(Story::title_for::<_, ui::Button>())
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
                                    .child(
                                        label(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Ghost)
                                            .state(state),
                                    )
                            })))
                            .child(Story::label("Ghost – Left Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        label(state.to_string())
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
                            .child(Story::label("Ghost – Right Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        label(state.to_string())
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
                            .child(Story::label("Filled"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        label(state.to_string())
                                            .color(ui::LabelColor::Muted)
                                            .size(ui::LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("Label")
                                            .variant(ButtonVariant::Filled)
                                            .state(state),
                                    )
                            })))
                            .child(Story::label("Filled – Left Button"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        label(state.to_string())
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
                            .child(Story::label("Filled – Right Button"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        label(state.to_string())
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
                            .child(Story::label("Fixed With"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        label(state.to_string())
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
                            .child(Story::label("Fixed With – Left Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        label(state.to_string())
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
                            .child(Story::label("Fixed With – Right Icon"))
                            .child(h_stack().gap_2().children(states.clone().map(|state| {
                                v_stack()
                                    .gap_1()
                                    .child(
                                        label(state.to_string())
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
