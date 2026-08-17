//! [String]-like wrappers around [Bytes] and [BytesMut].
//!
//! The [Bytes] and [BytesMut] provide a buffer of bytes with ability to create owned slices into
//! the same shared memory allocation. This allows cheap manipulation of data.
//!
//! Strings are mostly just byte buffers with extra APIs to manipulate them. The standard [String]
//! type is built as a wrapper around [Vec]. We build similar wrappers around the [Bytes] and
//! [BytesMut], gaining the ability to create owned shared slices for textual data as well.
//!
//! Users are expected to use the [Str] and [StrMut] types. Note that these are type aliases around
//! the [StrInner] type. The latter is means to implement both in one go and contains all the
//! documentation, but is not meant to be used directly.
//!
//! # Splitting
//!
//! The [prim@str] type from standard library (which the types here dereference to) allows for
//! slicing and splitting in many convenient ways. They, however, return borrowed string slices
//! (`&str`), which might pose some problems.
//!
//! The [Str], and to certain extent, the [StrMut] type additionally allow cheap splitting and
//! slicing that produce owned [Str] and [StrMut] respectively. They are slightly more expensive
//! than the slicing than the ones returning `&str`, but only by incrementing internal reference
//! counts. They do not clone the actual string data, like `.to_owned()` on the standard library
//! methods would. These methods are available in addition to the standard ones.
//!
//! There are three ways how this can be done:
//!
//! * By dedicated methods, like [lines_bytes][StrInner::lines_bytes] (in general, the name of the
//!   standard method suffixed with `_bytes`).
//! * By using the [BytesIter] iterator manually.
//! * By using the standard-library methods, producing `&str` and translating it back to [Str] with
//!   [slice][StrInner::slice] or [StrInner::slice_ref].
//!
//! # Examples
//!
//! ```rust
//! # use bytes::Bytes;
//! # use bytes_utils::{Str, StrMut};
//! let mut builder = StrMut::new();
//! builder += "Hello";
//! builder.push(' ');
//! builder.push_str("World");
//! assert_eq!("Hello World", builder);
//!
//! let s1 = builder.split_built().freeze();
//! // This is a cheap copy, in the form of incrementing a reference count.
//! let s2 = s1.clone();
//! assert_eq!("Hello World", s1);
//! assert_eq!("Hello World", s2);
//! // Slicing is cheap as well, even though the returned things are Str and therefore owned too.
//! assert_eq!("ello", s1.slice(1..5));
//! // We have taken the data out of the builder, but the rest of its capacity can be used for
//! // further things.
//! assert_eq!("", builder);
//!
//! // Creating from strings and similar works
//! let a = Str::from("Hello");
//! assert_eq!("Hello", a);
//!
//! let e = Str::new();
//! assert_eq!("", e);
//!
//! // And from static str in O(1)
//! let b = Str::from_static("World");
//! assert_eq!("World", b);
//!
//! // And from Bytes too.
//! let b = Str::try_from(Bytes::from_static(b"World")).expect("Must be utf8");
//! assert_eq!("World", b);
//! // Invalid utf8 is refused.
//! Str::try_from(Bytes::from_static(&[0, 0, 255])).unwrap_err();
//! ```

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::convert::Infallible;
use core::fmt::{Debug, Display, Formatter, Result as FmtResult, Write};
use core::hash::{Hash, Hasher};
use core::iter::{self, FromIterator};
use core::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut};
use core::str::{self, FromStr};

#[cfg(feature = "std")]
use std::error::Error;

use bytes::{Bytes, BytesMut};
use either::Either;

#[cfg(feature = "serde")]
mod serde_impl;

/// Error when creating [Str] or [StrMut] from invalid UTF8 data.
#[derive(Copy, Clone, Debug)]
pub struct Utf8Error<S> {
    e: core::str::Utf8Error,
    inner: S,
}

impl<S> Utf8Error<S> {
    /// Returns the byte buffer back to the caller.
    pub fn into_inner(self) -> S {
        self.inner
    }

