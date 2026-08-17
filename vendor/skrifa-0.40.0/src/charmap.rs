//! Mapping of characters (codepoints, not graphemes) to nominal glyph identifiers.
//!
//! If you have never run into character to glyph mapping before
//! [Glyph IDs and the 'cmap' table](https://rsheeter.github.io/font101/#glyph-ids-and-the-cmap-table)
//! might be informative.
//!
//! The functionality in this module provides a 1-to-1 mapping from Unicode
//! characters (or [Unicode variation sequences](http://unicode.org/faq/vs.html)) to
//! nominal or "default" internal glyph identifiers for a given font.
//! This is a necessary first step, but generally insufficient for proper layout of
//! [complex text](https://en.wikipedia.org/wiki/Complex_text_layout) or even
//! simple text containing diacritics and ligatures.
//!
//! Comprehensive mapping of characters to positioned glyphs requires a process called
//! shaping. For more detail, see: [Why do I need a shaping engine?](https://harfbuzz.github.io/why-do-i-need-a-shaping-engine.html)

use read_fonts::{
    tables::cmap::{
        self, Cmap, Cmap12, Cmap12Iter, Cmap14, Cmap14Iter, Cmap4, Cmap4Iter, CmapIterLimits,
        CmapSubtable, EncodingRecord, PlatformId,
    },
    types::GlyphId,
    FontData, FontRef, TableProvider,
};

pub use read_fonts::tables::cmap::MapVariant;

/// Mapping of characters to nominal glyph identifiers.
///
/// The mappings are derived from the [cmap](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap)
/// table. Depending on the font, the returned mapping may have entries that point to virtual/phantom glyph
/// ids beyond the `num_glyphs` entry of the `maxp` table, which are only used during the shaping process,
/// for example.
///
/// ## Obtaining a Charmap
///
/// Typically a Charmap is acquired by calling [charmap](crate::MetadataProvider::charmap) on a [FontRef].
///
/// ## Selection strategy
///
/// Fonts may contain multiple subtables in various formats supporting different encodings. The selection
/// strategy implemented here is designed to choose mappings that capture the broadest available Unicode
/// coverage:
///
/// * Unicode characters: a symbol mapping subtable is selected if available. Otherwise, subtables supporting
///   the Unicode full repertoire or Basic Multilingual Plane (BMP) are preferred, in that order. Formats
///   [4](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap#format-4-segment-mapping-to-delta-values)
///   and [12](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap#format-12-segmented-coverage) are
///   supported.
///
/// * Unicode variation sequences: these are provided by a format
///   [14](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
///   subtable.
///
#[derive(Clone, Default)]
pub struct Charmap<'a> {
    codepoint_subtable: Option<CodepointSubtable<'a>>,
    variant_subtable: Option<Cmap14<'a>>,
    cmap12_limits: CmapIterLimits,
}

impl<'a> Charmap<'a> {
    /// Creates a new character map from the given font.
    pub fn new(font: &FontRef<'a>) -> Self {
        let Ok(cmap) = font.cmap() else {
            return Default::default();
        };
        let selection = MappingSelection::new(font, &cmap);
        Self {
            codepoint_subtable: selection
                .codepoint_subtable
                .map(|subtable| CodepointSubtable {
                    subtable,
                    is_symbol: selection.mapping_index.codepoint_subtable_is_symbol,
                }),
            variant_subtable: selection.variant_subtable,
            cmap12_limits: selection.mapping_index.cmap12_limits,
        }
    }

    /// Returns true if a suitable Unicode character mapping is available.
    pub fn has_map(&self) -> bool {
        self.codepoint_subtable.is_some()
    }

    /// Returns true if a symbol mapping was selected.
    pub fn is_symbol(&self) -> bool {
        self.codepoint_subtable
            .as_ref()
            .map(|x| x.is_symbol)
            .unwrap_or(false)
    }

    /// Returns true if a Unicode variation sequence mapping is available.
    pub fn has_variant_map(&self) -> bool {
        self.variant_subtable.is_some()
    }

