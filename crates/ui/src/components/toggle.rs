use gpui::{
    AnyElement, AnyView, ElementId, Hsla, IntoElement, Styled, Window, div, hsla, prelude::*,
};
use std::sync::Arc;

use crate::utils::is_light;
use crate::{Color, Icon, IconName, ToggleState};
use crate::{ElevationIndex, KeyBinding, prelude::*};

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
#[derive(IntoElement, RegisterComponent)]
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
            ToggleStyle::ElevationBased(_) => cx.theme().colors().border,
            ToggleStyle::Custom(color) => color.opacity(0.3),
        }
    }

    /// container size
    pub fn container_size() -> Pixels {
        px(20.0)
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let group_id = format!("checkbox_group_{:?}", self.id);
        let color = if self.disabled {
            Color::Disabled
        } else {
            Color::Selected
        };
        let icon = match self.toggle_state {
            ToggleState::Selected => {
                if self.placeholder {
                    None
                } else {
                    Some(
                        Icon::new(IconName::Check)
                            .size(IconSize::Small)
                            .color(color),
                    )
                }
            }
            ToggleState::Indeterminate => {
                Some(Icon::new(IconName::Dash).size(IconSize::Small).color(color))
            }
            ToggleState::Unselected => None,
        };

        let bg_color = self.bg_color(cx);
        let border_color = self.border_color(cx);
        let hover_border_color = border_color.alpha(0.7);

        let size = Self::container_size();

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
                    .m_1()
                    .size_4()
                    .rounded_xs()
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .when(self.disabled, |this| this.cursor_not_allowed())
                    .when(self.disabled, |this| {
                        this.bg(cx.theme().colors().element_disabled.opacity(0.6))
                    })
                    .when(!self.disabled, |this| {
                        this.group_hover(group_id.clone(), |el| el.border_color(hover_border_color))
                    })
                    .when(self.placeholder, |this| {
                        this.child(
                            div()
                                .flex_none()
                                .rounded_full()
                                .bg(color.color(cx).alpha(0.5))
                                .size(px(4.)),
                        )
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
#[derive(IntoElement, RegisterComponent)]
pub struct CheckboxWithLabel {
    id: ElementId,
    label: Label,
    checked: ToggleState,
    on_click: Arc<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>,
    filled: bool,
    style: ToggleStyle,
    checkbox_position: IconPosition,
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
            checkbox_position: IconPosition::Start,
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

    pub fn checkbox_position(mut self, position: IconPosition) -> Self {
        self.checkbox_position = position;
        self
    }
}

impl RenderOnce for CheckboxWithLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .gap(DynamicSpacing::Base08.rems(cx))
            .when(self.checkbox_position == IconPosition::Start, |this| {
                this.child(
                    Checkbox::new(self.id.clone(), self.checked)
                        .style(self.style.clone())
                        .when(self.filled, Checkbox::fill)
                        .on_click({
                            let on_click = self.on_click.clone();
                            move |checked, window, cx| {
                                (on_click)(checked, window, cx);
                            }
                        }),
                )
            })
            .child(
                div()
                    .id(SharedString::from(format!("{}-label", self.id)))
                    .on_click({
                        let on_click = self.on_click.clone();
                        move |_event, window, cx| {
                            (on_click)(&self.checked.inverse(), window, cx);
                        }
                    })
                    .child(self.label),
            )
            .when(self.checkbox_position == IconPosition::End, |this| {
                this.child(
                    Checkbox::new(self.id.clone(), self.checked)
                        .style(self.style)
                        .when(self.filled, Checkbox::fill)
                        .on_click(move |checked, window, cx| {
                            (self.on_click)(checked, window, cx);
                        }),
                )
            })
    }
}

/// Defines the color for a switch component.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum SwitchColor {
    #[default]
    Default,
    Accent,
    Error,
    Warning,
    Success,
    Custom(Hsla),
}

impl SwitchColor {
    fn get_colors(&self, is_on: bool, cx: &App) -> (Hsla, Hsla) {
        if !is_on {
            return (
                cx.theme().colors().element_disabled,
                cx.theme().colors().border,
            );
        }

        match self {
            SwitchColor::Default => {
                let colors = cx.theme().colors();
                let base_color = colors.text;
                let bg_color = colors.element_background.blend(base_color.opacity(0.08));
                (bg_color, colors.border_variant)
            }
            SwitchColor::Accent => {
                let status = cx.theme().status();
                (status.info.opacity(0.4), status.info.opacity(0.2))
            }
            SwitchColor::Error => {
                let status = cx.theme().status();
                (status.error.opacity(0.4), status.error.opacity(0.2))
            }
            SwitchColor::Warning => {
                let status = cx.theme().status();
                (status.warning.opacity(0.4), status.warning.opacity(0.2))
            }
            SwitchColor::Success => {
                let status = cx.theme().status();
                (status.success.opacity(0.4), status.success.opacity(0.2))
            }
            SwitchColor::Custom(color) => (*color, color.opacity(0.6)),
        }
    }
}

