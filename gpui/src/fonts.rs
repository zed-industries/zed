pub use font_kit::metrics::Metrics;
pub use font_kit::properties::{Properties, Weight};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontId(pub usize);

pub type GlyphId = u32;
