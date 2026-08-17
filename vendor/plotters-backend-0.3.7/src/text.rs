use super::{BackendColor, BackendCoord};
use std::error::Error;

/// Describes font family.
/// This can be either a specific font family name, such as "arial",
/// or a general font family class, such as "serif" and "sans-serif"
#[derive(Clone, Copy)]
pub enum FontFamily<'a> {
    /// The system default serif font family
    Serif,
    /// The system default sans-serif font family
    SansSerif,
    /// The system default monospace font
    Monospace,
    /// A specific font family name
    Name(&'a str),
}

impl<'a> FontFamily<'a> {
    /// Make a CSS compatible string for the font family name.
    /// This can be used as the value of `font-family` attribute in SVG.
    pub fn as_str(&self) -> &str {
        match self {
            FontFamily::Serif => "serif",
            FontFamily::SansSerif => "sans-serif",
            FontFamily::Monospace => "monospace",
            FontFamily::Name(face) => face,
        }
    }
}

impl<'a> From<&'a str> for FontFamily<'a> {
    fn from(from: &'a str) -> FontFamily<'a> {
        match from.to_lowercase().as_str() {
            "serif" => FontFamily::Serif,
            "sans-serif" => FontFamily::SansSerif,
            "monospace" => FontFamily::Monospace,
            _ => FontFamily::Name(from),
        }
    }
}

/// Text anchor attributes are used to properly position the text.
///
/// # Examples
///
/// In the example below, the text anchor (X) position is `Pos::new(HPos::Right, VPos::Center)`.
/// ```text
///    ***** X
/// ```
/// The position is always relative to the text regardless of its rotation.
/// In the example below, the text has style
/// `style.transform(FontTransform::Rotate90).pos(Pos::new(HPos::Center, VPos::Top))`.
/// ```text
///        *
///        *
///        * X
///        *
///        *
/// ```
pub mod text_anchor {
    /// The horizontal position of the anchor point relative to the text.
    #[derive(Clone, Copy, Default)]
    pub enum HPos {
        /// Anchor point is on the left side of the text
        #[default]
        Left,
        /// Anchor point is on the right side of the text
        Right,
        /// Anchor point is in the horizontal center of the text
        Center,
    }

    /// The vertical position of the anchor point relative to the text.
    #[derive(Clone, Copy, Default)]
    pub enum VPos {
        /// Anchor point is on the top of the text
        #[default]
        Top,
        /// Anchor point is in the vertical center of the text
        Center,
        /// Anchor point is on the bottom of the text
        Bottom,
    }

    /// The text anchor position.
    #[derive(Clone, Copy, Default)]
    pub struct Pos {
        /// The horizontal position of the anchor point
        pub h_pos: HPos,
        /// The vertical position of the anchor point
        pub v_pos: VPos,
    }

    impl Pos {
        /// Create a new text anchor position.
        ///
        /// - `h_pos`: The horizontal position of the anchor point
        /// - `v_pos`: The vertical position of the anchor point
        /// - **returns** The newly created text anchor position
        ///
        /// ```rust
        /// use plotters_backend::text_anchor::{Pos, HPos, VPos};
        ///
        /// let pos = Pos::new(HPos::Left, VPos::Top);
        /// ```
        pub fn new(h_pos: HPos, v_pos: VPos) -> Self {
            Pos { h_pos, v_pos }
        }
    }
}

/// Specifying text transformations
#[derive(Clone)]
pub enum FontTransform {
    /// Nothing to transform
    None,
    /// Rotating the text 90 degree clockwise
    Rotate90,
    /// Rotating the text 180 degree clockwise
    Rotate180,
    /// Rotating the text 270 degree clockwise
    Rotate270,
}

impl FontTransform {
    /// Transform the coordinate to perform the rotation
    ///
    /// - `x`: The x coordinate in pixels before transform
    /// - `y`: The y coordinate in pixels before transform
    /// - **returns**: The coordinate after transform
    pub fn transform(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            FontTransform::None => (x, y),
            FontTransform::Rotate90 => (-y, x),
            FontTransform::Rotate180 => (-x, -y),
            FontTransform::Rotate270 => (y, -x),
        }
    }
}

/// Describes the font style. Such as Italic, Oblique, etc.
#[derive(Clone, Copy)]
pub enum FontStyle {
    /// The normal style
    Normal,
    /// The oblique style
    Oblique,
    /// The italic style
    Italic,
    /// The bold style
    Bold,
}

impl FontStyle {
    /// Convert the font style into a CSS compatible string which can be used in `font-style` attribute.
    pub fn as_str(&self) -> &str {
        match self {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
            FontStyle::Oblique => "oblique",
            FontStyle::Bold => "bold",
        }
    }
}

impl<'a> From<&'a str> for FontStyle {
    fn from(from: &'a str) -> FontStyle {
        match from.to_lowercase().as_str() {
            "normal" => FontStyle::Normal,
            "italic" => FontStyle::Italic,
            "oblique" => FontStyle::Oblique,
            "bold" => FontStyle::Bold,
            _ => FontStyle::Normal,
        }
    }
}

/// The trait that abstracts a style of a text.
///
/// This is used because the the backend crate have no knowledge about how
/// the text handling is implemented in plotters.
///
/// But the backend still wants to know some information about the font, for
/// the backend doesn't handles text drawing, may want to call the `draw` method which
/// is implemented by the plotters main crate. While for the backend that handles the
/// text drawing, those font information provides instructions about how the text should be
/// rendered: color, size, slant, anchor, font, etc.
///
/// This trait decouples the detailed implementation about the font and the backend code which
/// wants to perform some operation on the font.
///
pub trait BackendTextStyle {
    /// The error type of this text style implementation
    type FontError: Error + Sync + Send + 'static;

    fn color(&self) -> BackendColor {
        BackendColor {
            alpha: 1.0,
            rgb: (0, 0, 0),
        }
    }

    fn size(&self) -> f64 {
        1.0
    }

    fn transform(&self) -> FontTransform {
        FontTransform::None
    }

    fn style(&self) -> FontStyle {
        FontStyle::Normal
    }

    fn anchor(&self) -> text_anchor::Pos {
        text_anchor::Pos::default()
    }

    fn family(&self) -> FontFamily;

    #[allow(clippy::type_complexity)]
    fn layout_box(&self, text: &str) -> Result<((i32, i32), (i32, i32)), Self::FontError>;

    fn draw<E, DrawFunc: FnMut(i32, i32, BackendColor) -> Result<(), E>>(
        &self,
        text: &str,
        pos: BackendCoord,
        draw: DrawFunc,
    ) -> Result<Result<(), E>, Self::FontError>;
}
