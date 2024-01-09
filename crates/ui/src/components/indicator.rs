use gpui::Position;

use crate::prelude::*;

#[derive(Default)]
pub enum IndicatorStyle {
    #[default]
    Dot,
    Bar,
}

#[derive(IntoElement)]
pub struct Indicator {
    position: Position,
    style: IndicatorStyle,
    color: Color,
}

impl Indicator {
    pub fn dot() -> Self {
        Self {
            position: Position::Relative,
            style: IndicatorStyle::Dot,
            color: Color::Default,
        }
    }

    pub fn bar() -> Self {
        Self {
            position: Position::Relative,
            style: IndicatorStyle::Dot,
            color: Color::Default,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn absolute(mut self) -> Self {
        self.position = Position::Absolute;
        self
    }
}

impl RenderOnce for Indicator {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex_none()
            .map(|this| match self.style {
                IndicatorStyle::Dot => this.w_1p5().h_1p5().rounded_full(),
                IndicatorStyle::Bar => this.w_full().h_1p5().rounded_t_md(),
            })
            .when(self.position == Position::Absolute, |this| this.absolute())
            .bg(self.color.color(cx))
    }
}
