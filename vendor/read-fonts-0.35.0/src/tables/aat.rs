//! Apple Advanced Typography common tables.
//!
//! See <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html>

include!("../../generated/generated_aat.rs");

/// Predefined classes.
///
/// See <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html>
pub mod class {
    pub const END_OF_TEXT: u8 = 0;
    pub const OUT_OF_BOUNDS: u8 = 1;
    pub const DELETED_GLYPH: u8 = 2;
}

impl Lookup0<'_> {
    pub fn values<T: LookupValue>(&self) -> Result<&[BigEndian<T>], ReadError> {
        let data = self.values_data();
        let data_len = data.len();
        let n_elems = data_len / T::RAW_BYTE_LEN;
        let len_in_bytes = n_elems * T::RAW_BYTE_LEN;
        FontData::new(&data[..len_in_bytes])
            .cursor()
            .read_array::<BigEndian<T>>(n_elems)
    }
    pub fn value<T: LookupValue>(&self, index: u16) -> Result<T, ReadError> {
        self.values::<T>()?
            .get(index as usize)
            .map(|val| val.get())
            .ok_or(ReadError::OutOfBounds)
    }
}

/// Lookup segment for format 2.
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
#[repr(C, packed)]
pub struct LookupSegment2<T>
where
    T: LookupValue,
{
    /// Last glyph index in this segment.
    pub last_glyph: BigEndian<u16>,
    /// First glyph index in this segment.
    pub first_glyph: BigEndian<u16>,
    /// The lookup value.
    pub value: BigEndian<T>,
}

/// Note: this requires `LookupSegment2` to be `repr(packed)`.
impl<T: LookupValue> FixedSize for LookupSegment2<T> {
    const RAW_BYTE_LEN: usize = std::mem::size_of::<Self>();
}

impl Lookup2<'_> {
    pub fn value<T: LookupValue>(&self, index: u16) -> Result<T, ReadError> {
        let segments = self.segments::<T>()?;
        let ix = match segments.binary_search_by(|segment| segment.first_glyph.get().cmp(&index)) {
            Ok(ix) => ix,
            Err(ix) => ix.saturating_sub(1),
        };
        let segment = segments.get(ix).ok_or(ReadError::OutOfBounds)?;
        if (segment.first_glyph.get()..=segment.last_glyph.get()).contains(&index) {
            let value = segment.value;
            return Ok(value.get());
        }
        Err(ReadError::OutOfBounds)
    }

    pub fn segments<T: LookupValue>(&self) -> Result<&[LookupSegment2<T>], ReadError> {
        FontData::new(self.segments_data())
            .cursor()
            .read_array(self.n_units() as usize)
    }
}

impl Lookup4<'_> {
    pub fn value<T: LookupValue>(&self, index: u16) -> Result<T, ReadError> {
        let segments = self.segments();
        let ix = match segments.binary_search_by(|segment| segment.first_glyph.get().cmp(&index)) {
            Ok(ix) => ix,
            Err(ix) => ix.saturating_sub(1),
        };
        let segment = segments.get(ix).ok_or(ReadError::OutOfBounds)?;
        if (segment.first_glyph.get()..=segment.last_glyph.get()).contains(&index) {
            let base_offset = segment.value_offset() as usize;
            let offset = base_offset
                + index
                    .checked_sub(segment.first_glyph())
                    .ok_or(ReadError::OutOfBounds)? as usize
                    * T::RAW_BYTE_LEN;
            return self.offset_data().read_at(offset);
        }
        Err(ReadError::OutOfBounds)
    }
    pub fn segment_values<T: LookupValue>(
        &self,
        segment: usize,
    ) -> Result<&[BigEndian<T>], ReadError> {
        let segment = self.segments().get(segment).ok_or(ReadError::OutOfBounds)?;
        let base_offset = segment.value_offset() as usize;
        let n_elems = segment
            .last_glyph
            .get()
            .checked_sub(segment.first_glyph.get())
            .ok_or(ReadError::MalformedData(
                "invalid segment in format 4 AAT lookup table",
            ))? as usize
            + 1;
        self.offset_data()
            .read_array::<BigEndian<T>>(base_offset..base_offset + n_elems * T::RAW_BYTE_LEN)
    }
}

/// Lookup single record for format 6.
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
#[repr(C, packed)]
pub struct LookupSingle<T>
where
    T: LookupValue,
{
    /// The glyph index.
    pub glyph: BigEndian<u16>,
    /// The lookup value.
    pub value: BigEndian<T>,
}

