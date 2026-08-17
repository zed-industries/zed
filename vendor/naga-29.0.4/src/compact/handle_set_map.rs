use alloc::vec::Vec;

use crate::arena::{Arena, Handle, HandleSet, Range};

type Index = crate::non_max_u32::NonMaxU32;

/// A map keyed by handles.
///
/// In most cases, this is used to map from old handle indices to new,
/// compressed handle indices.
#[derive(Debug)]
pub struct HandleMap<T, U = Index> {
    /// The indices assigned to handles in the compacted module.
    ///
    /// If `new_index[i]` is `Some(n)`, then `n` is the `Index` of the
    /// compacted `Handle` corresponding to the pre-compacted `Handle`
    /// whose index is `i`.
    new_index: Vec<Option<U>>,

    /// This type is indexed by values of type `T`.
    as_keys: core::marker::PhantomData<T>,
}

impl<T, U> HandleMap<T, U> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            new_index: Vec::with_capacity(capacity),
            as_keys: core::marker::PhantomData,
        }
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&U> {
        self.new_index.get(handle.index()).unwrap_or(&None).as_ref()
    }

    pub fn insert(&mut self, handle: Handle<T>, value: U) -> Option<U> {
        if self.new_index.len() <= handle.index() {
            self.new_index.resize_with(handle.index() + 1, || None);
        }
        self.new_index[handle.index()].replace(value)
    }
}

impl<T: 'static> HandleMap<T> {
    pub fn from_set(set: HandleSet<T>) -> Self {
        let mut next_index = Index::new(0).unwrap();
        Self {
            new_index: set
                .all_possible()
                .map(|handle| {
                    if set.contains(handle) {
                        // This handle will be retained in the compacted version,
                        // so assign it a new index.
                        let this = next_index;
                        next_index = next_index.checked_add(1).unwrap();
                        Some(this)
                    } else {
                        // This handle will be omitted in the compacted version.
                        None
                    }
                })
                .collect(),
            as_keys: core::marker::PhantomData,
        }
    }

    /// Return true if `old` is used in the compacted module.
    pub fn used(&self, old: Handle<T>) -> bool {
        self.new_index[old.index()].is_some()
    }

    /// Return the counterpart to `old` in the compacted module.
    ///
    /// If we thought `old` wouldn't be used in the compacted module, return
    /// `None`.
    pub fn try_adjust(&self, old: Handle<T>) -> Option<Handle<T>> {
        log::trace!(
            "adjusting {} handle [{}] -> [{:?}]",
            core::any::type_name::<T>(),
            old.index(),
            self.new_index[old.index()]
        );
        self.new_index[old.index()].map(Handle::new)
    }

    /// Return the counterpart to `old` in the compacted module.
    ///
    /// If we thought `old` wouldn't be used in the compacted module, panic.
    pub fn adjust(&self, handle: &mut Handle<T>) {
        *handle = self.try_adjust(*handle).unwrap();
    }

    /// Like `adjust`, but for optional handles.
    pub fn adjust_option(&self, handle: &mut Option<Handle<T>>) {
        if let Some(ref mut handle) = *handle {
            self.adjust(handle);
        }
    }

    /// Shrink `range` to include only used handles.
    ///
    /// Fortunately, compaction doesn't arbitrarily scramble the expressions
    /// in the arena, but instead preserves the order of the elements while
    /// squeezing out unused ones. That means that a contiguous range in the
    /// pre-compacted arena always maps to a contiguous range in the
    /// post-compacted arena. So we just need to adjust the endpoints.
    ///
    /// Compaction may have eliminated the endpoints themselves.
    ///
    /// Use `compacted_arena` to bounds-check the result.
    pub fn adjust_range(&self, range: &mut Range<T>, compacted_arena: &Arena<T>) {
        let mut index_range = range.index_range();
        let compacted;
        if let Some(first) = index_range.find_map(|i| self.new_index[i as usize]) {
            // The first call to `find_map` mutated `index_range` to hold the
            // remainder of original range, which is exactly the range we need
            // to search for the new last handle.
            if let Some(last) = index_range.rev().find_map(|i| self.new_index[i as usize]) {
                // Build an end-exclusive range, given the two included indices
                // `first` and `last`.
                compacted = first.get()..last.get() + 1;
            } else {
                // The range contains only a single live handle, which
                // we identified with the first `find_map` call.
                compacted = first.get()..first.get() + 1;
            }
        } else {
            compacted = 0..0;
        };
        *range = Range::from_index_range(compacted, compacted_arena);
    }
}
