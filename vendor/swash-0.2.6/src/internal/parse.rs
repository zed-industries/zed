//! Parsing primitives.

use core::ops::Range;

/// Buffer wrapping a byte slice for safely reading big endian data.
#[derive(Copy, Clone)]
pub struct Bytes<'a>(pub &'a [u8]);

impl<'a> Bytes<'a> {
    /// Creates a new bytes instance for the specified buffer.
    pub fn new(data: &'a [u8]) -> Self {
        Self(data)
    }

    /// Creates a new bytes instance for the specified buffer and offset.
    pub fn with_offset(data: &'a [u8], offset: usize) -> Option<Self> {
        Some(Self(data.get(offset..)?))
    }

    /// Creates a new bytes instance with the specified range of data.
    pub fn with_range(data: &'a [u8], range: Range<usize>) -> Option<Self> {
        Some(Self(data.get(range)?))
    }

    /// Returns the underlying data.
    pub fn data(&self) -> &'a [u8] {
        self.0
    }

    /// Returns the length of the underlying data.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the specified range is within the bounds of the
    /// underlying data.
    pub fn check_range(&self, offset: usize, len: usize) -> bool {
        let end = self.0.len();
        (offset < end) & (end - offset >= len)
    }

    /// Returns an error if the specified range is not within the bounds of
    /// the underlying data.
    pub fn ensure_range(&self, offset: usize, len: usize) -> Option<()> {
        if self.check_range(offset, len) {
            Some(())
        } else {
            None
        }
    }

    /// Reads a value of the specified type at some offset.
    #[inline(always)]
    pub fn read<T: FromBeData>(&self, offset: usize) -> Option<T> {
        T::from_be_data(self.0, offset)
    }

    /// Reads a u8 value at some offset.
    #[inline(always)]
    pub fn read_u8(&self, offset: usize) -> Option<u8> {
        u8::from_be_data(self.0, offset)
    }

    /// Reads a u16 value at some offset.
    #[inline(always)]
    pub fn read_u16(&self, offset: usize) -> Option<u16> {
        u16::from_be_data(self.0, offset)
    }

    /// Reads a u24 value at the specified offset.
    #[inline(always)]
    pub fn read_u24(&self, offset: usize) -> Option<u32> {
        U24::from_be_data(self.0, offset).map(|x| x.0)
    }

    /// Reads a u32 value at some offset.
    #[inline(always)]
    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        u32::from_be_data(self.0, offset)
    }

    /// Reads an i8 value at some offset.
    #[inline(always)]
    pub fn read_i8(&self, offset: usize) -> Option<i8> {
        i8::from_be_data(self.0, offset)
    }

    /// Reads an i16 value at some offset.
    #[inline(always)]
    pub fn read_i16(&self, offset: usize) -> Option<i16> {
        i16::from_be_data(self.0, offset)
    }

    /// Reads a value of the specified type at some offset, or returns the
    /// default value on bounds check failure.
    pub fn read_or_default<T: FromBeData + Default>(&self, offset: usize) -> T {
        T::from_be_data(self.0, offset).unwrap_or_default()
    }

    /// Returns a value of the specified type at some offset without bounds
    /// checking.
    #[inline(always)]
    pub unsafe fn read_unchecked<T: FromBeData>(&self, offset: usize) -> T {
        T::from_be_data_unchecked(self.0, offset)
    }

    /// Reads an array of values of the specified type and length at some
    /// offset.
    pub fn read_array<T: FromBeData>(&self, offset: usize, len: usize) -> Option<Array<'a, T>> {
        let len = len * T::SIZE;
        if !self.check_range(offset, len) {
            return None;
        }
        Some(Array::new(&self.0[offset..offset + len]))
    }

    /// Reads a sequence of bytes at the specified offset and length.
    pub fn read_bytes(&self, offset: usize, len: usize) -> Option<&'a [u8]> {
        if !self.check_range(offset, len) {
            return None;
        }
        Some(&self.0[offset..offset + len])
    }

    /// Creates a new stream at the specified offset.
    pub fn stream_at(&self, offset: usize) -> Option<Stream<'a>> {
        Stream::with_offset(self.0, offset)
    }
}