    /// Maps a character to a nominal glyph identifier.
    ///
    /// Returns `None` if a mapping does not exist.
    pub fn map(&self, ch: impl Into<u32>) -> Option<GlyphId> {
        self.codepoint_subtable.as_ref()?.map(ch.into())
    }

    /// Returns an iterator over all mappings of codepoint to nominal glyph
    /// identifiers in the character map.
    pub fn mappings(&self) -> Mappings<'a> {
        self.codepoint_subtable
            .as_ref()
            .map(|subtable| {
                Mappings(match &subtable.subtable {
                    SupportedSubtable::Format4(cmap4) => MappingsInner::Format4(cmap4.iter()),
                    SupportedSubtable::Format12(cmap12) => {
                        MappingsInner::Format12(cmap12.iter_with_limits(self.cmap12_limits))
                    }
                })
            })
            .unwrap_or(Mappings(MappingsInner::None))
    }

    /// Maps a character and variation selector to a nominal glyph identifier.
    ///
    /// Returns `None` if a mapping does not exist.
    pub fn map_variant(&self, ch: impl Into<u32>, selector: impl Into<u32>) -> Option<MapVariant> {
        self.variant_subtable.as_ref()?.map_variant(ch, selector)
    }

    /// Returns an iterator over all mappings of character and variation
    /// selector to nominal glyph identifier in the character map.
    pub fn variant_mappings(&self) -> VariantMappings<'a> {
        VariantMappings(self.variant_subtable.clone().map(|cmap14| cmap14.iter()))
    }
}

/// Cacheable indices of selected mapping tables for materializing a character
/// map.
///
/// Since [`Charmap`] carries a lifetime, it is difficult to store in a cache.
/// This type serves as an acceleration structure that allows for construction
/// of a character map while skipping the search for the most suitable Unicode
/// mappings.
#[derive(Copy, Clone, Default, Debug)]
pub struct MappingIndex {
    /// Index of Unicode or symbol mapping subtable.
    codepoint_subtable: Option<u16>,
    /// True if the above is a symbol mapping.
    codepoint_subtable_is_symbol: bool,
    /// Index of Unicode variation selector subtable.
    variant_subtable: Option<u16>,
    /// Limits for iterating a cmap format 12 subtable.
    cmap12_limits: CmapIterLimits,
}

impl MappingIndex {
    /// Finds the indices of the most suitable Unicode mapping tables in the
    /// given font.
    pub fn new(font: &FontRef) -> Self {
        let Ok(cmap) = font.cmap() else {
            return Default::default();
        };
        MappingSelection::new(font, &cmap).mapping_index
    }

    /// Creates a new character map for the given font using the tables referenced by
    /// the precomputed indices.
    ///
    /// The font should be the same as the one used to construct this object.
    pub fn charmap<'a>(&self, font: &FontRef<'a>) -> Charmap<'a> {
        let Ok(cmap) = font.cmap() else {
            return Default::default();
        };
        let records = cmap.encoding_records();
        let data = cmap.offset_data();
        Charmap {
            codepoint_subtable: self
                .codepoint_subtable
                .and_then(|index| get_subtable(data, records, index))
                .and_then(SupportedSubtable::new)
                .map(|subtable| CodepointSubtable {
                    subtable,
                    is_symbol: self.codepoint_subtable_is_symbol,
                }),
            variant_subtable: self
                .variant_subtable
                .and_then(|index| get_subtable(data, records, index))
                .and_then(|subtable| match subtable {
                    CmapSubtable::Format14(cmap14) => Some(cmap14),
                    _ => None,
                }),
            cmap12_limits: self.cmap12_limits,
        }
    }
}

/// Iterator over all mappings of character to nominal glyph identifier
/// in a character map.
///
/// This is created with the [`Charmap::mappings`] method.
#[derive(Clone)]
pub struct Mappings<'a>(MappingsInner<'a>);

impl Iterator for Mappings<'_> {
    type Item = (u32, GlyphId);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = match &mut self.0 {
                MappingsInner::None => None,
                MappingsInner::Format4(iter) => iter.next(),
                MappingsInner::Format12(iter) => iter.next(),
            }?;
            if item.1 != GlyphId::NOTDEF {
                return Some(item);
            }
        }
    }
}

