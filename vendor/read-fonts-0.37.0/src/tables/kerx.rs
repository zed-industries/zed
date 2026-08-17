//! The [Extended Kerning (kerx)](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6kerx.html) table.

use super::aat::{safe_read_array_to_end, ExtendedStateTable, LookupU16, LookupU32};

include!("../../generated/generated_kerx.rs");

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
    // length, coverage, tuple_count: all u32
    pub const HEADER_LEN: usize = u32::RAW_BYTE_LEN * 3;

    /// True if the table has vertical kerning values.
    #[inline]
    pub fn is_vertical(&self) -> bool {
        self.coverage() & 0x80000000 != 0
    }

    /// True if the table has horizontal kerning values.    
    #[inline]
    pub fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }

    /// True if the table has cross-stream kerning values.
    ///
    /// If text is normally written horizontally, adjustments will be
    /// vertical. If adjustment values are positive, the text will be
    /// moved up. If they are negative, the text will be moved down.
    /// If text is normally written vertically, adjustments will be
    /// horizontal. If adjustment values are positive, the text will be
    /// moved to the right. If they are negative, the text will be moved
    /// to the left.
    #[inline]
    pub fn is_cross_stream(&self) -> bool {
        self.coverage() & 0x40000000 != 0
    }

    /// True if the table has variation kerning values.
    #[inline]
    pub fn is_variable(&self) -> bool {
        self.coverage() & 0x20000000 != 0
    }

    /// Process direction flag. If clear, process the glyphs forwards,
    /// that is, from first to last in the glyph stream. If we, process
    /// them from last to first. This flag only applies to state-table
    /// based 'kerx' subtables (types 1 and 4).
    #[inline]
    pub fn process_direction(&self) -> bool {
        self.coverage() & 0x10000000 != 0
    }

    /// Returns an enum representing the actual subtable data.
    pub fn kind(&self) -> Result<SubtableKind<'a>, ReadError> {
        SubtableKind::read_with_args(
            FontData::new(self.data()),
            &(self.coverage(), self.tuple_count()),
        )
    }
}

/// The various `kerx` subtable formats.
#[derive(Clone)]
pub enum SubtableKind<'a> {
    Format0(Subtable0<'a>),
    Format1(Subtable1<'a>),
    Format2(Subtable2<'a>),
    Format4(Subtable4<'a>),
    Format6(Subtable6<'a>),
}

impl ReadArgs for SubtableKind<'_> {
    type Args = (u32, u32);
}

impl<'a> FontReadWithArgs<'a> for SubtableKind<'a> {
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        // Format is low byte of coverage
        let format = args.0 & 0xFF;
        let tuple_count = args.1;
        match format {
            0 => Ok(Self::Format0(Subtable0::read(data)?)),
            1 => Ok(Self::Format1(Subtable1::read(data)?)),
            2 => Ok(Self::Format2(Subtable2::read(data)?)),
            // No format 3
            4 => Ok(Self::Format4(Subtable4::read(data)?)),
            // No format 5
            6 => Ok(Self::Format6(Subtable6::read_with_args(
                data,
                &tuple_count,
            )?)),
            _ => Err(ReadError::InvalidFormat(format as _)),
        }
    }
}

impl Subtable0<'_> {
    /// Returns the kerning adjustment for the given pair.
    pub fn kerning(&self, left: GlyphId, right: GlyphId) -> Option<i32> {
        pair_kerning(self.pairs(), left, right)
    }
}

pub(crate) fn pair_kerning(pairs: &[Subtable0Pair], left: GlyphId, right: GlyphId) -> Option<i32> {
    let left: GlyphId16 = left.try_into().ok()?;
    let right: GlyphId16 = right.try_into().ok()?;
    fn make_key(left: GlyphId16, right: GlyphId16) -> u32 {
        (left.to_u32() << 16) | right.to_u32()
    }
    let idx = pairs
        .binary_search_by_key(&make_key(left, right), |pair| {
            make_key(pair.left(), pair.right())
        })
        .ok()?;
    pairs.get(idx).map(|pair| pair.value() as i32)
}

