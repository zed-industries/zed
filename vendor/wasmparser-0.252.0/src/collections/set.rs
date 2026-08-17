//! Type definitions for a default set.

use core::{
    borrow::Borrow,
    fmt::{self, Debug},
    hash::Hash,
    iter::FusedIterator,
    ops::{BitAnd, BitOr, BitXor, Sub},
};

#[cfg(all(
    feature = "hash-collections",
    not(feature = "prefer-btree-collections")
))]
mod detail {
    use crate::collections::hash;
    use hashbrown::hash_set;

    pub type SetImpl<T> = hash_set::HashSet<T, hash::RandomState>;
    pub type IterImpl<'a, T> = hash_set::Iter<'a, T>;
    pub type IntoIterImpl<T> = hash_set::IntoIter<T>;
    pub type DifferenceImpl<'a, T> = hash_set::Difference<'a, T, hash::RandomState>;
    pub type IntersectionImpl<'a, T> = hash_set::Intersection<'a, T, hash::RandomState>;
    pub type SymmetricDifferenceImpl<'a, T> =
        hash_set::SymmetricDifference<'a, T, hash::RandomState>;
    pub type UnionImpl<'a, T> = hash_set::Union<'a, T, hash::RandomState>;
}

#[cfg(any(
    not(feature = "hash-collections"),
    feature = "prefer-btree-collections"
))]
mod detail {
    use alloc::collections::btree_set;

    pub type SetImpl<T> = btree_set::BTreeSet<T>;
    pub type IterImpl<'a, T> = btree_set::Iter<'a, T>;
    pub type IntoIterImpl<T> = btree_set::IntoIter<T>;
    pub type DifferenceImpl<'a, T> = btree_set::Difference<'a, T>;
    pub type IntersectionImpl<'a, T> = btree_set::Intersection<'a, T>;
    pub type SymmetricDifferenceImpl<'a, T> = btree_set::SymmetricDifference<'a, T>;
    pub type UnionImpl<'a, T> = btree_set::Union<'a, T>;
}

/// A default set of values.
///
/// Provides an API compatible with both [`HashSet`] and [`BTreeSet`].
///
/// [`HashSet`]: hashbrown::HashSet
/// [`BTreeSet`]: alloc::collections::BTreeSet
#[derive(Debug, Clone)]
pub struct Set<T> {
    /// The underlying hash-set or btree-set data structure used.
    inner: detail::SetImpl<T>,
}

impl<T> Default for Set<T> {
    #[inline]
    fn default() -> Self {
        Self {
            inner: detail::SetImpl::default(),
        }
    }
}

impl<T> Set<T> {
    /// Clears the [`Set`], removing all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// The elements are visited in unsorted (and unspecified) order.
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        T: Ord,
        F: FnMut(&T) -> bool,
    {
        self.inner.retain(f)
    }

    /// Returns the number of elements in the [`Set`].
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the [`Set`] contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator that yields the items in the [`Set`].
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.inner.iter(),
        }
    }
}

