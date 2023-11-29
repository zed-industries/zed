use gpui::{Div, IntoElement};

use crate::prelude::*;

enum DividerDirection {
    Horizontal,
    Vertical,
}

#[derive(IntoElement)]
pub struct Divider {
    direction: DividerDirection,
    inset: bool,
}

impl RenderOnce for Divider {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .map(|this| match self.direction {
                DividerDirection::Horizontal => {
                    this.h_px().w_full().when(self.inset, |this| this.mx_1p5())
                }
                DividerDirection::Vertical => {
                    this.w_px().h_full().when(self.inset, |this| this.my_1p5())
                }
            })
            .bg(cx.theme().colors().border_variant)
    }
}

impl Divider {
    pub fn horizontal() -> Self {
        Self {
            direction: DividerDirection::Horizontal,
            inset: false,
        }
    }

    pub fn vertical() -> Self {
        Self {
            direction: DividerDirection::Vertical,
            inset: false,
        }
    }

    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }
}
