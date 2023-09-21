use gpui2::elements::{div, svg};
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::prelude::{ButtonVariant, InteractionState};
use crate::theme::theme;

#[derive(Element)]
pub struct IconButton {
    path: &'static str,
    variant: ButtonVariant,
    state: InteractionState,
}

pub fn icon_button(path: &'static str) -> IconButton {
    IconButton {
        path,
        variant: ButtonVariant::default(),
        state: InteractionState::default(),
    }
}

impl IconButton {
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

        let icon_color;

        if self.state == InteractionState::Disabled {
            icon_color = theme.highest.base.disabled.foreground;
        } else {
            icon_color = theme.highest.base.default.foreground;
        }

        let mut div = div();
        if self.variant == ButtonVariant::Filled {
            div = div.fill(theme.highest.on.default.background);
        }

        div.w_7()
            .h_6()
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .hover()
            .fill(theme.highest.base.hovered.background)
            .active()
            .fill(theme.highest.base.pressed.background)
            .child(svg().path(self.path).w_4().h_4().fill(icon_color))
    }
}
