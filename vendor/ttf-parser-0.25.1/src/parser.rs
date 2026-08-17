//! Binary parsing utils.
//!
//! This module should not be used directly, unless you're planning to parse
//! some tables manually.

use core::convert::{TryFrom, TryInto};
use core::ops::Range;

/// A trait for parsing raw binary data of fixed size.
///
/// This is a low-level, internal trait that should not be used directly.
pub trait FromData: Sized {
    /// Object's raw data size.
    ///
    /// Not always the same as `mem::size_of`.
    const SIZE: usize;

    /// Parses an object from a raw data.
    fn parse(data: &[u8]) -> Option<Self>;
}

/// A trait for parsing raw binary data of variable size.
///
/// This is a low-level, internal trait that should not be used directly.
pub trait FromSlice<'a>: Sized {
    /// Parses an object from a raw data.
    fn parse(data: &'a [u8]) -> Option<Self>;
}

impl FromData for () {
    const SIZE: usize = 0;

    #[inline]
    fn parse(_: &[u8]) -> Option<Self> {
        Some(())
    }
}

impl FromData for u8 {
    const SIZE: usize = 1;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.get(0).copied()
    }
}

impl FromData for i8 {
    const SIZE: usize = 1;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.get(0).copied().map(|n| n as i8)
    }
}

impl FromData for u16 {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(u16::from_be_bytes)
    }
}

impl FromData for i16 {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(i16::from_be_bytes)
    }
}

impl FromData for u32 {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(u32::from_be_bytes)
    }
}

impl FromData for i32 {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(i32::from_be_bytes)
    }
}

impl FromData for u64 {
    const SIZE: usize = 8;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(u64::from_be_bytes)
    }
}

/// A u24 number.
///
/// Stored as u32, but encoded as 3 bytes in the font.
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/otff#data-types>
#[derive(Clone, Copy, Debug)]
pub struct U24(pub u32);

impl FromData for U24 {
    const SIZE: usize = 3;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let data: [u8; 3] = data.try_into().ok()?;
        Some(U24(u32::from_be_bytes([0, data[0], data[1], data[2]])))
    }
}

/// A 16-bit signed fixed number with the low 14 bits of fraction (2.14).
#[derive(Clone, Copy, Debug)]
pub struct F2DOT14(pub i16);

impl F2DOT14 {
    /// Converts i16 to f32.
    #[inline]
    pub fn to_f32(self) -> f32 {
        f32::from(self.0) / 16384.0
    }

    #[cfg(feature = "variable-fonts")]
    #[inline]
    pub fn apply_float_delta(&self, delta: f32) -> f32 {
        self.to_f32() + (delta as f64 * (1.0 / 16384.0)) as f32
    }
}

impl FromData for F2DOT14 {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        i16::parse(data).map(F2DOT14)
    }
}

/// A 32-bit signed fixed-point number (16.16).
#[derive(Clone, Copy, Debug)]
pub struct Fixed(pub f32);

impl FromData for Fixed {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        // TODO: is it safe to cast?
        i32::parse(data).map(|n| Fixed(n as f32 / 65536.0))
    }
}

impl Fixed {
    #[cfg(feature = "variable-fonts")]
    #[inline]
    pub(crate) fn apply_float_delta(&self, delta: f32) -> f32 {
        self.0 + (delta as f64 * (1.0 / 65536.0)) as f32
    }
}

/// A safe u32 to usize casting.
///
/// Rust doesn't implement `From<u32> for usize`,
/// because it has to support 16 bit targets.
/// We don't, so we can allow this.
pub trait NumFrom<T>: Sized {
    /// Converts u32 into usize.
    fn num_from(_: T) -> Self;
}

impl NumFrom<u32> for usize {
    #[inline]
    fn num_from(v: u32) -> Self {
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        {
            v as usize
        }

        // compilation error on 16 bit targets
    }
}

impl NumFrom<char> for usize {
    #[inline]
    fn num_from(v: char) -> Self {
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        {
            v as usize
        }

        // compilation error on 16 bit targets
    }
}

/// Just like TryFrom<N>, but for numeric types not supported by the Rust's std.
pub trait TryNumFrom<T>: Sized {
    /// Casts between numeric types.
    fn try_num_from(_: T) -> Option<Self>;
}

