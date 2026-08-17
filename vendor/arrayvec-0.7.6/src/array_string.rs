use std::borrow::{Borrow, BorrowMut};
use std::cmp;
use std::convert::TryFrom;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
#[cfg(feature="std")]
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::str::FromStr;
use std::str::Utf8Error;

use crate::CapacityError;
use crate::LenUint;
use crate::char::encode_utf8;
use crate::utils::MakeMaybeUninit;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize, Serializer, Deserializer};


/// A string with a fixed capacity.
///
/// The `ArrayString` is a string backed by a fixed size array. It keeps track
/// of its length, and is parameterized by `CAP` for the maximum capacity.
///
/// `CAP` is of type `usize` but is range limited to `u32::MAX`; attempting to create larger
/// arrayvecs with larger capacity will panic.
///
/// The string is a contiguous value that you can store directly on the stack
/// if needed.
#[derive(Copy)]
#[repr(C)]
pub struct ArrayString<const CAP: usize> {
    // the `len` first elements of the array are initialized
    len: LenUint,
    xs: [MaybeUninit<u8>; CAP],
}

impl<const CAP: usize> Default for ArrayString<CAP>
{
    /// Return an empty `ArrayString`
    fn default() -> ArrayString<CAP> {
        ArrayString::new()
    }
}

impl<const CAP: usize> ArrayString<CAP>
{
    /// Create a new empty `ArrayString`.
    ///
    /// Capacity is inferred from the type parameter.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<16>::new();
    /// string.push_str("foo");
    /// assert_eq!(&string[..], "foo");
    /// assert_eq!(string.capacity(), 16);
    /// ```
    pub fn new() -> ArrayString<CAP> {
        assert_capacity_limit!(CAP);
        unsafe {
            ArrayString { xs: MaybeUninit::uninit().assume_init(), len: 0 }
        }
    }

    /// Create a new empty `ArrayString` (const fn).
    ///
    /// Capacity is inferred from the type parameter.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// static ARRAY: ArrayString<1024> = ArrayString::new_const();
    /// ```
    pub const fn new_const() -> ArrayString<CAP> {
        assert_capacity_limit_const!(CAP);
        ArrayString { xs: MakeMaybeUninit::ARRAY, len: 0 }
    }

    /// Return the length of the string.
    #[inline]
    pub const fn len(&self) -> usize { self.len as usize }

    /// Returns whether the string is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    /// Create a new `ArrayString` from a `str`.
    ///
    /// Capacity is inferred from the type parameter.
    ///
    /// **Errors** if the backing array is not large enough to fit the string.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<3>::from("foo").unwrap();
    /// assert_eq!(&string[..], "foo");
    /// assert_eq!(string.len(), 3);
    /// assert_eq!(string.capacity(), 3);
    /// ```
    pub fn from(s: &str) -> Result<Self, CapacityError<&str>> {
        let mut arraystr = Self::new();
        arraystr.try_push_str(s)?;
        Ok(arraystr)
    }

    /// Create a new `ArrayString` from a byte string literal.
    ///
    /// **Errors** if the byte string literal is not valid UTF-8.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let string = ArrayString::from_byte_string(b"hello world").unwrap();
    /// ```
    pub fn from_byte_string(b: &[u8; CAP]) -> Result<Self, Utf8Error> {
        let len = str::from_utf8(b)?.len();
        debug_assert_eq!(len, CAP);
        let mut vec = Self::new();
        unsafe {
            (b as *const [u8; CAP] as *const [MaybeUninit<u8>; CAP])
                .copy_to_nonoverlapping(&mut vec.xs as *mut [MaybeUninit<u8>; CAP], 1);
            vec.set_len(CAP);
        }
        Ok(vec)
    }

    /// Create a new `ArrayString` value fully filled with ASCII NULL characters (`\0`). Useful
    /// to be used as a buffer to collect external data or as a buffer for intermediate processing.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let string = ArrayString::<16>::zero_filled();
    /// assert_eq!(string.len(), 16);
    /// ```
    #[inline]
    pub fn zero_filled() -> Self {
        assert_capacity_limit!(CAP);
        // SAFETY: `assert_capacity_limit` asserts that `len` won't overflow and
        // `zeroed` fully fills the array with nulls.
        unsafe {
            ArrayString {
                xs: MaybeUninit::zeroed().assume_init(),
                len: CAP as _
            }
        }
    }

