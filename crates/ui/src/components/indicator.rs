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
    border_color: Option<Color>,
    pub color: Color,
}

impl Indicator {
    pub fn dot() -> Self {
        Self {
            kind: IndicatorKind::Dot,
            border_color: None,
            color: Color::Default,
        }
    }

    pub fn bar() -> Self {
        Self {
            kind: IndicatorKind::Bar,
            border_color: None,

            color: Color::Default,
        }
    }

    pub fn icon(icon: impl Into<AnyIcon>) -> Self {
        Self {
            kind: IndicatorKind::Icon(icon.into()),
            border_color: None,

            color: Color::Default,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl RenderOnce for Indicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let container = div().flex_none();
        let container = if let Some(border_color) = self.border_color {
            if matches!(self.kind, IndicatorKind::Dot | IndicatorKind::Bar) {
                container.border_1().border_color(border_color.color(cx))
            } else {
                container
            }
        } else {
            container
        };

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
