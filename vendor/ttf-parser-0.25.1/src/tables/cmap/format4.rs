use core::convert::TryFrom;

use crate::parser::{LazyArray16, Stream};
use crate::GlyphId;

/// A [format 4](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-4-segment-mapping-to-delta-values)
/// subtable.
#[derive(Clone, Copy)]
pub struct Subtable4<'a> {
    start_codes: LazyArray16<'a, u16>,
    end_codes: LazyArray16<'a, u16>,
    id_deltas: LazyArray16<'a, i16>,
    id_range_offsets: LazyArray16<'a, u16>,
    id_range_offset_pos: usize,
    // The whole subtable data.
    data: &'a [u8],
}

impl<'a> Subtable4<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        s.advance(6); // format + length + language
        let seg_count_x2 = s.read::<u16>()?;
        if seg_count_x2 < 2 {
            return None;
        }

        let seg_count = seg_count_x2 / 2;
        s.advance(6); // searchRange + entrySelector + rangeShift

        let end_codes = s.read_array16::<u16>(seg_count)?;
        s.skip::<u16>(); // reservedPad
        let start_codes = s.read_array16::<u16>(seg_count)?;
        let id_deltas = s.read_array16::<i16>(seg_count)?;
        let id_range_offset_pos = s.offset();
        let id_range_offsets = s.read_array16::<u16>(seg_count)?;

        Some(Self {
            start_codes,
            end_codes,
            id_deltas,
            id_range_offsets,
            id_range_offset_pos,
            data,
        })
    }

    /// Returns a glyph index for a code point.
    ///
    /// Returns `None` when `code_point` is larger than `u16`.
    pub fn glyph_index(&self, code_point: u32) -> Option<GlyphId> {
        // This subtable supports code points only in a u16 range.
        let code_point = u16::try_from(code_point).ok()?;

        // A custom binary search.
        let mut start = 0;
        let mut end = self.start_codes.len();
        while end > start {
            let index = (start + end) / 2;
            let end_value = self.end_codes.get(index)?;
            if end_value >= code_point {
                let start_value = self.start_codes.get(index)?;
                if start_value > code_point {
                    end = index;
                } else {
                    let id_range_offset = self.id_range_offsets.get(index)?;
                    let id_delta = self.id_deltas.get(index)?;
                    if id_range_offset == 0 {
                        return Some(GlyphId(code_point.wrapping_add(id_delta as u16)));
                    } else if id_range_offset == 0xFFFF {
                        // Some malformed fonts have 0xFFFF as the last offset,
                        // which is invalid and should be ignored.
                        return None;
                    }

                    let delta = (u32::from(code_point) - u32::from(start_value)) * 2;
                    let delta = u16::try_from(delta).ok()?;

                    let id_range_offset_pos =
                        (self.id_range_offset_pos + usize::from(index) * 2) as u16;
                    let pos = id_range_offset_pos.wrapping_add(delta);
                    let pos = pos.wrapping_add(id_range_offset);

                    let glyph_array_value: u16 = Stream::read_at(self.data, usize::from(pos))?;

                    // 0 indicates missing glyph.
                    if glyph_array_value == 0 {
                        return None;
                    }

                    let glyph_id = (glyph_array_value as i16).wrapping_add(id_delta);
                    return u16::try_from(glyph_id).ok().map(GlyphId);
                }
            } else {
                start = index + 1;
            }
        }

        None
    }

    /// Calls `f` for each codepoint defined in this table.
    pub fn codepoints(&self, mut f: impl FnMut(u32)) {
        for (start, end) in self.start_codes.into_iter().zip(self.end_codes) {
            // OxFFFF value is special and indicates codes end.
            if start == end && start == 0xFFFF {
                break;
            }

            for code_point in start..=end {
                f(u32::from(code_point));
            }
        }
    }
}

impl core::fmt::Debug for Subtable4<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable4 {{ ... }}")
    }
}
