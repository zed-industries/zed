// This table has a pretty complex parsing algorithm.
// A detailed explanation can be found here:
// https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-2-high-byte-mapping-through-table
// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html
// https://github.com/fonttools/fonttools/blob/a360252709a3d65f899915db0a5bd753007fdbb7/Lib/fontTools/ttLib/tables/_c_m_a_p.py#L360

use core::convert::TryFrom;

use crate::parser::{FromData, LazyArray16, Stream};
use crate::GlyphId;

#[derive(Clone, Copy)]
struct SubHeaderRecord {
    first_code: u16,
    entry_count: u16,
    id_delta: i16,
    id_range_offset: u16,
}

impl FromData for SubHeaderRecord {
    const SIZE: usize = 8;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(SubHeaderRecord {
            first_code: s.read::<u16>()?,
            entry_count: s.read::<u16>()?,
            id_delta: s.read::<i16>()?,
            id_range_offset: s.read::<u16>()?,
        })
    }
}

/// A [format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-2-high-byte-mapping-through-table)
/// subtable.
#[derive(Clone, Copy)]
pub struct Subtable2<'a> {
    sub_header_keys: LazyArray16<'a, u16>,
    sub_headers_offset: usize,
    sub_headers: LazyArray16<'a, SubHeaderRecord>,
    // The whole subtable data.
    data: &'a [u8],
}

impl<'a> Subtable2<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        s.skip::<u16>(); // format
        s.skip::<u16>(); // length
        s.skip::<u16>(); // language
        let sub_header_keys = s.read_array16::<u16>(256)?;
        // The maximum index in a sub_header_keys is a sub_headers count.
        let sub_headers_count = sub_header_keys.into_iter().map(|n| n / 8).max()? + 1;

        // Remember sub_headers offset before reading. Will be used later.
        let sub_headers_offset = s.offset();
        let sub_headers = s.read_array16::<SubHeaderRecord>(sub_headers_count)?;

        Some(Self {
            sub_header_keys,
            sub_headers_offset,
            sub_headers,
            data,
        })
    }

    /// Returns a glyph index for a code point.
    ///
    /// Returns `None` when `code_point` is larger than `u16`.
    pub fn glyph_index(&self, code_point: u32) -> Option<GlyphId> {
        // This subtable supports code points only in a u16 range.
        let code_point = u16::try_from(code_point).ok()?;
        let high_byte = code_point >> 8;
        let low_byte = code_point & 0x00FF;

        let i = if code_point < 0xff {
            // 'SubHeader 0 is special: it is used for single-byte character codes.'
            0
        } else {
            // 'Array that maps high bytes to subHeaders: value is subHeader index × 8.'
            self.sub_header_keys.get(high_byte)? / 8
        };

        let sub_header = self.sub_headers.get(i)?;

        let first_code = sub_header.first_code;
        let range_end = first_code.checked_add(sub_header.entry_count)?;
        if low_byte < first_code || low_byte >= range_end {
            return None;
        }

        // SubHeaderRecord::id_range_offset points to SubHeaderRecord::first_code
        // in the glyphIndexArray. So we have to advance to our code point.
        let index_offset = usize::from(low_byte.checked_sub(first_code)?) * u16::SIZE;

        // 'The value of the idRangeOffset is the number of bytes
        // past the actual location of the idRangeOffset'.
        let offset = self.sub_headers_offset
                // Advance to required subheader.
                + SubHeaderRecord::SIZE * usize::from(i + 1)
                // Move back to idRangeOffset start.
                - u16::SIZE
                // Use defined offset.
                + usize::from(sub_header.id_range_offset)
                // Advance to required index in the glyphIndexArray.
                + index_offset;

        let glyph: u16 = Stream::read_at(self.data, offset)?;
        if glyph == 0 {
            return None;
        }

        u16::try_from((i32::from(glyph) + i32::from(sub_header.id_delta)) % 65536)
            .ok()
            .map(GlyphId)
    }

    /// Calls `f` for each codepoint defined in this table.
    pub fn codepoints(&self, f: impl FnMut(u32)) {
        let _ = self.codepoints_inner(f);
    }

    #[inline]
    fn codepoints_inner(&self, mut f: impl FnMut(u32)) -> Option<()> {
        for first_byte in 0u16..256 {
            let i = self.sub_header_keys.get(first_byte)? / 8;
            let sub_header = self.sub_headers.get(i)?;
            let first_code = sub_header.first_code;

            if i == 0 {
                // This is a single byte code.
                let range_end = first_code.checked_add(sub_header.entry_count)?;
                if first_byte >= first_code && first_byte < range_end {
                    f(u32::from(first_byte));
                }
            } else {
                // This is a two byte code.
                let base = first_code.checked_add(first_byte << 8)?;
                for k in 0..sub_header.entry_count {
                    let code_point = base.checked_add(k)?;
                    f(u32::from(code_point));
                }
            }
        }

        Some(())
    }
}

impl core::fmt::Debug for Subtable2<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable2 {{ ... }}")
    }
}
