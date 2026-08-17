use super::color::Color;
use super::font::{FontDesc, FontError, FontFamily, FontStyle, FontTransform};
use super::size::{HasDimension, SizeDesc};
use super::BLACK;
pub use plotters_backend::text_anchor;
use plotters_backend::{BackendColor, BackendCoord, BackendStyle, BackendTextStyle};

/// Style of a text
#[derive(Clone)]
pub struct TextStyle<'a> {
    /// The font description
    pub font: FontDesc<'a>,
    /// The text color
    pub color: BackendColor,
    /// The anchor point position
    pub pos: text_anchor::Pos,
}

/// Trait for values that can be converted into `TextStyle` values
pub trait IntoTextStyle<'a> {
    /** Converts the value into a TextStyle value.

    `parent` is used in some cases to convert a font size from points to pixels.

    # Example

    ```
    use plotters::prelude::*;
    let drawing_area = SVGBackend::new("into_text_style.svg", (200, 100)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let text_style = ("sans-serif", 20, &RED).into_text_style(&drawing_area);
    drawing_area.draw_text("This is a big red label", &text_style, (10, 50)).unwrap();
    ```

    The result is a text label styled accordingly:

    ![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@f030ed3/apidoc/into_text_style.svg)

    */
    fn into_text_style<P: HasDimension>(self, parent: &P) -> TextStyle<'a>;

    /** Specifies the color of the text element

    # Example

    ```
    use plotters::prelude::*;
    let drawing_area = SVGBackend::new("with_color.svg", (200, 100)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let text_style = ("sans-serif", 20).with_color(RED).into_text_style(&drawing_area);
    drawing_area.draw_text("This is a big red label", &text_style, (10, 50)).unwrap();
    ```

    The result is a text label styled accordingly:

    ![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@f030ed3/apidoc/with_color.svg)

    # See also

    [`FontDesc::color()`]

    [`IntoTextStyle::into_text_style()`] for a more succinct example

    */
    fn with_color<C: Color>(self, color: C) -> TextStyleBuilder<'a, Self>
    where
        Self: Sized,
    {
        TextStyleBuilder {
            base: self,
            new_color: Some(color.to_backend_color()),
            new_pos: None,
            _phatom: std::marker::PhantomData,
        }
    }

    /** Specifies the position of the text anchor relative to the text element

    # Example

    ```
    use plotters::{prelude::*,style::text_anchor::{HPos, Pos, VPos}};
    let anchor_position = (200,100);
    let anchor_left_bottom = Pos::new(HPos::Left, VPos::Bottom);
    let anchor_right_top = Pos::new(HPos::Right, VPos::Top);
    let drawing_area = SVGBackend::new("with_anchor.svg", (400, 200)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    drawing_area.draw(&Circle::new(anchor_position, 5, RED.filled()));
    let text_style_right_top = BLACK.with_anchor::<RGBColor>(anchor_right_top).into_text_style(&drawing_area);
    drawing_area.draw_text("The anchor sits at the right top of this label", &text_style_right_top, anchor_position);
    let text_style_left_bottom = BLACK.with_anchor::<RGBColor>(anchor_left_bottom).into_text_style(&drawing_area);
    drawing_area.draw_text("The anchor sits at the left bottom of this label", &text_style_left_bottom, anchor_position);
    ```

    The result has a red pixel at the center and two text labels positioned accordingly:

    ![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@b0b94d5/apidoc/with_anchor.svg)

    # See also

    [`TextStyle::pos()`]

    */
    fn with_anchor<C: Color>(self, pos: text_anchor::Pos) -> TextStyleBuilder<'a, Self>
    where
        Self: Sized,
    {
        TextStyleBuilder {
            base: self,
            new_pos: Some(pos),
            new_color: None,
            _phatom: std::marker::PhantomData,
        }
    }
}

pub struct TextStyleBuilder<'a, T: IntoTextStyle<'a>> {
    base: T,
    new_color: Option<BackendColor>,
    new_pos: Option<text_anchor::Pos>,
    _phatom: std::marker::PhantomData<&'a T>,
}

impl<'a, T: IntoTextStyle<'a>> IntoTextStyle<'a> for TextStyleBuilder<'a, T> {
    fn into_text_style<P: HasDimension>(self, parent: &P) -> TextStyle<'a> {
        let mut base = self.base.into_text_style(parent);
        if let Some(color) = self.new_color {
            base.color = color;
        }
        if let Some(pos) = self.new_pos {
            base = base.pos(pos);
        }
        base
    }
}

impl<'a> TextStyle<'a> {
    /// Sets the color of the style.
    ///
    /// - `color`: The required color
    /// - **returns** The up-to-dated text style
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let style = TextStyle::from(("sans-serif", 20).into_font()).color(&RED);
    /// ```
    pub fn color<C: Color>(&self, color: &'a C) -> Self {
        Self {
            font: self.font.clone(),
            color: color.to_backend_color(),
            pos: self.pos,
        }
    }

    /// Sets the font transformation of the style.
    ///
    /// - `trans`: The required transformation
    /// - **returns** The up-to-dated text style
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let style = TextStyle::from(("sans-serif", 20).into_font()).transform(FontTransform::Rotate90);
    /// ```
    pub fn transform(&self, trans: FontTransform) -> Self {
        Self {
            font: self.font.clone().transform(trans),
            color: self.color,
            pos: self.pos,
        }
    }

