//! This module contains a bunch of traits necessary for processing byte strings.
//!
//! Most notable are:
//! * `Source` - implemented by default for `&str`, `&[u8]` and wrapper types, used by the `Lexer`.
//! * `Slice` - slices of `Source`, returned by `Lexer::slice`.

use core::fmt::Debug;
use core::ops::{Deref, Range};

/// Trait for types the `Lexer` can read from.
///
/// Most notably this is implemented for `&str`. It is unlikely you will
/// ever want to use this Trait yourself, unless implementing a new `Source`
/// the `Lexer` can use.
///
/// SAFETY: Unless the unsafe functions of this trait are disabled with the `forbid_unsafe`
/// feature, the correctness of the unsafe functions of this trait depend on the correct
/// implementation of the `len` and `find_boundary` functions so generated code does not request
/// out-of-bounds access.
#[allow(clippy::len_without_is_empty)]
pub trait Source {
    /// A type this `Source` can be sliced into.
    type Slice<'a>: PartialEq + Eq + Debug
    where
        Self: 'a;

    /// Length of the source
    fn len(&self) -> usize;

    /// Read a chunk of bytes into an array. Returns `None` when reading
    /// out of bounds would occur.
    ///
    /// This is very useful for matching fixed-size byte arrays, and tends
    /// to be very fast at it too, since the compiler knows the byte lengths.
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// let foo = "foo";
    ///
    /// assert_eq!(foo.read(0), Some(b"foo"));     // Option<&[u8; 3]>
    /// assert_eq!(foo.read(0), Some(b"fo"));      // Option<&[u8; 2]>
    /// assert_eq!(foo.read(2), Some(b'o'));       // Option<u8>
    /// assert_eq!(foo.read::<&[u8; 4]>(0), None); // Out of bounds
    /// assert_eq!(foo.read::<&[u8; 2]>(2), None); // Out of bounds
    /// ```
    fn read<'a, Chunk>(&'a self, offset: usize) -> Option<Chunk>
    where
        Chunk: self::Chunk<'a>;

    /// Read a byte without doing bounds checks.
    ///
    /// # Safety
    ///
    /// Offset should not exceed bounds.
    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn read_byte_unchecked(&self, offset: usize) -> u8;

    /// Read a byte with bounds checking.
    #[cfg(feature = "forbid_unsafe")]
    fn read_byte(&self, offset: usize) -> u8;

    /// Get a slice of the source at given range. This is analogous to
    /// `slice::get(range)`.
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// let foo = "It was the year when they finally immanentized the Eschaton.";
    /// assert_eq!(<str as Source>::slice(&foo, 51..59), Some("Eschaton"));
    /// ```
    fn slice(&self, range: Range<usize>) -> Option<Self::Slice<'_>>;

    /// Get a slice of the source at given range. This is analogous to
    /// `slice::get_unchecked(range)`.
    ///
    /// # Safety
    ///
    /// Range should not exceed bounds.
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// let foo = "It was the year when they finally immanentized the Eschaton.";
    ///
    /// unsafe {
    ///     assert_eq!(<str as Source>::slice_unchecked(&foo, 51..59), "Eschaton");
    /// }
    /// ```
    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> Self::Slice<'_>;

    /// For `&str` sources attempts to find the closest `char` boundary at which source
    /// can be sliced, starting from `index`.
    ///
    /// For binary sources (`&[u8]`) this should just return `index` back.
    #[inline]
    fn find_boundary(&self, index: usize) -> usize {
        index
    }

    /// Check if `index` is valid for this `Source`, that is:
    ///
    /// + It's not larger than the byte length of the `Source`.
    /// + (`str` only) It doesn't land in the middle of a UTF-8 code point.
    fn is_boundary(&self, index: usize) -> bool;
}

impl Source for str {
    type Slice<'a> = &'a str;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn read<'a, Chunk>(&'a self, offset: usize) -> Option<Chunk>
    where
        Chunk: self::Chunk<'a>,
    {
        #[cfg(not(feature = "forbid_unsafe"))]
        if offset + (Chunk::SIZE - 1) < self.len() {
            // # Safety: we just performed a bound check.
            Some(unsafe { Chunk::from_ptr(self.as_ptr().add(offset)) })
        } else {
            None
        }

        #[cfg(feature = "forbid_unsafe")]
        Chunk::from_slice(self.as_bytes().slice(offset..Chunk::SIZE + offset)?)
    }

    #[inline]
    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn read_byte_unchecked(&self, offset: usize) -> u8 {
        Chunk::from_ptr(self.as_ptr().add(offset))
    }

    #[inline]
    #[cfg(feature = "forbid_unsafe")]
    fn read_byte(&self, offset: usize) -> u8 {
        self.as_bytes()[offset]
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Option<&str> {
        self.get(range)
    }

    #[cfg(not(feature = "forbid_unsafe"))]
    #[inline]
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> &str {
        debug_assert!(
            range.start <= self.len() && range.end <= self.len(),
            "Reading out of bounds {:?} for {}!",
            range,
            self.len()
        );

        self.get_unchecked(range)
    }

    #[inline]
    fn find_boundary(&self, mut index: usize) -> usize {
        while !self.is_char_boundary(index) {
            index += 1;
        }

        index
    }

    #[inline]
    fn is_boundary(&self, index: usize) -> bool {
        self.is_char_boundary(index)
    }
}

