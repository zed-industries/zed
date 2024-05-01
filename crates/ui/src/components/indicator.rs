use gpui::Transformation;

use crate::{prelude::*, AnyIcon};

#[derive(Default)]
pub enum IndicatorStyle {
    #[default]
    Dot,
    Bar,
    Icon(AnyIcon),
}

#[derive(IntoElement)]
pub struct Indicator {
    style: IndicatorStyle,
    pub color: Color,
}

impl Indicator {
    pub fn dot() -> Self {
        Self {
            style: IndicatorStyle::Dot,
            color: Color::Default,
        }
    }

    pub fn bar() -> Self {
        Self {
            style: IndicatorStyle::Dot,
            color: Color::Default,
        }
    }

    pub fn icon(icon: impl Into<AnyIcon>) -> Self {
        Self {
            style: IndicatorStyle::Icon(icon.into()),
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

        match self.style {
            IndicatorStyle::Icon(icon) => container
                .child(icon.map(|icon| icon.custom_size(rems_from_px(8.)).color(self.color))),
            IndicatorStyle::Dot => container
                .w_1p5()
                .h_1p5()
                .rounded_full()
                .bg(self.color.color(cx)),
            IndicatorStyle::Bar => container
                .w_full()
                .h_1p5()
                .rounded_t_md()
                .bg(self.color.color(cx)),
        }
    }
}

#[derive(IntoElement)]
pub struct IndicatorIcon {
    icon: Icon,
    transformation: Option<Transformation>,
}

impl IndicatorIcon {
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            transformation: None,
        }
    }

    pub fn transformation(mut self, transformation: Transformation) -> Self {
        self.transformation = Some(transformation);
        self
    }
}

impl RenderOnce for IndicatorIcon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.icon
            .custom_size(rems_from_px(8.))
            .when_some(self.transformation, |this, transformation| {
                this.transform(transformation)
            })
    }
}
