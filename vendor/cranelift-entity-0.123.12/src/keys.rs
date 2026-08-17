//! A double-ended iterator over entity references.
//!
//! When `core::iter::Step` is stabilized, `Keys` could be implemented as a wrapper around
//! `core::ops::Range`, but for now, we implement it manually.

use crate::EntityRef;
use core::marker::PhantomData;

/// Iterate over all keys in order.
pub struct Keys<K: EntityRef> {
    pos: usize,
    rev_pos: usize,
    unused: PhantomData<K>,
}

impl<K: EntityRef> Keys<K> {
    /// Create a `Keys` iterator that visits `len` entities starting from 0.
    pub fn with_len(len: usize) -> Self {
        Self {
            pos: 0,
            rev_pos: len,
            unused: PhantomData,
        }
    }
}

impl<K: EntityRef> Iterator for Keys<K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.rev_pos {
            let k = K::new(self.pos);
            self.pos += 1;
            Some(k)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.rev_pos - self.pos;
        (size, Some(size))
    }
}

impl<K: EntityRef> DoubleEndedIterator for Keys<K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.rev_pos > self.pos {
            let k = K::new(self.rev_pos - 1);
            self.rev_pos -= 1;
            Some(k)
        } else {
            None
        }
    }
}

impl<K: EntityRef> ExactSizeIterator for Keys<K> {}
