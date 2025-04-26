use gpui::{App, IntoElement, Rems, RenderOnce, Size, Styled, Window, svg};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};
use ui_macros::{DerivePathStr, path_str};

use crate::Color;
use crate::prelude::*;

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
#[derive(IntoElement, RegisterComponent)]
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

impl Component for Vector {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Images
    }

    fn name() -> &'static str {
        "Vector"
    }

    fn description() -> Option<&'static str> {
        Some("A vector image component that can be displayed at specific sizes.")
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), _window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
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
        assert_eq!(VectorName::ZedLogo.path(), "images/zed_logo.svg");
    }
}