    /// Return the capacity of the `ArrayString`.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let string = ArrayString::<3>::new();
    /// assert_eq!(string.capacity(), 3);
    /// ```
    #[inline(always)]
    pub const fn capacity(&self) -> usize { CAP }

    /// Return if the `ArrayString` is completely filled.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<1>::new();
    /// assert!(!string.is_full());
    /// string.push_str("A");
    /// assert!(string.is_full());
    /// ```
    pub const fn is_full(&self) -> bool { self.len() == self.capacity() }

    /// Returns the capacity left in the `ArrayString`.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<3>::from("abc").unwrap();
    /// string.pop();
    /// assert_eq!(string.remaining_capacity(), 1);
    /// ```
    pub const fn remaining_capacity(&self) -> usize {
        self.capacity() - self.len()
    }

    /// Adds the given char to the end of the string.
    ///
    /// ***Panics*** if the backing array is not large enough to fit the additional char.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<2>::new();
    ///
    /// string.push('a');
    /// string.push('b');
    ///
    /// assert_eq!(&string[..], "ab");
    /// ```
    #[track_caller]
    pub fn push(&mut self, c: char) {
        self.try_push(c).unwrap();
    }

    /// Adds the given char to the end of the string.
    ///
    /// Returns `Ok` if the push succeeds.
    ///
    /// **Errors** if the backing array is not large enough to fit the additional char.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<2>::new();
    ///
    /// string.try_push('a').unwrap();
    /// string.try_push('b').unwrap();
    /// let overflow = string.try_push('c');
    ///
    /// assert_eq!(&string[..], "ab");
    /// assert_eq!(overflow.unwrap_err().element(), 'c');
    /// ```
    pub fn try_push(&mut self, c: char) -> Result<(), CapacityError<char>> {
        let len = self.len();
        unsafe {
            let ptr = self.as_mut_ptr().add(len);
            let remaining_cap = self.capacity() - len;
            match encode_utf8(c, ptr, remaining_cap) {
                Ok(n) => {
                    self.set_len(len + n);
                    Ok(())
                }
                Err(_) => Err(CapacityError::new(c)),
            }
        }
    }

    /// Adds the given string slice to the end of the string.
    ///
    /// ***Panics*** if the backing array is not large enough to fit the string.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<2>::new();
    ///
    /// string.push_str("a");
    /// string.push_str("d");
    ///
    /// assert_eq!(&string[..], "ad");
    /// ```
    #[track_caller]
    pub fn push_str(&mut self, s: &str) {
        self.try_push_str(s).unwrap()
    }