impl<T> Set<T>
where
    T: Eq + Hash + Ord,
{
    /// Reserves capacity for at least `additional` more elements to be inserted in the [`Set`].
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        #[cfg(all(
            feature = "hash-collections",
            not(feature = "prefer-btree-collections")
        ))]
        self.inner.reserve(additional);
        #[cfg(any(
            not(feature = "hash-collections"),
            feature = "prefer-btree-collections"
        ))]
        let _ = additional;
    }

    /// Returns true if the [`Set`] contains an element equal to the `value`.
    #[inline]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.contains(value)
    }

    /// Returns a reference to the element in the [`Set`], if any, that is equal to the `value`.
    #[inline]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.get(value)
    }

    /// Adds `value` to the [`Set`].
    ///
    /// Returns whether the value was newly inserted:
    ///
    /// - Returns `true` if the set did not previously contain an equal value.
    /// - Returns `false` otherwise and the entry is not updated.
    #[inline]
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }

    /// If the set contains an element equal to the value, removes it from the [`Set`] and drops it.
    ///
    /// Returns `true` if such an element was present, otherwise `false`.
    #[inline]
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.remove(value)
    }

    /// Removes and returns the element in the [`Set`], if any, that is equal to
    /// the value.
    ///
    /// The value may be any borrowed form of the set's element type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the element type.
    #[inline]
    pub fn take<Q>(&mut self, value: &Q) -> Option<T>
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Ord,
    {
        self.inner.take(value)
    }

    /// Adds a value to the [`Set`], replacing the existing value, if any, that is equal to the given
    /// one. Returns the replaced value.
    #[inline]
    pub fn replace(&mut self, value: T) -> Option<T> {
        self.inner.replace(value)
    }

    /// Returns `true` if `self` has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.inner.is_disjoint(&other.inner)
    }

    /// Returns `true` if the [`Set`] is a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    #[inline]
    pub fn is_subset(&self, other: &Self) -> bool {
        self.inner.is_subset(&other.inner)
    }

    /// Returns `true` if the [`Set`] is a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    #[inline]
    pub fn is_superset(&self, other: &Self) -> bool {
        self.inner.is_superset(&other.inner)
    }

    /// Visits the values representing the difference,
    /// i.e., the values that are in `self` but not in `other`.
    #[inline]
    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, T> {
        Difference {
            inner: self.inner.difference(&other.inner),
        }
    }

    /// Visits the values representing the symmetric difference,
    /// i.e., the values that are in `self` or in `other` but not in both.
    #[inline]
    pub fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, T> {
        SymmetricDifference {
            inner: self.inner.symmetric_difference(&other.inner),
        }
    }

    /// Visits the values representing the intersection,
    /// i.e., the values that are both in `self` and `other`.
    ///
    /// When an equal element is present in `self` and `other`
    /// then the resulting `Intersection` may yield references to
    /// one or the other. This can be relevant if `T` contains fields which
    /// are not compared by its `Eq` implementation, and may hold different
    /// value between the two equal copies of `T` in the two sets.
    #[inline]
    pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T> {
        Intersection {
            inner: self.inner.intersection(&other.inner),
        }
    }

    /// Visits the values representing the union,
    /// i.e., all the values in `self` or `other`, without duplicates.
    #[inline]
    pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T> {
        Union {
            inner: self.inner.union(&other.inner),
        }
    }
}

impl<T> PartialEq for Set<T>
where
    T: Eq + Hash,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for Set<T> where T: Eq + Hash {}

impl<T> FromIterator<T> for Set<T>
where
    T: Hash + Eq + Ord,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            inner: <detail::SetImpl<T>>::from_iter(iter),
        }
    }
}

impl<'a, T> IntoIterator for &'a Set<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Extend<&'a T> for Set<T>
where
    T: Hash + Eq + Ord + Copy + 'a,
{
    #[inline]
    fn extend<Iter: IntoIterator<Item = &'a T>>(&mut self, iter: Iter) {
        self.inner.extend(iter)
    }
}

impl<T> Extend<T> for Set<T>
where
    T: Hash + Eq + Ord,
{
    #[inline]
    fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
        self.inner.extend(iter)
    }
}

impl<'a, T> BitAnd<Self> for &'a Set<T>
where
    T: Eq + Hash + Ord + Clone + 'a,
{
    type Output = Set<T>;

    #[inline]
    fn bitand(self, rhs: Self) -> Set<T> {
        Set {
            inner: BitAnd::bitand(&self.inner, &rhs.inner),
        }
    }
}

impl<'a, T> BitOr<Self> for &'a Set<T>
where
    T: Eq + Hash + Ord + Clone + 'a,
{
    type Output = Set<T>;

    #[inline]
    fn bitor(self, rhs: Self) -> Set<T> {
        Set {
            inner: BitOr::bitor(&self.inner, &rhs.inner),
        }
    }
}

impl<'a, T> BitXor<Self> for &'a Set<T>
where
    T: Eq + Hash + Ord + Clone + 'a,
{
    type Output = Set<T>;

    #[inline]
    fn bitxor(self, rhs: Self) -> Set<T> {
        Set {
            inner: BitXor::bitxor(&self.inner, &rhs.inner),
        }
    }
}