impl TryNumFrom<f32> for u8 {
    #[inline]
    fn try_num_from(v: f32) -> Option<Self> {
        i32::try_num_from(v).and_then(|v| u8::try_from(v).ok())
    }
}

impl TryNumFrom<f32> for i16 {
    #[inline]
    fn try_num_from(v: f32) -> Option<Self> {
        i32::try_num_from(v).and_then(|v| i16::try_from(v).ok())
    }
}

impl TryNumFrom<f32> for u16 {
    #[inline]
    fn try_num_from(v: f32) -> Option<Self> {
        i32::try_num_from(v).and_then(|v| u16::try_from(v).ok())
    }
}

#[allow(clippy::manual_range_contains)]
impl TryNumFrom<f32> for i32 {
    #[inline]
    fn try_num_from(v: f32) -> Option<Self> {
        // Based on https://github.com/rust-num/num-traits/blob/master/src/cast.rs

        // Float as int truncates toward zero, so we want to allow values
        // in the exclusive range `(MIN-1, MAX+1)`.

        // We can't represent `MIN-1` exactly, but there's no fractional part
        // at this magnitude, so we can just use a `MIN` inclusive boundary.
        const MIN: f32 = i32::MIN as f32;
        // We can't represent `MAX` exactly, but it will round up to exactly
        // `MAX+1` (a power of two) when we cast it.
        const MAX_P1: f32 = i32::MAX as f32;
        if v >= MIN && v < MAX_P1 {
            Some(v as i32)
        } else {
            None
        }
    }
}

/// A slice-like container that converts internal binary data only on access.
///
/// Array values are stored in a continuous data chunk.
#[derive(Clone, Copy)]
pub struct LazyArray16<'a, T> {
    data: &'a [u8],
    data_type: core::marker::PhantomData<T>,
}

impl<T> Default for LazyArray16<'_, T> {
    #[inline]
    fn default() -> Self {
        LazyArray16 {
            data: &[],
            data_type: core::marker::PhantomData,
        }
    }
}

impl<'a, T: FromData> LazyArray16<'a, T> {
    /// Creates a new `LazyArray`.
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        LazyArray16 {
            data,
            data_type: core::marker::PhantomData,
        }
    }

    /// Returns a value at `index`.
    #[inline]
    pub fn get(&self, index: u16) -> Option<T> {
        if index < self.len() {
            let start = usize::from(index) * T::SIZE;
            let end = start + T::SIZE;
            self.data.get(start..end).and_then(T::parse)
        } else {
            None
        }
    }

    /// Returns the last value.
    #[inline]
    pub fn last(&self) -> Option<T> {
        if !self.is_empty() {
            self.get(self.len() - 1)
        } else {
            None
        }
    }

    /// Returns sub-array.
    #[inline]
    pub fn slice(&self, range: Range<u16>) -> Option<Self> {
        let start = usize::from(range.start) * T::SIZE;
        let end = usize::from(range.end) * T::SIZE;
        Some(LazyArray16 {
            data: self.data.get(start..end)?,
            ..LazyArray16::default()
        })
    }

    /// Returns array's length.
    #[inline]
    pub fn len(&self) -> u16 {
        (self.data.len() / T::SIZE) as u16
    }

    /// Checks if array is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Performs a binary search by specified `key`.
    #[inline]
    pub fn binary_search(&self, key: &T) -> Option<(u16, T)>
    where
        T: Ord,
    {
        self.binary_search_by(|p| p.cmp(key))
    }

    /// Performs a binary search using specified closure.
    #[inline]
    pub fn binary_search_by<F>(&self, mut f: F) -> Option<(u16, T)>
    where
        F: FnMut(&T) -> core::cmp::Ordering,
    {
        // Based on Rust std implementation.

        use core::cmp::Ordering;

        let mut size = self.len();
        if size == 0 {
            return None;
        }

        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            // mid is always in [0, size), that means mid is >= 0 and < size.
            // mid >= 0: by definition
            // mid < size: mid = size / 2 + size / 4 + size / 8 ...
            let cmp = f(&self.get(mid)?);
            base = if cmp == Ordering::Greater { base } else { mid };
            size -= half;
        }

        // base is always in [0, size) because base <= mid.
        let value = self.get(base)?;
        if f(&value) == Ordering::Equal {
            Some((base, value))
        } else {
            None
        }
    }
}

