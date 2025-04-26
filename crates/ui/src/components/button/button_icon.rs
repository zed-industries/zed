use crate::{Icon, IconName, IconSize, IconWithIndicator, Indicator, prelude::*};
use gpui::Hsla;

/// An icon that appears within a button.
///
/// Can be used as either an icon alongside a label, like in [`Button`](crate::Button),
/// or as a standalone icon, like in [`IconButton`](crate::IconButton).
#[derive(IntoElement, RegisterComponent)]
pub(super) struct ButtonIcon {
    icon: IconName,
    size: IconSize,
    color: Color,
    disabled: bool,
    selected: bool,
    selected_icon: Option<IconName>,
    selected_icon_color: Option<Color>,
    selected_style: Option<ButtonStyle>,
    indicator: Option<Indicator>,
    indicator_border_color: Option<Hsla>,
}

impl ButtonIcon {
    pub fn new(icon: IconName) -> Self {
        Self {
            icon,
            size: IconSize::default(),
            color: Color::default(),
            disabled: false,
            selected: false,
            selected_icon: None,
            selected_icon_color: None,
            selected_style: None,
            indicator: None,
            indicator_border_color: None,
        }
    }

    pub fn size(mut self, size: impl Into<Option<IconSize>>) -> Self {
        if let Some(size) = size.into() {
            self.size = size;
        }
        self
    }

    pub fn color(mut self, color: impl Into<Option<Color>>) -> Self {
        if let Some(color) = color.into() {
            self.color = color;
        }
        self
    }

    pub fn selected_icon(mut self, icon: impl Into<Option<IconName>>) -> Self {
        self.selected_icon = icon.into();
        self
    }

    pub fn selected_icon_color(mut self, color: impl Into<Option<Color>>) -> Self {
        self.selected_icon_color = color.into();
        self
    }

    pub fn indicator(mut self, indicator: Indicator) -> Self {
        self.indicator = Some(indicator);
        self
    }

    pub fn indicator_border_color(mut self, color: Option<Hsla>) -> Self {
        self.indicator_border_color = color;
        self
    }
}

impl Disableable for ButtonIcon {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for ButtonIcon {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl SelectableButton for ButtonIcon {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.selected_style = Some(style);
        self
    }
}

impl RenderOnce for ButtonIcon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon = self
            .selected_icon
            .filter(|_| self.selected)
            .unwrap_or(self.icon);

        let icon_color = if self.disabled {
            Color::Disabled
        } else if self.selected_style.is_some() && self.selected {
            self.selected_style.unwrap().into()
        } else if self.selected {
            self.selected_icon_color.unwrap_or(Color::Selected)
        } else {
            self.color
        };

        let icon = Icon::new(icon).size(self.size).color(icon_color);

        match self.indicator {
            Some(indicator) => IconWithIndicator::new(icon, Some(indicator))
                .indicator_border_color(self.indicator_border_color)
                .into_any_element(),
            None => icon.into_any_element(),
        }
    }
}

impl Component for ButtonIcon {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn name() -> &'static str {
        "ButtonIcon"
    }

    fn description() -> Option<&'static str> {
        Some("An icon component specifically designed for use within buttons.")
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), _window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Usage",
                        vec![
                            single_example(
                                "Default",
                                ButtonIcon::new(IconName::Star).into_any_element(),
                            ),
                            single_example(
                                "Custom Size",
                                ButtonIcon::new(IconName::Star)
                                    .size(IconSize::Medium)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Custom Color",
                                ButtonIcon::new(IconName::Star)
                                    .color(Color::Accent)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "States",
                        vec![
                            single_example(
                                "Selected",
                                ButtonIcon::new(IconName::Star)
                                    .toggle_state(true)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Disabled",
                                ButtonIcon::new(IconName::Star)
                                    .disabled(true)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "With Indicator",
                        vec![
                            single_example(
                                "Default Indicator",
                                ButtonIcon::new(IconName::Star)
                                    .indicator(Indicator::dot())
                                    .into_any_element(),
                            ),
                            single_example(
                                "Custom Indicator",
                                ButtonIcon::new(IconName::Star)
                                    .indicator(Indicator::dot().color(Color::Error))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
