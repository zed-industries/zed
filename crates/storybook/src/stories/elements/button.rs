use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use strum::IntoEnumIterator;
use ui::{button, theme, IconAsset, IconPosition};
use ui::{h_stack, label, prelude::*, v_stack};

use crate::story::Story;

#[derive(Element, Default)]
pub struct ButtonStory {}

impl ButtonStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let states = InteractionState::iter();

        Story::container()
            .child(Story::title_for::<_, ui::Button>())
            .child(Story::label("Ghost (Default)"))
            .child(h_stack().gap_2().children(states.clone().map(|state| {
                let state_name = state.as_str().clone();

                v_stack()
                    .gap_1()
                    .child(
                        label(&state_name)
                            .color(ui::LabelColor::Muted)
                            .size(ui::LabelSize::Small),
                    )
                    .child(button("Label").variant(ButtonVariant::Ghost).state(state))
            })))
            .child(Story::label("Ghost With Left Icon"))
            .child(h_stack().gap_2().children(states.clone().map(|state| {
                let state_name = state.as_str().clone();

                v_stack()
                    .gap_1()
                    .child(
                        label(&state_name)
                            .color(ui::LabelColor::Muted)
                            .size(ui::LabelSize::Small),
                    )
                    .child(
                        button("Label")
                            .variant(ButtonVariant::Ghost)
                            .icon(IconAsset::Plus)
                            .icon_position(IconPosition::Left)
                            .state(state),
                    )
            })))
            .child(Story::label("Ghost With Right Icon"))
            .child(h_stack().gap_2().children(states.clone().map(|state| {
                let state_name = state.as_str().clone();

                v_stack()
                    .gap_1()
                    .child(
                        label(&state_name)
                            .color(ui::LabelColor::Muted)
                            .size(ui::LabelSize::Small),
                    )
                    .child(
                        button("Label")
                            .variant(ButtonVariant::Ghost)
                            .icon(IconAsset::Plus)
                            .icon_position(IconPosition::Right)
                            .state(state),
                    )
            })))
            .child(Story::label("Filled"))
            .child(h_stack().gap_2().children(states.clone().map(|state| {
                let state_name = state.as_str().clone();

                v_stack()
                    .gap_1()
                    .child(
                        label(&state_name)
                            .color(ui::LabelColor::Muted)
                            .size(ui::LabelSize::Small),
                    )
                    .child(button("Label").variant(ButtonVariant::Filled).state(state))
            })))
            .child(Story::label("Filled – Left Button"))
            .child(h_stack().gap_2().children(states.clone().map(|state| {
                let state_name = state.as_str().clone();

                v_stack()
                    .gap_1()
                    .child(
                        label(&state_name)
                            .color(ui::LabelColor::Muted)
                            .size(ui::LabelSize::Small),
                    )
                    .child(
                        button("Label")
                            .variant(ButtonVariant::Filled)
                            .icon(IconAsset::Plus)
                            .icon_position(IconPosition::Left)
                            .state(state),
                    )
            })))
            .child(Story::label("Filled – Right Button"))
            .child(h_stack().gap_2().children(states.clone().map(|state| {
                let state_name = state.as_str().clone();

                v_stack()
                    .gap_1()
                    .child(
                        label(&state_name)
                            .color(ui::LabelColor::Muted)
                            .size(ui::LabelSize::Small),
                    )
                    .child(
                        button("Label")
                            .variant(ButtonVariant::Filled)
                            .icon(IconAsset::Plus)
                            .icon_position(IconPosition::Right)
                            .state(state),
                    )
            })))
    }
}
