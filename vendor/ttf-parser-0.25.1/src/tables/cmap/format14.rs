use crate::parser::{FromData, LazyArray32, Offset, Offset32, Stream, U24};
use crate::GlyphId;

#[derive(Clone, Copy)]
struct VariationSelectorRecord {
    var_selector: u32,
    default_uvs_offset: Option<Offset32>,
    non_default_uvs_offset: Option<Offset32>,
}

impl FromData for VariationSelectorRecord {
    const SIZE: usize = 11;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(VariationSelectorRecord {
            var_selector: s.read::<U24>()?.0,
            default_uvs_offset: s.read::<Option<Offset32>>()?,
            non_default_uvs_offset: s.read::<Option<Offset32>>()?,
        })
    }
}

#[derive(Clone, Copy)]
struct UVSMappingRecord {
    unicode_value: u32,
    glyph_id: GlyphId,
}

impl FromData for UVSMappingRecord {
    const SIZE: usize = 5;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(UVSMappingRecord {
            unicode_value: s.read::<U24>()?.0,
            glyph_id: s.read::<GlyphId>()?,
        })
    }
}

#[derive(Clone, Copy)]
struct UnicodeRangeRecord {
    start_unicode_value: u32,
    additional_count: u8,
}

impl UnicodeRangeRecord {
    fn contains(&self, c: u32) -> bool {
        // Never overflows, since `start_unicode_value` is actually u24.
        let end = self.start_unicode_value + u32::from(self.additional_count);
        (self.start_unicode_value..=end).contains(&c)
    }
}

impl FromData for UnicodeRangeRecord {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(UnicodeRangeRecord {
            start_unicode_value: s.read::<U24>()?.0,
            additional_count: s.read::<u8>()?,
        })
    }
}

/// A result of a variation glyph mapping.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GlyphVariationResult {
    /// Glyph was found in the variation encoding table.
    Found(GlyphId),
    /// Glyph should be looked in other, non-variation tables.
    ///
    /// Basically, you should use `Encoding::glyph_index` or `Face::glyph_index`
    /// in this case.
    UseDefault,
}

/// A [format 14](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
/// subtable.
#[derive(Clone, Copy)]
pub struct Subtable14<'a> {
    records: LazyArray32<'a, VariationSelectorRecord>,
    // The whole subtable data.
    data: &'a [u8],
}

impl<'a> Subtable14<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        s.skip::<u16>(); // format
        s.skip::<u32>(); // length
        let count = s.read::<u32>()?;
        let records = s.read_array32::<VariationSelectorRecord>(count)?;
        Some(Self { records, data })
    }

    /// Returns a glyph index for a code point.
    pub fn glyph_index(&self, code_point: u32, variation: u32) -> Option<GlyphVariationResult> {
        let (_, record) = self
            .records
            .binary_search_by(|v| v.var_selector.cmp(&variation))?;

        if let Some(offset) = record.default_uvs_offset {
            let data = self.data.get(offset.to_usize()..)?;
            let mut s = Stream::new(data);
            let count = s.read::<u32>()?;
            let ranges = s.read_array32::<UnicodeRangeRecord>(count)?;
            for range in ranges {
                if range.contains(code_point) {
                    return Some(GlyphVariationResult::UseDefault);
                }
            }
        }

        if let Some(offset) = record.non_default_uvs_offset {
            let data = self.data.get(offset.to_usize()..)?;
            let mut s = Stream::new(data);
            let count = s.read::<u32>()?;
            let uvs_mappings = s.read_array32::<UVSMappingRecord>(count)?;
            let (_, mapping) =
                uvs_mappings.binary_search_by(|v| v.unicode_value.cmp(&code_point))?;
            return Some(GlyphVariationResult::Found(mapping.glyph_id));
        }

        None
    }
}

impl core::fmt::Debug for Subtable14<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable14 {{ ... }}")
    }
}
