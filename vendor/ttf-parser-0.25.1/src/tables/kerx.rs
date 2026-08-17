//! An [Extended Kerning Table](
//! https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6kerx.html) implementation.

// TODO: find a way to test this table
// This table is basically untested because it uses Apple's State Tables
// and I have no idea how to generate them.

use core::num::NonZeroU16;

use crate::kern::KerningPair;
use crate::parser::{FromData, LazyArray32, NumFrom, Offset, Offset32, Stream};
use crate::{aat, GlyphId};

const HEADER_SIZE: usize = 12;

/// A format 0 subtable.
///
/// Ordered List of Kerning Pairs.
///
/// The same as in `kern`, but uses `LazyArray32` instead of `LazyArray16`.
#[derive(Clone, Copy, Debug)]
pub struct Subtable0<'a> {
    /// A list of kerning pairs.
    pub pairs: LazyArray32<'a, KerningPair>,
}

impl<'a> Subtable0<'a> {
    /// Parses a subtable from raw data.
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let number_of_pairs = s.read::<u32>()?;
        s.advance(12); // search_range (u32) + entry_selector (u32) + range_shift (u32)
        let pairs = s.read_array32::<KerningPair>(number_of_pairs)?;
        Some(Self { pairs })
    }

    /// Returns kerning for a pair of glyphs.
    #[inline]
    pub fn glyphs_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i16> {
        let needle = u32::from(left.0) << 16 | u32::from(right.0);
        self.pairs
            .binary_search_by(|v| v.pair.cmp(&needle))
            .map(|(_, v)| v.value)
    }
}

/// A state machine entry.
#[derive(Clone, Copy, Debug)]
pub struct EntryData {
    /// An action index.
    pub action_index: u16,
}

impl FromData for EntryData {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(EntryData {
            action_index: s.read::<u16>()?,
        })
    }
}

/// A format 1 subtable.
///
/// State Table for Contextual Kerning.
#[derive(Clone)]
pub struct Subtable1<'a> {
    /// A state table.
    pub state_table: aat::ExtendedStateTable<'a, EntryData>,
    actions_data: &'a [u8],
}

impl<'a> Subtable1<'a> {
    fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let state_table = aat::ExtendedStateTable::parse(number_of_glyphs, &mut s)?;

        // Actions offset is right after the state table.
        let actions_offset = s.read::<Offset32>()?;
        // Actions offset is from the start of the state table and not from the start of subtable.
        // And since we don't know the length of the actions data,
        // simply store all the data after the offset.
        let actions_data = data.get(actions_offset.to_usize()..)?;

        Some(Subtable1 {
            state_table,
            actions_data,
        })
    }

    /// Returns kerning at action index.
    #[inline]
    pub fn glyphs_kerning(&self, action_index: u16) -> Option<i16> {
        Stream::read_at(self.actions_data, usize::from(action_index) * i16::SIZE)
    }
}

impl<'a> core::ops::Deref for Subtable1<'a> {
    type Target = aat::ExtendedStateTable<'a, EntryData>;

    fn deref(&self) -> &Self::Target {
        &self.state_table
    }
}

impl core::fmt::Debug for Subtable1<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable1 {{ ... }}")
    }
}

/// A format 2 subtable.
///
/// Simple n x m Array of Kerning Values.
///
/// The same as in `kern`, but uses 32bit offsets instead of 16bit one.
#[derive(Clone, Copy)]
pub struct Subtable2<'a>(&'a [u8]); // TODO: parse actual structure

impl<'a> Subtable2<'a> {
    /// Returns kerning for a pair of glyphs.
    pub fn glyphs_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i16> {
        let mut s = Stream::new(self.0);
        s.skip::<u32>(); // row_width

        // Offsets are from beginning of the subtable and not from the `data` start,
        // so we have to subtract the header.
        let left_hand_table_offset = s.read::<Offset32>()?.to_usize().checked_sub(HEADER_SIZE)?;
        let right_hand_table_offset = s.read::<Offset32>()?.to_usize().checked_sub(HEADER_SIZE)?;
        let array_offset = s.read::<Offset32>()?.to_usize().checked_sub(HEADER_SIZE)?;

        // 'The array can be indexed by completing the left-hand and right-hand class mappings,
        // adding the class values to the address of the subtable,
        // and fetching the kerning value to which the new address points.'

        let left_class =
            crate::kern::get_format2_class(left.0, left_hand_table_offset, self.0).unwrap_or(0);
        let right_class =
            crate::kern::get_format2_class(right.0, right_hand_table_offset, self.0).unwrap_or(0);

        // 'Values within the left-hand offset table should not be less than the kerning array offset.'
        if usize::from(left_class) < array_offset {
            return None;
        }

        // Classes are already premultiplied, so we only need to sum them.
        let index = usize::from(left_class) + usize::from(right_class);
        let value_offset = index.checked_sub(HEADER_SIZE)?;
        Stream::read_at::<i16>(self.0, value_offset)
    }
}

impl core::fmt::Debug for Subtable2<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable2 {{ ... }}")
    }
}