/// The type 1 `kerx` subtable.
#[derive(Clone)]
pub struct Subtable1<'a> {
    pub state_table: ExtendedStateTable<'a, BigEndian<u16>>,
    /// Contains the set of kerning values, one for each state.
    pub values: &'a [BigEndian<i16>],
}

impl<'a> FontRead<'a> for Subtable1<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let state_table = ExtendedStateTable::read(data)?;
        let mut cursor = data.cursor();
        cursor.advance_by(ExtendedStateTable::<()>::HEADER_LEN);
        let values_offset = cursor.read::<u32>()? as usize;
        let values = super::aat::safe_read_array_to_end(&data, values_offset)?;
        Ok(Self {
            state_table,
            values,
        })
    }
}

/// The type 2 `kerx` subtable.
#[derive(Clone)]
pub struct Subtable2<'a> {
    pub data: FontData<'a>,
    /// Left-hand offset table.
    pub left_offset_table: LookupU16<'a>,
    /// Right-hand offset table.
    pub right_offset_table: LookupU16<'a>,
    /// Kerning values.
    pub array: &'a [BigEndian<i16>],
}

impl<'a> FontRead<'a> for Subtable2<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        // Skip rowWidth field
        cursor.advance_by(u32::RAW_BYTE_LEN);
        // The offsets here are from the beginning of the subtable and not
        // from the "data" section, so we need to hand parse and subtract
        // the header size.
        let left_offset = (cursor.read::<u32>()? as usize)
            .checked_sub(Subtable::HEADER_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        let right_offset = (cursor.read::<u32>()? as usize)
            .checked_sub(Subtable::HEADER_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        let array_offset = (cursor.read::<u32>()? as usize)
            .checked_sub(Subtable::HEADER_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        let left_offset_table =
            LookupU16::read(data.slice(left_offset..).ok_or(ReadError::OutOfBounds)?)?;
        let right_offset_table =
            LookupU16::read(data.slice(right_offset..).ok_or(ReadError::OutOfBounds)?)?;
        let array = safe_read_array_to_end(&data, array_offset)?;
        Ok(Self {
            data,
            left_offset_table,
            right_offset_table,
            array,
        })
    }
}

impl Subtable2<'_> {
    /// Returns the kerning adjustment for the given pair.
    pub fn kerning(&self, left: GlyphId, right: GlyphId) -> Option<i32> {
        let left: u16 = left.to_u32().try_into().ok()?;
        let right: u16 = right.to_u32().try_into().ok()?;
        let left_idx = self.left_offset_table.value(left).unwrap_or(0) as usize;
        let right_idx = self.right_offset_table.value(right).unwrap_or(0) as usize;
        self.array
            .get(left_idx + right_idx)
            .map(|value| value.get() as i32)
    }
}

/// The type 4 `kerx` subtable.
#[derive(Clone)]
pub struct Subtable4<'a> {
    pub state_table: ExtendedStateTable<'a, BigEndian<u16>>,
    /// Flags for control point positioning.
    pub flags: u32,
    pub actions: Subtable4Actions<'a>,
}

impl<'a> FontRead<'a> for Subtable4<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let state_table = ExtendedStateTable::read(data)?;
        let mut cursor = data.cursor();
        cursor.advance_by(ExtendedStateTable::<()>::HEADER_LEN);
        let flags = cursor.read::<u32>()?;
        let action_type = (flags & 0xC0000000) >> 30;
        let offset = (flags & 0x00FFFFFF) as usize;
        let actions = match action_type {
            0 => Subtable4Actions::ControlPoints(safe_read_array_to_end(&data, offset)?),
            1 => Subtable4Actions::AnchorPoints(safe_read_array_to_end(&data, offset)?),
            2 => Subtable4Actions::ControlPointCoords(safe_read_array_to_end(&data, offset)?),
            _ => {
                return Err(ReadError::MalformedData(
                    "invalid action type in kerx subtable 4",
                ))
            }
        };
        Ok(Self {
            state_table,
            flags,
            actions,
        })
    }
}

