//! A fast, efficient, sparse, & ordered unsigned integer (u32) bit set which is invertible.
//!
//! The bitset is implemented using fixed size pages which allows it to compactly
//! represent sparse membership. However, the set excels when set members are typically
//! clustered together. For example when representing glyph id or unicode codepoint values
//! in a font.
//!
//! The set can have inclusive (the set of integers which are members) or
//! exclusive (the set of integers which are not members) membership. The
//! exclusive/inverted version of the set is useful for patterns such as
//! "keep all codepoints except for {x, y, z, ...}".
//!
//! When constructing a new [`IntSet`] from an existing lists of integer values the most efficient
//! way to create the set is to initialize it from a sorted (ascending) iterator of the values.
//!
//! For a type to be stored in the [`IntSet`] it must implement the [`Domain`] trait, and all
//! unique values of that type must be able to be mapped to and from a unique `u32` value.
//! See the [`Domain`] trait for more information.

mod bitpage;
mod bitset;
mod input_bit_stream;
mod output_bit_stream;
pub mod sparse_bit_set;

pub use bitset::U32Set;
use core::ops::{Bound, RangeBounds};
use font_types::{GlyphId, GlyphId16, NameId, Tag};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::RangeInclusive;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

/// A fast & efficient invertible ordered set for small (up to 32-bit) unsigned integer types.
#[derive(Clone)]
pub struct IntSet<T>(Membership, PhantomData<T>);

/// Defines the domain of `IntSet` member types.
///
/// Members of `IntSet` must implement this trait. Members of `IntSet`'s must meet the following
/// conditions to be used in an `IntSet`:
///
/// 1. Every possible unique value of `T` must be able map to and from a unique `u32`
///    integer.
///
/// 2. The mapped `u32` values must retain the same ordering as the values in `T`.
///
/// 3. `ordered_values`() must iterate over all values in `T` in sorted order (ascending).
///
/// `from_u32`() will only ever be called with u32 values that are part of the domain of T as defined
/// by an implementation of this trait. So it doesn't need to correctly handle values
/// that are outside the domain of `T`.
pub trait Domain: Sized + Copy {
    /// Converts this value of `T` to a value in u32.
    ///
    /// The mapped value must maintain the same ordering as `T`.
    fn to_u32(&self) -> u32;

    /// Returns `true` if the value is part of this domain.
    fn contains(value: u32) -> bool;

    /// Converts a mapped u32 value back to T.
    ///
    /// Will only ever be called with values produced by `to_u32`.
    fn from_u32(member: InDomain) -> Self;

    /// Returns true if all u32 values between the mapped u32 min and mapped u32 max value of T are used.
    fn is_continuous() -> bool;

    /// Returns an iterator which iterates over all values in the domain of `T`
    ///
    /// Values should be converted to `u32`'s according to the mapping defined in
    /// `to_u32`/`from_u32`.
    fn ordered_values() -> impl DoubleEndedIterator<Item = u32>;

    /// Return an iterator which iterates over all values of T in the given range.
    ///
    /// Values should be converted to `u32`'s according to the mapping defined in
    /// `to_u32`/`from_u32`.
    fn ordered_values_range(range: RangeInclusive<Self>) -> impl DoubleEndedIterator<Item = u32>;

    /// Returns the number of members in the domain.
    fn count() -> u64;
}

/// Marks a mapped value as being in the domain of `T` for [`Domain`].
///
/// See [`Domain`] for more information.
pub struct InDomain(u32);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum Membership {
    /// Records a set of integers which are members of the set.
    Inclusive(U32Set),

    /// Records the set of integers which are not members of the set.
    Exclusive(U32Set),
}

impl InDomain {
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl<T> Default for IntSet<T> {
    fn default() -> IntSet<T> {
        IntSet::empty()
    }
}

impl<T: Domain> IntSet<T> {
    /// Returns an iterator over all members of the set in sorted ascending order.
    ///
    /// Note: iteration of inverted sets can be extremely slow due to the very large number of members in the set
    /// care should be taken when using `.iter()` in combination with an inverted set.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = T> + '_ {
        let u32_iter = match &self.0 {
            Membership::Inclusive(s) => Iter::new_bidirectional(s.iter(), None),
            Membership::Exclusive(s) => {
                Iter::new_bidirectional(s.iter(), Some(T::ordered_values()))
            }
        };
        u32_iter.map(|v| T::from_u32(InDomain(v)))
    }

    /// If this is an inclusive membership set then returns an iterator over the members, otherwise returns `None`.
    pub fn inclusive_iter(&self) -> Option<impl DoubleEndedIterator<Item = T> + '_> {
        match &self.0 {
            Membership::Inclusive(s) => Some(s.iter().map(|v| T::from_u32(InDomain(v)))),
            Membership::Exclusive(_) => None,
        }
    }

    fn iter_from_u32(&self, value: T) -> impl Iterator<Item = u32> + '_ {
        match &self.0 {
            Membership::Inclusive(s) => Iter::new(s.iter_from(value.to_u32()), None),
            Membership::Exclusive(s) => {
                let value_u32 = value.to_u32();
                let max = T::ordered_values().next_back();
                let it = max.map(|max| T::ordered_values_range(value..=T::from_u32(InDomain(max))));
                let min = it.and_then(|mut it| it.next());

                if let (Some(min), Some(max)) = (min, max) {
                    Iter::new(
                        s.iter_from(value_u32),
                        Some(T::ordered_values_range(
                            T::from_u32(InDomain(min))..=T::from_u32(InDomain(max)),
                        )),
                    )
                } else {
                    // either min or max doesn't exist, so just return an iterator that has no values.
                    let mut it = Iter::new(s.iter_from(u32::MAX), None);
                    it.next();
                    it
                }
            }
        }
    }

    /// Returns an iterator over the members of this set that are after `value` in ascending order.
    ///
    /// Note: iteration of inverted sets can be extremely slow due to the very large number of members in the set
    /// care should be taken when using `.iter()` in combination with an inverted set.
    pub fn iter_after(&self, value: T) -> impl Iterator<Item = T> + '_ {
        self.range((Bound::Excluded(value), Bound::Unbounded))
    }

    /// Returns an iterator over members of this set that are in `range`.
    pub fn range<R: RangeBounds<T>>(&self, range: R) -> impl Iterator<Item = T> + '_ {
        let mut it = match range.start_bound() {
            Bound::Included(start) | Bound::Excluded(start) => {
                self.iter_from_u32(*start).peekable()
            }
            Bound::Unbounded => {
                let min = T::from_u32(InDomain(T::ordered_values().next().unwrap()));
                self.iter_from_u32(min).peekable()
            }
        };

        if let Bound::Excluded(start) = range.start_bound() {
            it.next_if_eq(&start.to_u32());
        }

        let range_end = range.end_bound().cloned();
        it.take_while(move |v| match range_end {
            Bound::Included(end) => *v <= end.to_u32(),
            Bound::Excluded(end) => *v < end.to_u32(),
            Bound::Unbounded => true,
        })
        .map(move |v| T::from_u32(InDomain(v)))
    }

    /// Returns an iterator over all disjoint ranges of values within the set in sorted ascending order.
    pub fn iter_ranges(&self) -> impl Iterator<Item = RangeInclusive<T>> + '_ {
        self.iter_ranges_invertible(false)
    }

    /// Returns an iterator over all disjoint ranges of values not within the set in sorted ascending order.
    pub fn iter_excluded_ranges(&self) -> impl Iterator<Item = RangeInclusive<T>> + '_ {
        self.iter_ranges_invertible(true)
    }

    fn iter_ranges_invertible(
        &self,
        inverted: bool,
    ) -> impl Iterator<Item = RangeInclusive<T>> + '_ {
        let u32_iter = match (&self.0, inverted) {
            (Membership::Inclusive(s), false) | (Membership::Exclusive(s), true)
                if T::is_continuous() =>
            {
                RangeIter::Inclusive::<_, _, T> {
                    ranges: s.iter_ranges(),
                }
            }
            (Membership::Inclusive(s), false) | (Membership::Exclusive(s), true) => {
                RangeIter::InclusiveDiscontinuous::<_, _, T> {
                    ranges: s.iter_ranges(),
                    current_range: None,
                    phantom: PhantomData::<T>,
                }
            }
            (Membership::Exclusive(s), false) | (Membership::Inclusive(s), true)
                if T::is_continuous() =>
            {
                RangeIter::Exclusive::<_, _, T> {
                    ranges: s.iter_ranges(),
                    min: T::ordered_values().next().unwrap(),
                    max: T::ordered_values().next_back().unwrap(),
                    done: false,
                }
            }
            (Membership::Exclusive(s), false) | (Membership::Inclusive(s), true) => {
                RangeIter::ExclusiveDiscontinuous::<_, _, T> {
                    all_values: Some(T::ordered_values()),
                    set: s,
                    next_value: None,
                }
            }
        };

        u32_iter.map(|r| T::from_u32(InDomain(*r.start()))..=T::from_u32(InDomain(*r.end())))
    }

    /// Adds a value to the set.
    ///
    /// Returns `true` if the value was newly inserted.
    pub fn insert(&mut self, val: T) -> bool {
        let val = val.to_u32();
        match &mut self.0 {
            Membership::Inclusive(s) => s.insert(val),
            Membership::Exclusive(s) => s.remove(val),
        }
    }

    /// Add all values in range as members of this set.
    pub fn insert_range(&mut self, range: RangeInclusive<T>) {
        if T::is_continuous() {
            let range = range.start().to_u32()..=range.end().to_u32();
            match &mut self.0 {
                Membership::Inclusive(s) => s.insert_range(range),
                Membership::Exclusive(s) => s.remove_range(range),
            }
        } else {
            let range = T::ordered_values_range(range);
            match &mut self.0 {
                Membership::Inclusive(s) => s.extend(range),
                Membership::Exclusive(s) => s.remove_all(range),
            }
        }
    }

    /// An alternate version of [`extend()`] which is optimized for inserting an unsorted iterator of values.
    ///
    /// [`extend()`]: Self::extend
    pub fn extend_unsorted<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        let iter = iter.into_iter().map(|v| v.to_u32());
        match &mut self.0 {
            Membership::Inclusive(s) => s.extend_unsorted(iter),
            Membership::Exclusive(s) => s.remove_all(iter),
        }
    }

    /// Removes a value from the set. Returns whether the value was present in the set.
    pub fn remove(&mut self, val: T) -> bool {
        let val = val.to_u32();
        match &mut self.0 {
            Membership::Inclusive(s) => s.remove(val),
            Membership::Exclusive(s) => s.insert(val),
        }
    }

    // Removes all values in iter from the set.
    pub fn remove_all<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        let iter = iter.into_iter().map(|v| v.to_u32());
        match &mut self.0 {
            Membership::Inclusive(s) => s.remove_all(iter),
            Membership::Exclusive(s) => s.extend(iter),
        }
    }

    /// Removes all values in range as members of this set.
    pub fn remove_range(&mut self, range: RangeInclusive<T>) {
        if T::is_continuous() {
            let range = range.start().to_u32()..=range.end().to_u32();
            match &mut self.0 {
                Membership::Inclusive(s) => s.remove_range(range),
                Membership::Exclusive(s) => s.insert_range(range),
            }
        } else {
            let range = T::ordered_values_range(range);
            match &mut self.0 {
                Membership::Inclusive(s) => s.remove_all(range),
                Membership::Exclusive(s) => s.extend(range),
            }
        }
    }

    /// Sets the members of this set to the union of self and other.
    pub fn union(&mut self, other: &IntSet<T>) {
        match (&mut self.0, &other.0) {
            (Membership::Inclusive(a), Membership::Inclusive(b)) => a.union(b),
            (Membership::Inclusive(a), Membership::Exclusive(b)) => {
                a.reversed_subtract(b);
                self.invert();
            }
            (Membership::Exclusive(a), Membership::Inclusive(b)) => a.subtract(b),
            (Membership::Exclusive(a), Membership::Exclusive(b)) => a.intersect(b),
        }
    }

    /// Sets the members of this set to the intersection of self and other.
    pub fn intersect(&mut self, other: &IntSet<T>) {
        match (&mut self.0, &other.0) {
            (Membership::Inclusive(a), Membership::Inclusive(b)) => a.intersect(b),
            (Membership::Inclusive(a), Membership::Exclusive(b)) => a.subtract(b),
            (Membership::Exclusive(a), Membership::Inclusive(b)) => {
                a.reversed_subtract(b);
                self.invert();
            }
            (Membership::Exclusive(a), Membership::Exclusive(b)) => a.union(b),
        }
    }

    /// Sets the members of this set to self - other.
    pub fn subtract(&mut self, other: &IntSet<T>) {
        match (&mut self.0, &other.0) {
            (Membership::Inclusive(a), Membership::Inclusive(b)) => a.subtract(b),
            (Membership::Inclusive(a), Membership::Exclusive(b)) => a.intersect(b),
            (Membership::Exclusive(a), Membership::Inclusive(b)) => a.union(b),
            (Membership::Exclusive(a), Membership::Exclusive(b)) => {
                a.reversed_subtract(b);
                self.invert();
            }
        }
    }

    /// Returns true if this set contains at least one element in 'range'.
    pub fn intersects_range(&self, range: RangeInclusive<T>) -> bool {
        self.range(range).next().is_some()
    }

    /// Returns true if this set contains at least one element in 'other'.
    pub fn intersects_set(&self, other: &IntSet<T>) -> bool {
        // Iterate the smaller set and check for member ship in the larger set
        // Estimate the true size as the number of pages.
        let (a, b) = match (&self.0, &other.0) {
            (Membership::Inclusive(us), Membership::Inclusive(them)) => {
                // Can utilize the bitset intersects method.
                return us.intersects_set(them);
            }

            (
                Membership::Inclusive(us) | Membership::Exclusive(us),
                Membership::Inclusive(them) | Membership::Exclusive(them),
            ) => {
                if us.num_pages() > them.num_pages() {
                    (self, other)
                } else {
                    (other, self)
                }
            }
        };

        for range in b.iter_ranges() {
            if a.intersects_range(range) {
                return true;
            }
        }
        false
    }

    /// Returns first element in the set, if any. This element is always the minimum of all elements in the set.
    pub fn first(&self) -> Option<T> {
        return self.iter().next();
    }

    /// Returns the last element in the set, if any. This element is always the maximum of all elements in the set.
    pub fn last(&self) -> Option<T> {
        return self.iter().next_back();
    }

    /// Returns `true` if the set contains a value.
    pub fn contains(&self, val: T) -> bool {
        let val = val.to_u32();
        match &self.0 {
            Membership::Inclusive(s) => s.contains(val),
            Membership::Exclusive(s) => !s.contains(val),
        }
    }

    /// Returns the number of members in this set.
    pub fn len(&self) -> u64 {
        match &self.0 {
            Membership::Inclusive(s) => s.len(),
            Membership::Exclusive(s) => T::count() - s.len(),
        }
    }

    /// Return true if there are no members in this set.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl IntSet<u32> {
    pub(crate) fn from_bitset(set: U32Set) -> IntSet<u32> {
        IntSet(Membership::Inclusive(set), PhantomData::<u32>)
    }
}

