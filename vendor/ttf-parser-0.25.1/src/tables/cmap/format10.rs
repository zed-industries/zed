use crate::parser::{LazyArray32, Stream};
use crate::GlyphId;

/// A [format 10](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-10-trimmed-array)
/// subtable.
#[derive(Clone, Copy, Debug)]
pub struct Subtable10<'a> {
    /// First character code covered.
    pub first_code_point: u32,
    /// Array of glyph indices for the character codes covered.
    pub glyphs: LazyArray32<'a, GlyphId>,
}

impl<'a> Subtable10<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        s.skip::<u16>(); // format
        s.skip::<u16>(); // reserved
        s.skip::<u32>(); // length
        s.skip::<u32>(); // language
        let first_code_point = s.read::<u32>()?;
        let count = s.read::<u32>()?;
        let glyphs = s.read_array32::<GlyphId>(count)?;
        Some(Self {
            first_code_point,
            glyphs,
        })
    }

    /// Returns a glyph index for a code point.
    pub fn glyph_index(&self, code_point: u32) -> Option<GlyphId> {
        let idx = code_point.checked_sub(self.first_code_point)?;
        self.glyphs.get(idx)
    }

    /// Calls `f` for each codepoint defined in this table.
    pub fn codepoints(&self, mut f: impl FnMut(u32)) {
        for i in 0..self.glyphs.len() {
            if let Some(code_point) = self.first_code_point.checked_add(i) {
                f(code_point);
            }
        }
    }
}
