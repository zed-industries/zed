use super::color::{Color, RGBAColor};
use plotters_backend::{BackendColor, BackendStyle};

/// Style for any shape
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShapeStyle {
    /// Specification of the color.
    pub color: RGBAColor,
    /// Whether the style is filled with color.
    pub filled: bool,
    /// Stroke width.
    pub stroke_width: u32,
}

impl ShapeStyle {
    /**
    Returns a filled style with the same color and stroke width.

    # Example

    ```
    use plotters::prelude::*;
    let original_style = ShapeStyle {
        color: BLUE.mix(0.6),
        filled: false,
        stroke_width: 2,
    };
    let filled_style = original_style.filled();
    let drawing_area = SVGBackend::new("shape_style_filled.svg", (400, 200)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    drawing_area.draw(&Circle::new((150, 100), 90, original_style));
    drawing_area.draw(&Circle::new((250, 100), 90, filled_style));
    ```

    The result is a figure with two circles, one of them filled:

    ![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@b0b94d5/apidoc/shape_style_filled.svg)
    */
    pub fn filled(&self) -> Self {
        Self {
            color: self.color.to_rgba(),
            filled: true,
            stroke_width: self.stroke_width,
        }
    }

    /**
    Returns a new style with the same color and the specified stroke width.

    # Example

    ```
    use plotters::prelude::*;
    let original_style = ShapeStyle {
        color: BLUE.mix(0.6),
        filled: false,
        stroke_width: 2,
    };
    let new_style = original_style.stroke_width(5);
    let drawing_area = SVGBackend::new("shape_style_stroke_width.svg", (400, 200)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    drawing_area.draw(&Circle::new((150, 100), 90, original_style));
    drawing_area.draw(&Circle::new((250, 100), 90, new_style));
    ```

    The result is a figure with two circles, one of them thicker than the other:

    ![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@b0b94d5/apidoc/shape_style_stroke_width.svg)
    */
    pub fn stroke_width(&self, width: u32) -> Self {
        Self {
            color: self.color.to_rgba(),
            filled: self.filled,
            stroke_width: width,
        }
    }
}

impl<T: Color> From<T> for ShapeStyle {
    fn from(f: T) -> Self {
        ShapeStyle {
            color: f.to_rgba(),
            filled: false,
            stroke_width: 1,
        }
    }
}

impl BackendStyle for ShapeStyle {
    /// Returns the color as interpreted by the backend.
    fn color(&self) -> BackendColor {
        self.color.to_backend_color()
    }
    /// Returns the stroke width.
    fn stroke_width(&self) -> u32 {
        self.stroke_width
    }
}
