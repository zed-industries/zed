//! A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).

use core::{
    borrow,
    char::DecodeUtf16Error,
    cmp::Ordering,
    fmt,
    fmt::{Arguments, Write},
    hash, iter,
    ops::{self, Range, RangeBounds},
    str::{self, Utf8Error},
};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

use crate::{
    len_type::LenType,
    vec::{OwnedVecStorage, Vec, VecInner, ViewVecStorage},
    CapacityError,
};

mod drain;
pub use drain::Drain;

/// A possible error value when converting a [`String`] from a UTF-16 byte slice.
///
/// This type is the error type for the [`from_utf16`] method on [`String`].
///
/// [`from_utf16`]: String::from_utf16
#[derive(Debug)]
pub enum FromUtf16Error {
    /// The capacity of the `String` is too small for the given operation.
    Capacity(CapacityError),
    /// Error decoding UTF-16.
    DecodeUtf16(DecodeUtf16Error),
}

impl fmt::Display for FromUtf16Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capacity(err) => write!(f, "{err}"),
            Self::DecodeUtf16(err) => write!(f, "invalid UTF-16: {err}"),
        }
    }
}

impl core::error::Error for FromUtf16Error {}

impl From<CapacityError> for FromUtf16Error {
    fn from(e: CapacityError) -> Self {
        Self::Capacity(e)
    }
}

mod storage {
    use super::{StringInner, StringView};
    use crate::{
        vec::{OwnedVecStorage, VecStorage, ViewVecStorage},
        LenType,
    };

    /// Trait defining how data for a String is stored.
    ///
    /// There's two implementations available:
    ///
    /// - [`OwnedStorage`]: stores the data in an array whose size is known at compile time.
    /// - [`ViewStorage`]: stores the data in an unsized slice
    ///
    /// This allows [`String`] to be generic over either sized or unsized storage. The
    /// [`string`](super) module contains a [`StringInner`] struct that's generic on
    /// [`StringStorage`], and two type aliases for convenience:
    ///
    /// - [`String<N>`](crate::string::String) = `StringInner<OwnedStorage<u8, N>>`
    /// - [`StringView<T>`](crate::string::StringView) = `StringInner<ViewStorage<u8>>`
    ///
    /// `String` can be unsized into `StrinsgView`, either by unsizing coercions such as `&mut
    /// String -> &mut StringView` or `Box<String> -> Box<StringView>`, or explicitly with
    /// [`.as_view()`](crate::string::String::as_view) or
    /// [`.as_mut_view()`](crate::string::String::as_mut_view).
    ///
    /// This trait is sealed, so you cannot implement it for your own types. You can only use
    /// the implementations provided by this crate.
    ///
    /// [`StringInner`]: super::StringInner
    /// [`String`]: super::String
    /// [`OwnedStorage`]: super::OwnedStorage
    /// [`ViewStorage`]: super::ViewStorage
    pub trait StringStorage: StringStorageSealed {}
    pub trait StringStorageSealed: VecStorage<u8> {
        fn as_string_view<LenT: LenType>(this: &StringInner<LenT, Self>) -> &StringView<LenT>
        where
            Self: StringStorage;
        fn as_string_mut_view<LenT: LenType>(
            this: &mut StringInner<LenT, Self>,
        ) -> &mut StringView<LenT>
        where
            Self: StringStorage;
    }

    impl<const N: usize> StringStorage for OwnedVecStorage<u8, N> {}
    impl<const N: usize> StringStorageSealed for OwnedVecStorage<u8, N> {
        fn as_string_view<LenT: LenType>(this: &StringInner<LenT, Self>) -> &StringView<LenT>
        where
            Self: StringStorage,
        {
            this
        }
        fn as_string_mut_view<LenT: LenType>(
            this: &mut StringInner<LenT, Self>,
        ) -> &mut StringView<LenT>
        where
            Self: StringStorage,
        {
            this
        }
    }

    impl StringStorage for ViewVecStorage<u8> {}

    impl StringStorageSealed for ViewVecStorage<u8> {
        fn as_string_view<LenT: LenType>(this: &StringInner<LenT, Self>) -> &StringView<LenT>
        where
            Self: StringStorage,
        {
            this
        }
        fn as_string_mut_view<LenT: LenType>(
            this: &mut StringInner<LenT, Self>,
        ) -> &mut StringView<LenT>
        where
            Self: StringStorage,
        {
            this
        }
    }
}

pub use storage::StringStorage;

/// Implementation of [`StringStorage`] that stores the data in an array whose size is known at
/// compile time.
pub type OwnedStorage<const N: usize> = OwnedVecStorage<u8, N>;
/// Implementation of [`StringStorage`] that stores the data in an unsized slice.
pub type ViewStorage = ViewVecStorage<u8>;

/// Base struct for [`String`] and [`StringView`], generic over the [`StringStorage`].
///
/// In most cases you should use [`String`] or [`StringView`] directly. Only use this
/// struct if you want to write code that's generic over both.
#[cfg_attr(feature = "zeroize", derive(Zeroize), zeroize(bound = "S: Zeroize"))]
pub struct StringInner<LenT: LenType, S: StringStorage + ?Sized> {
    vec: VecInner<u8, LenT, S>,
}