    /// The inner description of why the data is invalid UTF8.
    pub fn utf8_error(&self) -> str::Utf8Error {
        self.e
    }
}

impl<S> Display for Utf8Error<S> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Display::fmt(&self.e, fmt)
    }
}

#[cfg(feature = "std")]
impl<S: Debug> Error for Utf8Error<S> {}

/// Direction of iteration.
///
/// See [BytesIter].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Move forward (in the normal direction) in the string.
    Forward,

    /// Move backwards in the string.
    Backward,
}

/// Manual splitting iterator.
///
/// The methods on [Str] and [StrMut] that iterate use this internally. But it can also be used
/// manually to generate other iterators that split the original into parts.
#[derive(Clone, Debug)]
pub struct BytesIter<S, F> {
    bytes: Option<S>,
    extract: F,
    direction: Direction,
}

impl<S, F> BytesIter<S, F>
where
    S: Storage,
    F: FnMut(&str) -> Option<(usize, usize)>,
{
    /// A constructor of the iterator.
    ///
    /// The `direction` specifies in what order chunks should be yielded.
    ///
    /// The `ext` closure is always called with the rest of not yet split string. It shall return
    /// the byte indices of the chunk and separator border. In case of forward iteration, it is the
    /// end of them and the separator needs to end further to the string (or at the same position).
    /// In the backwards direction, it is in reverse ‒ they specify their starts and the separator
    /// is before the chunk.
    ///
    /// # Panics
    ///
    /// If the indices don't point at a character boundary, the iteration will panic. It'll also
    /// panic if the returned indices are reversed or if they are out of bounds.
    pub fn new(s: StrInner<S>, direction: Direction, ext: F) -> Self {
        Self {
            bytes: Some(s.0),
            extract: ext,
            direction,
        }
    }
}

impl<S, F> Iterator for BytesIter<S, F>
where
    S: Storage,
    F: FnMut(&str) -> Option<(usize, usize)>,
{
    type Item = StrInner<S>;

    fn next(&mut self) -> Option<StrInner<S>> {
        let storage = self.bytes.take()?;
        // Safety: we keep sure it is valid UTF8 on the API boundary.
        let whole_str = unsafe { str::from_utf8_unchecked(storage.as_ref()) };
        fn split<S: Storage>(storage: S, left: usize, right: usize) -> (S, S) {
            let whole_str = unsafe { str::from_utf8_unchecked(storage.as_ref()) };
            // Sanity-check we are not slicing in the middle of utf8 code point. This would
            // panic if we do. It would also panic if we are out of range, which is also good.
            assert!(whole_str.is_char_boundary(left));
            assert!(whole_str.is_char_boundary(right));

            // Now that we are sure this is legal, we are going to slice the byte data for real.
            let (with_sep, end) = storage.split_at(right);
            let (start, _sep) = with_sep.split_at(left);
            (start, end)
        }
        match ((self.extract)(whole_str), self.direction) {
            (Some((chunk_end, sep_end)), Direction::Forward) => {
                assert!(chunk_end <= sep_end);
                let (start, end) = split(storage, chunk_end, sep_end);

                self.bytes = Some(end);
                Some(StrInner(start))
            }
            (Some((chunk_start, sep_start)), Direction::Backward) => {
                assert!(sep_start <= chunk_start);
                let (start, end) = split(storage, sep_start, chunk_start);

                self.bytes = Some(start);
                Some(StrInner(end))
            }
            (None, _) => {
                // No separator found -> return the whole rest (and keep None in ourselves)
                Some(StrInner(storage))
            }
        }
    }
}

/// Find a separator position, for use with the [BytesIter].
fn sep_find<F: Fn(char) -> bool>(s: &str, is_sep: F) -> Option<(usize, usize)> {
    let sep_start = s.find(&is_sep)?;
    let sep_end = s[sep_start..]
        .find(|c| !is_sep(c))
        .map(|e| e + sep_start)
        .unwrap_or_else(|| s.len());
    Some((sep_start, sep_end))
}

