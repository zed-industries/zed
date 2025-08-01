use gpui::{AnyView, ClickEvent};

use crate::{ButtonLike, ButtonLikeRounding, ElevationIndex, TintColor, prelude::*};

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
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn sort_name() -> &'static str {
        "ButtonC"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
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

pub struct ButtonConfiguration {
    label: SharedString,
    icon: Option<IconName>,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    selected: bool,
}

mod private {
    pub trait ToggleButtonStyle {}
}

pub trait ButtonBuilder: 'static + private::ToggleButtonStyle {
    fn into_configuration(self) -> ButtonConfiguration;
}

pub struct ToggleButtonSimple {
    label: SharedString,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    selected: bool,
}

impl ToggleButtonSimple {
    pub fn new(
        label: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            on_click: Box::new(on_click),
            selected: false,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl private::ToggleButtonStyle for ToggleButtonSimple {}

impl ButtonBuilder for ToggleButtonSimple {
    fn into_configuration(self) -> ButtonConfiguration {
        ButtonConfiguration {
            label: self.label,
            icon: None,
            on_click: self.on_click,
            selected: self.selected,
        }
    }
}

pub struct ToggleButtonWithIcon {
    label: SharedString,
    icon: IconName,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    selected: bool,
}

impl ToggleButtonWithIcon {
    pub fn new(
        label: impl Into<SharedString>,
        icon: IconName,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            icon,
            on_click: Box::new(on_click),
            selected: false,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl private::ToggleButtonStyle for ToggleButtonWithIcon {}

impl ButtonBuilder for ToggleButtonWithIcon {
    fn into_configuration(self) -> ButtonConfiguration {
        ButtonConfiguration {
            label: self.label,
            icon: Some(self.icon),
            on_click: self.on_click,
            selected: self.selected,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ToggleButtonGroupStyle {
    Transparent,
    Filled,
    Outlined,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ToggleButtonGroupSize {
    Default,
    Medium,
}

#[derive(IntoElement)]
pub struct ToggleButtonGroup<T, const COLS: usize = 3, const ROWS: usize = 1>
where
    T: ButtonBuilder,
{
    group_name: &'static str,
    rows: [[T; COLS]; ROWS],
    style: ToggleButtonGroupStyle,
    size: ToggleButtonGroupSize,
    button_width: Rems,
    selected_index: usize,
}

impl<T: ButtonBuilder, const COLS: usize> ToggleButtonGroup<T, COLS> {
    pub fn single_row(group_name: &'static str, buttons: [T; COLS]) -> Self {
        Self {
            group_name,
            rows: [buttons],
            style: ToggleButtonGroupStyle::Transparent,
            size: ToggleButtonGroupSize::Default,
            button_width: rems_from_px(100.),
            selected_index: 0,
        }
    }
}

impl<T: ButtonBuilder, const COLS: usize> ToggleButtonGroup<T, COLS, 2> {
    pub fn two_rows(group_name: &'static str, first_row: [T; COLS], second_row: [T; COLS]) -> Self {
        Self {
            group_name,
            rows: [first_row, second_row],
            style: ToggleButtonGroupStyle::Transparent,
            size: ToggleButtonGroupSize::Default,
            button_width: rems_from_px(100.),
            selected_index: 0,
        }
    }
}

impl<T: ButtonBuilder, const COLS: usize, const ROWS: usize> ToggleButtonGroup<T, COLS, ROWS> {
    pub fn style(mut self, style: ToggleButtonGroupStyle) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, size: ToggleButtonGroupSize) -> Self {
        self.size = size;
        self
    }

    pub fn button_width(mut self, button_width: Rems) -> Self {
        self.button_width = button_width;
        self
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }
}

impl<T: ButtonBuilder, const COLS: usize, const ROWS: usize> RenderOnce
    for ToggleButtonGroup<T, COLS, ROWS>
{
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let entries =
            self.rows.into_iter().enumerate().map(|(row_index, row)| {
                row.into_iter().enumerate().map(move |(col_index, button)| {
                    let ButtonConfiguration {
                        label,
                        icon,
                        on_click,
                        selected,
                    } = button.into_configuration();

                    let entry_index = row_index * COLS + col_index;

                    ButtonLike::new((self.group_name, entry_index))
                        .when(entry_index == self.selected_index || selected, |this| {
                            this.toggle_state(true)
                                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                        })
                        .rounding(None)
                        .when(self.style == ToggleButtonGroupStyle::Filled, |button| {
                            button.style(ButtonStyle::Filled)
                        })
                        .when(self.size == ToggleButtonGroupSize::Medium, |button| {
                            button.size(ButtonSize::Medium)
                        })
                        .child(
                            h_flex()
                                .min_w(self.button_width)
                                .gap_1p5()
                                .px_3()
                                .py_1()
                                .justify_center()
                                .when_some(icon, |this, icon| {
                                    this.py_2()
                                        .child(Icon::new(icon).size(IconSize::XSmall).map(|this| {
                                            if entry_index == self.selected_index || selected {
                                                this.color(Color::Accent)
                                            } else {
                                                this.color(Color::Muted)
                                            }
                                        }))
                                })
                                .child(Label::new(label).size(LabelSize::Small).when(
                                    entry_index == self.selected_index || selected,
                                    |this| this.color(Color::Accent),
                                )),
                        )
                        .on_click(on_click)
                        .into_any_element()
                })
            });

        let border_color = cx.theme().colors().border.opacity(0.6);
        let is_outlined_or_filled = self.style == ToggleButtonGroupStyle::Outlined
            || self.style == ToggleButtonGroupStyle::Filled;
        let is_transparent = self.style == ToggleButtonGroupStyle::Transparent;

        v_flex()
            .rounded_md()
            .overflow_hidden()
            .map(|this| {
                if is_transparent {
                    this.gap_px()
                } else {
                    this.border_1().border_color(border_color)
                }
            })
            .children(entries.enumerate().map(|(row_index, row)| {
                let last_row = row_index == ROWS - 1;
                h_flex()
                    .when(!is_outlined_or_filled, |this| this.gap_px())
                    .when(is_outlined_or_filled && !last_row, |this| {
                        this.border_b_1().border_color(border_color)
                    })
                    .children(row.enumerate().map(|(item_index, item)| {
                        let last_item = item_index == COLS - 1;
                        div()
                            .when(is_outlined_or_filled && !last_item, |this| {
                                this.border_r_1().border_color(border_color)
                            })
                            .child(item)
                    }))
            }))
    }
}

fn register_toggle_button_group() {
    component::register_component::<ToggleButtonGroup<ToggleButtonSimple>>();
}

component::__private::inventory::submit! {
    component::ComponentFn::new(register_toggle_button_group)
}

impl<T: ButtonBuilder, const COLS: usize, const ROWS: usize> Component
    for ToggleButtonGroup<T, COLS, ROWS>
{
    fn name() -> &'static str {
        "ToggleButtonGroup"
    }

    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn sort_name() -> &'static str {
        "ButtonG"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group_with_title(
                    "Transparent Variant",
                    vec![
                        single_example(
                            "Single Row Group",
                            ToggleButtonGroup::single_row(
                                "single_row_test",
                                [
                                    ToggleButtonSimple::new("First", |_, _, _| {}),
                                    ToggleButtonSimple::new("Second", |_, _, _| {}),
                                    ToggleButtonSimple::new("Third", |_, _, _| {}),
                                ],
                            )
                            .selected_index(1)
                            .button_width(rems_from_px(100.))
                            .into_any_element(),
                        ),
                        single_example(
                            "Single Row Group with icons",
                            ToggleButtonGroup::single_row(
                                "single_row_test_icon",
                                [
                                    ToggleButtonWithIcon::new(
                                        "First",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Second",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Third",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                            )
                            .selected_index(1)
                            .button_width(rems_from_px(100.))
                            .into_any_element(),
                        ),
                        single_example(
                            "Multiple Row Group",
                            ToggleButtonGroup::two_rows(
                                "multiple_row_test",
                                [
                                    ToggleButtonSimple::new("First", |_, _, _| {}),
                                    ToggleButtonSimple::new("Second", |_, _, _| {}),
                                    ToggleButtonSimple::new("Third", |_, _, _| {}),
                                ],
                                [
                                    ToggleButtonSimple::new("Fourth", |_, _, _| {}),
                                    ToggleButtonSimple::new("Fifth", |_, _, _| {}),
                                    ToggleButtonSimple::new("Sixth", |_, _, _| {}),
                                ],
                            )
                            .selected_index(3)
                            .button_width(rems_from_px(100.))
                            .into_any_element(),
                        ),
                        single_example(
                            "Multiple Row Group with Icons",
                            ToggleButtonGroup::two_rows(
                                "multiple_row_test_icons",
                                [
                                    ToggleButtonWithIcon::new(
                                        "First",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Second",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Third",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                                [
                                    ToggleButtonWithIcon::new(
                                        "Fourth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Fifth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Sixth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                            )
                            .selected_index(3)
                            .button_width(rems_from_px(100.))
                            .into_any_element(),
                        ),
                    ],
                )])
                .children(vec![example_group_with_title(
                    "Outlined Variant",
                    vec![
                        single_example(
                            "Single Row Group",
                            ToggleButtonGroup::single_row(
                                "single_row_test_outline",
                                [
                                    ToggleButtonSimple::new("First", |_, _, _| {}),
                                    ToggleButtonSimple::new("Second", |_, _, _| {}),
                                    ToggleButtonSimple::new("Third", |_, _, _| {}),
                                ],
                            )
                            .selected_index(1)
                            .style(ToggleButtonGroupStyle::Outlined)
                            .into_any_element(),
                        ),
                        single_example(
                            "Single Row Group with icons",
                            ToggleButtonGroup::single_row(
                                "single_row_test_icon_outlined",
                                [
                                    ToggleButtonWithIcon::new(
                                        "First",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Second",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Third",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                            )
                            .selected_index(1)
                            .button_width(rems_from_px(100.))
                            .style(ToggleButtonGroupStyle::Outlined)
                            .into_any_element(),
                        ),
                        single_example(
                            "Multiple Row Group",
                            ToggleButtonGroup::two_rows(
                                "multiple_row_test",
                                [
                                    ToggleButtonSimple::new("First", |_, _, _| {}),
                                    ToggleButtonSimple::new("Second", |_, _, _| {}),
                                    ToggleButtonSimple::new("Third", |_, _, _| {}),
                                ],
                                [
                                    ToggleButtonSimple::new("Fourth", |_, _, _| {}),
                                    ToggleButtonSimple::new("Fifth", |_, _, _| {}),
                                    ToggleButtonSimple::new("Sixth", |_, _, _| {}),
                                ],
                            )
                            .selected_index(3)
                            .button_width(rems_from_px(100.))
                            .style(ToggleButtonGroupStyle::Outlined)
                            .into_any_element(),
                        ),
                        single_example(
                            "Multiple Row Group with Icons",
                            ToggleButtonGroup::two_rows(
                                "multiple_row_test",
                                [
                                    ToggleButtonWithIcon::new(
                                        "First",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Second",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Third",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                                [
                                    ToggleButtonWithIcon::new(
                                        "Fourth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Fifth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Sixth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                            )
                            .selected_index(3)
                            .button_width(rems_from_px(100.))
                            .style(ToggleButtonGroupStyle::Outlined)
                            .into_any_element(),
                        ),
                    ],
                )])
                .children(vec![example_group_with_title(
                    "Filled Variant",
                    vec![
                        single_example(
                            "Single Row Group",
                            ToggleButtonGroup::single_row(
                                "single_row_test_outline",
                                [
                                    ToggleButtonSimple::new("First", |_, _, _| {}),
                                    ToggleButtonSimple::new("Second", |_, _, _| {}),
                                    ToggleButtonSimple::new("Third", |_, _, _| {}),
                                ],
                            )
                            .selected_index(2)
                            .style(ToggleButtonGroupStyle::Filled)
                            .into_any_element(),
                        ),
                        single_example(
                            "Single Row Group with icons",
                            ToggleButtonGroup::single_row(
                                "single_row_test_icon_outlined",
                                [
                                    ToggleButtonWithIcon::new(
                                        "First",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Second",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Third",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                            )
                            .selected_index(1)
                            .button_width(rems_from_px(100.))
                            .style(ToggleButtonGroupStyle::Filled)
                            .into_any_element(),
                        ),
                        single_example(
                            "Multiple Row Group",
                            ToggleButtonGroup::two_rows(
                                "multiple_row_test",
                                [
                                    ToggleButtonSimple::new("First", |_, _, _| {}),
                                    ToggleButtonSimple::new("Second", |_, _, _| {}),
                                    ToggleButtonSimple::new("Third", |_, _, _| {}),
                                ],
                                [
                                    ToggleButtonSimple::new("Fourth", |_, _, _| {}),
                                    ToggleButtonSimple::new("Fifth", |_, _, _| {}),
                                    ToggleButtonSimple::new("Sixth", |_, _, _| {}),
                                ],
                            )
                            .selected_index(3)
                            .button_width(rems_from_px(100.))
                            .style(ToggleButtonGroupStyle::Filled)
                            .into_any_element(),
                        ),
                        single_example(
                            "Multiple Row Group with Icons",
                            ToggleButtonGroup::two_rows(
                                "multiple_row_test",
                                [
                                    ToggleButtonWithIcon::new(
                                        "First",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Second",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Third",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                                [
                                    ToggleButtonWithIcon::new(
                                        "Fourth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Fifth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                    ToggleButtonWithIcon::new(
                                        "Sixth",
                                        IconName::AiZed,
                                        |_, _, _| {},
                                    ),
                                ],
                            )
                            .selected_index(3)
                            .button_width(rems_from_px(100.))
                            .style(ToggleButtonGroupStyle::Filled)
                            .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
