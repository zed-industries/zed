//! The [morx (Extended Glyph Metamorphosis)](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html) table.

use super::aat::{safe_read_array_to_end, ExtendedStateTable, LookupU16};

include!("../../generated/generated_morx.rs");

impl VarSize for Chain<'_> {
    type Size = u32;

    fn read_len_at(data: FontData, pos: usize) -> Option<usize> {
        // Size in a chain is second field beyond 4 byte `defaultFlags`
        data.read_at::<u32>(pos.checked_add(u32::RAW_BYTE_LEN)?)
            .ok()
            .map(|size| size as usize)
    }
}

impl VarSize for Subtable<'_> {
    type Size = u32;

    fn read_len_at(data: FontData, pos: usize) -> Option<usize> {
        // The default implementation assumes that the length field itself
        // is not included in the total size which is not true of this
        // table.
        data.read_at::<u32>(pos).ok().map(|size| size as usize)
    }
}

impl<'a> Subtable<'a> {
    /// If true, this subtable will process glyphs in logical order (or reverse
    /// logical order, depending on the value of bit 0x80000000).
    #[inline]
    pub fn is_logical(&self) -> bool {
        self.coverage() & 0x10000000 != 0
    }

    /// If true, this subtable will be applied to both horizontal and vertical
    /// text (i.e. the state of bit 0x80000000 is ignored).
    #[inline]
    pub fn is_all_directions(&self) -> bool {
        self.coverage() & 0x20000000 != 0
    }

    /// If true, this subtable will process glyphs in descending order.
    /// Otherwise, it will process the glyphs in ascending order.
    #[inline]
    pub fn is_backwards(&self) -> bool {
        self.coverage() & 0x40000000 != 0
    }

    /// If true, this subtable will only be applied to vertical text.
    /// Otherwise, this subtable will only be applied to horizontal
    /// text.
    #[inline]
    pub fn is_vertical(&self) -> bool {
        self.coverage() & 0x80000000 != 0
    }

    /// Returns an enum representing the actual subtable data.
    pub fn kind(&self) -> Result<SubtableKind<'a>, ReadError> {
        SubtableKind::read_with_args(FontData::new(self.data()), &self.coverage())
    }
}

/// The various `morx` subtable formats.
#[derive(Clone)]
pub enum SubtableKind<'a> {
    Rearrangement(ExtendedStateTable<'a>),
    Contextual(ContextualSubtable<'a>),
    Ligature(LigatureSubtable<'a>),
    NonContextual(LookupU16<'a>),
    Insertion(InsertionSubtable<'a>),
}

impl ReadArgs for SubtableKind<'_> {
    type Args = u32;
}

impl<'a> FontReadWithArgs<'a> for SubtableKind<'a> {
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        // Format is low byte of coverage
        let format = *args & 0xFF;
        match format {
            0 => Ok(Self::Rearrangement(ExtendedStateTable::read(data)?)),
            1 => Ok(Self::Contextual(ContextualSubtable::read(data)?)),
            2 => Ok(Self::Ligature(LigatureSubtable::read(data)?)),
            // 3 is reserved
            4 => Ok(Self::NonContextual(LookupU16::read(data)?)),
            5 => Ok(Self::Insertion(InsertionSubtable::read(data)?)),
            _ => Err(ReadError::InvalidFormat(format as _)),
        }
    }
}

/// Contextual glyph substitution subtable.
#[derive(Clone)]
pub struct ContextualSubtable<'a> {
    pub state_table: ExtendedStateTable<'a, ContextualEntryData>,
    /// List of lookups specifying substitutions. The index into this array
    /// is specified by the action in the state table.
    pub lookups: ArrayOfOffsets<'a, LookupU16<'a>, Offset32>,
}

