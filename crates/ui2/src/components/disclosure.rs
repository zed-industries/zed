use gpui::{ClickEvent, Listener};

use crate::prelude::*;
use crate::{Color, Icon, IconButton, IconSize};

#[derive(IntoElement)]
pub struct Disclosure {
    is_open: bool,
    on_toggle: Option<Listener<ClickEvent>>,
}

impl Disclosure {
    pub fn new(is_open: bool) -> Self {
        Self {
            is_open,
            on_toggle: None,
        }
    }

    pub fn on_toggle(mut self, handler: Listener<ClickEvent>) -> Self {
        self.on_toggle = Some(handler);
        self
    }
}

impl RenderOnce for Disclosure {
    type Rendered = IconButton;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        IconButton::new(
            "toggle",
            match self.is_open {
                true => Icon::ChevronDown,
                false => Icon::ChevronRight,
            },
        )
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small)
        .when_some(self.on_toggle, move |this, on_toggle| {
            this.on_click(on_toggle)
        })
    }
}