/// Note: this requires `LookupSingle` to be `repr(packed)`.
impl<T: LookupValue> FixedSize for LookupSingle<T> {
    const RAW_BYTE_LEN: usize = std::mem::size_of::<Self>();
}

impl Lookup6<'_> {
    pub fn value<T: LookupValue>(&self, index: u16) -> Result<T, ReadError> {
        let entries = self.entries::<T>()?;
        if let Ok(ix) = entries.binary_search_by_key(&index, |entry| entry.glyph.get()) {
            let entry = &entries[ix];
            let value = entry.value;
            return Ok(value.get());
        }
        Err(ReadError::OutOfBounds)
    }

    pub fn entries<T: LookupValue>(&self) -> Result<&[LookupSingle<T>], ReadError> {
        FontData::new(self.entries_data())
            .cursor()
            .read_array(self.n_units() as usize)
    }
}

impl Lookup8<'_> {
    pub fn value<T: LookupValue>(&self, index: u16) -> Result<T, ReadError> {
        index
            .checked_sub(self.first_glyph())
            .and_then(|ix| {
                self.value_array()
                    .get(ix as usize)
                    .map(|val| T::from_u16(val.get()))
            })
            .ok_or(ReadError::OutOfBounds)
    }
}

impl Lookup10<'_> {
    pub fn value<T: LookupValue>(&self, index: u16) -> Result<T, ReadError> {
        let ix = index
            .checked_sub(self.first_glyph())
            .ok_or(ReadError::OutOfBounds)? as usize;
        let unit_size = self.unit_size() as usize;
        let offset = ix * unit_size;
        let mut cursor = FontData::new(self.values_data()).cursor();
        cursor.advance_by(offset);
        let val = match unit_size {
            1 => cursor.read::<u8>()? as u32,
            2 => cursor.read::<u16>()? as u32,
            4 => cursor.read::<u32>()?,
            _ => {
                return Err(ReadError::MalformedData(
                    "invalid unit_size in format 10 AAT lookup table",
                ))
            }
        };
        Ok(T::from_u32(val))
    }
}

impl Lookup<'_> {
    pub fn value<T: LookupValue>(&self, index: u16) -> Result<T, ReadError> {
        match self {
            Lookup::Format0(lookup) => lookup.value::<T>(index),
            Lookup::Format2(lookup) => lookup.value::<T>(index),
            Lookup::Format4(lookup) => lookup.value::<T>(index),
            Lookup::Format6(lookup) => lookup.value::<T>(index),
            Lookup::Format8(lookup) => lookup.value::<T>(index),
            Lookup::Format10(lookup) => lookup.value::<T>(index),
        }
    }
}

#[derive(Clone)]
pub struct TypedLookup<'a, T> {
    pub lookup: Lookup<'a>,
    _marker: std::marker::PhantomData<fn() -> T>,
}

impl<T: LookupValue> TypedLookup<'_, T> {
    /// Returns the value associated with the given index.
    pub fn value(&self, index: u16) -> Result<T, ReadError> {
        self.lookup.value::<T>(index)
    }
}

impl<'a, T> FontRead<'a> for TypedLookup<'a, T> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        Ok(Self {
            lookup: Lookup::read(data)?,
            _marker: std::marker::PhantomData,
        })
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a, T> SomeTable<'a> for TypedLookup<'a, T> {
    fn type_name(&self) -> &str {
        "TypedLookup"
    }

    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        self.lookup.get_field(idx)
    }
}

/// Trait for values that can be read from lookup tables.
pub trait LookupValue: Copy + Scalar + bytemuck::AnyBitPattern {
    fn from_u16(v: u16) -> Self;
    fn from_u32(v: u32) -> Self;
}

impl LookupValue for u16 {
    fn from_u16(v: u16) -> Self {
        v
    }

    fn from_u32(v: u32) -> Self {
        // intentionally truncates
        v as _
    }
}

impl LookupValue for u32 {
    fn from_u16(v: u16) -> Self {
        v as _
    }

    fn from_u32(v: u32) -> Self {
        v
    }
}

impl LookupValue for GlyphId16 {
    fn from_u16(v: u16) -> Self {
        GlyphId16::from(v)
    }

    fn from_u32(v: u32) -> Self {
        // intentionally truncates
        GlyphId16::from(v as u16)
    }
}

pub type LookupU16<'a> = TypedLookup<'a, u16>;
pub type LookupU32<'a> = TypedLookup<'a, u32>;
pub type LookupGlyphId<'a> = TypedLookup<'a, GlyphId16>;

