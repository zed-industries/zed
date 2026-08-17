//! Module containing the [`WeakVec`] API.

use alloc::{sync::Weak, vec::Vec};

/// An optimized container for `Weak` references of `T` that minimizes reallocations by
/// dropping older elements that no longer have strong references to them.
#[derive(Debug)]
pub(crate) struct WeakVec<T> {
    inner: Vec<Weak<T>>,
}

impl<T> Default for WeakVec<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> WeakVec<T> {
    pub(crate) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Pushes a new element to this collection.
    ///
    /// If the inner Vec needs to be reallocated, we will first drop older elements that
    /// no longer have strong references to them.
    pub(crate) fn push(&mut self, value: Weak<T>) {
        if self.inner.len() == self.inner.capacity() {
            // Iterating backwards has the advantage that we don't do more work than we have to.
            for i in (0..self.inner.len()).rev() {
                if self.inner[i].strong_count() == 0 {
                    self.inner.swap_remove(i);
                }
            }

            // Make sure our capacity is twice the number of live elements.
            // Leaving some spare capacity ensures that we won't re-scan immediately.
            self.inner.reserve_exact(self.inner.len());
        }

        self.inner.push(value);
    }
}

pub(crate) struct WeakVecIter<T> {
    inner: alloc::vec::IntoIter<Weak<T>>,
}

impl<T> Iterator for WeakVecIter<T> {
    type Item = Weak<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> IntoIterator for WeakVec<T> {
    type Item = Weak<T>;
    type IntoIter = WeakVecIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        WeakVecIter {
            inner: self.inner.into_iter(),
        }
    }
}
