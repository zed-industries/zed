use std::rc::Rc;

use gpui::{AnyView, ClickEvent, relative};

use crate::{ButtonLike, ButtonLikeRounding, TintColor, Tooltip, prelude::*};

/// The position of a [`ToggleButton`] within a group of buttons.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ToggleButtonPosition {
    /// The toggle button is one of the leftmost of the group.
    leftmost: bool,
    /// The toggle button is one of the rightmost of the group.
    rightmost: bool,
    /// The toggle button is one of the topmost of the group.
    topmost: bool,
    /// The toggle button is one of the bottommost of the group.
    bottommost: bool,
}

impl ToggleButtonPosition {
    pub const HORIZONTAL_FIRST: Self = Self {
        leftmost: true,
        ..Self::HORIZONTAL_MIDDLE
    };
    pub const HORIZONTAL_MIDDLE: Self = Self {
        leftmost: false,
        rightmost: false,
        topmost: true,
        bottommost: true,
    };
    pub const HORIZONTAL_LAST: Self = Self {
        rightmost: true,
        ..Self::HORIZONTAL_MIDDLE
    };

    pub(crate) fn to_rounding(self) -> ButtonLikeRounding {
        ButtonLikeRounding {
            top_left: self.topmost && self.leftmost,
            top_right: self.topmost && self.rightmost,
            bottom_right: self.bottommost && self.rightmost,
            bottom_left: self.bottommost && self.leftmost,
        }
    }
}

pub struct ButtonConfiguration {
    label: SharedString,
    icon: Option<IconName>,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    selected: bool,
    tooltip: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyView>>,
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
    tooltip: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyView>>,
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
            tooltip: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Rc::new(tooltip));
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
            tooltip: self.tooltip,
        }
    }
}

pub struct ToggleButtonWithIcon {
    label: SharedString,
    icon: IconName,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    selected: bool,
    tooltip: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyView>>,
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
            tooltip: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Rc::new(tooltip));
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
            tooltip: self.tooltip,
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
    Large,
    Custom(Rems),
}

#[derive(IntoElement)]
pub struct ToggleButtonGroup<T, const COLS: usize = 3, const ROWS: usize = 1>
where
    T: ButtonBuilder,
{
    group_name: SharedString,
    rows: [[T; COLS]; ROWS],
    style: ToggleButtonGroupStyle,
    size: ToggleButtonGroupSize,
    label_size: LabelSize,
    group_width: Option<DefiniteLength>,
    auto_width: bool,
    selected_index: usize,
    tab_index: Option<isize>,
}

impl<T: ButtonBuilder, const COLS: usize> ToggleButtonGroup<T, COLS> {
    pub fn single_row(group_name: impl Into<SharedString>, buttons: [T; COLS]) -> Self {
        Self {
            group_name: group_name.into(),
            rows: [buttons],
            style: ToggleButtonGroupStyle::Transparent,
            size: ToggleButtonGroupSize::Default,
            label_size: LabelSize::Small,
            group_width: None,
            auto_width: false,
            selected_index: 0,
            tab_index: None,
        }
    }
}

