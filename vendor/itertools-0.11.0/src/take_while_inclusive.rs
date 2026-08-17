use core::iter::FusedIterator;
use std::fmt;

/// An iterator adaptor that consumes elements while the given predicate is
/// `true`, including the element for which the predicate first returned
/// `false`.
///
/// See [`.take_while_inclusive()`](crate::Itertools::take_while_inclusive)
/// for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct TakeWhileInclusive<'a, I: 'a, F> {
    iter: &'a mut I,
    predicate: F,
    done: bool,
}

impl<'a, I, F> TakeWhileInclusive<'a, I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    /// Create a new [`TakeWhileInclusive`] from an iterator and a predicate.
    pub fn new(iter: &'a mut I, predicate: F) -> Self {
        Self { iter, predicate, done: false}
    }
}

impl<'a, I, F> fmt::Debug for TakeWhileInclusive<'a, I, F>
    where I: Iterator + fmt::Debug,
{
    debug_fmt_fields!(TakeWhileInclusive, iter);
}

impl<'a, I, F> Iterator for TakeWhileInclusive<'a, I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.iter.next().map(|item| {
                if !(self.predicate)(&item) {
                    self.done = true;
                }
                item
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            (0, Some(0))
        } else {
            (0, self.iter.size_hint().1)
        }
    }
}

impl<I, F> FusedIterator for TakeWhileInclusive<'_, I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool
{
}