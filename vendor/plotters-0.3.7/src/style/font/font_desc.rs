use super::{FontData, FontDataInternal};
use crate::style::text_anchor::Pos;
use crate::style::{Color, TextStyle};

use std::convert::From;

pub use plotters_backend::{FontFamily, FontStyle, FontTransform};

/// The error type for the font implementation
pub type FontError = <FontDataInternal as FontData>::ErrorType;

/// The type we used to represent a result of any font operations
pub type FontResult<T> = Result<T, FontError>;

/// Describes a font
#[derive(Clone)]
pub struct FontDesc<'a> {
    size: f64,
    family: FontFamily<'a>,
    data: FontResult<FontDataInternal>,
    transform: FontTransform,
    style: FontStyle,
}

impl<'a> FontDesc<'a> {
    /// Create a new font
    ///
    /// - `family`: The font family name
    /// - `size`: The size of the font
    /// - `style`: The font variations
    /// - **returns** The newly created font description
    pub fn new(family: FontFamily<'a>, size: f64, style: FontStyle) -> Self {
        Self {
            size,
            family,
            data: FontDataInternal::new(family, style),
            transform: FontTransform::None,
            style,
        }
    }

    /// Create a new font desc with the same font but different size
    ///
    /// - `size`: The new size to set
    /// - **returns** The newly created font descriptor with a new size
    pub fn resize(&self, size: f64) -> Self {
        Self {
            size,
            family: self.family,
            data: self.data.clone(),
            transform: self.transform.clone(),
            style: self.style,
        }
    }

    /// Set the style of the font
    ///
    /// - `style`: The new style
    /// - **returns** The new font description with this style applied
    pub fn style(&self, style: FontStyle) -> Self {
        Self {
            size: self.size,
            family: self.family,
            data: self.data.clone(),
            transform: self.transform.clone(),
            style,
        }
    }

    /// Set the font transformation
    ///
    /// - `trans`: The new transformation
    /// - **returns** The new font description with this font transformation applied
    pub fn transform(&self, trans: FontTransform) -> Self {
        Self {
            size: self.size,
            family: self.family,
            data: self.data.clone(),
            transform: trans,
            style: self.style,
        }
    }

    /// Get the font transformation description
    pub fn get_transform(&self) -> FontTransform {
        self.transform.clone()
    }

    /** Returns a new text style object with the specified `color`.

    # Example

    ```
    use plotters::prelude::*;
    let text_style = ("sans-serif", 20).into_font().color(&RED);
    let drawing_area = SVGBackend::new("font_desc_color.svg", (200, 100)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    drawing_area.draw_text("This is a big red label", &text_style, (10, 50));
    ```

    The result is a text label colorized accordingly:

    ![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@f030ed3/apidoc/font_desc_color.svg)

    # See also

    [`IntoTextStyle::with_color()`](crate::style::IntoTextStyle::with_color)

    [`IntoTextStyle::into_text_style()`](crate::style::IntoTextStyle::into_text_style) for a more succinct example

    */
    pub fn color<C: Color>(&self, color: &C) -> TextStyle<'a> {
        TextStyle {
            font: self.clone(),
            color: color.to_backend_color(),
            pos: Pos::default(),
        }
    }

    /// Returns the font family
    pub fn get_family(&self) -> FontFamily {
        self.family
    }

    /// Get the name of the font
    pub fn get_name(&self) -> &str {
        self.family.as_str()
    }

    /// Get the name of the style
    pub fn get_style(&self) -> FontStyle {
        self.style
    }

    /// Get the size of font
    pub fn get_size(&self) -> f64 {
        self.size
    }

    /// Get the size of the text if rendered in this font
    ///
    /// For a TTF type, zero point of the layout box is the left most baseline char of the string
    /// Thus the upper bound of the box is most likely be negative
    pub fn layout_box(&self, text: &str) -> FontResult<((i32, i32), (i32, i32))> {
        match &self.data {
            Ok(ref font) => font.estimate_layout(self.size, text),
            Err(e) => Err(e.clone()),
        }
    }

    /// Get the size of the text if rendered in this font.
    /// This is similar to `layout_box` function, but it apply the font transformation
    /// and estimate the overall size of the font
    pub fn box_size(&self, text: &str) -> FontResult<(u32, u32)> {
        let ((min_x, min_y), (max_x, max_y)) = self.layout_box(text)?;
        let (w, h) = self.get_transform().transform(max_x - min_x, max_y - min_y);
        Ok((w.unsigned_abs(), h.unsigned_abs()))
    }

    /// Actually draws a font with a drawing function
    pub fn draw<E, DrawFunc: FnMut(i32, i32, f32) -> Result<(), E>>(
        &self,
        text: &str,
        (x, y): (i32, i32),
        draw: DrawFunc,
    ) -> FontResult<Result<(), E>> {
        match &self.data {
            Ok(ref font) => font.draw((x, y), self.size, text, draw),
            Err(e) => Err(e.clone()),
        }
    }
}

impl<'a> From<&'a str> for FontDesc<'a> {
    fn from(from: &'a str) -> FontDesc<'a> {
        FontDesc::new(from.into(), 12.0, FontStyle::Normal)
    }
}

impl<'a> From<FontFamily<'a>> for FontDesc<'a> {
    fn from(family: FontFamily<'a>) -> FontDesc<'a> {
        FontDesc::new(family, 12.0, FontStyle::Normal)
    }
}

impl<'a, T: Into<f64>> From<(FontFamily<'a>, T)> for FontDesc<'a> {
    fn from((family, size): (FontFamily<'a>, T)) -> FontDesc<'a> {
        FontDesc::new(family, size.into(), FontStyle::Normal)
    }
}

impl<'a, T: Into<f64>> From<(&'a str, T)> for FontDesc<'a> {
    fn from((typeface, size): (&'a str, T)) -> FontDesc<'a> {
        FontDesc::new(typeface.into(), size.into(), FontStyle::Normal)
    }
}

impl<'a, T: Into<f64>, S: Into<FontStyle>> From<(FontFamily<'a>, T, S)> for FontDesc<'a> {
    fn from((family, size, style): (FontFamily<'a>, T, S)) -> FontDesc<'a> {
        FontDesc::new(family, size.into(), style.into())
    }
}

impl<'a, T: Into<f64>, S: Into<FontStyle>> From<(&'a str, T, S)> for FontDesc<'a> {
    fn from((typeface, size, style): (&'a str, T, S)) -> FontDesc<'a> {
        FontDesc::new(typeface.into(), size.into(), style.into())
    }
}

/// The trait that allows some type turns into a font description
pub trait IntoFont<'a> {
    /// Make the font description from the source type
    fn into_font(self) -> FontDesc<'a>;
}

impl<'a, T: Into<FontDesc<'a>>> IntoFont<'a> for T {
    fn into_font(self) -> FontDesc<'a> {
        self.into()
    }
}
