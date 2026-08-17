/*! The [`Arena`], [`UniqueArena`], and [`Handle`] types.

To improve translator performance and reduce memory usage, most structures are
stored in an [`Arena`]. An `Arena<T>` stores a series of `T` values, indexed by
[`Handle<T>`] values, which are just wrappers around integer indexes.
For example, a `Function`'s expressions are stored in an `Arena<Expression>`,
and compound expressions refer to their sub-expressions via `Handle<Expression>`
values.

A [`UniqueArena`] is just like an `Arena`, except that it stores only a single
instance of each value. The value type must implement `Eq` and `Hash`. Like an
`Arena`, inserting a value into a `UniqueArena` returns a `Handle` which can be
used to efficiently access the value, without a hash lookup. Inserting a value
multiple times returns the same `Handle`.

If the `span` feature is enabled, both `Arena` and `UniqueArena` can associate a
source code span with each element.

[`Handle<T>`]: Handle
*/

mod handle;
mod handle_set;
mod handlevec;
mod range;
mod unique_arena;

pub use handle::{BadHandle, Handle};
pub(crate) use handle_set::HandleSet;
pub(crate) use handlevec::HandleVec;
pub use range::{BadRangeError, Range};
pub use unique_arena::UniqueArena;

use alloc::vec::Vec;
use core::{fmt, ops};

use crate::Span;

use handle::Index;

/// An arena holding some kind of component (e.g., type, constant,
/// instruction, etc.) that can be referenced.
///
/// Adding new items to the arena produces a strongly-typed [`Handle`].
/// The arena can be indexed using the given handle to obtain
/// a reference to the stored item.
#[derive(Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(transparent))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(test, derive(PartialEq))]
pub struct Arena<T> {
    /// Values of this arena.
    data: Vec<T>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    span_info: Vec<Span>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for Arena<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<T> Arena<T> {
    /// Create a new arena with no initial capacity allocated.
    pub const fn new() -> Self {
        Arena {
            data: Vec::new(),
            span_info: Vec::new(),
        }
    }

    /// Extracts the inner vector.
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }

    /// Returns the current number of items stored in this arena.
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the arena contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over the items stored in this arena, returning both
    /// the item's handle and a reference to it.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (Handle<T>, &T)> + ExactSizeIterator {
        self.data
            .iter()
            .enumerate()
            .map(|(i, v)| (Handle::from_usize(i), v))
    }

    /// Returns an iterator over the items stored in this arena, returning both
    /// the item's handle and a reference to it.
    pub fn iter_mut_span(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = (Handle<T>, &mut T, &Span)> + ExactSizeIterator {
        self.data
            .iter_mut()
            .zip(self.span_info.iter())
            .enumerate()
            .map(|(i, (v, span))| (Handle::from_usize(i), v, span))
    }

    /// Drains the arena, returning an iterator over the items stored.
    pub fn drain(&mut self) -> impl DoubleEndedIterator<Item = (Handle<T>, T, Span)> {
        let arena = core::mem::take(self);
        arena
            .data
            .into_iter()
            .zip(arena.span_info)
            .enumerate()
            .map(|(i, (v, span))| (Handle::from_usize(i), v, span))
    }

    /// Returns a iterator over the items stored in this arena,
    /// returning both the item's handle and a mutable reference to it.
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = (Handle<T>, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(i, v)| (Handle::from_usize(i), v))
    }

    /// Adds a new value to the arena, returning a typed handle.
    pub fn append(&mut self, value: T, span: Span) -> Handle<T> {
        let index = self.data.len();
        self.data.push(value);
        self.span_info.push(span);
        Handle::from_usize(index)
    }

    /// Fetch a handle to an existing type.
    pub fn fetch_if<F: Fn(&T) -> bool>(&self, fun: F) -> Option<Handle<T>> {
        self.data
            .iter()
            .position(fun)
            .map(|index| Handle::from_usize(index))
    }

    /// Adds a value with a custom check for uniqueness:
    /// returns a handle pointing to
    /// an existing element if the check succeeds, or adds a new
    /// element otherwise.
    pub fn fetch_if_or_append<F: Fn(&T, &T) -> bool>(
        &mut self,
        value: T,
        span: Span,
        fun: F,
    ) -> Handle<T> {
        if let Some(index) = self.data.iter().position(|d| fun(d, &value)) {
            Handle::from_usize(index)
        } else {
            self.append(value, span)
        }
    }

    /// Adds a value with a check for uniqueness, where the check is plain comparison.
    pub fn fetch_or_append(&mut self, value: T, span: Span) -> Handle<T>
    where
        T: PartialEq,
    {
        self.fetch_if_or_append(value, span, T::eq)
    }

    pub fn try_get(&self, handle: Handle<T>) -> Result<&T, BadHandle> {
        self.data
            .get(handle.index())
            .ok_or_else(|| BadHandle::new(handle))
    }

    /// Get a mutable reference to an element in the arena.
    pub fn get_mut(&mut self, handle: Handle<T>) -> &mut T {
        self.data.get_mut(handle.index()).unwrap()
    }

    /// Get the range of handles from a particular number of elements to the end.
    pub fn range_from(&self, old_length: usize) -> Range<T> {
        let range = old_length as u32..self.data.len() as u32;
        Range::from_index_range(range, self)
    }

    /// Clears the arena keeping all allocations
    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn get_span(&self, handle: Handle<T>) -> Span {
        *self
            .span_info
            .get(handle.index())
            .unwrap_or(&Span::default())
    }

    /// Assert that `handle` is valid for this arena.
    pub fn check_contains_handle(&self, handle: Handle<T>) -> Result<(), BadHandle> {
        if handle.index() < self.data.len() {
            Ok(())
        } else {
            Err(BadHandle::new(handle))
        }
    }

    /// Assert that `range` is valid for this arena.
    pub fn check_contains_range(&self, range: &Range<T>) -> Result<(), BadRangeError> {
        // Since `range.inner` is a `Range<u32>`, we only need to check that the
        // start precedes the end, and that the end is in range.
        if range.inner.start > range.inner.end {
            return Err(BadRangeError::new(range.clone()));
        }

        // Empty ranges are tolerated: they can be produced by compaction.
        if range.inner.start == range.inner.end {
            return Ok(());
        }

        let last_handle = Handle::new(Index::new(range.inner.end - 1).unwrap());
        if self.check_contains_handle(last_handle).is_err() {
            return Err(BadRangeError::new(range.clone()));
        }

        Ok(())
    }

    pub(crate) fn retain_mut<P>(&mut self, mut predicate: P)
    where
        P: FnMut(Handle<T>, &mut T) -> bool,
    {
        let mut index = 0;
        let mut retained = 0;
        self.data.retain_mut(|elt| {
            let handle = Handle::from_usize(index);
            let keep = predicate(handle, elt);

            // Since `predicate` needs mutable access to each element,
            // we can't feasibly call it twice, so we have to compact
            // spans by hand in parallel as part of this iteration.
            if keep {
                self.span_info[retained] = self.span_info[index];
                retained += 1;
            }

            index += 1;
            keep
        });

        self.span_info.truncate(retained);
    }
}

