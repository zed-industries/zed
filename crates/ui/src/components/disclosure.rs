#![allow(missing_docs)]
use crate::internal::prelude::*;
use gpui::{ClickEvent, CursorStyle};
use std::sync::Arc;

use crate::{Color, IconButton, IconButtonShape, IconName, IconSize};

register_components!(disclosure, [Disclosure]);

// TODO: This should be DisclosureControl, not Disclosure
#[derive(IntoElement)]
pub struct Disclosure {
    id: ElementId,
    is_open: bool,
    selected: bool,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    cursor_style: CursorStyle,
}

impl Disclosure {
    pub fn new(id: impl Into<ElementId>, is_open: bool) -> Self {
        Self {
            id: id.into(),
            is_open,
            selected: false,
            on_toggle: None,
            cursor_style: CursorStyle::PointingHand,
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

impl Selectable for Disclosure {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Clickable for Disclosure {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_toggle = Some(Arc::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.cursor_style = cursor_style;
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
        .shape(IconButtonShape::Square)
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small)
        .selected(self.selected)
        .when_some(self.on_toggle, move |this, on_toggle| {
            this.on_click(move |event, cx| on_toggle(event, cx))
        })
    }
}

impl ComponentPreview for Disclosure {
    fn description() -> impl Into<Option<&'static str>> {
        "A Disclosure component is used to show or hide content. It's typically used in expandable/collapsible sections or tree-like structures."
    }

    fn examples() -> Vec<ComponentExampleGroup<Self>> {
        vec![example_group(vec![
            single_example("Closed", Disclosure::new("closed", false)),
            single_example("Open", Disclosure::new("open", true)),
            single_example(
                "Open (Selected)",
                Disclosure::new("open", true).selected(true),
            ),
        ])]
    }
}
