use gpui::{svg, App, IntoElement, Rems, RenderOnce, Size, Styled, Window};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};
use ui_macros::{path_str, DerivePathStr};

use crate::Color;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Serialize,
    Deserialize,
    DerivePathStr,
)]
#[strum(serialize_all = "snake_case")]
#[path_str(prefix = "images", suffix = ".svg")]
pub enum VectorName {
    ZedLogo,
    ZedXCopilot,
}

/// A vector image, such as an SVG.
///
/// A [`Vector`] is different from an [`crate::Icon`] in that it is intended
/// to be displayed at a specific size, or series of sizes, rather
/// than conforming to the standard size of an icon.
#[derive(IntoElement)]
pub struct Vector {
    path: &'static str,
    color: Color,
    size: Size<Rems>,
}

impl Vector {
    /// Creates a new [`Vector`] image with the given [`VectorName`] and size.
    pub fn new(vector: VectorName, width: Rems, height: Rems) -> Self {
        Self {
            path: vector.path(),
            color: Color::default(),
            size: Size { width, height },
        }
    }

    /// Creates a new [`Vector`] image where the width and height are the same.
    pub fn square(vector: VectorName, size: Rems) -> Self {
        Self::new(vector, size, size)
    }

    /// Sets the vector color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the vector size.
    pub fn size(mut self, size: impl Into<Size<Rems>>) -> Self {
        let size = size.into();

        self.size = size;
        self
    }
}

impl RenderOnce for Vector {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let width = self.size.width;
        let height = self.size.height;

        svg()
            // By default, prevent the SVG from stretching
            // to fill its container.
            .flex_none()
            .w(width)
            .h(height)
            .path(self.path)
            .text_color(self.color.color(cx))
    }
}

#[cfg(feature = "stories")]
pub mod story {
    use gpui::Render;
    use story::{Story, StoryItem, StorySection};
    use strum::IntoEnumIterator;

    use crate::prelude::*;

    use super::{Vector, VectorName};

    pub struct VectorStory;

    impl Render for VectorStory {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            Story::container().child(StorySection::new().children(VectorName::iter().map(
                |vector| StoryItem::new(format!("{:?}", vector), Vector::square(vector, rems(8.))),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_path() {
        assert_eq!(VectorName::ZedLogo.path(), "images/zed_logo.svg");
    }
}
