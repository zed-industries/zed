//! Experimental generic traversal of font tables.
//!
//! This module defines functionality that allows untyped access to font table
//! data. This is used as the basis for things like debug printing.
//!
//! The basis of traversal is the [`SomeTable`] trait, which is implemented for
//! all font tables. This trait provides the table's name, as well as ordered access
//! to the table's fields. Using this, it is possible to iterate through a table
//! and its subtables, records, and values.
//!
//! # Warning
//!
//! This functionality is considered experimental, and the API may break or be
//! removed without warning.

use std::{fmt::Debug, ops::Deref};

use types::{
    BigEndian, F2Dot14, FWord, Fixed, GlyphId16, Int24, LongDateTime, MajorMinor, NameId, Nullable,
    Offset16, Offset24, Offset32, Scalar, Tag, UfWord, Uint24, Version16Dot16,
};

use crate::{
    array::{ComputedArray, VarLenArray},
    read::{ComputeSize, ReadArgs},
    FontData, FontRead, FontReadWithArgs, ReadError, VarSize,
};

/// Types of fields in font tables.
///
/// Fields can either be scalars, offsets to tables, or arrays.
pub enum FieldType<'a> {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I24(Int24),
    U24(Uint24),
    Tag(Tag),
    FWord(FWord),
    UfWord(UfWord),
    MajorMinor(MajorMinor),
    Version16Dot16(Version16Dot16),
    F2Dot14(F2Dot14),
    Fixed(Fixed),
    LongDateTime(LongDateTime),
    GlyphId16(GlyphId16),
    NameId(NameId),
    BareOffset(OffsetType),
    ResolvedOffset(ResolvedOffset<'a>),
    /// Used in tables like name/post so we can actually print the strings
    StringOffset(StringOffset<'a>),
    /// Used in COLR/CPAL
    ArrayOffset(ArrayOffset<'a>),
    Record(RecordResolver<'a>),
    Array(Box<dyn SomeArray<'a> + 'a>),
    Unknown,
}

/// Any offset type.
#[derive(Clone, Copy)]
pub enum OffsetType {
    Offset16(u16),
    Offset24(Uint24),
    Offset32(u32),
}

impl OffsetType {
    /// Return this offset as a u32.
    pub fn to_u32(self) -> u32 {
        match self {
            Self::Offset16(val) => val.into(),
            Self::Offset24(val) => val.into(),
            Self::Offset32(val) => val,
        }
    }
}

/// An offset, as well as the table it references.
pub struct ResolvedOffset<'a> {
    /// The raw offset
    pub offset: OffsetType,
    /// The parsed table pointed to by this offset, or an error if parsing fails.
    pub target: Result<Box<dyn SomeTable<'a> + 'a>, ReadError>,
}

/// An offset to string data.
///
/// This is a special case for the name table (and maybe elsewhere?)
pub struct StringOffset<'a> {
    pub offset: OffsetType,
    pub target: Result<Box<dyn SomeString<'a> + 'a>, ReadError>,
}

/// An offset to an array.
pub struct ArrayOffset<'a> {
    pub offset: OffsetType,
    pub target: Result<Box<dyn SomeArray<'a> + 'a>, ReadError>,
}

pub(crate) struct ArrayOfOffsets<'a, O> {
    type_name: &'static str,
    offsets: &'a [O],
    resolver: Box<dyn Fn(&O) -> FieldType<'a> + 'a>,
}

impl<'a, O> SomeArray<'a> for ArrayOfOffsets<'a, O> {
    fn type_name(&self) -> &str {
        self.type_name
    }

    fn len(&self) -> usize {
        self.offsets.len()
    }

    fn get(&self, idx: usize) -> Option<FieldType<'a>> {
        let off = self.offsets.get(idx)?;
        let target = (self.resolver)(off);
        Some(target)
    }
}

impl<'a> FieldType<'a> {
    /// makes a field, handling the case where this array may not be present in
    /// all versions
    pub fn array_of_records<T>(
        type_name: &'static str,
        records: &'a [T],
        data: FontData<'a>,
    ) -> FieldType<'a>
    where
        T: Clone + SomeRecord<'a> + 'a,
    {
        ArrayOfRecords {
            type_name,
            data,
            records,
        }
        .into()
    }

    // Convenience method for handling computed arrays
    pub fn computed_array<T>(
        type_name: &'static str,
        array: ComputedArray<'a, T>,
        data: FontData<'a>,
    ) -> FieldType<'a>
    where
        T: FontReadWithArgs<'a> + ComputeSize + SomeRecord<'a> + 'a,
        T::Args: Copy + 'static,
    {
        ComputedArrayOfRecords {
            type_name,
            data,
            array,
        }
        .into()
    }

