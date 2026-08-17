//! A fixed-capacity hash set where the iteration order is independent of the hash values.
use core::{
    borrow::Borrow,
    fmt,
    hash::{BuildHasher, Hash},
};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

use hash32::{BuildHasherDefault, FnvHasher};

use crate::index_map::{self, IndexMap};

/// An [`IndexSet`] using the default FNV hasher.
///
/// A list of all Methods and Traits available for `FnvIndexSet` can be found in
/// the [`IndexSet`] documentation.
///
/// # Examples
/// ```
/// use heapless::index_set::FnvIndexSet;
///
/// // A hash set with a capacity of 16 elements allocated on the stack
/// let mut books = FnvIndexSet::<_, 16>::new();
///
/// // Add some books.
/// books.insert("A Dance With Dragons").unwrap();
/// books.insert("To Kill a Mockingbird").unwrap();
/// books.insert("The Odyssey").unwrap();
/// books.insert("The Great Gatsby").unwrap();
///
/// // Check for a specific one.
/// if !books.contains("The Winds of Winter") {
///     println!(
///         "We have {} books, but The Winds of Winter ain't one.",
///         books.len()
///     );
/// }
///
/// // Remove a book.
/// books.remove("The Odyssey");
///
/// // Iterate over everything.
/// for book in &books {
///     println!("{}", book);
/// }
/// ```
pub type FnvIndexSet<T, const N: usize> = IndexSet<T, BuildHasherDefault<FnvHasher>, N>;

/// Fixed capacity [`IndexSet`](https://docs.rs/indexmap/2/indexmap/set/struct.IndexSet.html).
///
/// Note that you cannot use `IndexSet` directly, since it is generic around the hashing algorithm
/// in use. Pick a concrete instantiation like [`FnvIndexSet`] instead
/// or create your own.
///
/// Note that the capacity of the `IndexSet` must be a power of 2.
///
/// # Examples
/// Since `IndexSet` cannot be used directly, we're using its `FnvIndexSet` instantiation
/// for this example.
///
/// ```
/// use heapless::index_set::FnvIndexSet;
///
/// // A hash set with a capacity of 16 elements allocated on the stack
/// let mut books = FnvIndexSet::<_, 16>::new();
///
/// // Add some books.
/// books.insert("A Dance With Dragons").unwrap();
/// books.insert("To Kill a Mockingbird").unwrap();
/// books.insert("The Odyssey").unwrap();
/// books.insert("The Great Gatsby").unwrap();
///
/// // Check for a specific one.
/// if !books.contains("The Winds of Winter") {
///     println!(
///         "We have {} books, but The Winds of Winter ain't one.",
///         books.len()
///     );
/// }
///
/// // Remove a book.
/// books.remove("The Odyssey");
///
/// // Iterate over everything.
/// for book in &books {
///     println!("{}", book);
/// }
/// ```
#[cfg_attr(feature = "zeroize", derive(Zeroize), zeroize(bound = "T: Zeroize"))]
pub struct IndexSet<T, S, const N: usize> {
    map: IndexMap<T, (), S, N>,
}

impl<T, S, const N: usize> IndexSet<T, BuildHasherDefault<S>, N> {
    /// Creates an empty `IndexSet`
    pub const fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }
}

