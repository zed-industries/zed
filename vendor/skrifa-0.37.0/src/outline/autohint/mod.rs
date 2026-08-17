//! Runtime autohinting support.

mod hint;
mod instance;
mod metrics;
mod outline;
mod shape;
mod style;
mod topo;

pub use instance::GlyphStyles;
pub(crate) use instance::Instance;

/// All constants are defined based on a UPEM of 2048.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.h#L34>
fn derived_constant(units_per_em: i32, value: i32) -> i32 {
    value * units_per_em / 2048
}