impl<T: ButtonBuilder, const COLS: usize> ToggleButtonGroup<T, COLS, 2> {
    pub fn two_rows(
        group_name: impl Into<SharedString>,
        first_row: [T; COLS],
        second_row: [T; COLS],
    ) -> Self {
        Self {
            group_name: group_name.into(),
            rows: [first_row, second_row],
            style: ToggleButtonGroupStyle::Transparent,
            size: ToggleButtonGroupSize::Default,
            label_size: LabelSize::Small,
            group_width: None,
            auto_width: false,
            selected_index: 0,
            tab_index: None,
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

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    /// Makes the button group size itself to fit the content of the buttons,
    /// rather than filling the full width of its parent.
    pub fn auto_width(mut self) -> Self {
        self.auto_width = true;
        self
    }

    pub fn label_size(mut self, label_size: LabelSize) -> Self {
        self.label_size = label_size;
        self
    }

    /// Sets the tab index for the toggle button group.
    /// The tab index is set to the initial value provided, then the
    /// value is incremented by the number of buttons in the group.
    pub fn tab_index(mut self, tab_index: &mut isize) -> Self {
        self.tab_index = Some(*tab_index);
        *tab_index += (COLS * ROWS) as isize;
        self
    }

    const fn button_width() -> DefiniteLength {
        relative(1. / COLS as f32)
    }
}

impl<T: ButtonBuilder, const COLS: usize, const ROWS: usize> FixedWidth
    for ToggleButtonGroup<T, COLS, ROWS>
{
    fn width(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.group_width = Some(width.into());
        self
    }

    fn full_width(mut self) -> Self {
        self.group_width = Some(relative(1.));
        self
    }
}

impl<T: ButtonBuilder, const COLS: usize, const ROWS: usize> RenderOnce
    for ToggleButtonGroup<T, COLS, ROWS>
{
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let custom_height = match self.size {
            ToggleButtonGroupSize::Custom(height) => Some(height),
            _ => None,
        };

        let entries =
            self.rows.into_iter().enumerate().map(|(row_index, row)| {
                let group_name = self.group_name.clone();
                row.into_iter().enumerate().map(move |(col_index, button)| {
                    let ButtonConfiguration {
                        label,
                        icon,
                        on_click,
                        selected,
                        tooltip,
                    } = button.into_configuration();

                    let entry_index = row_index * COLS + col_index;

                    ButtonLike::new((group_name.clone(), entry_index))
                        .when(!self.auto_width, |this| this.full_width())
                        .rounding(Some(
                            ToggleButtonPosition {
                                leftmost: col_index == 0,
                                rightmost: col_index == COLS - 1,
                                topmost: row_index == 0,
                                bottommost: row_index == ROWS - 1,
                            }
                            .to_rounding(),
                        ))
                        .when_some(self.tab_index, |this, tab_index| {
                            this.tab_index(tab_index + entry_index as isize)
                        })
                        .when(entry_index == self.selected_index || selected, |this| {
                            this.toggle_state(true)
                                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                        })
                        .when(self.style == ToggleButtonGroupStyle::Filled, |button| {
                            button.style(ButtonStyle::Filled)
                        })
                        .when(self.size == ToggleButtonGroupSize::Medium, |button| {
                            button.size(ButtonSize::Medium)
                        })
                        .when(self.size == ToggleButtonGroupSize::Large, |button| {
                            button.size(ButtonSize::Large)
                        })
                        .when_some(custom_height, |button, height| button.height(height.into()))
                        .child(
                            h_flex()
                                .w_full()
                                .px_2()
                                .gap_1p5()
                                .justify_center()
                                .flex_none()
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
                                .child(Label::new(label).size(self.label_size).when(
                                    entry_index == self.selected_index || selected,
                                    |this| this.color(Color::Accent),
                                )),
                        )
                        .when_some(tooltip, |this, tooltip| {
                            this.tooltip(move |window, cx| tooltip(window, cx))
                        })
                        .on_click(on_click)
                        .into_any_element()
                })
            });

        let border_color = cx.theme().colors().border.opacity(0.6);
        let is_outlined_or_filled = self.style == ToggleButtonGroupStyle::Outlined
            || self.style == ToggleButtonGroupStyle::Filled;
        let is_transparent = self.style == ToggleButtonGroupStyle::Transparent;

        v_flex()
            .map(|this| {
                if let Some(width) = self.group_width {
                    this.w(width)
                } else if self.auto_width {
                    this
                } else {
                    this.w_full()
                }
            })
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
                            .when(!self.auto_width, |this| this.w(Self::button_width()))
                            .overflow_hidden()
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
                            .width(rems_from_px(100.))
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
                            .width(rems_from_px(100.))
                            .style(ToggleButtonGroupStyle::Filled)
                            .into_any_element(),
                        ),
                    ],
                )])
                .children(vec![single_example(
                    "With Tooltips",
                    ToggleButtonGroup::single_row(
                        "with_tooltips",
                        [
                            ToggleButtonSimple::new("First", |_, _, _| {})
                                .tooltip(Tooltip::text("This is a tooltip. Hello!")),
                            ToggleButtonSimple::new("Second", |_, _, _| {})
                                .tooltip(Tooltip::text("This is a tooltip. Hey?")),
                            ToggleButtonSimple::new("Third", |_, _, _| {})
                                .tooltip(Tooltip::text("This is a tooltip. Get out of here now!")),
                        ],
                    )
                    .selected_index(1)
                    .into_any_element(),
                )])
                .into_any_element(),
        )
    }
}
