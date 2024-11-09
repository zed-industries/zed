#![allow(missing_docs)]

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

impl ComponentPreview for Checkbox {
    fn description() -> impl Into<Option<&'static str>> {
        "A checkbox lets people choose between a pair of opposing states, like enabled and disabled, using a different appearance to indicate each state."
    }

    fn examples() -> Vec<ComponentExampleGroup<Self>> {
        vec![
            example_group_with_title(
                "Default",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_unselected", Selection::Unselected),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new("checkbox_indeterminate", Selection::Indeterminate),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_selected", Selection::Selected),
                    ),
                ],
            ),
            example_group_with_title(
                "Disabled",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_disabled_unselected", Selection::Unselected)
                            .disabled(true),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new("checkbox_disabled_indeterminate", Selection::Indeterminate)
                            .disabled(true),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_disabled_selected", Selection::Selected)
                            .disabled(true),
                    ),
                ],
            ),
        ]
    }
}

use std::sync::Arc;

/// A [`Checkbox`] that has a [`Label`].
#[derive(IntoElement)]
pub struct CheckboxWithLabel {
    id: ElementId,
    label: Label,
    checked: Selection,
    on_click: Arc<dyn Fn(&Selection, &mut WindowContext) + 'static>,
}

impl CheckboxWithLabel {
    pub fn new(
        id: impl Into<ElementId>,
        label: Label,
        checked: Selection,
        on_click: impl Fn(&Selection, &mut WindowContext) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label,
            checked,
            on_click: Arc::new(on_click),
        }
    }
}

impl RenderOnce for CheckboxWithLabel {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .gap(Spacing::Large.rems(cx))
            .child(Checkbox::new(self.id.clone(), self.checked).on_click({
                let on_click = self.on_click.clone();
                move |checked, cx| {
                    (on_click)(checked, cx);
                }
            }))
            .child(
                div()
                    .id(SharedString::from(format!("{}-label", self.id)))
                    .on_click(move |_event, cx| {
                        (self.on_click)(&self.checked.inverse(), cx);
                    })
                    .child(self.label),
            )
    }
}

impl ComponentPreview for CheckboxWithLabel {
    fn description() -> impl Into<Option<&'static str>> {
        "A checkbox with an associated label, allowing users to select an option while providing a descriptive text."
    }

    fn examples() -> Vec<ComponentExampleGroup<Self>> {
        vec![example_group(vec![
            single_example(
                "Unselected",
                CheckboxWithLabel::new(
                    "checkbox_with_label_unselected",
                    Label::new("Always save on quit"),
                    Selection::Unselected,
                    |_, _| {},
                ),
            ),
            single_example(
                "Indeterminate",
                CheckboxWithLabel::new(
                    "checkbox_with_label_indeterminate",
                    Label::new("Always save on quit"),
                    Selection::Indeterminate,
                    |_, _| {},
                ),
            ),
            single_example(
                "Selected",
                CheckboxWithLabel::new(
                    "checkbox_with_label_selected",
                    Label::new("Always save on quit"),
                    Selection::Selected,
                    |_, _| {},
                ),
            ),
        ])]
    }
}