/// A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
pub type String<const N: usize, LenT = usize> = StringInner<LenT, OwnedStorage<N>>;

/// A dynamic capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
pub type StringView<LenT = usize> = StringInner<LenT, ViewStorage>;

impl<LenT: LenType, const N: usize> String<N, LenT> {
    /// Constructs a new, empty `String` with a fixed capacity of `N` bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// // allocate the string on the stack
    /// let mut s: String<4> = String::new();
    ///
    /// // allocate the string in a static variable
    /// static mut S: String<4> = String::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { vec: Vec::new() }
    }

    /// Decodes a UTF-16–encoded slice `v` into a `String`, returning [`Err`]
    /// if `v` contains any invalid data.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// // 𝄞music
    /// let v = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063];
    /// let s: String<10> = String::from_utf16(v).unwrap();
    /// assert_eq!(s, "𝄞music");
    ///
    /// // 𝄞mu<invalid>ic
    /// let v = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    /// assert!(String::<10>::from_utf16(v).is_err());
    /// ```
    #[inline]
    pub fn from_utf16(v: &[u16]) -> Result<Self, FromUtf16Error> {
        let mut s = Self::new();

        for c in char::decode_utf16(v.iter().cloned()) {
            match c {
                Ok(c) => {
                    s.push(c).map_err(|_| CapacityError)?;
                }
                Err(err) => {
                    return Err(FromUtf16Error::DecodeUtf16(err));
                }
            }
        }

        Ok(s)
    }

    /// Convert UTF-8 bytes into a `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::{String, Vec};
    ///
    /// let mut sparkle_heart = Vec::<u8, 4>::new();
    /// sparkle_heart.extend_from_slice(&[240, 159, 146, 150]);
    ///
    /// let sparkle_heart: String<4> = String::from_utf8(sparkle_heart)?;
    /// assert_eq!("💖", sparkle_heart);
    /// # Ok::<(), core::str::Utf8Error>(())
    /// ```
    ///
    /// Invalid UTF-8:
    ///
    /// ```
    /// use core::str::Utf8Error;
    /// use heapless::{String, Vec};
    ///
    /// let mut vec = Vec::<u8, 4>::new();
    /// vec.extend_from_slice(&[0, 159, 146, 150]);
    ///
    /// let e: Utf8Error = String::from_utf8(vec).unwrap_err();
    /// assert_eq!(e.valid_up_to(), 1);
    /// # Ok::<(), core::str::Utf8Error>(())
    /// ```
    #[inline]
    pub fn from_utf8(vec: Vec<u8, N, LenT>) -> Result<Self, Utf8Error> {
        core::str::from_utf8(&vec)?;

        // SAFETY: UTF-8 invariant has just been checked by `str::from_utf8`.
        Ok(unsafe { Self::from_utf8_unchecked(vec) })
    }

    /// Convert UTF-8 bytes into a `String`, without checking that the string
    /// contains valid UTF-8.
    ///
    /// # Safety
    ///
    /// The bytes passed in must be valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::{String, Vec};
    ///
    /// let mut sparkle_heart = Vec::<u8, 4>::new();
    /// sparkle_heart.extend_from_slice(&[240, 159, 146, 150]);
    ///
    /// // Safety: `sparkle_heart` Vec is known to contain valid UTF-8
    /// let sparkle_heart: String<4> = unsafe { String::from_utf8_unchecked(sparkle_heart) };
    /// assert_eq!("💖", sparkle_heart);
    /// ```
    #[inline]
    pub unsafe fn from_utf8_unchecked(vec: Vec<u8, N, LenT>) -> Self {
        Self { vec }
    }

    /// Converts a `String` into a byte vector.
    ///
    /// This consumes the `String`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let s: String<4> = String::try_from("ab")?;
    /// let b = s.into_bytes();
    /// assert!(b.len() == 2);
    ///
    /// assert_eq!(&[b'a', b'b'], &b[..]);
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn into_bytes(self) -> Vec<u8, N, LenT> {
        self.vec
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> StringInner<LenT, S> {
    /// Removes the specified range from the string in bulk, returning all
    /// removed characters as an iterator.
    ///
    /// The returned iterator keeps a mutable borrow on the string to optimize
    /// its implementation.
    ///
    /// # Panics
    ///
    /// Panics if the starting point or end point do not lie on a [`char`]
    /// boundary, or if they're out of bounds.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to
    /// [`core::mem::forget`], for example), the string may still contain a copy
    /// of any drained characters, or may have lost characters arbitrarily,
    /// including characters outside the range.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s = String::<32>::try_from("α is alpha, β is beta").unwrap();
    /// let beta_offset = s.find('β').unwrap_or(s.len());
    ///
    /// // Remove the range up until the β from the string
    /// let t: String<32> = s.drain(..beta_offset).collect();
    /// assert_eq!(t, "α is alpha, ");
    /// assert_eq!(s, "β is beta");
    ///
    /// // A full range clears the string, like `clear()` does
    /// s.drain(..);
    /// assert_eq!(s, "");
    /// ```
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, LenT>
    where
        R: RangeBounds<usize>,
    {
        // Memory safety
        //
        // The `String` version of `Drain` does not have the memory safety issues
        // of the `Vec` version. The data is just plain bytes.
        // Because the range removal happens in `Drop`, if the `Drain` iterator is leaked,
        // the removal will not happen.
        let Range { start, end } = crate::slice::range(range, ..self.len());
        assert!(self.is_char_boundary(start));
        assert!(self.is_char_boundary(end));

        // Take out two simultaneous borrows. The &mut String won't be accessed
        // until iteration is over, in Drop.
        let self_ptr = self.as_mut_view() as *mut _;
        // SAFETY: `slice::range` and `is_char_boundary` do the appropriate bounds checks.
        let chars_iter = unsafe { self.get_unchecked(start..end) }.chars();

        Drain {
            start: LenT::from_usize(start),
            end: LenT::from_usize(end),
            iter: chars_iter,
            string: self_ptr,
        }
    }

    /// Get a reference to the `String`, erasing the `N` const-generic.
    ///
    ///
    /// ```rust
    /// # use heapless::string::{String, StringView};
    /// let s: String<10, _> = String::try_from("hello").unwrap();
    /// let view: &StringView = s.as_view();
    /// ```
    ///
    /// It is often preferable to do the same through type coerction, since `String<N>` implements
    /// `Unsize<StringView>`:
    ///
    /// ```rust
    /// # use heapless::string::{String, StringView};
    /// let s: String<10, _> = String::try_from("hello").unwrap();
    /// let view: &StringView = &s;
    /// ```
    #[inline]
    pub fn as_view(&self) -> &StringView<LenT> {
        S::as_string_view(self)
    }

    /// Get a mutable reference to the `String`, erasing the `N` const-generic.
    ///
    ///
    /// ```rust
    /// # use heapless::string::{String, StringView};
    /// let mut s: String<10> = String::try_from("hello").unwrap();
    /// let view: &mut StringView = s.as_mut_view();
    /// ```
    ///
    /// It is often preferable to do the same through type coerction, since `String<N>` implements
    /// `Unsize<StringView>`:
    ///
    /// ```rust
    /// # use heapless::string::{String, StringView};
    /// let mut s: String<10> = String::try_from("hello").unwrap();
    /// let view: &mut StringView = &mut s;
    /// ```
    #[inline]
    pub fn as_mut_view(&mut self) -> &mut StringView<LenT> {
        S::as_string_mut_view(self)
    }

    /// Extracts a string slice containing the entire string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<4> = String::try_from("ab")?;
    /// assert!(s.as_str() == "ab");
    ///
    /// let _s = s.as_str();
    /// // s.push('c'); // <- cannot borrow `s` as mutable because it is also borrowed as immutable
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.vec.as_slice()) }
    }

    /// Converts a `String` into a mutable string slice.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<4> = String::try_from("ab")?;
    /// let s = s.as_mut_str();
    /// s.make_ascii_uppercase();
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe { str::from_utf8_unchecked_mut(self.vec.as_mut_slice()) }
    }

    /// Returns a mutable reference to the contents of this `String`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed
    /// to it are valid UTF-8. If this constraint is violated, it may cause
    /// memory unsafety issues with future users of the `String`, as the rest of
    /// the library assumes that `String`s are valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("hello")?;
    ///
    /// unsafe {
    ///     let vec = s.as_mut_vec();
    ///     assert_eq!(&[104, 101, 108, 108, 111][..], &vec[..]);
    ///
    ///     vec.reverse();
    /// }
    /// assert_eq!(s, "olleh");
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    pub unsafe fn as_mut_vec(&mut self) -> &mut VecInner<u8, LenT, S> {
        &mut self.vec
    }

    /// Appends a given string slice onto the end of this `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("foo")?;
    ///
    /// assert!(s.push_str("bar").is_ok());
    ///
    /// assert_eq!("foobar", s);
    ///
    /// assert!(s.push_str("tender").is_err());
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn push_str(&mut self, string: &str) -> Result<(), CapacityError> {
        self.vec.extend_from_slice(string.as_bytes())
    }

    /// Returns the maximum number of elements the String can hold.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<4> = String::new();
    /// assert!(s.capacity() == 4);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Appends the given [`char`] to the end of this `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("abc")?;
    ///
    /// s.push('1').unwrap();
    /// s.push('2').unwrap();
    /// s.push('3').unwrap();
    ///
    /// assert!("abc123" == s.as_str());
    ///
    /// assert_eq!("abc123", s);
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn push(&mut self, c: char) -> Result<(), CapacityError> {
        match c.len_utf8() {
            1 => self.vec.push(c as u8).map_err(|_| CapacityError),
            _ => self
                .vec
                .extend_from_slice(c.encode_utf8(&mut [0; 4]).as_bytes()),
        }
    }

    /// Shortens this `String` to the specified length.
    ///
    /// If `new_len` is greater than the string's current length, this has no
    /// effect.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the string
    ///
    /// # Panics
    ///
    /// Panics if `new_len` does not lie on a [`char`] boundary.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("hello")?;
    ///
    /// s.truncate(2);
    ///
    /// assert_eq!("he", s);
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        if new_len <= self.len() {
            assert!(self.is_char_boundary(new_len));
            self.vec.truncate(new_len);
        }
    }

    /// Removes the last character from the string buffer and returns it.
    ///
    /// Returns [`None`] if this `String` is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("foo")?;
    ///
    /// assert_eq!(s.pop(), Some('o'));
    /// assert_eq!(s.pop(), Some('o'));
    /// assert_eq!(s.pop(), Some('f'));
    ///
    /// assert_eq!(s.pop(), None);
    /// Ok::<(), heapless::CapacityError>(())
    /// ```
    pub fn pop(&mut self) -> Option<char> {
        let ch = self.chars().next_back()?;

        // pop bytes that correspond to `ch`
        for _ in 0..ch.len_utf8() {
            unsafe {
                self.vec.pop_unchecked();
            }
        }

        Some(ch)
    }

    /// Removes a [`char`] from this `String` at a byte position and returns it.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(n).
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than or equal to the `String`'s length,
    /// or if it does not lie on a [`char`] boundary.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("foo").unwrap();
    ///
    /// assert_eq!(s.remove(0), 'f');
    /// assert_eq!(s.remove(1), 'o');
    /// assert_eq!(s.remove(0), 'o');
    /// ```
    #[inline]
    pub fn remove(&mut self, index: usize) -> char {
        let ch = self[index..]
            .chars()
            .next()
            .unwrap_or_else(|| panic!("cannot remove a char from the end of a string"));

        let next = index + ch.len_utf8();
        let len = self.len();
        let ptr = self.vec.as_mut_ptr();
        unsafe {
            core::ptr::copy(ptr.add(next), ptr.add(index), len - next);
            self.vec.set_len(len - (next - index));
        }
        ch
    }

    /// Truncates this `String`, removing all contents.
    ///
    /// While this means the `String` will have a length of zero, it does not
    /// touch its capacity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("foo")?;
    ///
    /// s.clear();
    ///
    /// assert!(s.is_empty());
    /// assert_eq!(0, s.len());
    /// assert_eq!(8, s.capacity());
    /// Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Inserts a character into this `String` at a byte position.
    ///
    /// This is an *O*(*n*) operation as it requires copying every element in the
    /// buffer.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than the `String`'s length, or if it does not
    /// lie on a [`char`] boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<4> = String::new();
    ///
    /// s.insert(0, 'f').unwrap();
    /// s.insert(1, 'o').unwrap();
    /// s.insert(2, 'o').unwrap();
    ///
    /// assert_eq!("foo", s);
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn insert(&mut self, idx: usize, ch: char) -> Result<(), CapacityError> {
        assert!(self.is_char_boundary(idx), "index must be a char boundary");

        let len = self.len();
        let ch_len = ch.len_utf8();

        // Check if there is enough capacity
        if len + ch_len > self.capacity() {
            return Err(CapacityError);
        }

        // SAFETY: Move the bytes starting from `idx` to their new location `ch_len`
        // bytes ahead. This is safe because we checked `len + ch_len` does not
        // exceed the capacity and `idx` is a char boundary.
        unsafe {
            let ptr = self.vec.as_mut_ptr();
            core::ptr::copy(ptr.add(idx), ptr.add(idx + ch_len), len - idx);
        }

        // SAFETY: Encode the character into the vacated region if `idx != len`,
        // or into the uninitialized spare capacity otherwise. This is safe
        // because `is_char_boundary` checks that `idx <= len`, and we checked that
        // `(idx + ch_len)` does not exceed the capacity.
        unsafe {
            let buf = core::slice::from_raw_parts_mut(self.vec.as_mut_ptr().add(idx), ch_len);
            ch.encode_utf8(buf);
        }

        // SAFETY: Update the length to include the newly added bytes. This is
        // safe because we checked that `len + ch_len` does not exceed the capacity.
        unsafe {
            self.vec.set_len(len + ch_len);
        }

        Ok(())
    }

    /// Inserts a string slice into this `String` at a byte position.
    ///
    /// This is an *O*(*n*) operation as it requires copying every element in the
    /// buffer.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than the `String`'s length, or if it does not
    /// lie on a [`char`] boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s: String<8> = String::try_from("bar")?;
    ///
    /// s.insert_str(0, "foo")?;
    ///
    /// assert_eq!("foobar", s);
    /// # Ok::<(), heapless::CapacityError>(())
    /// ```
    #[inline]
    pub fn insert_str(&mut self, idx: usize, string: &str) -> Result<(), CapacityError> {
        assert!(self.is_char_boundary(idx), "index must be a char boundary");

        let len = self.len();
        let string_len = string.len();

        // Check if there is enough capacity
        if len + string_len > self.capacity() {
            return Err(CapacityError);
        }

        // SAFETY: Move the bytes starting from `idx` to their new location
        // `string_len` bytes ahead. This is safe because we checked there is
        // sufficient capacity, and `idx` is a char boundary.
        unsafe {
            let ptr = self.vec.as_mut_ptr();
            core::ptr::copy(ptr.add(idx), ptr.add(idx + string_len), len - idx);
        }

        // SAFETY: Copy the new string slice into the vacated region if `idx != len`,
        // or into the uninitialized spare capacity otherwise. The borrow checker
        // ensures that the source and destination do not overlap.
        unsafe {
            core::ptr::copy_nonoverlapping(
                string.as_ptr(),
                self.vec.as_mut_ptr().add(idx),
                string_len,
            );
        }

        // SAFETY: Update the length to include the newly added bytes.
        unsafe {
            self.vec.set_len(len + string_len);
        }

        Ok(())
    }
}

