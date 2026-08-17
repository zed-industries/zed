/*!
A collection of [Apple Advanced Typography](
https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6AATIntro.html)
related types.
*/

use core::num::NonZeroU16;

use crate::parser::{FromData, LazyArray16, NumFrom, Offset, Offset16, Offset32, Stream};
use crate::GlyphId;

/// Predefined states.
pub mod state {
    #![allow(missing_docs)]
    pub const START_OF_TEXT: u16 = 0;
}

/// Predefined classes.
///
/// Search for _Class Code_ in [Apple Advanced Typography Font Tables](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html).
pub mod class {
    #![allow(missing_docs)]
    pub const END_OF_TEXT: u8 = 0;
    pub const OUT_OF_BOUNDS: u8 = 1;
    pub const DELETED_GLYPH: u8 = 2;
}

/// A State Table entry.
///
/// Used by legacy and extended tables.
#[derive(Clone, Copy, Debug)]
pub struct GenericStateEntry<T: FromData> {
    /// A new state.
    pub new_state: u16,
    /// Entry flags.
    pub flags: u16,
    /// Additional data.
    ///
    /// Use `()` if no data expected.
    pub extra: T,
}

impl<T: FromData> FromData for GenericStateEntry<T> {
    const SIZE: usize = 4 + T::SIZE;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(GenericStateEntry {
            new_state: s.read::<u16>()?,
            flags: s.read::<u16>()?,
            extra: s.read::<T>()?,
        })
    }
}

impl<T: FromData> GenericStateEntry<T> {
    /// Checks that entry has an offset.
    #[inline]
    pub fn has_offset(&self) -> bool {
        self.flags & 0x3FFF != 0
    }

    /// Returns a value offset.
    ///
    /// Used by kern::format1 subtable.
    #[inline]
    pub fn value_offset(&self) -> ValueOffset {
        ValueOffset(self.flags & 0x3FFF)
    }

    /// If set, reset the kerning data (clear the stack).
    #[inline]
    pub fn has_reset(&self) -> bool {
        self.flags & 0x2000 != 0
    }

    /// If set, advance to the next glyph before going to the new state.
    #[inline]
    pub fn has_advance(&self) -> bool {
        self.flags & 0x4000 == 0
    }

    /// If set, push this glyph on the kerning stack.
    #[inline]
    pub fn has_push(&self) -> bool {
        self.flags & 0x8000 != 0
    }

    /// If set, remember this glyph as the marked glyph.
    ///
    /// Used by kerx::format4 subtable.
    ///
    /// Yes, the same as [`has_push`](Self::has_push).
    #[inline]
    pub fn has_mark(&self) -> bool {
        self.flags & 0x8000 != 0
    }
}

/// A legacy state entry used by [StateTable].
pub type StateEntry = GenericStateEntry<()>;

/// A type-safe wrapper for a kerning value offset.
#[derive(Clone, Copy, Debug)]
pub struct ValueOffset(u16);

impl ValueOffset {
    /// Returns the next offset.
    ///
    /// After reaching u16::MAX will start from 0.
    #[inline]
    pub fn next(self) -> Self {
        ValueOffset(self.0.wrapping_add(u16::SIZE as u16))
    }
}

/// A [State Table](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html).
///
/// Also called `STHeader`.
///
/// Currently used by `kern` table.
#[derive(Clone)]
pub struct StateTable<'a> {
    number_of_classes: u16,
    first_glyph: GlyphId,
    class_table: &'a [u8],
    state_array_offset: u16,
    state_array: &'a [u8],
    entry_table: &'a [u8],
    actions: &'a [u8],
}

impl<'a> StateTable<'a> {
    pub(crate) fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let number_of_classes: u16 = s.read()?;
        // Note that in format1 subtable, offsets are not from the subtable start,
        // but from subtable start + `header_size`.
        // So there is not need to subtract the `header_size`.
        let class_table_offset = s.read::<Offset16>()?.to_usize();
        let state_array_offset = s.read::<Offset16>()?.to_usize();
        let entry_table_offset = s.read::<Offset16>()?.to_usize();
        // Ignore `values_offset` since we don't use it.