/// Empty data type for a state table entry with no payload.
///
/// Note: this type is only intended for use as the type parameter for
/// `StateEntry`. The inner field is private and this type cannot be
/// constructed outside of this module.
#[derive(Copy, Clone, bytemuck::AnyBitPattern, Debug)]
pub struct NoPayload(());

impl FixedSize for NoPayload {
    const RAW_BYTE_LEN: usize = 0;
}

/// Entry in an (extended) state table.
#[derive(Clone, Debug)]
pub struct StateEntry<T = NoPayload> {
    /// Index of the next state.
    pub new_state: u16,
    /// Flag values are table specific.
    pub flags: u16,
    /// Payload is table specific.
    pub payload: T,
}

impl<'a, T: bytemuck::AnyBitPattern + FixedSize> FontRead<'a> for StateEntry<T> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let new_state = cursor.read()?;
        let flags = cursor.read()?;
        let remaining = cursor.remaining().ok_or(ReadError::OutOfBounds)?;
        let payload = *remaining.read_ref_at(0)?;
        Ok(Self {
            new_state,
            flags,
            payload,
        })
    }
}

impl<T> FixedSize for StateEntry<T>
where
    T: FixedSize,
{
    // Two u16 fields + payload
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN + u16::RAW_BYTE_LEN + T::RAW_BYTE_LEN;
}

/// Table for driving a finite state machine for layout.
///
/// The input to the state machine consists of the current state
/// and a glyph class. The output is an [entry](StateEntry) containing
/// the next state and a payload that is dependent on the type of
/// layout action being performed.
///
/// See <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html#StateHeader>
/// for more detail.
#[derive(Clone)]
pub struct StateTable<'a> {
    pub header: StateHeader<'a>,
    n_classes: usize,
    class_first_glyph: u16,
    class_array: &'a [u8],
    state_array: &'a [u8],
    entry_table: &'a [u8],
}

impl StateTable<'_> {
    pub const HEADER_LEN: usize = u16::RAW_BYTE_LEN * 4;

    /// Returns the class table entry for the given glyph identifier.
    pub fn class(&self, glyph_id: GlyphId16) -> Result<u8, ReadError> {
        let glyph_id = glyph_id.to_u16();
        if glyph_id == 0xFFFF {
            return Ok(class::DELETED_GLYPH);
        }
        glyph_id
            .checked_sub(self.class_first_glyph)
            .and_then(|ix| self.class_array.get(ix as usize).copied())
            .ok_or(ReadError::OutOfBounds)
    }

    /// Returns the entry for the given state and class.
    #[inline(always)]
    pub fn entry(&self, state: u16, class: u8) -> Result<StateEntry, ReadError> {
        let mut class = class as usize;
        if class >= self.n_classes {
            class = class::OUT_OF_BOUNDS as usize;
        }
        let entry_ix = self
            .state_array
            .get(state as usize * self.n_classes + class)
            .copied()
            .ok_or(ReadError::OutOfBounds)? as usize;
        let entry_offset = entry_ix * 4;
        let entry_data = self
            .entry_table
            .get(entry_offset..)
            .ok_or(ReadError::OutOfBounds)?;
        let mut entry = StateEntry::read(FontData::new(entry_data))?;
        // For legacy state tables, the newState is a byte offset into
        // the state array. Convert this to an index for consistency.
        let new_state = (entry.new_state as i32)
            .checked_sub(self.header.state_array_offset().to_u32() as i32)
            .ok_or(ReadError::OutOfBounds)?
            / self.n_classes as i32;
        entry.new_state = new_state.try_into().map_err(|_| ReadError::OutOfBounds)?;
        Ok(entry)
    }

    /// Reads scalar values that are referenced from state table entries.
    pub fn read_value<T: Scalar>(&self, offset: usize) -> Result<T, ReadError> {
        self.header.offset_data().read_at::<T>(offset)
    }
}

impl<'a> FontRead<'a> for StateTable<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let header = StateHeader::read(data)?;
        // Each state has a 1-byte entry per class so state_size == n_classes
        let n_classes = header.state_size() as usize;
        if n_classes == 0 {
            // This will result in a divide by 0 in all cases
            return Err(ReadError::MalformedData("empty AAT state table"));
        }
        let class_table = header.class_table()?;
        let class_first_glyph = class_table.first_glyph();
        let class_array = class_table.class_array();
        let state_array = header.state_array()?.data();
        let entry_table = header.entry_table()?.data();
        Ok(Self {
            header: StateHeader::read(data)?,
            n_classes,
            class_first_glyph,
            class_array,
            state_array,
            entry_table,
        })
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for StateTable<'a> {
    fn type_name(&self) -> &str {
        "StateTable"
    }

    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        self.header.get_field(idx)
    }
}

