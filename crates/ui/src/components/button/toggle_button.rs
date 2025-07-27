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

mod private {
    pub trait Sealed {}
}

pub trait ButtonBuilder: 'static + private::Sealed {
    fn label(&self) -> impl Into<SharedString>;
    fn icon(&self) -> Option<IconName>;
    fn on_click(self) -> Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
}

pub struct ToggleButtonSimple {
    label: SharedString,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
}

impl ToggleButtonSimple {
    pub fn new(
        label: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            on_click: Box::new(on_click),
        }
    }
}

impl private::Sealed for ToggleButtonSimple {}

impl ButtonBuilder for ToggleButtonSimple {
    fn label(&self) -> impl Into<SharedString> {
        self.label.clone()
    }

    fn icon(&self) -> Option<IconName> {
        None
    }

    fn on_click(self) -> Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> {
        self.on_click
    }
}

pub struct ToggleButtonWithIcon {
    label: SharedString,
    icon: IconName,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
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
        }
    }
}

impl private::Sealed for ToggleButtonWithIcon {}

impl ButtonBuilder for ToggleButtonWithIcon {
    fn label(&self) -> impl Into<SharedString> {
        self.label.clone()
    }

    fn icon(&self) -> Option<IconName> {
        Some(self.icon)
    }

    fn on_click(self) -> Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> {
        self.on_click
    }
}

struct ToggleButtonRow<T: ButtonBuilder> {
    items: Vec<T>,
    index_offset: usize,
    last_item_idx: usize,
    is_last_row: bool,
}

impl<T: ButtonBuilder> ToggleButtonRow<T> {
    fn new(items: Vec<T>, index_offset: usize, is_last_row: bool) -> Self {
        Self {
            index_offset,
            last_item_idx: index_offset + items.len() - 1,
            is_last_row,
            items,
        }
    }
}

enum ToggleButtonGroupRows<T: ButtonBuilder> {
    Single(Vec<T>),
    Multiple(Vec<T>, Vec<T>),
}

impl<T: ButtonBuilder> ToggleButtonGroupRows<T> {
    fn items(self) -> impl IntoIterator<Item = ToggleButtonRow<T>> {
        match self {
            ToggleButtonGroupRows::Single(items) => {
                vec![ToggleButtonRow::new(items, 0, true)]
            }
            ToggleButtonGroupRows::Multiple(first_row, second_row) => {
                let row_len = first_row.len();
                vec![
                    ToggleButtonRow::new(first_row, 0, false),
                    ToggleButtonRow::new(second_row, row_len, true),
                ]
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ToggleButtonGroupStyle {
    Transparent,
    Filled,
    Outlined,
}

#[derive(IntoElement)]
pub struct ToggleButtonGroup<T>
where
    T: ButtonBuilder,
{
    group_name: SharedString,
    rows: ToggleButtonGroupRows<T>,
    style: ToggleButtonGroupStyle,
    button_width: Rems,
    selected_index: usize,
}

impl<T: ButtonBuilder> ToggleButtonGroup<T> {
    pub fn single_row(
        group_name: impl Into<SharedString>,
        buttons: impl IntoIterator<Item = T>,
    ) -> Self {
        Self {
            group_name: group_name.into(),
            rows: ToggleButtonGroupRows::Single(Vec::from_iter(buttons)),
            style: ToggleButtonGroupStyle::Transparent,
            button_width: rems_from_px(100.),
            selected_index: 0,
        }
    }

    pub fn multiple_rows<const ROWS: usize>(
        group_name: impl Into<SharedString>,
        first_row: [T; ROWS],
        second_row: [T; ROWS],
    ) -> Self {
        Self {
            group_name: group_name.into(),
            rows: ToggleButtonGroupRows::Multiple(
                Vec::from_iter(first_row),
                Vec::from_iter(second_row),
            ),
            style: ToggleButtonGroupStyle::Transparent,
            button_width: rems_from_px(100.),
            selected_index: 0,
        }
    }

    pub fn style(mut self, style: ToggleButtonGroupStyle) -> Self {
        self.style = style;
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

impl<T: ButtonBuilder> RenderOnce for ToggleButtonGroup<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let rows = self.rows.items().into_iter().map(|row| {
            (
                row.items
                    .into_iter()
                    .enumerate()
                    .map(move |(index, item)| (index + row.index_offset, row.last_item_idx, item))
                    .map(|(index, last_item_idx, item)| {
                        (
                            ButtonLike::new((self.group_name.clone(), index))
                                .when(index == self.selected_index, |this| {
                                    this.toggle_state(true)
                                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                })
                                .rounding(None)
                                .when(self.style == ToggleButtonGroupStyle::Filled, |button| {
                                    button.style(ButtonStyle::Filled)
                                })
                                .child(
                                    h_flex()
                                        .min_w(self.button_width)
                                        .gap_1p5()
                                        .justify_center()
                                        .when_some(item.icon(), |this, icon| {
                                            this.child(Icon::new(icon).size(IconSize::XSmall).map(
                                                |this| {
                                                    if index == self.selected_index {
                                                        this.color(Color::Accent)
                                                    } else {
                                                        this.color(Color::Muted)
                                                    }
                                                },
                                            ))
                                        })
                                        .child(
                                            Label::new(item.label())
                                                .when(index == self.selected_index, |this| {
                                                    this.color(Color::Accent)
                                                }),
                                        ),
                                )
                                .on_click(item.on_click()),
                            index == last_item_idx,
                        )
                    }),
                row.is_last_row,
            )
        });

        let is_outlined_or_filled = self.style == ToggleButtonGroupStyle::Outlined
            || self.style == ToggleButtonGroupStyle::Filled;
        let is_transparent = self.style == ToggleButtonGroupStyle::Transparent;
        let border_color = cx.theme().colors().border.opacity(0.6);

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
            .children(rows.map(|(items, last_row)| {
                h_flex()
                    .when(!is_outlined_or_filled, |this| this.gap_px())
                    .when(is_outlined_or_filled && !last_row, |this| {
                        this.border_b_1().border_color(border_color)
                    })
                    .children(items.map(|(item, last_item)| {
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

impl<T: ButtonBuilder> Component for ToggleButtonGroup<T> {
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
                            ToggleButtonGroup::multiple_rows(
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
                            ToggleButtonGroup::multiple_rows(
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
                            ToggleButtonGroup::multiple_rows(
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
                            ToggleButtonGroup::multiple_rows(
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
                            ToggleButtonGroup::multiple_rows(
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
                            ToggleButtonGroup::multiple_rows(
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