        // Parse class subtable.
        let mut s = Stream::new_at(data, class_table_offset)?;
        let first_glyph: GlyphId = s.read()?;
        let number_of_glyphs: u16 = s.read()?;
        // The class table contains u8, so it's easier to use just a slice
        // instead of a LazyArray.
        let class_table = s.read_bytes(usize::from(number_of_glyphs))?;

        Some(StateTable {
            number_of_classes,
            first_glyph,
            class_table,
            state_array_offset: state_array_offset as u16,
            // We don't know the actual data size and it's kinda expensive to calculate.
            // So we are simply storing all the data past the offset.
            // Despite the fact that they may overlap.
            state_array: data.get(state_array_offset..)?,
            entry_table: data.get(entry_table_offset..)?,
            // `ValueOffset` defines an offset from the start of the subtable data.
            // We do not check that the provided offset is actually after `values_offset`.
            actions: data,
        })
    }

    /// Returns a glyph class.
    #[inline]
    pub fn class(&self, glyph_id: GlyphId) -> Option<u8> {
        if glyph_id.0 == 0xFFFF {
            return Some(class::DELETED_GLYPH);
        }

        let idx = glyph_id.0.checked_sub(self.first_glyph.0)?;
        self.class_table.get(usize::from(idx)).copied()
    }

    /// Returns a class entry.
    #[inline]
    pub fn entry(&self, state: u16, mut class: u8) -> Option<StateEntry> {
        if u16::from(class) >= self.number_of_classes {
            class = class::OUT_OF_BOUNDS;
        }

        let entry_idx = self
            .state_array
            .get(usize::from(state) * usize::from(self.number_of_classes) + usize::from(class))?;

        Stream::read_at(self.entry_table, usize::from(*entry_idx) * StateEntry::SIZE)
    }

    /// Returns kerning at offset.
    #[inline]
    pub fn kerning(&self, offset: ValueOffset) -> Option<i16> {
        Stream::read_at(self.actions, usize::from(offset.0))
    }

    /// Produces a new state.
    #[inline]
    pub fn new_state(&self, state: u16) -> u16 {
        let n = (i32::from(state) - i32::from(self.state_array_offset))
            / i32::from(self.number_of_classes);

        use core::convert::TryFrom;
        u16::try_from(n).unwrap_or(0)
    }
}

impl core::fmt::Debug for StateTable<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "StateTable {{ ... }}")
    }
}

/// An [Extended State Table](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html).
///
/// Also called `STXHeader`.
///
/// Currently used by `kerx` and `morx` tables.
#[derive(Clone)]
pub struct ExtendedStateTable<'a, T> {
    number_of_classes: u32,
    lookup: Lookup<'a>,
    state_array: &'a [u8],
    entry_table: &'a [u8],
    entry_type: core::marker::PhantomData<T>,
}

impl<'a, T: FromData> ExtendedStateTable<'a, T> {
    // TODO: make private
    /// Parses an Extended State Table from a stream.
    ///
    /// `number_of_glyphs` is from the `maxp` table.
    pub fn parse(number_of_glyphs: NonZeroU16, s: &mut Stream<'a>) -> Option<Self> {
        let data = s.tail()?;

        let number_of_classes = s.read::<u32>()?;
        // Note that offsets are not from the subtable start,
        // but from subtable start + `header_size`.
        // So there is not need to subtract the `header_size`.
        let lookup_table_offset = s.read::<Offset32>()?.to_usize();
        let state_array_offset = s.read::<Offset32>()?.to_usize();
        let entry_table_offset = s.read::<Offset32>()?.to_usize();

        Some(ExtendedStateTable {
            number_of_classes,
            lookup: Lookup::parse(number_of_glyphs, data.get(lookup_table_offset..)?)?,
            // We don't know the actual data size and it's kinda expensive to calculate.
            // So we are simply storing all the data past the offset.
            // Despite the fact that they may overlap.
            state_array: data.get(state_array_offset..)?,
            entry_table: data.get(entry_table_offset..)?,
            entry_type: core::marker::PhantomData,
        })
    }

    /// Returns a glyph class.
    #[inline]
    pub fn class(&self, glyph_id: GlyphId) -> Option<u16> {
        if glyph_id.0 == 0xFFFF {
            return Some(u16::from(class::DELETED_GLYPH));
        }

        self.lookup.value(glyph_id)
    }

