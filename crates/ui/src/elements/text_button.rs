use crate::prelude::{ButtonVariant, InteractionState};
use crate::theme::theme;
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct TextButton {
    label: &'static str,
    variant: ButtonVariant,
    state: InteractionState,
}

pub fn text_button(label: &'static str) -> TextButton {
    TextButton {
        label,
        variant: ButtonVariant::default(),
        state: InteractionState::default(),
    }
}

impl TextButton {
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

        let text_color_default;
        let text_color_hover;
        let text_color_active;

        let background_color_default;
        let background_color_hover;
        let background_color_active;

        let div = div();

        match self.variant {
            ButtonVariant::Ghost => {
                text_color_default = theme.lowest.base.default.foreground;
                text_color_hover = theme.lowest.base.hovered.foreground;
                text_color_active = theme.lowest.base.pressed.foreground;
                background_color_default = theme.lowest.base.default.background;
                background_color_hover = theme.lowest.base.hovered.background;
                background_color_active = theme.lowest.base.pressed.background;
            }
            ButtonVariant::Filled => {
                text_color_default = theme.lowest.base.default.foreground;
                text_color_hover = theme.lowest.base.hovered.foreground;
                text_color_active = theme.lowest.base.pressed.foreground;
                background_color_default = theme.lowest.on.default.background;
                background_color_hover = theme.lowest.on.hovered.background;
                background_color_active = theme.lowest.on.pressed.background;
            }
        };
        div.h_6()
            .px_1()
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .text_xs()
            .text_color(text_color_default)
            .fill(background_color_default)
            .hover()
            .text_color(text_color_hover)
            .fill(background_color_hover)
            .active()
            .text_color(text_color_active)
            .fill(background_color_active)
            .child(self.label.clone())
    }
}
