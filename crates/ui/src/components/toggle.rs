use gpui::{
    div, hsla, prelude::*, AnyView, CursorStyle, ElementId, Hsla, IntoElement, Styled, Window,
};
use std::sync::Arc;

use crate::utils::is_light;
use crate::{prelude::*, ElevationIndex, KeyBinding};
use crate::{Color, Icon, IconName, ToggleState};

// TODO: Checkbox, CheckboxWithLabel, and Switch could all be
// restructured to use a ToggleLike, similar to Button/Buttonlike, Label/Labellike

/// Creates a new checkbox.
pub fn checkbox(id: impl Into<ElementId>, toggle_state: ToggleState) -> Checkbox {
    Checkbox::new(id, toggle_state)
}

/// Creates a new switch.
pub fn switch(id: impl Into<ElementId>, toggle_state: ToggleState) -> Switch {
    Switch::new(id, toggle_state)
}

/// The visual style of a toggle.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ToggleStyle {
    /// Toggle has a transparent background
    #[default]
    Ghost,
    /// Toggle has a filled background based on the
    /// elevation index of the parent container
    ElevationBased(ElevationIndex),
    /// A custom style using a color to tint the toggle
    Custom(Hsla),
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
    placeholder: bool,
    on_click: Option<Box<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>>,
    filled: bool,
    style: ToggleStyle,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    label: Option<SharedString>,
}

impl Checkbox {
    /// Creates a new [`Checkbox`].
    pub fn new(id: impl Into<ElementId>, checked: ToggleState) -> Self {
        Self {
            id: id.into(),
            toggle_state: checked,
            disabled: false,
            on_click: None,
            filled: false,
            style: ToggleStyle::default(),
            tooltip: None,
            label: None,
            placeholder: false,
        }
    }

    /// Sets the disabled state of the [`Checkbox`].
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the disabled state of the [`Checkbox`].
    pub fn placeholder(mut self, placeholder: bool) -> Self {
        self.placeholder = placeholder;
        self
    }

    /// Binds a handler to the [`Checkbox`] that will be called when clicked.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Sets the `fill` setting of the checkbox, indicating whether it should be filled.
    pub fn fill(mut self) -> Self {
        self.filled = true;
        self
    }

    /// Sets the style of the checkbox using the specified [`ToggleStyle`].
    pub fn style(mut self, style: ToggleStyle) -> Self {
        self.style = style;
        self
    }

    /// Match the style of the checkbox to the current elevation using [`ToggleStyle::ElevationBased`].
    pub fn elevation(mut self, elevation: ElevationIndex) -> Self {
        self.style = ToggleStyle::ElevationBased(elevation);
        self
    }

    /// Sets the tooltip for the checkbox.
    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    /// Set the label for the checkbox.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Checkbox {
    fn bg_color(&self, cx: &App) -> Hsla {
        let style = self.style.clone();
        match (style, self.filled) {
            (ToggleStyle::Ghost, false) => cx.theme().colors().ghost_element_background,
            (ToggleStyle::Ghost, true) => cx.theme().colors().element_background,
            (ToggleStyle::ElevationBased(_), false) => gpui::transparent_black(),
            (ToggleStyle::ElevationBased(elevation), true) => elevation.darker_bg(cx),
            (ToggleStyle::Custom(_), false) => gpui::transparent_black(),
            (ToggleStyle::Custom(color), true) => color.opacity(0.2),
        }
    }

    fn border_color(&self, cx: &App) -> Hsla {
        if self.disabled {
            return cx.theme().colors().border_variant;
        }

        match self.style.clone() {
            ToggleStyle::Ghost => cx.theme().colors().border,
            ToggleStyle::ElevationBased(elevation) => elevation.on_elevation_bg(cx),
            ToggleStyle::Custom(color) => color.opacity(0.3),
        }
    }

    /// container size
    pub fn container_size(cx: &App) -> Rems {
        DynamicSpacing::Base20.rems(cx)
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let group_id = format!("checkbox_group_{:?}", self.id);
        let color = if self.disabled {
            Color::Disabled
        } else if self.placeholder {
            Color::Placeholder
        } else {
            Color::Selected
        };
        let icon = match self.toggle_state {
            ToggleState::Selected => Some(if self.placeholder {
                Icon::new(IconName::Circle)
                    .size(IconSize::XSmall)
                    .color(color)
            } else {
                Icon::new(IconName::Check)
                    .size(IconSize::Small)
                    .color(color)
            }),
            ToggleState::Indeterminate => {
                Some(Icon::new(IconName::Dash).size(IconSize::Small).color(color))
            }
            ToggleState::Unselected => None,
        };
        if self.placeholder {}

        let bg_color = self.bg_color(cx);
        let border_color = self.border_color(cx);

        let size = Self::container_size(cx);

        let checkbox = h_flex()
            .id(self.id.clone())
            .justify_center()
            .items_center()
            .size(size)
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
                    .when(self.disabled, |this| {
                        this.cursor(CursorStyle::OperationNotAllowed)
                    })
                    .when(self.disabled, |this| {
                        this.bg(cx.theme().colors().element_disabled.opacity(0.6))
                    })
                    .when(!self.disabled, |this| {
                        this.group_hover(group_id.clone(), |el| {
                            el.bg(cx.theme().colors().element_hover)
                        })
                    })
                    .children(icon),
            );