    // Convenience method for handling VarLenArrays
    pub fn var_array<T>(
        type_name: &'static str,
        array: VarLenArray<'a, T>,
        data: FontData<'a>,
    ) -> FieldType<'a>
    where
        T: FontRead<'a> + VarSize + SomeRecord<'a> + 'a,
    {
        VarLenArrayOfRecords {
            type_name,
            data,
            array,
        }
        .into()
    }

    /// Convenience method for creating a `FieldType` from an array of offsets.
    ///
    /// The `resolver` argument is a function that takes an offset and resolves
    /// it.
    pub fn array_of_offsets<O>(
        type_name: &'static str,
        offsets: &'a [O],
        resolver: impl Fn(&O) -> FieldType<'a> + 'a,
    ) -> Self
where {
        FieldType::Array(Box::new(ArrayOfOffsets {
            type_name,
            offsets,
            resolver: Box::new(resolver),
        }))
    }

    /// Convenience method for creating a `FieldType` from an offset to an array.
    pub fn offset_to_array_of_scalars<T: SomeArray<'a> + 'a>(
        offset: impl Into<OffsetType>,
        result: impl Into<Option<Result<T, ReadError>>>,
    ) -> Self {
        let offset = offset.into();
        match result.into() {
            Some(target) => FieldType::ArrayOffset(ArrayOffset {
                offset,
                target: target.map(|x| Box::new(x) as Box<dyn SomeArray>),
            }),
            None => FieldType::BareOffset(offset),
        }
    }

    /// Convenience method for creating a `FieldType` from an offset to an array.
    pub fn offset_to_array_of_records<T: Clone + SomeRecord<'a> + 'a>(
        offset: impl Into<OffsetType>,
        result: impl Into<Option<Result<&'a [T], ReadError>>>,
        type_name: &'static str,
        data: FontData<'a>,
    ) -> Self {
        let offset = offset.into();
        match result.into() {
            Some(target) => {
                let target = target.map(|records| {
                    Box::new(ArrayOfRecords {
                        type_name,
                        data,
                        records,
                    }) as Box<dyn SomeArray>
                });
                FieldType::ArrayOffset(ArrayOffset { offset, target })
            }
            None => FieldType::BareOffset(offset),
        }
    }

    //FIXME: I bet this is generating a *lot* of code
    /// Convenience method for creating a `FieldType` for a resolved offset.
    ///
    /// This handles cases where offsets are nullable, in which case the `result`
    /// argument may be `None`.
    pub fn offset<T: SomeTable<'a> + 'a>(
        offset: impl Into<OffsetType>,
        result: impl Into<Option<Result<T, ReadError>>>,
    ) -> Self {
        let offset = offset.into();
        match result.into() {
            Some(target) => FieldType::ResolvedOffset(ResolvedOffset {
                offset,
                target: target.map(|x| Box::new(x) as Box<dyn SomeTable>),
            }),
            None => FieldType::BareOffset(offset),
        }
    }

    /// Convenience method for creating a `FieldType` from an unknown offset.
    pub fn unknown_offset(offset: impl Into<OffsetType>) -> Self {
        Self::BareOffset(offset.into())
    }
}

/// A generic field in a font table.
pub struct Field<'a> {
    /// The field's name.
    pub name: &'static str,
    /// The field's value.
    pub value: FieldType<'a>,
}

/// A generic table type.
///
/// This is intended to be used as a trait object, and is a way of generically
/// representing any table, providing ordered access to that table's fields.
pub trait SomeTable<'a> {
    /// The name of this table
    fn type_name(&self) -> &str;
    /// Access this table's fields, in declaration order.
    fn get_field(&self, idx: usize) -> Option<Field<'a>>;
}

impl<'a> dyn SomeTable<'a> + 'a {
    /// Returns an iterator over this table's fields.
    pub fn iter(&self) -> impl Iterator<Item = Field<'a>> + '_ {
        FieldIter {
            table: self,
            idx: 0,
        }
    }
}

struct FieldIter<'a, 'b> {
    table: &'b dyn SomeTable<'a>,
    idx: usize,
}

impl<'a> Iterator for FieldIter<'a, '_> {
    type Item = Field<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let this = self.idx;
        self.idx += 1;
        self.table.get_field(this)
    }
}

impl<'a> SomeTable<'a> for Box<dyn SomeTable<'a> + 'a> {
    fn type_name(&self) -> &str {
        self.deref().type_name()
    }

    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        self.deref().get_field(idx)
    }
}