impl<'a> core::ops::Deref for Bytes<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Stream over a byte slice for safely reading big endian data.
#[derive(Copy, Clone)]
pub struct Stream<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Stream<'a> {
    /// Creates a new stream wrapping the specified bytes.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    /// Creates a new stream with the specified data and offset.
    pub fn with_offset(data: &'a [u8], offset: usize) -> Option<Self> {
        let data = data.get(offset..)?;
        Some(Self { data, offset: 0 })
    }

    /// Creates a new stream with the specified range of data.
    pub fn with_range(data: &'a [u8], range: Range<usize>) -> Option<Self> {
        let data = data.get(range)?;
        Some(Self { data, offset: 0 })
    }

    /// Returns the underlying buffer for the cursor.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Returns the length of the underlying buffer.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the current offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the number of bytes available for reading.
    pub fn remaining(&self) -> usize {
        self.data.len() - self.offset
    }

    /// Sets the offset.
    pub fn set_offset(&mut self, offset: usize) -> Option<()> {
        if offset > self.data.len() {
            return None;
        }
        self.offset = offset;
        Some(())
    }

    /// Returns true if the specified number of bytes can be read.
    pub fn check_range(&self, len: usize) -> bool {
        self.data.len() - self.offset >= len
    }

    /// Returns an error of the specified number of bytes cannot be read.
    pub fn ensure_range(&self, len: usize) -> Option<()> {
        if self.check_range(len) {
            Some(())
        } else {
            None
        }
    }

    /// Skips the specified number of bytes.
    pub fn skip(&mut self, bytes: usize) -> Option<()> {
        self.set_offset(self.offset.checked_add(bytes)?)
    }

    /// Reads a value of the specified type and advances the offset.
    pub fn read<T: FromBeData>(&mut self) -> Option<T> {
        if self.data.len() - self.offset < T::SIZE {
            None
        } else {
            let v = unsafe { T::from_be_data_unchecked(self.data, self.offset) };
            self.offset += T::SIZE;
            Some(v)
        }
    }

    /// Reads a u8 value and advances the offset.
    #[inline(always)]
    pub fn read_u8(&mut self) -> Option<u8> {
        self.read::<u8>()
    }

    /// Reads a u16 value and advances the offset.
    #[inline(always)]
    pub fn read_u16(&mut self) -> Option<u16> {
        self.read::<u16>()
    }

    /// Reads a u32 value and advances the offset.
    #[inline(always)]
    pub fn read_u32(&mut self) -> Option<u32> {
        self.read::<u32>()
    }

    /// Reads an i8 value and advances the offset.
    #[inline(always)]
    pub fn read_i8(&mut self) -> Option<i8> {
        self.read::<i8>()
    }

    /// Reads an i16 value and advances the offset.
    #[inline(always)]
    pub fn read_i16(&mut self) -> Option<i16> {
        self.read::<i16>()
    }

    /// Reads an array of values of the specified type and length and
    /// advances the offset.
    pub fn read_array<T: FromBeData>(&mut self, len: usize) -> Option<Array<'a, T>> {
        let len = len * T::SIZE;
        if !self.check_range(len) {
            return None;
        }
        let arr = Array::new(&self.data[self.offset..self.offset + len]);
        self.offset += len;
        Some(arr)
    }

    /// Reads a sequence of bytes of the specified length and advances the
    /// offset.
    pub fn read_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        if !self.check_range(len) {
            return None;
        }
        let bytes = &self.data[self.offset..self.offset + len];
        self.offset += len;
        Some(bytes)
    }
}

/// An array wrapping a byte buffer over a sequence of values that implement
/// [`FromBeData`].
#[derive(Copy, Clone)]
pub struct Array<'a, T: FromBeData> {
    data: &'a [u8],
    len: usize,
    _p: core::marker::PhantomData<T>,
}

impl<T> core::fmt::Debug for Array<'_, T>
where
    T: core::fmt::Debug + FromBeData,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", value)?;
        }
        write!(f, "]")
    }
}

impl<'a, T: FromBeData> Array<'a, T> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            len: data.len() / T::SIZE,
            _p: core::marker::PhantomData {},
        }
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the element at the specified index.
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(T::from_be_data_unchecked(self.data, index * T::SIZE)) }
        }
    }

    /// Returns the element at the specified index, or some value if the index
    /// is out of bounds.
    pub fn get_or(&self, index: usize, or: T) -> T {
        if index >= self.len {
            or
        } else {
            unsafe { T::from_be_data_unchecked(self.data, index * T::SIZE) }
        }
    }

    /// Returns the element at the specified index without bounds checking.
    pub unsafe fn get_unchecked(&self, index: usize) -> T {
        T::from_be_data_unchecked(self.data, index * T::SIZE)
    }

    /// Performs a binary search over the array using the specified comparator
    /// function. Returns the index and value of the element on success, or
    /// `None` if a match was not found.
    pub fn binary_search_by<F>(&self, mut f: F) -> Option<(usize, T)>
    where
        F: FnMut(&T) -> core::cmp::Ordering,
    {
        // Taken from Rust core library.
        use core::cmp::Ordering::*;
        let mut size = self.len;
        if size == 0 {
            return None;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            // SAFETY: the call is made safe by the following inconstants:
            // - `mid >= 0`: by definition
            // - `mid < size`: `mid = size / 2 + size / 4 + size / 8 ...`
            let element = unsafe { self.get_unchecked(mid) };
            base = match f(&element) {
                Greater => base,
                Less => mid,
                Equal => return Some((mid, element)),
            };
            size -= half;
        }
        None
    }

    /// Returns an iterator over the elements of the array.
    pub fn iter(&self) -> ArrayIter<'a, T> {
        ArrayIter {
            inner: *self,
            cur: 0,
        }
    }
}

