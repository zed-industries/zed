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

        // A checkbox could be in an indeterminate state,
        // for example the indeterminate state could represent:
        //  - a group of options of which only some are selected
        //  - an enabled option that is no longer available
        //  - a previously agreed to license that has been updated
        //
        // For the sake of styles we treat the indeterminate state as selected,
        // but its icon will be different.
        let selected =
            self.checked == Selection::Selected || self.checked == Selection::Indeterminate;

        // We could use something like this to make the checkbox background when selected:
        //
        // ```rs
        // ...
        // .when(selected, |this| {
        //     this.bg(cx.theme().colors().element_selected)
        // })
        // ```
        //
        // But we use a match instead here because the checkbox might be disabled,
        // and it could be disabled _while_ it is selected, as well as while it is not selected.
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
            // Rather than adding `px_1()` to add some space around the checkbox,
            // we use a larger parent element to create a slightly larger
            // click area for the checkbox.
            .size_5()
            // Because we've enlarged the click area, we need to create a
            // `group` to pass down interactivity events to the checkbox.
            .group(group_id.clone())
            .child(
                div()
                    .flex()
                    // This prevent the flex element from growing
                    // or shrinking in response to any size changes
                    .flex_none()
                    // The combo of `justify_center()` and `items_center()`
                    // is used frequently to center elements in a flex container.
                    //
                    // We use this to center the icon in the checkbox.
                    .justify_center()
                    .items_center()
                    .m_1()
                    .size_4()
                    .rounded_sm()
                    .bg(bg_color)
                    .border()
                    .border_color(border_color)
                    // We only want the interactivity states to fire when we
                    // are in a checkbox that isn't disabled.
                    .when(!self.disabled, |this| {
                        // Here instead of `hover()` we use `group_hover()`
                        // to pass it the group id.
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
