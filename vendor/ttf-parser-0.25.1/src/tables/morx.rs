//! An [Extended Glyph Metamorphosis Table](
//! https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html) implementation.

// Note: We do not have tests for this table because it has a very complicated structure.
// Specifically, the State Machine Tables. I have no idea how to generate them.
// And all fonts that use this table are mainly Apple one, so we cannot use them for legal reasons.
//
// On the other hand, this table is tested indirectly by https://github.com/harfbuzz/rustybuzz
// And it has like 170 tests. Which is pretty good.
// Therefore after applying any changes to this table,
// you have to check that all rustybuzz tests are still passing.

use core::num::NonZeroU16;

use crate::parser::{FromData, LazyArray32, NumFrom, Offset, Offset32, Stream};
use crate::{aat, GlyphId};

/// The feature table is used to compute the sub-feature flags
/// for a list of requested features and settings.
#[derive(Clone, Copy, Debug)]
pub struct Feature {
    /// The type of feature.
    pub kind: u16,
    /// The feature's setting (aka selector).
    pub setting: u16,
    /// Flags for the settings that this feature and setting enables.
    pub enable_flags: u32,
    /// Complement of flags for the settings that this feature and setting disable.
    pub disable_flags: u32,
}

impl FromData for Feature {
    const SIZE: usize = 12;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Feature {
            kind: s.read::<u16>()?,
            setting: s.read::<u16>()?,
            enable_flags: s.read::<u32>()?,
            disable_flags: s.read::<u32>()?,
        })
    }
}

/// A contextual subtable state table trailing data.
#[derive(Clone, Copy, Debug)]
pub struct ContextualEntryData {
    /// A mark index.
    pub mark_index: u16,
    /// A current index.
    pub current_index: u16,
}

impl FromData for ContextualEntryData {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(ContextualEntryData {
            mark_index: s.read::<u16>()?,
            current_index: s.read::<u16>()?,
        })
    }
}

/// A contextual subtable.
#[derive(Clone)]
pub struct ContextualSubtable<'a> {
    /// The contextual glyph substitution state table.
    pub state: aat::ExtendedStateTable<'a, ContextualEntryData>,
    offsets_data: &'a [u8],
    offsets: LazyArray32<'a, Offset32>,
    number_of_glyphs: NonZeroU16,
}

impl<'a> ContextualSubtable<'a> {
    fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let state = aat::ExtendedStateTable::parse(number_of_glyphs, &mut s)?;

        // While the spec clearly states that this is an
        // 'offset from the beginning of the state subtable',
        // it's actually not. Subtable header should not be included.
        let offset = s.read::<Offset32>()?.to_usize();

        // The offsets list is unsized.
        let offsets_data = data.get(offset..)?;
        let offsets = LazyArray32::<Offset32>::new(offsets_data);

        Some(ContextualSubtable {
            state,
            offsets_data,
            offsets,
            number_of_glyphs,
        })
    }

    /// Returns a [Lookup](aat::Lookup) at index.
    pub fn lookup(&self, index: u32) -> Option<aat::Lookup<'a>> {
        let offset = self.offsets.get(index)?.to_usize();
        let lookup_data = self.offsets_data.get(offset..)?;
        aat::Lookup::parse(self.number_of_glyphs, lookup_data)
    }
}

impl core::fmt::Debug for ContextualSubtable<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "ContextualSubtable {{ ... }}")
    }
}

/// A ligature subtable.
#[derive(Clone, Debug)]
pub struct LigatureSubtable<'a> {
    /// A state table.
    pub state: aat::ExtendedStateTable<'a, u16>,
    /// Ligature actions.
    pub ligature_actions: LazyArray32<'a, u32>,
    /// Ligature components.
    pub components: LazyArray32<'a, u16>,
    /// Ligatures.
    pub ligatures: LazyArray32<'a, GlyphId>,
}

impl<'a> LigatureSubtable<'a> {
    fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let state = aat::ExtendedStateTable::parse(number_of_glyphs, &mut s)?;

        // Offset are from `ExtendedStateTable`/`data`, not from subtable start.
        let ligature_action_offset = s.read::<Offset32>()?.to_usize();
        let component_offset = s.read::<Offset32>()?.to_usize();
        let ligature_offset = s.read::<Offset32>()?.to_usize();

