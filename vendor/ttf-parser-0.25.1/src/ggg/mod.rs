//! Common data types used by GDEF/GPOS/GSUB tables.
//!
//! <https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2>

// A heavily modified port of https://github.com/harfbuzz/rustybuzz implementation
// originally written by https://github.com/laurmaedje

use crate::parser::{FromData, FromSlice, LazyArray16, Stream};
use crate::GlyphId;

mod chained_context;
mod context;
#[cfg(feature = "variable-fonts")]
mod feature_variations;
mod layout_table;
mod lookup;

pub use chained_context::*;
pub use context::*;
#[cfg(feature = "variable-fonts")]
pub use feature_variations::*;
pub use layout_table::*;
pub use lookup::*;

/// A record that describes a range of glyph IDs.
#[derive(Clone, Copy, Debug)]
pub struct RangeRecord {
    /// First glyph ID in the range
    pub start: GlyphId,
    /// Last glyph ID in the range
    pub end: GlyphId,
    /// Coverage Index of first glyph ID in range.
    pub value: u16,
}

impl LazyArray16<'_, RangeRecord> {
    /// Returns a [`RangeRecord`] for a glyph.
    pub fn range(&self, glyph: GlyphId) -> Option<RangeRecord> {
        self.binary_search_by(|record| {
            if glyph < record.start {
                core::cmp::Ordering::Greater
            } else if glyph <= record.end {
                core::cmp::Ordering::Equal
            } else {
                core::cmp::Ordering::Less
            }
        })
        .map(|p| p.1)
    }
}

impl FromData for RangeRecord {
    const SIZE: usize = 6;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(RangeRecord {
            start: s.read::<GlyphId>()?,
            end: s.read::<GlyphId>()?,
            value: s.read::<u16>()?,
        })
    }
}

/// A [Coverage Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#coverage-table).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum Coverage<'a> {
    Format1 {
        /// Array of glyph IDs. Sorted.
        glyphs: LazyArray16<'a, GlyphId>,
    },
    Format2 {
        /// Array of glyph ranges. Ordered by `RangeRecord.start`.
        records: LazyArray16<'a, RangeRecord>,
    },
}

impl<'a> FromSlice<'a> for Coverage<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let count = s.read::<u16>()?;
                let glyphs = s.read_array16(count)?;
                Some(Self::Format1 { glyphs })
            }
            2 => {
                let count = s.read::<u16>()?;
                let records = s.read_array16(count)?;
                Some(Self::Format2 { records })
            }
            _ => None,
        }
    }
}

impl<'a> Coverage<'a> {
    /// Checks that glyph is present.
    pub fn contains(&self, glyph: GlyphId) -> bool {
        self.get(glyph).is_some()
    }

    /// Returns the coverage index of the glyph or `None` if it is not covered.
    pub fn get(&self, glyph: GlyphId) -> Option<u16> {
        match self {
            Self::Format1 { glyphs } => glyphs.binary_search(&glyph).map(|p| p.0),
            Self::Format2 { records } => {
                let record = records.range(glyph)?;
                let offset = glyph.0 - record.start.0;
                record.value.checked_add(offset)
            }
        }
    }
}

/// A value of [Class Definition Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table).
pub type Class = u16;

/// A [Class Definition Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum ClassDefinition<'a> {
    Format1 {
        start: GlyphId,
        classes: LazyArray16<'a, Class>,
    },
    Format2 {
        records: LazyArray16<'a, RangeRecord>,
    },
    Empty,
}

impl<'a> ClassDefinition<'a> {
    #[inline]
    pub(crate) fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let start = s.read::<GlyphId>()?;
                let count = s.read::<u16>()?;
                let classes = s.read_array16(count)?;
                Some(Self::Format1 { start, classes })
            }
            2 => {
                let count = s.read::<u16>()?;
                let records = s.read_array16(count)?;
                Some(Self::Format2 { records })
            }
            _ => None,
        }
    }

    /// Returns the glyph class of the glyph (zero if it is not defined).
    pub fn get(&self, glyph: GlyphId) -> Class {
        match self {
            Self::Format1 { start, classes } => glyph
                .0
                .checked_sub(start.0)
                .and_then(|index| classes.get(index)),
            Self::Format2 { records } => records.range(glyph).map(|record| record.value),
            Self::Empty => Some(0),
        }
        .unwrap_or(0)
    }
}
