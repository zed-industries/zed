//! raw font bytes

#![deny(clippy::arithmetic_side_effects)]
use std::ops::{Range, RangeBounds};

use bytemuck::AnyBitPattern;
use types::{BigEndian, FixedSize, Scalar};

use crate::array::ComputedArray;
use crate::read::{ComputeSize, FontReadWithArgs, ReadError};
use crate::table_ref::TableRef;
use crate::FontRead;

/// A reference to raw binary font data.
///
/// This is a wrapper around a byte slice, that provides convenience methods
/// for parsing and validating that data.
#[derive(Debug, Default, Clone, Copy)]
pub struct FontData<'a> {
    bytes: &'a [u8],
}

/// A cursor for validating bytes during parsing.
///
/// This type improves the ergonomics of validation blah blah
///
/// # Note
///
/// call `finish` when you're done to ensure you're in bounds
#[derive(Debug, Default, Clone, Copy)]
pub struct Cursor<'a> {
    pos: usize,
    data: FontData<'a>,
}

impl<'a> FontData<'a> {
    /// Empty data, useful for some tests and examples
    pub const EMPTY: FontData<'static> = FontData { bytes: &[] };

    /// Create a new `FontData` with these bytes.
    ///
    /// You generally don't need to do this? It is handled for you when loading
    /// data from disk, but may be useful in tests.
    pub const fn new(bytes: &'a [u8]) -> Self {
        FontData { bytes }
    }

