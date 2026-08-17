//! A robust, ergonomic, high performance crate for OpenType fonts.
//!  
//! Skrifa is a mid level library that provides access to various types
//! of [`metadata`](MetadataProvider) contained in a font as well as support
//! for loading glyph [`outlines`](outline).
//!
//! It is described as "mid level" because the library is designed to sit
//! above low level font parsing (provided by [`read-fonts`](https://crates.io/crates/read-fonts))
//! and below a higher level text layout engine.
//!
//! See the [readme](https://github.com/googlefonts/fontations/blob/main/skrifa/README.md)
//! for additional details.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(not(any(feature = "libm", feature = "std")))]
compile_error!("Either feature \"std\" or \"libm\" must be enabled for this crate.");

#[cfg(not(any(test, feature = "std")))]
#[macro_use]
extern crate core as std;

#[macro_use]
extern crate alloc;

/// Expose our "raw" underlying parser crate.
pub extern crate read_fonts as raw;

pub mod attribute;
pub mod bitmap;
pub mod charmap;
pub mod color;
pub mod font;
pub mod instance;
pub mod metrics;
pub mod outline;

pub mod setting;
pub mod string;

mod collections;
mod decycler;
mod glyph_name;
mod provider;
mod variation;

pub use glyph_name::{GlyphName, GlyphNameSource, GlyphNames};
#[doc(inline)]
pub use outline::{OutlineGlyph, OutlineGlyphCollection};
pub use variation::{Axis, AxisCollection, NamedInstance, NamedInstanceCollection};

/// Useful collection of common types suitable for glob importing.
pub mod prelude {
    #[doc(no_inline)]
    pub use super::{
        font::FontRef,
        instance::{LocationRef, NormalizedCoord, Size},
        GlyphId, MetadataProvider, Tag,
    };
}

pub use read_fonts::{
    types::{GlyphId, GlyphId16, Tag},
    FontRef,
};

#[doc(inline)]
pub use provider::MetadataProvider;

/// Limit for recursion when loading TrueType composite glyphs.
const GLYF_COMPOSITE_RECURSION_LIMIT: usize = 32;