        // All three arrays are unsized, so we're simply reading/mapping all the data past offset.
        let ligature_actions = LazyArray32::<u32>::new(data.get(ligature_action_offset..)?);
        let components = LazyArray32::<u16>::new(data.get(component_offset..)?);
        let ligatures = LazyArray32::<GlyphId>::new(data.get(ligature_offset..)?);

        Some(LigatureSubtable {
            state,
            ligature_actions,
            components,
            ligatures,
        })
    }
}

/// A contextual subtable state table trailing data.
#[derive(Clone, Copy, Debug)]
pub struct InsertionEntryData {
    /// A current insert index.
    pub current_insert_index: u16,
    /// A marked insert index.
    pub marked_insert_index: u16,
}

impl FromData for InsertionEntryData {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(InsertionEntryData {
            current_insert_index: s.read::<u16>()?,
            marked_insert_index: s.read::<u16>()?,
        })
    }
}

/// An insertion subtable.
#[derive(Clone, Debug)]
pub struct InsertionSubtable<'a> {
    /// A state table.
    pub state: aat::ExtendedStateTable<'a, InsertionEntryData>,
    /// Insertion glyphs.
    pub glyphs: LazyArray32<'a, GlyphId>,
}

impl<'a> InsertionSubtable<'a> {
    fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let state = aat::ExtendedStateTable::parse(number_of_glyphs, &mut s)?;
        let offset = s.read::<Offset32>()?.to_usize();

        // TODO: unsized array?
        // The list is unsized.
        let glyphs = LazyArray32::<GlyphId>::new(data.get(offset..)?);

        Some(InsertionSubtable { state, glyphs })
    }
}

/// A subtable kind.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum SubtableKind<'a> {
    Rearrangement(aat::ExtendedStateTable<'a, ()>),
    Contextual(ContextualSubtable<'a>),
    Ligature(LigatureSubtable<'a>),
    NonContextual(aat::Lookup<'a>),
    Insertion(InsertionSubtable<'a>),
}

/// A subtable coverage.
#[derive(Clone, Copy, Debug)]
pub struct Coverage(u8);

#[rustfmt::skip]
impl Coverage {
    /// If true, this subtable will process glyphs in logical order
    /// (or reverse logical order if [`is_vertical`](Self::is_vertical) is also true).
    #[inline] pub fn is_logical(self) -> bool { self.0 & 0x10 != 0 }
    /// If true, this subtable will be applied to both horizontal and vertical text
    /// ([`is_vertical`](Self::is_vertical) should be ignored).
    #[inline] pub fn is_all_directions(self) -> bool { self.0 & 0x20 != 0 }
    /// If true, this subtable will process glyphs in descending order.
    #[inline] pub fn is_backwards(self) -> bool { self.0 & 0x40 != 0 }
    /// If true, this subtable will only be applied to vertical text.
    #[inline] pub fn is_vertical(self) -> bool { self.0 & 0x80 != 0 }
}

/// A subtable in a metamorphosis chain.
#[derive(Clone, Debug)]
pub struct Subtable<'a> {
    /// A subtable kind.
    pub kind: SubtableKind<'a>,
    /// A subtable coverage.
    pub coverage: Coverage,
    /// Subtable feature flags.
    pub feature_flags: u32,
}

/// A list of subtables in a metamorphosis chain.
///
/// The internal data layout is not designed for random access,
/// therefore we're not providing the `get()` method and only an iterator.
#[derive(Clone, Copy)]
pub struct Subtables<'a> {
    count: u32,
    data: &'a [u8],
    number_of_glyphs: NonZeroU16,
}

impl<'a> IntoIterator for Subtables<'a> {
    type Item = Subtable<'a>;
    type IntoIter = SubtablesIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SubtablesIter {
            index: 0,
            count: self.count,
            stream: Stream::new(self.data),
            number_of_glyphs: self.number_of_glyphs,
        }
    }
}

impl core::fmt::Debug for Subtables<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtables {{ ... }}")
    }
}

/// An iterator over a metamorphosis chain subtables.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct SubtablesIter<'a> {
    index: u32,
    count: u32,
    stream: Stream<'a>,
    number_of_glyphs: NonZeroU16,
}

impl<'a> Iterator for SubtablesIter<'a> {
    type Item = Subtable<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.count {
            return None;
        }

        let s = &mut self.stream;
        if s.at_end() {
            return None;
        }

