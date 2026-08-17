use core::convert::TryFrom;

use crate::parser::{FromData, LazyArray32, Stream};
use crate::GlyphId;

#[derive(Clone, Copy)]
pub struct SequentialMapGroup {
    pub start_char_code: u32,
    pub end_char_code: u32,
    pub start_glyph_id: u32,
}

impl FromData for SequentialMapGroup {
    const SIZE: usize = 12;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(SequentialMapGroup {
            start_char_code: s.read::<u32>()?,
            end_char_code: s.read::<u32>()?,
            start_glyph_id: s.read::<u32>()?,
        })
    }
}

/// A [format 12](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-12-segmented-coverage)
/// subtable.
#[derive(Clone, Copy)]
pub struct Subtable12<'a> {
    groups: LazyArray32<'a, SequentialMapGroup>,
}

impl<'a> Subtable12<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        s.skip::<u16>(); // format
        s.skip::<u16>(); // reserved
        s.skip::<u32>(); // length
        s.skip::<u32>(); // language
        let count = s.read::<u32>()?;
        let groups = s.read_array32::<SequentialMapGroup>(count)?;
        Some(Self { groups })
    }

    /// Returns a glyph index for a code point.
    pub fn glyph_index(&self, code_point: u32) -> Option<GlyphId> {
        let (_, group) = self.groups.binary_search_by(|range| {
            use core::cmp::Ordering;

            if range.start_char_code > code_point {
                Ordering::Greater
            } else if range.end_char_code < code_point {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })?;

        let id = group
            .start_glyph_id
            .checked_add(code_point)?
            .checked_sub(group.start_char_code)?;
        u16::try_from(id).ok().map(GlyphId)
    }

    /// Calls `f` for each codepoint defined in this table.
    pub fn codepoints(&self, mut f: impl FnMut(u32)) {
        for group in self.groups {
            for code_point in group.start_char_code..=group.end_char_code {
                f(code_point);
            }
        }
    }
}

impl core::fmt::Debug for Subtable12<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable12 {{ ... }}")
    }
}
