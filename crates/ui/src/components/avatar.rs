use crate::prelude::*;
use gpui::{img, Hsla, ImageSource, Img, IntoElement, Styled};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

/// An element that renders a user avatar with customizable appearance options.
///
/// # Examples
///
/// ```
/// Avatar::new("path/to/image.png")
///     .shape(Shape::Circle)
///     .grayscale(true)
///     .border_color(cx.theme().colors().border)
/// ```
#[derive(IntoElement)]
pub struct Avatar {
    image: Img,
    border_color: Option<Hsla>,
    is_available: Option<bool>,
}

impl RenderOnce for Avatar {
    fn render(mut self, cx: &mut WindowContext) -> impl IntoElement {
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
                    .bg(cx.theme().colors().ghost_element_background),
            )
            .children(self.is_available.map(|is_free| {
                // HACK: non-integer sizes result in oval indicators.
                let indicator_size = (size * 0.4).round();

                div()
                    .absolute()
                    .z_index(1)
                    .bg(if is_free {
                        cx.theme().status().created
                    } else {
                        cx.theme().status().deleted
                    })
                    .size(indicator_size)
                    .rounded(indicator_size)
                    .bottom_0()
                    .right_0()
            }))
    }
}

impl Avatar {
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Avatar {
            image: img(src),
            is_available: None,
            border_color: None,
        }
    }

    /// Sets the shape of the avatar image.
    ///
    /// This method allows the shape of the avatar to be specified using the [Shape] enum.
    /// It modifies the corner radius of the image to match the specified shape.
    ///
    /// # Examples
    ///
    /// ```
    /// Avatar::new("path/to/image.png").shape(Shape::Circle);
    /// ```
    pub fn shape(mut self, shape: Shape) -> Self {
        self.image = match shape {
            Shape::Circle => self.image.rounded_full(),
            Shape::RoundedRectangle => self.image.rounded_md(),
        };
        self
    }

    /// Applies a grayscale filter to the avatar image.
    ///
    /// # Examples
    ///
    /// ```
    /// let avatar = Avatar::new("path/to/image.png").grayscale(true);
    /// ```
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