impl<'a> FontRead<'a> for ContextualSubtable<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let state_table = ExtendedStateTable::read(data)?;
        let mut cursor = data.cursor();
        cursor.advance_by(ExtendedStateTable::<()>::HEADER_LEN);
        let offset = cursor.read::<u32>()? as usize;
        let end = data.len();
        let offsets_data = FontData::new(data.read_array(offset..end)?);
        let raw_offsets: &[BigEndian<Offset32>] = safe_read_array_to_end(&offsets_data, 0)?;
        let lookups = ArrayOfOffsets::new(raw_offsets, offsets_data, ());
        Ok(Self {
            state_table,
            lookups,
        })
    }
}

/// Ligature glyph substitution subtable.
#[derive(Clone)]
pub struct LigatureSubtable<'a> {
    pub state_table: ExtendedStateTable<'a, BigEndian<u16>>,
    /// Contains the set of ligature stack actions, one for each state.
    pub ligature_actions: &'a [BigEndian<u32>],
    /// Array of component indices which are summed to determine the index
    /// of the final ligature glyph.
    pub components: &'a [BigEndian<u16>],
    /// Output ligature glyphs.
    pub ligatures: &'a [BigEndian<GlyphId16>],
}

impl<'a> FontRead<'a> for LigatureSubtable<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let state_table = ExtendedStateTable::read(data)?;
        let mut cursor = data.cursor();
        cursor.advance_by(ExtendedStateTable::<()>::HEADER_LEN);
        // None of these arrays have associated sizes, so we just read until
        // the end of the data.
        let lig_action_offset = cursor.read::<u32>()? as usize;
        let component_offset = cursor.read::<u32>()? as usize;
        let ligature_offset = cursor.read::<u32>()? as usize;
        let ligature_actions = safe_read_array_to_end(&data, lig_action_offset)?;
        let components = safe_read_array_to_end(&data, component_offset)?;
        let ligatures = safe_read_array_to_end(&data, ligature_offset)?;
        Ok(Self {
            state_table,
            ligature_actions,
            components,
            ligatures,
        })
    }
}

/// Insertion glyph substitution subtable.
#[derive(Clone)]
pub struct InsertionSubtable<'a> {
    pub state_table: ExtendedStateTable<'a, InsertionEntryData>,
    /// Insertion glyph table. The index and count of glyphs to insert is
    /// determined by the state machine.
    pub glyphs: &'a [BigEndian<GlyphId16>],
}

impl<'a> FontRead<'a> for InsertionSubtable<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let state_table = ExtendedStateTable::read(data)?;
        let mut cursor = data.cursor();
        cursor.advance_by(ExtendedStateTable::<()>::HEADER_LEN);
        let glyphs_offset = cursor.read::<u32>()? as usize;
        let glyphs = safe_read_array_to_end(&data, glyphs_offset)?;
        Ok(Self {
            state_table,
            glyphs,
        })
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for Chain<'a> {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "Chain",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("default_flags", self.default_flags())),
                _ => None,
            }),
            data,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for Subtable<'a> {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "Subtable",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("coverage", self.coverage())),
                1usize => Some(Field::new("sub_feature_flags", self.sub_feature_flags())),
                _ => None,
            }),
            data,
        }
    }
}

#[cfg(test)]
// Literal bytes are grouped according to layout in the spec
// for readabiity
#[allow(clippy::unusual_byte_groupings)]
mod tests {
    use super::*;
    use crate::{FontRef, TableProvider};

    #[test]
    fn parse_chain_flags_features() {
        let font = FontRef::new(font_test_data::morx::FOUR).unwrap();
        let morx = font.morx().unwrap();
        let chain = morx.chains().iter().next().unwrap().unwrap();
        assert_eq!(chain.default_flags(), 1);
        let feature = chain.features()[0];
        assert_eq!(feature.feature_type(), 4);
        assert_eq!(feature.feature_settings(), 0);
        assert_eq!(feature.enable_flags(), 1);
        assert_eq!(feature.disable_flags(), 0xFFFFFFFF);
    }

    #[test]
    fn parse_rearrangement() {
        let font = FontRef::new(font_test_data::morx::FOUR).unwrap();
        let morx = font.morx().unwrap();
        let chain = morx.chains().iter().next().unwrap().unwrap();
        let subtable = chain.subtables().iter().next().unwrap().unwrap();
        assert_eq!(subtable.coverage(), 0x20_0000_00);
        // Rearrangement is just a state table
        let SubtableKind::Rearrangement(_kind) = subtable.kind().unwrap() else {
            panic!("expected rearrangement subtable!");
        };
    }

