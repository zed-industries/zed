use crate::prelude::*;
use gpui::{img, Div, Hsla, ImageData, ImageSource, Img, IntoElement, Styled};
use std::sync::Arc;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

#[derive(IntoElement)]
pub struct Avatar {
    image: Img,
    border_color: Option<Hsla>,
    is_available: Option<bool>,
}

impl RenderOnce for Avatar {
    type Rendered = Div;

    fn render(mut self, cx: &mut WindowContext) -> Self::Rendered {
        if self.image.style().corner_radii.top_left.is_none() {
            self = self.shape(Shape::Circle);
        }

        let size = cx.rem_size();

        div()
            .size(size + px(2.))
            .map(|mut div| {
                div.style().corner_radii = self.image.style().corner_radii.clone();
                div
            })
            .when_some(self.border_color, |this, color| {
                this.border().border_color(color)
            })
            .child(
                self.image
                    .size(size)
                    // todo!(Pull the avatar fallback background from the theme.)
                    .bg(gpui::red()),
            )
            .children(self.is_available.map(|is_free| {
                // HACK: non-integer sizes result in oval indicators.
                let indicator_size = (size * 0.4).round();

                div()
                    .absolute()
                    .z_index(1)
                    .bg(if is_free { gpui::green() } else { gpui::red() })
                    .size(indicator_size)
                    .rounded(indicator_size)
                    .bottom_0()
                    .right_0()
            }))
    }
}

impl Avatar {
    pub fn uri(src: impl Into<SharedString>) -> Self {
        Self::source(src.into().into())
    }

    pub fn data(src: Arc<ImageData>) -> Self {
        Self::source(src.into())
    }

    pub fn source(src: ImageSource) -> Self {
        Self {
            image: img(src),
            is_available: None,
            border_color: None,
        }
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.image = match shape {
            Shape::Circle => self.image.rounded_full(),
            Shape::RoundedRectangle => self.image.rounded_md(),
        };
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.image = self.image.grayscale(grayscale);
        self
    }

    pub fn border_color(mut self, color: impl Into<Hsla>) -> Self {
        self.border_color = Some(color.into());
        self
    }

    pub fn availability_indicator(mut self, is_available: impl Into<Option<bool>>) -> Self {
        self.is_available = is_available.into();
        self
    }
}