impl<T> IntSet<T> {
    /// Create a new, (empty) `IntSet`.
    ///
    /// You can create a new full set with [`IntSet::all`].
    pub const fn new() -> Self {
        Self::empty()
    }

    /// Create a new empty set (inclusive).
    pub const fn empty() -> Self {
        IntSet(Membership::Inclusive(U32Set::empty()), PhantomData::<T>)
    }

    /// Create a new set which contains all integers (exclusive).
    pub const fn all() -> Self {
        IntSet(Membership::Exclusive(U32Set::empty()), PhantomData::<T>)
    }

    /// Returns true if this set is inverted (has exclusive membership).
    pub fn is_inverted(&self) -> bool {
        match &self.0 {
            Membership::Inclusive(_) => false,
            Membership::Exclusive(_) => true,
        }
    }

    /// Return the inverted version of this set.
    pub fn invert(&mut self) {
        let reuse_storage = match &mut self.0 {
            // take the existing storage to reuse in a new set of the opposite
            // type.
            Membership::Inclusive(s) | Membership::Exclusive(s) => {
                std::mem::replace(s, U32Set::empty())
            }
        };

        // reuse the storage with a membership of the opposite type.
        self.0 = match &mut self.0 {
            Membership::Inclusive(_) => Membership::Exclusive(reuse_storage),
            Membership::Exclusive(_) => Membership::Inclusive(reuse_storage),
        };
    }

    /// Clears the set, removing all values.
    pub fn clear(&mut self) {
        let mut reuse_storage = match &mut self.0 {
            // if we're inclusive, we just clear the storage
            Membership::Inclusive(s) => {
                s.clear();
                return;
            }
            // otherwise take the existing storage to reuse in a new
            // inclusive set:
            Membership::Exclusive(s) => std::mem::replace(s, U32Set::empty()),
        };
        // reuse the now empty storage and mark us as inclusive
        reuse_storage.clear();
        self.0 = Membership::Inclusive(reuse_storage);
    }
}

impl<T: Domain> FromIterator<T> for IntSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s = IntSet::empty();
        s.extend(iter);
        s
    }
}

impl<T: Domain> Extend<T> for IntSet<T> {
    /// Extends a collection with the contents of an iterator.
    ///
    /// This implementation is optimized to provide the best performance when the iterator contains sorted values.
    /// Consider using [`extend_unsorted()`] if the iterator is known to contain unsorted values.
    ///
    /// [`extend_unsorted()`]: IntSet::extend_unsorted
    fn extend<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        let iter = iter.into_iter().map(|v| v.to_u32());
        match &mut self.0 {
            Membership::Inclusive(s) => s.extend(iter),
            Membership::Exclusive(s) => s.remove_all(iter),
        }
    }
}

impl<T: Domain> PartialEq for IntSet<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Membership::Inclusive(a), Membership::Inclusive(b)) => a == b,
            (Membership::Exclusive(a), Membership::Exclusive(b)) => a == b,
            (Membership::Inclusive(_), Membership::Exclusive(_))
            | (Membership::Exclusive(_), Membership::Inclusive(_)) => {
                // while these sets have different membership modes, they can still be equal if they happen to have
                // the same effective set of members. In this case fallback to checking via iterator equality.
                // iter_ranges() is used instead of iter() because for exclusive sets it's likely to be significantly
                // faster and have far less items.
                if self.len() == other.len() {
                    let r = self
                        .iter_ranges()
                        .map(|r| r.start().to_u32()..=r.end().to_u32())
                        .eq(other
                            .iter_ranges()
                            .map(|r| r.start().to_u32()..=r.end().to_u32()));
                    r
                } else {
                    // Shortcut iteration equality check if lengths aren't the same.
                    false
                }
            }
        }
    }
}

impl<T: Domain> Hash for IntSet<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Because equality considers two sets with the same effective members (but different membership modes) as
        // equal, hash must be based on the effective member set as well. Exclusive sets may have extremely large numbers
        // of effective members, so here we use iter_ranges() to produce the hash, which should typically produce a more
        // reasonable numbers elements.
        self.iter_ranges()
            .map(|r| r.start().to_u32()..=r.end().to_u32())
            .for_each(|r| r.hash(state));
    }
}

impl<T: Domain + Ord> Ord for IntSet<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (&self.0, &other.0) {
            (Membership::Inclusive(a), Membership::Inclusive(b)) => a.cmp(b),
            _ => {
                let mut this = self
                    .iter_ranges()
                    .map(|r| r.start().to_u32()..=r.end().to_u32());
                let mut other = other
                    .iter_ranges()
                    .map(|r| r.start().to_u32()..=r.end().to_u32());
                loop {
                    match (this.next(), other.next()) {
                        (Some(a), Some(b)) => {
                            let cmp = a.start().cmp(b.start());
                            if cmp != Ordering::Equal {
                                return cmp;
                            }

                            match a.end().cmp(b.end()) {
                                Ordering::Equal => continue,
                                // If a range isn't equal then there are two possible scenarios:
                                // 1. The set with the shorter range has at least one more range.
                                //    In this case the set with the shorter range's next element will always be bigger
                                //    then the other set's next element and should be considered greater.
                                // 2. The set with the shorter range does not have anymore ranges, in that case we
                                //    know the other set has at least one more element and thus should be considered greater.
                                Ordering::Less => {
                                    return if this.next().is_some() {
                                        Ordering::Greater
                                    } else {
                                        Ordering::Less
                                    };
                                }
                                Ordering::Greater => {
                                    return if other.next().is_some() {
                                        Ordering::Less
                                    } else {
                                        Ordering::Greater
                                    };
                                }
                            }
                        }
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                    }
                }
            }
        }
    }
}

impl<T: Domain + Ord> PartialOrd for IntSet<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Domain> Eq for IntSet<T> {}

impl<T: Domain, const N: usize> From<[T; N]> for IntSet<T> {
    fn from(value: [T; N]) -> Self {
        value.into_iter().collect()
    }
}

