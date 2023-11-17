use crate::{h_stack, prelude::*, ClickHandler, Icon, IconElement};
use gpui::{prelude::*, Action, AnyView, MouseButton};
use std::sync::Arc;

struct IconButtonHandlers<V: 'static> {
    click: Option<ClickHandler<V>>,
}

impl<V: 'static> Default for IconButtonHandlers<V> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Component)]
pub struct IconButton<V: 'static> {
    id: ElementId,
    icon: Icon,
    color: TextColor,
    variant: ButtonVariant,
    state: InteractionState,
    selected: bool,
    tooltip: Option<Box<dyn Fn(&mut V, &mut ViewContext<V>) -> AnyView + 'static>>,
    handlers: IconButtonHandlers<V>,
}

impl<V: 'static> IconButton<V> {
    pub fn new(id: impl Into<ElementId>, icon: Icon) -> Self {
        Self {
            id: id.into(),
            icon,
            color: TextColor::default(),
            variant: ButtonVariant::default(),
            state: InteractionState::default(),
            selected: false,
            tooltip: None,
            handlers: IconButtonHandlers::default(),
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    pub fn color(mut self, color: TextColor) -> Self {
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

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn tooltip(
        mut self,
        tooltip: impl Fn(&mut V, &mut ViewContext<V>) -> AnyView + 'static,
    ) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn on_click(mut self, handler: impl 'static + Fn(&mut V, &mut ViewContext<V>)) -> Self {
        self.handlers.click = Some(Arc::new(handler));
        self
    }

    pub fn action(self, action: Box<dyn Action>) -> Self {
        self.on_click(move |this, cx| cx.dispatch_action(action.boxed_clone()))
    }

    fn render(mut self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let icon_color = match (self.state, self.color) {
            (InteractionState::Disabled, _) => TextColor::Disabled,
            (InteractionState::Active, _) => TextColor::Selected,
            _ => self.color,
        };

        let (mut bg_color, bg_hover_color, bg_active_color) = match self.variant {
            ButtonVariant::Filled => (
                cx.theme().colors().element_background,
                cx.theme().colors().element_hover,
                cx.theme().colors().element_active,
            ),
            ButtonVariant::Ghost => (
                cx.theme().colors().ghost_element_background,
                cx.theme().colors().ghost_element_hover,
                cx.theme().colors().ghost_element_active,
            ),
        };

        if self.selected {
            bg_color = bg_hover_color;
        }

        let mut button = h_stack()
            .id(self.id.clone())
            .justify_center()
            .rounded_md()
            .p_1()
            .bg(bg_color)
            .cursor_pointer()
            .hover(|style| style.bg(bg_hover_color))
            .active(|style| style.bg(bg_active_color))
            .child(IconElement::new(self.icon).color(icon_color));

        if let Some(click_handler) = self.handlers.click.clone() {
            button = button.on_mouse_down(MouseButton::Left, move |state, event, cx| {
                cx.stop_propagation();
                click_handler(state, cx);
            })
        }

        if let Some(tooltip) = self.tooltip.take() {
            if !self.selected {
                button = button.tooltip(move |view: &mut V, cx| (tooltip)(view, cx))
            }
        }

        button
    }
}
