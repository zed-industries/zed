use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::iter::{DoubleEndedIterator, FromIterator};
use core::ops::{BitAnd, BitOr, RangeInclusive};

#[cfg(feature = "serde1")]
use core::marker::PhantomData;
#[cfg(feature = "serde1")]
use serde::{
    de::{Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};

use crate::std_ext::*;
use crate::RangeInclusiveMap;

/// Intersection iterator over two [`RangeInclusiveSet`].
pub type Intersection<'a, T> = crate::operations::Intersection<'a, RangeInclusive<T>, Iter<'a, T>>;

/// Union iterator over two [`RangeInclusiveSet`].
pub type Union<'a, T> = crate::operations::Union<'a, RangeInclusive<T>, Iter<'a, T>>;

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// A set whose items are stored as ranges bounded
/// inclusively below and above `(start..=end)`.
///
/// See [`RangeInclusiveMap`]'s documentation for more details.
///
/// [`RangeInclusiveMap`]: crate::RangeInclusiveMap
pub struct RangeInclusiveSet<T, StepFnsT = T> {
    rm: RangeInclusiveMap<T, (), StepFnsT>,
}

impl<T, StepFnsT> Default for RangeInclusiveSet<T, StepFnsT> {
    fn default() -> Self {
        Self {
            rm: RangeInclusiveMap::default(),
        }
    }
}

#[cfg(feature = "quickcheck")]
impl<K> quickcheck::Arbitrary for RangeInclusiveSet<K>
where
    K: quickcheck::Arbitrary + Ord + StepLite,
{
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            rm: RangeInclusiveMap::arbitrary(g),
        }
    }
}

impl<T> RangeInclusiveSet<T, T>
where
    T: Ord + Clone + StepLite,
{
    /// Makes a new empty `RangeInclusiveSet`.
    #[cfg(feature = "const_fn")]
    pub const fn new() -> Self {
        Self::new_with_step_fns()
    }

    /// Makes a new empty `RangeInclusiveSet`.
    #[cfg(not(feature = "const_fn"))]
    pub fn new() -> Self {
        Self::new_with_step_fns()
    }
}

