/*!
A [Kerning Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/kern) implementation.

Supports both
[OpenType](https://docs.microsoft.com/en-us/typography/opentype/spec/kern)
and
[Apple Advanced Typography](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6kern.html)
variants.

Since there is no single correct way to process a kerning data,
we have to provide an access to kerning subtables, so a caller can implement
a kerning algorithm manually.
But we still try to keep the API as high-level as possible.
*/

#[cfg(feature = "apple-layout")]
use crate::aat;
use crate::parser::{FromData, LazyArray16, NumFrom, Offset, Offset16, Stream};
use crate::GlyphId;

#[derive(Clone, Copy, Debug)]
struct OTCoverage(u8);

#[rustfmt::skip]
impl OTCoverage {
    #[inline] fn is_horizontal(self) -> bool { self.0 & (1 << 0) != 0 }
    #[inline] fn has_cross_stream(self) -> bool { self.0 & (1 << 2) != 0 }
}

impl FromData for OTCoverage {
    const SIZE: usize = 1;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.get(0).copied().map(OTCoverage)
    }
}

#[derive(Clone, Copy, Debug)]
struct AATCoverage(u8);

#[rustfmt::skip]
impl AATCoverage {
    #[inline] fn is_horizontal(self) -> bool { self.0 & (1 << 7) == 0 }
    #[inline] fn has_cross_stream(self) -> bool { self.0 & (1 << 6) != 0 }
    #[inline] fn is_variable(self) -> bool { self.0 & (1 << 5) != 0 }
}

impl FromData for AATCoverage {
    const SIZE: usize = 1;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.get(0).copied().map(AATCoverage)
    }
}

/// A kerning pair.
#[derive(Clone, Copy, Debug)]
pub struct KerningPair {
    /// Glyphs pair.
    ///
    /// In the kern table spec, a kerning pair is stored as two u16,
    /// but we are using one u32, so we can binary search it directly.
    pub pair: u32,
    /// Kerning value.
    pub value: i16,
}

impl KerningPair {
    /// Returns left glyph ID.
    #[inline]
    pub fn left(&self) -> GlyphId {
        GlyphId((self.pair >> 16) as u16)
    }

    /// Returns right glyph ID.
    #[inline]
    pub fn right(&self) -> GlyphId {
        GlyphId(self.pair as u16)
    }
}

impl FromData for KerningPair {
    const SIZE: usize = 6;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(KerningPair {
            pair: s.read::<u32>()?,
            value: s.read::<i16>()?,
        })
    }
}

/// A kerning subtable format.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Format<'a> {
    Format0(Subtable0<'a>),
    #[cfg(feature = "apple-layout")]
    Format1(aat::StateTable<'a>),
    #[cfg(not(feature = "apple-layout"))]
    Format1,
    Format2(Subtable2<'a>),
    Format3(Subtable3<'a>),
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
            Format::Format2(ref subtable) => subtable.glyphs_kerning(left, right),
            Format::Format3(ref subtable) => subtable.glyphs_kerning(left, right),
            _ => None,
        }
    }
}

/// A list of subtables.
///
/// The internal data layout is not designed for random access,
/// therefore we're not providing the `get()` method and only an iterator.
#[derive(Clone, Copy)]
pub struct Subtables<'a> {
    /// Indicates an Apple Advanced Typography format.
    is_aat: bool,
    /// The total number of tables.
    count: u32,
    /// Actual data. Starts right after the `kern` header.
    data: &'a [u8],
}

impl<'a> Subtables<'a> {
    /// Returns the number of subtables.
    pub fn len(&self) -> u32 {
        self.count
    }

    /// Checks if there are any subtables.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
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
            is_aat: self.is_aat,
            table_index: 0,
            number_of_tables: self.count,
            stream: Stream::new(self.data),
        }
    }
}