impl<T, S, const N: usize> IndexSet<T, S, N> {
    /// Returns the number of elements the set can hold
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let set = FnvIndexSet::<i32, 16>::new();
    /// assert_eq!(set.capacity(), 16);
    /// ```
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Return an iterator over the values of the set, in insertion order
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut set = FnvIndexSet::<_, 16>::new();
    /// set.insert("a").unwrap();
    /// set.insert("b").unwrap();
    ///
    /// // Will print in insertion order: a, b
    /// for x in set.iter() {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.map.iter(),
        }
    }

    /// Get the first value
    ///
    /// Computes in *O*(1) time
    pub fn first(&self) -> Option<&T> {
        self.map.first().map(|(k, _v)| k)
    }

    /// Get the last value
    ///
    /// Computes in *O*(1) time
    pub fn last(&self) -> Option<&T> {
        self.map.last().map(|(k, _v)| k)
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut v: FnvIndexSet<_, 16> = FnvIndexSet::new();
    /// assert_eq!(v.len(), 0);
    /// v.insert(1).unwrap();
    /// assert_eq!(v.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut v: FnvIndexSet<_, 16> = FnvIndexSet::new();
    /// assert!(v.is_empty());
    /// v.insert(1).unwrap();
    /// assert!(!v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns `true` if the set is full.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut v: FnvIndexSet<_, 4> = FnvIndexSet::new();
    /// assert!(!v.is_full());
    /// v.insert(1).unwrap();
    /// v.insert(2).unwrap();
    /// v.insert(3).unwrap();
    /// v.insert(4).unwrap();
    /// assert!(v.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.map.is_full()
    }

    /// Clears the set, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut v: FnvIndexSet<_, 16> = FnvIndexSet::new();
    /// v.insert(1).unwrap();
    /// v.clear();
    /// assert!(v.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<T, S, const N: usize> IndexSet<T, S, N>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    /// Visits the values representing the difference, i.e. the values that are in `self` but not in
    /// `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut a: FnvIndexSet<_, 16> = [1, 2, 3].iter().cloned().collect();
    /// let mut b: FnvIndexSet<_, 16> = [4, 2, 3, 4].iter().cloned().collect();
    ///
    /// // Can be seen as `a - b`.
    /// for x in a.difference(&b) {
    ///     println!("{}", x); // Print 1
    /// }
    ///
    /// let diff: FnvIndexSet<_, 16> = a.difference(&b).collect();
    /// assert_eq!(diff, [1].iter().collect::<FnvIndexSet<_, 16>>());
    ///
    /// // Note that difference is not symmetric,
    /// // and `b - a` means something else:
    /// let diff: FnvIndexSet<_, 16> = b.difference(&a).collect();
    /// assert_eq!(diff, [4].iter().collect::<FnvIndexSet<_, 16>>());
    /// ```
    pub fn difference<'a, S2, const N2: usize>(
        &'a self,
        other: &'a IndexSet<T, S2, N2>,
    ) -> Difference<'a, T, S2, N2>
    where
        S2: BuildHasher,
    {
        Difference {
            iter: self.iter(),
            other,
        }
    }

    /// Visits the values representing the symmetric difference, i.e. the values that are in `self`
    /// or in `other` but not in both.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut a: FnvIndexSet<_, 16> = [1, 2, 3].iter().cloned().collect();
    /// let mut b: FnvIndexSet<_, 16> = [4, 2, 3, 4].iter().cloned().collect();
    ///
    /// // Print 1, 4 in that order.
    /// for x in a.symmetric_difference(&b) {
    ///     println!("{}", x);
    /// }
    ///
    /// let diff1: FnvIndexSet<_, 16> = a.symmetric_difference(&b).collect();
    /// let diff2: FnvIndexSet<_, 16> = b.symmetric_difference(&a).collect();
    ///
    /// assert_eq!(diff1, diff2);
    /// assert_eq!(diff1, [1, 4].iter().collect::<FnvIndexSet<_, 16>>());
    /// ```
    pub fn symmetric_difference<'a, S2, const N2: usize>(
        &'a self,
        other: &'a IndexSet<T, S2, N2>,
    ) -> impl Iterator<Item = &'a T>
    where
        S2: BuildHasher,
    {
        self.difference(other).chain(other.difference(self))
    }

    /// Visits the values representing the intersection, i.e. the values that are both in `self` and
    /// `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut a: FnvIndexSet<_, 16> = [1, 2, 3].iter().cloned().collect();
    /// let mut b: FnvIndexSet<_, 16> = [4, 2, 3, 4].iter().cloned().collect();
    ///
    /// // Print 2, 3 in that order.
    /// for x in a.intersection(&b) {
    ///     println!("{}", x);
    /// }
    ///
    /// let intersection: FnvIndexSet<_, 16> = a.intersection(&b).collect();
    /// assert_eq!(intersection, [2, 3].iter().collect::<FnvIndexSet<_, 16>>());
    /// ```
    pub fn intersection<'a, S2, const N2: usize>(
        &'a self,
        other: &'a IndexSet<T, S2, N2>,
    ) -> Intersection<'a, T, S2, N2>
    where
        S2: BuildHasher,
    {
        Intersection {
            iter: self.iter(),
            other,
        }
    }

    /// Visits the values representing the union, i.e. all the values in `self` or `other`, without
    /// duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut a: FnvIndexSet<_, 16> = [1, 2, 3].iter().cloned().collect();
    /// let mut b: FnvIndexSet<_, 16> = [4, 2, 3, 4].iter().cloned().collect();
    ///
    /// // Print 1, 2, 3, 4 in that order.
    /// for x in a.union(&b) {
    ///     println!("{}", x);
    /// }
    ///
    /// let union: FnvIndexSet<_, 16> = a.union(&b).collect();
    /// assert_eq!(union, [1, 2, 3, 4].iter().collect::<FnvIndexSet<_, 16>>());
    /// ```
    pub fn union<'a, S2, const N2: usize>(
        &'a self,
        other: &'a IndexSet<T, S2, N2>,
    ) -> impl Iterator<Item = &'a T>
    where
        S2: BuildHasher,
    {
        self.iter().chain(other.difference(self))
    }

    /// Returns `true` if the set contains a value.
    ///
    /// The value may be any borrowed form of the set's value type, but `Hash` and `Eq` on the
    /// borrowed form must match those for the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let set: FnvIndexSet<_, 16> = [1, 2, 3].iter().cloned().collect();
    /// assert_eq!(set.contains(&1), true);
    /// assert_eq!(set.contains(&4), false);
    /// ```
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + Eq + Hash,
    {
        self.map.contains_key(value)
    }

    /// Returns `true` if `self` has no elements in common with `other`. This is equivalent to
    /// checking for an empty intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let a: FnvIndexSet<_, 16> = [1, 2, 3].iter().cloned().collect();
    /// let mut b = FnvIndexSet::<_, 16>::new();
    ///
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(4).unwrap();
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(1).unwrap();
    /// assert_eq!(a.is_disjoint(&b), false);
    /// ```
    pub fn is_disjoint<S2, const N2: usize>(&self, other: &IndexSet<T, S2, N2>) -> bool
    where
        S2: BuildHasher,
    {
        self.iter().all(|v| !other.contains(v))
    }

    /// Returns `true` if the set is a subset of another, i.e. `other` contains at least all the
    /// values in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let sup: FnvIndexSet<_, 16> = [1, 2, 3].iter().cloned().collect();
    /// let mut set = FnvIndexSet::<_, 16>::new();
    ///
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(2).unwrap();
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(4).unwrap();
    /// assert_eq!(set.is_subset(&sup), false);
    /// ```
    pub fn is_subset<S2, const N2: usize>(&self, other: &IndexSet<T, S2, N2>) -> bool
    where
        S2: BuildHasher,
    {
        self.iter().all(|v| other.contains(v))
    }

    // Returns `true` if the set is a superset of another, i.e. `self` contains at least all the
    // values in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let sub: FnvIndexSet<_, 16> = [1, 2].iter().cloned().collect();
    /// let mut set = FnvIndexSet::<_, 16>::new();
    ///
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(0).unwrap();
    /// set.insert(1).unwrap();
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(2).unwrap();
    /// assert_eq!(set.is_superset(&sub), true);
    /// ```
    pub fn is_superset<S2, const N2: usize>(&self, other: &IndexSet<T, S2, N2>) -> bool
    where
        S2: BuildHasher,
    {
        other.is_subset(self)
    }

    /// Adds a value to the set.
    ///
    /// If the set did not have this value present, `true` is returned.
    ///
    /// If the set did have this value present, `false` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut set = FnvIndexSet::<_, 16>::new();
    ///
    /// assert_eq!(set.insert(2).unwrap(), true);
    /// assert_eq!(set.insert(2).unwrap(), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn insert(&mut self, value: T) -> Result<bool, T> {
        self.map
            .insert(value, ())
            .map(|old| old.is_none())
            .map_err(|(k, _)| k)
    }

    /// Removes a value from the set. Returns `true` if the value was present in the set.
    ///
    /// The value may be any borrowed form of the set's value type, but `Hash` and `Eq` on the
    /// borrowed form must match those for the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::index_set::FnvIndexSet;
    ///
    /// let mut set = FnvIndexSet::<_, 16>::new();
    ///
    /// set.insert(2).unwrap();
    /// assert_eq!(set.remove(&2), true);
    /// assert_eq!(set.remove(&2), false);
    /// ```
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + Eq + Hash,
    {
        self.map.remove(value).is_some()
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.map.retain(move |k, _| f(k));
    }
}