impl<T, StepFnsT> RangeInclusiveSet<T, StepFnsT>
where
    T: Ord + Clone,
    StepFnsT: StepFns<T>,
{
    /// Makes a new empty `RangeInclusiveSet`, specifying successor and
    /// predecessor functions defined separately from `T` itself.
    ///
    /// This is useful as a workaround for Rust's "orphan rules",
    /// which prevent you from implementing `StepLite` for `T` if `T`
    /// is a foreign type.
    ///
    /// **NOTE:** This will likely be deprecated and then eventually
    /// removed once the standard library's [Step](core::iter::Step)
    /// trait is stabilised, as most crates will then likely implement [Step](core::iter::Step)
    /// for their types where appropriate.
    ///
    /// See [this issue](https://github.com/rust-lang/rust/issues/42168)
    /// for details about that stabilization process.
    #[cfg(not(feature = "const_fn"))]
    pub fn new_with_step_fns() -> Self {
        Self {
            rm: RangeInclusiveMap::new_with_step_fns(),
        }
    }

    #[cfg(feature = "const_fn")]
    pub const fn new_with_step_fns() -> Self {
        Self {
            rm: RangeInclusiveMap::new_with_step_fns(),
        }
    }

    /// Returns a reference to the range covering the given key, if any.
    pub fn get(&self, value: &T) -> Option<&RangeInclusive<T>> {
        self.rm.get_key_value(value).map(|(range, _)| range)
    }

    /// Returns `true` if any range in the set covers the specified value.
    pub fn contains(&self, value: &T) -> bool {
        self.rm.contains_key(value)
    }

    /// Gets an ordered iterator over all ranges,
    /// ordered by range.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.rm.iter(),
        }
    }

    /// Clears the set, removing all elements.
    pub fn clear(&mut self) {
        self.rm.clear();
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.rm.len()
    }

    /// Returns true if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.rm.is_empty()
    }

    /// Insert a range into the set.
    ///
    /// If the inserted range either overlaps or is immediately adjacent
    /// any existing range, then the ranges will be coalesced into
    /// a single contiguous range.
    ///
    /// # Panics
    ///
    /// Panics if range `start > end`.
    pub fn insert(&mut self, range: RangeInclusive<T>) {
        self.rm.insert(range, ());
    }

    /// Removes a range from the set, if all or any of it was present.
    ///
    /// If the range to be removed _partially_ overlaps any ranges
    /// in the set, then those ranges will be contracted to no
    /// longer cover the removed range.
    ///
    /// # Panics
    ///
    /// Panics if range `start > end`.
    pub fn remove(&mut self, range: RangeInclusive<T>) {
        self.rm.remove(range);
    }

    /// Gets an iterator over all the maximally-sized ranges
    /// contained in `outer_range` that are not covered by
    /// any range stored in the set.
    ///
    /// The iterator element type is `RangeInclusive<T>`.
    pub fn gaps<'a>(&'a self, outer_range: &'a RangeInclusive<T>) -> Gaps<'a, T, StepFnsT> {
        Gaps {
            inner: self.rm.gaps(outer_range),
        }
    }

    /// Gets an iterator over all the stored ranges that are
    /// either partially or completely overlapped by the given range.
    ///
    /// The iterator element type is `RangeInclusive<T>`.
    pub fn overlapping<R: Borrow<RangeInclusive<T>>>(&'_ self, range: R) -> Overlapping<'_, T, R> {
        Overlapping {
            inner: self.rm.overlapping(range),
        }
    }

    /// Returns `true` if any range in the set completely or partially
    /// overlaps the given range.
    pub fn overlaps(&self, range: &RangeInclusive<T>) -> bool {
        self.overlapping(range).next().is_some()
    }

    /// Returns the first range in the set, if one exists. The range is the minimum range in this
    /// set.
    pub fn first(&self) -> Option<&RangeInclusive<T>> {
        self.rm.first_range_value().map(|(range, _)| range)
    }

    /// Returns the last range in the set, if one exists. The range is the maximum range in this
    /// set.
    pub fn last(&self) -> Option<&RangeInclusive<T>> {
        self.rm.last_range_value().map(|(range, _)| range)
    }

    /// Return an iterator over the union of two range sets.
    pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T> {
        Union::new(self.iter(), other.iter())
    }

    /// Return an iterator over the intersection of two range sets.
    pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T> {
        Intersection::new(self.iter(), other.iter())
    }
}

/// An iterator over the ranges of a `RangeInclusiveSet`.
///
/// This `struct` is created by the [`iter`] method on [`RangeInclusiveSet`]. See its
/// documentation for more.
///
/// [`iter`]: RangeInclusiveSet::iter
pub struct Iter<'a, T> {
    inner: super::inclusive_map::Iter<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(range, _)| range)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K> DoubleEndedIterator for Iter<'a, K>
where
    K: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(range, _)| range)
    }
}

/// An owning iterator over the ranges of a `RangeInclusiveSet`.
///
/// This `struct` is created by the [`into_iter`] method on [`RangeInclusiveSet`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: IntoIterator::into_iter
pub struct IntoIter<T> {
    inner: super::inclusive_map::IntoIter<T, ()>,
}

impl<T> IntoIterator for RangeInclusiveSet<T> {
    type Item = RangeInclusive<T>;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.rm.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = RangeInclusive<T>;
    fn next(&mut self) -> Option<RangeInclusive<T>> {
        self.inner.next().map(|(range, _)| range)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<K> DoubleEndedIterator for IntoIter<K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(range, _)| range)
    }
}

