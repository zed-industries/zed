use crate::prelude::*;

use gpui::{img, AnyElement, Hsla, ImageSource, Img, IntoElement, Styled};

/// The shape of an [`Avatar`].
#[derive(Debug, Default, PartialEq, Clone)]
pub enum AvatarShape {
    /// The avatar is shown in a circle.
    #[default]
    Circle,
    /// The avatar is shown in a rectangle with rounded corners.
    RoundedRectangle,
}

/// An element that renders a user avatar with customizable appearance options.
///
/// # Examples
///
/// ```
/// use ui::{Avatar, AvatarShape};
///
/// Avatar::new("path/to/image.png")
///     .shape(AvatarShape::Circle)
///     .grayscale(true)
///     .border_color(gpui::red());
/// ```
#[derive(IntoElement)]
pub struct Avatar {
    image: Img,
    size: Option<AbsoluteLength>,
    border_color: Option<Hsla>,
    indicator: Option<AnyElement>,
}

impl Avatar {
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Avatar {
            image: img(src),
            size: None,
            border_color: None,
            indicator: None,
        }
    }

    /// Sets the shape of the avatar image.
    ///
    /// This method allows the shape of the avatar to be specified using an [`AvatarShape`].
    /// It modifies the corner radius of the image to match the specified shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::{Avatar, AvatarShape};
    ///
    /// Avatar::new("path/to/image.png").shape(AvatarShape::Circle);
    /// ```
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.image = match shape {
            AvatarShape::Circle => self.image.rounded_full(),
            AvatarShape::RoundedRectangle => self.image.rounded_md(),
        };
        self
    }

    /// Applies a grayscale filter to the avatar image.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::{Avatar, AvatarShape};
    ///
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

    /// Size overrides the avatar size. By default they are 1rem.
    pub fn size<L: Into<AbsoluteLength>>(mut self, size: impl Into<Option<L>>) -> Self {
        self.size = size.into().map(Into::into);
        self
    }

    pub fn indicator<E: IntoElement>(mut self, indicator: impl Into<Option<E>>) -> Self {
        self.indicator = indicator.into().map(IntoElement::into_any_element);
        self
    }
}

impl RenderOnce for Avatar {
    fn render(mut self, cx: &mut WindowContext) -> impl IntoElement {
        if self.image.style().corner_radii.top_left.is_none() {
            self = self.shape(AvatarShape::Circle);
        }

        let border_width = if self.border_color.is_some() {
            px(2.)
        } else {
            px(0.)
        };

        let image_size = self.size.unwrap_or_else(|| rems(1.).into());
        let container_size = image_size.to_pixels(cx.rem_size()) + border_width * 2.;

        div()
            .size(container_size)
            .map(|mut div| {
                div.style().corner_radii = self.image.style().corner_radii.clone();
                div
            })
            .when_some(self.border_color, |this, color| {
                this.border_width(border_width).border_color(color)
            })
            .child(
                self.image
                    .size(image_size)
                    .bg(cx.theme().colors().ghost_element_background),
            )
            .children(self.indicator.map(|indicator| div().child(indicator)))
    }
}