impl<'a, T> Sub<Self> for &'a Set<T>
where
    T: Eq + Hash + Ord + Clone + 'a,
{
    type Output = Set<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Set<T> {
        Set {
            inner: Sub::sub(&self.inner, &rhs.inner),
        }
    }
}

/// An iterator over the items of a [`Set`].
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    inner: detail::IterImpl<'a, T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T: 'a> ExactSizeIterator for Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T: 'a> FusedIterator for Iter<'a, T> where detail::IterImpl<'a, T>: FusedIterator {}

impl<T> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}

/// An iterator over the owned items of an [`Set`].
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: detail::IntoIterImpl<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> FusedIterator for IntoIter<T> where detail::IntoIterImpl<T>: FusedIterator {}

/// A lazy iterator producing elements in the difference of [`Set`]s.
///
/// This `struct` is created by the [`difference`] method on [`Set`].
/// See its documentation for more.
///
/// [`difference`]: Set::difference
pub struct Difference<'a, T: 'a> {
    inner: detail::DifferenceImpl<'a, T>,
}

impl<T> Debug for Difference<'_, T>
where
    T: Debug + Hash + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> Clone for Difference<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T> Iterator for Difference<'a, T>
where
    T: Hash + Eq + Ord,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for Difference<'a, T>
where
    T: Hash + Eq + Ord,
    detail::DifferenceImpl<'a, T>: FusedIterator,
{
}

/// A lazy iterator producing elements in the intersection of [`Set`]s.
///
/// This `struct` is created by the [`intersection`] method on [`Set`].
/// See its documentation for more.
///
/// [`intersection`]: Set::intersection
pub struct Intersection<'a, T: 'a> {
    inner: detail::IntersectionImpl<'a, T>,
}

impl<T> Debug for Intersection<'_, T>
where
    T: Debug + Hash + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> Clone for Intersection<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T> Iterator for Intersection<'a, T>
where
    T: Hash + Eq + Ord,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for Intersection<'a, T>
where
    T: Hash + Eq + Ord,
    detail::IntersectionImpl<'a, T>: FusedIterator,
{
}

/// A lazy iterator producing elements in the symmetric difference of [`Set`]s.
///
/// This `struct` is created by the [`symmetric_difference`] method on
/// [`Set`]. See its documentation for more.
///
/// [`symmetric_difference`]: Set::symmetric_difference
pub struct SymmetricDifference<'a, T: 'a> {
    inner: detail::SymmetricDifferenceImpl<'a, T>,
}

impl<T> Debug for SymmetricDifference<'_, T>
where
    T: Debug + Hash + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> Clone for SymmetricDifference<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T> Iterator for SymmetricDifference<'a, T>
where
    T: Hash + Eq + Ord,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for SymmetricDifference<'a, T>
where
    T: Hash + Eq + Ord,
    detail::SymmetricDifferenceImpl<'a, T>: FusedIterator,
{
}

/// A lazy iterator producing elements in the union of [`Set`]s.
///
/// This `struct` is created by the [`union`] method on
/// [`Set`]. See its documentation for more.
///
/// [`union`]: Set::union
pub struct Union<'a, T: 'a> {
    inner: detail::UnionImpl<'a, T>,
}

impl<T> Debug for Union<'_, T>
where
    T: Debug + Hash + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> Clone for Union<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T> Iterator for Union<'a, T>
where
    T: Hash + Eq + Ord,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for Union<'a, T>
where
    T: Hash + Eq + Ord,
    detail::UnionImpl<'a, T>: FusedIterator,
{
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Set<T>
where
    T: serde::Serialize + Eq + Hash + Ord,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serde::Serialize::serialize(&self.inner, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'a, T> serde::Deserialize<'a> for Set<T>
where
    T: serde::Deserialize<'a> + Eq + Hash + Ord,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        Ok(Set {
            inner: serde::Deserialize::deserialize(deserializer)?,
        })
    }
}
