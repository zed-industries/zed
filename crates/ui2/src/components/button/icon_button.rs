use gpui::{Action, AnyView};

use crate::prelude::*;
use crate::{ButtonCommon, ButtonLike, ButtonSize2, ButtonStyle2, Icon, IconElement, IconSize};

#[derive(IntoElement)]
pub struct IconButton {
    base: ButtonLike,
    icon: Icon,
    icon_size: IconSize,
    icon_color: Color,
    selected: bool,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: Icon) -> Self {
        Self {
            base: ButtonLike::new(id),
            icon,
            icon_size: IconSize::default(),
            icon_color: Color::Default,
            selected: false,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = icon_size;
        self
    }

    // TODO: Rename to `icon_color`.
    pub fn color(mut self, icon_color: Color) -> Self {
        self.icon_color = icon_color;
        self
    }

    pub fn action(self, action: Box<dyn Action>) -> Self {
        self.on_click(move |_event, cx| cx.dispatch_action(action.boxed_clone()))
    }
}

impl Clickable for IconButton {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}

impl Disableable for IconButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl ButtonCommon for IconButton {
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    fn style(mut self, style: ButtonStyle2) -> Self {
        self.base = self.base.style(style);
        self
    }

    fn size(mut self, size: ButtonSize2) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }
}

impl RenderOnce for IconButton {
    type Rendered = ButtonLike;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        let icon_color = if self.base.disabled {
            Color::Disabled
        } else if self.selected {
            Color::Selected
        } else {
            self.icon_color
        };

        self.base.child(
            IconElement::new(self.icon)
                .size(self.icon_size)
                .color(icon_color),
        )
    }
}