// We can't just derive this automatically, because that would
// expose irrelevant (and private) implementation details.
// Instead implement it in the same way that the underlying BTreeSet does.
impl<T: Debug> Debug for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T> FromIterator<RangeInclusive<T>> for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite,
{
    fn from_iter<I: IntoIterator<Item = RangeInclusive<T>>>(iter: I) -> Self {
        let mut range_set = RangeInclusiveSet::new();
        range_set.extend(iter);
        range_set
    }
}

impl<T> Extend<RangeInclusive<T>> for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite,
{
    fn extend<I: IntoIterator<Item = RangeInclusive<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |range| {
            self.insert(range);
        })
    }
}

#[cfg(feature = "serde1")]
impl<T> Serialize for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.rm.btm.len()))?;
        for range in self.iter() {
            seq.serialize_element(&(&range.start(), &range.end()))?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde1")]
impl<'de, T> Deserialize<'de> for RangeInclusiveSet<T>
where
    T: Ord + Clone + StepLite + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RangeInclusiveSetVisitor::new())
    }
}

#[cfg(feature = "serde1")]
struct RangeInclusiveSetVisitor<T> {
    marker: PhantomData<fn() -> RangeInclusiveSet<T>>,
}

#[cfg(feature = "serde1")]
impl<T> RangeInclusiveSetVisitor<T> {
    fn new() -> Self {
        RangeInclusiveSetVisitor {
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "serde1")]
impl<'de, T> Visitor<'de> for RangeInclusiveSetVisitor<T>
where
    T: Ord + Clone + StepLite + Deserialize<'de>,
{
    type Value = RangeInclusiveSet<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("RangeInclusiveSet")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut range_inclusive_set = RangeInclusiveSet::new();
        while let Some((start, end)) = access.next_element()? {
            range_inclusive_set.insert(start..=end);
        }
        Ok(range_inclusive_set)
    }
}

/// An iterator over all ranges not covered by a `RangeInclusiveSet`.
///
/// This `struct` is created by the [`gaps`] method on [`RangeInclusiveSet`]. See its
/// documentation for more.
///
/// [`gaps`]: RangeInclusiveSet::gaps
pub struct Gaps<'a, T, StepFnsT> {
    inner: crate::inclusive_map::Gaps<'a, T, (), StepFnsT>,
}

// `Gaps` is always fused. (See definition of `next` below.)
impl<'a, T, StepFnsT> core::iter::FusedIterator for Gaps<'a, T, StepFnsT>
where
    T: Ord + Clone,
    StepFnsT: StepFns<T>,
{
}

impl<'a, T, StepFnsT> Iterator for Gaps<'a, T, StepFnsT>
where
    T: Ord + Clone,
    StepFnsT: StepFns<T>,
{
    type Item = RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator over all stored ranges partially or completely
/// overlapped by a given range.
///
/// This `struct` is created by the [`overlapping`] method on [`RangeInclusiveSet`]. See its
/// documentation for more.
///
/// [`overlapping`]: RangeInclusiveSet::overlapping
pub struct Overlapping<'a, T, R: Borrow<RangeInclusive<T>> = &'a RangeInclusive<T>> {
    inner: crate::inclusive_map::Overlapping<'a, T, (), R>,
}

// `Overlapping` is always fused. (See definition of `next` below.)
impl<'a, T, R: Borrow<RangeInclusive<T>>> core::iter::FusedIterator for Overlapping<'a, T, R> where
    T: Ord + Clone
{
}

impl<'a, T, R: Borrow<RangeInclusive<T>>> Iterator for Overlapping<'a, T, R>
where
    T: Ord + Clone,
{
    type Item = &'a RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _v)| k)
    }
}

impl<'a, T, R: Borrow<RangeInclusive<T>>> DoubleEndedIterator for Overlapping<'a, T, R>
where
    T: Ord + Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(k, _v)| k)
    }
}