/// Separator for an empty pattern.
fn empty_sep(s: &str, limit: usize) -> Option<(usize, usize)> {
    let char_end = s
        .char_indices()
        .skip(1)
        .map(|(i, _)| i)
        .chain(iter::once(s.len()).take((!s.is_empty()) as usize))
        .take(limit)
        .next()?;
    Some((char_end, char_end))
}

fn rempty_sep(s: &str, limit: usize) -> Option<(usize, usize)> {
    let char_start = s.char_indices().rev().map(|(i, _)| i).take(limit).next()?;
    Some((char_start, char_start))
}

/// The backing storage for [StrInner]
///
/// This is currently a technical detail of the crate, users are not expected to implement this
/// trait. Use [Str] or [StrMut] type aliases.
///
/// # Safety
///
/// The storage must act "sane". But what exactly it means is not yet analyzed and may change in
/// future versions. Don't implement the trait (at least not yet).
pub unsafe trait Storage: AsRef<[u8]> + Default + Sized {
    /// A type that can be used to build the storage incrementally.
    ///
    /// For mutable storages, it may be itself. For immutable one, there needs to be a mutable
    /// counterpart that can be converted to immutable later on.
    type Creator: Default + StorageMut;

    /// Converts the creator (mutable storage) to self.
    ///
    /// In case of mutable storages, this should be identity.
    fn from_creator(creator: Self::Creator) -> Self;

    /// Splits the storage at the given byte index and creates two non-overlapping instances.
    fn split_at(self, at: usize) -> (Self, Self);
}

unsafe impl Storage for Bytes {
    type Creator = BytesMut;
    fn from_creator(creator: Self::Creator) -> Self {
        creator.freeze()
    }
    fn split_at(mut self, at: usize) -> (Self, Self) {
        let right = self.split_off(at);
        (self, right)
    }
}

unsafe impl Storage for BytesMut {
    type Creator = BytesMut;
    fn from_creator(creator: Self::Creator) -> Self {
        creator
    }
    fn split_at(mut self, at: usize) -> (Self, Self) {
        let right = self.split_off(at);
        (self, right)
    }
}

/// Trait for extra functionality of a mutable storage.
///
/// This is in addition to what an immutable storage must satisfy.
///
/// # Safety
///
/// The storage must act "sane". But what exactly it means is not yet analyzed and may change in
/// future versions. Don't implement the trait (at least not yet).
pub unsafe trait StorageMut: Storage + AsMut<[u8]> {
    /// An immutable counter-part storage.
    type Immutable: Storage<Creator = Self>;

    /// Adds some more bytes to the end of the storage.
    fn push_slice(&mut self, s: &[u8]);
}

unsafe impl StorageMut for BytesMut {
    type Immutable = Bytes;
    fn push_slice(&mut self, s: &[u8]) {
        self.extend_from_slice(s)
    }
}

/// Implementation of the [Str] and [StrMut] types.
///
/// For technical reasons, both are implemented in one go as this type. For the same reason, most
/// of the documentation can be found here. Users are expected to use the [Str] and [StrMut]
/// instead.
#[derive(Clone, Default)]
pub struct StrInner<S>(S);

impl<S: Storage> StrInner<S> {
    /// Creates an empty instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Extracts the inner byte storage.
    pub fn into_inner(self) -> S {
        self.0
    }

    /// Access to the inner storage.
    pub fn inner(&self) -> &S {
        &self.0
    }

    /// Creates an instance from an existing byte storage.
    ///
    /// It may fail if the content is not valid UTF8.
    ///
    /// A [try_from][TryFrom::try_from] may be used instead.
    pub fn from_inner(s: S) -> Result<Self, Utf8Error<S>> {
        match str::from_utf8(s.as_ref()) {
            Ok(_) => Ok(Self(s)),
            Err(e) => Err(Utf8Error { e, inner: s }),
        }
    }

    /// Same as [from_inner][StrInner::from_inner], but without the checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure content is valid UTF8.
    pub const unsafe fn from_inner_unchecked(s: S) -> Self {
        Self(s)
    }