    /// Sets the anchor position.
    ///
    /// - `pos`: The required anchor position
    /// - **returns** The up-to-dated text style
    ///
    /// ```rust
    /// use plotters::prelude::*;
    /// use plotters::style::text_anchor::{Pos, HPos, VPos};
    ///
    /// let pos = Pos::new(HPos::Left, VPos::Top);
    /// let style = TextStyle::from(("sans-serif", 20).into_font()).pos(pos);
    /// ```
    ///
    /// # See also
    ///
    /// [`IntoTextStyle::with_anchor()`]
    pub fn pos(&self, pos: text_anchor::Pos) -> Self {
        Self {
            font: self.font.clone(),
            color: self.color,
            pos,
        }
    }
}

impl<'a> IntoTextStyle<'a> for FontDesc<'a> {
    fn into_text_style<P: HasDimension>(self, _: &P) -> TextStyle<'a> {
        self.into()
    }
}

impl<'a> IntoTextStyle<'a> for TextStyle<'a> {
    fn into_text_style<P: HasDimension>(self, _: &P) -> TextStyle<'a> {
        self
    }
}

impl<'a> IntoTextStyle<'a> for &'a str {
    fn into_text_style<P: HasDimension>(self, _: &P) -> TextStyle<'a> {
        self.into()
    }
}

impl<'a> IntoTextStyle<'a> for FontFamily<'a> {
    fn into_text_style<P: HasDimension>(self, _: &P) -> TextStyle<'a> {
        self.into()
    }
}

impl IntoTextStyle<'static> for u32 {
    fn into_text_style<P: HasDimension>(self, _: &P) -> TextStyle<'static> {
        TextStyle::from((FontFamily::SansSerif, self))
    }
}

impl IntoTextStyle<'static> for f64 {
    fn into_text_style<P: HasDimension>(self, _: &P) -> TextStyle<'static> {
        TextStyle::from((FontFamily::SansSerif, self))
    }
}

impl<'a, T: Color> IntoTextStyle<'a> for &'a T {
    fn into_text_style<P: HasDimension>(self, _: &P) -> TextStyle<'a> {
        TextStyle::from(FontFamily::SansSerif).color(self)
    }
}

impl<'a, F: Into<FontFamily<'a>>, T: SizeDesc> IntoTextStyle<'a> for (F, T) {
    fn into_text_style<P: HasDimension>(self, parent: &P) -> TextStyle<'a> {
        (self.0.into(), self.1.in_pixels(parent)).into()
    }
}

impl<'a, F: Into<FontFamily<'a>>, T: SizeDesc, C: Color> IntoTextStyle<'a> for (F, T, &'a C) {
    fn into_text_style<P: HasDimension>(self, parent: &P) -> TextStyle<'a> {
        IntoTextStyle::into_text_style((self.0, self.1), parent).color(self.2)
    }
}

impl<'a, F: Into<FontFamily<'a>>, T: SizeDesc> IntoTextStyle<'a> for (F, T, FontStyle) {
    fn into_text_style<P: HasDimension>(self, parent: &P) -> TextStyle<'a> {
        (self.0.into(), self.1.in_pixels(parent), self.2).into()
    }
}

impl<'a, F: Into<FontFamily<'a>>, T: SizeDesc, C: Color> IntoTextStyle<'a>
    for (F, T, FontStyle, &'a C)
{
    fn into_text_style<P: HasDimension>(self, parent: &P) -> TextStyle<'a> {
        IntoTextStyle::into_text_style((self.0, self.1, self.2), parent).color(self.3)
    }
}

/// Make sure that we are able to automatically copy the `TextStyle`
impl<'a, 'b: 'a> From<&'b TextStyle<'a>> for TextStyle<'a> {
    fn from(this: &'b TextStyle<'a>) -> Self {
        this.clone()
    }
}

impl<'a, T: Into<FontDesc<'a>>> From<T> for TextStyle<'a> {
    fn from(font: T) -> Self {
        Self {
            font: font.into(),
            color: BLACK.to_backend_color(),
            pos: text_anchor::Pos::default(),
        }
    }
}

impl<'a> BackendTextStyle for TextStyle<'a> {
    type FontError = FontError;
    fn color(&self) -> BackendColor {
        self.color
    }

    fn size(&self) -> f64 {
        self.font.get_size()
    }

    fn transform(&self) -> FontTransform {
        self.font.get_transform()
    }

    fn style(&self) -> FontStyle {
        self.font.get_style()
    }

    #[allow(clippy::type_complexity)]
    fn layout_box(&self, text: &str) -> Result<((i32, i32), (i32, i32)), Self::FontError> {
        self.font.layout_box(text)
    }

    fn anchor(&self) -> text_anchor::Pos {
        self.pos
    }

    fn family(&self) -> FontFamily {
        self.font.get_family()
    }

    fn draw<E, DrawFunc: FnMut(i32, i32, BackendColor) -> Result<(), E>>(
        &self,
        text: &str,
        pos: BackendCoord,
        mut draw: DrawFunc,
    ) -> Result<Result<(), E>, Self::FontError> {
        let color = self.color.color();
        self.font.draw(text, pos, move |x, y, a| {
            let mix_color = color.mix(a as f64);
            draw(x, y, mix_color)
        })
    }
}