        h_flex()
            .id(self.id)
            .gap(DynamicSpacing::Base06.rems(cx))
            .child(checkbox)
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        on_click(&self.toggle_state.inverse(), window, cx)
                    })
                },
            )
            // TODO: Allow label size to be different from default.
            // TODO: Allow label color to be different from muted.
            .when_some(self.label, |this, label| {
                this.child(Label::new(label).color(Color::Muted))
            })
            .when_some(self.tooltip, |this, tooltip| {
                this.tooltip(move |window, cx| tooltip(window, cx))
            })
    }
}

/// A [`Checkbox`] that has a [`Label`].
#[derive(IntoElement)]
pub struct CheckboxWithLabel {
    id: ElementId,
    label: Label,
    checked: ToggleState,
    on_click: Arc<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>,
    filled: bool,
    style: ToggleStyle,
}

// TODO: Remove `CheckboxWithLabel` now that `label` is a method of `Checkbox`.
impl CheckboxWithLabel {
    /// Creates a checkbox with an attached label.
    pub fn new(
        id: impl Into<ElementId>,
        label: Label,
        checked: ToggleState,
        on_click: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label,
            checked,
            on_click: Arc::new(on_click),
            filled: false,
            style: ToggleStyle::default(),
        }
    }

    /// Sets the style of the checkbox using the specified [`ToggleStyle`].
    pub fn style(mut self, style: ToggleStyle) -> Self {
        self.style = style;
        self
    }

    /// Match the style of the checkbox to the current elevation using [`ToggleStyle::ElevationBased`].
    pub fn elevation(mut self, elevation: ElevationIndex) -> Self {
        self.style = ToggleStyle::ElevationBased(elevation);
        self
    }

    /// Sets the `fill` setting of the checkbox, indicating whether it should be filled.
    pub fn fill(mut self) -> Self {
        self.filled = true;
        self
    }
}

impl RenderOnce for CheckboxWithLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .gap(DynamicSpacing::Base08.rems(cx))
            .child(
                Checkbox::new(self.id.clone(), self.checked)
                    .style(self.style)
                    .when(self.filled, Checkbox::fill)
                    .on_click({
                        let on_click = self.on_click.clone();
                        move |checked, window, cx| {
                            (on_click)(checked, window, cx);
                        }
                    }),
            )
            .child(
                div()
                    .id(SharedString::from(format!("{}-label", self.id)))
                    .on_click(move |_event, window, cx| {
                        (self.on_click)(&self.checked.inverse(), window, cx);
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
    on_click: Option<Box<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>>,
    label: Option<SharedString>,
    key_binding: Option<KeyBinding>,
}

impl Switch {
    /// Creates a new [`Switch`].
    pub fn new(id: impl Into<ElementId>, state: ToggleState) -> Self {
        Self {
            id: id.into(),
            toggle_state: state,
            disabled: false,
            on_click: None,
            label: None,
            key_binding: None,
        }
    }

    /// Sets the disabled state of the [`Switch`].
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Binds a handler to the [`Switch`] that will be called when clicked.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Sets the label of the [`Switch`].
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Display the keybinding that triggers the switch action.
    pub fn key_binding(mut self, key_binding: impl Into<Option<KeyBinding>>) -> Self {
        self.key_binding = key_binding.into();
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
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

        let switch = h_flex()
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
            );

        h_flex()
            .id(self.id)
            .gap(DynamicSpacing::Base06.rems(cx))
            .child(switch)
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        on_click(&self.toggle_state.inverse(), window, cx)
                    })
                },
            )
            .when_some(self.label, |this, label| {
                this.child(Label::new(label).size(LabelSize::Small))
            })
            .children(self.key_binding)
    }
}

