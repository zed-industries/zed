/// The implementation of an actual font implementation
///
/// This exists since for the image rendering task, we want to use
/// the system font. But in wasm application, we want the browser
/// to handle all the font issue.
///
/// Thus we need different mechanism for the font implementation

#[cfg(all(
    not(all(target_arch = "wasm32", not(target_os = "wasi"))),
    feature = "ttf"
))]
mod ttf;
#[cfg(all(
    not(all(target_arch = "wasm32", not(target_os = "wasi"))),
    feature = "ttf"
))]
use ttf::FontDataInternal;

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "wasi"),
    feature = "ab_glyph"
))]
mod ab_glyph;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "wasi"),
    feature = "ab_glyph"
))]
pub use self::ab_glyph::register_font;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "wasi"),
    feature = "ab_glyph",
    not(feature = "ttf")
))]
use self::ab_glyph::FontDataInternal;

#[cfg(all(
    not(all(target_arch = "wasm32", not(target_os = "wasi"))),
    not(feature = "ttf"),
    not(feature = "ab_glyph")
))]
mod naive;
#[cfg(all(
    not(all(target_arch = "wasm32", not(target_os = "wasi"))),
    not(feature = "ttf"),
    not(feature = "ab_glyph")
))]
use naive::FontDataInternal;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
mod web;
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
use web::FontDataInternal;

mod font_desc;
pub use font_desc::*;

/// Represents a box where a text label can be fit
pub type LayoutBox = ((i32, i32), (i32, i32));

pub trait FontData: Clone {
    type ErrorType: Sized + std::error::Error + Clone;
    fn new(family: FontFamily, style: FontStyle) -> Result<Self, Self::ErrorType>;
    fn estimate_layout(&self, size: f64, text: &str) -> Result<LayoutBox, Self::ErrorType>;
    fn draw<E, DrawFunc: FnMut(i32, i32, f32) -> Result<(), E>>(
        &self,
        _pos: (i32, i32),
        _size: f64,
        _text: &str,
        _draw: DrawFunc,
    ) -> Result<Result<(), E>, Self::ErrorType> {
        panic!("The font implementation is unable to draw text");
    }
}
