//! The [`HandleVec`] type and associated definitions.

use super::handle::Handle;
use alloc::{vec, vec::Vec};

use core::marker::PhantomData;
use core::ops;

/// A [`Vec`] indexed by [`Handle`]s.
///
/// A `HandleVec<T, U>` is a [`Vec<U>`] indexed by values of type `Handle<T>`,
/// rather than `usize`.
///
/// Rather than a `push` method, `HandleVec` has an [`insert`] method, analogous
/// to [`HashMap::insert`], that requires you to provide the handle at which the
/// new value should appear. However, since `HandleVec` only supports insertion
/// at the end, the given handle's index must be equal to the `HandleVec`'s
/// current length; otherwise, the insertion will panic.
///
/// [`insert`]: HandleVec::insert
/// [`HashMap::insert`]: hashbrown::HashMap::insert
#[derive(Debug)]
pub(crate) struct HandleVec<T, U> {
    inner: Vec<U>,
    as_keys: PhantomData<T>,
}

impl<T, U> Default for HandleVec<T, U> {
    fn default() -> Self {
        Self {
            inner: vec![],
            as_keys: PhantomData,
        }
    }
}

#[allow(dead_code)]
impl<T, U> HandleVec<T, U> {
    pub(crate) const fn new() -> Self {
        Self {
            inner: vec![],
            as_keys: PhantomData,
        }
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            as_keys: PhantomData,
        }
    }

    pub(crate) const fn len(&self) -> usize {
        self.inner.len()
    }

    /// Insert a mapping from `handle` to `value`.
    ///
    /// Unlike a [`HashMap`], a `HandleVec` can only have new entries inserted at
    /// the end, like [`Vec::push`]. So the index of `handle` must equal
    /// [`self.len()`].
    ///
    /// [`HashMap`]: hashbrown::HashMap
    /// [`self.len()`]: HandleVec::len
    pub(crate) fn insert(&mut self, handle: Handle<T>, value: U) {
        assert_eq!(handle.index(), self.inner.len());
        self.inner.push(value);
    }

    pub(crate) fn get(&self, handle: Handle<T>) -> Option<&U> {
        self.inner.get(handle.index())
    }

    pub(crate) fn clear(&mut self) {
        self.inner.clear()
    }

    pub(crate) fn resize(&mut self, len: usize, fill: U)
    where
        U: Clone,
    {
        self.inner.resize(len, fill);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &U> {
        self.inner.iter()
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut U> {
        self.inner.iter_mut()
    }
}

impl<T, U> ops::Index<Handle<T>> for HandleVec<T, U> {
    type Output = U;

    fn index(&self, handle: Handle<T>) -> &Self::Output {
        &self.inner[handle.index()]
    }
}

impl<T, U> ops::IndexMut<Handle<T>> for HandleVec<T, U> {
    fn index_mut(&mut self, handle: Handle<T>) -> &mut Self::Output {
        &mut self.inner[handle.index()]
    }
}