impl<'a, T: FromData + core::fmt::Debug + Copy> core::fmt::Debug for LazyArray16<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

impl<'a, T: FromData> IntoIterator for LazyArray16<'a, T> {
    type Item = T;
    type IntoIter = LazyArrayIter16<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        LazyArrayIter16 {
            data: self,
            index: 0,
        }
    }
}

/// An iterator over `LazyArray16`.
#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
pub struct LazyArrayIter16<'a, T> {
    data: LazyArray16<'a, T>,
    index: u16,
}

impl<T: FromData> Default for LazyArrayIter16<'_, T> {
    #[inline]
    fn default() -> Self {
        LazyArrayIter16 {
            data: LazyArray16::new(&[]),
            index: 0,
        }
    }
}

impl<'a, T: FromData> Iterator for LazyArrayIter16<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1; // TODO: check
        self.data.get(self.index - 1)
    }

    #[inline]
    fn count(self) -> usize {
        usize::from(self.data.len().saturating_sub(self.index))
    }
}

/// A slice-like container that converts internal binary data only on access.
///
/// This is a low-level, internal structure that should not be used directly.
#[derive(Clone, Copy)]
pub struct LazyArray32<'a, T> {
    data: &'a [u8],
    data_type: core::marker::PhantomData<T>,
}

impl<T> Default for LazyArray32<'_, T> {
    #[inline]
    fn default() -> Self {
        LazyArray32 {
            data: &[],
            data_type: core::marker::PhantomData,
        }
    }
}

impl<'a, T: FromData> LazyArray32<'a, T> {
    /// Creates a new `LazyArray`.
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        LazyArray32 {
            data,
            data_type: core::marker::PhantomData,
        }
    }

    /// Returns a value at `index`.
    #[inline]
    pub fn get(&self, index: u32) -> Option<T> {
        if index < self.len() {
            let start = usize::num_from(index) * T::SIZE;
            let end = start + T::SIZE;
            self.data.get(start..end).and_then(T::parse)
        } else {
            None
        }
    }

    /// Returns array's length.
    #[inline]
    pub fn len(&self) -> u32 {
        (self.data.len() / T::SIZE) as u32
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Performs a binary search by specified `key`.
    #[inline]
    pub fn binary_search(&self, key: &T) -> Option<(u32, T)>
    where
        T: Ord,
    {
        self.binary_search_by(|p| p.cmp(key))
    }

    /// Performs a binary search using specified closure.
    #[inline]
    pub fn binary_search_by<F>(&self, mut f: F) -> Option<(u32, T)>
    where
        F: FnMut(&T) -> core::cmp::Ordering,
    {
        // Based on Rust std implementation.

        use core::cmp::Ordering;

        let mut size = self.len();
        if size == 0 {
            return None;
        }

        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            // mid is always in [0, size), that means mid is >= 0 and < size.
            // mid >= 0: by definition
            // mid < size: mid = size / 2 + size / 4 + size / 8 ...
            let cmp = f(&self.get(mid)?);
            base = if cmp == Ordering::Greater { base } else { mid };
            size -= half;
        }

        // base is always in [0, size) because base <= mid.
        let value = self.get(base)?;
        if f(&value) == Ordering::Equal {
            Some((base, value))
        } else {
            None
        }
    }
}

impl<'a, T: FromData + core::fmt::Debug + Copy> core::fmt::Debug for LazyArray32<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

impl<'a, T: FromData> IntoIterator for LazyArray32<'a, T> {
    type Item = T;
    type IntoIter = LazyArrayIter32<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        LazyArrayIter32 {
            data: self,
            index: 0,
        }
    }
}

/// An iterator over `LazyArray32`.
#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
pub struct LazyArrayIter32<'a, T> {
    data: LazyArray32<'a, T>,
    index: u32,
}

impl<'a, T: FromData> Iterator for LazyArrayIter32<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1; // TODO: check
        self.data.get(self.index - 1)
    }

    #[inline]
    fn count(self) -> usize {
        usize::num_from(self.data.len().saturating_sub(self.index))
    }
}