impl From<SwitchColor> for Color {
    fn from(color: SwitchColor) -> Self {
        match color {
            SwitchColor::Default => Color::Default,
            SwitchColor::Accent => Color::Accent,
            SwitchColor::Error => Color::Error,
            SwitchColor::Warning => Color::Warning,
            SwitchColor::Success => Color::Success,
            SwitchColor::Custom(_) => Color::Default,
        }
    }
}

/// # Switch
///
/// Switches are used to represent opposite states, such as enabled or disabled.
#[derive(IntoElement, RegisterComponent)]
pub struct Switch {
    id: ElementId,
    toggle_state: ToggleState,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>>,
    label: Option<SharedString>,
    key_binding: Option<KeyBinding>,
    color: SwitchColor,
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
            color: SwitchColor::default(),
        }
    }

    /// Sets the color of the switch using the specified [`SwitchColor`].
    pub fn color(mut self, color: SwitchColor) -> Self {
        self.color = color;
        self
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
        let thumb_color = base_color;
        let (bg_color, border_color) = self.color.get_colors(is_on, cx);

        let bg_hover_color = if is_on {
            bg_color.blend(base_color.opacity(0.16 * adjust_ratio))
        } else {
            bg_color.blend(base_color.opacity(0.05 * adjust_ratio))
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
                    .size_full()
                    .rounded_full()
                    .px(DynamicSpacing::Base02.px(cx))
                    .bg(bg_color)
                    .when(!self.disabled, |this| {
                        this.group_hover(group_id.clone(), |el| el.bg(bg_hover_color))
                    })
                    .border_1()
                    .border_color(border_color)
                    .child(
                        div()
                            .size(DynamicSpacing::Base12.rems(cx))
                            .rounded_full()
                            .bg(thumb_color)
                            .opacity(thumb_opacity),
                    ),
            );

        h_flex()
            .id(self.id)
            .gap(DynamicSpacing::Base06.rems(cx))
            .cursor_pointer()
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

/// A [`Switch`] that has a [`Label`].
#[derive(IntoElement)]
pub struct SwitchWithLabel {
    id: ElementId,
    label: Label,
    toggle_state: ToggleState,
    on_click: Arc<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>,
    disabled: bool,
    color: SwitchColor,
}

impl SwitchWithLabel {
    /// Creates a switch with an attached label.
    pub fn new(
        id: impl Into<ElementId>,
        label: Label,
        toggle_state: impl Into<ToggleState>,
        on_click: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label,
            toggle_state: toggle_state.into(),
            on_click: Arc::new(on_click),
            disabled: false,
            color: SwitchColor::default(),
        }
    }

    /// Sets the disabled state of the [`SwitchWithLabel`].
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the color of the switch using the specified [`SwitchColor`].
    pub fn color(mut self, color: SwitchColor) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for SwitchWithLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(SharedString::from(format!("{}-container", self.id)))
            .gap(DynamicSpacing::Base08.rems(cx))
            .child(
                Switch::new(self.id.clone(), self.toggle_state)
                    .disabled(self.disabled)
                    .color(self.color)
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
                    .child(self.label),
            )
    }
}

