use super::range_wrapper::RangeStartWrapper;
use crate::range_wrapper::RangeEndWrapper;
use crate::std_ext::*;
use alloc::collections::BTreeMap;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::hash::Hash;
use core::iter::{DoubleEndedIterator, FromIterator};
use core::ops::{Bound, Range};
use core::prelude::v1::*;

#[cfg(feature = "serde1")]
use core::marker::PhantomData;
#[cfg(feature = "serde1")]
use serde::{
    de::{Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};

/// A map whose keys are stored as (half-open) ranges bounded
/// inclusively below and exclusively above `(start..end)`.
///
/// Contiguous and overlapping ranges that map to the same value
/// are coalesced into a single range.
#[derive(Clone, Eq)]
pub struct RangeMap<K, V> {
    // Wrap ranges so that they are `Ord`.
    // See `range_wrapper.rs` for explanation.
    pub(crate) btm: BTreeMap<RangeStartWrapper<K>, V>,
}

impl<K, V> Default for RangeMap<K, V> {
    fn default() -> Self {
        Self {
            btm: BTreeMap::default(),
        }
    }
}

impl<K, V> Hash for RangeMap<K, V>
where
    K: Hash,
    V: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.btm.len());
        for elt in self.iter() {
            elt.hash(state);
        }
    }
}

impl<K, V> PartialEq for RangeMap<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &RangeMap<K, V>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<K, V> PartialOrd for RangeMap<K, V>
where
    K: PartialOrd,
    V: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &RangeMap<K, V>) -> Option<Ordering> {
        self.expanded_iter().partial_cmp(other.expanded_iter())
    }
}

impl<K, V> Ord for RangeMap<K, V>
where
    K: Ord,
    V: Ord,
{
    #[inline]
    fn cmp(&self, other: &RangeMap<K, V>) -> Ordering {
        self.expanded_iter().cmp(other.expanded_iter())
    }
}

#[cfg(feature = "quickcheck")]
impl<K, V> quickcheck::Arbitrary for RangeMap<K, V>
where
    K: quickcheck::Arbitrary + Ord,
    V: quickcheck::Arbitrary + PartialEq,
{
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // REVISIT: allocation could be avoided if Gen::gen_size were public (https://github.com/BurntSushi/quickcheck/issues/326#issue-2653601170)
        <alloc::vec::Vec<(Range<_>, _)>>::arbitrary(g)
            .into_iter()
            .filter(|(range, _)| !range.is_empty())
            .collect()
    }
}

impl<K, V> RangeMap<K, V> {
    /// Makes a new empty `RangeMap`.
    #[cfg(feature = "const_fn")]
    pub const fn new() -> Self {
        RangeMap {
            btm: BTreeMap::new(),
        }
    }

    /// Makes a new empty `RangeMap`.
    #[cfg(not(feature = "const_fn"))]
    pub fn new() -> Self {
        RangeMap {
            btm: BTreeMap::new(),
        }
    }

    /// Gets an iterator over all pairs of key range and value,
    /// ordered by key range.
    ///
    /// The iterator element type is `(&'a Range<K>, &'a V)`.
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.btm.iter(),
        }
    }

    /// Clears the map, removing all elements.
    pub fn clear(&mut self) {
        self.btm.clear();
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.btm.len()
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.btm.is_empty()
    }

    /// Returns an iterator that includes both ends of the key range.
    ///
    /// Mainly used for comparisons.
    fn expanded_iter(&self) -> impl Iterator<Item = (&K, &K, &V)> {
        self.btm.iter().map(|(k, v)| (&k.start, &k.end, v))
    }
}

impl<K, V> RangeMap<K, V>
where
    K: Ord + Clone,
{
    /// Returns a reference to the value corresponding to the given key,
    /// if the key is covered by any range in the map.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_key_value(key).map(|(_range, value)| value)
    }

    /// Returns the range-value pair (as a pair of references) corresponding
    /// to the given key, if the key is covered by any range in the map.
    pub fn get_key_value(&self, key: &K) -> Option<(&Range<K>, &V)> {
        // The only stored range that could contain the given key is the
        // last stored range whose start is less than or equal to this key.
        let key_as_start = RangeStartWrapper::new(key.clone()..key.clone());
        self.btm
            .range((Bound::Unbounded, Bound::Included(key_as_start)))
            .next_back()
            .filter(|(start_wrapper, _value)| {
                // Does the only candidate range contain
                // the requested key?
                start_wrapper.end_wrapper.range.contains(key)
            })
            .map(|(start_wrapper, value)| (&start_wrapper.end_wrapper.range, value))
    }

    /// Returns `true` if any range in the map covers the specified key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Gets an iterator over all the maximally-sized ranges
    /// contained in `outer_range` that are not covered by
    /// any range stored in the map.
    ///
    /// If the start and end of the outer range are the same
    /// and it does not overlap any stored range, then a single
    /// empty gap will be returned.
    ///
    /// The iterator element type is `Range<K>`.
    pub fn gaps<'a>(&'a self, outer_range: &'a Range<K>) -> Gaps<'a, K, V> {
        let overlap_iter = self.overlapping(outer_range);
        Gaps {
            candidate_start: &outer_range.start,
            query_end: &outer_range.end,
            btm_range_iter: overlap_iter.btm_range_iter,
        }
    }

    /// Gets an iterator over all the stored ranges that are
    /// either partially or completely overlapped by the given range.
    pub fn overlapping<R: Borrow<Range<K>>>(&'_ self, range: R) -> Overlapping<'_, K, V, R> {
        // Find the first matching stored range by its _end_,
        // using sneaky layering and `Borrow` implementation. (See `range_wrappers` module.)
        let start_sliver =
            RangeEndWrapper::new(range.borrow().start.clone()..range.borrow().start.clone());
        let btm_range_iter = self
            .btm
            .range::<RangeEndWrapper<K>, (Bound<&RangeEndWrapper<K>>, Bound<_>)>((
                Bound::Excluded(&start_sliver),
                Bound::Unbounded,
            ));
        Overlapping {
            query_range: range,
            btm_range_iter,
        }
    }

    /// Returns `true` if any range in the map completely or partially
    /// overlaps the given range.
    pub fn overlaps(&self, range: &Range<K>) -> bool {
        self.overlapping(range).next().is_some()
    }

    /// Returns the first range-value pair in this map, if one exists. The range in this pair is the
    /// minimum range in the map.
    pub fn first_range_value(&self) -> Option<(&Range<K>, &V)> {
        self.btm
            .first_key_value()
            .map(|(range, value)| (&range.end_wrapper.range, value))
    }

    /// Returns the last range-value pair in this map, if one exists. The range in this pair is the
    /// maximum range in the map.
    pub fn last_range_value(&self) -> Option<(&Range<K>, &V)> {
        self.btm
            .last_key_value()
            .map(|(range, value)| (&range.end_wrapper.range, value))
    }
}

