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
            .child(Story::title_for::<_, Button<V>>(cx))
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
            .child(Story::label(cx, "Button with `on_click`"))
            .child(
                Button::new("Label")
                    .variant(ButtonVariant::Ghost)
                    // NOTE: There currently appears to be a bug in GPUI2 where only the last event handler will fire.
                    // So adding additional buttons with `on_click`s after this one will cause this `on_click` to not fire.
                    .on_click(|_view, _cx| println!("Button clicked.")),
            )
    }
}