/// A generic trait for records, which need to be passed in data
/// in order to fully resolve themselves.
pub trait SomeRecord<'a> {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a>;
}

/// A struct created from a record and the data it needs to resolve any
/// contained offsets.
pub struct RecordResolver<'a> {
    pub(crate) name: &'static str,
    pub(crate) get_field: Box<dyn Fn(usize, FontData<'a>) -> Option<Field<'a>> + 'a>,
    pub(crate) data: FontData<'a>,
}

/// A generic trait for arrays.
pub trait SomeArray<'a> {
    /// The name of this type. For an array of u16s, this is `[u16]`.
    fn type_name(&self) -> &str;

    /// The length of the array.
    fn len(&self) -> usize;

    /// Returns `true` if this array is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the item at `idx`, or `None` if `idx` is out of bounds.
    fn get(&self, idx: usize) -> Option<FieldType<'a>>;
}

impl<'a> dyn SomeArray<'a> + 'a {
    /// Return an iterator over the contents of this array.
    pub fn iter(&self) -> impl Iterator<Item = FieldType<'a>> + '_ {
        ArrayIter {
            array: self,
            idx: 0,
        }
    }
}

struct ArrayIter<'a, 'b> {
    array: &'b dyn SomeArray<'a>,
    idx: usize,
}

impl<'a> Iterator for ArrayIter<'a, '_> {
    type Item = FieldType<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let this = self.idx;
        self.idx += 1;
        self.array.get(this)
    }
}

impl<'a, T: Scalar + Into<FieldType<'a>>> SomeArray<'a> for &'a [BigEndian<T>]
where
    BigEndian<T>: Copy, // i don't know why i need this??
{
    fn len(&self) -> usize {
        (*self).len()
    }

    fn get(&self, idx: usize) -> Option<FieldType<'a>> {
        (*self).get(idx).map(|val| val.get().into())
    }

    fn type_name(&self) -> &str {
        let full_name = std::any::type_name::<T>();
        full_name.split("::").last().unwrap_or(full_name)
    }
}

impl<'a> SomeArray<'a> for &'a [u8] {
    fn type_name(&self) -> &str {
        "u8"
    }

    fn len(&self) -> usize {
        (*self).len()
    }

    fn get(&self, idx: usize) -> Option<FieldType<'a>> {
        (*self).get(idx).copied().map(Into::into)
    }
}

impl<'a> SomeArray<'a> for Box<dyn SomeArray<'a> + 'a> {
    fn type_name(&self) -> &str {
        self.deref().type_name()
    }

    fn len(&self) -> usize {
        self.deref().len()
    }

    fn get(&self, idx: usize) -> Option<FieldType<'a>> {
        self.deref().get(idx)
    }
}

pub trait SomeString<'a> {
    fn iter_chars(&self) -> Box<dyn Iterator<Item = char> + 'a>;
}

impl<'a> SomeString<'a> for Box<dyn SomeString<'a> + 'a> {
    fn iter_chars(&self) -> Box<dyn Iterator<Item = char> + 'a> {
        self.deref().iter_chars()
    }
}

// only used as Box<dyn SomeArray<'a>>
struct ArrayOfRecords<'a, T> {
    pub(crate) type_name: &'static str,
    pub(crate) data: FontData<'a>,
    pub(crate) records: &'a [T],
}

// only used as Box<dyn SomeArray<'a>>
struct ComputedArrayOfRecords<'a, T: ReadArgs> {
    pub(crate) type_name: &'static str,
    pub(crate) data: FontData<'a>,
    pub(crate) array: ComputedArray<'a, T>,
}

struct VarLenArrayOfRecords<'a, T> {
    pub(crate) type_name: &'static str,
    pub(crate) data: FontData<'a>,
    pub(crate) array: VarLenArray<'a, T>,
}

impl<'a, T> SomeArray<'a> for ComputedArrayOfRecords<'a, T>
where
    T: FontReadWithArgs<'a> + ComputeSize + SomeRecord<'a> + 'a,
    T::Args: Copy + 'static,
    Self: 'a,
{
    fn len(&self) -> usize {
        self.array.len()
    }

    fn get(&self, idx: usize) -> Option<FieldType<'a>> {
        self.array
            .get(idx)
            .ok()
            .map(|record| record.traverse(self.data).into())
    }

    fn type_name(&self) -> &str {
        self.type_name
    }
}

impl<'a, T: SomeRecord<'a> + Clone> SomeArray<'a> for ArrayOfRecords<'a, T> {
    fn type_name(&self) -> &str {
        self.type_name
    }

    fn len(&self) -> usize {
        self.records.len()
    }