    /// Splits the string into two at the given index.
    ///
    /// # Panics
    ///
    /// If the index is not at char boundary.
    pub fn split_at_bytes(self, at: usize) -> (Self, Self) {
        assert!(self.deref().is_char_boundary(at));
        let (l, r) = self.0.split_at(at);
        (Self(l), Self(r))
    }

    /// Splits into whitespace separated "words".
    ///
    /// This acts like [split_whitespace][str::split_whitespace], but yields owned instances. It
    /// doesn't clone the content, it just increments some reference counts.
    pub fn split_whitespace_bytes(self) -> impl Iterator<Item = Self> {
        BytesIter::new(self, Direction::Forward, |s| {
            sep_find(s, char::is_whitespace)
        })
        .filter(|s| !s.is_empty())
    }

    /// Splits into whitespace separated "words".
    ///
    /// This acts like [split_ascii_whitespace][str::split_ascii_whitespace], but yields owned
    /// instances. This doesn't clone the content, it just increments some reference counts.
    pub fn split_ascii_whitespace_bytes(self) -> impl Iterator<Item = Self> {
        BytesIter::new(self, Direction::Forward, |s| {
            sep_find(s, |c| c.is_ascii() && (c as u8).is_ascii_whitespace())
        })
        .filter(|s| !s.is_empty())
    }

    /// Splits into lines.
    ///
    /// This acts like [lines][str::lines], but yields owned instances. The content is not cloned,
    /// this just increments some reference counts.
    pub fn lines_bytes(self) -> impl Iterator<Item = Self> {
        if self.is_empty() {
            Either::Left(iter::empty())
        } else {
            let iter = BytesIter::new(self, Direction::Forward, |s| sep_find(s, |c| c == '\n'))
                .map(|s| match s.chars().next() {
                    Some('\r') => s.split_at_bytes(1).1,
                    _ => s,
                });
            Either::Right(iter)
        }
    }

    /// Splits with the provided separator.
    ///
    /// This acts somewhat like [split][str::split], but yields owned instances. Also, it accepts
    /// only string patters (since the `Pattern` is not stable ☹). The content is not cloned, this
    /// just increments some reference counts.
    pub fn split_bytes<'s>(self, sep: &'s str) -> impl Iterator<Item = Self> + 's
    where
        S: 's,
    {
        if sep.is_empty() {
            let bulk = BytesIter::new(self, Direction::Forward, |s| empty_sep(s, usize::MAX));
            Either::Left(iter::once(Self::default()).chain(bulk))
        } else {
            let sep_find = move |s: &str| s.find(sep).map(|pos| (pos, pos + sep.len()));
            Either::Right(BytesIter::new(self, Direction::Forward, sep_find))
        }
    }

    /// Splits max. `n` times according to the given pattern.
    ///
    /// This acts somewhat like [splitn][str::splitn], but yields owned instances. Also, it accepts
    /// only string patters (since the `Pattern` is not stable ☹). The content is not cloned, this
    /// just increments some reference counts.
    pub fn splitn_bytes<'s>(self, mut n: usize, sep: &'s str) -> impl Iterator<Item = Self> + 's
    where
        S: 's,
    {
        // TODO: This seems to work, but is ugly. Any idea how to simplify?
        if sep.is_empty() {
            if n <= 1 {
                Either::Left(Either::Left(iter::once(self).take(n)))
            } else {
                n -= 1;
                let bulk = BytesIter::new(self, Direction::Forward, move |s| {
                    n -= 1;
                    empty_sep(s, n)
                });
                Either::Left(Either::Right(iter::once(Self::default()).chain(bulk)))
            }
        } else {
            let sep_find = move |s: &str| {
                n -= 1;
                if n == 0 {
                    None
                } else {
                    s.find(sep).map(|pos| (pos, pos + sep.len()))
                }
            };
            Either::Right(BytesIter::new(self, Direction::Forward, sep_find).take(n))
        }
    }

    /// A reverse version of [split_bytes][Self::split_bytes].
    pub fn rsplit_bytes<'s>(self, sep: &'s str) -> impl Iterator<Item = Self> + 's
    where
        S: 's,
    {
        if sep.is_empty() {
            let bulk = BytesIter::new(self, Direction::Backward, |s| rempty_sep(s, usize::MAX));
            Either::Left(iter::once(Self::default()).chain(bulk))
        } else {
            let sep_find = move |s: &str| s.rfind(sep).map(|pos| (pos + sep.len(), pos));
            Either::Right(BytesIter::new(self, Direction::Backward, sep_find))
        }
    }

    /// A reverse version of [splitn_bytes][Self::splitn_bytes].
    pub fn rsplitn_bytes<'s>(self, mut n: usize, sep: &'s str) -> impl Iterator<Item = Self> + 's
    where
        S: 's,
    {
        // TODO: This seems to work, but is ugly. Any idea how to simplify?
        if sep.is_empty() {
            if n <= 1 {
                Either::Left(Either::Left(iter::once(self).take(n)))
            } else {
                n -= 1;
                let bulk = BytesIter::new(self, Direction::Backward, move |s| {
                    n -= 1;
                    rempty_sep(s, n)
                });
                Either::Left(Either::Right(iter::once(Self::default()).chain(bulk)))
            }
        } else {
            let sep_find = move |s: &str| {
                n -= 1;
                if n == 0 {
                    None
                } else {
                    s.rfind(sep).map(|pos| (pos + sep.len(), pos))
                }
            };
            Either::Right(BytesIter::new(self, Direction::Backward, sep_find).take(n))
        }
    }
}

