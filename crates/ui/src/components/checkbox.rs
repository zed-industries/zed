#![allow(missing_docs)]

use crate::{
    prelude::*,
    register_components,
    utils::{component_preview, component_preview_group},
};
use gpui::{div, prelude::*, ElementId, IntoElement, Styled, WindowContext};
use std::sync::Arc;

register_components!(user, [Checkbox, CheckboxWithLabel]);

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
            .size(DynamicSpacing::Base20.rems(cx))
            .group(group_id.clone())
            .child(
                div()
                    .flex()
                    .flex_none()
                    .justify_center()
                    .items_center()
                    .m(DynamicSpacing::Base04.px(cx))
                    .size(DynamicSpacing::Base16.rems(cx))
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

impl ComponentElement for Checkbox {
    fn description() -> impl Into<Option<&'static str>> {
        "A checkbox allows people to toggle between two states, typically representing on/off, or a pair of opposites."
    }

    fn scope() -> &'static str {
        "input"
    }

    fn preview(_cx: &WindowContext) -> Option<gpui::AnyElement> {
        Some(
            component_preview_group()
                .child(
                    component_preview("Default")
                        .child(Checkbox::new("checkbox-1", Selection::Unselected)),
                )
                .child(
                    component_preview("Selected")
                        .child(Checkbox::new("checkbox-2", Selection::Selected)),
                )
                .child(
                    component_preview("Indeterminate")
                        .child(Checkbox::new("checkbox-3", Selection::Indeterminate)),
                )
                .child(
                    component_preview("Disabled")
                        .child(Checkbox::new("checkbox-4", Selection::Selected).disabled(true)),
                )
                .into_any_element(),
        )
    }
}

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
            .gap(DynamicSpacing::Base08.rems(cx))
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

impl ComponentElement for CheckboxWithLabel {
    fn description() -> impl Into<Option<&'static str>> {
        "A checkbox with an associated label."
    }

    fn scope() -> &'static str {
        "input"
    }

    fn preview(_cx: &WindowContext) -> Option<gpui::AnyElement> {
        Some(
            v_flex()
                .gap_3()
                .child(
                    component_preview_group()
                        .child(component_preview("Default").child(CheckboxWithLabel::new(
                            "checkbox-1",
                            Label::new("Show Completions"),
                            Selection::Unselected,
                            |_, _| {},
                        )))
                        .child(component_preview("Selected").child(CheckboxWithLabel::new(
                            "checkbox-2",
                            Label::new("Show Completions"),
                            Selection::Selected,
                            |_, _| {},
                        ))),
                )
                .child(
                    component_preview_group().child(
                        component_preview("Indeterminate").child(
                            v_flex()
                                .child(CheckboxWithLabel::new(
                                    "checkbox-3",
                                    Label::new("Show Completions"),
                                    Selection::Indeterminate,
                                    |_, _| {},
                                ))
                                .child(h_flex().child(div().w_5().h_full()).child(
                                    CheckboxWithLabel::new(
                                        "checkbox-4",
                                        Label::new("Editor"),
                                        Selection::Selected,
                                        |_, _| {},
                                    ),
                                ))
                                .child(h_flex().child(div().w_5().h_full()).child(
                                    CheckboxWithLabel::new(
                                        "checkbox-5",
                                        Label::new("Assistant"),
                                        Selection::Unselected,
                                        |_, _| {},
                                    ),
                                )),
                        ),
                    ),
                )
                .into_any_element(),
        )
    }
}