impl<T, S, const N: usize> Clone for IndexSet<T, S, N>
where
    T: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<T, S, const N: usize> fmt::Debug for IndexSet<T, S, N>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T, S, const N: usize> Default for IndexSet<T, S, N>
where
    S: Default,
{
    fn default() -> Self {
        Self {
            map: <_>::default(),
        }
    }
}

impl<T, S1, S2, const N1: usize, const N2: usize> PartialEq<IndexSet<T, S2, N2>>
    for IndexSet<T, S1, N1>
where
    T: Eq + Hash,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, other: &IndexSet<T, S2, N2>) -> bool {
        self.len() == other.len() && self.is_subset(other)
    }
}

impl<T, S, const N: usize> Extend<T> for IndexSet<T, S, N>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.map.extend(iterable.into_iter().map(|k| (k, ())));
    }
}

impl<'a, T, S, const N: usize> Extend<&'a T> for IndexSet<T, S, N>
where
    T: 'a + Eq + Hash + Copy,
    S: BuildHasher,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.extend(iterable.into_iter().cloned());
    }
}

impl<T, S, const N: usize> FromIterator<T> for IndexSet<T, S, N>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut set = Self::default();
        set.extend(iter);
        set
    }
}

impl<'a, T, S, const N: usize> IntoIterator for &'a IndexSet<T, S, N>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the items of a [`IndexSet`].
///
/// This `struct` is created by the [`iter`](IndexSet::iter) method on [`IndexSet`]. See its
/// documentation for more.
pub struct Iter<'a, T> {
    iter: index_map::Iter<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