    /// Returns a class entry.
    #[inline]
    pub fn entry(&self, state: u16, mut class: u16) -> Option<GenericStateEntry<T>> {
        if u32::from(class) >= self.number_of_classes {
            class = u16::from(class::OUT_OF_BOUNDS);
        }

        let state_idx =
            usize::from(state) * usize::num_from(self.number_of_classes) + usize::from(class);

        let entry_idx: u16 = Stream::read_at(self.state_array, state_idx * u16::SIZE)?;
        Stream::read_at(
            self.entry_table,
            usize::from(entry_idx) * GenericStateEntry::<T>::SIZE,
        )
    }
}

impl<T> core::fmt::Debug for ExtendedStateTable<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "ExtendedStateTable {{ ... }}")
    }
}

/// A [lookup table](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html).
///
/// u32 values in Format10 tables will be truncated to u16.
/// u64 values in Format10 tables are not supported.
#[derive(Clone)]
pub struct Lookup<'a> {
    data: LookupInner<'a>,
}

impl<'a> Lookup<'a> {
    /// Parses a lookup table from raw data.
    ///
    /// `number_of_glyphs` is from the `maxp` table.
    #[inline]
    pub fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        LookupInner::parse(number_of_glyphs, data).map(|data| Self { data })
    }

    /// Returns a value associated with the specified glyph.
    #[inline]
    pub fn value(&self, glyph_id: GlyphId) -> Option<u16> {
        self.data.value(glyph_id)
    }
}

impl core::fmt::Debug for Lookup<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Lookup {{ ... }}")
    }
}

#[derive(Clone)]
enum LookupInner<'a> {
    Format1(LazyArray16<'a, u16>),
    Format2(BinarySearchTable<'a, LookupSegment>),
    Format4(BinarySearchTable<'a, LookupSegment>, &'a [u8]),
    Format6(BinarySearchTable<'a, LookupSingle>),
    Format8 {
        first_glyph: u16,
        values: LazyArray16<'a, u16>,
    },
    Format10 {
        value_size: u16,
        first_glyph: u16,
        glyph_count: u16,
        data: &'a [u8],
    },
}

impl<'a> LookupInner<'a> {
    fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let format = s.read::<u16>()?;
        match format {
            0 => {
                let values = s.read_array16::<u16>(number_of_glyphs.get())?;
                Some(Self::Format1(values))
            }
            2 => {
                let bsearch = BinarySearchTable::<LookupSegment>::parse(s.tail()?)?;
                Some(Self::Format2(bsearch))
            }
            4 => {
                let bsearch = BinarySearchTable::<LookupSegment>::parse(s.tail()?)?;
                Some(Self::Format4(bsearch, data))
            }
            6 => {
                let bsearch = BinarySearchTable::<LookupSingle>::parse(s.tail()?)?;
                Some(Self::Format6(bsearch))
            }
            8 => {
                let first_glyph = s.read::<u16>()?;
                let glyph_count = s.read::<u16>()?;
                let values = s.read_array16::<u16>(glyph_count)?;
                Some(Self::Format8 {
                    first_glyph,
                    values,
                })
            }
            10 => {
                let value_size = s.read::<u16>()?;
                let first_glyph = s.read::<u16>()?;
                let glyph_count = s.read::<u16>()?;
                Some(Self::Format10 {
                    value_size,
                    first_glyph,
                    glyph_count,
                    data: s.tail()?,
                })
            }
            _ => None,
        }
    }

    fn value(&self, glyph_id: GlyphId) -> Option<u16> {
        match self {
            Self::Format1(values) => values.get(glyph_id.0),
            Self::Format2(ref bsearch) => bsearch.get(glyph_id).map(|v| v.value),
            Self::Format4(ref bsearch, data) => {
                // In format 4, LookupSegment contains an offset to a list of u16 values.
                // One value for each glyph in the LookupSegment range.
                let segment = bsearch.get(glyph_id)?;
                let index = glyph_id.0.checked_sub(segment.first_glyph)?;
                let offset = usize::from(segment.value) + u16::SIZE * usize::from(index);
                Stream::read_at::<u16>(data, offset)
            }
            Self::Format6(ref bsearch) => bsearch.get(glyph_id).map(|v| v.value),
            Self::Format8 {
                first_glyph,
                values,
            } => {
                let idx = glyph_id.0.checked_sub(*first_glyph)?;
                values.get(idx)
            }
            Self::Format10 {
                value_size,
                first_glyph,
                glyph_count,
                data,
            } => {
                let idx = glyph_id.0.checked_sub(*first_glyph)?;
                let mut s = Stream::new(data);
                match value_size {
                    1 => s.read_array16::<u8>(*glyph_count)?.get(idx).map(u16::from),
                    2 => s.read_array16::<u16>(*glyph_count)?.get(idx),
                    // TODO: we should return u32 here, but this is not supported yet
                    4 => s
                        .read_array16::<u32>(*glyph_count)?
                        .get(idx)
                        .map(|n| n as u16),
                    _ => None, // 8 is also supported
                }
            }
        }
    }
}

/// A binary searching table as defined at
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html
#[derive(Clone)]
struct BinarySearchTable<'a, T: BinarySearchValue> {
    values: LazyArray16<'a, T>,
    len: NonZeroU16, // values length excluding termination segment
}

