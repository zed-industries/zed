use crate::{h_stack, prelude::*, Icon, IconElement};
use gpui::{prelude::*, Action, AnyView, Div, MouseButton, MouseDownEvent, Stateful};

#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: Icon,
    color: Color,
    variant: ButtonVariant,
    state: InteractionState,
    selected: bool,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>>,
    on_mouse_down: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
}

impl RenderOnce for IconButton {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let icon_color = match (self.state, self.color) {
            (InteractionState::Disabled, _) => Color::Disabled,
            (InteractionState::Active, _) => Color::Selected,
            _ => self.color,
        };

        let (mut bg_color, bg_active_color) = match self.variant {
            ButtonVariant::Filled => (
                cx.theme().colors().element_background,
                cx.theme().colors().element_active,
            ),
            ButtonVariant::Ghost => (
                cx.theme().colors().ghost_element_background,
                cx.theme().colors().ghost_element_active,
            ),
        };

        if self.selected {
            bg_color = cx.theme().colors().element_selected;
        }

        let mut button = h_stack()
            .id(self.id.clone())
            .justify_center()
            .rounded_md()
            .p_1()
            .bg(bg_color)
            .cursor_pointer()
            // Nate: Trying to figure out the right places we want to show a
            // hover state here. I think it is a bit heavy to have it on every
            // place we use an icon button.
            // .hover(|style| style.bg(bg_hover_color))
            .active(|style| style.bg(bg_active_color))
            .child(IconElement::new(self.icon).color(icon_color));

        if let Some(click_handler) = self.on_mouse_down {
            button = button.on_mouse_down(MouseButton::Left, move |event, cx| {
                cx.stop_propagation();
                click_handler(event, cx);
            })
        }

        if let Some(tooltip) = self.tooltip {
            if !self.selected {
                button = button.tooltip(move |cx| tooltip(cx))
            }
        }

        button
    }
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: Icon) -> Self {
        Self {
            id: id.into(),
            icon,
            color: Color::default(),
            variant: ButtonVariant::default(),
            state: InteractionState::default(),
            selected: false,
            tooltip: None,
            on_mouse_down: None,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
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

    pub fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn on_click(
        mut self,
        handler: impl 'static + Fn(&MouseDownEvent, &mut WindowContext),
    ) -> Self {
        self.on_mouse_down = Some(Box::new(handler));
        self
    }

    pub fn action(self, action: Box<dyn Action>) -> Self {
        self.on_click(move |_event, cx| cx.dispatch_action(action.boxed_clone()))
    }
}
