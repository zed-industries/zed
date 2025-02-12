use crate::{prelude::*, Indicator};

use gpui::{img, AnyElement, Hsla, ImageSource, Img, IntoElement, Styled};

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
#[derive(IntoElement, IntoComponent)]
pub struct Avatar {
    image: Img,
    size: Option<AbsoluteLength>,
    border_color: Option<Hsla>,
    indicator: Option<AnyElement>,
}

impl Avatar {
    /// Creates a new avatar element with the specified image source.
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Avatar {
            image: img(src),
            size: None,
            border_color: None,
            indicator: None,
        }
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

    /// Sets the border color of the avatar.
    ///
    /// This might be used to match the border to the background color of
    /// the parent element to create the illusion of cropping another
    /// shape underneath (for example in face piles.)
    pub fn border_color(mut self, color: impl Into<Hsla>) -> Self {
        self.border_color = Some(color.into());
        self
    }

    /// Size overrides the avatar size. By default they are 1rem.
    pub fn size<L: Into<AbsoluteLength>>(mut self, size: impl Into<Option<L>>) -> Self {
        self.size = size.into().map(Into::into);
        self
    }

    /// Sets the current indicator to be displayed on the avatar, if any.
    pub fn indicator<E: IntoElement>(mut self, indicator: impl Into<Option<E>>) -> Self {
        self.indicator = indicator.into().map(IntoElement::into_any_element);
        self
    }
}

impl RenderOnce for Avatar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border_width = if self.border_color.is_some() {
            px(2.)
        } else {
            px(0.)
        };

        let image_size = self.size.unwrap_or_else(|| rems(1.).into());
        let container_size = image_size.to_pixels(window.rem_size()) + border_width * 2.;

        div()
            .size(container_size)
            .rounded_full()
            .when_some(self.border_color, |this, color| {
                this.border(border_width).border_color(color)
            })
            .child(
                self.image
                    .size(image_size)
                    .rounded_full()
                    .bg(cx.theme().colors().ghost_element_background),
            )
            .children(self.indicator.map(|indicator| div().child(indicator)))
    }
}

// View this component preview using `workspace: open component-preview`
impl ComponentPreview for Avatar {
    fn preview(_window: &mut Window, _cx: &App) -> AnyElement {
        let example_avatar = "https://avatars.githubusercontent.com/u/1714999?v=4";

        v_flex()
            .gap_6()
            .children(vec![
                example_group_with_title(
                    "Sizes",
                    vec![
                        single_example(
                            "Default",
                            Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .into_any_element(),
                        ),
                        single_example(
                            "Small",
                            Avatar::new(example_avatar).size(px(24.)).into_any_element(),
                        ),
                        single_example(
                            "Large",
                            Avatar::new(example_avatar).size(px(48.)).into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "Styles",
                    vec![
                        single_example("Default", Avatar::new(example_avatar).into_any_element()),
                        single_example(
                            "Grayscale",
                            Avatar::new(example_avatar)
                                .grayscale(true)
                                .into_any_element(),
                        ),
                        single_example(
                            "With Border",
                            Avatar::new(example_avatar)
                                .border_color(gpui::red())
                                .into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "With Indicator",
                    vec![single_example(
                        "Dot",
                        Avatar::new(example_avatar)
                            .indicator(Indicator::dot().color(Color::Success))
                            .into_any_element(),
                    )],
                ),
            ])
            .into_any_element()
    }
}