/// A container of Anchor Points used by [`Subtable4`].
#[derive(Clone, Copy)]
pub struct AnchorPoints<'a>(&'a [u8]);

impl AnchorPoints<'_> {
    /// Returns a mark and current anchor points at action index.
    pub fn get(&self, action_index: u16) -> Option<(u16, u16)> {
        // Each action contains two 16-bit fields, so we must
        // double the action_index to get the correct offset here.
        let offset = usize::from(action_index) * u16::SIZE * 2;
        let mut s = Stream::new_at(self.0, offset)?;
        Some((s.read::<u16>()?, s.read::<u16>()?))
    }
}

impl core::fmt::Debug for AnchorPoints<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "AnchorPoints {{ ... }}")
    }
}

/// A format 4 subtable.
///
/// State Table for Control Point/Anchor Point Positioning.
///
/// Note: I wasn't able to find any fonts that actually use
/// `ControlPointActions` and/or `ControlPointCoordinateActions`,
/// therefore only `AnchorPointActions` is supported.
#[derive(Clone)]
pub struct Subtable4<'a> {
    /// A state table.
    pub state_table: aat::ExtendedStateTable<'a, EntryData>,
    /// Anchor points.
    pub anchor_points: AnchorPoints<'a>,
}

impl<'a> Subtable4<'a> {
    fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let state_table = aat::ExtendedStateTable::parse(number_of_glyphs, &mut s)?;
        let flags = s.read::<u32>()?;
        let action_type = ((flags & 0xC0000000) >> 30) as u8;
        let points_offset = usize::num_from(flags & 0x00FFFFFF);

        // We support only Anchor Point Actions.
        if action_type != 1 {
            return None;
        }

        Some(Self {
            state_table,
            anchor_points: AnchorPoints(data.get(points_offset..)?),
        })
    }
}

impl<'a> core::ops::Deref for Subtable4<'a> {
    type Target = aat::ExtendedStateTable<'a, EntryData>;

    fn deref(&self) -> &Self::Target {
        &self.state_table
    }
}

impl core::fmt::Debug for Subtable4<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable4 {{ ... }}")
    }
}

/// A format 6 subtable.
///
/// Simple Index-based n x m Array of Kerning Values.
#[derive(Clone, Copy)]
pub struct Subtable6<'a> {
    data: &'a [u8],
    number_of_glyphs: NonZeroU16,
}

impl<'a> Subtable6<'a> {
    // TODO: parse actual structure
    fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Self {
        Subtable6 {
            number_of_glyphs,
            data,
        }
    }

    /// Returns kerning for a pair of glyphs.
    pub fn glyphs_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i16> {
        use core::convert::TryFrom;

        let mut s = Stream::new(self.data);
        let flags = s.read::<u32>()?;
        s.skip::<u16>(); // row_count
        s.skip::<u16>(); // col_count
                         // All offsets are from the start of the subtable.
        let row_index_table_offset = s.read::<Offset32>()?.to_usize().checked_sub(HEADER_SIZE)?;
        let column_index_table_offset =
            s.read::<Offset32>()?.to_usize().checked_sub(HEADER_SIZE)?;
        let kerning_array_offset = s.read::<Offset32>()?.to_usize().checked_sub(HEADER_SIZE)?;
        let kerning_vector_offset = s.read::<Offset32>()?.to_usize().checked_sub(HEADER_SIZE)?;

        let row_index_table_data = self.data.get(row_index_table_offset..)?;
        let column_index_table_data = self.data.get(column_index_table_offset..)?;
        let kerning_array_data = self.data.get(kerning_array_offset..)?;
        let kerning_vector_data = self.data.get(kerning_vector_offset..)?;

        let has_long_values = flags & 0x00000001 != 0;
        if has_long_values {
            let l: u32 = aat::Lookup::parse(self.number_of_glyphs, row_index_table_data)?
                .value(left)
                .unwrap_or(0) as u32;

            let r: u32 = aat::Lookup::parse(self.number_of_glyphs, column_index_table_data)?
                .value(right)
                .unwrap_or(0) as u32;

            let array_offset = usize::try_from(l + r).ok()?.checked_mul(i32::SIZE)?;
            let vector_offset: u32 = Stream::read_at(kerning_array_data, array_offset)?;

            Stream::read_at(kerning_vector_data, usize::num_from(vector_offset))
        } else {
            let l: u16 = aat::Lookup::parse(self.number_of_glyphs, row_index_table_data)?
                .value(left)
                .unwrap_or(0);

            let r: u16 = aat::Lookup::parse(self.number_of_glyphs, column_index_table_data)?
                .value(right)
                .unwrap_or(0);

            let array_offset = usize::from(l + r).checked_mul(i16::SIZE)?;
            let vector_offset: u16 = Stream::read_at(kerning_array_data, array_offset)?;

            Stream::read_at(kerning_vector_data, usize::from(vector_offset))
        }
    }
}

impl core::fmt::Debug for Subtable6<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtable6 {{ ... }}")
    }
}