impl<T: Domain + Debug> Debug for IntSet<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T> Display for IntSet<T>
where
    T: Domain + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ranges = self.iter_ranges().peekable();
        write!(f, "{{ ")?;
        while let Some(range) = ranges.next() {
            write!(f, "{}..={}", range.start(), range.end())?;
            if ranges.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

#[cfg(feature = "serde")]
impl<T: Domain> serde::Serialize for IntSet<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Domain> serde::Deserialize<'de> for IntSet<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let members = Membership::deserialize(deserializer)?;
        let bits = match &members {
            Membership::Inclusive(bit_set) => bit_set,
            Membership::Exclusive(bit_set) => bit_set,
        };

        if let Some(bad) = bits.iter().find(|val| !T::contains(*val)) {
            return Err(serde::de::Error::custom(format!(
                "value '{bad}' out of range for domain {}",
                std::any::type_name::<T>(),
            )));
        }
        Ok(IntSet(members, PhantomData))
    }
}

struct Iter<SetIter, AllValuesIter> {
    set_values: SetIter,
    all_values: Option<AllValuesIter>,
    next_skipped_forward: Option<u32>,
    next_skipped_backward: Option<u32>,
}

impl<SetIter, AllValuesIter> Iter<SetIter, AllValuesIter>
where
    SetIter: Iterator<Item = u32>,
    AllValuesIter: Iterator<Item = u32>,
{
    fn new(
        mut set_values: SetIter,
        all_values: Option<AllValuesIter>,
    ) -> Iter<SetIter, AllValuesIter> {
        match all_values {
            Some(_) => Iter {
                next_skipped_forward: set_values.next(),
                next_skipped_backward: None,
                set_values,
                all_values,
            },
            None => Iter {
                next_skipped_forward: None,
                next_skipped_backward: None,
                set_values,
                all_values,
            },
        }
    }
}

impl<SetIter, AllValuesIter> Iter<SetIter, AllValuesIter>
where
    SetIter: DoubleEndedIterator<Item = u32>,
    AllValuesIter: DoubleEndedIterator<Item = u32>,
{
    fn new_bidirectional(
        mut set_values: SetIter,
        all_values: Option<AllValuesIter>,
    ) -> Iter<SetIter, AllValuesIter> {
        match all_values {
            Some(_) => Iter {
                next_skipped_forward: set_values.next(),
                next_skipped_backward: set_values.next_back(),
                set_values,
                all_values,
            },
            None => Iter {
                set_values,
                all_values,
                next_skipped_forward: None,
                next_skipped_backward: None,
            },
        }
    }
}

impl<SetIter, AllValuesIter> Iterator for Iter<SetIter, AllValuesIter>
where
    SetIter: Iterator<Item = u32>,
    AllValuesIter: Iterator<Item = u32>,
{
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let Some(all_values_it) = &mut self.all_values else {
            return self.set_values.next();
        };

        for index in all_values_it.by_ref() {
            let index = index.to_u32();
            loop {
                let Some(skip) = self.next_skipped_forward else {
                    // There are no skips left in the iterator, but there may still be a skipped
                    // number on the backwards iteration, so check that.
                    if let Some(skip) = self.next_skipped_backward {
                        if skip == index {
                            // this index should be skipped, go to the next one.
                            break;
                        }
                    }
                    // No-longer any values to skip, can freely return index
                    return Some(index);
                };

                if index < skip {
                    // Not a skipped value
                    return Some(index);
                }

                self.next_skipped_forward = self.set_values.next();
                if index > skip {
                    // We've passed the skip value, need to check the next one.
                    continue;
                }

                // index == skip, so we need to skip this index.
                break;
            }
        }
        None
    }
}

impl<SetIter, AllValuesIter> DoubleEndedIterator for Iter<SetIter, AllValuesIter>
where
    SetIter: DoubleEndedIterator<Item = u32>,
    AllValuesIter: DoubleEndedIterator<Item = u32>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(all_values_it) = &mut self.all_values else {
            return self.set_values.next_back();
        };

        for index in all_values_it.by_ref().rev() {
            let index = index.to_u32();
            loop {
                let Some(skip) = self.next_skipped_backward else {
                    // There are no skips left in the iterator, but there may still be a skipped
                    // number on the backwards iteration, so check that.
                    if let Some(skip) = self.next_skipped_forward {
                        if skip == index {
                            // this index should be skipped, go to the next one.
                            break;
                        }
                    }
                    // No-longer any values to skip, can freely return index
                    return Some(index);
                };

                if index > skip {
                    // Not a skipped value
                    return Some(index);
                }

                self.next_skipped_backward = self.set_values.next_back();
                if index < skip {
                    // We've passed the skip value, need to check the next one.
                    continue;
                }

                // index == skip, so we need to skip this index.
                break;
            }
        }
        None
    }
}

enum RangeIter<'a, InclusiveRangeIter, AllValuesIter, T>
where
    InclusiveRangeIter: Iterator<Item = RangeInclusive<u32>>,
    AllValuesIter: Iterator<Item = u32>,
    T: Domain,
{
    Inclusive {
        ranges: InclusiveRangeIter,
    },
    InclusiveDiscontinuous {
        ranges: InclusiveRangeIter,
        current_range: Option<RangeInclusive<u32>>,
        phantom: PhantomData<T>,
    },
    Exclusive {
        ranges: InclusiveRangeIter,
        min: u32,
        max: u32,
        done: bool,
    },
    ExclusiveDiscontinuous {
        all_values: Option<AllValuesIter>,
        set: &'a U32Set,
        next_value: Option<u32>,
    },
}

impl<InclusiveRangeIter, AllValuesIter, T> Iterator
    for RangeIter<'_, InclusiveRangeIter, AllValuesIter, T>
where
    InclusiveRangeIter: Iterator<Item = RangeInclusive<u32>>,
    AllValuesIter: Iterator<Item = u32>,
    T: Domain,
{
    type Item = RangeInclusive<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RangeIter::Inclusive { ranges } => ranges.next(),
            RangeIter::InclusiveDiscontinuous {
                ranges,
                current_range,
                phantom: _,
            } => loop {
                // Discontinuous domains need special handling since members of the domain may be adjacent
                // while their u32 representations may not be. So this iterator implementation compares successive
                // ranges from the underlying u32 range iterator and merges any ranges that are found to be adjacent
                // in the domain of type T.
                let Some(next_range) = ranges.next() else {
                    // No more ranges, commit the one we've got.
                    return current_range.take();
                };

                let Some(range) = current_range.clone() else {
                    // Populate current range, then move to the next so we can check if it's adjacent.
                    *current_range = Some(next_range);
                    continue;
                };

                // Check if next_range can merge into current_range
                if RangeIter::<InclusiveRangeIter, AllValuesIter, T>::are_values_adjacent(
                    *range.end(),
                    *next_range.start(),
                ) {
                    // Do the merge, and check next
                    *current_range = Some(*range.start()..=*next_range.end());
                    continue;
                }

                // No merge is possible, return current range and replace it with next
                *current_range = Some(next_range);
                return Some(range);
            },
            RangeIter::Exclusive {
                ranges,
                min,
                max,
                done,
            } => RangeIter::<InclusiveRangeIter, AllValuesIter, T>::next_exclusive(
                ranges, min, max, done,
            ),
            RangeIter::ExclusiveDiscontinuous {
                all_values,
                set,
                next_value,
            } => RangeIter::<InclusiveRangeIter, AllValuesIter, T>::next_discontinuous(
                all_values, set, next_value,
            ),
        }
    }
}

impl<'a, InclusiveRangeIter, AllValuesIter, T> RangeIter<'a, InclusiveRangeIter, AllValuesIter, T>
where
    InclusiveRangeIter: Iterator<Item = RangeInclusive<u32>>,
    AllValuesIter: Iterator<Item = u32>,
    T: Domain,
{
    /// Iterate the ranges of an exclusive set where the domain is continuous.
    fn next_exclusive(
        ranges: &mut InclusiveRangeIter,
        min: &mut u32,
        max: &mut u32,
        done: &mut bool,
    ) -> Option<RangeInclusive<u32>> {
        if *done {
            return None;
        }

        loop {
            let next_range = ranges.next();

            let Some(next_range) = next_range else {
                *done = true;
                return Some(*min..=*max);
            };

            if next_range.contains(min) {
                if *next_range.end() >= *max {
                    break;
                }
                *min = next_range.end() + 1;
                continue;
            }

            let result = *min..=(next_range.start() - 1);
            if *next_range.end() < *max {
                *min = next_range.end() + 1;
            } else {
                *done = true;
            }
            return Some(result);
        }

        *done = true;
        None
    }

    /// Iterate the ranges of an exclusive set where the domain is discontinuous.
    fn next_discontinuous(
        all_values: &mut Option<AllValuesIter>,
        set: &'a U32Set,
        next_value: &mut Option<u32>,
    ) -> Option<RangeInclusive<u32>> {
        let all_values_iter = all_values.as_mut().unwrap();

        let mut current_range: Option<RangeInclusive<u32>> = None;
        loop {
            let next = next_value.take().or_else(|| all_values_iter.next());
            let Some(next) = next else {
                // No more values, so the current range is over, return it.
                return current_range;
            };

            if set.contains(next) {
                if let Some(range) = current_range {
                    // current range has ended, return it. No need to save 'next' as it's not in the set.
                    return Some(range);
                }
                continue;
            }

            let Some(range) = current_range.as_ref() else {
                current_range = Some(next..=next);
                continue;
            };

            current_range = Some(*range.start()..=next);
        }
    }

    fn are_values_adjacent(a: u32, b: u32) -> bool {
        let mut it = T::ordered_values_range(T::from_u32(InDomain(a))..=T::from_u32(InDomain(b)));
        it.next(); // skip 'a'
        if let Some(second) = it.next() {
            // if a and b are adject then the second value in the iterator should be 'b'
            return second.to_u32() == b.to_u32();
        }
        false
    }
}

impl Domain for u32 {
    fn to_u32(&self) -> u32 {
        *self
    }

    fn from_u32(member: InDomain) -> u32 {
        member.value()
    }

    fn contains(_value: u32) -> bool {
        true
    }

    fn is_continuous() -> bool {
        true
    }

    fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
        u32::MIN..=u32::MAX
    }

    fn ordered_values_range(range: RangeInclusive<u32>) -> impl DoubleEndedIterator<Item = u32> {
        range
    }

    fn count() -> u64 {
        (u32::MAX as u64) - (u32::MIN as u64) + 1
    }
}

impl Domain for u16 {
    fn to_u32(&self) -> u32 {
        *self as u32
    }