#[derive(Clone)]
enum MappingsInner<'a> {
    None,
    Format4(Cmap4Iter<'a>),
    Format12(Cmap12Iter<'a>),
}

/// Iterator over all mappings of character and variation selector to
/// nominal glyph identifier in a character map.
///
/// This is created with the [`Charmap::variant_mappings`] method.
#[derive(Clone)]
pub struct VariantMappings<'a>(Option<Cmap14Iter<'a>>);

impl Iterator for VariantMappings<'_> {
    type Item = (u32, u32, MapVariant);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()?.next()
    }
}

fn get_subtable<'a>(
    data: FontData<'a>,
    records: &[EncodingRecord],
    index: u16,
) -> Option<CmapSubtable<'a>> {
    records
        .get(index as usize)
        .and_then(|record| record.subtable(data).ok())
}

#[derive(Clone)]
struct CodepointSubtable<'a> {
    subtable: SupportedSubtable<'a>,
    /// True if the subtable is a symbol mapping.
    is_symbol: bool,
}

impl CodepointSubtable<'_> {
    fn map(&self, codepoint: u32) -> Option<GlyphId> {
        self.map_impl(codepoint).or_else(|| {
            if self.is_symbol && codepoint <= 0x00FF {
                // From HarfBuzz:
                // For symbol-encoded OpenType fonts, we duplicate the
                // U+F000..F0FF range at U+0000..U+00FF.  That's what
                // Windows seems to do, and that's hinted about at:
                // https://docs.microsoft.com/en-us/typography/opentype/spec/recom
                // under "Non-Standard (Symbol) Fonts".
                // See <https://github.com/harfbuzz/harfbuzz/blob/453ded05392af38bba9f89587edce465e86ffa6b/src/hb-ot-cmap-table.hh#L1595>
                self.map_impl(codepoint + 0xF000)
            } else {
                None
            }
        })
    }

    fn map_impl(&self, codepoint: u32) -> Option<GlyphId> {
        let gid = match &self.subtable {
            SupportedSubtable::Format4(subtable) => subtable.map_codepoint(codepoint),
            SupportedSubtable::Format12(subtable) => subtable.map_codepoint(codepoint),
        }?;
        (gid != GlyphId::NOTDEF).then_some(gid)
    }
}

#[derive(Clone)]
enum SupportedSubtable<'a> {
    Format4(Cmap4<'a>),
    Format12(Cmap12<'a>),
}

impl<'a> SupportedSubtable<'a> {
    fn new(subtable: CmapSubtable<'a>) -> Option<Self> {
        Some(match subtable {
            CmapSubtable::Format4(cmap4) => Self::Format4(cmap4),
            CmapSubtable::Format12(cmap12) => Self::Format12(cmap12),
            _ => return None,
        })
    }

    fn from_cmap_record(cmap: &Cmap<'a>, record: &cmap::EncodingRecord) -> Option<Self> {
        Self::new(record.subtable(cmap.offset_data()).ok()?)
    }
}

/// The mapping kind of a cmap subtable.
///
/// The ordering is significant and determines the priority of subtable
/// selection (greater is better).
#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum MappingKind {
    None = 0,
    UnicodeBmp = 1,
    UnicodeFull = 2,
    Symbol = 3,
}

/// The result of searching the cmap table for the "best" available
/// subtables.
///
/// For `codepoint_subtable`, best means either symbol (which is preferred)
/// or a Unicode subtable with the greatest coverage.
///
/// For `variant_subtable`, best means a format 14 subtable.
struct MappingSelection<'a> {
    /// The mapping index accelerator that holds indices of the following
    /// subtables.
    mapping_index: MappingIndex,
    /// Either a symbol subtable or the Unicode subtable with the
    /// greatest coverage.
    codepoint_subtable: Option<SupportedSubtable<'a>>,
    /// Subtable that supports mapping Unicode variation sequences.
    variant_subtable: Option<Cmap14<'a>>,
}

