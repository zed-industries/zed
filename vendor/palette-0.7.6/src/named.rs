//! A collection of named color constants. Can be toggled with the `"named"` and
//! `"named_from_str"` Cargo features.
//!
//! They are taken from the [SVG keyword
//! colors](https://www.w3.org/TR/SVG11/types.html#ColorKeywords) (same as in
//! CSS3) and they can be used as if they were pixel values:
//!
//! ```
//! use palette::Srgb;
//! use palette::named;
//!
//! //From constant
//! let from_const = Srgb::<f32>::from_format(named::OLIVE).into_linear();
#![cfg_attr(feature = "named_from_str", doc = "")]
#![cfg_attr(feature = "named_from_str", doc = "//From name string")]
#![cfg_attr(feature = "named_from_str", doc = "let olive = named::from_str(\"olive\").expect(\"unknown color\");")]
#![cfg_attr(feature = "named_from_str", doc = "let from_str = Srgb::<f32>::from_format(olive).into_linear();")]
#![cfg_attr(feature = "named_from_str", doc = "")]
#![cfg_attr(feature = "named_from_str", doc = "assert_eq!(from_const, from_str);")]
//! ```

include!(concat!(env!("OUT_DIR"), "/named.rs"));

/// Get a SVG/CSS3 color by name. Can be toggled with the `"named_from_str"`
/// Cargo feature.
///
/// The names are the same as the constants, but lower case.
#[cfg(feature = "named_from_str")]
pub fn from_str(name: &str) -> Option<crate::Srgb<u8>> {
    COLORS.get(name).cloned()
}