impl<'a, T: BinarySearchValue + core::fmt::Debug> BinarySearchTable<'a, T> {
    #[inline(never)]
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let segment_size = s.read::<u16>()?;
        let number_of_segments = s.read::<u16>()?;
        s.advance(6); // search_range + entry_selector + range_shift

        if usize::from(segment_size) != T::SIZE {
            return None;
        }

        if number_of_segments == 0 {
            return None;
        }

        let values = s.read_array16::<T>(number_of_segments)?;

        // 'The number of termination values that need to be included is table-specific.
        // The value that indicates binary search termination is 0xFFFF.'
        let mut len = number_of_segments;
        if values.last()?.is_termination() {
            len = len.checked_sub(1)?;
        }

        Some(BinarySearchTable {
            len: NonZeroU16::new(len)?,
            values,
        })
    }

    fn get(&self, key: GlyphId) -> Option<T> {
        let mut min = 0;
        let mut max = (self.len.get() as isize) - 1;
        while min <= max {
            let mid = (min + max) / 2;
            let v = self.values.get(mid as u16)?;
            match v.contains(key) {
                core::cmp::Ordering::Less => max = mid - 1,
                core::cmp::Ordering::Greater => min = mid + 1,
                core::cmp::Ordering::Equal => return Some(v),
            }
        }

        None
    }
}

trait BinarySearchValue: FromData {
    fn is_termination(&self) -> bool;
    fn contains(&self, glyph_id: GlyphId) -> core::cmp::Ordering;
}

#[derive(Clone, Copy, Debug)]
struct LookupSegment {
    last_glyph: u16,
    first_glyph: u16,
    value: u16,
}

impl FromData for LookupSegment {
    const SIZE: usize = 6;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(LookupSegment {
            last_glyph: s.read::<u16>()?,
            first_glyph: s.read::<u16>()?,
            value: s.read::<u16>()?,
        })
    }
}

impl BinarySearchValue for LookupSegment {
    #[inline]
    fn is_termination(&self) -> bool {
        self.last_glyph == 0xFFFF && self.first_glyph == 0xFFFF
    }

    #[inline]
    fn contains(&self, id: GlyphId) -> core::cmp::Ordering {
        if id.0 < self.first_glyph {
            core::cmp::Ordering::Less
        } else if id.0 <= self.last_glyph {
            core::cmp::Ordering::Equal
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct LookupSingle {
    glyph: u16,
    value: u16,
}

impl FromData for LookupSingle {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(LookupSingle {
            glyph: s.read::<u16>()?,
            value: s.read::<u16>()?,
        })
    }
}

impl BinarySearchValue for LookupSingle {
    #[inline]
    fn is_termination(&self) -> bool {
        self.glyph == 0xFFFF
    }

    #[inline]
    fn contains(&self, id: GlyphId) -> core::cmp::Ordering {
        id.0.cmp(&self.glyph)
    }
}