impl<'a> MappingSelection<'a> {
    fn new(font: &FontRef<'a>, cmap: &Cmap<'a>) -> Self {
        const ENCODING_MS_SYMBOL: u16 = 0;
        const ENCODING_MS_UNICODE_CS: u16 = 1;
        const ENCODING_APPLE_ID_UNICODE_32: u16 = 4;
        const ENCODING_APPLE_ID_VARIANT_SELECTOR: u16 = 5;
        const ENCODING_MS_ID_UCS_4: u16 = 10;
        let mut mapping_index = MappingIndex::default();
        let mut mapping_kind = MappingKind::None;
        let mut codepoint_subtable = None;
        let mut variant_subtable = None;
        let mut maybe_choose_subtable = |kind, index, subtable| {
            if kind > mapping_kind {
                mapping_kind = kind;
                mapping_index.codepoint_subtable_is_symbol = kind == MappingKind::Symbol;
                mapping_index.codepoint_subtable = Some(index as u16);
                codepoint_subtable = Some(subtable);
            }
        };
        // This generally follows the same strategy as FreeType, searching the encoding
        // records in reverse and prioritizing UCS-4 subtables over UCS-2.
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/ac5babe87629107c43f627e2cd17c6cf4f2ecd43/src/base/ftobjs.c#L1370>
        // The exception is that we prefer a symbol subtable over all others which matches the behavior
        // of HarfBuzz.
        // See <https://github.com/harfbuzz/harfbuzz/blob/453ded05392af38bba9f89587edce465e86ffa6b/src/hb-ot-cmap-table.hh#L1818>
        for (i, record) in cmap.encoding_records().iter().enumerate().rev() {
            match (record.platform_id(), record.encoding_id()) {
                (PlatformId::Unicode, ENCODING_APPLE_ID_VARIANT_SELECTOR) => {
                    // Unicode variation sequences
                    if let Ok(CmapSubtable::Format14(subtable)) =
                        record.subtable(cmap.offset_data())
                    {
                        if variant_subtable.is_none() {
                            mapping_index.variant_subtable = Some(i as u16);
                            variant_subtable = Some(subtable);
                        }
                    }
                }
                (PlatformId::Windows, ENCODING_MS_SYMBOL) => {
                    // Symbol
                    if let Some(subtable) = SupportedSubtable::from_cmap_record(cmap, record) {
                        maybe_choose_subtable(MappingKind::Symbol, i, subtable);
                    }
                }
                (PlatformId::Windows, ENCODING_MS_ID_UCS_4)
                | (PlatformId::Unicode, ENCODING_APPLE_ID_UNICODE_32) => {
                    // Unicode full repertoire
                    if let Some(subtable) = SupportedSubtable::from_cmap_record(cmap, record) {
                        maybe_choose_subtable(MappingKind::UnicodeFull, i, subtable);
                    }
                }
                (PlatformId::ISO, _)
                | (PlatformId::Unicode, _)
                | (PlatformId::Windows, ENCODING_MS_UNICODE_CS) => {
                    // Unicode BMP only
                    if let Some(subtable) = SupportedSubtable::from_cmap_record(cmap, record) {
                        maybe_choose_subtable(MappingKind::UnicodeBmp, i, subtable);
                    }
                }
                _ => {}
            }
        }
        mapping_index.cmap12_limits = CmapIterLimits::default_for_font(font);
        Self {
            mapping_index,
            codepoint_subtable,
            variant_subtable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MetadataProvider;
    use read_fonts::FontRef;

    #[test]
    fn choose_format_12_over_4() {
        let font = FontRef::new(font_test_data::CMAP12_FONT1).unwrap();
        let charmap = font.charmap();
        assert!(matches!(
            charmap.codepoint_subtable.unwrap().subtable,
            SupportedSubtable::Format12(..)
        ));
    }

    #[test]
    fn choose_format_4() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let charmap = font.charmap();
        assert!(matches!(
            charmap.codepoint_subtable.unwrap().subtable,
            SupportedSubtable::Format4(..)
        ));
    }

    #[test]
    fn choose_symbol() {
        let font = FontRef::new(font_test_data::CMAP4_SYMBOL_PUA).unwrap();
        let charmap = font.charmap();
        assert!(charmap.is_symbol());
        assert!(matches!(
            charmap.codepoint_subtable.unwrap().subtable,
            SupportedSubtable::Format4(..)
        ));
    }

    #[test]
    fn map_format_4() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let charmap = font.charmap();
        assert_eq!(charmap.map('A'), Some(GlyphId::new(1)));
        assert_eq!(charmap.map('À'), Some(GlyphId::new(2)));
        assert_eq!(charmap.map('`'), Some(GlyphId::new(3)));
        assert_eq!(charmap.map('B'), None);
    }