    /// Adds the given string slice to the end of the string.
    ///
    /// Returns `Ok` if the push succeeds.
    ///
    /// **Errors** if the backing array is not large enough to fit the string.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<2>::new();
    ///
    /// string.try_push_str("a").unwrap();
    /// let overflow1 = string.try_push_str("bc");
    /// string.try_push_str("d").unwrap();
    /// let overflow2 = string.try_push_str("ef");
    ///
    /// assert_eq!(&string[..], "ad");
    /// assert_eq!(overflow1.unwrap_err().element(), "bc");
    /// assert_eq!(overflow2.unwrap_err().element(), "ef");
    /// ```
    pub fn try_push_str<'a>(&mut self, s: &'a str) -> Result<(), CapacityError<&'a str>> {
        if s.len() > self.capacity() - self.len() {
            return Err(CapacityError::new(s));
        }
        unsafe {
            let dst = self.as_mut_ptr().add(self.len());
            let src = s.as_ptr();
            ptr::copy_nonoverlapping(src, dst, s.len());
            let newl = self.len() + s.len();
            self.set_len(newl);
        }
        Ok(())
    }

    /// Removes the last character from the string and returns it.
    ///
    /// Returns `None` if this `ArrayString` is empty.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    /// 
    /// let mut s = ArrayString::<3>::from("foo").unwrap();
    ///
    /// assert_eq!(s.pop(), Some('o'));
    /// assert_eq!(s.pop(), Some('o'));
    /// assert_eq!(s.pop(), Some('f'));
    ///
    /// assert_eq!(s.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<char> {
        let ch = match self.chars().rev().next() {
            Some(ch) => ch,
            None => return None,
        };
        let new_len = self.len() - ch.len_utf8();
        unsafe {
            self.set_len(new_len);
        }
        Some(ch)
    }

    /// Shortens this `ArrayString` to the specified length.
    ///
    /// If `new_len` is greater than the string’s current length, this has no
    /// effect.
    ///
    /// ***Panics*** if `new_len` does not lie on a `char` boundary.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    ///
    /// let mut string = ArrayString::<6>::from("foobar").unwrap();
    /// string.truncate(3);
    /// assert_eq!(&string[..], "foo");
    /// string.truncate(4);
    /// assert_eq!(&string[..], "foo");
    /// ```
    pub fn truncate(&mut self, new_len: usize) {
        if new_len <= self.len() {
            assert!(self.is_char_boundary(new_len));
            unsafe { 
                // In libstd truncate is called on the underlying vector,
                // which in turns drops each element.
                // As we know we don't have to worry about Drop,
                // we can just set the length (a la clear.)
                self.set_len(new_len);
            }
        }
    }

    /// Removes a `char` from this `ArrayString` at a byte position and returns it.
    ///
    /// This is an `O(n)` operation, as it requires copying every element in the
    /// array.
    ///
    /// ***Panics*** if `idx` is larger than or equal to the `ArrayString`’s length,
    /// or if it does not lie on a `char` boundary.
    ///
    /// ```
    /// use arrayvec::ArrayString;
    /// 
    /// let mut s = ArrayString::<3>::from("foo").unwrap();
    ///
    /// assert_eq!(s.remove(0), 'f');
    /// assert_eq!(s.remove(1), 'o');
    /// assert_eq!(s.remove(0), 'o');
    /// ```
    pub fn remove(&mut self, idx: usize) -> char {
        let ch = match self[idx..].chars().next() {
            Some(ch) => ch,
            None => panic!("cannot remove a char from the end of a string"),
        };

        let next = idx + ch.len_utf8();
        let len = self.len();
        let ptr = self.as_mut_ptr();
        unsafe {
            ptr::copy(
                ptr.add(next),
                ptr.add(idx),
                len - next);
            self.set_len(len - (next - idx));
        }
        ch
    }

    /// Make the string empty.
    pub fn clear(&mut self) {
        unsafe {
            self.set_len(0);
        }
    }

    /// Set the strings’s length.
    ///
    /// This function is `unsafe` because it changes the notion of the
    /// number of “valid” bytes in the string. Use with care.
    ///
    /// This method uses *debug assertions* to check the validity of `length`
    /// and may use other debug assertions.
    pub unsafe fn set_len(&mut self, length: usize) {
        // type invariant that capacity always fits in LenUint
        debug_assert!(length <= self.capacity());
        self.len = length as LenUint;
    }

    /// Return a string slice of the whole `ArrayString`.
    pub fn as_str(&self) -> &str {
        self
    }

    /// Return a mutable string slice of the whole `ArrayString`.
    pub fn as_mut_str(&mut self) -> &mut str {
        self
    }

    /// Return a raw pointer to the string's buffer.
    pub fn as_ptr(&self) -> *const u8 {
        self.xs.as_ptr() as *const u8
    }

    /// Return a raw mutable pointer to the string's buffer.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.xs.as_mut_ptr() as *mut u8
    }
}

impl<const CAP: usize> Deref for ArrayString<CAP>
{
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        unsafe {
            let sl = slice::from_raw_parts(self.as_ptr(), self.len());
            str::from_utf8_unchecked(sl)
        }
    }
}

