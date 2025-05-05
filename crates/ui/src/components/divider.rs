use gpui::{Hsla, IntoElement};

use crate::prelude::*;

pub fn divider() -> Divider {
    Divider {
        style: DividerStyle::Solid,
        direction: DividerDirection::Horizontal,
        color: DividerColor::default(),
        inset: false,
    }
}

pub fn vertical_divider() -> Divider {
    Divider {
        style: DividerStyle::Solid,
        direction: DividerDirection::Vertical,
        color: DividerColor::default(),
        inset: false,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum DividerStyle {
    Solid,
    Dashed,
}

#[derive(Clone, Copy, PartialEq)]
enum DividerDirection {
    Horizontal,
    Vertical,
}

/// The color of a [`Divider`].
#[derive(Default)]
pub enum DividerColor {
    Border,
    #[default]
    BorderVariant,
}

impl DividerColor {
    pub fn hsla(self, cx: &mut App) -> Hsla {
        match self {
            DividerColor::Border => cx.theme().colors().border,
            DividerColor::BorderVariant => cx.theme().colors().border_variant,
        }
    }
}

#[derive(IntoElement, RegisterComponent)]
pub struct Divider {
    style: DividerStyle,
    direction: DividerDirection,
    color: DividerColor,
    inset: bool,
}

impl RenderOnce for Divider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.style {
            DividerStyle::Solid => self.render_solid(cx).into_any_element(),
            DividerStyle::Dashed => self.render_dashed(cx).into_any_element(),
        }
    }
}

impl Divider {
    pub fn horizontal() -> Self {
        Self {
            style: DividerStyle::Solid,
            direction: DividerDirection::Horizontal,
            color: DividerColor::default(),
            inset: false,
        }
    }

    pub fn vertical() -> Self {
        Self {
            style: DividerStyle::Solid,
            direction: DividerDirection::Vertical,
            color: DividerColor::default(),
            inset: false,
        }
    }

    pub fn horizontal_dashed() -> Self {
        Self {
            style: DividerStyle::Dashed,
            direction: DividerDirection::Horizontal,
            color: DividerColor::default(),
            inset: false,
        }
    }

    pub fn vertical_dashed() -> Self {
        Self {
            style: DividerStyle::Dashed,
            direction: DividerDirection::Vertical,
            color: DividerColor::default(),
            inset: false,
        }
    }

    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }

    pub fn color(mut self, color: DividerColor) -> Self {
        self.color = color;
        self
    }

    pub fn render_solid(self, cx: &mut App) -> impl IntoElement {
        div()
            .map(|this| match self.direction {
                DividerDirection::Horizontal => {
                    this.h_px().w_full().when(self.inset, |this| this.mx_1p5())
                }
                DividerDirection::Vertical => {
                    this.w_px().h_full().when(self.inset, |this| this.my_1p5())
                }
            })
            .bg(self.color.hsla(cx))
    }

    // TODO: Use canvas or a shader here
    // This obviously is a short term approach
    pub fn render_dashed(self, cx: &mut App) -> impl IntoElement {
        let segment_count = 128;
        let segment_count_f = segment_count as f32;
        let segment_min_w = 6.;
        let base = match self.direction {
            DividerDirection::Horizontal => h_flex(),
            DividerDirection::Vertical => v_flex(),
        };
        let (w, h) = match self.direction {
            DividerDirection::Horizontal => (px(segment_min_w), px(1.)),
            DividerDirection::Vertical => (px(1.), px(segment_min_w)),
        };
        let color = self.color.hsla(cx);
        let total_min_w = segment_min_w * segment_count_f * 2.; // * 2 because of the gap

        base.min_w(px(total_min_w))
            .map(|this| {
                if self.direction == DividerDirection::Horizontal {
                    this.w_full().h_px()
                } else {
                    this.w_px().h_full()
                }
            })
            .gap(px(segment_min_w))
            .overflow_hidden()
            .children(
                (0..segment_count).map(|_| div().flex_grow().flex_shrink_0().w(w).h(h).bg(color)),
            )
    }
}

impl Component for Divider {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn description() -> Option<&'static str> {
        Some(
            "Visual separator used to create divisions between groups of content or sections in a layout.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Horizontal Dividers",
                        vec![
                            single_example("Default", Divider::horizontal().into_any_element()),
                            single_example(
                                "Border Color",
                                Divider::horizontal()
                                    .color(DividerColor::Border)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Inset",
                                Divider::horizontal().inset().into_any_element(),
                            ),
                            single_example(
                                "Dashed",
                                Divider::horizontal_dashed().into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Vertical Dividers",
                        vec![
                            single_example(
                                "Default",
                                div().h_16().child(Divider::vertical()).into_any_element(),
                            ),
                            single_example(
                                "Border Color",
                                div()
                                    .h_16()
                                    .child(Divider::vertical().color(DividerColor::Border))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Inset",
                                div()
                                    .h_16()
                                    .child(Divider::vertical().inset())
                                    .into_any_element(),
                            ),
                            single_example(
                                "Dashed",
                                div()
                                    .h_16()
                                    .child(Divider::vertical_dashed())
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Example Usage",
                        vec![single_example(
                            "Between Content",
                            v_flex()
                                .gap_4()
                                .px_4()
                                .child(Label::new("Section One"))
                                .child(Divider::horizontal())
                                .child(Label::new("Section Two"))
                                .child(Divider::horizontal_dashed())
                                .child(Label::new("Section Three"))
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
