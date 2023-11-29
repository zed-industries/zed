use std::rc::Rc;

use gpui::ClickEvent;

use crate::prelude::*;
use crate::{Color, Icon, OldIconButton, IconSize};

#[derive(IntoElement)]
pub struct Disclosure {
    is_open: bool,
    on_toggle: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Disclosure {
    pub fn new(is_open: bool) -> Self {
        Self {
            is_open,
            on_toggle: None,
        }
    }

    pub fn on_toggle(
        mut self,
        handler: impl Into<Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>>,
    ) -> Self {
        self.on_toggle = handler.into();
        self
    }
}

impl RenderOnce for Disclosure {
    type Rendered = OldIconButton;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        OldIconButton::new(
            "toggle",
            if self.is_open {
                Icon::ChevronDown
            } else {
                Icon::ChevronRight
            },
        )
        .color(Color::Muted)
        .size(IconSize::Small)
        .when_some(self.on_toggle, move |this, on_toggle| {
            this.on_click(move |event, cx| on_toggle(event, cx))
        })
    }
}