        let len = s.read::<u32>()?;
        let coverage = Coverage(s.read::<u8>()?);
        s.skip::<u16>(); // reserved
        let kind = s.read::<u8>()?;
        let feature_flags = s.read::<u32>()?;

        const HEADER_LEN: usize = 12;
        let len = usize::num_from(len).checked_sub(HEADER_LEN)?;
        let subtables_data = s.read_bytes(len)?;

        let kind = match kind {
            0 => {
                let mut s = Stream::new(subtables_data);
                let table = aat::ExtendedStateTable::parse(self.number_of_glyphs, &mut s)?;
                SubtableKind::Rearrangement(table)
            }
            1 => {
                let table = ContextualSubtable::parse(self.number_of_glyphs, subtables_data)?;
                SubtableKind::Contextual(table)
            }
            2 => {
                let table = LigatureSubtable::parse(self.number_of_glyphs, subtables_data)?;
                SubtableKind::Ligature(table)
            }
            // 3 - reserved
            4 => SubtableKind::NonContextual(aat::Lookup::parse(
                self.number_of_glyphs,
                subtables_data,
            )?),
            5 => {
                let table = InsertionSubtable::parse(self.number_of_glyphs, subtables_data)?;
                SubtableKind::Insertion(table)
            }
            _ => return None,
        };

        Some(Subtable {
            kind,
            coverage,
            feature_flags,
        })
    }
}

/// A metamorphosis chain.
#[derive(Clone, Copy, Debug)]
pub struct Chain<'a> {
    /// Default chain features.
    pub default_flags: u32,
    /// A list of chain features.
    pub features: LazyArray32<'a, Feature>,
    /// A list of chain subtables.
    pub subtables: Subtables<'a>,
}

/// A list of metamorphosis chains.
///
/// The internal data layout is not designed for random access,
/// therefore we're not providing the `get()` method and only an iterator.
#[derive(Clone, Copy)]
pub struct Chains<'a> {
    data: &'a [u8],
    count: u32,
    number_of_glyphs: NonZeroU16,
}

impl<'a> Chains<'a> {
    fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        s.skip::<u16>(); // version
        s.skip::<u16>(); // reserved
        let count = s.read::<u32>()?;

        Some(Chains {
            count,
            data: s.tail()?,
            number_of_glyphs,
        })
    }
}

impl<'a> IntoIterator for Chains<'a> {
    type Item = Chain<'a>;
    type IntoIter = ChainsIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ChainsIter {
            index: 0,
            count: self.count,
            stream: Stream::new(self.data),
            number_of_glyphs: self.number_of_glyphs,
        }
    }
}

impl core::fmt::Debug for Chains<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Chains {{ ... }}")
    }
}

/// An iterator over metamorphosis chains.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct ChainsIter<'a> {
    index: u32,
    count: u32,
    stream: Stream<'a>,
    number_of_glyphs: NonZeroU16,
}

impl<'a> Iterator for ChainsIter<'a> {
    type Item = Chain<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.count {
            return None;
        }

        if self.stream.at_end() {
            return None;
        }

        let default_flags = self.stream.read::<u32>()?;
        let len = self.stream.read::<u32>()?;
        let features_count = self.stream.read::<u32>()?;
        let subtables_count = self.stream.read::<u32>()?;

        let features = self.stream.read_array32::<Feature>(features_count)?;

        const HEADER_LEN: usize = 16;
        let len = usize::num_from(len)
            .checked_sub(HEADER_LEN)?
            .checked_sub(Feature::SIZE * usize::num_from(features_count))?;

        let subtables_data = self.stream.read_bytes(len)?;

        let subtables = Subtables {
            data: subtables_data,
            count: subtables_count,
            number_of_glyphs: self.number_of_glyphs,
        };

        Some(Chain {
            default_flags,
            features,
            subtables,
        })
    }
}

/// An [Extended Glyph Metamorphosis Table](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html).
///
/// Subtable Glyph Coverage used by morx v3 is not supported.
#[derive(Clone)]
pub struct Table<'a> {
    /// A list of metamorphosis chains.
    pub chains: Chains<'a>,
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    ///
    /// `number_of_glyphs` is from the `maxp` table.
    pub fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        Chains::parse(number_of_glyphs, data).map(|chains| Self { chains })
    }
}
