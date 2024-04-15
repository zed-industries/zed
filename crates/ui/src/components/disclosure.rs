use std::sync::Arc;

use gpui::ClickEvent;

use crate::{prelude::*, Color, IconButton, IconName, IconSize};

#[derive(IntoElement)]
pub struct Disclosure {
    id: ElementId,
    is_open: bool,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Disclosure {
    pub fn new(id: impl Into<ElementId>, is_open: bool) -> Self {
        Self {
            id: id.into(),
            is_open,
            on_toggle: None,
        }
    }

    pub fn on_toggle(
        mut self,
        handler: impl Into<Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>>,
    ) -> Self {
        self.on_toggle = handler.into();
        self
    }
}

impl RenderOnce for Disclosure {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        IconButton::new(
            self.id,
            match self.is_open {
                true => IconName::ChevronDown,
                false => IconName::ChevronRight,
            },
        )
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small)
        .when_some(self.on_toggle, move |this, on_toggle| {
            this.on_click(move |event, cx| on_toggle(event, cx))
        })
    }
}