impl<LenT: LenType, const N: usize> Default for String<N, LenT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, LenT: LenType, const N: usize> TryFrom<&'a str> for String<N, LenT> {
    type Error = CapacityError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut new = Self::new();
        new.push_str(s)?;
        Ok(new)
    }
}

impl<LenT: LenType, const N: usize> str::FromStr for String<N, LenT> {
    type Err = CapacityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new = Self::new();
        new.push_str(s)?;
        Ok(new)
    }
}

impl<LenT: LenType, const N: usize> iter::FromIterator<char> for String<N, LenT> {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut new = Self::new();
        for c in iter {
            new.push(c).unwrap();
        }
        new
    }
}

impl<'a, LenT: LenType, const N: usize> iter::FromIterator<&'a char> for String<N, LenT> {
    fn from_iter<T: IntoIterator<Item = &'a char>>(iter: T) -> Self {
        let mut new = Self::new();
        for c in iter {
            new.push(*c).unwrap();
        }
        new
    }
}

impl<'a, LenT: LenType, const N: usize> iter::FromIterator<&'a str> for String<N, LenT> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut new = Self::new();
        for c in iter {
            new.push_str(c).unwrap();
        }
        new
    }
}

impl<LenT: LenType, const N: usize> Clone for String<N, LenT> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
        }
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> fmt::Debug for StringInner<LenT, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Debug>::fmt(self, f)
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> fmt::Display for StringInner<LenT, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Display>::fmt(self, f)
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> hash::Hash for StringInner<LenT, S> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        <str as hash::Hash>::hash(self, hasher);
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> fmt::Write for StringInner<LenT, S> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.push_str(s).map_err(|_| fmt::Error)
    }

    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.push(c).map_err(|_| fmt::Error)
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> ops::Deref for StringInner<LenT, S> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> ops::DerefMut for StringInner<LenT, S> {
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> borrow::Borrow<str> for StringInner<LenT, S> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl<LenT: LenType, S: StringStorage + ?Sized> borrow::BorrowMut<str> for StringInner<LenT, S> {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> AsRef<str> for StringInner<LenT, S> {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> AsRef<[u8]> for StringInner<LenT, S> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<LenT1: LenType, LenT2: LenType, S1: StringStorage + ?Sized, S2: StringStorage + ?Sized>
    PartialEq<StringInner<LenT1, S1>> for StringInner<LenT2, S2>
{
    fn eq(&self, rhs: &StringInner<LenT1, S1>) -> bool {
        str::eq(&**self, &**rhs)
    }
}

// String<N> == str
impl<LenT: LenType, S: StringStorage + ?Sized> PartialEq<str> for StringInner<LenT, S> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        str::eq(self, other)
    }
}

// String<N> == &'str
impl<LenT: LenType, S: StringStorage + ?Sized> PartialEq<&str> for StringInner<LenT, S> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        str::eq(self, &other[..])
    }
}

