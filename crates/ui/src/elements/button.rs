use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, Hsla, IntoElement, ParentElement, ViewContext};

use crate::{label, prelude::*, LabelColor};
use crate::{theme, LabelSize};

#[derive(Element)]
pub struct Button {
    label: &'static str,
    variant: ButtonVariant,
    state: InteractionState,
}

pub fn button(label: &'static str) -> Button {
    Button {
        label,
        variant: ButtonVariant::default(),
        state: InteractionState::default(),
    }
}

impl Button {
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let system_color = SystemColor::new();

        let mut label = label(self.label.clone()).size(LabelSize::Small);
        let background_color: Hsla;

        match (self.variant, self.state) {
            (ButtonVariant::Ghost, InteractionState::Enabled) => {
                label = label.color(LabelColor::default());
                background_color = system_color.transparent;
            }
            (ButtonVariant::Ghost, InteractionState::Hovered) => {
                label = label.color(LabelColor::default());
                background_color = theme.lowest.base.hovered.background;
            }
            (ButtonVariant::Ghost, InteractionState::Active) => {
                label = label.color(LabelColor::default());
                background_color = theme.lowest.base.pressed.background;
            }
            (ButtonVariant::Ghost, InteractionState::Disabled) => {
                label = label.color(LabelColor::Disabled);
                background_color = system_color.transparent;
            }
            (ButtonVariant::Ghost, InteractionState::Focused) => {
                label = label.color(LabelColor::default());
                background_color = theme.lowest.accent.default.background;
            }
            (ButtonVariant::Filled, InteractionState::Enabled) => {
                label = label.color(LabelColor::default());
                background_color = theme.lowest.on.default.background;
            }
            (ButtonVariant::Filled, InteractionState::Hovered) => {
                label = label.color(LabelColor::default());
                background_color = theme.lowest.on.hovered.background;
            }
            (ButtonVariant::Filled, InteractionState::Active) => {
                label = label.color(LabelColor::default());
                background_color = theme.lowest.on.pressed.background;
            }
            (ButtonVariant::Filled, InteractionState::Disabled) => {
                label = label.color(LabelColor::Disabled);
                background_color = theme.lowest.on.default.background;
            }
            (ButtonVariant::Filled, InteractionState::Focused) => {
                label = label.color(LabelColor::default());
                background_color = theme.lowest.accent.default.background;
            }
        }

        div()
            .h_6()
            .px_1()
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .fill(background_color)
            .child(label)
    }
}