/// An iterator over kerning subtables.
#[allow(missing_debug_implementations)]
#[derive(Clone, Default)]
pub struct SubtablesIter<'a> {
    /// Indicates an Apple Advanced Typography format.
    is_aat: bool,
    /// The current table index,
    table_index: u32,
    /// The total number of tables.
    number_of_tables: u32,
    /// Actual data. Starts right after `kern` header.
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

        if self.is_aat {
            const HEADER_SIZE: u8 = 8;

            let table_len = self.stream.read::<u32>()?;
            let coverage = self.stream.read::<AATCoverage>()?;
            let format_id = self.stream.read::<u8>()?;
            self.stream.skip::<u16>(); // variation tuple index

            if format_id > 3 {
                // Unknown format.
                return None;
            }

            // Subtract the header size.
            let data_len = usize::num_from(table_len).checked_sub(usize::from(HEADER_SIZE))?;
            let data = self.stream.read_bytes(data_len)?;

            let format = match format_id {
                0 => Format::Format0(Subtable0::parse(data)?),
                #[cfg(feature = "apple-layout")]
                1 => Format::Format1(aat::StateTable::parse(data)?),
                #[cfg(not(feature = "apple-layout"))]
                1 => Format::Format1,
                2 => Format::Format2(Subtable2::parse(HEADER_SIZE, data)?),
                3 => Format::Format3(Subtable3::parse(data)?),
                _ => return None,
            };

            Some(Subtable {
                horizontal: coverage.is_horizontal(),
                variable: coverage.is_variable(),
                has_cross_stream: coverage.has_cross_stream(),
                has_state_machine: format_id == 1,
                format,
            })
        } else {
            const HEADER_SIZE: u8 = 6;

            self.stream.skip::<u16>(); // version
            let table_len = self.stream.read::<u16>()?;
            // In the OpenType variant, `format` comes first.
            let format_id = self.stream.read::<u8>()?;
            let coverage = self.stream.read::<OTCoverage>()?;

            if format_id != 0 && format_id != 2 {
                // Unknown format.
                return None;
            }

            let data_len = if self.number_of_tables == 1 {
                // An OpenType `kern` table with just one subtable is a special case.
                // The `table_len` property is mainly required to jump to the next subtable,
                // but if there is only one subtable, this property can be ignored.
                // This is abused by some fonts, to get around the `u16` size limit.
                self.stream.tail()?.len()
            } else {
                // Subtract the header size.
                usize::from(table_len).checked_sub(usize::from(HEADER_SIZE))?
            };

            let data = self.stream.read_bytes(data_len)?;

            let format = match format_id {
                0 => Format::Format0(Subtable0::parse(data)?),
                2 => Format::Format2(Subtable2::parse(HEADER_SIZE, data)?),
                _ => return None,
            };

            Some(Subtable {
                horizontal: coverage.is_horizontal(),
                variable: false, // Only AAT supports it.
                has_cross_stream: coverage.has_cross_stream(),
                has_state_machine: format_id == 1,
                format,
            })
        }
    }
}

/// A format 0 subtable.
///
/// Ordered List of Kerning Pairs.
#[derive(Clone, Copy, Debug)]
pub struct Subtable0<'a> {
    /// A list of kerning pairs.
    pub pairs: LazyArray16<'a, KerningPair>,
}

impl<'a> Subtable0<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let number_of_pairs = s.read::<u16>()?;
        s.advance(6); // search_range (u16) + entry_selector (u16) + range_shift (u16)
        let pairs = s.read_array16::<KerningPair>(number_of_pairs)?;
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

/// A format 2 subtable.
///
/// Simple n x m Array of Kerning Values.
#[derive(Clone, Copy, Debug)]
pub struct Subtable2<'a> {
    // TODO: parse actual structure
    data: &'a [u8],
    header_len: u8,
}

