//! A [Glyph Definition Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/gdef) implementation.

use crate::opentype_layout::{Class, ClassDefinition, Coverage};
use crate::parser::{FromSlice, LazyArray16, Offset, Offset16, Offset32, Stream};
use crate::GlyphId;

#[cfg(feature = "variable-fonts")]
use crate::var_store::ItemVariationStore;
#[cfg(feature = "variable-fonts")]
use crate::NormalizedCoordinate;

/// A [glyph class](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#glyph-class-definition-table).
#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum GlyphClass {
    Base = 1,
    Ligature = 2,
    Mark = 3,
    Component = 4,
}

/// A [Glyph Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef).
#[allow(missing_debug_implementations)]
#[derive(Clone, Copy, Default)]
pub struct Table<'a> {
    glyph_classes: Option<ClassDefinition<'a>>,
    mark_attach_classes: Option<ClassDefinition<'a>>,
    mark_glyph_coverage_offsets: Option<(&'a [u8], LazyArray16<'a, Offset32>)>,
    #[cfg(feature = "variable-fonts")]
    variation_store: Option<ItemVariationStore<'a>>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let version = s.read::<u32>()?;
        if !(version == 0x00010000 || version == 0x00010002 || version == 0x00010003) {
            return None;
        }

        let glyph_class_def_offset = s.read::<Option<Offset16>>()?;
        s.skip::<Offset16>(); // attachListOffset
        s.skip::<Offset16>(); // ligCaretListOffset
        let mark_attach_class_def_offset = s.read::<Option<Offset16>>()?;

        let mut mark_glyph_sets_def_offset: Option<Offset16> = None;
        if version > 0x00010000 {
            mark_glyph_sets_def_offset = s.read::<Option<Offset16>>()?;
        }

        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut var_store_offset: Option<Offset32> = None;

        #[cfg(feature = "variable-fonts")]
        {
            if version > 0x00010002 {
                var_store_offset = s.read::<Option<Offset32>>()?;
            }
        }

        let mut table = Table::default();

        if let Some(offset) = glyph_class_def_offset {
            if let Some(subdata) = data.get(offset.to_usize()..) {
                table.glyph_classes = ClassDefinition::parse(subdata);
            }
        }

        if let Some(offset) = mark_attach_class_def_offset {
            if let Some(subdata) = data.get(offset.to_usize()..) {
                table.mark_attach_classes = ClassDefinition::parse(subdata);
            }
        }

        if let Some(offset) = mark_glyph_sets_def_offset {
            if let Some(subdata) = data.get(offset.to_usize()..) {
                let mut s = Stream::new(subdata);
                let format = s.read::<u16>()?;
                if format == 1 {
                    if let Some(count) = s.read::<u16>() {
                        if let Some(array) = s.read_array16::<Offset32>(count) {
                            table.mark_glyph_coverage_offsets = Some((subdata, array));
                        }
                    }
                }
            }
        }

        #[cfg(feature = "variable-fonts")]
        {
            if let Some(offset) = var_store_offset {
                if let Some(subdata) = data.get(offset.to_usize()..) {
                    let s = Stream::new(subdata);
                    table.variation_store = ItemVariationStore::parse(s);
                }
            }
        }

        Some(table)
    }

    /// Checks that face has
    /// [Glyph Class Definition Table](
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#glyph-class-definition-table).
    #[inline]
    pub fn has_glyph_classes(&self) -> bool {
        self.glyph_classes.is_some()
    }

    /// Returns glyph's class according to
    /// [Glyph Class Definition Table](
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#glyph-class-definition-table).
    ///
    /// Returns `None` when *Glyph Class Definition Table* is not set
    /// or glyph class is not set or invalid.
    #[inline]
    pub fn glyph_class(&self, glyph_id: GlyphId) -> Option<GlyphClass> {
        match self.glyph_classes?.get(glyph_id) {
            1 => Some(GlyphClass::Base),
            2 => Some(GlyphClass::Ligature),
            3 => Some(GlyphClass::Mark),
            4 => Some(GlyphClass::Component),
            _ => None,
        }
    }

    /// Returns glyph's mark attachment class according to
    /// [Mark Attachment Class Definition Table](
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-attachment-class-definition-table).
    ///
    /// All glyphs not assigned to a class fall into Class 0.
    #[inline]
    pub fn glyph_mark_attachment_class(&self, glyph_id: GlyphId) -> Class {
        self.mark_attach_classes
            .map(|def| def.get(glyph_id))
            .unwrap_or(0)
    }

    /// Checks that glyph is a mark according to
    /// [Mark Glyph Sets Table](
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-glyph-sets-table).
    ///
    /// `set_index` allows checking a specific glyph coverage set.
    /// Otherwise all sets will be checked.
    #[inline]
    pub fn is_mark_glyph(&self, glyph_id: GlyphId, set_index: Option<u16>) -> bool {
        is_mark_glyph_impl(self, glyph_id, set_index).is_some()
    }

    /// Returns glyph's variation delta at a specified index according to
    /// [Item Variation Store Table](
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#item-variation-store-table).
    #[cfg(feature = "variable-fonts")]
    #[inline]
    pub fn glyph_variation_delta(
        &self,
        outer_index: u16,
        inner_index: u16,
        coordinates: &[NormalizedCoordinate],
    ) -> Option<f32> {
        self.variation_store
            .and_then(|store| store.parse_delta(outer_index, inner_index, coordinates))
    }
}

#[inline(never)]
fn is_mark_glyph_impl(table: &Table, glyph_id: GlyphId, set_index: Option<u16>) -> Option<()> {
    let (data, offsets) = table.mark_glyph_coverage_offsets?;

    if let Some(set_index) = set_index {
        if let Some(offset) = offsets.get(set_index) {
            let table = Coverage::parse(data.get(offset.to_usize()..)?)?;
            if table.contains(glyph_id) {
                return Some(());
            }
        }
    } else {
        for offset in offsets {
            let table = Coverage::parse(data.get(offset.to_usize()..)?)?;
            if table.contains(glyph_id) {
                return Some(());
            }
        }
    }

    None
}
