//! ASN.1 `SET OF` support.
//!
//! # Ordering Notes
//!
//! Some DER serializer implementations fail to properly sort elements of a
//! `SET OF`. This is technically non-canonical, but occurs frequently
//! enough that most DER decoders tolerate it. Unfortunately because
//! of that, we must also follow suit.
//!
//! However, all types in this module sort elements of a set at decode-time,
//! ensuring they'll be in the proper order if reserialized.

use crate::{
    arrayvec, ord::iter_cmp, ArrayVec, Decode, DecodeValue, DerOrd, Encode, EncodeValue, Error,
    ErrorKind, FixedTag, Header, Length, Reader, Result, Tag, ValueOrd, Writer,
};
use core::cmp::Ordering;

#[cfg(feature = "alloc")]
use {alloc::vec::Vec, core::slice};

/// ASN.1 `SET OF` backed by an array.
///
/// This type implements an append-only `SET OF` type which is stack-based
/// and does not depend on `alloc` support.
// TODO(tarcieri): use `ArrayVec` when/if it's merged into `core`
// See: https://github.com/rust-lang/rfcs/pull/2990
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SetOf<T, const N: usize>
where
    T: DerOrd,
{
    inner: ArrayVec<T, N>,
}

impl<T, const N: usize> SetOf<T, N>
where
    T: DerOrd,
{
    /// Create a new [`SetOf`].
    pub fn new() -> Self {
        Self {
            inner: ArrayVec::default(),
        }
    }

    /// Add an element to this [`SetOf`].
    ///
    /// Items MUST be added in lexicographical order according to the
    /// [`DerOrd`] impl on `T`.
    pub fn add(&mut self, new_elem: T) -> Result<()> {
        // Ensure set elements are lexicographically ordered
        if let Some(last_elem) = self.inner.last() {
            if new_elem.der_cmp(last_elem)? != Ordering::Greater {
                return Err(ErrorKind::SetOrdering.into());
            }
        }

        self.inner.add(new_elem)
    }

    /// Get the nth element from this [`SetOf`].
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    /// Iterate over the elements of this [`SetOf`].
    pub fn iter(&self) -> SetOfIter<'_, T> {
        SetOfIter {
            inner: self.inner.iter(),
        }
    }

    /// Is this [`SetOf`] empty?
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Number of elements in this [`SetOf`].
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, const N: usize> Default for SetOf<T, N>
where
    T: DerOrd,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, const N: usize> DecodeValue<'a> for SetOf<T, N>
where
    T: Decode<'a> + DerOrd,
{
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        reader.read_nested(header.length, |reader| {
            let mut result = Self::new();

            while !reader.is_finished() {
                result.inner.add(T::decode(reader)?)?;
            }

            der_sort(result.inner.as_mut())?;
            validate(result.inner.as_ref())?;
            Ok(result)
        })
    }
}

impl<'a, T, const N: usize> EncodeValue for SetOf<T, N>
where
    T: 'a + Decode<'a> + Encode + DerOrd,
{
    fn value_len(&self) -> Result<Length> {
        self.iter()
            .fold(Ok(Length::ZERO), |len, elem| len + elem.encoded_len()?)
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        for elem in self.iter() {
            elem.encode(writer)?;
        }

        Ok(())
    }
}

impl<'a, T, const N: usize> FixedTag for SetOf<T, N>
where
    T: Decode<'a> + DerOrd,
{
    const TAG: Tag = Tag::Set;
}

impl<T, const N: usize> TryFrom<[T; N]> for SetOf<T, N>
where
    T: DerOrd,
{
    type Error = Error;

    fn try_from(mut arr: [T; N]) -> Result<SetOf<T, N>> {
        der_sort(&mut arr)?;

        let mut result = SetOf::new();

        for elem in arr {
            result.add(elem)?;
        }

        Ok(result)
    }
}

impl<T, const N: usize> ValueOrd for SetOf<T, N>
where
    T: DerOrd,
{
    fn value_cmp(&self, other: &Self) -> Result<Ordering> {
        iter_cmp(self.iter(), other.iter())
    }
}

/// Iterator over the elements of an [`SetOf`].
#[derive(Clone, Debug)]
pub struct SetOfIter<'a, T> {
    /// Inner iterator.
    inner: arrayvec::Iter<'a, T>,
}

impl<'a, T> Iterator for SetOfIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }
}

impl<'a, T> ExactSizeIterator for SetOfIter<'a, T> {}

