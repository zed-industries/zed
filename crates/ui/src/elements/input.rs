use crate::prelude::*;
use crate::theme;

#[derive(Default, PartialEq)]
pub enum InputVariant {
    #[default]
    Ghost,
    Filled,
}

#[derive(Element)]
pub struct Input {
    placeholder: &'static str,
    value: String,
    state: InteractionState,
    variant: InputVariant,
}

impl Input {
    pub fn new(placeholder: &'static str) -> Self {
        Self {
            placeholder,
            value: "".to_string(),
            state: InteractionState::default(),
            variant: InputVariant::default(),
        }
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let text_el;
        let text_color;
        let background_color_default;
        let background_color_active;

        let mut border_color_default = theme.middle.base.default.border;
        let mut border_color_hover = theme.middle.base.hovered.border;
        let mut border_color_active = theme.middle.base.pressed.border;
        let border_color_focus = theme.middle.base.pressed.background;

        match self.variant {
            InputVariant::Ghost => {
                background_color_default = theme.middle.base.default.background;
                background_color_active = theme.middle.base.active.background;
            }
            InputVariant::Filled => {
                background_color_default = theme.middle.on.default.background;
                background_color_active = theme.middle.on.active.background;
            }
        };

        if self.state == InteractionState::Focused {
            border_color_default = theme.players[0].cursor;
            border_color_hover = theme.players[0].cursor;
            border_color_active = theme.players[0].cursor;
        }

        if self.state == InteractionState::Focused || self.state == InteractionState::Active {
            text_el = self.value.clone();
            text_color = theme.lowest.base.default.foreground;
        } else {
            text_el = self.placeholder.to_string().clone();
            text_color = theme.lowest.base.disabled.foreground;
        }

        div()
            .h_7()
            .w_full()
            .px_2()
            .border()
            .border_color(border_color_default)
            .fill(background_color_default)
            .hover()
            .border_color(border_color_hover)
            .active()
            .border_color(border_color_active)
            .fill(background_color_active)
            .flex()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .text_sm()
                    .text_color(text_color)
                    .child(text_el)
                    .child(div().text_color(theme.players[0].cursor).child("|")),
            )
    }
}