// str == String<N>
impl<LenT: LenType, S: StringStorage + ?Sized> PartialEq<StringInner<LenT, S>> for str {
    #[inline]
    fn eq(&self, other: &StringInner<LenT, S>) -> bool {
        Self::eq(self, &other[..])
    }
}

// &'str == String<N>
impl<LenT: LenType, S: StringStorage + ?Sized> PartialEq<StringInner<LenT, S>> for &str {
    #[inline]
    fn eq(&self, other: &StringInner<LenT, S>) -> bool {
        str::eq(self, &other[..])
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> Eq for StringInner<LenT, S> {}

impl<LenT1: LenType, LenT2: LenType, S1: StringStorage + ?Sized, S2: StringStorage + ?Sized>
    PartialOrd<StringInner<LenT1, S1>> for StringInner<LenT2, S2>
{
    #[inline]
    fn partial_cmp(&self, other: &StringInner<LenT1, S1>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> Ord for StringInner<LenT, S> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

/// Equivalent to [`format`](https://doc.rust-lang.org/std/fmt/fn.format.html).
///
/// Please note that using [`format!`] might be preferable.
///
/// # Errors
///
/// There are two possible error cases. Both return the unit type [`core::fmt::Error`].
///
/// - In case the formatting exceeds the string's capacity. This error does not exist in
/// the standard library as the string would just grow.
/// - If a formatting trait implementation returns an error. The standard library panics
/// in this case.
///
/// [`format!`]: crate::format!
#[doc(hidden)]
pub fn format<const N: usize, LenT: LenType>(
    args: Arguments<'_>,
) -> Result<String<N, LenT>, fmt::Error> {
    fn format_inner<const N: usize, LenT: LenType>(
        args: Arguments<'_>,
    ) -> Result<String<N, LenT>, fmt::Error> {
        let mut output = String::new();
        output.write_fmt(args)?;
        Ok(output)
    }

    args.as_str().map_or_else(
        || format_inner(args),
        |s| s.try_into().map_err(|_| fmt::Error),
    )
}

/// Macro that creates a fixed capacity [`String`]. Equivalent to [`format!`](https://doc.rust-lang.org/std/macro.format.html).
///
/// The macro's arguments work in the same way as the regular macro.
///
/// It is possible to explicitly specify the capacity of the returned string as the first argument.
/// In this case it is necessary to disambiguate by separating the capacity with a semicolon.
///
/// # Errors
///
/// There are two possible error cases. Both return the unit type [`core::fmt::Error`].
///
/// - In case the formatting exceeds the string's capacity. This error does not exist in the
///   standard library as the string would just grow.
/// - If a formatting trait implementation returns an error. The standard library panics in this
///   case.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), core::fmt::Error> {
/// use heapless::{format, String};
///
/// // Notice semicolon instead of comma!
/// format!(4; "test")?;
/// format!(15; "hello {}", "world!")?;
/// format!(20; "x = {}, y = {y}", 10, y = 30)?;
/// let (x, y) = (1, 2);
/// format!(12; "{x} + {y} = 3")?;
///
/// let implicit: String<10> = format!("speed = {}", 7)?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! format {
    // Without semicolon as separator to disambiguate between arms, Rust just
    // chooses the first so that the format string would land in $max.
    ($max:expr; $lenT:path; $($arg:tt)*) => {{
        let res = $crate::_export::format::<$max, $lenT>(core::format_args!($($arg)*));
        res
    }};
    ($max:expr; $($arg:tt)*) => {{
        let res = $crate::_export::format::<$max, usize>(core::format_args!($($arg)*));
        res
    }};
    ($($arg:tt)*) => {{
        let res = $crate::_export::format(core::format_args!($($arg)*));
        res
    }};
}

macro_rules! impl_try_from_num {
    ($num:ty, $size:expr) => {
        impl<LenT: LenType, const N: usize> core::convert::TryFrom<$num> for String<N, LenT> {
            type Error = ();
            fn try_from(s: $num) -> Result<Self, Self::Error> {
                let mut new = String::new();
                write!(&mut new, "{}", s).map_err(|_| ())?;
                Ok(new)
            }
        }
    };
}

impl_try_from_num!(i8, 4);
impl_try_from_num!(i16, 6);
impl_try_from_num!(i32, 11);
impl_try_from_num!(i64, 20);

impl_try_from_num!(u8, 3);
impl_try_from_num!(u16, 5);
impl_try_from_num!(u32, 10);
impl_try_from_num!(u64, 20);

#[cfg(test)]
mod tests {
    use crate::{CapacityError, String, Vec};

    #[test]
    fn static_new() {
        static mut _S: String<8> = String::new();
    }

    #[test]
    fn clone() {
        let s1: String<20> = String::try_from("abcd").unwrap();
        let mut s2 = s1.clone();
        s2.push_str(" efgh").unwrap();

        assert_eq!(s1, "abcd");
        assert_eq!(s2, "abcd efgh");
    }

    #[test]
    fn cmp() {
        let s1: String<4> = String::try_from("abcd").unwrap();
        let s2: String<4> = String::try_from("zzzz").unwrap();

        assert!(s1 < s2);
    }

    #[test]
    fn cmp_heterogenous_size() {
        let s1: String<4> = String::try_from("abcd").unwrap();
        let s2: String<8> = String::try_from("zzzz").unwrap();

        assert!(s1 < s2);
    }

    #[test]
    fn debug() {
        use core::fmt::Write;

        let s: String<8> = String::try_from("abcd").unwrap();
        let mut std_s = std::string::String::new();
        write!(std_s, "{s:?}").unwrap();
        assert_eq!("\"abcd\"", std_s);
    }

    #[test]
    fn display() {
        use core::fmt::Write;

        let s: String<8> = String::try_from("abcd").unwrap();
        let mut std_s = std::string::String::new();
        write!(std_s, "{s}").unwrap();
        assert_eq!("abcd", std_s);
    }

    #[test]
    fn empty() {
        let s: String<4> = String::new();
        assert!(s.capacity() == 4);
        assert_eq!(s, "");
        assert_eq!(s.len(), 0);
        assert_ne!(s.len(), 4);
    }

    #[test]
    fn try_from() {
        let s: String<4> = String::try_from("123").unwrap();
        assert!(s.len() == 3);
        assert_eq!(s, "123");

        let _: CapacityError = String::<2>::try_from("123").unwrap_err();
    }

    #[test]
    fn from_str() {
        use core::str::FromStr;

        let s: String<4> = String::<4>::from_str("123").unwrap();
        assert!(s.len() == 3);
        assert_eq!(s, "123");

        let _: CapacityError = String::<2>::from_str("123").unwrap_err();
    }

    #[test]
    fn from_iter() {
        let mut v: Vec<char, 5> = Vec::new();
        v.push('h').unwrap();
        v.push('e').unwrap();
        v.push('l').unwrap();
        v.push('l').unwrap();
        v.push('o').unwrap();
        let string1: String<5> = v.iter().collect(); //&char
        let string2: String<5> = "hello".chars().collect(); //char
        assert_eq!(string1, "hello");
        assert_eq!(string2, "hello");
    }

    #[test]
    #[should_panic]
    fn from_panic() {
        let _: String<4> = String::try_from("12345").unwrap();
    }

    #[test]
    fn try_from_num() {
        let v: String<20> = String::try_from(18446744073709551615_u64).unwrap();
        assert_eq!(v, "18446744073709551615");

        let _: () = String::<2>::try_from(18446744073709551615_u64).unwrap_err();
    }

    #[test]
    fn into_bytes() {
        let s: String<4> = String::try_from("ab").unwrap();
        let b: Vec<u8, 4> = s.into_bytes();
        assert_eq!(b.len(), 2);
        assert_eq!(b"ab", &b[..]);
    }

    #[test]
    fn as_str() {
        let s: String<4> = String::try_from("ab").unwrap();

        assert_eq!(s.as_str(), "ab");
        // should be moved to fail test
        //    let _s = s.as_str();
        // s.push('c'); // <- cannot borrow `s` as mutable because it is also borrowed as immutable
    }

    #[test]
    fn as_mut_str() {
        let mut s: String<4> = String::try_from("ab").unwrap();
        let s = s.as_mut_str();
        s.make_ascii_uppercase();
        assert_eq!(s, "AB");
    }

    #[test]
    fn push_str() {
        let mut s: String<8> = String::try_from("foo").unwrap();
        assert!(s.push_str("bar").is_ok());
        assert_eq!("foobar", s);
        assert_eq!(s, "foobar");
        assert!(s.push_str("tender").is_err());
        assert_eq!("foobar", s);
        assert_eq!(s, "foobar");
    }

    #[test]
    fn push() {
        let mut s: String<6> = String::try_from("abc").unwrap();
        assert!(s.push('1').is_ok());
        assert!(s.push('2').is_ok());
        assert!(s.push('3').is_ok());
        assert!(s.push('4').is_err());
        assert!("abc123" == s.as_str());
    }

    #[test]
    fn as_bytes() {
        let s: String<8> = String::try_from("hello").unwrap();
        assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
    }

    #[test]
    fn truncate() {
        let mut s: String<8> = String::try_from("hello").unwrap();
        s.truncate(6);
        assert_eq!(s.len(), 5);
        s.truncate(2);
        assert_eq!(s.len(), 2);
        assert_eq!("he", s);
        assert_eq!(s, "he");
    }

    #[test]
    fn pop() {
        let mut s: String<8> = String::try_from("foo").unwrap();
        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('f'));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn pop_uenc() {
        let mut s: String<8> = String::try_from("é").unwrap();
        assert_eq!(s.len(), 3);
        match s.pop() {
            Some(c) => {
                assert_eq!(s.len(), 1);
                assert_eq!(c, '\u{0301}'); // acute accent of e
            }
            None => panic!(),
        };
    }

    #[test]
    fn is_empty() {
        let mut v: String<8> = String::new();
        assert!(v.is_empty());
        let _ = v.push('a');
        assert!(!v.is_empty());
    }

    #[test]
    fn clear() {
        let mut s: String<8> = String::try_from("foo").unwrap();
        s.clear();
        assert!(s.is_empty());
        assert_eq!(0, s.len());
        assert_eq!(8, s.capacity());
    }

    #[test]
    fn remove() {
        let mut s: String<8> = String::try_from("foo").unwrap();
        assert_eq!(s.remove(0), 'f');
        assert_eq!(s.as_str(), "oo");
    }

    #[test]
    fn remove_uenc() {
        let mut s: String<8> = String::try_from("ĝėēƶ").unwrap();
        assert_eq!(s.remove(2), 'ė');
        assert_eq!(s.remove(2), 'ē');
        assert_eq!(s.remove(2), 'ƶ');
        assert_eq!(s.as_str(), "ĝ");
    }

    #[test]
    fn remove_uenc_combo_characters() {
        let mut s: String<8> = String::try_from("héy").unwrap();
        assert_eq!(s.remove(2), '\u{0301}');
        assert_eq!(s.as_str(), "hey");
    }

    #[test]
    fn format() {
        let number = 5;
        let float = 3.12;
        let formatted = format!(15; "{:0>3} plus {float}", number).unwrap();
        assert_eq!(formatted, "005 plus 3.12");
    }
    #[test]
    fn format_inferred_capacity() {
        let number = 5;
        let float = 3.12;
        let formatted: String<15> = format!("{:0>3} plus {float}", number).unwrap();
        assert_eq!(formatted, "005 plus 3.12");
    }

    #[test]
    fn format_overflow() {
        let i = 1234567;
        let formatted = format!(4; "13{}", i);
        assert_eq!(formatted, Err(core::fmt::Error));
    }

    #[test]
    fn format_plain_string_overflow() {
        let formatted = format!(2; "123");
        assert_eq!(formatted, Err(core::fmt::Error));
    }

    #[test]
    fn insert() {
        let mut s: String<6> = String::try_from("123").unwrap();
        assert!(s.insert(0, 'a').is_ok());
        assert_eq!(s, "a123");

        assert!(s.insert(2, 'b').is_ok());
        assert_eq!(s, "a1b23");

        assert!(s.insert(s.len(), '4').is_ok());
        assert_eq!(s, "a1b234");

        assert_eq!(s.len(), 6);
        assert!(s.insert(0, 'd').is_err());
        assert_eq!(s, "a1b234");
    }

    #[test]
    fn insert_unicode() {
        let mut s: String<21> = String::try_from("ĝėēƶ").unwrap();
        let idx = s.find("ė").unwrap();

        assert!(s.insert(idx, '🦀').is_ok());
        assert_eq!(s, "ĝ🦀ėēƶ");

        s.insert(s.len(), '🦀').unwrap();
        assert_eq!(s, "ĝ🦀ėēƶ🦀");

        s.insert(0, '🦀').unwrap();
        assert_eq!(s, "🦀ĝ🦀ėēƶ🦀");

        assert_eq!(s.len(), 20);
        assert_eq!('ƶ'.len_utf8(), 2);
        assert!(s.insert(0, 'ƶ').is_err());
        assert_eq!(s, "🦀ĝ🦀ėēƶ🦀");
    }

    #[test]
    #[should_panic = "index must be a char boundary"]
    fn insert_at_non_char_boundary_panics() {
        let mut s: String<8> = String::try_from("é").unwrap();
        _ = s.insert(1, 'a');
    }

    #[test]
    #[should_panic = "index must be a char boundary"]
    fn insert_beyond_length_panics() {
        let mut s: String<8> = String::try_from("a").unwrap();
        _ = s.insert(2, 'a');
    }

    #[test]
    fn insert_str() {
        let mut s: String<14> = String::try_from("bar").unwrap();
        assert!(s.insert_str(0, "foo").is_ok());
        assert_eq!(s, "foobar");

        assert!(s.insert_str(3, "baz").is_ok());
        assert_eq!(s, "foobazbar");

        assert!(s.insert_str(s.len(), "end").is_ok());
        assert_eq!(s, "foobazbarend");

        assert_eq!(s.len(), 12);
        assert!(s.insert_str(0, "def").is_err());
        assert_eq!(s, "foobazbarend");
    }

    #[test]
    fn insert_str_unicode() {
        let mut s: String<20> = String::try_from("Héllô").unwrap();
        let idx = s.find("lô").unwrap();

        assert!(s.insert_str(idx, "p, í'm ").is_ok());
        assert_eq!(s, "Hélp, í'm lô");

        assert!(s.insert_str(s.len(), "st").is_ok());
        assert_eq!(s, "Hélp, í'm lôst");

        assert_eq!(s.len(), 17);
        assert_eq!("🦀".len(), 4);
        assert!(s.insert_str(0, "🦀").is_err());
        assert_eq!(s, "Hélp, í'm lôst");
    }

    #[test]
    #[should_panic = "index must be a char boundary"]
    fn insert_str_at_non_char_boundary_panics() {
        let mut s: String<8> = String::try_from("é").unwrap();
        _ = s.insert_str(1, "a");
    }

    #[test]
    #[should_panic = "index must be a char boundary"]
    fn insert_str_beyond_length_panics() {
        let mut s: String<8> = String::try_from("a").unwrap();
        _ = s.insert_str(2, "a");
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_string_zeroize() {
        use zeroize::Zeroize;

        let mut s: String<32> = String::try_from("sensitive_password").unwrap();

        assert_eq!(s.as_str(), "sensitive_password");
        assert!(!s.is_empty());
        assert_eq!(s.len(), 18);

        s.truncate(9);
        assert_eq!(s.as_str(), "sensitive");
        assert_eq!(s.len(), 9);

        // zeroized using Vec's implementation
        s.zeroize();

        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }
}