impl<const CAP: usize> DerefMut for ArrayString<CAP>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        unsafe {
            let len = self.len();
            let sl = slice::from_raw_parts_mut(self.as_mut_ptr(), len);
            str::from_utf8_unchecked_mut(sl)
        }
    }
}

impl<const CAP: usize> PartialEq for ArrayString<CAP>
{
    fn eq(&self, rhs: &Self) -> bool {
        **self == **rhs
    }
}

impl<const CAP: usize> PartialEq<str> for ArrayString<CAP>
{
    fn eq(&self, rhs: &str) -> bool {
        &**self == rhs
    }
}

impl<const CAP: usize> PartialEq<ArrayString<CAP>> for str
{
    fn eq(&self, rhs: &ArrayString<CAP>) -> bool {
        self == &**rhs
    }
}

impl<const CAP: usize> Eq for ArrayString<CAP> 
{ }

impl<const CAP: usize> Hash for ArrayString<CAP>
{
    fn hash<H: Hasher>(&self, h: &mut H) {
        (**self).hash(h)
    }
}

impl<const CAP: usize> Borrow<str> for ArrayString<CAP>
{
    fn borrow(&self) -> &str { self }
}

impl<const CAP: usize> BorrowMut<str> for ArrayString<CAP>
{
    fn borrow_mut(&mut self) -> &mut str { self }
}

impl<const CAP: usize> AsRef<str> for ArrayString<CAP>
{
    fn as_ref(&self) -> &str { self }
}

impl<const CAP: usize> fmt::Debug for ArrayString<CAP>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { (**self).fmt(f) }
}

#[cfg(feature="std")]
impl<const CAP: usize> AsRef<Path> for ArrayString<CAP> {
    fn as_ref(&self) -> &Path {
        self.as_str().as_ref()
    }
}

impl<const CAP: usize> fmt::Display for ArrayString<CAP>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { (**self).fmt(f) }
}

/// `Write` appends written data to the end of the string.
impl<const CAP: usize> fmt::Write for ArrayString<CAP>
{
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.try_push(c).map_err(|_| fmt::Error)
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.try_push_str(s).map_err(|_| fmt::Error)
    }
}

impl<const CAP: usize> Clone for ArrayString<CAP>
{
    fn clone(&self) -> ArrayString<CAP> {
        *self
    }
    fn clone_from(&mut self, rhs: &Self) {
        // guaranteed to fit due to types matching.
        self.clear();
        self.try_push_str(rhs).ok();
    }
}

impl<const CAP: usize> PartialOrd for ArrayString<CAP>
{
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        (**self).partial_cmp(&**rhs)
    }
    fn lt(&self, rhs: &Self) -> bool { **self < **rhs }
    fn le(&self, rhs: &Self) -> bool { **self <= **rhs }
    fn gt(&self, rhs: &Self) -> bool { **self > **rhs }
    fn ge(&self, rhs: &Self) -> bool { **self >= **rhs }
}

impl<const CAP: usize> PartialOrd<str> for ArrayString<CAP>
{
    fn partial_cmp(&self, rhs: &str) -> Option<cmp::Ordering> {
        (**self).partial_cmp(rhs)
    }
    fn lt(&self, rhs: &str) -> bool { &**self < rhs }
    fn le(&self, rhs: &str) -> bool { &**self <= rhs }
    fn gt(&self, rhs: &str) -> bool { &**self > rhs }
    fn ge(&self, rhs: &str) -> bool { &**self >= rhs }
}

impl<const CAP: usize> PartialOrd<ArrayString<CAP>> for str
{
    fn partial_cmp(&self, rhs: &ArrayString<CAP>) -> Option<cmp::Ordering> {
        self.partial_cmp(&**rhs)
    }
    fn lt(&self, rhs: &ArrayString<CAP>) -> bool { self < &**rhs }
    fn le(&self, rhs: &ArrayString<CAP>) -> bool { self <= &**rhs }
    fn gt(&self, rhs: &ArrayString<CAP>) -> bool { self > &**rhs }
    fn ge(&self, rhs: &ArrayString<CAP>) -> bool { self >= &**rhs }
}