/// A [`LazyArray16`]-like container, but data is accessed by offsets.
///
/// Unlike [`LazyArray16`], internal storage is not continuous.
///
/// Multiple offsets can point to the same data.
#[derive(Clone, Copy)]
pub struct LazyOffsetArray16<'a, T: FromSlice<'a>> {
    data: &'a [u8],
    // Zero offsets must be ignored, therefore we're using `Option<Offset16>`.
    offsets: LazyArray16<'a, Option<Offset16>>,
    data_type: core::marker::PhantomData<T>,
}

impl<'a, T: FromSlice<'a>> LazyOffsetArray16<'a, T> {
    /// Creates a new `LazyOffsetArray16`.
    #[allow(dead_code)]
    pub fn new(data: &'a [u8], offsets: LazyArray16<'a, Option<Offset16>>) -> Self {
        Self {
            data,
            offsets,
            data_type: core::marker::PhantomData,
        }
    }

    /// Parses `LazyOffsetArray16` from raw data.
    #[allow(dead_code)]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let offsets = s.read_array16(count)?;
        Some(Self {
            data,
            offsets,
            data_type: core::marker::PhantomData,
        })
    }

    /// Returns a value at `index`.
    #[inline]
    pub fn get(&self, index: u16) -> Option<T> {
        let offset = self.offsets.get(index)??.to_usize();
        self.data.get(offset..).and_then(T::parse)
    }

    /// Returns array's length.
    #[inline]
    pub fn len(&self) -> u16 {
        self.offsets.len()
    }

    /// Checks if array is empty.
    #[inline]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T: FromSlice<'a> + core::fmt::Debug + Copy> core::fmt::Debug for LazyOffsetArray16<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

/// An iterator over [`LazyOffsetArray16`] values.
#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
pub struct LazyOffsetArrayIter16<'a, T: FromSlice<'a>> {
    array: LazyOffsetArray16<'a, T>,
    index: u16,
}

impl<'a, T: FromSlice<'a>> IntoIterator for LazyOffsetArray16<'a, T> {
    type Item = T;
    type IntoIter = LazyOffsetArrayIter16<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        LazyOffsetArrayIter16 {
            array: self,
            index: 0,
        }
    }
}

impl<'a, T: FromSlice<'a>> Iterator for LazyOffsetArrayIter16<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            self.index += 1;
            self.array.get(self.index - 1)
        } else {
            None
        }
    }

    #[inline]
    fn count(self) -> usize {
        usize::from(self.array.len().saturating_sub(self.index))
    }
}