impl<S: StorageMut> StrInner<S> {
    /// Appends a string.
    pub fn push_str(&mut self, s: &str) {
        self.0.push_slice(s.as_bytes());
    }

    /// Appends one character.
    pub fn push(&mut self, c: char) {
        self.push_str(c.encode_utf8(&mut [0; 4]));
    }

    /// Provides mutable access to the inner buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the content stays valid UTF8.
    pub unsafe fn inner_mut(&mut self) -> &mut S {
        &mut self.0
    }

    /// Turns the mutable variant into an immutable one.
    ///
    /// The advantage is that it can then be shared (also by small parts).
    pub fn freeze(self) -> StrInner<S::Immutable> {
        StrInner(S::Immutable::from_creator(self.0))
    }
}

impl<S: Storage> Deref for StrInner<S> {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.0.as_ref()) }
    }
}

impl<S: StorageMut> DerefMut for StrInner<S> {
    fn deref_mut(&mut self) -> &mut str {
        unsafe { str::from_utf8_unchecked_mut(self.0.as_mut()) }
    }
}

impl<S, T> AsRef<T> for StrInner<S>
where
    S: Storage,
    str: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<S: StorageMut> AsMut<str> for StrInner<S> {
    fn as_mut(&mut self) -> &mut str {
        self.deref_mut()
    }
}

impl<S: Storage> Borrow<str> for StrInner<S> {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl<S: StorageMut> BorrowMut<str> for StrInner<S> {
    fn borrow_mut(&mut self) -> &mut str {
        self.deref_mut()
    }
}

impl<S: Storage> Debug for StrInner<S> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Debug::fmt(self.deref(), fmt)
    }
}

impl<S: Storage> Display for StrInner<S> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Display::fmt(self.deref(), fmt)
    }
}

impl<S: Storage> Hash for StrInner<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

impl<S, I> Index<I> for StrInner<S>
where
    S: Storage,
    str: Index<I>,
{
    type Output = <str as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.deref().index(index)
    }
}

impl<S, I> IndexMut<I> for StrInner<S>
where
    S: StorageMut,
    str: IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.deref_mut().index_mut(index)
    }
}

impl<S: StorageMut> Add<&str> for StrInner<S> {
    type Output = Self;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.push_str(rhs);
        self
    }
}