impl<const CAP: usize> Ord for ArrayString<CAP>
{
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        (**self).cmp(&**rhs)
    }
}

impl<const CAP: usize> FromStr for ArrayString<CAP>
{
    type Err = CapacityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from(s).map_err(CapacityError::simplify)
    }
}

#[cfg(feature="serde")]
/// Requires crate feature `"serde"`
impl<const CAP: usize> Serialize for ArrayString<CAP>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&*self)
    }
}

#[cfg(feature="serde")]
/// Requires crate feature `"serde"`
impl<'de, const CAP: usize> Deserialize<'de> for ArrayString<CAP> 
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::{self, Visitor};
        use std::marker::PhantomData;

        struct ArrayStringVisitor<const CAP: usize>(PhantomData<[u8; CAP]>);

        impl<'de, const CAP: usize> Visitor<'de> for ArrayStringVisitor<CAP> {
            type Value = ArrayString<CAP>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string no more than {} bytes long", CAP)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: de::Error,
            {
                ArrayString::from(v).map_err(|_| E::invalid_length(v.len(), &self))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: de::Error,
            {
                let s = str::from_utf8(v).map_err(|_| E::invalid_value(de::Unexpected::Bytes(v), &self))?;

                ArrayString::from(s).map_err(|_| E::invalid_length(s.len(), &self))
            }
        }

        deserializer.deserialize_str(ArrayStringVisitor(PhantomData))
    }
}

#[cfg(feature = "borsh")]
/// Requires crate feature `"borsh"`
impl<const CAP: usize> borsh::BorshSerialize for ArrayString<CAP> {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        <str as borsh::BorshSerialize>::serialize(&*self, writer)
    }
}

#[cfg(feature = "borsh")]
/// Requires crate feature `"borsh"`
impl<const CAP: usize> borsh::BorshDeserialize for ArrayString<CAP> {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        let len = <u32 as borsh::BorshDeserialize>::deserialize_reader(reader)? as usize;
        if len > CAP {
            return Err(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                format!("Expected a string no more than {} bytes long", CAP),
            ))
        }

        let mut buf = [0u8; CAP];
        let buf = &mut buf[..len];
        reader.read_exact(buf)?;

        let s = str::from_utf8(&buf).map_err(|err| {
            borsh::io::Error::new(borsh::io::ErrorKind::InvalidData, err.to_string())
        })?;
        Ok(Self::from(s).unwrap())
    }
}

impl<'a, const CAP: usize> TryFrom<&'a str> for ArrayString<CAP>
{
    type Error = CapacityError<&'a str>;

    fn try_from(f: &'a str) -> Result<Self, Self::Error> {
        let mut v = Self::new();
        v.try_push_str(f)?;
        Ok(v)
    }
}

impl<'a, const CAP: usize> TryFrom<fmt::Arguments<'a>> for ArrayString<CAP>
{
    type Error = CapacityError<fmt::Error>;

    fn try_from(f: fmt::Arguments<'a>) -> Result<Self, Self::Error> {
        use fmt::Write;
        let mut v = Self::new();
        v.write_fmt(f).map_err(|e| CapacityError::new(e))?;
        Ok(v)
    }
}

#[cfg(feature = "zeroize")]
/// "Best efforts" zeroing of the `ArrayString`'s buffer when the `zeroize` feature is enabled.
///
/// The length is set to 0, and the buffer is dropped and zeroized.
/// Cannot ensure that previous moves of the `ArrayString` did not leave values on the stack.
///
/// ```
/// use arrayvec::ArrayString;
/// use zeroize::Zeroize;
/// let mut string = ArrayString::<6>::from("foobar").unwrap();
/// string.zeroize();
/// assert_eq!(string.len(), 0);
/// unsafe { string.set_len(string.capacity()) };
/// assert_eq!(&*string, "\0\0\0\0\0\0");
/// ```
impl<const CAP: usize> zeroize::Zeroize for ArrayString<CAP> {
    fn zeroize(&mut self) {
        // There are no elements to drop
        self.clear();
        // Zeroize the backing array.
        self.xs.zeroize();
    }
}