impl Component for Checkbox {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> Option<&'static str> {
        Some("A checkbox component that can be used for multiple choice selections")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "States",
                        vec![
                            single_example(
                                "Unselected",
                                Checkbox::new("checkbox_unselected", ToggleState::Unselected)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Placeholder",
                                Checkbox::new("checkbox_indeterminate", ToggleState::Selected)
                                    .placeholder(true)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Indeterminate",
                                Checkbox::new("checkbox_indeterminate", ToggleState::Indeterminate)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Selected",
                                Checkbox::new("checkbox_selected", ToggleState::Selected)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Styles",
                        vec![
                            single_example(
                                "Default",
                                Checkbox::new("checkbox_default", ToggleState::Selected)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Filled",
                                Checkbox::new("checkbox_filled", ToggleState::Selected)
                                    .fill()
                                    .into_any_element(),
                            ),
                            single_example(
                                "ElevationBased",
                                Checkbox::new("checkbox_elevation", ToggleState::Selected)
                                    .style(ToggleStyle::ElevationBased(
                                        ElevationIndex::EditorSurface,
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Custom Color",
                                Checkbox::new("checkbox_custom", ToggleState::Selected)
                                    .style(ToggleStyle::Custom(hsla(142.0 / 360., 0.68, 0.45, 0.7)))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Disabled",
                        vec![
                            single_example(
                                "Unselected",
                                Checkbox::new(
                                    "checkbox_disabled_unselected",
                                    ToggleState::Unselected,
                                )
                                .disabled(true)
                                .into_any_element(),
                            ),
                            single_example(
                                "Selected",
                                Checkbox::new("checkbox_disabled_selected", ToggleState::Selected)
                                    .disabled(true)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "With Label",
                        vec![single_example(
                            "Default",
                            Checkbox::new("checkbox_with_label", ToggleState::Selected)
                                .label("Always save on quit")
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}

impl Component for Switch {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> Option<&'static str> {
        Some("A switch component that represents binary states like on/off")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "States",
                        vec![
                            single_example(
                                "Off",
                                Switch::new("switch_off", ToggleState::Unselected)
                                    .on_click(|_, _, _cx| {})
                                    .into_any_element(),
                            ),
                            single_example(
                                "On",
                                Switch::new("switch_on", ToggleState::Selected)
                                    .on_click(|_, _, _cx| {})
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Colors",
                        vec![
                            single_example(
                                "Default",
                                Switch::new("switch_default_style", ToggleState::Selected)
                                    .color(SwitchColor::Default)
                                    .on_click(|_, _, _cx| {})
                                    .into_any_element(),
                            ),
                            single_example(
                                "Accent",
                                Switch::new("switch_accent_style", ToggleState::Selected)
                                    .color(SwitchColor::Accent)
                                    .on_click(|_, _, _cx| {})
                                    .into_any_element(),
                            ),
                            single_example(
                                "Error",
                                Switch::new("switch_error_style", ToggleState::Selected)
                                    .color(SwitchColor::Error)
                                    .on_click(|_, _, _cx| {})
                                    .into_any_element(),
                            ),
                            single_example(
                                "Warning",
                                Switch::new("switch_warning_style", ToggleState::Selected)
                                    .color(SwitchColor::Warning)
                                    .on_click(|_, _, _cx| {})
                                    .into_any_element(),
                            ),
                            single_example(
                                "Success",
                                Switch::new("switch_success_style", ToggleState::Selected)
                                    .color(SwitchColor::Success)
                                    .on_click(|_, _, _cx| {})
                                    .into_any_element(),
                            ),
                            single_example(
                                "Custom",
                                Switch::new("switch_custom_style", ToggleState::Selected)
                                    .color(SwitchColor::Custom(hsla(300.0 / 360.0, 0.6, 0.6, 1.0)))
                                    .on_click(|_, _, _cx| {})
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Disabled",
                        vec![
                            single_example(
                                "Off",
                                Switch::new("switch_disabled_off", ToggleState::Unselected)
                                    .disabled(true)
                                    .into_any_element(),
                            ),
                            single_example(
                                "On",
                                Switch::new("switch_disabled_on", ToggleState::Selected)
                                    .disabled(true)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "With Label",
                        vec![
                            single_example(
                                "Label",
                                Switch::new("switch_with_label", ToggleState::Selected)
                                    .label("Always save on quit")
                                    .into_any_element(),
                            ),
                            // TODO: Where did theme_preview_keybinding go?
                            // single_example(
                            //     "Keybinding",
                            //     Switch::new("switch_with_keybinding", ToggleState::Selected)
                            //         .key_binding(theme_preview_keybinding("cmd-shift-e"))
                            //         .into_any_element(),
                            // ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}

impl Component for CheckboxWithLabel {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> Option<&'static str> {
        Some("A checkbox component with an attached label")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group_with_title(
                    "States",
                    vec![
                        single_example(
                            "Unselected",
                            CheckboxWithLabel::new(
                                "checkbox_with_label_unselected",
                                Label::new("Always save on quit"),
                                ToggleState::Unselected,
                                |_, _, _| {},
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Indeterminate",
                            CheckboxWithLabel::new(
                                "checkbox_with_label_indeterminate",
                                Label::new("Always save on quit"),
                                ToggleState::Indeterminate,
                                |_, _, _| {},
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Selected",
                            CheckboxWithLabel::new(
                                "checkbox_with_label_selected",
                                Label::new("Always save on quit"),
                                ToggleState::Selected,
                                |_, _, _| {},
                            )
                            .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