/// ASN.1 `SET OF` backed by a [`Vec`].
///
/// This type implements an append-only `SET OF` type which is heap-backed
/// and depends on `alloc` support.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SetOfVec<T>
where
    T: DerOrd,
{
    inner: Vec<T>,
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: DerOrd> Default for SetOfVec<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T> SetOfVec<T>
where
    T: DerOrd,
{
    /// Create a new [`SetOfVec`].
    pub fn new() -> Self {
        Self {
            inner: Vec::default(),
        }
    }

    /// Add an element to this [`SetOfVec`].
    ///
    /// Items MUST be added in lexicographical order according to the
    /// [`DerOrd`] impl on `T`.
    pub fn add(&mut self, new_elem: T) -> Result<()> {
        // Ensure set elements are lexicographically ordered
        if let Some(last_elem) = self.inner.last() {
            if new_elem.der_cmp(last_elem)? != Ordering::Greater {
                return Err(ErrorKind::SetOrdering.into());
            }
        }

        self.inner.push(new_elem);
        Ok(())
    }

    /// Borrow the elements of this [`SetOfVec`] as a slice.
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    /// Get the nth element from this [`SetOfVec`].
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    /// Convert this [`SetOfVec`] into the inner [`Vec`].
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    /// Iterate over the elements of this [`SetOfVec`].
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.inner.iter()
    }

    /// Is this [`SetOfVec`] empty?
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Number of elements in this [`SetOfVec`].
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T> AsRef<[T]> for SetOfVec<T>
where
    T: DerOrd,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T> DecodeValue<'a> for SetOfVec<T>
where
    T: Decode<'a> + DerOrd,
{
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        reader.read_nested(header.length, |reader| {
            let mut inner = Vec::new();

            while !reader.is_finished() {
                inner.push(T::decode(reader)?);
            }

            der_sort(inner.as_mut())?;
            validate(inner.as_ref())?;
            Ok(Self { inner })
        })
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T> EncodeValue for SetOfVec<T>
where
    T: 'a + Decode<'a> + Encode + DerOrd,
{
    fn value_len(&self) -> Result<Length> {
        self.iter()
            .fold(Ok(Length::ZERO), |len, elem| len + elem.encoded_len()?)
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        for elem in self.iter() {
            elem.encode(writer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T> FixedTag for SetOfVec<T>
where
    T: DerOrd,
{
    const TAG: Tag = Tag::Set;
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T> From<SetOfVec<T>> for Vec<T>
where
    T: DerOrd,
{
    fn from(set: SetOfVec<T>) -> Vec<T> {
        set.into_vec()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T> TryFrom<Vec<T>> for SetOfVec<T>
where
    T: DerOrd,
{
    type Error = Error;

    fn try_from(mut vec: Vec<T>) -> Result<SetOfVec<T>> {
        // TODO(tarcieri): use `[T]::sort_by` here?
        der_sort(vec.as_mut_slice())?;
        Ok(SetOfVec { inner: vec })
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T, const N: usize> TryFrom<[T; N]> for SetOfVec<T>
where
    T: DerOrd,
{
    type Error = Error;

    fn try_from(arr: [T; N]) -> Result<SetOfVec<T>> {
        Vec::from(arr).try_into()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T> ValueOrd for SetOfVec<T>
where
    T: DerOrd,
{
    fn value_cmp(&self, other: &Self) -> Result<Ordering> {
        iter_cmp(self.iter(), other.iter())
    }
}

/// Sort a mut slice according to its [`DerOrd`], returning any errors which
/// might occur during the comparison.
///
/// The algorithm is insertion sort, which should perform well when the input
/// is mostly sorted to begin with.
///
/// This function is used rather than Rust's built-in `[T]::sort_by` in order
/// to support heapless `no_std` targets as well as to enable bubbling up
/// sorting errors.
#[allow(clippy::integer_arithmetic)]
fn der_sort<T: DerOrd>(slice: &mut [T]) -> Result<()> {
    for i in 0..slice.len() {
        let mut j = i;

        while j > 0 && slice[j - 1].der_cmp(&slice[j])? == Ordering::Greater {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }

    Ok(())
}

/// Validate the elements of a `SET OF`, ensuring that they are all in order
/// and that there are no duplicates.
fn validate<T: DerOrd>(slice: &[T]) -> Result<()> {
    if let Some(len) = slice.len().checked_sub(1) {
        for i in 0..len {
            let j = i.checked_add(1).ok_or(ErrorKind::Overflow)?;

            match slice.get(i..=j) {
                Some([a, b]) => {
                    if a.der_cmp(b)? != Ordering::Less {
                        return Err(ErrorKind::SetOrdering.into());
                    }
                }
                _ => return Err(Tag::Set.value_error()),
            }
        }
    }

    Ok(())
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::{SetOf, SetOfVec};
    use alloc::vec::Vec;

    #[test]
    fn setof_tryfrom_array() {
        let arr = [3u16, 2, 1, 65535, 0];
        let set = SetOf::try_from(arr).unwrap();
        assert_eq!(
            set.iter().cloned().collect::<Vec<u16>>(),
            &[0, 1, 2, 3, 65535]
        );
    }

    #[test]
    fn setofvec_tryfrom_array() {
        let arr = [3u16, 2, 1, 65535, 0];
        let set = SetOfVec::try_from(arr).unwrap();
        assert_eq!(set.as_ref(), &[0, 1, 2, 3, 65535]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn setofvec_tryfrom_vec() {
        let vec = vec![3u16, 2, 1, 65535, 0];
        let set = SetOfVec::try_from(vec).unwrap();
        assert_eq!(set.as_ref(), &[0, 1, 2, 3, 65535]);
    }
}