/// An extended kerning subtable format.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Format<'a> {
    Format0(Subtable0<'a>),
    Format1(Subtable1<'a>),
    Format2(Subtable2<'a>),
    Format4(Subtable4<'a>),
    Format6(Subtable6<'a>),
}

/// A kerning subtable.
#[derive(Clone, Debug)]
pub struct Subtable<'a> {
    /// Indicates that subtable is for horizontal text.
    pub horizontal: bool,
    /// Indicates that subtable is variable.
    pub variable: bool,
    /// Indicates that subtable has a cross-stream values.
    pub has_cross_stream: bool,
    /// Indicates that subtable uses a state machine.
    ///
    /// In this case `glyphs_kerning()` will return `None`.
    pub has_state_machine: bool,
    /// The tuple count.
    ///
    /// This value is only used with variation fonts and should be 0 for all other fonts.
    pub tuple_count: u32,
    /// Subtable format.
    pub format: Format<'a>,
}

impl<'a> Subtable<'a> {
    /// Returns kerning for a pair of glyphs.
    ///
    /// Returns `None` in case of state machine based subtable.
    #[inline]
    pub fn glyphs_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i16> {
        match self.format {
            Format::Format0(ref subtable) => subtable.glyphs_kerning(left, right),
            Format::Format1(_) => None,
            Format::Format2(ref subtable) => subtable.glyphs_kerning(left, right),
            Format::Format4(_) => None,
            Format::Format6(ref subtable) => subtable.glyphs_kerning(left, right),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Coverage(u8);

#[rustfmt::skip]
impl Coverage {
    // TODO: use hex
    #[inline] pub fn is_horizontal(self) -> bool { self.0 & (1 << 7) == 0 }
    #[inline] pub fn has_cross_stream(self) -> bool { self.0 & (1 << 6) != 0 }
    #[inline] pub fn is_variable(self) -> bool { self.0 & (1 << 5) != 0 }
}

/// A list of extended kerning subtables.
///
/// The internal data layout is not designed for random access,
/// therefore we're not providing the `get()` method and only an iterator.
#[derive(Clone, Copy)]
pub struct Subtables<'a> {
    /// The number of glyphs from the `maxp` table.
    number_of_glyphs: NonZeroU16,
    /// The total number of tables.
    number_of_tables: u32,
    /// Actual data. Starts right after the `kerx` header.
    data: &'a [u8],
}

impl core::fmt::Debug for Subtables<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Subtables {{ ... }}")
    }
}

impl<'a> IntoIterator for Subtables<'a> {
    type Item = Subtable<'a>;
    type IntoIter = SubtablesIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SubtablesIter {
            number_of_glyphs: self.number_of_glyphs,
            table_index: 0,
            number_of_tables: self.number_of_tables,
            stream: Stream::new(self.data),
        }
    }
}

/// An iterator over extended kerning subtables.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct SubtablesIter<'a> {
    /// The number of glyphs from the `maxp` table.
    number_of_glyphs: NonZeroU16,
    /// The current table index.
    table_index: u32,
    /// The total number of tables.
    number_of_tables: u32,
    /// Actual data. Starts right after the `kerx` header.
    stream: Stream<'a>,
}

impl<'a> Iterator for SubtablesIter<'a> {
    type Item = Subtable<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.table_index == self.number_of_tables {
            return None;
        }

        if self.stream.at_end() {
            return None;
        }

        let s = &mut self.stream;

        let table_len = s.read::<u32>()?;
        let coverage = Coverage(s.read::<u8>()?);
        s.skip::<u16>(); // unused
        let raw_format = s.read::<u8>()?;
        let tuple_count = s.read::<u32>()?;

        // Subtract the header size.
        let data_len = usize::num_from(table_len).checked_sub(HEADER_SIZE)?;
        let data = s.read_bytes(data_len)?;

        let format = match raw_format {
            0 => Subtable0::parse(data).map(Format::Format0)?,
            1 => Subtable1::parse(self.number_of_glyphs, data).map(Format::Format1)?,
            2 => Format::Format2(Subtable2(data)),
            4 => Subtable4::parse(self.number_of_glyphs, data).map(Format::Format4)?,
            6 => Format::Format6(Subtable6::parse(self.number_of_glyphs, data)),
            _ => {
                // Unknown format.
                return None;
            }
        };

        self.table_index += 1;

        Some(Subtable {
            horizontal: coverage.is_horizontal(),
            variable: coverage.is_variable(),
            has_cross_stream: coverage.has_cross_stream(),
            has_state_machine: raw_format == 1 || raw_format == 4,
            tuple_count,
            format,
        })
    }
}

/// An [Extended Kerning Table](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6kerx.html).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// A list of subtables.
    pub subtables: Subtables<'a>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    ///
    /// `number_of_glyphs` is from the `maxp` table.
    pub fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        s.skip::<u16>(); // version
        s.skip::<u16>(); // padding
        let number_of_tables = s.read::<u32>()?;
        let subtables = Subtables {
            number_of_glyphs,
            number_of_tables,
            data: s.tail()?,
        };

        Some(Table { subtables })
    }
}