impl<T: Ord + Clone + StepLite, const N: usize> From<[RangeInclusive<T>; N]>
    for RangeInclusiveSet<T>
{
    fn from(value: [RangeInclusive<T>; N]) -> Self {
        let mut set = Self::new();
        for value in IntoIterator::into_iter(value) {
            set.insert(value);
        }
        set
    }
}

/// Create a [`RangeInclusiveSet`] from a list of ranges.
///
/// # Example
///
/// ```rust
/// # use rangemap::range_inclusive_set;
/// let set = range_inclusive_set![0..=100, 200..=300, 400..=500];
/// ```
#[macro_export]
macro_rules! range_inclusive_set {
    ($($range:expr),* $(,)?) => {{
        $crate::RangeInclusiveSet::from([$($range),*])
    }};
}

impl<T: Ord + Clone + StepLite> BitAnd for &RangeInclusiveSet<T> {
    type Output = RangeInclusiveSet<T>;

    fn bitand(self, other: Self) -> Self::Output {
        self.intersection(other).collect()
    }
}

impl<T: Ord + Clone + StepLite> BitOr for &RangeInclusiveSet<T> {
    type Output = RangeInclusiveSet<T>;

    fn bitor(self, other: Self) -> Self::Output {
        self.union(other).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc as std;
    use alloc::{format, vec, vec::Vec};
    use proptest::prelude::*;
    use test_strategy::proptest;

    impl<T> Arbitrary for RangeInclusiveSet<T>
    where
        T: Ord + Clone + StepLite + Debug + Arbitrary + 'static,
    {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_parameters: Self::Parameters) -> Self::Strategy {
            any::<Vec<RangeInclusive<T>>>()
                .prop_map(|ranges| ranges.into_iter().collect::<RangeInclusiveSet<T>>())
                .boxed()
        }
    }

    #[proptest]
    fn test_first(set: RangeInclusiveSet<u64>) {
        assert_eq!(set.first(), set.iter().min_by_key(|range| range.start()));
    }

    #[proptest]
    #[allow(clippy::len_zero)]
    fn test_len(mut map: RangeInclusiveSet<u64>) {
        assert_eq!(map.len(), map.iter().count());
        assert_eq!(map.is_empty(), map.len() == 0);
        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        assert_eq!(map.iter().count(), 0);
    }

    #[proptest]
    fn test_last(set: RangeInclusiveSet<u64>) {
        assert_eq!(set.last(), set.iter().max_by_key(|range| range.end()));
    }

    #[proptest]
    fn test_iter_reversible(set: RangeInclusiveSet<u64>) {
        let forward: Vec<_> = set.iter().collect();
        let mut backward: Vec<_> = set.iter().rev().collect();
        backward.reverse();
        assert_eq!(forward, backward);
    }

    #[proptest]
    fn test_into_iter_reversible(set: RangeInclusiveSet<u64>) {
        let forward: Vec<_> = set.clone().into_iter().collect();
        let mut backward: Vec<_> = set.into_iter().rev().collect();
        backward.reverse();
        assert_eq!(forward, backward);
    }

    #[proptest]
    fn test_overlapping_reversible(set: RangeInclusiveSet<u64>, range: RangeInclusive<u64>) {
        let forward: Vec<_> = set.overlapping(&range).collect();
        let mut backward: Vec<_> = set.overlapping(&range).rev().collect();
        backward.reverse();
        assert_eq!(forward, backward);
    }

    // neccessary due to assertion on empty ranges
    fn filter_ranges<T: Ord>(ranges: Vec<RangeInclusive<T>>) -> Vec<RangeInclusive<T>> {
        ranges
            .into_iter()
            .filter(|range| range.start() != range.end())
            .collect()
    }

    #[proptest]
    fn test_arbitrary_set_u8(ranges: Vec<RangeInclusive<u8>>) {
        let ranges: Vec<_> = filter_ranges(ranges);
        let set = ranges
            .iter()
            .fold(RangeInclusiveSet::new(), |mut set, range| {
                set.insert(range.clone());
                set
            });

        for value in 0..u8::MAX {
            assert_eq!(
                set.contains(&value),
                ranges.iter().any(|range| range.contains(&value))
            );
        }
    }

