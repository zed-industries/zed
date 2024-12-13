#![allow(missing_docs)]

use gpui::{div, prelude::*, ElementId, IntoElement, Styled, WindowContext};
use std::sync::Arc;

use crate::prelude::*;
use crate::utils::is_light;
use crate::{Color, Icon, IconName, ToggleState};

/// Creates a new checkbox
pub fn checkbox(id: impl Into<ElementId>, toggle_state: ToggleState) -> Checkbox {
    Checkbox::new(id, toggle_state)
}

/// Creates a new switch
pub fn switch(id: impl Into<ElementId>, toggle_state: ToggleState) -> Switch {
    Switch::new(id, toggle_state)
}

/// # Checkbox
///
/// Checkboxes are used for multiple choices, not for mutually exclusive choices.
/// Each checkbox works independently from other checkboxes in the list,
/// therefore checking an additional box does not affect any other selections.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    toggle_state: ToggleState,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ToggleState, &mut WindowContext) + 'static>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, checked: ToggleState) -> Self {
        Self {
            id: id.into(),
            toggle_state: checked,
            disabled: false,
            on_click: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ToggleState, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let group_id = format!("checkbox_group_{:?}", self.id);

        let icon = match self.toggle_state {
            ToggleState::Selected => Some(Icon::new(IconName::Check).size(IconSize::Small).color(
                if self.disabled {
                    Color::Disabled
                } else {
                    Color::Selected
                },
            )),
            ToggleState::Indeterminate => Some(
                Icon::new(IconName::Dash)
                    .size(IconSize::Small)
                    .color(if self.disabled {
                        Color::Disabled
                    } else {
                        Color::Selected
                    }),
            ),
            ToggleState::Unselected => None,
        };

        let selected = self.toggle_state == ToggleState::Selected
            || self.toggle_state == ToggleState::Indeterminate;

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
                |this, on_click| {
                    this.on_click(move |_, cx| on_click(&self.toggle_state.inverse(), cx))
                },
            )
    }
}

/// A [`Checkbox`] that has a [`Label`].
#[derive(IntoElement)]
pub struct CheckboxWithLabel {
    id: ElementId,
    label: Label,
    checked: ToggleState,
    on_click: Arc<dyn Fn(&ToggleState, &mut WindowContext) + 'static>,
}

impl CheckboxWithLabel {
    pub fn new(
        id: impl Into<ElementId>,
        label: Label,
        checked: ToggleState,
        on_click: impl Fn(&ToggleState, &mut WindowContext) + 'static,
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

/// # Switch
///
/// Switches are used to represent opposite states, such as enabled or disabled.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    toggle_state: ToggleState,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ToggleState, &mut WindowContext) + 'static>>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>, state: ToggleState) -> Self {
        Self {
            id: id.into(),
            toggle_state: state,
            disabled: false,
            on_click: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ToggleState, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let is_on = self.toggle_state == ToggleState::Selected;
        let adjust_ratio = if is_light(cx) { 1.5 } else { 1.0 };
        let base_color = cx.theme().colors().text;

        let bg_color = if is_on {
            cx.theme()
                .colors()
                .element_background
                .blend(base_color.opacity(0.08))
        } else {
            cx.theme().colors().element_background
        };
        let thumb_color = base_color.opacity(0.8);
        let thumb_hover_color = base_color;
        let border_color = cx.theme().colors().border_variant;
        // Lighter themes need higher contrast borders
        let border_hover_color = if is_on {
            border_color.blend(base_color.opacity(0.16 * adjust_ratio))
        } else {
            border_color.blend(base_color.opacity(0.05 * adjust_ratio))
        };
        let thumb_opacity = match (is_on, self.disabled) {
            (_, true) => 0.2,
            (true, false) => 1.0,
            (false, false) => 0.5,
        };

        let group_id = format!("switch_group_{:?}", self.id);

        h_flex()
            .id(self.id)
            .items_center()
            .w(DynamicSpacing::Base32.rems(cx))
            .h(DynamicSpacing::Base20.rems(cx))
            .group(group_id.clone())
            .child(
                h_flex()
                    .when(is_on, |on| on.justify_end())
                    .when(!is_on, |off| off.justify_start())
                    .items_center()
                    .size_full()
                    .rounded_full()
                    .px(DynamicSpacing::Base02.px(cx))
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .when(!self.disabled, |this| {
                        this.group_hover(group_id.clone(), |el| el.border_color(border_hover_color))
                    })
                    .child(
                        div()
                            .size(DynamicSpacing::Base12.rems(cx))
                            .rounded_full()
                            .bg(thumb_color)
                            .when(!self.disabled, |this| {
                                this.group_hover(group_id.clone(), |el| el.bg(thumb_hover_color))
                            })
                            .opacity(thumb_opacity),
                    ),
            )
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_, cx| on_click(&self.toggle_state.inverse(), cx))
                },
            )
    }
}