impl<K, V> RangeMap<K, V>
where
    K: Ord + Clone,
    V: PartialEq + Clone,
{
    /// Insert a pair of key range and value into the map.
    ///
    /// If the inserted range partially or completely overlaps any
    /// existing range in the map, then the existing range (or ranges) will be
    /// partially or completely replaced by the inserted range.
    ///
    /// If the inserted range either overlaps or is immediately adjacent
    /// any existing range _mapping to the same value_, then the ranges
    /// will be coalesced into a single contiguous range.
    ///
    /// # Panics
    ///
    /// Panics if range `start >= end`.
    pub fn insert(&mut self, range: Range<K>, value: V) {
        // We don't want to have to make empty ranges make sense;
        // they don't represent anything meaningful in this structure.
        assert!(range.start < range.end);

        // Wrap up the given range so that we can "borrow"
        // it as a wrapper reference to either its start or end.
        // See `range_wrapper.rs` for explanation of these hacks.
        let mut new_start_wrapper: RangeStartWrapper<K> = RangeStartWrapper::new(range);
        let new_value = value;

        // Is there a stored range either overlapping the start of
        // the range to insert or immediately preceding it?
        //
        // If there is any such stored range, it will be the last
        // whose start is less than or equal to the start of the range to insert,
        // or the one before that if both of the above cases exist.
        let mut candidates = self
            .btm
            .range::<RangeStartWrapper<K>, (Bound<&RangeStartWrapper<K>>, Bound<&RangeStartWrapper<K>>)>((
                Bound::Unbounded,
                Bound::Included(&new_start_wrapper),
            ))
            .rev()
            .take(2)
            .filter(|(stored_start_wrapper, _stored_value)| {
                // Does the candidate range either overlap
                // or immediately precede the range to insert?
                // (Remember that it might actually cover the _whole_
                // range to insert and then some.)
                stored_start_wrapper
                    .end_wrapper
                    .range
                    .touches(&new_start_wrapper.end_wrapper.range)
            });
        if let Some(mut candidate) = candidates.next() {
            // Or the one before it if both cases described above exist.
            if let Some(another_candidate) = candidates.next() {
                candidate = another_candidate;
            }
            let (stored_start_wrapper, stored_value) = (candidate.0.clone(), candidate.1.clone());
            self.adjust_touching_ranges_for_insert(
                stored_start_wrapper,
                stored_value,
                &mut new_start_wrapper.end_wrapper.range,
                &new_value,
            );
        }

        // Are there any stored ranges whose heads overlap or immediately
        // follow the range to insert?
        //
        // If there are any such stored ranges (that weren't already caught above),
        // their starts will fall somewhere after the start of the range to insert,
        // and on or before its end.
        //
        // This time around, if the latter holds, it also implies
        // the former so we don't need to check here if they touch.
        //
        // REVISIT: Possible micro-optimisation: `impl Borrow<T> for RangeStartWrapper<T>`
        // and use that to search here, to avoid constructing another `RangeStartWrapper`.
        let new_range_end_as_start = RangeStartWrapper::new(
            new_start_wrapper.end_wrapper.range.end.clone()
                ..new_start_wrapper.end_wrapper.range.end.clone(),
        );
        while let Some((stored_start_wrapper, stored_value)) = self
            .btm
            .range::<RangeStartWrapper<K>, (Bound<&RangeStartWrapper<K>>, Bound<&RangeStartWrapper<K>>)>((
                Bound::Included(&new_start_wrapper),
                Bound::Included(&new_range_end_as_start),
            ))
            .next()
        {
            // One extra exception: if we have different values,
            // and the stored range starts at the end of the range to insert,
            // then we don't want to keep looping forever trying to find more!
            #[allow(clippy::suspicious_operation_groupings)]
            if stored_start_wrapper.end_wrapper.range.start
                == new_start_wrapper.end_wrapper.range.end
                && *stored_value != new_value
            {
                // We're beyond the last stored range that could be relevant.
                // Avoid wasting time on irrelevant ranges, or even worse, looping forever.
                // (`adjust_touching_ranges_for_insert` below assumes that the given range
                // is relevant, and behaves very poorly if it is handed a range that it
                // shouldn't be touching.)
                break;
            }

            let stored_start_wrapper = stored_start_wrapper.clone();
            let stored_value = stored_value.clone();

            self.adjust_touching_ranges_for_insert(
                stored_start_wrapper,
                stored_value,
                &mut new_start_wrapper.end_wrapper.range,
                &new_value,
            );
        }

        // Insert the (possibly expanded) new range, and we're done!
        self.btm.insert(new_start_wrapper, new_value);
    }

    /// Removes a range from the map, if all or any of it was present.
    ///
    /// If the range to be removed _partially_ overlaps any ranges
    /// in the map, then those ranges will be contracted to no
    /// longer cover the removed range.
    ///
    ///
    /// # Panics
    ///
    /// Panics if range `start >= end`.
    pub fn remove(&mut self, range: Range<K>) {
        // We don't want to have to make empty ranges make sense;
        // they don't represent anything meaningful in this structure.
        assert!(range.start < range.end);

        let start_wrapper: RangeStartWrapper<K> = RangeStartWrapper::new(range);
        let range = &start_wrapper.end_wrapper.range;

        // Is there a stored range overlapping the start of
        // the range to insert?
        //
        // If there is any such stored range, it will be the last
        // whose start is less than or equal to the start of the range to insert.
        if let Some((stored_start_wrapper, stored_value)) = self
            .btm
            .range::<RangeStartWrapper<K>, (Bound<&RangeStartWrapper<K>>, Bound<&RangeStartWrapper<K>>)>((Bound::Unbounded, Bound::Included(&start_wrapper)))
            .next_back()
            .filter(|(stored_start_wrapper, _stored_value)| {
                // Does the only candidate range overlap
                // the range to insert?
                stored_start_wrapper
                    .end_wrapper
                    .range
                    .overlaps(range)
            })
            .map(|(stored_start_wrapper, stored_value)| {
                (stored_start_wrapper.clone(), stored_value.clone())
            })
        {
            self.adjust_overlapping_ranges_for_remove(
                stored_start_wrapper,
                stored_value,
                range,
            );
        }

        // Are there any stored ranges whose heads overlap the range to insert?
        //
        // If there are any such stored ranges (that weren't already caught above),
        // their starts will fall somewhere after the start of the range to insert,
        // and before its end.
        //
        // REVISIT: Possible micro-optimisation: `impl Borrow<T> for RangeStartWrapper<T>`
        // and use that to search here, to avoid constructing another `RangeStartWrapper`.
        let new_range_end_as_start = RangeStartWrapper::new(range.end.clone()..range.end.clone());
        while let Some((stored_start_wrapper, stored_value)) = self
            .btm
            .range::<RangeStartWrapper<K>, (Bound<&RangeStartWrapper<K>>, Bound<&RangeStartWrapper<K>>)>((
                Bound::Excluded(&start_wrapper),
                Bound::Excluded(&new_range_end_as_start),
            ))
            .next()
            .map(|(stored_start_wrapper, stored_value)| {
                (stored_start_wrapper.clone(), stored_value.clone())
            })
        {
            self.adjust_overlapping_ranges_for_remove(
                stored_start_wrapper,
                stored_value,
                range,
            );
        }
    }

    fn adjust_touching_ranges_for_insert(
        &mut self,
        stored_start_wrapper: RangeStartWrapper<K>,
        stored_value: V,
        new_range: &mut Range<K>,
        new_value: &V,
    ) {
        use core::cmp::{max, min};

        if stored_value == *new_value {
            // The ranges have the same value, so we can "adopt"
            // the stored range.
            //
            // This means that no matter how big or where the stored range is,
            // we will expand the new range's bounds to subsume it,
            // and then delete the stored range.
            new_range.start = min(&new_range.start, &stored_start_wrapper.start).clone();
            new_range.end = max(&new_range.end, &stored_start_wrapper.end).clone();
            self.btm.remove(&stored_start_wrapper);
        } else {
            // The ranges have different values.
            if new_range.overlaps(&stored_start_wrapper.range) {
                // The ranges overlap. This is a little bit more complicated.
                // Delete the stored range, and then add back between
                // 0 and 2 subranges at the ends of the range to insert.
                self.btm.remove(&stored_start_wrapper);
                if stored_start_wrapper.start < new_range.start {
                    // Insert the piece left of the range to insert.
                    self.btm.insert(
                        RangeStartWrapper::new(
                            stored_start_wrapper.end_wrapper.range.start..new_range.start.clone(),
                        ),
                        stored_value.clone(),
                    );
                }
                if stored_start_wrapper.end_wrapper.range.end > new_range.end {
                    // Insert the piece right of the range to insert.
                    self.btm.insert(
                        RangeStartWrapper::new(
                            new_range.end.clone()..stored_start_wrapper.end_wrapper.range.end,
                        ),
                        stored_value,
                    );
                }
            } else {
                // No-op; they're not overlapping,
                // so we can just keep both ranges as they are.
            }
        }
    }

    fn adjust_overlapping_ranges_for_remove(
        &mut self,
        stored: RangeStartWrapper<K>,
        stored_value: V,
        range_to_remove: &Range<K>,
    ) {
        // Delete the stored range, and then add back between
        // 0 and 2 subranges at the ends of the range to insert.
        self.btm.remove(&stored);
        let stored_range = stored.end_wrapper;
        if stored_range.start < range_to_remove.start {
            // Insert the piece left of the range to insert.
            self.btm.insert(
                RangeStartWrapper::new(stored_range.range.start..range_to_remove.start.clone()),
                stored_value.clone(),
            );
        }
        if stored_range.range.end > range_to_remove.end {
            // Insert the piece right of the range to insert.
            self.btm.insert(
                RangeStartWrapper::new(range_to_remove.end.clone()..stored_range.range.end),
                stored_value,
            );
        }
    }
}