/// An iterator over the difference of two `IndexSet`s.
///
/// This is created by the [`IndexSet::difference`] method.
pub struct Difference<'a, T, S, const N: usize>
where
    S: BuildHasher,
    T: Eq + Hash,
{
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S, N>,
}

impl<'a, T, S, const N: usize> Iterator for Difference<'a, T, S, N>
where
    S: BuildHasher,
    T: Eq + Hash,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let elt = self.iter.next()?;
            if !self.other.contains(elt) {
                return Some(elt);
            }
        }
    }
}

/// An iterator over the intersection of two `IndexSet`s.
///
/// This is created by the [`IndexSet::intersection`] method.
pub struct Intersection<'a, T, S, const N: usize>
where
    S: BuildHasher,
    T: Eq + Hash,
{
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S, N>,
}

impl<'a, T, S, const N: usize> Iterator for Intersection<'a, T, S, N>
where
    S: BuildHasher,
    T: Eq + Hash,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let elt = self.iter.next()?;
            if self.other.contains(elt) {
                return Some(elt);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use static_assertions::assert_not_impl_any;

    use super::{BuildHasherDefault, IndexSet};

    // Ensure a `IndexSet` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(IndexSet<*const (), BuildHasherDefault<()>, 4>: Send);

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_index_set_zeroize() {
        use zeroize::Zeroize;

        let mut set: IndexSet<u8, BuildHasherDefault<hash32::FnvHasher>, 8> = IndexSet::new();
        for i in 1..=8 {
            set.insert(i).unwrap();
        }

        assert_eq!(set.len(), 8);
        assert!(set.contains(&8));

        // zeroized using index_map's implementation
        set.zeroize();

        assert_eq!(set.len(), 0);
        assert!(set.is_empty());

        set.insert(1).unwrap();
        assert_eq!(set.len(), 1);
        assert!(set.contains(&1));
    }
}
