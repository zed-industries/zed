use std::marker::PhantomData;
use std::sync::Arc;

use gpui3::{Interactive, MouseButton};

use crate::prelude::*;
use crate::{theme, ClickHandler, Icon, IconColor, IconElement};

struct IconButtonHandlers<S: 'static + Send + Sync> {
    click: Option<ClickHandler<S>>,
}

impl<S: 'static + Send + Sync> Default for IconButtonHandlers<S> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Element)]
pub struct IconButton<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    icon: Icon,
    color: IconColor,
    variant: ButtonVariant,
    state: InteractionState,
    handlers: IconButtonHandlers<S>,
}

impl<S: 'static + Send + Sync> IconButton<S> {
    pub fn new(icon: Icon) -> Self {
        Self {
            state_type: PhantomData,
            icon,
            color: IconColor::default(),
            variant: ButtonVariant::default(),
            state: InteractionState::default(),
            handlers: IconButtonHandlers::default(),
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    pub fn color(mut self, color: IconColor) -> Self {
        self.color = color;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&mut S, &mut ViewContext<S>) + 'static + Send + Sync,
    ) -> Self {
        self.handlers.click = Some(Arc::new(handler));
        self
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        let icon_color = match (self.state, self.color) {
            (InteractionState::Disabled, _) => IconColor::Disabled,
            _ => self.color,
        };

        let mut div = div();
        if self.variant == ButtonVariant::Filled {
            div = div.fill(theme.highest.on.default.background);
        }

        if let Some(click_handler) = self.handlers.click.clone() {
            div = div.on_click(MouseButton::Left, move |state, event, cx| {
                click_handler(state, cx);
            });
        }

        div.w_7()
            .h_6()
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .hover()
            .fill(theme.highest.base.hovered.background)
            // .active()
            // .fill(theme.highest.base.pressed.background)
            .child(IconElement::new(self.icon).color(icon_color))
    }
}