#[derive(Clone)]
pub struct ExtendedStateTable<'a, T = NoPayload> {
    pub n_classes: usize,
    pub class_table: LookupU16<'a>,
    state_array: &'a [BigEndian<u16>],
    entry_table: &'a [u8],
    _marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> ExtendedStateTable<'_, T> {
    pub const HEADER_LEN: usize = u32::RAW_BYTE_LEN * 4;
}

/// Table for driving a finite state machine for layout.
///
/// The input to the state machine consists of the current state
/// and a glyph class. The output is an [entry](StateEntry) containing
/// the next state and a payload that is dependent on the type of
/// layout action being performed.
///
/// See <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html#StateHeader>
/// for more detail.
impl<T> ExtendedStateTable<'_, T>
where
    T: FixedSize + bytemuck::AnyBitPattern,
{
    /// Returns the class table entry for the given glyph identifier.
    pub fn class(&self, glyph_id: GlyphId) -> Result<u16, ReadError> {
        let glyph_id: u16 = glyph_id
            .to_u32()
            .try_into()
            .map_err(|_| ReadError::OutOfBounds)?;
        if glyph_id == 0xFFFF {
            return Ok(class::DELETED_GLYPH as u16);
        }
        self.class_table.value(glyph_id)
    }

    /// Returns the entry for the given state and class.
    pub fn entry(&self, state: u16, class: u16) -> Result<StateEntry<T>, ReadError> {
        let mut class = class as usize;
        if class >= self.n_classes {
            class = class::OUT_OF_BOUNDS as usize;
        }
        let state_ix = state as usize * self.n_classes + class;
        let entry_ix = self
            .state_array
            .get(state_ix)
            .copied()
            .ok_or(ReadError::OutOfBounds)?
            .get() as usize;
        let entry_offset = entry_ix * StateEntry::<T>::RAW_BYTE_LEN;
        let entry_data = self
            .entry_table
            .get(entry_offset..)
            .ok_or(ReadError::OutOfBounds)?;
        StateEntry::read(FontData::new(entry_data))
    }
}

impl<'a, T> FontRead<'a> for ExtendedStateTable<'a, T> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let header = StxHeader::read(data)?;
        let n_classes = header.n_classes() as usize;
        let class_table = header.class_table()?;
        let state_array = header.state_array()?.data();
        let entry_table = header.entry_table()?.data();
        Ok(Self {
            n_classes,
            class_table,
            state_array,
            entry_table,
            _marker: std::marker::PhantomData,
        })
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a, T> SomeTable<'a> for ExtendedStateTable<'a, T> {
    fn type_name(&self) -> &str {
        "ExtendedStateTable"
    }

    fn get_field(&self, _idx: usize) -> Option<Field<'a>> {
        None
    }
}

