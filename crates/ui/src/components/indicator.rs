#![allow(missing_docs)]
use crate::{prelude::*, AnyIcon};

#[derive(Default)]
enum IndicatorKind {
    #[default]
    Dot,
    Bar,
    Icon(AnyIcon),
}

#[derive(IntoElement)]
pub struct Indicator {
    kind: IndicatorKind,
    pub color: Color,
}

impl Indicator {
    pub fn dot() -> Self {
        Self {
            kind: IndicatorKind::Dot,
            color: Color::Default,
        }
    }

    pub fn bar() -> Self {
        Self {
            kind: IndicatorKind::Bar,
            color: Color::Default,
        }
    }

    pub fn icon(icon: impl Into<AnyIcon>) -> Self {
        Self {
            kind: IndicatorKind::Icon(icon.into()),
            color: Color::Default,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Indicator {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let container = div().flex_none();

        match self.kind {
            IndicatorKind::Icon(icon) => container
                .child(icon.map(|icon| icon.custom_size(rems_from_px(8.)).color(self.color))),
            IndicatorKind::Dot => container
                .w_1p5()
                .h_1p5()
                .rounded_full()
                .bg(self.color.color(cx)),
            IndicatorKind::Bar => container
                .w_full()
                .h_1p5()
                .rounded_t_md()
                .bg(self.color.color(cx)),
        }
    }
}
