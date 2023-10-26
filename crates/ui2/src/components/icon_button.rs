use std::sync::Arc;

use gpui2::MouseButton;

use crate::{h_stack, prelude::*};
use crate::{ClickHandler, Icon, IconColor, IconElement};

struct IconButtonHandlers<S: 'static> {
    click: Option<ClickHandler<S>>,
}

impl<S: 'static> Default for IconButtonHandlers<S> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Component)]
pub struct IconButton<S: 'static> {
    id: ElementId,
    icon: Icon,
    color: IconColor,
    variant: ButtonVariant,
    state: InteractionState,
    handlers: IconButtonHandlers<S>,
}

impl<S: 'static> IconButton<S> {
    pub fn new(id: impl Into<ElementId>, icon: Icon) -> Self {
        Self {
            id: id.into(),
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

    pub fn on_click(mut self, handler: impl 'static + Fn(&mut S, &mut ViewContext<S>) + Send + Sync) -> Self {
        self.handlers.click = Some(Arc::new(handler));
        self
    }

    fn render(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
        let theme = theme(cx);

        let icon_color = match (self.state, self.color) {
            (InteractionState::Disabled, _) => IconColor::Disabled,
            _ => self.color,
        };

        let (bg_color, bg_hover_color, bg_active_color) = match self.variant {
            ButtonVariant::Filled => (
                theme.filled_element,
                theme.filled_element_hover,
                theme.filled_element_active,
            ),
            ButtonVariant::Ghost => (
                theme.ghost_element,
                theme.ghost_element_hover,
                theme.ghost_element_active,
            ),
        };

        let mut button = h_stack()
            .id(self.id.clone())
            .justify_center()
            .rounded_md()
            .py(ui_size(cx, 0.25))
            .px(ui_size(cx, 6. / 14.))
            .bg(bg_color)
            .hover(|style| style.bg(bg_hover_color))
            .active(|style| style.bg(bg_active_color))
            .child(IconElement::new(self.icon).color(icon_color));

        if let Some(click_handler) = self.handlers.click.clone() {
            button = button.on_mouse_down(MouseButton::Left, move |state, event, cx| {
                click_handler(state, cx);
            });
        }

        button
    }
}
