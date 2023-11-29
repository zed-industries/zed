use crate::{h_stack, prelude::*, Icon, IconElement, IconSize};
use gpui::{prelude::*, Action, AnyView, ClickEvent, Div, Stateful};

#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: Icon,
    color: Color,
    size: IconSize,
    variant: ButtonVariant,
    disabled: bool,
    selected: bool,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl RenderOnce for IconButton {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let icon_color = match (self.disabled, self.selected, self.color) {
            (true, _, _) => Color::Disabled,
            (false, true, _) => Color::Selected,
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
            .child(
                IconElement::new(self.icon)
                    .size(self.size)
                    .color(icon_color),
            );

        if let Some(click_handler) = self.on_click {
            button = button.on_click(move |event, cx| {
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
            size: Default::default(),
            variant: ButtonVariant::default(),
            selected: false,
            disabled: false,
            tooltip: None,
            on_click: None,
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

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn on_click(mut self, handler: impl 'static + Fn(&ClickEvent, &mut WindowContext)) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn action(self, action: Box<dyn Action>) -> Self {
        self.on_click(move |_event, cx| cx.dispatch_action(action.boxed_clone()))
    }
}