/// Actions for the type 4 `kerx` subtable.
#[derive(Clone)]
pub enum Subtable4Actions<'a> {
    /// Sequence of glyph outline point indices.
    ControlPoints(&'a [BigEndian<u16>]),
    /// Sequence of indices into the `ankr` table.
    AnchorPoints(&'a [BigEndian<u16>]),
    /// Sequence of coordinate values.
    ControlPointCoords(&'a [BigEndian<i16>]),
}

/// The type 6 `kerx` subtable.
#[derive(Clone)]
pub enum Subtable6<'a> {
    ShortValues(
        LookupU16<'a>,
        LookupU16<'a>,
        &'a [BigEndian<i16>],
        Option<&'a [BigEndian<i16>]>,
    ),
    LongValues(
        LookupU32<'a>,
        LookupU32<'a>,
        &'a [BigEndian<i32>],
        Option<&'a [BigEndian<i16>]>,
    ),
}

impl ReadArgs for Subtable6<'_> {
    type Args = u32;
}

impl<'a> FontReadWithArgs<'a> for Subtable6<'a> {
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        let tuple_count = *args;
        let mut cursor = data.cursor();
        let flags = cursor.read::<u32>()?;
        // Skip rowCount and columnCount
        cursor.advance_by(u16::RAW_BYTE_LEN * 2);
        // All offsets are relative to the parent subtable
        let row_index_table_offset = (cursor.read::<u32>()? as usize)
            .checked_sub(Subtable::HEADER_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        let column_index_table_offset = (cursor.read::<u32>()? as usize)
            .checked_sub(Subtable::HEADER_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        let kerning_array_offset = (cursor.read::<u32>()? as usize)
            .checked_sub(Subtable::HEADER_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        let kerning_vector = if tuple_count != 0 {
            let kerning_vector_offset = (cursor.read::<u32>()? as usize)
                .checked_sub(Subtable::HEADER_LEN)
                .ok_or(ReadError::OutOfBounds)?;
            Some(safe_read_array_to_end(&data, kerning_vector_offset)?)
        } else {
            None
        };
        let row_data = data
            .slice(row_index_table_offset..)
            .ok_or(ReadError::OutOfBounds)?;
        let column_data = data
            .slice(column_index_table_offset..)
            .ok_or(ReadError::OutOfBounds)?;
        if flags & 1 == 0 {
            let rows = LookupU16::read(row_data)?;
            let columns = LookupU16::read(column_data)?;
            let kerning_array = safe_read_array_to_end(&data, kerning_array_offset)?;
            Ok(Self::ShortValues(
                rows,
                columns,
                kerning_array,
                kerning_vector,
            ))
        } else {
            let rows = LookupU32::read(row_data)?;
            let columns = LookupU32::read(column_data)?;
            let kerning_array = safe_read_array_to_end(&data, kerning_array_offset)?;
            Ok(Self::LongValues(
                rows,
                columns,
                kerning_array,
                kerning_vector,
            ))
        }
    }
}

impl Subtable6<'_> {
    /// Returns the kerning adjustment for the given pair.
    pub fn kerning(&self, left: GlyphId, right: GlyphId) -> Option<i32> {
        let left: u16 = left.to_u32().try_into().ok()?;
        let right: u16 = right.to_u32().try_into().ok()?;
        fn tuple_kern(value: i32, vector: &Option<&[BigEndian<i16>]>) -> Option<i32> {
            if let Some(vector) = vector {
                vector
                    .get(value as usize >> 1)
                    .map(|value| value.get() as i32)
            } else {
                Some(value)
            }
        }
        match self {
            Self::ShortValues(rows, columns, array, vector) => {
                let left_idx = rows.value(left).unwrap_or_default();
                let right_idx = columns.value(right).unwrap_or_default();
                let idx = left_idx as usize + right_idx as usize;
                let value = array.get(idx).map(|value| value.get() as i32)?;
                tuple_kern(value, vector)
            }
            Self::LongValues(rows, columns, array, vector) => {
                let left_idx = rows.value(left).unwrap_or_default();
                let right_idx = columns.value(right).unwrap_or_default();
                let idx = (left_idx as usize).checked_add(right_idx as usize)?;
                let value = array.get(idx).map(|value| value.get())?;
                tuple_kern(value, vector)
            }
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
                1usize => Some(Field::new("tuple_count", self.tuple_count())),
                _ => None,
            }),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use font_test_data::bebuffer::BeBuffer;

    #[test]
    fn parse_subtable0() {
        let mut buf = BeBuffer::new();
        // n_pairs, bsearch params
        buf = buf.extend([6u32, 0, 0, 0]);
        // just some randomly generated pairs (left, right, kerning adjustment)
        let mut pairs = [
            (0u32, 1u32, -10i32),
            (2, 4, 22),
            (0, 3, -6),
            (8, 2, 500),
            (10, 1, 42),
            (9, 12, -1000),
        ];
        // pairs must be sorted by left and right packed into a u32
        pairs.sort_by_key(|pair| (pair.0 << 16) | pair.1);
        for pair in &pairs {
            buf = buf
                .push(pair.0 as u16)
                .push(pair.1 as u16)
                .push(pair.2 as i16);
        }
        let data = buf.to_vec();
        let subtable0 = Subtable0::read(FontData::new(&data)).unwrap();
        for pair in pairs {
            assert_eq!(
                subtable0.kerning(pair.0.into(), pair.1.into()),
                Some(pair.2)
            );
        }
    }

    #[test]
    fn parse_subtable1() {
        let data = FormatOneFour::One.build_subtable();
        let subtable1 = Subtable1::read(FontData::new(&data)).unwrap();
        let values = subtable1
            .values
            .iter()
            // The values array is unsized in the format so we need
            // to limit it for comparison
            .take(ONE_EXPECTED.len())
            .map(|value| value.get())
            .collect::<Vec<_>>();
        assert_eq!(values, &ONE_EXPECTED);
    }

    #[test]
    fn parse_subtable2() {
        let data = FormatTwoSix::Two.build_subtable();
        let subtable = Subtable2::read(FontData::new(&data)).unwrap();
        let mut values = vec![];
        for left in 0u32..4 {
            for right in 0u32..4 {
                let Some(kerning) = subtable.kerning(left.into(), right.into()) else {
                    panic!("expected kerning value for {left} and {right}");
                };
                values.push(kerning);
            }
        }
        assert_eq!(values, &TWO_SIX_EXPECTED);
    }

    #[test]
    fn parse_subtable4_control_points() {
        let data = FormatOneFour::FourControlPoints.build_subtable();
        let subtable4 = Subtable4::read(FontData::new(&data)).unwrap();
        let Subtable4Actions::ControlPoints(action) = &subtable4.actions else {
            panic!("expected subtable 4 control points action");
        };
        let values = action
            .chunks_exact(2)
            .take(FOUR_OUTLINE_ANKR_EXPECTED.len())
            .map(|values| (values[0].get(), values[1].get()))
            .collect::<Vec<_>>();
        assert_eq!(values, &FOUR_OUTLINE_ANKR_EXPECTED);
    }

    #[test]
    fn parse_subtable4_anchor_points() {
        let data = FormatOneFour::FourAnchorPoints.build_subtable();
        let subtable4 = Subtable4::read(FontData::new(&data)).unwrap();
        let Subtable4Actions::AnchorPoints(action) = &subtable4.actions else {
            panic!("expected subtable 4 anchor points action");
        };
        let values = action
            .chunks_exact(2)
            .take(FOUR_OUTLINE_ANKR_EXPECTED.len())
            .map(|values| (values[0].get(), values[1].get()))
            .collect::<Vec<_>>();
        assert_eq!(values, &FOUR_OUTLINE_ANKR_EXPECTED);
    }

    #[test]
    fn parse_subtable4_coords() {
        let data = FormatOneFour::FourCoords.build_subtable();
        let subtable4 = Subtable4::read(FontData::new(&data)).unwrap();
        let Subtable4Actions::ControlPointCoords(action) = &subtable4.actions else {
            panic!("expected subtable 4 coords action");
        };
        let values = action
            .chunks_exact(4)
            .take(FOUR_COORDS_EXPECTED.len())
            .map(|values| {
                [
                    values[0].get(),
                    values[1].get(),
                    values[2].get(),
                    values[3].get(),
                ]
            })
            .collect::<Vec<_>>();
        assert_eq!(values, &FOUR_COORDS_EXPECTED);
    }

    #[test]
    fn parse_subtable6_short() {
        let data = FormatTwoSix::SixShort.build_subtable();
        let subtable = Subtable6::read_with_args(FontData::new(&data), &0).unwrap();
        let Subtable6::ShortValues(..) = &subtable else {
            panic!("expected short values in subtable 6");
        };
        check_subtable6(subtable);
    }

    #[test]
    fn parse_subtable6_long() {
        let data = FormatTwoSix::SixLong.build_subtable();
        let subtable = Subtable6::read_with_args(FontData::new(&data), &0).unwrap();
        let Subtable6::LongValues(..) = &subtable else {
            panic!("expected long values in subtable 6");
        };
        check_subtable6(subtable);
    }

    #[test]
    fn parse_subtable6_long_vector() {
        let data = FormatTwoSix::SixLongVector.build_subtable();
        let subtable = Subtable6::read_with_args(FontData::new(&data), &1).unwrap();
        let Subtable6::LongValues(..) = &subtable else {
            panic!("expected long values in subtable 6");
        };
        check_subtable6(subtable);
    }

    fn check_subtable6(subtable: Subtable6) {
        let mut values = vec![];
        for left in 0u32..4 {
            for right in 0u32..4 {
                let Some(kerning) = subtable.kerning(left.into(), right.into()) else {
                    panic!("expected kerning value for {left} and {right}");
                };
                values.push(kerning);
            }
        }
        assert_eq!(values, &TWO_SIX_EXPECTED);
    }

    // Just kerning adjustment values
    const ONE_EXPECTED: [i16; 8] = [-40, -20, -10, 0, 10, 20, 40, 80];

    // Mark/Current glyph indices. Either outline points or indices into the ankr
    // table depending on format 4 action type.
    const FOUR_OUTLINE_ANKR_EXPECTED: [(u16, u16); 4] = [(0, 2), (2, 4), (4, 8), (8, 16)];

    // Mark/Current xy coordinates
    const FOUR_COORDS_EXPECTED: [[i16; 4]; 4] = [
        [-10, 10, -20, 20],
        [1, 2, 3, 4],
        [-1, -2, -3, -4],
        [10, -10, 20, -20],
    ];

    enum FormatOneFour {
        One,
        FourControlPoints,
        FourAnchorPoints,
        FourCoords,
    }

    impl FormatOneFour {
        fn build_subtable(&self) -> Vec<u8> {
            let mut flags_offset = ExtendedStateTable::<()>::HEADER_LEN + u32::RAW_BYTE_LEN;
            // Low bits are offset. Set the action type for format 4.
            match self {
                Self::FourAnchorPoints => {
                    flags_offset |= 1 << 30;
                }
                Self::FourCoords => {
                    flags_offset |= 2 << 30;
                }
                _ => {}
            }
            let mut buf = BeBuffer::new();
            buf = buf.push(flags_offset as u32);
            // Now add some data depending on the format
            match self {
                Self::One => {
                    buf = buf.extend(ONE_EXPECTED);
                }
                Self::FourControlPoints | Self::FourAnchorPoints => {
                    for indices in FOUR_OUTLINE_ANKR_EXPECTED {
                        buf = buf.push(indices.0).push(indices.1);
                    }
                }
                Self::FourCoords => {
                    for coords in FOUR_COORDS_EXPECTED {
                        buf = buf.extend(coords);
                    }
                }
            }
            let payload = buf.to_vec();
            let payload_len = payload.len() as u32;
            #[rustfmt::skip]
            let header = [
                6_u32, // number of classes
                payload_len + 16, // byte offset to class table
                payload_len + 52, // byte offset to state array
                payload_len + 88, // byte offset to entry array
            ];
            #[rustfmt::skip]
            let class_table = [
                6_u16, // format
                4,     // unit size (4 bytes)
                5,     // number of units
                16,    // search range
                2,     // entry selector
                0,     // range shift
                50, 4, // Input glyph 50 maps to class 4
                51, 4, // Input glyph 51 maps to class 4
                80, 5, // Input glyph 80 maps to class 5
                201, 4, // Input glyph 201 maps to class 4
                202, 4, // Input glyph 202 maps to class 4
                !0, !0
            ];
            #[rustfmt::skip]
            let state_array: [u16; 18] = [
                0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 2, 1,
            ];
            #[rustfmt::skip]
            let entry_table: [u16; 9] = [
                0, 0, 1,
                2, 0, 2,
                0, 0, 3,
            ];
            BeBuffer::new()
                .extend(header)
                .extend(payload)
                .extend(class_table)
                .extend(state_array)
                .extend(entry_table)
                .to_vec()
        }
    }

    const TWO_SIX_EXPECTED: [i32; 16] =
        [0i32, 10, 20, 0, 8, 4, -2, 8, 30, -10, -20, 30, 8, 4, -2, 8];

    enum FormatTwoSix {
        Two,
        SixShort,
        SixLong,
        SixLongVector,
    }

    impl FormatTwoSix {
        fn is_long(&self) -> bool {
            matches!(self, Self::SixLong | Self::SixLongVector)
        }

        fn is_six(&self) -> bool {
            !matches!(self, Self::Two)
        }

        fn has_kerning_vector(&self) -> bool {
            matches!(self, Self::SixLongVector)
        }

        // Common helper for building format 2/6 subtables
        fn build_subtable(&self) -> Vec<u8> {
            let mut buf = BeBuffer::new();
            let row_count = 3u32;
            let column_count = 3u32;
            let is_long = self.is_long();
            let has_kerning_vector = self.has_kerning_vector();
            if self.is_six() {
                // flags, rowCount, columnCount
                buf = buf
                    .push(if is_long { 1u32 } else { 0u32 })
                    .push(row_count as u16)
                    .push(column_count as u16);
            } else {
                // rowWidth
                buf = buf.push(row_count);
            }
            // Map 4 glyphs
            // 0 => row 0, column 0
            // 1 => row 2, column 1
            // 2 => row 1, column 2
            // 3 => row 2, column 0
            // values in the row table are pre-multiplied by column count
            #[allow(clippy::erasing_op, clippy::identity_op)]
            let row_table = build_lookup(
                &[
                    0 * column_count,
                    2 * column_count,
                    1 * column_count,
                    2 * column_count,
                ],
                is_long,
            );
            let column_table = build_lookup(&[0, 1, 2, 0], is_long);
            // 3x3 kerning matrix
            let kerning_array = [0i32, 10, 20, 30, -10, -20, 8, 4, -2];
            let mut offset =
                Subtable::HEADER_LEN + u32::RAW_BYTE_LEN * if self.is_six() { 5 } else { 4 };
            if has_kerning_vector {
                // optional offset for kerning vector
                offset += 4;
            }
            // row table offset
            buf = buf.push(offset as u32);
            offset += row_table.len();
            // column table offset
            buf = buf.push(offset as u32);
            offset += column_table.len();
            // kerning array offset
            buf = buf.push(offset as u32);
            if has_kerning_vector {
                // 9 32-bit offsets
                offset += 9 * 4;
                // kerning vector offset
                buf = buf.push(offset as u32);
                buf = buf.extend(row_table);
                buf = buf.extend(column_table);
                // With a kerning vector, the kerning array becomes an offset array
                let offsets: [u32; 9] = core::array::from_fn(|idx| idx as u32 * 2);
                buf = buf.extend(offsets);
                // And the value array is always 16-bit
                for value in &kerning_array {
                    buf = buf.push(*value as i16);
                }
            } else {
                buf = buf.extend(row_table);
                buf = buf.extend(column_table);
                if is_long {
                    buf = buf.extend(kerning_array);
                } else {
                    for value in &kerning_array {
                        buf = buf.push(*value as i16);
                    }
                }
            }
            buf.to_vec()
        }
    }

    // Builds a simple lookup table mapping the specified slice from
    // index -> value.
    // If `is_long` is true, builds a 32-bit lookup table, otherwise
    // builds a 16-bit table.
    fn build_lookup(values: &[u32], is_long: bool) -> Vec<u8> {
        let mut buf = BeBuffer::new();
        // format
        buf = buf.push(0u16);
        for value in values {
            if is_long {
                buf = buf.push(*value);
            } else {
                buf = buf.push(*value as u16);
            }
        }
        buf.to_vec()
    }
}