/// Reads an array of T from the given FontData, ensuring that the byte length
/// is a multiple of the size of T.
///
/// Many of the `morx` subtables have arrays without associated lengths so we
/// simply read to the end of the available data. The `FontData::read_array`
/// method will fail if the byte range provided is not exact so this helper
/// allows us to force the lengths to an acceptable value.
pub(crate) fn safe_read_array_to_end<'a, T: bytemuck::AnyBitPattern + FixedSize>(
    data: &FontData<'a>,
    offset: usize,
) -> Result<&'a [T], ReadError> {
    let len = data
        .len()
        .checked_sub(offset)
        .ok_or(ReadError::OutOfBounds)?;
    let end = offset + len / T::RAW_BYTE_LEN * T::RAW_BYTE_LEN;
    data.read_array(offset..end)
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;

    #[test]
    fn lookup_format_0() {
        #[rustfmt::skip]
        let words = [
            0_u16, // format
            0, 2, 4, 6, 8, 10, 12, 14, 16, // maps all glyphs to gid * 2
        ];
        let mut buf = BeBuffer::new();
        buf = buf.extend(words);
        let lookup = LookupU16::read(buf.data().into()).unwrap();
        for gid in 0..=8 {
            assert_eq!(lookup.value(gid).unwrap(), gid * 2);
        }
        assert!(lookup.value(9).is_err());
    }

    // Taken from example 2 at https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html
    #[test]
    fn lookup_format_2() {
        #[rustfmt::skip]
        let words = [
            2_u16, // format
            6,     // unit size (6 bytes)
            3,     // number of units
            12,    // search range
            1,     // entry selector
            6,     // range shift
            22, 20, 4, // First segment, mapping glyphs 20 through 22 to class 4
            24, 23, 5, // Second segment, mapping glyph 23 and 24 to class 5
            28, 25, 6, // Third segment, mapping glyphs 25 through 28 to class 6
        ];
        let mut buf = BeBuffer::new();
        buf = buf.extend(words);
        let lookup = LookupU16::read(buf.data().into()).unwrap();
        let expected = [(20..=22, 4), (23..=24, 5), (25..=28, 6)];
        for (range, class) in expected {
            for gid in range {
                assert_eq!(lookup.value(gid).unwrap(), class);
            }
        }
        for fail in [0, 10, 19, 29, 0xFFFF] {
            assert!(lookup.value(fail).is_err());
        }
    }

    #[test]
    fn lookup_format_4() {
        #[rustfmt::skip]
        let words = [
            4_u16, // format
            6,     // unit size (6 bytes)
            3,     // number of units
            12,    // search range
            1,     // entry selector
            6,     // range shift
            22, 20, 30, // First segment, mapping glyphs 20 through 22 to mapped data at offset 30
            24, 23, 36, // Second segment, mapping glyph 23 and 24 to mapped data at offset 36
            28, 25, 40, // Third segment, mapping glyphs 25 through 28 to mapped data at offset 40
            // mapped data
            3, 2, 1,
            100, 150,
            8, 6, 7, 9
        ];
        let mut buf = BeBuffer::new();
        buf = buf.extend(words);
        let lookup = LookupU16::read(buf.data().into()).unwrap();
        let expected = [
            (20, 3),
            (21, 2),
            (22, 1),
            (23, 100),
            (24, 150),
            (25, 8),
            (26, 6),
            (27, 7),
            (28, 9),
        ];
        for (in_glyph, out_glyph) in expected {
            assert_eq!(lookup.value(in_glyph).unwrap(), out_glyph);
        }
        for fail in [0, 10, 19, 29, 0xFFFF] {
            assert!(lookup.value(fail).is_err());
        }
    }

    // Taken from example 1 at https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6morx.html
    #[test]
    fn lookup_format_6() {
        #[rustfmt::skip]
        let words = [
            6_u16, // format
            4,     // unit size (4 bytes)
            4,     // number of units
            16,    // search range
            2,     // entry selector
            0,     // range shift
            50, 600, // Input glyph 50 maps to glyph 600
            51, 601, // Input glyph 51 maps to glyph 601
            201, 602, // Input glyph 201 maps to glyph 602
            202, 900, // Input glyph 202 maps to glyph 900
        ];
        let mut buf = BeBuffer::new();
        buf = buf.extend(words);
        let lookup = LookupU16::read(buf.data().into()).unwrap();
        let expected = [(50, 600), (51, 601), (201, 602), (202, 900)];
        for (in_glyph, out_glyph) in expected {
            assert_eq!(lookup.value(in_glyph).unwrap(), out_glyph);
        }
        for fail in [0, 10, 49, 52, 203, 0xFFFF] {
            assert!(lookup.value(fail).is_err());
        }
    }

    #[test]
    fn lookup_format_8() {
        #[rustfmt::skip]
        let words = [
            8_u16, // format
            201,   // first glyph
            7,     // glyph count
            3, 8, 2, 9, 1, 200, 60, // glyphs 201..209 mapped to these values
        ];
        let mut buf = BeBuffer::new();
        buf = buf.extend(words);
        let lookup = LookupU16::read(buf.data().into()).unwrap();
        let expected = &words[3..];
        for (gid, expected) in (201..209).zip(expected) {
            assert_eq!(lookup.value(gid).unwrap(), *expected);
        }
        for fail in [0, 10, 200, 210, 0xFFFF] {
            assert!(lookup.value(fail).is_err());
        }
    }

    #[test]
    fn lookup_format_10() {
        #[rustfmt::skip]
        let words = [
            10_u16, // format
            4,      // unit size, use 4 byte values
            201,   // first glyph
            7,     // glyph count
        ];
        // glyphs 201..209 mapped to these values
        let mapped = [3_u32, 8, 2902384, 9, 1, u32::MAX, 60];
        let mut buf = BeBuffer::new();
        buf = buf.extend(words).extend(mapped);
        let lookup = LookupU32::read(buf.data().into()).unwrap();
        for (gid, expected) in (201..209).zip(mapped) {
            assert_eq!(lookup.value(gid).unwrap(), expected);
        }
        for fail in [0, 10, 200, 210, 0xFFFF] {
            assert!(lookup.value(fail).is_err());
        }
    }

    #[test]
    fn extended_state_table() {
        #[rustfmt::skip]
        let header = [
            6_u32, // number of classes
            20, // byte offset to class table
            56, // byte offset to state array
            92, // byte offset to entry array
            0, // padding
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
        let entry_table: [u16; 12] = [
            0, 0, u16::MAX, u16::MAX,
            2, 0, u16::MAX, u16::MAX,
            0, 0, u16::MAX, 0,
        ];
        let buf = BeBuffer::new()
            .extend(header)
            .extend(class_table)
            .extend(state_array)
            .extend(entry_table);
        let table = ExtendedStateTable::<ContextualData>::read(buf.data().into()).unwrap();
        // check class lookups
        let [class_50, class_80, class_201] =
            [50, 80, 201].map(|gid| table.class(GlyphId::new(gid)).unwrap());
        assert_eq!(class_50, 4);
        assert_eq!(class_80, 5);
        assert_eq!(class_201, 4);
        // initial state
        let entry = table.entry(0, 4).unwrap();
        assert_eq!(entry.new_state, 0);
        assert_eq!(entry.payload.current_index, !0);
        // entry (state 0, class 5) should transition to state 2
        let entry = table.entry(0, 5).unwrap();
        assert_eq!(entry.new_state, 2);
        // from state 2, we transition back to state 0 when class is not 5
        // this also enables an action (payload.current_index != -1)
        let entry = table.entry(2, 4).unwrap();
        assert_eq!(entry.new_state, 0);
        assert_eq!(entry.payload.current_index, 0);
    }

    #[derive(Copy, Clone, Debug, bytemuck::AnyBitPattern)]
    #[repr(C, packed)]
    struct ContextualData {
        _mark_index: BigEndian<u16>,
        current_index: BigEndian<u16>,
    }

    impl FixedSize for ContextualData {
        const RAW_BYTE_LEN: usize = 4;
    }

    // Take from example at <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6kern.html>
    // with class table trimmed to 4 glyphs
    #[test]
    fn state_table() {
        #[rustfmt::skip]
        let header = [
            7_u16, // number of classes
            10, // byte offset to class table
            18, // byte offset to state array
            40, // byte offset to entry array
            64, // byte offset to value array (unused here)
        ];
        #[rustfmt::skip]
        let class_table = [
            3_u16, // first glyph
            4, // number of glyphs
        ];
        let classes = [1u8, 2, 3, 4];
        #[rustfmt::skip]
        let state_array: [u8; 22] = [
            2, 0, 0, 2, 1, 0, 0,
            2, 0, 0, 2, 1, 0, 0,
            2, 3, 3, 2, 3, 4, 5,
            0, // padding
        ];
        #[rustfmt::skip]
        let entry_table: [u16; 10] = [
            // The first column are offsets from the beginning of the state
            // table to some position in the state array
            18, 0x8112,
            32, 0x8112,
            18, 0x0000,
            32, 0x8114,
            18, 0x8116,
        ];
        let buf = BeBuffer::new()
            .extend(header)
            .extend(class_table)
            .extend(classes)
            .extend(state_array)
            .extend(entry_table);
        let table = StateTable::read(buf.data().into()).unwrap();
        // check class lookups
        for i in 0..4u8 {
            assert_eq!(table.class(GlyphId16::from(i as u16 + 3)).unwrap(), i + 1);
        }
        // (state, class) -> (new_state, flags)
        let cases = [
            ((0, 4), (2, 0x8112)),
            ((2, 1), (2, 0x8114)),
            ((1, 3), (0, 0x0000)),
            ((2, 5), (0, 0x8116)),
        ];
        for ((state, class), (new_state, flags)) in cases {
            let entry = table.entry(state, class).unwrap();
            assert_eq!(
                entry.new_state, new_state,
                "state {state}, class {class} should map to new state {new_state} (got {})",
                entry.new_state
            );
            assert_eq!(
                entry.flags, flags,
                "state {state}, class {class} should map to flags 0x{flags:X} (got 0x{:X})",
                entry.flags
            );
        }
    }
}