impl<'a> Subtable2<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(header_len: u8, data: &'a [u8]) -> Option<Self> {
        Some(Self { header_len, data })
    }

    /// Returns kerning for a pair of glyphs.
    pub fn glyphs_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i16> {
        let mut s = Stream::new(self.data);
        s.skip::<u16>(); // row_width

        // Offsets are from beginning of the subtable and not from the `data` start,
        // so we have to subtract the header.
        let header_len = usize::from(self.header_len);
        let left_hand_table_offset = s.read::<Offset16>()?.to_usize().checked_sub(header_len)?;
        let right_hand_table_offset = s.read::<Offset16>()?.to_usize().checked_sub(header_len)?;
        let array_offset = s.read::<Offset16>()?.to_usize().checked_sub(header_len)?;

        // 'The array can be indexed by completing the left-hand and right-hand class mappings,
        // adding the class values to the address of the subtable,
        // and fetching the kerning value to which the new address points.'

        let left_class = get_format2_class(left.0, left_hand_table_offset, self.data).unwrap_or(0);
        let right_class =
            get_format2_class(right.0, right_hand_table_offset, self.data).unwrap_or(0);

        // 'Values within the left-hand offset table should not be less than the kerning array offset.'
        if usize::from(left_class) < array_offset {
            return None;
        }

        // Classes are already premultiplied, so we only need to sum them.
        let index = usize::from(left_class) + usize::from(right_class);
        let value_offset = index.checked_sub(header_len)?;
        Stream::read_at::<i16>(self.data, value_offset)
    }
}

pub(crate) fn get_format2_class(glyph_id: u16, offset: usize, data: &[u8]) -> Option<u16> {
    let mut s = Stream::new_at(data, offset)?;
    let first_glyph = s.read::<u16>()?;
    let index = glyph_id.checked_sub(first_glyph)?;

    let number_of_classes = s.read::<u16>()?;
    let classes = s.read_array16::<u16>(number_of_classes)?;
    classes.get(index)
}

/// A format 3 subtable.
///
/// Simple n x m Array of Kerning Indices.
#[derive(Clone, Copy, Debug)]
pub struct Subtable3<'a> {
    // TODO: parse actual structure
    data: &'a [u8],
}

impl<'a> Subtable3<'a> {
    /// Parses a subtable from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        Some(Self { data })
    }

    /// Returns kerning for a pair of glyphs.
    #[inline]
    pub fn glyphs_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i16> {
        let mut s = Stream::new(self.data);
        let glyph_count = s.read::<u16>()?;
        let kerning_values_count = s.read::<u8>()?;
        let left_hand_classes_count = s.read::<u8>()?;
        let right_hand_classes_count = s.read::<u8>()?;
        s.skip::<u8>(); // reserved
        let indices_count =
            u16::from(left_hand_classes_count) * u16::from(right_hand_classes_count);

        let kerning_values = s.read_array16::<i16>(u16::from(kerning_values_count))?;
        let left_hand_classes = s.read_array16::<u8>(glyph_count)?;
        let right_hand_classes = s.read_array16::<u8>(glyph_count)?;
        let indices = s.read_array16::<u8>(indices_count)?;

        let left_class = left_hand_classes.get(left.0)?;
        let right_class = right_hand_classes.get(right.0)?;

        if left_class > left_hand_classes_count || right_class > right_hand_classes_count {
            return None;
        }

        let index =
            u16::from(left_class) * u16::from(right_hand_classes_count) + u16::from(right_class);
        let index = indices.get(index)?;
        kerning_values.get(u16::from(index))
    }
}

/// A [Kerning Table](https://docs.microsoft.com/en-us/typography/opentype/spec/kern).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// A list of subtables.
    pub subtables: Subtables<'a>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        // The `kern` table has two variants: OpenType and Apple.
        // And they both have different headers.
        // There are no robust way to distinguish them, so we have to guess.
        //
        // The OpenType one has the first two bytes (UInt16) as a version set to 0.
        // While Apple one has the first four bytes (Fixed) set to 1.0
        // So the first two bytes in case of an OpenType format will be 0x0000
        // and 0x0001 in case of an Apple format.
        let mut s = Stream::new(data);
        let version = s.read::<u16>()?;
        let subtables = if version == 0 {
            let count = s.read::<u16>()?;
            Subtables {
                is_aat: false,
                count: u32::from(count),
                data: s.tail()?,
            }
        } else {
            s.skip::<u16>(); // Skip the second part of u32 version.
                             // Note that AAT stores the number of tables as u32 and not as u16.
            let count = s.read::<u32>()?;
            Subtables {
                is_aat: true,
                count,
                data: s.tail()?,
            }
        };

        Some(Self { subtables })
    }
}