    #[proptest]
    #[allow(deprecated)]
    fn test_hash(left: RangeInclusiveSet<u64>, right: RangeInclusiveSet<u64>) {
        use core::hash::{Hash, Hasher, SipHasher};

        let hash = |set: &RangeInclusiveSet<_>| {
            let mut hasher = SipHasher::new();
            set.hash(&mut hasher);
            hasher.finish()
        };

        if left == right {
            assert!(
                hash(&left) == hash(&right),
                "if two values are equal, their hash must be equal"
            );
        }

        // if the hashes are equal the values might not be the same (collision)
        if hash(&left) != hash(&right) {
            assert!(
                left != right,
                "if two value's hashes are not equal, they must not be equal"
            );
        }
    }

    #[proptest]
    fn test_ord(left: RangeInclusiveSet<u64>, right: RangeInclusiveSet<u64>) {
        assert_eq!(
            left == right,
            left.cmp(&right).is_eq(),
            "ordering and equality must match"
        );
        assert_eq!(
            left.cmp(&right),
            left.partial_cmp(&right).unwrap(),
            "ordering is total for ordered parameters"
        );
    }

    #[test]
    fn test_from_array() {
        let mut set = RangeInclusiveSet::new();
        set.insert(0..=100);
        set.insert(200..=300);
        assert_eq!(set, RangeInclusiveSet::from([0..=100, 200..=300]));
    }

    #[test]
    fn test_macro() {
        assert_eq!(range_inclusive_set![], RangeInclusiveSet::<i64>::default());
        assert_eq!(
            range_inclusive_set![0..=100, 200..=300, 400..=500],
            [0..=100, 200..=300, 400..=500].iter().cloned().collect(),
        );
    }

    #[proptest]
    fn test_union_overlaps_u8(left: RangeInclusiveSet<u8>, right: RangeInclusiveSet<u8>) {
        let mut union = RangeInclusiveSet::new();
        for range in left.union(&right) {
            // there should not be any overlaps in the ranges returned by the union
            assert!(union.overlapping(&range).next().is_none());
            union.insert(range);
        }
    }

    #[proptest]
    fn test_union_contains_u8(left: RangeInclusiveSet<u8>, right: RangeInclusiveSet<u8>) {
        let union = (&left) | (&right);

        // value should be in the union if and only if it is in either set
        for value in 0..u8::MAX {
            assert_eq!(
                union.contains(&value),
                left.contains(&value) || right.contains(&value)
            );
        }
    }

    #[proptest]
    fn test_intersection_contains_u8(left: RangeInclusiveSet<u8>, right: RangeInclusiveSet<u8>) {
        let intersection = (&left) & (&right);

        // value should be in the intersection if and only if it is in both sets
        for value in 0..u8::MAX {
            assert_eq!(
                intersection.contains(&value),
                left.contains(&value) && right.contains(&value)
            );
        }
    }

    #[proptest]
    fn test_intersection_overlaps_u8(left: RangeInclusiveSet<u8>, right: RangeInclusiveSet<u8>) {
        let mut union = RangeInclusiveSet::new();
        for range in left.intersection(&right) {
            // there should not be any overlaps in the ranges returned by the
            // intersection
            assert!(union.overlapping(&range).next().is_none());
            union.insert(range);
        }
    }

    trait RangeInclusiveSetExt<T> {
        fn to_vec(&self) -> Vec<RangeInclusive<T>>;
    }

    impl<T> RangeInclusiveSetExt<T> for RangeInclusiveSet<T>
    where
        T: Ord + Clone + StepLite,
    {
        fn to_vec(&self) -> Vec<RangeInclusive<T>> {
            self.iter().cloned().collect()
        }
    }