impl<S: StorageMut> AddAssign<&str> for StrInner<S> {
    fn add_assign(&mut self, rhs: &str) {
        self.push_str(rhs);
    }
}

impl<S: StorageMut> Extend<char> for StrInner<S> {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        for c in iter {
            self.push(c);
        }
    }
}

impl<'a, S: StorageMut> Extend<&'a char> for StrInner<S> {
    fn extend<T: IntoIterator<Item = &'a char>>(&mut self, iter: T) {
        for c in iter {
            self.push(*c);
        }
    }
}

macro_rules! impl_extend {
    ($ty:ty $(, $lifetimes:lifetime )* ) => {
        impl<$($lifetimes, )* S: StorageMut> Extend<$ty> for StrInner<S> {
            fn extend<T: IntoIterator<Item = $ty>>(&mut self, iter: T) {
                for i in iter {
                    self.push_str(i.as_ref());
                }
            }
        }

        impl<$($lifetimes, )* S> FromIterator<$ty> for StrInner<S>
        where
            S: Storage,
        {
            fn from_iter<T: IntoIterator<Item = $ty>>(iter: T) -> Self {
                let mut creator = StrInner(S::Creator::default());
                creator.extend(iter);
                StrInner(S::from_creator(creator.0))
            }
        }
    };
}

impl_extend!(String);
impl_extend!(Box<str>);
impl_extend!(&'a String, 'a);
impl_extend!(&'a str, 'a);
impl_extend!(Cow<'a, str>, 'a);

macro_rules! impl_from {
    ($ty:ty $(, $lifetimes:lifetime )* ) => {
        impl<$($lifetimes, )* S> From<$ty> for StrInner<S>
        where
            S: Storage,
        {
            fn from(s: $ty) -> Self {
                iter::once(s).collect()
            }
        }
    };
}

impl_from!(&'a String, 'a);
impl_from!(&'a str, 'a);
impl_from!(Cow<'a, str>, 'a);

impl From<String> for Str {
    fn from(s: String) -> Self {
        let inner = Bytes::from(s.into_bytes());
        // Safety: inner is constructed from a str
        unsafe { Str::from_inner_unchecked(inner) }
    }
}

impl From<Box<str>> for Str {
    fn from(s: Box<str>) -> Self {
        let s: Box<[u8]> = s.into();
        let inner = Bytes::from(s);
        // Safety: inner is constructed from a str
        unsafe { Str::from_inner_unchecked(inner) }
    }
}

macro_rules! impl_try_from {
    ($ty: ty) => {
        impl TryFrom<$ty> for StrInner<$ty> {
            type Error = Utf8Error<$ty>;
            fn try_from(s: $ty) -> Result<Self, Utf8Error<$ty>> {
                Self::from_inner(s)
            }
        }

        impl From<StrInner<$ty>> for $ty {
            fn from(s: StrInner<$ty>) -> $ty {
                s.0
            }
        }
    };
}

impl_try_from!(Bytes);
impl_try_from!(BytesMut);

impl From<StrMut> for Str {
    fn from(s: StrMut) -> Self {
        s.freeze()
    }
}

impl<S: Storage> FromStr for StrInner<S> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl<S: Storage> PartialEq for StrInner<S> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<S: Storage> Eq for StrInner<S> {}

impl<S: Storage> PartialOrd for StrInner<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<S: Storage> Ord for StrInner<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

macro_rules! impl_partrial_eq {
    ($ty: ty $(, $lifetimes:lifetime )* ) => {
        impl<$($lifetimes, )* S: Storage> PartialEq<$ty> for StrInner<S> {
            fn eq(&self, other: &$ty) -> bool {
                self.deref() == other.deref()
            }
        }

        impl<$($lifetimes, )* S: Storage> PartialEq<StrInner<S>> for $ty {
            fn eq(&self, other: &StrInner<S>) -> bool {
                self.deref() == other.deref()
            }
        }

        impl<$($lifetimes, )* S: Storage> PartialOrd<$ty> for StrInner<S> {
            fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                Some(self.deref().cmp(other.deref()))
            }
        }

        impl<$($lifetimes, )* S: Storage> PartialOrd<StrInner<S>> for $ty {
            fn partial_cmp(&self, other: &StrInner<S>) -> Option<Ordering> {
                Some(self.deref().cmp(other.deref()))
            }
        }
    };
}

