use std::default;

use crate::{prelude::*, Color, Icon, IconButton, IconSize};
use gpui::ClickEvent;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum DisclosureControlStyle {
    #[default]
    AlwaysVisible,
    VisibleOnHover,
}

#[derive(IntoElement)]
pub struct Disclosure {
    is_open: bool,
    on_toggle: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    style: DisclosureControlStyle,
    group_name: Option<SharedString>,
}

impl Disclosure {
    pub fn new(is_open: bool) -> Self {
        Self {
            is_open,
            on_toggle: None,
            style: DisclosureControlStyle::default(),
            group_name: None,
        }
    }

    pub fn on_toggle(
        mut self,
        handler: impl Into<Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>>,
    ) -> Self {
        self.on_toggle = handler.into();
        self
    }

    pub fn style(mut self, style: DisclosureControlStyle) -> Self {
        self.style = style;
        self
    }

    pub fn group_name(mut self, group_name: impl Into<Option<SharedString>>) -> Self {
        self.group_name = group_name.into();
        self
    }
}

impl RenderOnce for Disclosure {
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        let group_name = self.group_name.unwrap_or("".into());

        div()
            .when(
                self.style == DisclosureControlStyle::VisibleOnHover,
                |this| {
                    this.absolute()
                        .invisible()
                        .group_hover(group_name, |this| this.visible())
                },
            )
            .child(
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
                    this.on_click(move |event, cx| on_toggle(event, cx))
                }),
            )
    }
}
