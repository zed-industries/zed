use std::rc::Rc;

use gpui::{ClickEvent, Div};

use crate::prelude::*;
use crate::{Color, Icon, IconButton, IconSize, Toggle};

#[derive(IntoElement)]
pub struct Disclosure {
    toggle: Toggle,
    on_toggle: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Disclosure {
    pub fn new(toggle: Toggle) -> Self {
        Self {
            toggle,
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
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        if !self.toggle.is_toggleable() {
            return div();
        }

        div().child(
            IconButton::new(
                "toggle",
                if self.toggle.is_toggled() {
                    Icon::ChevronDown
                } else {
                    Icon::ChevronRight
                },
            )
            .color(Color::Muted)
            .size(IconSize::Small)
            .when_some(self.on_toggle, move |this, on_toggle| {
                this.on_click(move |event, cx| on_toggle(event, cx))
            }),
        )
    }
}