    fn get(&self, idx: usize) -> Option<FieldType<'a>> {
        self.records
            .get(idx)
            .map(|record| record.clone().traverse(self.data).into())
    }
}

impl<'a, T> SomeArray<'a> for VarLenArrayOfRecords<'a, T>
where
    T: FontRead<'a> + VarSize + SomeRecord<'a> + 'a,
    Self: 'a,
{
    fn len(&self) -> usize {
        self.array.iter().count()
    }

    fn get(&self, idx: usize) -> Option<FieldType<'a>> {
        self.array
            .get(idx)?
            .ok()
            .map(|record| record.traverse(self.data).into())
    }

    fn type_name(&self) -> &str {
        self.type_name
    }
}

impl<'a> Field<'a> {
    /// Create a new field with the given name and value.
    pub fn new(name: &'static str, value: impl Into<FieldType<'a>>) -> Self {
        Field {
            name,
            value: value.into(),
        }
    }
}

/// A wrapper type that implements `Debug` for any table.
struct DebugPrintTable<'a, 'b>(pub &'b (dyn SomeTable<'a> + 'a));

/// A wrapper type that implements `Debug` for any array.
struct DebugPrintArray<'a, 'b>(pub &'b (dyn SomeArray<'a> + 'a));

impl<'a> Debug for FieldType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(arg0) => arg0.fmt(f),
            Self::U8(arg0) => arg0.fmt(f),
            Self::I16(arg0) => arg0.fmt(f),
            Self::U16(arg0) => arg0.fmt(f),
            Self::I32(arg0) => arg0.fmt(f),
            Self::U32(arg0) => arg0.fmt(f),
            Self::I24(arg0) => arg0.fmt(f),
            Self::U24(arg0) => arg0.fmt(f),
            Self::Tag(arg0) => arg0.fmt(f),
            Self::FWord(arg0) => arg0.to_i16().fmt(f),
            Self::UfWord(arg0) => arg0.to_u16().fmt(f),
            Self::MajorMinor(arg0) => write!(f, "{}.{}", arg0.major, arg0.minor),
            Self::Version16Dot16(arg0) => arg0.fmt(f),
            Self::F2Dot14(arg0) => arg0.fmt(f),
            Self::Fixed(arg0) => arg0.fmt(f),
            Self::LongDateTime(arg0) => arg0.as_secs().fmt(f),
            Self::GlyphId16(arg0) => {
                write!(f, "g")?;
                arg0.to_u16().fmt(f)
            }
            Self::NameId(arg0) => arg0.fmt(f),
            Self::StringOffset(string) => match &string.target {
                Ok(arg0) => arg0.as_ref().fmt(f),
                Err(_) => string.target.fmt(f),
            },
            Self::ArrayOffset(array) => match &array.target {
                Ok(arg0) => arg0.as_ref().fmt(f),
                Err(_) => array.target.fmt(f),
            },
            Self::BareOffset(arg0) => write!(f, "0x{:04X}", arg0.to_u32()),
            Self::ResolvedOffset(ResolvedOffset {
                target: Ok(arg0), ..
            }) => arg0.fmt(f),
            Self::ResolvedOffset(arg0) => arg0.target.fmt(f),
            Self::Record(arg0) => (arg0 as &(dyn SomeTable<'a> + 'a)).fmt(f),
            Self::Array(arg0) => arg0.fmt(f),
            Self::Unknown => write!(f, "no repr available"),
        }
    }
}

impl std::fmt::Debug for DebugPrintTable<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct(self.0.type_name());
        for field in self.0.iter() {
            debug_struct.field(field.name, &field.value);
        }
        debug_struct.finish()
    }
}

impl<'a> Debug for dyn SomeTable<'a> + 'a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DebugPrintTable(self).fmt(f)
    }
}

impl<'a> Debug for dyn SomeString<'a> + 'a {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\"")?;
        for c in self.iter_chars() {
            write!(f, "{c}")?
        }
        write!(f, "\"")
    }
}

impl std::fmt::Debug for DebugPrintArray<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut debug_list = f.debug_list();
        let mut idx = 0;
        while let Some(item) = self.0.get(idx) {
            idx += 1;
            debug_list.entry(&item);
        }
        debug_list.finish()
    }
}

impl<'a> Debug for dyn SomeArray<'a> + 'a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DebugPrintArray(self).fmt(f)
    }
}

impl std::fmt::Display for OffsetType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:+}", self.to_u32())
    }
}

// used to give us an auto-impl of Debug
impl<'a> SomeTable<'a> for RecordResolver<'a> {
    fn type_name(&self) -> &str {
        self.name
    }

    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        (self.get_field)(idx, self.data)
    }
}