    #[test]
    fn parse_contextual() {
        let font = FontRef::new(font_test_data::morx::EIGHTEEN).unwrap();
        let morx = font.morx().unwrap();
        let chain = morx.chains().iter().next().unwrap().unwrap();
        let subtable = chain.subtables().iter().next().unwrap().unwrap();
        assert_eq!(subtable.coverage(), 0x20_0000_01);
        let SubtableKind::Contextual(kind) = subtable.kind().unwrap() else {
            panic!("expected contextual subtable!");
        };
        let lookup = kind.lookups.get(0).unwrap();
        let expected = [None, None, Some(7u16), Some(8), Some(9), Some(10), Some(11)];
        let values = (0..7).map(|gid| lookup.value(gid).ok()).collect::<Vec<_>>();
        assert_eq!(values, &expected);
    }

    #[test]
    fn parse_ligature() {
        let font = FontRef::new(font_test_data::morx::FORTY_ONE).unwrap();
        let morx = font.morx().unwrap();
        let chain = morx.chains().iter().next().unwrap().unwrap();
        let subtable = chain.subtables().iter().next().unwrap().unwrap();
        assert_eq!(subtable.coverage(), 0x20_0000_02);
        let SubtableKind::Ligature(kind) = subtable.kind().unwrap() else {
            panic!("expected ligature subtable!");
        };
        let expected_actions = [0x3FFFFFFE, 0xBFFFFFFE];
        // Note, we limit the number of elements because the arrays do not
        // have specified lengths in the table
        let actions = kind
            .ligature_actions
            .iter()
            .take(2)
            .map(|action| action.get())
            .collect::<Vec<_>>();
        assert_eq!(actions, &expected_actions);
        let expected_components = [0u16, 1, 0, 0];
        // See above explanation for the limit
        let components = kind
            .components
            .iter()
            .take(4)
            .map(|comp| comp.get())
            .collect::<Vec<_>>();
        assert_eq!(components, &expected_components);
        let expected_ligatures = [GlyphId16::new(5), GlyphId16::new(6)];
        let ligatures = kind
            .ligatures
            .iter()
            .map(|gid| gid.get())
            .collect::<Vec<_>>();
        assert_eq!(ligatures, &expected_ligatures);
    }

    #[test]
    fn parse_non_contextual() {
        let font = FontRef::new(font_test_data::morx::ONE).unwrap();
        let morx = font.morx().unwrap();
        let chain = morx.chains().iter().next().unwrap().unwrap();
        let subtable = chain.subtables().iter().next().unwrap().unwrap();
        assert_eq!(subtable.coverage(), 0x20_0000_04);
        let SubtableKind::NonContextual(kind) = subtable.kind().unwrap() else {
            panic!("expected non-contextual subtable!");
        };
        let expected_values = [None, None, Some(5u16), None, Some(7)];
        let values = (0..5).map(|gid| kind.value(gid).ok()).collect::<Vec<_>>();
        assert_eq!(values, &expected_values);
    }

    #[test]
    fn parse_insertion() {
        let font = FontRef::new(font_test_data::morx::THIRTY_FOUR).unwrap();
        let morx = font.morx().unwrap();
        let chain = morx.chains().iter().next().unwrap().unwrap();
        let subtable = chain.subtables().iter().next().unwrap().unwrap();
        assert_eq!(subtable.coverage(), 0x20_0000_05);
        let SubtableKind::Insertion(kind) = subtable.kind().unwrap() else {
            panic!("expected insertion subtable!");
        };
        let mut expected_glyphs = vec![];
        for _ in 0..9 {
            for gid in [3, 2] {
                expected_glyphs.push(GlyphId16::new(gid));
            }
        }
        assert_eq!(kind.glyphs, &expected_glyphs);
    }
}