impl_partrial_eq!(String);
impl_partrial_eq!(Box<str>);
impl_partrial_eq!(&'a str, 'a);
impl_partrial_eq!(&'a mut str, 'a);
impl_partrial_eq!(Cow<'a, str>, 'a);

impl<S: StorageMut> Write for StrInner<S> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        self.push_str(s);
        Ok(())
    }
}
/// The [format] macro, but returning [Str].
///
/// # Examples
///
/// ```
/// use bytes_utils::{format_bytes, Str};
/// let s: Str = format_bytes!("Hello {}", "world");
/// assert_eq!("Hello world", s);
/// ```
#[macro_export]
macro_rules! format_bytes {
    ($($arg: tt)*) => {
        $crate::format_bytes_mut!($($arg)*).freeze()
    }
}

/// The [format] macro, but returning [StrMut].
///
/// # Examples
///
/// ```
/// use bytes_utils::{format_bytes_mut, StrMut};
/// let s: StrMut = format_bytes_mut!("Hello {}", "world");
/// assert_eq!("Hello world", s);
/// ```
#[macro_export]
macro_rules! format_bytes_mut {
    ($($arg: tt)*) => {{
        use std::fmt::Write;
        let mut buf = $crate::StrMut::default();
        write!(buf, $($arg)*).unwrap();
        buf
    }}
}

/// An immutable variant of [Bytes]-backed string.
///
/// The methods and their documentation are on [StrInner], but users are mostly expected to use
/// this and the [StrMut] aliases.
pub type Str = StrInner<Bytes>;

impl Str {
    /// Extracts a subslice of the string as an owned [Str].
    ///
    /// # Panics
    ///
    /// If the byte indices in the range are not on char boundaries.
    pub fn slice<R>(&self, range: R) -> Str
    where
        str: Index<R, Output = str>,
    {
        self.slice_ref(&self[range])
    }

    /// Extracts owned representation of the slice passed.
    ///
    /// This method accepts a string sub-slice of `self`. It then extracts the slice but as the
    /// [Str] type. This makes it easier to use "ordinary" string parsing/manipulation and then go
    /// back to holding the [Bytes]-based representation.
    ///
    /// This is zero-copy, the common part will be shared by reference counting.
    ///
    /// # Panics
    ///
    /// If the provided slice is not a sub-slice of `self`. This is checked based on address of the
    /// slice, not on the content.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use bytes_utils::Str;
    /// let owned = Str::from("Hello World");
    /// let borrowed_mid: &str = &owned[2..5];
    ///
    /// let mid: Str = owned.slice_ref(borrowed_mid);
    /// assert_eq!("Hello World", owned);
    /// assert_eq!("llo", mid);
    /// ```
    pub fn slice_ref(&self, subslice: &str) -> Self {
        let sub = self.0.slice_ref(subslice.as_bytes());
        Self(sub)
    }

    /// Create [`Str`] from static string in O(1).
    pub const fn from_static(s: &'static str) -> Self {
        let bytes = Bytes::from_static(s.as_bytes());
        // Safety: bytes is constructed from str
        unsafe { Str::from_inner_unchecked(bytes) }
    }
}

/// A mutable variant of [BytesMut]-backed string.
///
/// Unlike [Str], this one allows modifications (mostly additions), but also doesn't allow
/// overlapping/shared chunks.
///
/// This is internally backed by the [StrInner] type, so the documentation of the methods are on
/// that.
pub type StrMut = StrInner<BytesMut>;