    #[test]
    fn empty_set_is_empty() {
        let range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        assert_eq!(range_set.to_vec(), vec![]);
    }

    #[test]
    fn insert_into_empty_map() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        range_set.insert(0..=50);
        assert_eq!(range_set.to_vec(), vec![0..=50]);
    }

    #[test]
    fn remove_partially_overlapping() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        range_set.insert(0..=50);
        range_set.remove(25..=75);
        assert_eq!(range_set.to_vec(), vec![0..=24]);
    }

    #[test]
    fn gaps_between_items_floating_inside_outer_range() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●-● ◌ ◌ ◌
        range_set.insert(5..=6);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ●-● ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(2..=3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◆ ◌
        let outer_range = 1..=8;
        let mut gaps = range_set.gaps(&outer_range);
        // Should yield gaps at start, between items,
        // and at end.
        assert_eq!(gaps.next(), Some(1..=1));
        assert_eq!(gaps.next(), Some(4..=4));
        assert_eq!(gaps.next(), Some(7..=8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn overlapping_partial_edges_complete_middle() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();

        // 0 1 2 3 4 5 6 7 8 9
        // ●-● ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(0..=1);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-● ◌ ◌ ◌ ◌ ◌
        range_set.insert(3..=4);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ●-● ◌ ◌
        range_set.insert(6..=7);

        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---------◆ ◌ ◌ ◌
        let query_range = 1..=6;

        let mut overlapping = range_set.overlapping(&query_range);

        // Should yield partially overlapped range at start.
        assert_eq!(overlapping.next(), Some(&(0..=1)));
        // Should yield completely overlapped range in middle.
        assert_eq!(overlapping.next(), Some(&(3..=4)));
        // Should yield partially overlapped range at end.
        assert_eq!(overlapping.next(), Some(&(6..=7)));
        // Gaps iterator should be fused.
        assert_eq!(overlapping.next(), None);
        assert_eq!(overlapping.next(), None);
    }

    ///
    /// impl Debug
    ///

    #[test]
    fn set_debug_repr_looks_right() {
        let mut set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();

        // Empty
        assert_eq!(format!("{:?}", set), "{}");

        // One entry
        set.insert(2..=5);
        assert_eq!(format!("{:?}", set), "{2..=5}");

        // Many entries
        set.insert(7..=8);
        set.insert(10..=11);
        assert_eq!(format!("{:?}", set), "{2..=5, 7..=8, 10..=11}");
    }

    // impl Default where T: ?Default

    #[test]
    fn always_default() {
        struct NoDefault;
        RangeInclusiveSet::<NoDefault>::default();
    }

    // impl Serialize

    #[cfg(feature = "serde1")]
    #[test]
    fn serialization() {
        let mut range_set: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---◆ ◌ ◌ ◌ ◌ ◌ ◌
        range_set.insert(1..=3);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆---◆ ◌ ◌
        range_set.insert(5..=7);
        let output = serde_json::to_string(&range_set).expect("Failed to serialize");
        assert_eq!(output, "[[1,3],[5,7]]");
    }

    // impl Deserialize

    #[cfg(feature = "serde1")]
    #[test]
    fn deserialization() {
        let input = "[[1,3],[5,7]]";
        let range_set: RangeInclusiveSet<u32> =
            serde_json::from_str(input).expect("Failed to deserialize");
        let reserialized = serde_json::to_string(&range_set).expect("Failed to re-serialize");
        assert_eq!(reserialized, input);
    }

    // const fn

    #[cfg(feature = "const_fn")]
    const _SET: RangeInclusiveSet<u32> = RangeInclusiveSet::new();
    #[cfg(feature = "const_fn")]
    const _SET2: RangeInclusiveSet<u32> = RangeInclusiveSet::new_with_step_fns();

    #[cfg(feature = "quickcheck")]
    quickcheck::quickcheck! {
        fn prop(xs: RangeInclusiveSet<usize, usize>) -> bool {
            xs == xs
        }
    }
}