/// Iterator over the elements of an array.
#[derive(Clone)]
#[doc(hidden)]
pub struct ArrayIter<'a, T: FromBeData> {
    inner: Array<'a, T>,
    cur: usize,
}

impl<'a, T: FromBeData + 'a> Iterator for ArrayIter<'a, T> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.inner.len - self.cur;
        (remaining, Some(remaining))
    }

    fn next(&mut self) -> Option<T> {
        if self.cur >= self.inner.len {
            return None;
        }
        self.cur += 1;
        unsafe { Some(self.inner.get_unchecked(self.cur - 1)) }
    }
}

impl<'a, T: FromBeData + 'a> IntoIterator for Array<'a, T> {
    type IntoIter = ArrayIter<'a, T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        ArrayIter {
            inner: self,
            cur: 0,
        }
    }
}

/// Interface for reading big endian data from a buffer.
pub trait FromBeData: Sized + Copy + Clone {
    const SIZE: usize = core::mem::size_of::<Self>();

    #[inline(always)]
    fn from_be_data(buf: &[u8], offset: usize) -> Option<Self> {
        let len = buf.len();
        if (offset < len) && ((len - offset) >= Self::SIZE) {
            unsafe { Some(Self::from_be_data_unchecked(buf, offset)) }
        } else {
            None
        }
    }

    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self;
}

pub(crate) const USE_UNALIGNED_READS_LE: bool =
    cfg!(any(target_arch = "x86", target_arch = "x86_64")) && cfg!(not(debug_assertions));

impl FromBeData for u8 {
    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        *buf.get_unchecked(offset)
    }
}

impl FromBeData for i8 {
    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        *buf.get_unchecked(offset) as i8
    }
}

impl FromBeData for u16 {
    #[inline(always)]
    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        if USE_UNALIGNED_READS_LE {
            (buf.as_ptr().add(offset) as *const u16)
                .read_unaligned()
                .swap_bytes()
        } else {
            (*buf.get_unchecked(offset) as u16) << 8 | *buf.get_unchecked(offset + 1) as u16
        }
    }
}

impl FromBeData for i16 {
    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        u16::from_be_data_unchecked(buf, offset) as i16
    }
}

impl FromBeData for u32 {
    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        if USE_UNALIGNED_READS_LE {
            (buf.as_ptr().add(offset) as *const u32)
                .read_unaligned()
                .swap_bytes()
        } else {
            (*buf.get_unchecked(offset) as u32) << 24
                | (*buf.get_unchecked(offset + 1) as u32) << 16
                | (*buf.get_unchecked(offset + 2) as u32) << 8
                | *buf.get_unchecked(offset + 3) as u32
        }
    }
}

impl FromBeData for i32 {
    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        u32::from_be_data_unchecked(buf, offset) as i32
    }
}

impl FromBeData for u64 {
    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        if USE_UNALIGNED_READS_LE {
            (buf.as_ptr().add(offset) as *const u64)
                .read_unaligned()
                .swap_bytes()
        } else {
            (*buf.get_unchecked(offset) as u64) << 56
                | (*buf.get_unchecked(offset + 1) as u64) << 48
                | (*buf.get_unchecked(offset + 2) as u64) << 40
                | (*buf.get_unchecked(offset + 3) as u64) << 32
                | (*buf.get_unchecked(offset + 4) as u64) << 24
                | (*buf.get_unchecked(offset + 5) as u64) << 16
                | (*buf.get_unchecked(offset + 6) as u64) << 8
                | *buf.get_unchecked(offset + 7) as u64
        }
    }
}

/// Unsigned 24-bit integer.
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct U24(pub u32);

impl FromBeData for U24 {
    const SIZE: usize = 3;

    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        Self(
            (*buf.get_unchecked(offset) as u32) << 16
                | (*buf.get_unchecked(offset + 1) as u32) << 8
                | *buf.get_unchecked(offset + 2) as u32,
        )
    }
}

impl FromBeData for () {
    unsafe fn from_be_data_unchecked(_buf: &[u8], _offset: usize) -> Self {}
}