    /// The length of the data, in bytes
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// `true` if the data has a length of zero bytes.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns self[pos..]
    pub fn split_off(&self, pos: usize) -> Option<FontData<'a>> {
        self.bytes.get(pos..).map(|bytes| FontData { bytes })
    }

    /// returns self[..pos], and updates self to = self[pos..];
    pub fn take_up_to(&mut self, pos: usize) -> Option<FontData<'a>> {
        if pos > self.len() {
            return None;
        }
        let (head, tail) = self.bytes.split_at(pos);
        self.bytes = tail;
        Some(FontData { bytes: head })
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> Option<FontData<'a>> {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        self.bytes.get(bounds).map(|bytes| FontData { bytes })
    }

    /// Read a scalar at the provided location in the data.
    pub fn read_at<T: Scalar>(&self, offset: usize) -> Result<T, ReadError> {
        let end = offset
            .checked_add(T::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        self.bytes
            .get(offset..end)
            .and_then(T::read)
            .ok_or(ReadError::OutOfBounds)
    }

    /// Read a big-endian value at the provided location in the data.
    pub fn read_be_at<T: Scalar>(&self, offset: usize) -> Result<BigEndian<T>, ReadError> {
        let end = offset
            .checked_add(T::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        self.bytes
            .get(offset..end)
            .and_then(BigEndian::from_slice)
            .ok_or(ReadError::OutOfBounds)
    }

    pub fn read_with_args<T>(&self, range: Range<usize>, args: &T::Args) -> Result<T, ReadError>
    where
        T: FontReadWithArgs<'a>,
    {
        self.slice(range)
            .ok_or(ReadError::OutOfBounds)
            .and_then(|data| T::read_with_args(data, args))
    }

    fn check_in_bounds(&self, offset: usize) -> Result<(), ReadError> {
        self.bytes
            .get(..offset)
            .ok_or(ReadError::OutOfBounds)
            .map(|_| ())
    }

    /// Interpret the bytes at the provided offset as a reference to `T`.
    ///
    /// Returns an error if the slice `offset..` is shorter than `T::RAW_BYTE_LEN`.
    ///
    /// This is a wrapper around [`read_ref_unchecked`][], which panics if
    /// the type does not uphold the required invariants.
    ///
    /// # Panics
    ///
    /// This function will panic if `T` is zero-sized, has an alignment
    /// other than one, or has any internal padding.
    ///
    /// [`read_ref_unchecked`]: [Self::read_ref_unchecked]
    pub fn read_ref_at<T: AnyBitPattern + FixedSize>(
        &self,
        offset: usize,
    ) -> Result<&'a T, ReadError> {
        let end = offset
            .checked_add(T::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        self.bytes
            .get(offset..end)
            .ok_or(ReadError::OutOfBounds)
            .map(bytemuck::from_bytes)
    }

    /// Interpret the bytes at the provided offset as a slice of `T`.
    ///
    /// Returns an error if `range` is out of bounds for the underlying data,
    /// or if the length of the range is not a multiple of `T::RAW_BYTE_LEN`.
    ///
    /// This is a wrapper around [`read_array_unchecked`][], which panics if
    /// the type does not uphold the required invariants.
    ///
    /// # Panics
    ///
    /// This function will panic if `T` is zero-sized, has an alignment
    /// other than one, or has any internal padding.
    ///
    /// [`read_array_unchecked`]: [Self::read_array_unchecked]
    pub fn read_array<T: AnyBitPattern + FixedSize>(
        &self,
        range: Range<usize>,
    ) -> Result<&'a [T], ReadError> {
        let bytes = self
            .bytes
            .get(range.clone())
            .ok_or(ReadError::OutOfBounds)?;
        if bytes
            .len()
            .checked_rem(std::mem::size_of::<T>())
            .unwrap_or(1) // definitely != 0
            != 0
        {
            return Err(ReadError::InvalidArrayLen);
        };
        Ok(bytemuck::cast_slice(bytes))
    }

    pub(crate) fn cursor(&self) -> Cursor<'a> {
        Cursor {
            pos: 0,
            data: *self,
        }
    }

    /// Return the data as a byte slice
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a> Cursor<'a> {
    pub(crate) fn advance<T: Scalar>(&mut self) {
        self.pos = self.pos.saturating_add(T::RAW_BYTE_LEN);
    }

    pub(crate) fn advance_by(&mut self, n_bytes: usize) {
        self.pos = self.pos.saturating_add(n_bytes);
    }

    /// Read a variable length u32 and advance the cursor
    pub(crate) fn read_u32_var(&mut self) -> Result<u32, ReadError> {
        let mut next = || self.read::<u8>().map(|v| v as u32);
        let b0 = next()?;
        // TODO this feels possible to simplify, e.g. compute length, loop taking one and shifting and or'ing
        #[allow(clippy::arithmetic_side_effects)] // these are all checked
        let result = match b0 {
            _ if b0 < 0x80 => b0,
            _ if b0 < 0xC0 => ((b0 - 0x80) << 8) | next()?,
            _ if b0 < 0xE0 => ((b0 - 0xC0) << 16) | (next()? << 8) | next()?,
            _ if b0 < 0xF0 => ((b0 - 0xE0) << 24) | (next()? << 16) | (next()? << 8) | next()?,
            _ => {
                // TODO: << 32 doesn't make sense. (b0 - 0xF0) << 32
                (next()? << 24) | (next()? << 16) | (next()? << 8) | next()?
            }
        };

        Ok(result)
    }

    /// Read a scalar and advance the cursor.
    pub(crate) fn read<T: Scalar>(&mut self) -> Result<T, ReadError> {
        let temp = self.data.read_at(self.pos);
        self.advance::<T>();
        temp
    }

    /// Read a big-endian value and advance the cursor.
    pub(crate) fn read_be<T: Scalar>(&mut self) -> Result<BigEndian<T>, ReadError> {
        let temp = self.data.read_be_at(self.pos);
        self.advance::<T>();
        temp
    }

    pub(crate) fn read_with_args<T>(&mut self, args: &T::Args) -> Result<T, ReadError>
    where
        T: FontReadWithArgs<'a> + ComputeSize,
    {
        let len = T::compute_size(args)?;
        let range_end = self.pos.checked_add(len).ok_or(ReadError::OutOfBounds)?;
        let temp = self.data.read_with_args(self.pos..range_end, args);
        self.advance_by(len);
        temp
    }

    // only used in records that contain arrays :/
    pub(crate) fn read_computed_array<T>(
        &mut self,
        len: usize,
        args: &T::Args,
    ) -> Result<ComputedArray<'a, T>, ReadError>
    where
        T: FontReadWithArgs<'a> + ComputeSize,
    {
        let len = len
            .checked_mul(T::compute_size(args)?)
            .ok_or(ReadError::OutOfBounds)?;
        let range_end = self.pos.checked_add(len).ok_or(ReadError::OutOfBounds)?;
        let temp = self.data.read_with_args(self.pos..range_end, args);
        self.advance_by(len);
        temp
    }

    pub(crate) fn read_array<T: AnyBitPattern + FixedSize>(
        &mut self,
        n_elem: usize,
    ) -> Result<&'a [T], ReadError> {
        let len = n_elem
            .checked_mul(T::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        let end = self.pos.checked_add(len).ok_or(ReadError::OutOfBounds)?;
        let temp = self.data.read_array(self.pos..end);
        self.advance_by(len);
        temp
    }

    /// return the current position, or an error if we are out of bounds
    pub(crate) fn position(&self) -> Result<usize, ReadError> {
        self.data.check_in_bounds(self.pos).map(|_| self.pos)
    }

    // used when handling fields with an implicit length, which must be at the
    // end of a table.
    pub(crate) fn remaining_bytes(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub(crate) fn remaining(self) -> Option<FontData<'a>> {
        self.data.split_off(self.pos)
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub(crate) fn finish<T>(self, shape: T) -> Result<TableRef<'a, T>, ReadError> {
        let data = self.data;
        data.check_in_bounds(self.pos)?;
        Ok(TableRef { data, shape })
    }
}

// useful so we can have offsets that are just to data
impl<'a> FontRead<'a> for FontData<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        Ok(data)
    }
}

impl AsRef<[u8]> for FontData<'_> {
    fn as_ref(&self) -> &[u8] {
        self.bytes
    }
}

impl<'a> From<&'a [u8]> for FontData<'a> {
    fn from(src: &'a [u8]) -> FontData<'a> {
        FontData::new(src)
    }
}

//kind of ugly, but makes FontData work with FontBuilder. If FontBuilder stops using
//Cow in its API, we can probably get rid of this?
#[cfg(feature = "std")]
impl<'a> From<FontData<'a>> for std::borrow::Cow<'a, [u8]> {
    fn from(src: FontData<'a>) -> Self {
        src.bytes.into()
    }
}