#[cfg(feature = "deserialize")]
impl<'de, T> serde::Deserialize<'de> for Arena<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = Vec::deserialize(deserializer)?;
        let span_info = core::iter::repeat_n(Span::default(), data.len()).collect();

        Ok(Self { data, span_info })
    }
}

impl<T> ops::Index<Handle<T>> for Arena<T> {
    type Output = T;
    fn index(&self, handle: Handle<T>) -> &T {
        &self.data[handle.index()]
    }
}

impl<T> ops::IndexMut<Handle<T>> for Arena<T> {
    fn index_mut(&mut self, handle: Handle<T>) -> &mut T {
        &mut self.data[handle.index()]
    }
}

impl<T> ops::Index<Range<T>> for Arena<T> {
    type Output = [T];
    fn index(&self, range: Range<T>) -> &[T] {
        &self.data[range.inner.start as usize..range.inner.end as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_non_unique() {
        let mut arena: Arena<u8> = Arena::new();
        let t1 = arena.append(0, Default::default());
        let t2 = arena.append(0, Default::default());
        assert!(t1 != t2);
        assert!(arena[t1] == arena[t2]);
    }

    #[test]
    fn append_unique() {
        let mut arena: Arena<u8> = Arena::new();
        let t1 = arena.append(0, Default::default());
        let t2 = arena.append(1, Default::default());
        assert!(t1 != t2);
        assert!(arena[t1] != arena[t2]);
    }

    #[test]
    fn fetch_or_append_non_unique() {
        let mut arena: Arena<u8> = Arena::new();
        let t1 = arena.fetch_or_append(0, Default::default());
        let t2 = arena.fetch_or_append(0, Default::default());
        assert!(t1 == t2);
        assert!(arena[t1] == arena[t2])
    }

    #[test]
    fn fetch_or_append_unique() {
        let mut arena: Arena<u8> = Arena::new();
        let t1 = arena.fetch_or_append(0, Default::default());
        let t2 = arena.fetch_or_append(1, Default::default());
        assert!(t1 != t2);
        assert!(arena[t1] != arena[t2]);
    }
}