    fn from_u32(member: InDomain) -> u16 {
        member.value() as u16
    }

    fn contains(value: u32) -> bool {
        (u16::MIN as u32..=u16::MAX as _).contains(&value)
    }

    fn is_continuous() -> bool {
        true
    }

    fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
        (u16::MIN as u32)..=(u16::MAX as u32)
    }

    fn ordered_values_range(range: RangeInclusive<u16>) -> impl DoubleEndedIterator<Item = u32> {
        (*range.start() as u32)..=(*range.end() as u32)
    }

    fn count() -> u64 {
        (u16::MAX as u64) - (u16::MIN as u64) + 1
    }
}

impl Domain for u8 {
    fn to_u32(&self) -> u32 {
        *self as u32
    }

    fn from_u32(member: InDomain) -> u8 {
        member.value() as u8
    }

    fn contains(value: u32) -> bool {
        (u8::MIN as u32..=u8::MAX as _).contains(&value)
    }

    fn is_continuous() -> bool {
        true
    }

    fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
        (u8::MIN as u32)..=(u8::MAX as u32)
    }

    fn ordered_values_range(range: RangeInclusive<u8>) -> impl DoubleEndedIterator<Item = u32> {
        (*range.start() as u32)..=(*range.end() as u32)
    }

    fn count() -> u64 {
        (u8::MAX as u64) - (u8::MIN as u64) + 1
    }
}

impl Domain for GlyphId16 {
    fn to_u32(&self) -> u32 {
        self.to_u16() as u32
    }

    fn from_u32(member: InDomain) -> GlyphId16 {
        GlyphId16::new(member.value() as u16)
    }

    fn contains(value: u32) -> bool {
        (u16::MIN as u32..=u16::MAX as _).contains(&value)
    }

    fn is_continuous() -> bool {
        true
    }

    fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
        (u16::MIN as u32)..=(u16::MAX as u32)
    }

    fn ordered_values_range(
        range: RangeInclusive<GlyphId16>,
    ) -> impl DoubleEndedIterator<Item = u32> {
        range.start().to_u32()..=range.end().to_u32()
    }

    fn count() -> u64 {
        (u16::MAX as u64) - (u16::MIN as u64) + 1
    }
}

impl Domain for GlyphId {
    fn to_u32(&self) -> u32 {
        GlyphId::to_u32(*self)
    }

    fn from_u32(member: InDomain) -> GlyphId {
        GlyphId::from(member.value())
    }

    fn contains(_value: u32) -> bool {
        true
    }

    fn is_continuous() -> bool {
        true
    }

    fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
        u32::MIN..=u32::MAX
    }

    fn ordered_values_range(
        range: RangeInclusive<GlyphId>,
    ) -> impl DoubleEndedIterator<Item = u32> {
        range.start().to_u32()..=range.end().to_u32()
    }

    fn count() -> u64 {
        (u32::MAX as u64) - (u32::MIN as u64) + 1
    }
}

impl Domain for Tag {
    fn to_u32(&self) -> u32 {
        u32::from_be_bytes(self.to_be_bytes())
    }

    fn from_u32(member: InDomain) -> Tag {
        Tag::from_u32(member.value())
    }

    fn contains(_value: u32) -> bool {
        true
    }

    fn is_continuous() -> bool {
        true
    }

    fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
        u32::MIN..=u32::MAX
    }

    fn ordered_values_range(range: RangeInclusive<Tag>) -> impl DoubleEndedIterator<Item = u32> {
        range.start().to_u32()..=range.end().to_u32()
    }

    fn count() -> u64 {
        (u32::MAX as u64) - (u32::MIN as u64) + 1
    }
}

impl Domain for NameId {
    fn to_u32(&self) -> u32 {
        self.to_u16() as u32
    }

    fn from_u32(member: InDomain) -> NameId {
        NameId::new(member.value() as u16)
    }

    fn contains(value: u32) -> bool {
        (u16::MIN as u32..=u16::MAX as u32).contains(&value)
    }

    fn is_continuous() -> bool {
        true
    }

    fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
        (u16::MIN as u32)..=(u16::MAX as u32)
    }

    fn ordered_values_range(range: RangeInclusive<NameId>) -> impl DoubleEndedIterator<Item = u32> {
        (range.start().to_u16() as u32)..=(range.end().to_u16() as u32)
    }

    fn count() -> u64 {
        (u16::MAX as u64) - (u16::MIN as u64) + 1
    }
}

#[cfg(test)]
mod test {
    use core::cmp::Ordering;
    use std::{collections::HashSet, hash::Hash};

    use super::*;

    #[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy)]
    struct EvenInts(u16);

    impl Domain for EvenInts {
        fn to_u32(&self) -> u32 {
            self.0 as u32
        }

        fn contains(value: u32) -> bool {
            (value % 2) == 0
        }

        fn from_u32(member: InDomain) -> EvenInts {
            EvenInts(member.0 as u16)
        }

        fn is_continuous() -> bool {
            false
        }

        fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
            (u16::MIN..=u16::MAX)
                .filter(|v| v % 2 == 0)
                .map(|v| v as u32)
        }

        fn ordered_values_range(
            range: RangeInclusive<EvenInts>,
        ) -> impl DoubleEndedIterator<Item = u32> {
            Self::ordered_values()
                .filter(move |v| *v >= range.start().to_u32() && *v <= range.end().to_u32())
        }

