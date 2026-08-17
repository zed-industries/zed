/*!
  The style for shapes and text, font, color, etc.
*/
mod color;
pub mod colors;
mod font;
mod palette;
mod shape;
mod size;
mod text;

/// Definitions of palettes of accessibility
pub use self::palette::*;
pub use color::{Color, HSLColor, PaletteColor, RGBAColor, RGBColor};
pub use colors::{BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, TRANSPARENT, WHITE, YELLOW};

#[cfg(feature = "full_palette")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "full_palette")))]
pub use colors::full_palette;

#[cfg(all(not(target_arch = "wasm32"), feature = "ab_glyph"))]
pub use font::register_font;
pub use font::{
    FontDesc, FontError, FontFamily, FontResult, FontStyle, FontTransform, IntoFont, LayoutBox,
};

pub use shape::ShapeStyle;
pub use size::{AsRelative, RelativeSize, SizeDesc};
pub use text::text_anchor;
pub use text::{IntoTextStyle, TextStyle};
