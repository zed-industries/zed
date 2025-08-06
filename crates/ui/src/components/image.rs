use std::sync::Arc;

use gpui::Transformation;
use gpui::{App, IntoElement, Rems, RenderOnce, Size, Styled, Window, svg};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::Color;
use crate::prelude::*;

#[derive(
    Debug, PartialEq, Eq, Copy, Clone, EnumIter, EnumString, IntoStaticStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum VectorName {
    AiGrid,
    CertifiedUserStamp,
    DebuggerGrid,
    Grid,
    ProTrialStamp,
    ZedLogo,
    ZedXCopilot,
}

impl VectorName {
    /// Returns the path to this vector image.
    pub fn path(&self) -> Arc<str> {
        let file_stem: &'static str = self.into();
        format!("images/{file_stem}.svg").into()
    }
}

/// A vector image, such as an SVG.
///
/// A [`Vector`] is different from an [`crate::Icon`] in that it is intended
/// to be displayed at a specific size, or series of sizes, rather
/// than conforming to the standard size of an icon.
#[derive(IntoElement, RegisterComponent)]
pub struct Vector {
    path: Arc<str>,
    color: Color,
    size: Size<Rems>,
    transformation: Transformation,
}

impl Vector {
    /// Creates a new [`Vector`] image with the given [`VectorName`] and size.
    pub fn new(vector: VectorName, width: Rems, height: Rems) -> Self {
        Self {
            path: vector.path(),
            color: Color::default(),
            size: Size { width, height },
            transformation: Transformation::default(),
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

    pub fn transform(mut self, transformation: Transformation) -> Self {
        self.transformation = transformation;
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
            .with_transformation(self.transformation)
    }
}

impl Component for Vector {
    fn scope() -> ComponentScope {
        ComponentScope::Images
    }

    fn name() -> &'static str {
        "Vector"
    }

    fn description() -> Option<&'static str> {
        Some("A vector image component that can be displayed at specific sizes.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Usage",
                        vec![
                            single_example(
                                "Default",
                                Vector::square(VectorName::ZedLogo, rems(8.)).into_any_element(),
                            ),
                            single_example(
                                "Custom Size",
                                Vector::new(VectorName::ZedLogo, rems(12.), rems(6.))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Colored",
                        vec![
                            single_example(
                                "Accent Color",
                                Vector::square(VectorName::ZedLogo, rems(8.))
                                    .color(Color::Accent)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Error Color",
                                Vector::square(VectorName::ZedLogo, rems(8.))
                                    .color(Color::Error)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Different Vectors",
                        vec![
                            single_example(
                                "Zed Logo",
                                Vector::square(VectorName::ZedLogo, rems(8.)).into_any_element(),
                            ),
                            single_example(
                                "Zed X Copilot",
                                Vector::square(VectorName::ZedXCopilot, rems(8.))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_path() {
        assert_eq!(VectorName::ZedLogo.path().as_ref(), "images/zed_logo.svg");
    }
}