        fn count() -> u64 {
            ((u32::MAX as u64) - (u32::MIN as u64)).div_ceil(2)
        }
    }

    #[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone, Copy)]
    struct TwoParts(u16);

    impl Domain for TwoParts {
        fn to_u32(&self) -> u32 {
            self.0 as u32
        }

        fn contains(value: u32) -> bool {
            (2..=5).contains(&value) || (8..=16).contains(&value)
        }

        fn from_u32(member: InDomain) -> TwoParts {
            TwoParts(member.0 as u16)
        }

        fn is_continuous() -> bool {
            false
        }

        fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
            [2..=5, 8..=16].into_iter().flat_map(|it| it.into_iter())
        }

        fn ordered_values_range(
            range: RangeInclusive<TwoParts>,
        ) -> impl DoubleEndedIterator<Item = u32> {
            Self::ordered_values()
                .filter(move |v| *v >= range.start().to_u32() && *v <= range.end().to_u32())
        }

        fn count() -> u64 {
            4 + 9
        }
    }

    #[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy)]
    struct TwoPartsBounds(u32);

    impl Domain for TwoPartsBounds {
        fn to_u32(&self) -> u32 {
            self.0
        }

        fn contains(value: u32) -> bool {
            (0..=1).contains(&value) || (u32::MAX - 1..=u32::MAX).contains(&value)
        }

        fn from_u32(member: InDomain) -> TwoPartsBounds {
            TwoPartsBounds(member.0)
        }

        fn is_continuous() -> bool {
            false
        }

        fn ordered_values() -> impl DoubleEndedIterator<Item = u32> {
            [0..=1, u32::MAX - 1..=u32::MAX]
                .into_iter()
                .flat_map(|it| it.into_iter())
        }

        fn ordered_values_range(
            range: RangeInclusive<TwoPartsBounds>,
        ) -> impl DoubleEndedIterator<Item = u32> {
            Self::ordered_values()
                .filter(move |v| *v >= range.start().to_u32() && *v <= range.end().to_u32())
        }

        fn count() -> u64 {
            4
        }
    }

    #[test]
    fn from_sparse_set() {
        let bytes = [0b00001101, 0b00000011, 0b00110001];

        let set = IntSet::<u32>::from_sparse_bit_set(&bytes).unwrap();

        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(0..=17);

        assert_eq!(set, expected);
    }

    #[test]
    fn insert() {
        let mut empty = IntSet::<u32>::empty();
        let mut all = IntSet::<u32>::all();

        assert!(!empty.contains(10));
        assert!(empty.insert(10));
        assert!(empty.contains(10));
        assert!(!empty.insert(10));

        assert!(all.contains(10));
        assert!(!all.insert(10));
        assert!(all.contains(10));
        assert!(!all.insert(10));
    }

    #[test]
    fn remove() {
        let mut empty = IntSet::<u32>::empty();
        empty.insert(10);
        let mut all = IntSet::<u32>::all();

        assert!(empty.contains(10));
        assert!(empty.remove(10));
        assert!(!empty.contains(10));
        assert!(!empty.remove(10));

        assert!(all.contains(10));
        assert!(all.remove(10));
        assert!(!all.contains(10));
        assert!(!all.remove(10));
    }

    #[test]
    fn is_empty() {
        let mut set = IntSet::<u32>::empty();

        assert!(set.is_empty());
        set.insert(13);
        set.insert(800);
        assert!(!set.is_empty());

        set.invert();
        assert!(!set.is_empty());

        let mut empty = IntSet::<u32>::empty();
        assert!(empty.is_empty());
        empty.invert();
        assert!(!empty.is_empty());
    }

    #[test]
    fn first() {
        let set = IntSet::<u16>::empty();
        assert_eq!(set.first(), None);

        let set = IntSet::<u16>::all();
        assert_eq!(set.first(), Some(0));

        let mut set = IntSet::<u16>::empty();
        set.extend([0]);
        assert_eq!(set.first(), Some(0));

        let mut set = IntSet::<u16>::empty();
        set.extend([u16::MAX]);
        assert_eq!(set.first(), Some(u16::MAX));

        let mut set = IntSet::<u16>::empty();
        set.extend([100, 1000, 10000]);
        assert_eq!(set.first(), Some(100));

        set.invert();
        assert_eq!(set.first(), Some(0));

        set.remove_range(0..=100);
        assert_eq!(set.first(), Some(101));
    }

    #[test]
    fn last() {
        let set = IntSet::<u16>::empty();
        assert_eq!(set.last(), None);

        let set = IntSet::<u16>::all();
        assert_eq!(set.last(), Some(u16::MAX));

        let mut set = IntSet::<u16>::empty();
        set.extend([0]);
        assert_eq!(set.last(), Some(0));

        let mut set = IntSet::<u16>::empty();
        set.extend([u16::MAX]);
        assert_eq!(set.last(), Some(u16::MAX));

        let mut set = IntSet::<u16>::empty();
        set.extend([5, 7, 8]);
        assert_eq!(set.last(), Some(8));

        let mut set = IntSet::<u16>::empty();
        set.extend([100, 1000, 10000]);
        assert_eq!(set.last(), Some(10000));

        set.invert();
        assert_eq!(set.last(), Some(u16::MAX));

        set.remove_range(u16::MAX - 10..=u16::MAX);
        assert_eq!(set.last(), Some(u16::MAX - 11));
    }

    #[test]
    fn clear() {
        let mut set = IntSet::<u32>::empty();
        set.insert(13);
        set.insert(800);

        let mut set_inverted = IntSet::<u32>::empty();
        set_inverted.insert(13);
        set_inverted.insert(800);
        set_inverted.invert();

        set.clear();
        assert!(set.is_empty());
        set_inverted.clear();
        assert!(set_inverted.is_empty());
    }

    #[allow(deprecated)] // SipHasher required because of MSRV
    fn hash<T>(set: &IntSet<T>) -> u64
    where
        T: Domain,
    {
        use std::hash::Hasher;
        let mut h = std::hash::SipHasher::new();
        set.hash(&mut h);
        h.finish()
    }

    #[test]
    fn equal_and_hash() {
        let mut inc1 = IntSet::<u32>::empty();
        inc1.insert(14);
        inc1.insert(670);

        let mut inc2 = IntSet::<u32>::empty();
        inc2.insert(670);
        inc2.insert(14);

        let mut inc3 = inc1.clone();
        inc3.insert(5);

        let mut exc = inc1.clone();
        exc.invert();

        assert_eq!(inc1, inc2);
        assert_ne!(inc1, inc3);
        assert_ne!(inc1, exc);

        let set = HashSet::from([inc1.clone(), inc3.clone(), exc.clone()]);
        assert!(set.contains(&inc1));
        assert!(set.contains(&inc3));
        assert!(set.contains(&exc));

        assert_ne!(hash(&inc1), hash(&exc));
        assert_eq!(hash(&inc1), hash(&inc2));
    }

    #[test]
    fn equal_and_hash_mixed_membership_types() {
        let mut inverted_all = IntSet::<TwoParts>::all();
        let mut all = IntSet::<TwoParts>::empty();
        for v in TwoParts::ordered_values() {
            all.insert(TwoParts(v as u16));
        }

        assert_eq!(inverted_all, all);
        assert_eq!(hash(&all), hash(&inverted_all));

        inverted_all.remove(TwoParts(5));
        assert_ne!(inverted_all, all);

        all.remove(TwoParts(5));
        assert_eq!(inverted_all, all);
        assert_eq!(hash(&all), hash(&inverted_all));
    }

    #[test]
    fn iter() {
        let mut set = IntSet::<u32>::empty();
        set.insert(3);
        set.insert(8);
        set.insert(534);
        set.insert(700);
        set.insert(10000);
        set.insert(10001);
        set.insert(10002);

        let v: Vec<u32> = set.iter().collect();
        assert_eq!(v, vec![3, 8, 534, 700, 10000, 10001, 10002]);

        let v: Vec<u32> = set.inclusive_iter().unwrap().collect();
        assert_eq!(v, vec![3, 8, 534, 700, 10000, 10001, 10002]);
    }

    #[test]
    fn iter_backwards() {
        let mut set = IntSet::<u32>::empty();
        set.insert_range(1..=6);
        {
            let mut it = set.iter();
            assert_eq!(Some(1), it.next());
            assert_eq!(Some(6), it.next_back());
            assert_eq!(Some(5), it.next_back());
            assert_eq!(Some(2), it.next());
            assert_eq!(Some(3), it.next());
            assert_eq!(Some(4), it.next());
            assert_eq!(None, it.next());
            assert_eq!(None, it.next_back());
        }

        let mut set = IntSet::<u8>::empty();
        set.invert();
        set.remove_range(10..=255);
        set.remove(4);
        set.remove(8);
        {
            let mut it = set.iter();
            assert_eq!(Some(0), it.next());
            assert_eq!(Some(1), it.next());
            assert_eq!(Some(2), it.next());
            assert_eq!(Some(3), it.next());

            assert_eq!(Some(9), it.next_back());
            assert_eq!(Some(7), it.next_back());
            assert_eq!(Some(6), it.next_back());
            assert_eq!(Some(5), it.next_back());
            assert_eq!(None, it.next_back());

            assert_eq!(None, it.next());
        }

        let mut set = IntSet::<u8>::empty();
        set.invert();
        set.remove_range(10..=255);
        set.remove(4);
        set.remove(8);
        {
            let mut it = set.iter();
            assert_eq!(Some(0), it.next());
            assert_eq!(Some(1), it.next());
            assert_eq!(Some(2), it.next());
            assert_eq!(Some(3), it.next());
            assert_eq!(Some(5), it.next());

            assert_eq!(Some(9), it.next_back());
            assert_eq!(Some(7), it.next_back());
            assert_eq!(Some(6), it.next_back());
            assert_eq!(None, it.next_back());

            assert_eq!(None, it.next());
        }
    }

    #[test]
    fn exclusive_iter() {
        let mut set = IntSet::<u32>::all();
        set.remove(3);
        set.remove(7);
        set.remove(8);

        let mut iter = set.iter();

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(10));

        assert!(set.inclusive_iter().is_none());

        // Forward skip first
        let mut set = IntSet::<u32>::all();
        set.remove_range(0..=200);

        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(201));

        // Backward skip first
        let mut set = IntSet::<u8>::all();
        set.remove_range(200..=255);

        let mut iter = set.iter();
        assert_eq!(iter.next_back(), Some(199));
    }

    #[test]
    fn iter_ranges_inclusive() {
        let mut set = IntSet::<u32>::empty();
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![]);

        set.insert_range(200..=700);
        set.insert(5);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![5..=5, 200..=700]);

        let mut set = IntSet::<u32>::empty();
        set.insert_range(0..=0);
        set.insert_range(u32::MAX..=u32::MAX);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![0..=0, u32::MAX..=u32::MAX]);

        let mut set = IntSet::<u32>::empty();
        set.insert_range(0..=5);
        set.insert_range(u32::MAX - 5..=u32::MAX);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![0..=5, u32::MAX - 5..=u32::MAX]);

        let mut inverted = set.clone();
        inverted.invert();
        assert_eq!(
            set.iter_ranges().collect::<Vec<_>>(),
            inverted.iter_excluded_ranges().collect::<Vec<_>>()
        );
    }

    #[test]
    fn iter_ranges_inclusive_discontinuous() {
        let mut set = IntSet::<EvenInts>::empty();
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![]);

        set.insert_range(EvenInts(4)..=EvenInts(12));
        set.insert(EvenInts(16));

        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(
            items,
            vec![EvenInts(4)..=EvenInts(12), EvenInts(16)..=EvenInts(16)]
        );

        let mut inverted = set.clone();
        inverted.invert();
        assert_eq!(
            set.iter_ranges().collect::<Vec<_>>(),
            inverted.iter_excluded_ranges().collect::<Vec<_>>()
        );
    }

    #[test]
    fn iter_ranges_exclusive() {
        let mut set = IntSet::<u32>::all();
        set.remove_range(200..=700);
        set.remove(5);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![0..=4, 6..=199, 701..=u32::MAX]);

        let mut set = IntSet::<u32>::all();
        set.remove_range(0..=700);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![701..=u32::MAX]);

        let mut inverted = set.clone();
        inverted.invert();
        assert_eq!(
            set.iter_ranges().collect::<Vec<_>>(),
            inverted.iter_excluded_ranges().collect::<Vec<_>>()
        );

        let mut set = IntSet::<u32>::all();
        set.remove_range(u32::MAX - 10..=u32::MAX);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![0..=u32::MAX - 11]);

        let mut set = IntSet::<u16>::all();
        set.remove_range(0..=u16::MAX);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![]);

        let mut set = IntSet::<u16>::all();
        set.remove_range(0..=u16::MAX - 1);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![u16::MAX..=u16::MAX]);

        let mut set = IntSet::<u16>::all();
        set.remove_range(1..=u16::MAX);
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![0..=0]);

        let set = IntSet::<u32>::all();
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![0..=u32::MAX]);
    }

    #[test]
    fn iter_ranges_exclusive_discontinuous() {
        let mut set = IntSet::<EvenInts>::all();
        set.remove_range(EvenInts(0)..=EvenInts(8));
        set.remove_range(EvenInts(16)..=EvenInts(u16::MAX - 1));
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![EvenInts(10)..=EvenInts(14),]);

        let mut set = IntSet::<TwoParts>::all();
        set.remove_range(TwoParts(11)..=TwoParts(13));
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(
            items,
            vec![TwoParts(2)..=TwoParts(10), TwoParts(14)..=TwoParts(16),]
        );

        let mut inverted = set.clone();
        inverted.invert();
        assert_eq!(
            set.iter_ranges().collect::<Vec<_>>(),
            inverted.iter_excluded_ranges().collect::<Vec<_>>()
        );

        let mut set = IntSet::<TwoParts>::all();
        set.remove_range(TwoParts(2)..=TwoParts(16));
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![]);

        let mut set = IntSet::<TwoParts>::all();
        set.remove_range(TwoParts(2)..=TwoParts(5));
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![TwoParts(8)..=TwoParts(16),]);

        let mut set = IntSet::<TwoParts>::all();
        set.remove_range(TwoParts(6)..=TwoParts(16));
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![TwoParts(2)..=TwoParts(5),]);

        // Check we can safely iterate to the limits of u32.
        let set = IntSet::<TwoPartsBounds>::all();
        let items: Vec<_> = set.iter_ranges().collect();
        assert_eq!(items, vec![TwoPartsBounds(0)..=TwoPartsBounds(u32::MAX),]);
    }

    #[test]
    fn iter_range() {
        let mut set = IntSet::<u32>::empty();
        assert_eq!(set.iter_after(0).count(), 0);

        set.extend([5, 7, 10, 1250, 1300, 3001]);

        assert_eq!(
            set.iter_after(0).collect::<Vec<u32>>(),
            vec![5, 7, 10, 1250, 1300, 3001]
        );
        assert_eq!(
            set.range(0..).collect::<Vec<u32>>(),
            vec![5, 7, 10, 1250, 1300, 3001]
        );

        assert_eq!(
            set.iter_after(5).collect::<Vec<u32>>(),
            vec![7, 10, 1250, 1300, 3001]
        );
        assert_eq!(
            set.range(5..).collect::<Vec<u32>>(),
            vec![5, 7, 10, 1250, 1300, 3001]
        );

        assert_eq!(
            set.iter_after(700).collect::<Vec<u32>>(),
            vec![1250, 1300, 3001]
        );
        assert_eq!(
            set.range(700..).collect::<Vec<u32>>(),
            vec![1250, 1300, 3001]
        );
    }

    #[test]
    fn iter_after_from_exclusive() {
        let mut set = IntSet::<u32>::empty();
        set.extend([5, 7, 10, 1250, 1300, 3001]);
        set.invert();

        assert_eq!(
            set.iter_after(3).take(5).collect::<Vec<u32>>(),
            vec![4, 6, 8, 9, 11]
        );
        assert_eq!(
            set.range(3..).take(5).collect::<Vec<u32>>(),
            vec![3, 4, 6, 8, 9]
        );

        assert_eq!(
            set.iter_after(0).take(5).collect::<Vec<u32>>(),
            vec![1, 2, 3, 4, 6]
        );
        assert_eq!(
            set.range(0..).take(5).collect::<Vec<u32>>(),
            vec![0, 1, 2, 3, 4]
        );

        assert_eq!(
            set.iter_after(u32::MAX - 1).take(1).collect::<Vec<u32>>(),
            vec![u32::MAX]
        );
        assert_eq!(
            set.range(u32::MAX - 1..).take(2).collect::<Vec<u32>>(),
            vec![u32::MAX - 1, u32::MAX]
        );

        assert_eq!(set.iter_after(u32::MAX).take(1).count(), 0);
        set.remove(u32::MAX);
        assert_eq!(set.range(u32::MAX..).take(1).count(), 0);
        assert_eq!(set.iter_after(u32::MAX - 1).take(1).count(), 0);
    }

    #[test]
    fn iter_after_discontinuous() {
        let mut set = IntSet::<EvenInts>::empty();
        set.extend([EvenInts(6), EvenInts(10)]);
        set.invert();

        assert_eq!(
            set.iter_after(EvenInts(2))
                .take(5)
                .collect::<Vec<EvenInts>>(),
            vec![
                EvenInts(4),
                EvenInts(8),
                EvenInts(12),
                EvenInts(14),
                EvenInts(16)
            ]
        );
        assert_eq!(
            set.range(EvenInts(2)..).take(5).collect::<Vec<EvenInts>>(),
            vec![
                EvenInts(2),
                EvenInts(4),
                EvenInts(8),
                EvenInts(12),
                EvenInts(14)
            ]
        );

        assert_eq!(
            set.iter_after(EvenInts(4))
                .take(5)
                .collect::<Vec<EvenInts>>(),
            vec![
                EvenInts(8),
                EvenInts(12),
                EvenInts(14),
                EvenInts(16),
                EvenInts(18)
            ]
        );

        assert_eq!(
            set.iter_after(EvenInts(u16::MAX - 1))
                .collect::<Vec<EvenInts>>(),
            vec![]
        );
        assert_eq!(
            set.range(EvenInts(u16::MAX - 1)..)
                .collect::<Vec<EvenInts>>(),
            vec![EvenInts(u16::MAX - 1)]
        );

        assert_eq!(
            set.iter_after(EvenInts(u16::MAX - 5))
                .collect::<Vec<EvenInts>>(),
            vec![EvenInts(u16::MAX - 3), EvenInts(u16::MAX - 1)]
        );

        set.remove(EvenInts(u16::MAX - 1));
        assert_eq!(
            set.iter_after(EvenInts(u16::MAX - 5))
                .collect::<Vec<EvenInts>>(),
            vec![EvenInts(u16::MAX - 3),]
        );
        assert_eq!(
            set.range(EvenInts(u16::MAX - 5)..)
                .collect::<Vec<EvenInts>>(),
            vec![EvenInts(u16::MAX - 5), EvenInts(u16::MAX - 3),]
        );
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn range() {
        let mut set = IntSet::<u32>::empty();
        assert_eq!(set.range(0..=5).count(), 0);

        set.extend([5, 7, 10, 1250, 1300, 3001]);

        assert_eq!(set.range(0..=5).collect::<Vec<u32>>(), vec![5]);
        assert_eq!(set.range(5..=11).collect::<Vec<u32>>(), vec![5, 7, 10]);
        assert_eq!(set.range(5..10).collect::<Vec<u32>>(), vec![5, 7]);
        assert_eq!(set.range(..10).collect::<Vec<u32>>(), vec![5, 7]);
        assert_eq!(set.range(..=10).collect::<Vec<u32>>(), vec![5, 7, 10]);
        assert_eq!(set.range(6..=11).collect::<Vec<u32>>(), vec![7, 10]);

        assert!(set.range(7..6).collect::<Vec<u32>>().is_empty());
        assert!(set.range(7..7).collect::<Vec<u32>>().is_empty());
        assert_eq!(set.range(7..=7).collect::<Vec<u32>>(), vec![7]);

        assert!(set.range(5..=0).collect::<Vec<u32>>().is_empty());
    }

    #[test]
    fn from_iterator() {
        let s: IntSet<u32> = [3, 8, 12, 589].into_iter().collect();
        let mut expected = IntSet::<u32>::empty();
        expected.insert(3);
        expected.insert(8);
        expected.insert(12);
        expected.insert(589);

        assert_eq!(s, expected);
    }

    #[test]
    fn from_int_set_iterator() {
        let s1: IntSet<u32> = [3, 8, 12, 589].into_iter().collect();
        let s2: IntSet<u32> = s1.iter().collect();
        assert_eq!(s1, s2);
    }

    #[test]
    fn extend() {
        let mut s = IntSet::<u32>::empty();
        s.extend([3, 12]);
        s.extend([8, 10, 589]);

        let mut expected = IntSet::<u32>::empty();
        expected.insert(3);
        expected.insert(8);
        expected.insert(10);
        expected.insert(12);
        expected.insert(589);

        assert_eq!(s, expected);
    }

    #[test]
    fn extend_on_inverted() {
        let mut s = IntSet::<u32>::all();
        for i in 10..=20 {
            s.remove(i);
        }

        s.extend([12, 17, 18]);

        assert!(!s.contains(11));
        assert!(s.contains(12));
        assert!(!s.contains(13));

        assert!(!s.contains(16));
        assert!(s.contains(17));
        assert!(s.contains(18));
        assert!(!s.contains(19));
        assert!(s.contains(100));
    }

    #[test]
    fn remove_all() {
        let mut empty = IntSet::<u32>::empty();
        let mut all = IntSet::<u32>::all();

        empty.extend([1, 2, 3, 4]);

        empty.remove_all([2, 3]);
        all.remove_all([2, 3]);

        assert!(empty.contains(1));
        assert!(!empty.contains(2));
        assert!(!empty.contains(3));
        assert!(empty.contains(4));

        assert!(all.contains(1));
        assert!(!all.contains(2));
        assert!(!all.contains(3));
        assert!(all.contains(4));
    }

    #[test]
    fn remove_range() {
        let mut empty = IntSet::<u32>::empty();
        let mut all = IntSet::<u32>::all();

        empty.extend([1, 2, 3, 4]);

        empty.remove_range(2..=3);
        all.remove_range(2..=3);

        assert!(empty.contains(1));
        assert!(!empty.contains(2));
        assert!(!empty.contains(3));
        assert!(empty.contains(4));

        assert!(all.contains(1));
        assert!(!all.contains(2));
        assert!(!all.contains(3));
        assert!(all.contains(4));
    }

    #[test]
    fn insert_remove_range_boundary() {
        let mut set = IntSet::<u32>::empty();

        set.remove_range(u32::MAX - 10..=u32::MAX);
        assert!(!set.contains(u32::MAX));
        set.insert_range(u32::MAX - 10..=u32::MAX);
        assert!(set.contains(u32::MAX));
        set.remove_range(u32::MAX - 10..=u32::MAX);
        assert!(!set.contains(u32::MAX));

        set.remove_range(0..=10);
        assert!(!set.contains(0));
        set.insert_range(0..=10);
        assert!(set.contains(0));
        set.remove_range(0..=10);
        assert!(!set.contains(0));
    }

    #[test]
    fn insert_remove_range_exclusive_boundary() {
        let mut set = IntSet::<u32>::all();

        set.remove_range(u32::MAX - 10..=u32::MAX);
        assert!(!set.contains(u32::MAX));
        set.insert_range(u32::MAX - 10..=u32::MAX);
        assert!(set.contains(u32::MAX));
        set.remove_range(u32::MAX - 10..=u32::MAX);
        assert!(!set.contains(u32::MAX));

        set.remove_range(0..=10);
        assert!(!set.contains(0));
        set.insert_range(0..=10);
        assert!(set.contains(0));
        set.remove_range(0..=10);
        assert!(!set.contains(0));
    }

    struct SetOpInput {
        has_x: bool,
        inverted: bool,
        has_page: bool,
    }

    impl SetOpInput {
        fn get_all_inputs() -> Vec<SetOpInput> {
            let mut result: Vec<SetOpInput> = vec![];
            for has_x in [true, false] {
                for inverted in [true, false] {
                    result.push(SetOpInput {
                        has_x,
                        inverted,
                        has_page: false,
                    });
                    let can_have_empty_page = has_x == inverted;
                    if can_have_empty_page {
                        result.push(SetOpInput {
                            has_x,
                            inverted,
                            has_page: true,
                        });
                    }
                }
            }
            result
        }

        fn to_set(&self, x: u32) -> IntSet<u32> {
            let mut s = IntSet::<u32>::empty();
            if self.inverted {
                s.invert();
            }
            if self.has_page {
                // Ensure a page exists for x.
                if self.inverted {
                    s.remove(x);
                } else {
                    s.insert(x);
                }
            }
            if self.has_x {
                s.insert(x);
            } else {
                s.remove(x);
            }
            s
        }
    }

    fn set_operation_test_message(
        a: &SetOpInput,
        b: &SetOpInput,
        op_name: &str,
        should_contain_x: bool,
    ) -> String {
        format!(
            "{}{}{} {} {}{}{} failed. {}",
            if a.inverted { "i" } else { "" },
            if a.has_page { "p" } else { "" },
            if a.has_x { "13" } else { "" },
            op_name,
            if b.inverted { "i" } else { "" },
            if b.has_page { "p" } else { "" },
            if b.has_x { "13" } else { "" },
            if should_contain_x {
                "Result did not have 13."
            } else {
                "Result should not have 13."
            }
        )
    }

    fn check_union(a: &SetOpInput, b: &SetOpInput) {
        let x = 13;
        let mut set_a = a.to_set(x);
        let set_b = b.to_set(x);

        let should_contain_x = a.has_x || b.has_x;
        set_a.union(&set_b);

        assert_eq!(
            set_a.contains(x),
            should_contain_x,
            "{}",
            set_operation_test_message(a, b, "union", should_contain_x)
        );
    }

    fn check_intersect(a: &SetOpInput, b: &SetOpInput) {
        let x = 13;
        let mut set_a = a.to_set(x);
        let set_b = b.to_set(x);

        let should_contain_x = a.has_x && b.has_x;
        set_a.intersect(&set_b);

        assert_eq!(
            set_a.contains(x),
            should_contain_x,
            "{}",
            set_operation_test_message(a, b, "intersect", should_contain_x)
        );
    }

    fn check_subtract(a: &SetOpInput, b: &SetOpInput) {
        let x = 13;
        let mut set_a = a.to_set(x);
        let set_b = b.to_set(x);

        let should_contain_x = a.has_x && (!b.has_x);
        set_a.subtract(&set_b);

        assert_eq!(
            set_a.contains(x),
            should_contain_x,
            "{}",
            set_operation_test_message(a, b, "subtract", should_contain_x)
        );
    }

    #[test]
    fn set_operations() {
        for a in SetOpInput::get_all_inputs() {
            for b in SetOpInput::get_all_inputs() {
                check_union(&a, &b);
                check_intersect(&a, &b);
                check_subtract(&a, &b);
            }
        }
    }

    #[test]
    fn inverted() {
        let mut set = IntSet::<u32>::empty();

        set.insert(13);
        set.insert(800);
        assert!(set.contains(13));
        assert!(set.contains(800));
        assert_eq!(set.len(), 2);
        assert!(!set.is_inverted());

        set.invert();
        assert_eq!(set.len(), u32::MAX as u64 - 1);
        assert!(!set.contains(13));
        assert!(set.contains(80));
        assert!(!set.contains(800));
        assert!(set.is_inverted());

        set.remove(80);
        assert!(!set.contains(80));

        set.insert(13);
        assert!(set.contains(13));

        set.invert();
        assert!(set.contains(80));
        assert!(set.contains(800));
    }

    #[test]
    fn limited_domain_type() {
        let mut set = IntSet::<EvenInts>::empty();

        set.insert(EvenInts(2));
        set.insert(EvenInts(8));
        set.insert(EvenInts(12));
        set.insert_range(EvenInts(20)..=EvenInts(34));
        set.remove_range(EvenInts(30)..=EvenInts(34));

        assert!(set.contains(EvenInts(2)));
        assert!(!set.contains(EvenInts(4)));

        assert!(!set.contains(EvenInts(18)));
        assert!(!set.contains(EvenInts(19)));
        assert!(set.contains(EvenInts(20)));
        assert!(!set.contains(EvenInts(21)));
        assert!(set.contains(EvenInts(28)));
        assert!(!set.contains(EvenInts(29)));
        assert!(!set.contains(EvenInts(30)));

        let copy: IntSet<EvenInts> = set.iter().collect();
        assert_eq!(set, copy);

        set.invert();

        assert!(!set.contains(EvenInts(2)));
        assert!(set.contains(EvenInts(4)));

        let Some(max) = set.iter().max() else {
            panic!("should have a max");
        };

        assert_eq!(max.0, u16::MAX - 1);

        {
            let mut it = set.iter();
            assert_eq!(it.next(), Some(EvenInts(0)));
            assert_eq!(it.next(), Some(EvenInts(4)));
            assert_eq!(it.next(), Some(EvenInts(6)));
            assert_eq!(it.next(), Some(EvenInts(10)));
            assert_eq!(it.next(), Some(EvenInts(14)));
        }

        set.insert_range(EvenInts(6)..=EvenInts(10));
        {
            let mut it = set.iter();
            assert_eq!(it.next(), Some(EvenInts(0)));
            assert_eq!(it.next(), Some(EvenInts(4)));
            assert_eq!(it.next(), Some(EvenInts(6)));
            assert_eq!(it.next(), Some(EvenInts(8)));
            assert_eq!(it.next(), Some(EvenInts(10)));
            assert_eq!(it.next(), Some(EvenInts(14)));
        }

        set.remove_range(EvenInts(6)..=EvenInts(10));
        {
            let mut it = set.iter();
            assert_eq!(it.next(), Some(EvenInts(0)));
            assert_eq!(it.next(), Some(EvenInts(4)));
            assert_eq!(it.next(), Some(EvenInts(14)));
        }
    }

    #[test]
    fn with_u16() {
        let mut set = IntSet::<u16>::empty();

        set.insert(5);
        set.insert(8);
        set.insert(12);
        set.insert_range(200..=210);

        assert!(set.contains(5));
        assert!(!set.contains(6));
        assert!(!set.contains(199));
        assert!(set.contains(200));
        assert!(set.contains(210));
        assert!(!set.contains(211));

        let copy: IntSet<u16> = set.iter().collect();
        assert_eq!(set, copy);

        set.invert();

        assert!(!set.contains(5));
        assert!(set.contains(6));

        let Some(max) = set.iter().max() else {
            panic!("should have a max");
        };

        assert_eq!(max, u16::MAX);

        let mut it = set.iter();
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), Some(6));
    }

    #[test]
    fn with_glyph_id_16() {
        let mut set = IntSet::<font_types::GlyphId16>::empty();

        set.insert(GlyphId16::new(5));
        set.insert(GlyphId16::new(8));
        set.insert(GlyphId16::new(12));
        set.insert_range(GlyphId16::new(200)..=GlyphId16::new(210));

        assert!(set.contains(GlyphId16::new(5)));
        assert!(!set.contains(GlyphId16::new(6)));
        assert!(!set.contains(GlyphId16::new(199)));
        assert!(set.contains(GlyphId16::new(200)));
        assert!(set.contains(GlyphId16::new(210)));
        assert!(!set.contains(GlyphId16::new(211)));

        let copy: IntSet<GlyphId16> = set.iter().collect();
        assert_eq!(set, copy);

        set.invert();

        assert!(!set.contains(GlyphId16::new(5)));
        assert!(set.contains(GlyphId16::new(6)));

        let Some(max) = set.iter().max() else {
            panic!("should have a max");
        };

        assert_eq!(max, GlyphId16::new(u16::MAX));

        let mut it = set.iter();
        assert_eq!(it.next(), Some(GlyphId16::new(0)));
        assert_eq!(it.next(), Some(GlyphId16::new(1)));
        assert_eq!(it.next(), Some(GlyphId16::new(2)));
        assert_eq!(it.next(), Some(GlyphId16::new(3)));
        assert_eq!(it.next(), Some(GlyphId16::new(4)));
        assert_eq!(it.next(), Some(GlyphId16::new(6)));
    }

    #[test]
    fn with_glyph_id() {
        let mut set = IntSet::<font_types::GlyphId>::empty();

        set.insert(GlyphId::new(5));
        set.insert(GlyphId::new(8));
        set.insert(GlyphId::new(12));
        set.insert_range(GlyphId::new(200)..=GlyphId::new(210));

        assert!(set.contains(GlyphId::new(5)));
        assert!(!set.contains(GlyphId::new(6)));
        assert!(!set.contains(GlyphId::new(199)));
        assert!(set.contains(GlyphId::new(200)));
        assert!(set.contains(GlyphId::new(210)));
        assert!(!set.contains(GlyphId::new(211)));

        let copy: IntSet<GlyphId> = set.iter().collect();
        assert_eq!(set, copy);

        set.invert();

        assert!(!set.contains(GlyphId::new(5)));
        assert!(set.contains(GlyphId::new(6)));

        let mut it = set.iter();
        assert_eq!(it.next(), Some(GlyphId::new(0)));
        assert_eq!(it.next(), Some(GlyphId::new(1)));
        assert_eq!(it.next(), Some(GlyphId::new(2)));
        assert_eq!(it.next(), Some(GlyphId::new(3)));
        assert_eq!(it.next(), Some(GlyphId::new(4)));
        assert_eq!(it.next(), Some(GlyphId::new(6)));
    }

    #[test]
    fn with_tag() {
        let mut set = IntSet::<Tag>::empty();

        set.insert(Tag::new(b"GSUB"));
        set.insert(Tag::new(b"CFF "));
        set.insert(Tag::new(b"OS/2"));

        assert!(set.contains(Tag::new(b"GSUB")));
        assert!(!set.contains(Tag::new(b"GSU ")));
        assert!(set.contains(Tag::new(b"CFF ")));
        assert!(set.contains(Tag::new(b"OS/2")));

        let copy: IntSet<Tag> = set.iter().collect();
        assert_eq!(set, copy);

        set.invert();

        assert!(!set.contains(Tag::new(b"GSUB")));
        assert!(set.contains(Tag::new(b"GSU ")));
        assert!(!set.contains(Tag::new(b"CFF ")));
        assert!(!set.contains(Tag::new(b"OS/2")));
    }

    #[test]
    fn intersects_range() {
        let mut set = IntSet::<u32>::empty();
        assert!(!set.intersects_range(0..=0));
        assert!(!set.intersects_range(0..=100));
        assert!(!set.intersects_range(0..=u32::MAX));
        assert!(!set.intersects_range(u32::MAX..=u32::MAX));

        set.insert(1234);
        assert!(!set.intersects_range(0..=1233));
        assert!(!set.intersects_range(1235..=1240));

        assert!(set.intersects_range(1234..=1234));
        assert!(set.intersects_range(1230..=1240));
        assert!(set.intersects_range(0..=1234));
        assert!(set.intersects_range(1234..=u32::MAX));

        set.insert(0);
        assert!(set.intersects_range(0..=0));
        assert!(!set.intersects_range(1..=1));
    }

    #[test]
    fn intersects_set() {
        macro_rules! assert_intersects {
            ($lhs:path, $rhs:path, $expected:expr) => {
                assert_eq!($lhs.intersects_set(&$rhs), $expected);
                assert_eq!($rhs.intersects_set(&$lhs), $expected);
            };
        }

        assert!(!IntSet::<u32>::empty().intersects_set(&IntSet::<u32>::empty()));

        let empty = IntSet::<u32>::empty();
        let a = IntSet::from([1u32, 5, 6, 7, 8, 12]);
        let b = IntSet::from([2u32, 13]);
        let c = IntSet::from([8u32, 14]);
        let mut d = IntSet::all();
        d.remove_range(0u32..=13);
        let mut e = IntSet::all();
        e.remove_range(0u32..=100);

        assert_intersects!(a, b, false);
        assert_intersects!(a, c, true);
        assert_intersects!(a, d, false);

        assert_intersects!(b, c, false);
        assert_intersects!(b, d, false);
        assert_intersects!(b, e, false);

        assert_intersects!(c, d, true);
        assert_intersects!(c, e, false);

        assert_intersects!(d, e, true);

        assert_intersects!(a, empty, false);
        assert_intersects!(b, empty, false);
        assert_intersects!(c, empty, false);
        assert_intersects!(d, empty, false);
        assert_intersects!(e, empty, false);
    }

    #[test]
    fn intersects_range_discontinuous() {
        let mut set = IntSet::<EvenInts>::empty();
        assert!(!set.intersects_range(EvenInts(0)..=EvenInts(0)));
        assert!(!set.intersects_range(EvenInts(0)..=EvenInts(100)));
        assert!(!set.intersects_range(EvenInts(0)..=EvenInts(u16::MAX - 1)));
        assert!(!set.intersects_range(EvenInts(u16::MAX - 1)..=EvenInts(u16::MAX - 1)));

        set.insert(EvenInts(1234));
        assert!(!set.intersects_range(EvenInts(0)..=EvenInts(1232)));
        assert!(!set.intersects_range(EvenInts(1236)..=EvenInts(1240)));

        assert!(set.intersects_range(EvenInts(1234)..=EvenInts(1234)));
        assert!(set.intersects_range(EvenInts(1230)..=EvenInts(1240)));
        assert!(set.intersects_range(EvenInts(0)..=EvenInts(1234)));
        assert!(set.intersects_range(EvenInts(1234)..=EvenInts(u16::MAX - 1)));

        set.insert(EvenInts(0));
        assert!(set.intersects_range(EvenInts(0)..=EvenInts(0)));
        assert!(!set.intersects_range(EvenInts(2)..=EvenInts(2)));
    }

    #[test]
    fn intersects_range_exclusive() {
        let mut set = IntSet::<u32>::all();
        assert!(set.intersects_range(0..=0));
        assert!(set.intersects_range(0..=100));
        assert!(set.intersects_range(0..=u32::MAX));
        assert!(set.intersects_range(u32::MAX..=u32::MAX));

        set.remove(1234);
        assert!(set.intersects_range(0..=1233));
        assert!(set.intersects_range(1235..=1240));

        assert!(!set.intersects_range(1234..=1234));
        assert!(set.intersects_range(1230..=1240));
        assert!(set.intersects_range(0..=1234));
        assert!(set.intersects_range(1234..=u32::MAX));

        set.remove(0);
        assert!(!set.intersects_range(0..=0));
        assert!(set.intersects_range(1..=1));

        set.remove_range(5000..=5200);
        assert!(!set.intersects_range(5000..=5200));
        assert!(!set.intersects_range(5100..=5150));
        assert!(set.intersects_range(4999..=5200));
        assert!(set.intersects_range(5000..=5201));
    }

    #[test]
    fn intersects_range_exclusive_discontinuous() {
        let mut set = IntSet::<EvenInts>::all();
        assert!(set.intersects_range(EvenInts(0)..=EvenInts(0)));
        assert!(set.intersects_range(EvenInts(0)..=EvenInts(100)));
        assert!(set.intersects_range(EvenInts(0)..=EvenInts(u16::MAX - 1)));
        assert!(set.intersects_range(EvenInts(u16::MAX - 1)..=EvenInts(u16::MAX - 1)));

        set.remove(EvenInts(1234));
        assert!(set.intersects_range(EvenInts(0)..=EvenInts(1232)));
        assert!(set.intersects_range(EvenInts(1236)..=EvenInts(1240)));

        assert!(!set.intersects_range(EvenInts(1234)..=EvenInts(1234)));
        assert!(set.intersects_range(EvenInts(1230)..=EvenInts(1240)));
        assert!(set.intersects_range(EvenInts(0)..=EvenInts(1234)));
        assert!(set.intersects_range(EvenInts(1234)..=EvenInts(u16::MAX - 1)));

        set.remove(EvenInts(0));
        assert!(!set.intersects_range(EvenInts(0)..=EvenInts(0)));
        assert!(set.intersects_range(EvenInts(2)..=EvenInts(2)));

        set.remove_range(EvenInts(5000)..=EvenInts(5200));
        assert!(!set.intersects_range(EvenInts(5000)..=EvenInts(5200)));
        assert!(!set.intersects_range(EvenInts(5100)..=EvenInts(5150)));
        assert!(set.intersects_range(EvenInts(4998)..=EvenInts(5200)));
        assert!(set.intersects_range(EvenInts(5000)..=EvenInts(5202)));
    }

    #[test]
    fn length() {
        let mut s = IntSet::<u32>::empty();
        assert_eq!(s.len(), 0);
        s.insert(5);
        s.insert(5);
        s.insert(100);
        assert_eq!(s.len(), 2);

        s.invert();
        assert_eq!(s.len(), (u32::MAX - 1) as u64);

        assert_eq!(IntSet::<u32>::all().len(), (u32::MAX as u64) + 1);

        let mut s = IntSet::<TwoParts>::all();
        assert_eq!(s.len(), 13);
        s.remove(TwoParts::from_u32(InDomain(5)));
        assert_eq!(s.len(), 12);

        for v in TwoParts::ordered_values() {
            s.remove(TwoParts::from_u32(InDomain(v)));
        }
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn ordering() {
        macro_rules! assert_ord {
            ($lhs:expr, $rhs:expr, $ord:path) => {
                assert_eq!(
                    IntSet::from($lhs.clone()).cmp(&IntSet::from($rhs.clone())),
                    $ord,
                    "{:?}, {:?}",
                    $lhs,
                    $rhs
                )
            };
        }

        const EMPTY: [u16; 0] = [];
        assert_ord!(EMPTY, EMPTY, Ordering::Equal);
        assert_ord!(EMPTY, [0], Ordering::Less);
        assert_ord!([0u16], [0], Ordering::Equal);
        assert_ord!([0u16, 1, 2], [1, 2, 3], Ordering::Less);
        assert_ord!([0u16, 1, 4], [1, 2, 3], Ordering::Less);
        assert_ord!([1u16, 2, 3], [0, 2, 4], Ordering::Greater);
        assert_ord!([5u16, 4, 0], [1, 2, 3], Ordering::Less); // out of order
        assert_ord!([1u16, 2, 3], [1, 2, 3, 4], Ordering::Less); // out of order
        assert_ord!([2u16, 3, 4], [1, 2, 3, 4, 5], Ordering::Greater); // out of order

        // Exclusive - Exclusive
        let all = IntSet::<u16>::all();
        let mut all_but_0 = all.clone();
        all_but_0.remove(0);
        let mut all_but_5 = all.clone();
        all_but_5.remove(5);

        assert_eq!(all.cmp(&all), Ordering::Equal);
        assert_eq!(all.cmp(&all_but_0), Ordering::Less);
        assert_eq!(all_but_0.cmp(&all), Ordering::Greater);

        let mut a = IntSet::<u16>::all();
        a.remove_range(0..=5);
        a.remove_range(221..=1693);
        let mut b = IntSet::<u16>::all();
        b.remove_range(0..=1693);
        assert_eq!(a.cmp(&b), Ordering::Less);

        // Mixed
        let mut inc_all_but_0 = IntSet::<u16>::empty();
        inc_all_but_0.insert_range(1..=u16::MAX);
        let mut inc_all_but_5 = IntSet::<u16>::empty();
        inc_all_but_5.insert_range(0..=4);
        inc_all_but_5.insert_range(6..=u16::MAX);

        assert_eq!(all.cmp(&all), Ordering::Equal);
        assert_eq!(all.cmp(&inc_all_but_0), Ordering::Less);
        assert_eq!(inc_all_but_0.cmp(&all), Ordering::Greater);
        assert_eq!(inc_all_but_5.cmp(&all_but_0), Ordering::Less);

        let mut a = IntSet::<u16>::all();
        a.remove_range(8..=1160);
        let mut b = IntSet::<u16>::empty();
        b.insert_range(0..=259);

        assert_eq!(a.cmp(&b), Ordering::Greater);

        let mut a = IntSet::<u16>::all();
        a.remove_range(8..=u16::MAX);
        let mut b = IntSet::<u16>::empty();
        b.insert_range(0..=259);

        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[cfg(feature = "serde")]
    fn roundtrip_json<T: Domain>(set: &IntSet<T>) -> Result<IntSet<T>, serde_json::Error> {
        let json = serde_json::to_vec(&set).unwrap();
        serde_json::from_slice(&json)
    }

    #[test]
    #[cfg(feature = "serde")]
    fn simple_serde() {
        let mut set = IntSet::empty();
        set.insert(0u32);
        set.insert(u32::MAX);
        assert_eq!(roundtrip_json(&set).unwrap(), set);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_non_contiguous() {
        fn ev(val: u16) -> EvenInts {
            assert!(val % 2 == 0);
            EvenInts(val)
        }
        let set = IntSet::<EvenInts>::from([ev(2), ev(166), ev(u16::MAX - 1)]);
        assert_eq!(roundtrip_json(&set).unwrap(), set);
    }

    #[test]
    #[cfg(feature = "serde")]
    #[should_panic(expected = "out of range for domain")]
    fn serde_non_contiguous_out_of_domain() {
        let set = IntSet::from([1u16, 2, 3, 4, 5, 6, 7]);
        let bytes = serde_json::to_vec(&set).unwrap();
        serde_json::from_slice::<IntSet<EvenInts>>(&bytes).unwrap();
    }

    #[test]
    #[cfg(feature = "serde")]
    fn non_contiguous_inverted() {
        let all = IntSet::<u16>::all();
        let bytes = serde_json::to_vec(&all).unwrap();
        let readback: IntSet<EvenInts> = serde_json::from_slice(&bytes).unwrap();
        let mut iter = readback.iter().map(|v| v.0);
        let mut values = (&mut iter).take(5).collect::<Vec<_>>();
        values.extend(iter.rev().take(5));

        assert_eq!(values, [0, 2, 4, 6, 8, 65534, 65532, 65530, 65528, 65526])
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_inverted() {
        let mut set = IntSet::all();
        set.remove_range(0u16..=420);
        let bytes = serde_json::to_string(&set).unwrap();
        assert!(bytes.len() < 5000, "sanity check serialization");
        assert_eq!(roundtrip_json(&set).unwrap(), set)
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_inverted_out_of_domain() {
        let mut set = IntSet::all();
        set.remove_range(0u16..=250);
        let bytes = serde_json::to_string(&set).unwrap();
        let readback: IntSet<u8> = serde_json::from_str(&bytes).unwrap();
        assert_eq!(readback.len(), 5);
        assert_eq!(
            readback.iter().collect::<Vec<_>>(),
            [251, 252, 253, 254, 255]
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    #[should_panic(expected = "out of range for domain")]
    fn serde_out_of_domain() {
        let set = IntSet::from([u32::MAX]);
        let json = serde_json::to_vec(&set).unwrap();
        serde_json::from_slice::<IntSet<GlyphId16>>(&json).unwrap();
    }
}
