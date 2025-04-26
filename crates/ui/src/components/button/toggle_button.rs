use gpui::{AnyView, ClickEvent};

use crate::{ButtonLike, ButtonLikeRounding, ElevationIndex, prelude::*};

/// The position of a [`ToggleButton`] within a group of buttons.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ToggleButtonPosition {
    /// The toggle button is first in the group.
    First,

    /// The toggle button is in the middle of the group (i.e., it is not the first or last toggle button).
    Middle,

    /// The toggle button is last in the group.
    Last,
}

#[derive(IntoElement, RegisterComponent)]
pub struct ToggleButton {
    base: ButtonLike,
    position_in_group: Option<ToggleButtonPosition>,
    label: SharedString,
    label_color: Option<Color>,
}

impl ToggleButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: ButtonLike::new(id),
            position_in_group: None,
            label: label.into(),
            label_color: None,
        }
    }

    pub fn color(mut self, label_color: impl Into<Option<Color>>) -> Self {
        self.label_color = label_color.into();
        self
    }

    pub fn position_in_group(mut self, position: ToggleButtonPosition) -> Self {
        self.position_in_group = Some(position);
        self
    }

    pub fn first(self) -> Self {
        self.position_in_group(ToggleButtonPosition::First)
    }

    pub fn middle(self) -> Self {
        self.position_in_group(ToggleButtonPosition::Middle)
    }

    pub fn last(self) -> Self {
        self.position_in_group(ToggleButtonPosition::Last)
    }
}

impl Toggleable for ToggleButton {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.base = self.base.toggle_state(selected);
        self
    }
}

impl SelectableButton for ToggleButton {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.base.selected_style = Some(style);
        self
    }
}

impl FixedWidth for ToggleButton {
    fn width(mut self, width: DefiniteLength) -> Self {
        self.base.width = Some(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base.width = Some(relative(1.));
        self
    }
}

impl Disableable for ToggleButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Clickable for ToggleButton {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.base = self.base.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
        self
    }
}

impl ButtonCommon for ToggleButton {
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.style(style);
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.base = self.base.layer(elevation);
        self
    }
}

impl RenderOnce for ToggleButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;

        let label_color = if is_disabled {
            Color::Disabled
        } else if is_selected {
            Color::Selected
        } else {
            self.label_color.unwrap_or_default()
        };

        self.base
            .when_some(self.position_in_group, |this, position| match position {
                ToggleButtonPosition::First => this.rounding(ButtonLikeRounding::Left),
                ToggleButtonPosition::Middle => this.rounding(None),
                ToggleButtonPosition::Last => this.rounding(ButtonLikeRounding::Right),
            })
            .child(
                Label::new(self.label)
                    .color(label_color)
                    .line_height_style(LineHeightStyle::UiLabel),
            )
    }
}

impl Component for ToggleButton {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn sort_name() -> &'static str {
        "ButtonC"
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
                        "Button Styles",
                        vec![
                            single_example(
                                "Off",
                                ToggleButton::new("off", "Off")
                                    .layer(ElevationIndex::Background)
                                    .style(ButtonStyle::Filled)
                                    .into_any_element(),
                            ),
                            single_example(
                                "On",
                                ToggleButton::new("on", "On")
                                    .layer(ElevationIndex::Background)
                                    .toggle_state(true)
                                    .style(ButtonStyle::Filled)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Off – Disabled",
                                ToggleButton::new("disabled_off", "Disabled Off")
                                    .layer(ElevationIndex::Background)
                                    .disabled(true)
                                    .style(ButtonStyle::Filled)
                                    .into_any_element(),
                            ),
                            single_example(
                                "On – Disabled",
                                ToggleButton::new("disabled_on", "Disabled On")
                                    .layer(ElevationIndex::Background)
                                    .disabled(true)
                                    .toggle_state(true)
                                    .style(ButtonStyle::Filled)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Button Group",
                        vec![
                            single_example(
                                "Three Buttons",
                                h_flex()
                                    .child(
                                        ToggleButton::new("three_btn_first", "First")
                                            .layer(ElevationIndex::Background)
                                            .style(ButtonStyle::Filled)
                                            .first()
                                            .into_any_element(),
                                    )
                                    .child(
                                        ToggleButton::new("three_btn_middle", "Middle")
                                            .layer(ElevationIndex::Background)
                                            .style(ButtonStyle::Filled)
                                            .middle()
                                            .toggle_state(true)
                                            .into_any_element(),
                                    )
                                    .child(
                                        ToggleButton::new("three_btn_last", "Last")
                                            .layer(ElevationIndex::Background)
                                            .style(ButtonStyle::Filled)
                                            .last()
                                            .into_any_element(),
                                    )
                                    .into_any_element(),
                            ),
                            single_example(
                                "Two Buttons",
                                h_flex()
                                    .child(
                                        ToggleButton::new("two_btn_first", "First")
                                            .layer(ElevationIndex::Background)
                                            .style(ButtonStyle::Filled)
                                            .first()
                                            .into_any_element(),
                                    )
                                    .child(
                                        ToggleButton::new("two_btn_last", "Last")
                                            .layer(ElevationIndex::Background)
                                            .style(ButtonStyle::Filled)
                                            .last()
                                            .into_any_element(),
                                    )
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Alternate Sizes",
                        vec![
                            single_example(
                                "None",
                                ToggleButton::new("none", "None")
                                    .layer(ElevationIndex::Background)
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::None)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Compact",
                                ToggleButton::new("compact", "Compact")
                                    .layer(ElevationIndex::Background)
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Compact)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Large",
                                ToggleButton::new("large", "Large")
                                    .layer(ElevationIndex::Background)
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
