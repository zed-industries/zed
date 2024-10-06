#![allow(missing_docs)]
use gpui::{Hsla, IntoElement};

use crate::prelude::*;

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
    pub fn hsla(self, cx: &WindowContext) -> Hsla {
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
}

impl RenderOnce for Divider {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
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
}

impl Divider {
    pub fn horizontal() -> Self {
        Self {
            direction: DividerDirection::Horizontal,
            color: DividerColor::default(),
            inset: false,
        }
    }

    pub fn vertical() -> Self {
        Self {
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
}