impl<'a> From<u8> for FieldType<'a> {
    fn from(src: u8) -> FieldType<'a> {
        FieldType::U8(src)
    }
}

impl<'a> From<i8> for FieldType<'a> {
    fn from(src: i8) -> FieldType<'a> {
        FieldType::I8(src)
    }
}

impl<'a> From<u16> for FieldType<'a> {
    fn from(src: u16) -> FieldType<'a> {
        FieldType::U16(src)
    }
}

impl<'a> From<i16> for FieldType<'a> {
    fn from(src: i16) -> FieldType<'a> {
        FieldType::I16(src)
    }
}

impl<'a> From<u32> for FieldType<'a> {
    fn from(src: u32) -> FieldType<'a> {
        FieldType::U32(src)
    }
}

impl<'a> From<i32> for FieldType<'a> {
    fn from(src: i32) -> FieldType<'a> {
        FieldType::I32(src)
    }
}

impl<'a> From<Uint24> for FieldType<'a> {
    fn from(src: Uint24) -> FieldType<'a> {
        FieldType::U24(src)
    }
}

impl<'a> From<Int24> for FieldType<'a> {
    fn from(src: Int24) -> FieldType<'a> {
        FieldType::I24(src)
    }
}

impl<'a> From<Tag> for FieldType<'a> {
    fn from(src: Tag) -> FieldType<'a> {
        FieldType::Tag(src)
    }
}

impl<'a> From<FWord> for FieldType<'a> {
    fn from(src: FWord) -> FieldType<'a> {
        FieldType::FWord(src)
    }
}

impl<'a> From<UfWord> for FieldType<'a> {
    fn from(src: UfWord) -> FieldType<'a> {
        FieldType::UfWord(src)
    }
}

impl<'a> From<Fixed> for FieldType<'a> {
    fn from(src: Fixed) -> FieldType<'a> {
        FieldType::Fixed(src)
    }
}

impl<'a> From<F2Dot14> for FieldType<'a> {
    fn from(src: F2Dot14) -> FieldType<'a> {
        FieldType::F2Dot14(src)
    }
}

impl<'a> From<LongDateTime> for FieldType<'a> {
    fn from(src: LongDateTime) -> FieldType<'a> {
        FieldType::LongDateTime(src)
    }
}

impl<'a> From<MajorMinor> for FieldType<'a> {
    fn from(src: MajorMinor) -> FieldType<'a> {
        FieldType::MajorMinor(src)
    }
}

impl<'a> From<Version16Dot16> for FieldType<'a> {
    fn from(src: Version16Dot16) -> FieldType<'a> {
        FieldType::Version16Dot16(src)
    }
}

impl<'a> From<GlyphId16> for FieldType<'a> {
    fn from(src: GlyphId16) -> FieldType<'a> {
        FieldType::GlyphId16(src)
    }
}

impl<'a> From<NameId> for FieldType<'a> {
    fn from(src: NameId) -> FieldType<'a> {
        FieldType::NameId(src)
    }
}

impl<'a> From<RecordResolver<'a>> for FieldType<'a> {
    fn from(src: RecordResolver<'a>) -> Self {
        FieldType::Record(src)
    }
}

impl<'a, T: SomeArray<'a> + 'a> From<T> for FieldType<'a> {
    fn from(src: T) -> Self {
        FieldType::Array(Box::new(src))
    }
}

impl From<Offset16> for OffsetType {
    fn from(src: Offset16) -> OffsetType {
        OffsetType::Offset16(src.to_u32() as u16)
    }
}

impl From<Offset24> for OffsetType {
    fn from(src: Offset24) -> OffsetType {
        OffsetType::Offset24(Uint24::new(src.to_u32()))
    }
}

impl From<Offset32> for OffsetType {
    fn from(src: Offset32) -> OffsetType {
        OffsetType::Offset32(src.to_u32())
    }
}

impl<'a> From<Offset16> for FieldType<'a> {
    fn from(src: Offset16) -> FieldType<'a> {
        FieldType::BareOffset(src.into())
    }
}

impl<'a> From<Offset24> for FieldType<'a> {
    fn from(src: Offset24) -> FieldType<'a> {
        FieldType::BareOffset(src.into())
    }
}

impl<'a> From<Offset32> for FieldType<'a> {
    fn from(src: Offset32) -> FieldType<'a> {
        FieldType::BareOffset(src.into())
    }
}

impl<T: Into<OffsetType> + Clone> From<Nullable<T>> for OffsetType {
    fn from(src: Nullable<T>) -> Self {
        src.offset().clone().into()
    }
}