impl Source for [u8] {
    type Slice<'a> = &'a [u8];

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn read<'a, Chunk>(&'a self, offset: usize) -> Option<Chunk>
    where
        Chunk: self::Chunk<'a>,
    {
        #[cfg(not(feature = "forbid_unsafe"))]
        if offset + (Chunk::SIZE - 1) < self.len() {
            Some(unsafe { Chunk::from_ptr(self.as_ptr().add(offset)) })
        } else {
            None
        }

        #[cfg(feature = "forbid_unsafe")]
        Chunk::from_slice(self.slice(offset..Chunk::SIZE + offset)?)
    }

    #[inline]
    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn read_byte_unchecked(&self, offset: usize) -> u8 {
        Chunk::from_ptr(self.as_ptr().add(offset))
    }

    #[inline]
    #[cfg(feature = "forbid_unsafe")]
    fn read_byte(&self, offset: usize) -> u8 {
        self[offset]
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Option<&[u8]> {
        self.get(range)
    }

    #[cfg(not(feature = "forbid_unsafe"))]
    #[inline]
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> &[u8] {
        debug_assert!(
            range.start <= self.len() && range.end <= self.len(),
            "Reading out of bounds {:?} for {}!",
            range,
            self.len()
        );

        self.get_unchecked(range)
    }

    #[inline]
    fn is_boundary(&self, index: usize) -> bool {
        index <= self.len()
    }
}

impl<T> Source for T
where
    T: Deref,
    <T as Deref>::Target: Source,
{
    type Slice<'a> = <T::Target as Source>::Slice<'a>
        where T: 'a;

    fn len(&self) -> usize {
        self.deref().len()
    }

    fn read<'a, Chunk>(&'a self, offset: usize) -> Option<Chunk>
    where
        Chunk: self::Chunk<'a>,
    {
        self.deref().read(offset)
    }

    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn read_byte_unchecked(&self, offset: usize) -> u8 {
        self.deref().read_byte_unchecked(offset)
    }

    #[cfg(feature = "forbid_unsafe")]
    fn read_byte(&self, offset: usize) -> u8 {
        self.deref().read_byte(offset)
    }

    fn slice(&self, range: Range<usize>) -> Option<Self::Slice<'_>> {
        self.deref().slice(range)
    }

    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> Self::Slice<'_> {
        self.deref().slice_unchecked(range)
    }

    fn is_boundary(&self, index: usize) -> bool {
        self.deref().is_boundary(index)
    }

    fn find_boundary(&self, index: usize) -> usize {
        self.deref().find_boundary(index)
    }
}

/// A fixed, statically sized chunk of data that can be read from the `Source`.
///
/// This is implemented for `u8`, as well as byte arrays `&[u8; 1]` to `&[u8; 32]`.
pub trait Chunk<'source>: Sized + Copy + PartialEq + Eq {
    /// Size of the chunk being accessed in bytes.
    const SIZE: usize;

    /// Create a chunk from a raw byte pointer.
    ///
    /// # Safety
    ///
    /// Raw byte pointer should point to a valid location in source.
    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn from_ptr(ptr: *const u8) -> Self;

    /// Create a chunk from a slice.
    /// Returns None if the slice is not long enough to produce the chunk.
    #[cfg(feature = "forbid_unsafe")]
    fn from_slice(s: &'source [u8]) -> Option<Self>;
}

impl<'source> Chunk<'source> for u8 {
    const SIZE: usize = 1;

    #[inline]
    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn from_ptr(ptr: *const u8) -> Self {
        *ptr
    }

    #[inline]
    #[cfg(feature = "forbid_unsafe")]
    fn from_slice(s: &'source [u8]) -> Option<Self> {
        s.first().copied()
    }
}

impl<'source, const N: usize> Chunk<'source> for &'source [u8; N] {
    const SIZE: usize = N;

    #[inline]
    #[cfg(not(feature = "forbid_unsafe"))]
    unsafe fn from_ptr(ptr: *const u8) -> Self {
        &*(ptr as *const [u8; N])
    }

    #[inline]
    #[cfg(feature = "forbid_unsafe")]
    fn from_slice(s: &'source [u8]) -> Option<Self> {
        s.slice(0..Self::SIZE).and_then(|x| x.try_into().ok())
    }
}