/// An iterator over the entries of a `RangeMap`, ordered by key range.
///
/// The iterator element type is `(&'a Range<K>, &'a V)`.
///
/// This `struct` is created by the [`iter`] method on [`RangeMap`]. See its
/// documentation for more.
///
/// [`iter`]: RangeMap::iter
pub struct Iter<'a, K, V> {
    inner: alloc::collections::btree_map::Iter<'a, RangeStartWrapper<K>, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    type Item = (&'a Range<K>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(by_start, v)| (&by_start.end_wrapper.range, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner
            .next_back()
            .map(|(range, value)| (&range.end_wrapper.range, value))
    }
}

/// An owning iterator over the entries of a `RangeMap`, ordered by key range.
///
/// The iterator element type is `(Range<K>, V)`.
///
/// This `struct` is created by the [`into_iter`] method on [`RangeMap`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: IntoIterator::into_iter
pub struct IntoIter<K, V> {
    inner: alloc::collections::btree_map::IntoIter<RangeStartWrapper<K>, V>,
}

impl<K, V> IntoIterator for RangeMap<K, V> {
    type Item = (Range<K>, V);
    type IntoIter = IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.btm.into_iter(),
        }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (Range<K>, V);
    fn next(&mut self) -> Option<(Range<K>, V)> {
        self.inner
            .next()
            .map(|(by_start, v)| (by_start.end_wrapper.range, v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner
            .next_back()
            .map(|(range, value)| (range.end_wrapper.range, value))
    }
}

// We can't just derive this automatically, because that would
// expose irrelevant (and private) implementation details.
// Instead implement it in the same way that the underlying BTreeMap does.
impl<K: Debug, V: Debug> Debug for RangeMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> FromIterator<(Range<K>, V)> for RangeMap<K, V>
where
    K: Ord + Clone,
    V: PartialEq + Clone,
{
    fn from_iter<T: IntoIterator<Item = (Range<K>, V)>>(iter: T) -> Self {
        let mut range_map = RangeMap::new();
        range_map.extend(iter);
        range_map
    }
}

impl<K, V> Extend<(Range<K>, V)> for RangeMap<K, V>
where
    K: Ord + Clone,
    V: PartialEq + Clone,
{
    fn extend<T: IntoIterator<Item = (Range<K>, V)>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |(k, v)| {
            self.insert(k, v);
        })
    }
}

