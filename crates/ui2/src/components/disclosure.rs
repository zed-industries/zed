use std::rc::Rc;

use gpui::ClickEvent;

use crate::prelude::*;
use crate::{Color, Icon, IconButton, IconSize, ToggleState, Toggleable};

#[derive(IntoElement)]
pub struct Disclosure {
    state: ToggleState,
    on_toggle: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Disclosure {
    pub fn new(state: ToggleState) -> Self {
        Self {
            state,
            on_toggle: None,
        }
    }

    pub fn from_toggleable(toggleable: Toggleable) -> Option<Self> {
        match toggleable {
            Toggleable::Toggleable(state) => Some(Self::new(state)),
            Toggleable::NotToggleable => None,
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
    type Rendered = IconButton;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        IconButton::new(
            "toggle",
            match self.state {
                ToggleState::Toggled => Icon::ChevronDown,
                ToggleState::NotToggled => Icon::ChevronRight,
            },
        )
        .color(Color::Muted)
        .size(IconSize::Small)
        .when_some(self.on_toggle, move |this, on_toggle| {
            this.on_click(move |event, cx| on_toggle(event, cx))
        })
    }
}
