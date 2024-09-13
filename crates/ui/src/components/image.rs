use gpui::{svg, IntoElement, Rems, RenderOnce, SharedString, Size, Styled, WindowContext};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::Color;

const VECTOR_ASSETS_DIR: &str = "vectors";

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
    PathStaticStr,
)]
#[strum(serialize_all = "snake_case")]
#[derive_path_static_str(prefix = "vectors", suffix = "svg", delimiter = "/")]
pub enum VectorName {
    ZedLogo,
    ZedXCopilot,
}

impl VectorName {
    fn as_str(&self) -> &'static str {
        self.into()
    }

    pub fn path_str(&self) -> &'static str {
        path_str::<{ VECTOR_ASSETS_DIR }>(self)
    }
}

/// A vector image, such as an SVG.
///
/// A [Vector] is different from an [Icon] in that it is intended
/// to be displayed at a specific size, or series of sizes, rather
/// than conforming to the standard size of an icons.
#[derive(IntoElement)]
pub struct Vector {
    path: SharedString,
    color: Color,
    size: Size<Rems>,
}

impl Vector {
    /// Create a new [Vector] image with the given [VectorName] and size.
    pub fn new(vector: VectorName, width: Rems, height: Rems) -> Self {
        Self {
            path: vector.path(),
            color: Color::default(),
            size: Size { width, height },
        }
    }

    /// Create a new [Vector] image where the width and height are the same.
    pub fn square(vector: VectorName, size: Rems) -> Self {
        Self::new(vector, size, size)
    }

    /// Set the image color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the image size
    pub fn size(mut self, size: impl Into<Size<Rems>>) -> Self {
        let size = size.into();

        self.size = size;
        self
    }
}

impl RenderOnce for Vector {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
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
        fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
            Story::container().child(StorySection::new().children(
                // iter over all the segments
                VectorName::iter().map(|vector| {
                    StoryItem::new(format!("{:?}", vector), Vector::square(vector, rems(8.)))
                }),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn vector_name_path() {
        for vector_name in VectorName::iter() {
            let expected_path = format!("{}/{}.svg", VECTOR_ASSETS_DIR, vector_name.as_str());
            assert_eq!(vector_name.path(), expected_path);
        }
    }
}