#[cfg(feature = "serde1")]
impl<K, V> Serialize for RangeMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.btm.len()))?;
        for (k, v) in self.iter() {
            seq.serialize_element(&((&k.start, &k.end), &v))?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde1")]
impl<'de, K, V> Deserialize<'de> for RangeMap<K, V>
where
    K: Ord + Clone + Deserialize<'de>,
    V: PartialEq + Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RangeMapVisitor::new())
    }
}

#[cfg(feature = "serde1")]
struct RangeMapVisitor<K, V> {
    marker: PhantomData<fn() -> RangeMap<K, V>>,
}

#[cfg(feature = "serde1")]
impl<K, V> RangeMapVisitor<K, V> {
    fn new() -> Self {
        RangeMapVisitor {
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "serde1")]
impl<'de, K, V> Visitor<'de> for RangeMapVisitor<K, V>
where
    K: Ord + Clone + Deserialize<'de>,
    V: PartialEq + Clone + Deserialize<'de>,
{
    type Value = RangeMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("RangeMap")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut range_map = RangeMap::new();
        while let Some(((start, end), value)) = access.next_element()? {
            range_map.insert(start..end, value);
        }
        Ok(range_map)
    }
}

/// An iterator over all ranges not covered by a `RangeMap`.
///
/// The iterator element type is `Range<K>`.
///
/// This `struct` is created by the [`gaps`] method on [`RangeMap`]. See its
/// documentation for more.
///
/// [`gaps`]: RangeMap::gaps
pub struct Gaps<'a, K, V> {
    candidate_start: &'a K,
    query_end: &'a K,
    btm_range_iter: alloc::collections::btree_map::Range<'a, RangeStartWrapper<K>, V>,
}

// `Gaps` is always fused. (See definition of `next` below.)
impl<'a, K, V> core::iter::FusedIterator for Gaps<'a, K, V> where K: Ord + Clone {}

