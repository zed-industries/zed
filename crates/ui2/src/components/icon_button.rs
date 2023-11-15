use crate::{h_stack, prelude::*, ClickHandler, Icon, IconElement, TextTooltip};
use gpui::{prelude::*, MouseButton, VisualContext};
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
    tooltip: Option<SharedString>,
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

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl 'static + Fn(&mut V, &mut ViewContext<V>) + Send + Sync,
    ) -> Self {
        self.handlers.click = Some(Arc::new(handler));
        self
    }

    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let icon_color = match (self.state, self.color) {
            (InteractionState::Disabled, _) => TextColor::Disabled,
            _ => self.color,
        };

        let (bg_color, bg_hover_color, bg_active_color) = match self.variant {
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

        let mut button = h_stack()
            .id(self.id.clone())
            .justify_center()
            .rounded_md()
            .p_1()
            .bg(bg_color)
            .hover(|style| style.bg(bg_hover_color))
            .active(|style| style.bg(bg_active_color))
            .child(IconElement::new(self.icon).color(icon_color));

        if let Some(click_handler) = self.handlers.click.clone() {
            button = button.on_mouse_down(MouseButton::Left, move |state, event, cx| {
                cx.stop_propagation();
                click_handler(state, cx);
            });
        }

        if let Some(tooltip) = self.tooltip.clone() {
            button =
                button.tooltip(move |_, cx| cx.build_view(|cx| TextTooltip::new(tooltip.clone())));
        }

        button
    }
}
