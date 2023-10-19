use std::marker::PhantomData;
use std::sync::Arc;

use gpui3::{Interactive, MouseButton};

use crate::{h_stack, prelude::*};
use crate::{ClickHandler, Icon, IconColor, IconElement};

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

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        let icon_color = match (self.state, self.color) {
            (InteractionState::Disabled, _) => IconColor::Disabled,
            _ => self.color,
        };

        let (bg_color, bg_hover_color, bg_active_color) = match self.variant {
            ButtonVariant::Filled => (
                color.filled_element,
                color.filled_element_hover,
                color.filled_element_active,
            ),
            ButtonVariant::Ghost => (
                color.ghost_element,
                color.ghost_element_hover,
                color.ghost_element_active,
            ),
        };

        let mut button = h_stack()
            .justify_center()
            .rounded_md()
            .py(ui_size(0.25))
            .px(ui_size(6. / 14.))
            .bg(bg_color)
            .hover(|style| style.bg(bg_hover_color))
            // .active(|style| style.bg(bg_active_color))
            .child(IconElement::new(self.icon).color(icon_color));

        if let Some(click_handler) = self.handlers.click.clone() {
            button = button.on_mouse_down(MouseButton::Left, move |state, event, cx| {
                click_handler(state, cx);
            });
        }

        button
    }
}
