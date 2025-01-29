#![allow(missing_docs)]
use gpui::{pattern_horizontal_dash, pattern_vertical_dash, Background, Hsla, IntoElement};

use crate::prelude::*;

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

#[derive(IntoElement)]
pub struct Divider {
    direction: DividerDirection,
    color: DividerColor,
    inset: bool,
    is_dashed: bool,
}

impl RenderOnce for Divider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = self.color.hsla(cx);
        let background = if self.is_dashed {
            match self.direction {
                DividerDirection::Horizontal => pattern_horizontal_dash(color),
                DividerDirection::Vertical => pattern_vertical_dash(color),
            }
        } else {
            Background::from(color)
        };

        div()
            .map(|this| match self.direction {
                DividerDirection::Horizontal => {
                    this.h_px().w_full().when(self.inset, |this| this.mx_1p5())
                }
                DividerDirection::Vertical => {
                    this.w_px().h_full().when(self.inset, |this| this.my_1p5())
                }
            })
            .bg(background)
    }
}

impl Divider {
    pub fn horizontal() -> Self {
        Self {
            direction: DividerDirection::Horizontal,
            color: DividerColor::default(),
            inset: false,
            is_dashed: false,
        }
    }

    pub fn vertical() -> Self {
        Self {
            direction: DividerDirection::Vertical,
            color: DividerColor::default(),
            inset: false,
            is_dashed: false,
        }
    }

    pub fn horizontal_dashed() -> Self {
        Self {
            direction: DividerDirection::Horizontal,
            color: DividerColor::default(),
            inset: false,
            is_dashed: true,
        }
    }

    pub fn vertical_dashed() -> Self {
        Self {
            direction: DividerDirection::Vertical,
            color: DividerColor::default(),
            inset: false,
            is_dashed: true,
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
}
