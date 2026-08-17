//! Utilities for the `NSEnumerator` class.
use objc2::rc::Retained;
use objc2::Message;

use super::iter;
use crate::Foundation::NSEnumerator;

// TODO: Measure whether iterating through `nextObject` or fast enumeration is
// fastest.
// impl<T: Message> Iterator for NSEnumerator<T> {
//     type Item = Retained<T>;
//
//     #[inline]
//     fn next(&mut self) -> Option<Retained<T>> {
//         self.nextObject()
//     }
// }

unsafe impl<T: Message> iter::FastEnumerationHelper for NSEnumerator<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        None
    }
}

/// A consuming iterator over the items of a `NSEnumerator`.
#[derive(Debug)]
pub struct IntoIter<T: Message>(iter::IntoIter<NSEnumerator<T>>);

__impl_iter! {
    impl<'a, T: Message> Iterator<Item = Retained<T>> for IntoIter<T> { ... }
}

impl<T: Message> objc2::rc::RetainedIntoIterator for NSEnumerator<T> {
    type Item = Retained<T>;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn id_into_iter(this: Retained<Self>) -> Self::IntoIter {
        IntoIter(super::iter::IntoIter::new(this))
    }
}

// TODO: Does fast enumeration modify the enumeration while iterating?