impl ComponentPreview for Checkbox {
    fn description() -> impl Into<Option<&'static str>> {
        "A checkbox lets people choose between a pair of opposing states, like enabled and disabled, using a different appearance to indicate each state."
    }

    fn examples(_: &mut WindowContext) -> Vec<ComponentExampleGroup<Self>> {
        vec![
            example_group_with_title(
                "Default",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_unselected", ToggleState::Unselected),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new("checkbox_indeterminate", ToggleState::Indeterminate),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_selected", ToggleState::Selected),
                    ),
                ],
            ),
            example_group_with_title(
                "Disabled",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_disabled_unselected", ToggleState::Unselected)
                            .disabled(true),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new(
                            "checkbox_disabled_indeterminate",
                            ToggleState::Indeterminate,
                        )
                        .disabled(true),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_disabled_selected", ToggleState::Selected)
                            .disabled(true),
                    ),
                ],
            ),
        ]
    }
}

impl ComponentPreview for Switch {
    fn description() -> impl Into<Option<&'static str>> {
        "A switch toggles between two mutually exclusive states, typically used for enabling or disabling a setting."
    }

    fn examples(_cx: &mut WindowContext) -> Vec<ComponentExampleGroup<Self>> {
        vec![
            example_group_with_title(
                "Default",
                vec![
                    single_example(
                        "Off",
                        Switch::new("switch_off", ToggleState::Unselected).on_click(|_, _cx| {}),
                    ),
                    single_example(
                        "On",
                        Switch::new("switch_on", ToggleState::Selected).on_click(|_, _cx| {}),
                    ),
                ],
            ),
            example_group_with_title(
                "Disabled",
                vec![
                    single_example(
                        "Off",
                        Switch::new("switch_disabled_off", ToggleState::Unselected).disabled(true),
                    ),
                    single_example(
                        "On",
                        Switch::new("switch_disabled_on", ToggleState::Selected).disabled(true),
                    ),
                ],
            ),
        ]
    }
}

impl ComponentPreview for CheckboxWithLabel {
    fn description() -> impl Into<Option<&'static str>> {
        "A checkbox with an associated label, allowing users to select an option while providing a descriptive text."
    }

    fn examples(_: &mut WindowContext) -> Vec<ComponentExampleGroup<Self>> {
        vec![example_group(vec![
            single_example(
                "Unselected",
                CheckboxWithLabel::new(
                    "checkbox_with_label_unselected",
                    Label::new("Always save on quit"),
                    ToggleState::Unselected,
                    |_, _| {},
                ),
            ),
            single_example(
                "Indeterminate",
                CheckboxWithLabel::new(
                    "checkbox_with_label_indeterminate",
                    Label::new("Always save on quit"),
                    ToggleState::Indeterminate,
                    |_, _| {},
                ),
            ),
            single_example(
                "Selected",
                CheckboxWithLabel::new(
                    "checkbox_with_label_selected",
                    Label::new("Always save on quit"),
                    ToggleState::Selected,
                    |_, _| {},
                ),
            ),
        ])]
    }
}
