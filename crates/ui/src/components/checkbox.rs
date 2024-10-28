#![allow(missing_docs)]
mod checkbox_with_label;

pub use checkbox_with_label::*;

use gpui::{div, prelude::*, ElementId, IntoElement, Styled, WindowContext};

use crate::prelude::*;
use crate::{Color, Icon, IconName, Selection};

/// # Checkbox
///
/// Checkboxes are used for multiple choices, not for mutually exclusive choices.
/// Each checkbox works independently from other checkboxes in the list,
/// therefore checking an additional box does not affect any other selections.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    checked: Selection,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&Selection, &mut WindowContext) + 'static>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, checked: Selection) -> Self {
        Self {
            id: id.into(),
            checked,
            disabled: false,
            on_click: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&Selection, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let group_id = format!("checkbox_group_{:?}", self.id);

        let icon = match self.checked {
            Selection::Selected => Some(Icon::new(IconName::Check).size(IconSize::Small).color(
                if self.disabled {
                    Color::Disabled
                } else {
                    Color::Selected
                },
            )),
            Selection::Indeterminate => Some(
                Icon::new(IconName::Dash)
                    .size(IconSize::Small)
                    .color(if self.disabled {
                        Color::Disabled
                    } else {
                        Color::Selected
                    }),
            ),
            Selection::Unselected => None,
        };

        let selected =
            self.checked == Selection::Selected || self.checked == Selection::Indeterminate;

        let (bg_color, border_color) = match (self.disabled, selected) {
            (true, _) => (
                cx.theme().colors().ghost_element_disabled,
                cx.theme().colors().border_disabled,
            ),
            (false, true) => (
                cx.theme().colors().element_selected,
                cx.theme().colors().border,
            ),
            (false, false) => (
                cx.theme().colors().element_background,
                cx.theme().colors().border,
            ),
        };

        h_flex()
            .id(self.id)
            .justify_center()
            .items_center()
            .size(crate::styles::custom_spacing(cx, 20.))
            .group(group_id.clone())
            .child(
                div()
                    .flex()
                    .flex_none()
                    .justify_center()
                    .items_center()
                    .m(Spacing::Small.px(cx))
                    .size(crate::styles::custom_spacing(cx, 16.))
                    .rounded_sm()
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .when(!self.disabled, |this| {
                        this.group_hover(group_id.clone(), |el| {
                            el.bg(cx.theme().colors().element_hover)
                        })
                    })
                    .children(icon),
            )
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(move |_, cx| on_click(&self.checked.inverse(), cx)),
            )
    }
}