impl<'a, K, V> Iterator for Gaps<'a, K, V>
where
    K: Ord + Clone,
{
    type Item = Range<K>;

    fn next(&mut self) -> Option<Self::Item> {
        // Keep track of the next range in the map beyond the current returned range.
        for overlap in self.btm_range_iter.by_ref() {
            let overlap = &overlap.0.range;

            // If the range in the map has advanced beyond the query range, return
            // any tail gap.
            if *self.query_end <= overlap.start {
                break;
            }

            let original_start = core::mem::replace(&mut self.candidate_start, &overlap.end);

            // The query range overhangs to the left, return a gap.
            if *original_start < overlap.start {
                let gap = original_start.clone()..overlap.start.clone();
                return Some(gap);
            }

            // The remaining query range starts within the current
            // mapped range.
            self.candidate_start = &overlap.end;
        }

        // Now that we've run out of items, the only other possible
        // gap is at the end of the query range.
        if *self.candidate_start < *self.query_end {
            let gap = self.candidate_start.clone()..self.query_end.clone();
            self.candidate_start = self.query_end;
            return Some(gap);
        }

        None
    }
}

/// An iterator over all stored ranges partially or completely
/// overlapped by a given range.
///
/// The iterator element type is `(&'a Range<K>, &'a V)`.
///
/// This `struct` is created by the [`overlapping`] method on [`RangeMap`]. See its
/// documentation for more.
///
/// [`overlapping`]: RangeMap::overlapping
pub struct Overlapping<'a, K, V, R: Borrow<Range<K>> = &'a Range<K>> {
    query_range: R,
    btm_range_iter: alloc::collections::btree_map::Range<'a, RangeStartWrapper<K>, V>,
}

// `Overlapping` is always fused. (See definition of `next` below.)
impl<'a, K, V, R: Borrow<Range<K>>> core::iter::FusedIterator for Overlapping<'a, K, V, R> where
    K: Ord
{
}

impl<'a, K, V, R: Borrow<Range<K>>> Iterator for Overlapping<'a, K, V, R>
where
    K: Ord,
{
    type Item = (&'a Range<K>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((k, v)) = self.btm_range_iter.next() {
            if k.start < self.query_range.borrow().end {
                Some((&k.range, v))
            } else {
                // The rest of the items in the underlying iterator
                // are past the query range. We can keep taking items
                // from that iterator and this will remain true,
                // so this is enough to make the iterator fused.
                None
            }
        } else {
            None
        }
    }
}

impl<'a, K, V, R: Borrow<Range<K>>> DoubleEndedIterator for Overlapping<'a, K, V, R>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((k, v)) = self.btm_range_iter.next_back() {
            if k.start < self.query_range.borrow().end {
                return Some((&k.range, v));
            }
        }

        None
    }
}

impl<K: Ord + Clone, V: PartialEq + Clone, const N: usize> From<[(Range<K>, V); N]> for RangeMap<K, V> {
    fn from(value: [(Range<K>, V); N]) -> Self {
        let mut map = Self::new();
        for (range, value) in IntoIterator::into_iter(value) {
            map.insert(range, value);
        }
        map
    }
}

