// https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-13-many-to-one-range-mappings

use core::convert::TryFrom;

use super::format12::SequentialMapGroup;
use crate::parser::{LazyArray32, Stream};
use crate::GlyphId;

/// A [format 13](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-13-segmented-coverage)
/// subtable.
#[derive(Clone, Copy)]
pub struct Subtable13<'a> {
    groups: LazyArray32<'a, SequentialMapGroup>,
}

impl<'a> Subtable13<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        s.skip::<u16>(); // format
        s.skip::<u16>(); // reserved
        s.skip::<u32>(); // length
        s.skip::<u32>(); // language
        let count = s.read::<u32>()?;
        let groups = s.read_array32::<super::format12::SequentialMapGroup>(count)?;
        Some(Self { groups })
    }

    /// Returns a glyph index for a code point.
    pub fn glyph_index(&self, code_point: u32) -> Option<GlyphId> {
        for group in self.groups {
            let start_char_code = group.start_char_code;
            if code_point >= start_char_code && code_point <= group.end_char_code {
                return u16::try_from(group.start_glyph_id).ok().map(GlyphId);
            }
        }

        None
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

impl core::fmt::Debug for Subtable13<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable13 {{ ... }}")
    }
}