impl StrMut {
    /// Splits and returns the part of already built string, but keeps the extra capacity.
    pub fn split_built(&mut self) -> StrMut {
        StrInner(self.0.split())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use std::panic;

    use super::*;

    #[test]
    fn split_w_byte_index() {
        let v = Str::from("😈 ").split_whitespace_bytes().collect_vec();
        assert_eq!(1, v.len());
        assert_eq!("😈", v[0]);
    }

    #[test]
    fn split_same() {
        let v = Str::from("a").split_bytes("a").collect_vec();
        assert_eq!(2, v.len());
        assert_eq!("", v[0]);
        assert_eq!("", v[1]);
    }

    #[test]
    fn split_empty_pat() {
        let v = Str::from("a").split_bytes("").collect_vec();
        assert_eq!(3, v.len());
        assert_eq!("", v[0]);
        assert_eq!("a", v[1]);
        assert_eq!("", v[2]);
    }

    #[test]
    fn slice_checks_char_boundaries() {
        let v = Str::from("😈");
        assert_eq!(4, v.len());
        panic::catch_unwind(|| v.slice(1..)).unwrap_err();
    }

    #[test]
    fn split_at_bytes_mid() {
        let v = Str::from("hello");
        let (l, r) = v.split_at_bytes(2);
        assert_eq!("he", l);
        assert_eq!("llo", r);
    }

    #[test]
    fn split_at_bytes_begin() {
        let v = Str::from("hello");
        let (l, r) = v.split_at_bytes(0);
        assert_eq!("", l);
        assert_eq!("hello", r);
    }

    #[test]
    fn split_at_bytes_end() {
        let v = Str::from("hello");
        let (l, r) = v.split_at_bytes(5);
        assert_eq!("hello", l);
        assert_eq!("", r);
    }

    #[test]
    fn split_at_bytes_panic() {
        let v = Str::from("😈");
        assert_eq!(4, v.len());
        panic::catch_unwind(|| v.split_at_bytes(2)).unwrap_err();
    }

    #[cfg(not(miri))]
    mod proptests {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            #[test]
            fn split_whitespace(s: String) {
                let bstring = Str::from(&s);

                let bw = bstring.split_whitespace_bytes();
                let sw = s.split_whitespace();

                for (b, s) in bw.zip_eq(sw) {
                    prop_assert_eq!(b, s);
                }
            }

            #[test]
            fn split_ascii_whitespace(s: String) {
                let bstring = Str::from(&s);

                let bw = bstring.split_ascii_whitespace_bytes();
                let sw = s.split_ascii_whitespace();

                for (b, s) in bw.zip_eq(sw) {
                    prop_assert_eq!(b, s);
                }
            }

            #[test]
            fn lines(s: String) {
                let bstring = Str::from(&s);

                let bl = bstring.lines_bytes();
                let sl = s.lines();

                for (b, s) in bl.zip_eq(sl) {
                    prop_assert_eq!(b, s);
                }
            }

            #[test]
            fn split(s: String, pat: String) {
                let bstring = Str::from(&s);

                let bs = bstring.split_bytes(&pat);
                let ss = s.split(&pat);

                for (b, s) in bs.zip_eq(ss) {
                    prop_assert_eq!(b, s);
                }
            }

            #[test]
            fn split_n(s: String, pat: String, n in 0..5usize) {
                let bstring = Str::from(&s);

                let bs = bstring.splitn_bytes(n, &pat);
                let ss = s.splitn(n, &pat);

                for (b, s) in bs.zip_eq(ss) {
                    prop_assert_eq!(b, s);
                }
            }

            #[test]
            fn rsplit(s: String, pat: String) {
                let bstring = Str::from(&s);

                let bs = bstring.rsplit_bytes(&pat);
                let ss = s.rsplit(&pat);

                for (b, s) in bs.zip_eq(ss) {
                    prop_assert_eq!(b, s);
                }
            }

            #[test]
            fn rsplit_n(s: String, pat: String, n in 0..5usize) {
                let bstring = Str::from(&s);

                let bs = bstring.rsplitn_bytes(n, &pat);
                let ss = s.rsplitn(n, &pat);

                for (b, s) in bs.zip_eq(ss) {
                    prop_assert_eq!(b, s);
                }
            }
        }
    }
}