impl ComponentPreview for Checkbox {
    fn description() -> impl Into<Option<&'static str>> {
        "A checkbox lets people choose between a pair of opposing states, like enabled and disabled, using a different appearance to indicate each state."
    }

    fn examples(_window: &mut Window, _: &mut App) -> Vec<ComponentExampleGroup<Self>> {
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
                "Default (Filled)",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_unselected", ToggleState::Unselected).fill(),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new("checkbox_indeterminate", ToggleState::Indeterminate).fill(),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_selected", ToggleState::Selected).fill(),
                    ),
                ],
            ),
            example_group_with_title(
                "ElevationBased",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_unfilled_unselected", ToggleState::Unselected)
                            .style(ToggleStyle::ElevationBased(ElevationIndex::EditorSurface)),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new(
                            "checkbox_unfilled_indeterminate",
                            ToggleState::Indeterminate,
                        )
                        .style(ToggleStyle::ElevationBased(ElevationIndex::EditorSurface)),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_unfilled_selected", ToggleState::Selected)
                            .style(ToggleStyle::ElevationBased(ElevationIndex::EditorSurface)),
                    ),
                ],
            ),
            example_group_with_title(
                "ElevationBased (Filled)",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_filled_unselected", ToggleState::Unselected)
                            .fill()
                            .style(ToggleStyle::ElevationBased(ElevationIndex::EditorSurface)),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new("checkbox_filled_indeterminate", ToggleState::Indeterminate)
                            .fill()
                            .style(ToggleStyle::ElevationBased(ElevationIndex::EditorSurface)),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_filled_selected", ToggleState::Selected)
                            .fill()
                            .style(ToggleStyle::ElevationBased(ElevationIndex::EditorSurface)),
                    ),
                ],
            ),
            example_group_with_title(
                "Custom Color",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_custom_unselected", ToggleState::Unselected)
                            .style(ToggleStyle::Custom(hsla(142.0 / 360., 0.68, 0.45, 0.7))),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new("checkbox_custom_indeterminate", ToggleState::Indeterminate)
                            .style(ToggleStyle::Custom(hsla(142.0 / 360., 0.68, 0.45, 0.7))),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_custom_selected", ToggleState::Selected)
                            .style(ToggleStyle::Custom(hsla(142.0 / 360., 0.68, 0.45, 0.7))),
                    ),
                ],
            ),
            example_group_with_title(
                "Custom Color (Filled)",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new("checkbox_custom_filled_unselected", ToggleState::Unselected)
                            .fill()
                            .style(ToggleStyle::Custom(hsla(142.0 / 360., 0.68, 0.45, 0.7))),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new(
                            "checkbox_custom_filled_indeterminate",
                            ToggleState::Indeterminate,
                        )
                        .fill()
                        .style(ToggleStyle::Custom(hsla(
                            142.0 / 360.,
                            0.68,
                            0.45,
                            0.7,
                        ))),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_custom_filled_selected", ToggleState::Selected)
                            .fill()
                            .style(ToggleStyle::Custom(hsla(142.0 / 360., 0.68, 0.45, 0.7))),
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
            example_group_with_title(
                "Disabled (Filled)",
                vec![
                    single_example(
                        "Unselected",
                        Checkbox::new(
                            "checkbox_disabled_filled_unselected",
                            ToggleState::Unselected,
                        )
                        .fill()
                        .disabled(true),
                    ),
                    single_example(
                        "Indeterminate",
                        Checkbox::new(
                            "checkbox_disabled_filled_indeterminate",
                            ToggleState::Indeterminate,
                        )
                        .fill()
                        .disabled(true),
                    ),
                    single_example(
                        "Selected",
                        Checkbox::new("checkbox_disabled_filled_selected", ToggleState::Selected)
                            .fill()
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

    fn examples(_window: &mut Window, _cx: &mut App) -> Vec<ComponentExampleGroup<Self>> {
        vec![
            example_group_with_title(
                "Default",
                vec![
                    single_example(
                        "Off",
                        Switch::new("switch_off", ToggleState::Unselected).on_click(|_, _, _cx| {}),
                    ),
                    single_example(
                        "On",
                        Switch::new("switch_on", ToggleState::Selected).on_click(|_, _, _cx| {}),
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
            example_group_with_title(
                "Label Permutations",
                vec![
                    single_example(
                        "Label",
                        Switch::new("switch_with_label", ToggleState::Selected)
                            .label("Always save on quit"),
                    ),
                    single_example(
                        "Keybinding",
                        Switch::new("switch_with_label", ToggleState::Selected)
                            .key_binding(theme_preview_keybinding("cmd-shift-e")),
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

    fn examples(_window: &mut Window, _: &mut App) -> Vec<ComponentExampleGroup<Self>> {
        vec![example_group(vec![
            single_example(
                "Unselected",
                CheckboxWithLabel::new(
                    "checkbox_with_label_unselected",
                    Label::new("Always save on quit"),
                    ToggleState::Unselected,
                    |_, _, _| {},
                ),
            ),
            single_example(
                "Indeterminate",
                CheckboxWithLabel::new(
                    "checkbox_with_label_indeterminate",
                    Label::new("Always save on quit"),
                    ToggleState::Indeterminate,
                    |_, _, _| {},
                ),
            ),
            single_example(
                "Selected",
                CheckboxWithLabel::new(
                    "checkbox_with_label_selected",
                    Label::new("Always save on quit"),
                    ToggleState::Selected,
                    |_, _, _| {},
                ),
            ),
        ])]
    }
}