/// A streaming binary parser.
#[derive(Clone, Default, Debug)]
pub struct Stream<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Stream<'a> {
    /// Creates a new `Stream` parser.
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Stream { data, offset: 0 }
    }

    /// Creates a new `Stream` parser at offset.
    ///
    /// Returns `None` when `offset` is out of bounds.
    #[inline]
    pub fn new_at(data: &'a [u8], offset: usize) -> Option<Self> {
        if offset <= data.len() {
            Some(Stream { data, offset })
        } else {
            None
        }
    }

    /// Checks that stream reached the end of the data.
    #[inline]
    pub fn at_end(&self) -> bool {
        self.offset >= self.data.len()
    }

    /// Jumps to the end of the stream.
    ///
    /// Useful to indicate that we parsed all the data.
    #[inline]
    pub fn jump_to_end(&mut self) {
        self.offset = self.data.len();
    }

    /// Returns the current offset.
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the trailing data.
    ///
    /// Returns `None` when `Stream` is reached the end.
    #[inline]
    pub fn tail(&self) -> Option<&'a [u8]> {
        self.data.get(self.offset..)
    }

    /// Advances by `FromData::SIZE`.
    ///
    /// Doesn't check bounds.
    #[inline]
    pub fn skip<T: FromData>(&mut self) {
        self.advance(T::SIZE);
    }

    /// Advances by the specified `len`.
    ///
    /// Doesn't check bounds.
    #[inline]
    pub fn advance(&mut self, len: usize) {
        self.offset += len;
    }

    /// Advances by the specified `len` and checks for bounds.
    #[inline]
    pub fn advance_checked(&mut self, len: usize) -> Option<()> {
        if self.offset + len <= self.data.len() {
            self.advance(len);
            Some(())
        } else {
            None
        }
    }

    /// Parses the type from the steam.
    ///
    /// Returns `None` when there is not enough data left in the stream
    /// or the type parsing failed.
    #[inline]
    pub fn read<T: FromData>(&mut self) -> Option<T> {
        self.read_bytes(T::SIZE).and_then(T::parse)
    }

    /// Parses the type from the steam at offset.
    #[inline]
    pub fn read_at<T: FromData>(data: &[u8], offset: usize) -> Option<T> {
        data.get(offset..offset + T::SIZE).and_then(T::parse)
    }

    /// Reads N bytes from the stream.
    #[inline]
    pub fn read_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        // An integer overflow here on 32bit systems is almost guarantee to be caused
        // by an incorrect parsing logic from the caller side.
        // Simply using `checked_add` here would silently swallow errors, which is not what we want.
        debug_assert!(self.offset as u64 + len as u64 <= u32::MAX as u64);

        let v = self.data.get(self.offset..self.offset + len)?;
        self.advance(len);
        Some(v)
    }

    /// Reads the next `count` types as a slice.
    #[inline]
    pub fn read_array16<T: FromData>(&mut self, count: u16) -> Option<LazyArray16<'a, T>> {
        let len = usize::from(count) * T::SIZE;
        self.read_bytes(len).map(LazyArray16::new)
    }

    /// Reads the next `count` types as a slice.
    #[inline]
    pub fn read_array32<T: FromData>(&mut self, count: u32) -> Option<LazyArray32<'a, T>> {
        let len = usize::num_from(count) * T::SIZE;
        self.read_bytes(len).map(LazyArray32::new)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn read_at_offset16(&mut self, data: &'a [u8]) -> Option<&'a [u8]> {
        let offset = self.read::<Offset16>()?.to_usize();
        data.get(offset..)
    }
}

/// A common offset methods.
pub trait Offset {
    /// Converts the offset to `usize`.
    fn to_usize(&self) -> usize;
}

/// A type-safe u16 offset.
#[derive(Clone, Copy, Debug)]
pub struct Offset16(pub u16);

impl Offset for Offset16 {
    #[inline]
    fn to_usize(&self) -> usize {
        usize::from(self.0)
    }
}

impl FromData for Offset16 {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        u16::parse(data).map(Offset16)
    }
}

impl FromData for Option<Offset16> {
    const SIZE: usize = Offset16::SIZE;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let offset = Offset16::parse(data)?;
        if offset.0 != 0 {
            Some(Some(offset))
        } else {
            Some(None)
        }
    }
}

/// A type-safe u24 offset.
#[derive(Clone, Copy, Debug)]
pub struct Offset24(pub u32);

impl Offset for Offset24 {
    #[inline]
    fn to_usize(&self) -> usize {
        usize::num_from(self.0)
    }
}

impl FromData for Offset24 {
    const SIZE: usize = 3;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        U24::parse(data).map(|n| Offset24(n.0))
    }
}

impl FromData for Option<Offset24> {
    const SIZE: usize = Offset24::SIZE;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let offset = Offset24::parse(data)?;
        if offset.0 != 0 {
            Some(Some(offset))
        } else {
            Some(None)
        }
    }
}

/// A type-safe u32 offset.
#[derive(Clone, Copy, Debug)]
pub struct Offset32(pub u32);

impl Offset for Offset32 {
    #[inline]
    fn to_usize(&self) -> usize {
        usize::num_from(self.0)
    }
}

impl FromData for Offset32 {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        u32::parse(data).map(Offset32)
    }
}

impl FromData for Option<Offset32> {
    const SIZE: usize = Offset32::SIZE;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let offset = Offset32::parse(data)?;
        if offset.0 != 0 {
            Some(Some(offset))
        } else {
            Some(None)
        }
    }
}

#[inline]
pub fn i16_bound(min: i16, val: i16, max: i16) -> i16 {
    use core::cmp;
    cmp::max(min, cmp::min(max, val))
}

#[inline]
pub fn f32_bound(min: f32, val: f32, max: f32) -> f32 {
    debug_assert!(min.is_finite());
    debug_assert!(val.is_finite());
    debug_assert!(max.is_finite());

    if val > max {
        return max;
    } else if val < min {
        return min;
    }

    val
}