    #[test]
    fn map_format_12() {
        let font = FontRef::new(font_test_data::CMAP12_FONT1).unwrap();
        let charmap = font.charmap();
        assert_eq!(charmap.map(' '), None);
        assert_eq!(charmap.map(0x101723_u32), Some(GlyphId::new(1)));
        assert_eq!(charmap.map(0x101725_u32), Some(GlyphId::new(3)));
        assert_eq!(charmap.map(0x102523_u32), Some(GlyphId::new(6)));
        assert_eq!(charmap.map(0x102526_u32), Some(GlyphId::new(9)));
        assert_eq!(charmap.map(0x102527_u32), Some(GlyphId::new(10)));
    }

    #[test]
    fn map_symbol_pua() {
        let font = FontRef::new(font_test_data::CMAP4_SYMBOL_PUA).unwrap();
        let charmap = font.charmap();
        assert!(charmap.codepoint_subtable.as_ref().unwrap().is_symbol);
        assert_eq!(charmap.map(0xF001_u32), Some(GlyphId::new(1)));
        assert_eq!(charmap.map(0xF002_u32), Some(GlyphId::new(2)));
        assert_eq!(charmap.map(0xF003_u32), Some(GlyphId::new(3)));
        assert_eq!(charmap.map(0xF0FE_u32), Some(GlyphId::new(4)));
        // The following don't exist in the cmap table and are remapped into the U+F000..F0FF range
        // due to the selection of a symbol mapping subtable.
        assert_eq!(charmap.map(0x1_u32), Some(GlyphId::new(1)));
        assert_eq!(charmap.map(0x2_u32), Some(GlyphId::new(2)));
        assert_eq!(charmap.map(0x3_u32), Some(GlyphId::new(3)));
        assert_eq!(charmap.map(0xFE_u32), Some(GlyphId::new(4)));
    }

    #[test]
    fn map_variants() {
        use super::MapVariant::*;
        let font = FontRef::new(font_test_data::CMAP14_FONT1).unwrap();
        let charmap = font.charmap();
        let selector = '\u{e0100}';
        assert_eq!(charmap.map_variant('a', selector), None);
        assert_eq!(charmap.map_variant('\u{4e00}', selector), Some(UseDefault));
        assert_eq!(charmap.map_variant('\u{4e06}', selector), Some(UseDefault));
        assert_eq!(
            charmap.map_variant('\u{4e08}', selector),
            Some(Variant(GlyphId::new(25)))
        );
        assert_eq!(
            charmap.map_variant('\u{4e09}', selector),
            Some(Variant(GlyphId::new(26)))
        );
    }

    #[test]
    fn mappings() {
        for font_data in [
            font_test_data::VAZIRMATN_VAR,
            font_test_data::CMAP12_FONT1,
            font_test_data::SIMPLE_GLYF,
            font_test_data::CMAP4_SYMBOL_PUA,
        ] {
            let font = FontRef::new(font_data).unwrap();
            let charmap = font.charmap();
            for (codepoint, glyph_id) in charmap.mappings() {
                assert_ne!(
                    glyph_id,
                    GlyphId::NOTDEF,
                    "we should never encounter notdef glyphs"
                );
                assert_eq!(charmap.map(codepoint), Some(glyph_id));
            }
        }
    }

    #[test]
    fn variant_mappings() {
        let font = FontRef::new(font_test_data::CMAP14_FONT1).unwrap();
        let charmap = font.charmap();
        for (codepoint, selector, variant) in charmap.variant_mappings() {
            assert_eq!(charmap.map_variant(codepoint, selector), Some(variant));
        }
    }
}
