//! Utilities for the `NSEnumerator` class.
use objc2::rc::Retained;
use objc2::Message;

use crate::{iter, NSEnumerator};

// TODO: Measure whether iterating through `nextObject` or fast enumeration is
// fastest.
// impl<ObjectType: Message> Iterator for NSEnumerator<ObjectType> {
//     type Item = Retained<ObjectType>;
//
//     #[inline]
//     fn next(&mut self) -> Option<Retained<ObjectType>> {
//         self.nextObject()
//     }
// }

impl<ObjectType: Message> NSEnumerator<ObjectType> {
    /// Iterate over the enumerator's elements.
    #[inline]
    pub fn iter(&self) -> Iter<'_, ObjectType> {
        Iter(iter::Iter::new(self))
    }

    /// Iterate over the enumerator without retaining the elements.
    ///
    /// Consider using the [`iter`](Self::iter) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The enumerator and the underlying collection must not be mutated while
    /// the iterator is alive.
    #[inline]
    pub unsafe fn iter_unchecked(&self) -> IterUnchecked<'_, ObjectType> {
        IterUnchecked(iter::IterUnchecked::new(self))
    }
}

unsafe impl<ObjectType: Message> iter::FastEnumerationHelper for NSEnumerator<ObjectType> {
    type Item = ObjectType;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        None
    }
}

/// An iterator over the items in an enumerator.
#[derive(Debug)]
pub struct Iter<'a, ObjectType: Message>(iter::Iter<'a, NSEnumerator<ObjectType>>);

__impl_iter! {
    impl<'a, ObjectType: Message> Iterator<Item = Retained<ObjectType>> for Iter<'a, ObjectType> { ... }
}

/// An iterator over unretained items in an enumerator.
///
/// # Safety
///
/// The enumerator and the underlying collection must not be mutated while
/// this is alive.
#[derive(Debug)]
pub struct IterUnchecked<'a, ObjectType: Message + 'a>(
    iter::IterUnchecked<'a, NSEnumerator<ObjectType>>,
);

__impl_iter! {
    impl<'a, ObjectType: Message> Iterator<Item = &'a ObjectType> for IterUnchecked<'a, ObjectType> { ... }
}

/// A consuming iterator over the items in an enumerator.
#[derive(Debug)]
pub struct IntoIter<ObjectType: Message>(iter::IntoIter<NSEnumerator<ObjectType>>);

__impl_iter! {
    impl<ObjectType: Message> Iterator<Item = Retained<ObjectType>> for IntoIter<ObjectType> { ... }
}

__impl_into_iter! {
    impl<ObjectType: Message> IntoIterator for &NSEnumerator<ObjectType> {
        type IntoIter = Iter<'_, ObjectType>;
    }

    impl<ObjectType: Message> IntoIterator for Retained<NSEnumerator<ObjectType>> {
        #[uses(new)]
        type IntoIter = IntoIter<ObjectType>;
    }
}

// TODO: Does fast enumeration modify the enumeration while iterating?
