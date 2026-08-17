/*!
A [Character to Glyph Index Mapping Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cmap) implementation.

This module provides a low-level alternative to
[`Face::glyph_index`](../struct.Face.html#method.glyph_index) and
[`Face::glyph_variation_index`](../struct.Face.html#method.glyph_variation_index)
methods.
*/

use crate::parser::{FromData, LazyArray16, Offset, Offset32, Stream};
use crate::{name::PlatformId, GlyphId};

mod format0;
mod format10;
mod format12;
mod format13;
mod format14;
mod format2;
mod format4;
mod format6;

pub use format0::Subtable0;
pub use format10::Subtable10;
pub use format12::Subtable12;
pub use format13::Subtable13;
pub use format14::{GlyphVariationResult, Subtable14};
pub use format2::Subtable2;
pub use format4::Subtable4;
pub use format6::Subtable6;

/// A character encoding subtable variant.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum Format<'a> {
    ByteEncodingTable(Subtable0<'a>),
    HighByteMappingThroughTable(Subtable2<'a>),
    SegmentMappingToDeltaValues(Subtable4<'a>),
    TrimmedTableMapping(Subtable6<'a>),
    MixedCoverage, // unsupported
    TrimmedArray(Subtable10<'a>),
    SegmentedCoverage(Subtable12<'a>),
    ManyToOneRangeMappings(Subtable13<'a>),
    UnicodeVariationSequences(Subtable14<'a>),
}

/// A character encoding subtable.
#[derive(Clone, Copy, Debug)]
pub struct Subtable<'a> {
    /// Subtable platform.
    pub platform_id: PlatformId,
    /// Subtable encoding.
    pub encoding_id: u16,
    /// A subtable format.
    pub format: Format<'a>,
}

impl<'a> Subtable<'a> {
    /// Checks that the current encoding is Unicode compatible.
    #[inline]
    pub fn is_unicode(&self) -> bool {
        // https://docs.microsoft.com/en-us/typography/opentype/spec/name#windows-encoding-ids
        const WINDOWS_UNICODE_BMP_ENCODING_ID: u16 = 1;
        const WINDOWS_UNICODE_FULL_REPERTOIRE_ENCODING_ID: u16 = 10;

        match self.platform_id {
            PlatformId::Unicode => true,
            PlatformId::Windows if self.encoding_id == WINDOWS_UNICODE_BMP_ENCODING_ID => true,
            PlatformId::Windows => {
                // "Note: Subtable format 13 has the same structure as format 12; it differs only
                // in the interpretation of the startGlyphID/glyphID fields".
                let is_format_12_compatible = matches!(
                    self.format,
                    Format::SegmentedCoverage(..) | Format::ManyToOneRangeMappings(..)
                );

                // "Fonts that support Unicode supplementary-plane characters (U+10000 to U+10FFFF)
                // on the Windows platform must have a format 12 subtable for platform ID 3,
                // encoding ID 10."
                self.encoding_id == WINDOWS_UNICODE_FULL_REPERTOIRE_ENCODING_ID
                    && is_format_12_compatible
            }
            _ => false,
        }
    }

    /// Maps a character to a glyph ID.
    ///
    /// This is a low-level method and unlike `Face::glyph_index` it doesn't
    /// check that the current encoding is Unicode.
    /// It simply maps a `u32` codepoint number to a glyph ID.
    ///
    /// Returns `None`:
    /// - when glyph ID is `0`.
    /// - when format is `MixedCoverage`, since it's not supported.
    /// - when format is `UnicodeVariationSequences`. Use `glyph_variation_index` instead.
    #[inline]
    pub fn glyph_index(&self, code_point: u32) -> Option<GlyphId> {
        match self.format {
            Format::ByteEncodingTable(ref subtable) => subtable.glyph_index(code_point),
            Format::HighByteMappingThroughTable(ref subtable) => subtable.glyph_index(code_point),
            Format::SegmentMappingToDeltaValues(ref subtable) => subtable.glyph_index(code_point),
            Format::TrimmedTableMapping(ref subtable) => subtable.glyph_index(code_point),
            Format::MixedCoverage => None,
            Format::TrimmedArray(ref subtable) => subtable.glyph_index(code_point),
            Format::SegmentedCoverage(ref subtable) => subtable.glyph_index(code_point),
            Format::ManyToOneRangeMappings(ref subtable) => subtable.glyph_index(code_point),
            // This subtable should be accessed via glyph_variation_index().
            Format::UnicodeVariationSequences(_) => None,
        }
    }

    /// Resolves a variation of a glyph ID from two code points.
    ///
    /// Returns `None`:
    /// - when glyph ID is `0`.
    /// - when format is not `UnicodeVariationSequences`.
    #[inline]
    pub fn glyph_variation_index(
        &self,
        code_point: u32,
        variation: u32,
    ) -> Option<GlyphVariationResult> {
        match self.format {
            Format::UnicodeVariationSequences(ref subtable) => {
                subtable.glyph_index(code_point, variation)
            }
            _ => None,
        }
    }

    /// Calls `f` for all codepoints contained in this subtable.
    ///
    /// This is a low-level method and it doesn't check that the current
    /// encoding is Unicode. It simply calls the function `f` for all `u32`
    /// codepoints that are present in this subtable.
    ///
    /// Note that this may list codepoints for which `glyph_index` still returns
    /// `None` because this method finds all codepoints which were _defined_ in
    /// this subtable. The subtable may still map them to glyph ID `0`.
    ///
    /// Returns without doing anything:
    /// - when format is `MixedCoverage`, since it's not supported.
    /// - when format is `UnicodeVariationSequences`, since it's not supported.
    pub fn codepoints<F: FnMut(u32)>(&self, f: F) {
        match self.format {
            Format::ByteEncodingTable(ref subtable) => subtable.codepoints(f),
            Format::HighByteMappingThroughTable(ref subtable) => subtable.codepoints(f),
            Format::SegmentMappingToDeltaValues(ref subtable) => subtable.codepoints(f),
            Format::TrimmedTableMapping(ref subtable) => subtable.codepoints(f),
            Format::MixedCoverage => {} // unsupported
            Format::TrimmedArray(ref subtable) => subtable.codepoints(f),
            Format::SegmentedCoverage(ref subtable) => subtable.codepoints(f),
            Format::ManyToOneRangeMappings(ref subtable) => subtable.codepoints(f),
            Format::UnicodeVariationSequences(_) => {} // unsupported
        };
    }
}

#[derive(Clone, Copy)]
struct EncodingRecord {
    platform_id: PlatformId,
    encoding_id: u16,
    offset: Offset32,
}

impl FromData for EncodingRecord {
    const SIZE: usize = 8;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(EncodingRecord {
            platform_id: s.read::<PlatformId>()?,
            encoding_id: s.read::<u16>()?,
            offset: s.read::<Offset32>()?,
        })
    }
}

/// A list of subtables.
#[derive(Clone, Copy, Default)]
pub struct Subtables<'a> {
    data: &'a [u8],
    records: LazyArray16<'a, EncodingRecord>,
}

impl core::fmt::Debug for Subtables<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtables {{ ... }}")
    }
}

impl<'a> Subtables<'a> {
    /// Returns a subtable at an index.
    pub fn get(&self, index: u16) -> Option<Subtable<'a>> {
        let record = self.records.get(index)?;
        let data = self.data.get(record.offset.to_usize()..)?;
        let format = match Stream::read_at::<u16>(data, 0)? {
            0 => Format::ByteEncodingTable(Subtable0::parse(data)?),
            2 => Format::HighByteMappingThroughTable(Subtable2::parse(data)?),
            4 => Format::SegmentMappingToDeltaValues(Subtable4::parse(data)?),
            6 => Format::TrimmedTableMapping(Subtable6::parse(data)?),
            8 => Format::MixedCoverage, // unsupported
            10 => Format::TrimmedArray(Subtable10::parse(data)?),
            12 => Format::SegmentedCoverage(Subtable12::parse(data)?),
            13 => Format::ManyToOneRangeMappings(Subtable13::parse(data)?),
            14 => Format::UnicodeVariationSequences(Subtable14::parse(data)?),
            _ => return None,
        };

        Some(Subtable {
            platform_id: record.platform_id,
            encoding_id: record.encoding_id,
            format,
        })
    }

    /// Returns the number of subtables.
    #[inline]
    pub fn len(&self) -> u16 {
        self.records.len()
    }

    /// Checks if there are any subtables.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl<'a> IntoIterator for Subtables<'a> {
    type Item = Subtable<'a>;
    type IntoIter = SubtablesIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SubtablesIter {
            subtables: self,
            index: 0,
        }
    }
}

/// An iterator over [`Subtables`].
#[allow(missing_debug_implementations)]
pub struct SubtablesIter<'a> {
    subtables: Subtables<'a>,
    index: u16,
}

impl<'a> Iterator for SubtablesIter<'a> {
    type Item = Subtable<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.subtables.len() {
            self.index += 1;
            self.subtables.get(self.index - 1)
        } else {
            None
        }
    }
}

/// A [Character to Glyph Index Mapping Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/cmap).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// A list of subtables.
    pub subtables: Subtables<'a>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        s.skip::<u16>(); // version
        let count = s.read::<u16>()?;
        let records = s.read_array16::<EncodingRecord>(count)?;
        Some(Table {
            subtables: Subtables { data, records },
        })
    }
}