/// Create a [`RangeMap`] from key-value pairs.
///
/// # Example
///
/// ```rust
/// # use rangemap::range_map;
/// let map = range_map!{
///     0..100 => "abc",
///     100..200 => "def",
///     200..300 => "ghi"
/// };
/// ```
#[macro_export]
macro_rules! range_map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        $crate::RangeMap::from([$(($k, $v)),*])
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc as std;
    use alloc::{format, string::String, vec, vec::Vec};
    use proptest::prelude::*;
    use test_strategy::proptest;

    impl<K, V> Arbitrary for RangeMap<K, V>
    where
        K: Ord + Clone + Debug + Arbitrary + 'static,
        V: Clone + PartialEq + Arbitrary + 'static,
    {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_parameters: Self::Parameters) -> Self::Strategy {
            any::<Vec<(Range<K>, V)>>()
                .prop_map(|ranges| ranges.into_iter().collect::<RangeMap<K, V>>())
                .boxed()
        }
    }

    #[proptest]
    fn test_first(set: RangeMap<u64, String>) {
        assert_eq!(
            set.first_range_value(),
            set.iter().min_by_key(|(range, _)| range.start)
        );
    }

    #[proptest]
    #[allow(clippy::len_zero)]
    fn test_len(mut map: RangeMap<u64, String>) {
        assert_eq!(map.len(), map.iter().count());
        assert_eq!(map.is_empty(), map.len() == 0);
        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        assert_eq!(map.iter().count(), 0);
    }

    #[proptest]
    fn test_last(set: RangeMap<u64, String>) {
        assert_eq!(
            set.last_range_value(),
            set.iter().max_by_key(|(range, _)| range.end)
        );
    }

    #[proptest]
    fn test_iter_reversible(set: RangeMap<u64, String>) {
        let forward: Vec<_> = set.iter().collect();
        let mut backward: Vec<_> = set.iter().rev().collect();
        backward.reverse();
        assert_eq!(forward, backward);
    }

    #[proptest]
    fn test_into_iter_reversible(set: RangeMap<u64, String>) {
        let forward: Vec<_> = set.clone().into_iter().collect();
        let mut backward: Vec<_> = set.into_iter().rev().collect();
        backward.reverse();
        assert_eq!(forward, backward);
    }

    #[proptest]
    fn test_overlapping_reversible(set: RangeMap<u64, String>, range: Range<u64>) {
        let forward: Vec<_> = set.overlapping(&range).collect();
        let mut backward: Vec<_> = set.overlapping(&range).rev().collect();
        backward.reverse();
        assert_eq!(forward, backward);
    }

    #[proptest]
    fn test_arbitrary_map_u8(ranges: Vec<(Range<u8>, String)>) {
        let ranges: Vec<_> = ranges
            .into_iter()
            .filter(|(range, _value)| range.start != range.end)
            .collect();
        let set = ranges
            .iter()
            .fold(RangeMap::new(), |mut set, (range, value)| {
                set.insert(range.clone(), value.clone());
                set
            });

        for value in 0..u8::MAX {
            assert_eq!(
                set.get(&value),
                ranges
                    .iter()
                    .rev()
                    .find(|(range, _value)| range.contains(&value))
                    .map(|(_range, value)| value)
            );
        }
    }

    #[proptest]
    #[allow(deprecated)]
    fn test_hash(left: RangeMap<u64, u64>, right: RangeMap<u64, u64>) {
        use core::hash::{Hash, Hasher, SipHasher};

        let hash = |set: &RangeMap<_, _>| {
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
    fn test_ord(left: RangeMap<u64, u64>, right: RangeMap<u64, u64>) {
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
        let mut map = RangeMap::new();
        map.insert(0..100, "hello");
        map.insert(200..300, "world");
        assert_eq!(
            map,
            RangeMap::from([(0..100, "hello"), (200..300, "world")])
        );
    }

    #[test]
    fn test_macro() {
        assert_eq!(range_map![], RangeMap::<i64, i64>::default());
        assert_eq!(
            range_map!(0..100 => "abc", 100..200 => "def", 200..300 => "ghi"),
            [(0..100, "abc"), (100..200, "def"), (200..300, "ghi")]
                .iter()
                .cloned()
                .collect(),
        );
    }

    trait RangeMapExt<K, V> {
        fn to_vec(&self) -> Vec<(Range<K>, V)>;
    }

    impl<K, V> RangeMapExt<K, V> for RangeMap<K, V>
    where
        K: Ord + Clone,
        V: PartialEq + Clone,
    {
        fn to_vec(&self) -> Vec<(Range<K>, V)> {
            self.iter().map(|(kr, v)| (kr.clone(), v.clone())).collect()
        }
    }

    //
    // Insertion tests
    //

    #[test]
    fn empty_map_is_empty() {
        let range_map: RangeMap<u32, bool> = RangeMap::new();
        assert_eq!(range_map.to_vec(), vec![]);
    }

    #[test]
    fn insert_into_empty_map() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(0..50, false);
        assert_eq!(range_map.to_vec(), vec![(0..50, false)]);
    }

    #[test]
    fn new_same_value_immediately_following_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    fn new_different_value_immediately_following_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_map.insert(3..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
    }

    #[test]
    fn new_same_value_overlapping_end_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-----◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    fn new_different_value_overlapping_end_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-----◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_map.insert(3..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
    }

    #[test]
    fn new_same_value_immediately_preceding_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    fn new_different_value_immediately_preceding_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        range_map.insert(3..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
    }

    #[test]
    fn new_same_value_wholly_inside_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        range_map.insert(1..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(2..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    fn new_different_value_wholly_inside_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------◇ ◌ ◌ ◌ ◌
        range_map.insert(1..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(2..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌
        // ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌
        assert_eq!(
            range_map.to_vec(),
            vec![(1..2, true), (2..4, false), (4..5, true)]
        );
    }

    #[test]
    fn replace_at_end_of_existing_range_should_coalesce() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
        range_map.insert(3..5, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
    }

    #[test]
    // Test every permutation of a bunch of touching and overlapping ranges.
    fn lots_of_interesting_ranges() {
        use crate::dense::DenseU32RangeMap;
        use permutator::Permutation;

        let mut ranges_with_values = [
            (2..3, false),
            // A duplicate duplicates
            (2..3, false),
            // Almost a duplicate, but with a different value
            (2..3, true),
            // A few small ranges, some of them overlapping others,
            // some of them touching others
            (3..5, true),
            (4..6, true),
            (5..7, true),
            // A really big range
            (2..6, true),
        ];

        ranges_with_values.permutation().for_each(|permutation| {
            let mut range_map: RangeMap<u32, bool> = RangeMap::new();
            let mut dense: DenseU32RangeMap<bool> = DenseU32RangeMap::new();

            for (k, v) in permutation {
                // Insert it into both maps.
                range_map.insert(k.clone(), v);
                // NOTE: Clippy's `range_minus_one` lint is a bit overzealous here,
                // because we _can't_ pass an open-ended range to `insert`.
                #[allow(clippy::range_minus_one)]
                dense.insert(k.start..=(k.end - 1), v);

                // At every step, both maps should contain the same stuff.
                let sparse = range_map.to_vec();
                let dense = dense.to_end_exclusive_vec();
                assert_eq!(sparse, dense);
            }
        });
    }

    //
    // Get* tests
    //

    #[test]
    fn get() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(0..50, false);
        assert_eq!(range_map.get(&49), Some(&false));
        assert_eq!(range_map.get(&50), None);
    }

    #[test]
    fn get_key_value() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(0..50, false);
        assert_eq!(range_map.get_key_value(&49), Some((&(0..50), &false)));
        assert_eq!(range_map.get_key_value(&50), None);
    }

    //
    // Removal tests
    //

    #[test]
    fn remove_from_empty_map() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.remove(0..50);
        assert_eq!(range_map.to_vec(), vec![]);
    }

    #[test]
    fn remove_non_covered_range_before_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(0..25);
        assert_eq!(range_map.to_vec(), vec![(25..75, false)]);
    }

    #[test]
    fn remove_non_covered_range_after_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(75..100);
        assert_eq!(range_map.to_vec(), vec![(25..75, false)]);
    }

    #[test]
    fn remove_overlapping_start_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(0..30);
        assert_eq!(range_map.to_vec(), vec![(30..75, false)]);
    }

    #[test]
    fn remove_middle_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(30..70);
        assert_eq!(range_map.to_vec(), vec![(25..30, false), (70..75, false)]);
    }

    #[test]
    fn remove_overlapping_end_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(70..100);
        assert_eq!(range_map.to_vec(), vec![(25..70, false)]);
    }

    #[test]
    fn remove_exactly_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(25..75);
        assert_eq!(range_map.to_vec(), vec![]);
    }

    #[test]
    fn remove_superset_of_stored() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(25..75, false);
        range_map.remove(0..100);
        assert_eq!(range_map.to_vec(), vec![]);
    }

    // Gaps tests

    #[test]
    fn whole_range_is_a_gap() {
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        let range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let outer_range = 1..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn whole_range_is_covered_exactly() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---------◌ ◌ ◌ ◌
        range_map.insert(1..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---------◇ ◌ ◌ ◌
        let outer_range = 1..6;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_before_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(5..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_touching_start_of_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●-------◌ ◌ ◌ ◌ ◌
        range_map.insert(1..5, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(5..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_overlapping_start_of_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ●---------◌ ◌ ◌ ◌
        range_map.insert(1..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield from the end of the stored item
        // to the end of the outer range.
        assert_eq!(gaps.next(), Some(6..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_starting_at_start_of_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌
        range_map.insert(5..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield from the item onwards.
        assert_eq!(gaps.next(), Some(6..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn items_floating_inside_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌
        range_map.insert(5..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(3..4, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let outer_range = 1..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield gaps at start, between items,
        // and at end.
        assert_eq!(gaps.next(), Some(1..3));
        assert_eq!(gaps.next(), Some(4..5));
        assert_eq!(gaps.next(), Some(6..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_ending_at_end_of_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ●-◌ ◌
        range_map.insert(7..8, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
        let outer_range = 5..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield from the start of the outer range
        // up to the start of the stored item.
        assert_eq!(gaps.next(), Some(5..7));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_overlapping_end_of_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ●---◌ ◌ ◌ ◌
        range_map.insert(4..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌
        let outer_range = 2..5;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield from the start of the outer range
        // up to the start of the stored item.
        assert_eq!(gaps.next(), Some(2..4));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_touching_end_of_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ●-------◌ ◌
        range_map.insert(4..8, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        let outer_range = 1..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..4));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn item_after_outer_range() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ●---◌ ◌
        range_map.insert(6..7, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        let outer_range = 1..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield the entire outer range.
        assert_eq!(gaps.next(), Some(1..4));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_items_away_from_both_sides() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌
        range_map.insert(5..7, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should not yield any gaps, because a zero-width outer range covers no values.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_items_touching_both_sides() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(2..4, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌
        range_map.insert(4..6, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn empty_outer_range_with_item_straddling() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
        range_map.insert(2..5, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
        let outer_range = 4..4;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield no gaps.
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn no_empty_gaps() {
        // Make two ranges different values so they don't
        // get coalesced.
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌
        range_map.insert(4..5, true);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(3..4, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let outer_range = 1..8;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield gaps at start and end, but not between the
        // two touching items. (4 is covered, so there should be no gap.)
        assert_eq!(gaps.next(), Some(1..3));
        assert_eq!(gaps.next(), Some(5..8));
        assert_eq!(gaps.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    #[test]
    fn adjacent_small_items() {
        // Items two items next to each other at the start, and at the end.
        let mut range_map: RangeMap<u8, bool> = RangeMap::new();
        range_map.insert(0..1, false);
        range_map.insert(1..2, true);
        range_map.insert(253..254, false);
        range_map.insert(254..255, true);

        let outer_range = 0..255;
        let mut gaps = range_map.gaps(&outer_range);
        // Should yield one big gap in the middle.
        assert_eq!(gaps.next(), Some(2..253));
        // Gaps iterator should be fused.
        assert_eq!(gaps.next(), None);
        assert_eq!(gaps.next(), None);
    }

    // This test fails in v1.0.2
    #[test]
    fn outer_range_lies_within_first_of_two_stored_ranges() {
        let mut range_map: RangeMap<u64, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◆----------◇ ◌ ◌ ◌ ◌
        range_map.insert(0..5, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌◌ ◌ ◆---◇ ◌
        range_map.insert(6..8, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆--◇ ◌  ◌  ◌  ◌  ◌
        let outer_range: Range<u64> = 1..3;
        let mut gaps = range_map.gaps(&outer_range);
        assert_eq!(gaps.next(), None);
    }

    // Overlapping tests

    #[test]
    fn overlapping_with_empty_map() {
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        let range_map: RangeMap<u32, ()> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆-------------◇ ◌
        let query_range = 1..8;
        let mut overlapping = range_map.overlapping(&query_range);
        // Should not yield any items.
        assert_eq!(overlapping.next(), None);
        // Gaps iterator should be fused.
        assert_eq!(overlapping.next(), None);
    }

    #[test]
    fn overlapping_partial_edges_complete_middle() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();

        // 0 1 2 3 4 5 6 7 8 9
        // ●---◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(0..2, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(3..4, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●---◌ ◌ ◌
        range_map.insert(5..7, ());

        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---------◇ ◌ ◌ ◌
        let query_range = 1..6;

        let mut overlapping = range_map.overlapping(&query_range);

        // Should yield partially overlapped range at start.
        assert_eq!(overlapping.next(), Some((&(0..2), &())));
        // Should yield completely overlapped range in middle.
        assert_eq!(overlapping.next(), Some((&(3..4), &())));
        // Should yield partially overlapped range at end.
        assert_eq!(overlapping.next(), Some((&(5..7), &())));
        // Gaps iterator should be fused.
        assert_eq!(overlapping.next(), None);
        assert_eq!(overlapping.next(), None);
    }

    #[test]
    fn overlapping_non_overlapping_edges_complete_middle() {
        let mut range_map: RangeMap<u32, ()> = RangeMap::new();

        // 0 1 2 3 4 5 6 7 8 9
        // ●---◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(0..2, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(3..4, ());
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ●---◌ ◌ ◌
        range_map.insert(5..7, ());

        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌
        let query_range = 2..5;

        let mut overlapping = range_map.overlapping(&query_range);

        // Should only yield the completely overlapped range in middle.
        // (Not the ranges that are touched by not covered to either side.)
        assert_eq!(overlapping.next(), Some((&(3..4), &())));
        // Gaps iterator should be fused.
        assert_eq!(overlapping.next(), None);
        assert_eq!(overlapping.next(), None);
    }

    ///
    /// impl Debug
    ///

    #[test]
    fn map_debug_repr_looks_right() {
        let mut map: RangeMap<u32, ()> = RangeMap::new();

        // Empty
        assert_eq!(format!("{:?}", map), "{}");

        // One entry
        map.insert(2..5, ());
        assert_eq!(format!("{:?}", map), "{2..5: ()}");

        // Many entries
        map.insert(6..7, ());
        map.insert(8..9, ());
        assert_eq!(format!("{:?}", map), "{2..5: (), 6..7: (), 8..9: ()}");
    }

    // impl Default where T: ?Default

    #[test]
    fn always_default() {
        struct NoDefault;
        RangeMap::<NoDefault, NoDefault>::default();
    }

    // Iterator Tests

    #[test]
    fn into_iter_matches_iter() {
        // Just use vec since that's the same implementation we'd expect
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        range_map.insert(1..3, false);
        range_map.insert(3..5, true);

        let cloned = range_map.to_vec();
        let consumed = range_map.into_iter().collect::<Vec<_>>();

        // Correct value
        assert_eq!(cloned, vec![(1..3, false), (3..5, true)]);

        // Equality
        assert_eq!(cloned, consumed);
    }

    // Equality
    #[test]
    fn eq() {
        let mut a: RangeMap<u32, bool> = RangeMap::new();
        a.insert(1..3, false);

        let mut b: RangeMap<u32, bool> = RangeMap::new();
        b.insert(1..4, false);

        let mut c: RangeMap<u32, bool> = RangeMap::new();
        c.insert(1..3, false);

        assert_ne!(a, b);
        assert_ne!(b, a);

        assert_eq!(a, c);
        assert_eq!(c, a);
        assert_eq!(a, a);
    }

    // Ord
    #[test]
    fn partial_ord() {
        let mut a: RangeMap<u32, bool> = RangeMap::new();
        a.insert(1..3, false);

        let mut b: RangeMap<u32, bool> = RangeMap::new();
        b.insert(1..4, false);

        assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));

        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
    }

    #[test]
    fn ord() {
        let mut a: RangeMap<u32, bool> = RangeMap::new();
        a.insert(1..3, false);

        let mut b: RangeMap<u32, bool> = RangeMap::new();
        b.insert(1..4, false);

        assert_eq!(a.cmp(&a), Ordering::Equal);

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    // impl Serialize

    #[cfg(feature = "serde1")]
    #[test]
    fn serialization() {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
        range_map.insert(1..3, false);
        // 0 1 2 3 4 5 6 7 8 9
        // ◌ ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌
        range_map.insert(5..7, true);
        let output = serde_json::to_string(&range_map).expect("Failed to serialize");
        assert_eq!(output, "[[[1,3],false],[[5,7],true]]");
    }

    // impl Deserialize

    #[cfg(feature = "serde1")]
    #[test]
    fn deserialization() {
        let input = "[[[1,3],false],[[5,7],true]]";
        let range_map: RangeMap<u32, bool> =
            serde_json::from_str(input).expect("Failed to deserialize");
        let reserialized = serde_json::to_string(&range_map).expect("Failed to re-serialize");
        assert_eq!(reserialized, input);
    }

    // const fn

    #[cfg(feature = "const_fn")]
    const _MAP: RangeMap<u32, bool> = RangeMap::new();

    #[cfg(feature = "quickcheck")]
    quickcheck::quickcheck! {
        fn prop(xs: RangeMap<usize, usize>) -> bool {
            xs == xs
        }
    }
}
