//! Common [scalar data types][data types] used in font files
//!
//! [data types]: https://docs.microsoft.com/en-us/typography/opentype/spec/otff#data-types

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::doc_markdown)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate core as std;

mod bbox;
mod fixed;
mod fword;
mod glyph_id;
mod int24;
mod longdatetime;
mod name_id;
mod offset;
mod point;
mod raw;
mod tag;
mod uint24;
mod version;

#[cfg(all(test, feature = "serde"))]
mod serde_test;

pub use bbox::BoundingBox;
pub use fixed::{F26Dot6, F2Dot14, F4Dot12, F6Dot10, Fixed};
pub use fword::{FWord, UfWord};
pub use glyph_id::{GlyphId, GlyphId16, TryFromGlyphIdError};
pub use int24::Int24;
pub use longdatetime::LongDateTime;
pub use name_id::NameId;
pub use offset::{Nullable, Offset16, Offset24, Offset32};
pub use point::Point;
pub use raw::{BigEndian, FixedSize, Scalar};
pub use tag::{InvalidTag, Tag};
pub use uint24::Uint24;
pub use version::{Compatible, MajorMinor, Version16Dot16};

/// The header tag for a font collection file.
pub const TTC_HEADER_TAG: Tag = Tag::new(b"ttcf");

/// The SFNT version for fonts containing TrueType outlines.
pub const TT_SFNT_VERSION: u32 = 0x00010000;
/// The SFNT version for legacy Apple fonts containing TrueType outlines.
pub const TRUE_SFNT_VERSION: u32 = 0x74727565;
/// The SFNT version for fonts containing CFF outlines.
pub const CFF_SFNT_VERSION: u32 = 0x4F54544F;
