//! Utilities for the `NSSet` and `NSMutableSet` classes.
use alloc::vec::Vec;
#[cfg(feature = "NSEnumerator")]
use core::fmt;

use objc2::rc::{Retained, RetainedFromIterator};
use objc2::{msg_send, AnyThread, Message};

#[cfg(feature = "NSEnumerator")]
use crate::iter;
use crate::{util, NSMutableSet, NSSet};

/// Convenience creation methods.
impl<ObjectType: Message> NSSet<ObjectType> {
    /// Creates an [`NSSet`] from a slice of `Retained`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSSet::from_retained_slice(&strs);
    /// ```
    pub fn from_retained_slice(slice: &[Retained<ObjectType>]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_retained_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&ObjectType]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }
}

/// Convenience creation methods.
impl<ObjectType: Message> NSMutableSet<ObjectType> {
    /// Creates an [`NSMutableSet`] from a slice of `Retained`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSMutableSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSMutableSet::from_retained_slice(&strs);
    /// ```
    pub fn from_retained_slice(slice: &[Retained<ObjectType>]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_retained_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&ObjectType]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }
}

/// Direct, unsafe object accessors.
///
/// Foundation's collection types store their items in such a way that they
/// can give out references to their data without having to autorelease it
/// first, see [the docs][collections-own].
///
/// This means that we can more efficiently access the set's objects, but
/// _only_ if the set isn't mutated via e.g. `NSMutableSet` methods while
/// doing so - otherwise, we might end up accessing a deallocated object.
///
/// [collections-own]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmPractical.html#//apple_ref/doc/uid/TP40004447-SW12
impl<ObjectType: Message> NSSet<ObjectType> {
    /// A direct reference to an arbitrary object in the set.
    ///
    /// Consider using the [`anyObject`](Self::anyObject) method instead,
    /// unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The set must not be mutated while the reference is live.
    #[doc(alias = "anyObject")]
    pub unsafe fn anyObject_unchecked(&self) -> Option<&ObjectType> {
        // SAFETY: Upheld by caller.
        unsafe { msg_send![self, anyObject] }
    }

    /// A direct reference the member that matches the given object.
    ///
    /// Consider using the [`member`](Self::member) method instead,
    /// unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The set must not be mutated while the returned reference is live.
    #[doc(alias = "member:")]
    pub unsafe fn member_unchecked(&self, object: &ObjectType) -> Option<&ObjectType> {
        // SAFETY: Upheld by caller.
        unsafe { msg_send![self, member: object] }
    }

    /// Iterate over the set in an arbitrary order without retaining the
    /// elements.
    ///
    /// Consider using the [`iter`](Self::iter) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The set must not be mutated for the lifetime of the iterator, or the
    /// elements it returns.
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub unsafe fn iter_unchecked(&self) -> IterUnchecked<'_, ObjectType> {
        IterUnchecked(super::iter::IterUnchecked::new(self))
    }
}

/// Various accessor methods.
impl<ObjectType: Message> NSSet<ObjectType> {
    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{ns_string, NSSet};
    ///
    /// let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
    /// let set = NSSet::from_slice(&strs);
    /// assert_eq!(set.len(), 3);
    /// ```
    #[doc(alias = "count")]
    pub fn len(&self) -> usize {
        self.count()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSSet, NSObject};
    ///
    /// let set = NSSet::<NSObject>::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// An iterator visiting all elements in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{ns_string, NSSet};
    ///
    /// let set = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("three")]);
    /// for s in set.iter() {
    ///     println!("{s}");
    /// }
    /// ```
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, ObjectType> {
        Iter(super::iter::Iter::new(self))
    }

    /// Returns a [`Vec`] containing the set's elements in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSSet, NSString};
    ///
    /// let strs = [
    ///     NSString::from_str("one"),
    ///     NSString::from_str("two"),
    ///     NSString::from_str("three"),
    /// ];
    /// let set = NSSet::from_retained_slice(&strs);
    /// let vec = set.to_vec();
    /// assert_eq!(vec.len(), 3);
    /// ```
    #[cfg(feature = "NSEnumerator")]
    pub fn to_vec(&self) -> Vec<Retained<ObjectType>> {
        self.iter().collect()
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: Message> iter::FastEnumerationHelper for NSSet<ObjectType> {
    type Item = ObjectType;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: Message> iter::FastEnumerationHelper for NSMutableSet<ObjectType> {
    type Item = ObjectType;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An iterator over the items of a set.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct Iter<'a, ObjectType: Message>(iter::Iter<'a, NSSet<ObjectType>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, ObjectType: Message> Iterator<Item = Retained<ObjectType>> for Iter<'a, ObjectType> { ... }
}

/// An unchecked iterator over the items of a set.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IterUnchecked<'a, ObjectType: Message>(iter::IterUnchecked<'a, NSSet<ObjectType>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, ObjectType: Message> Iterator<Item = &'a ObjectType> for IterUnchecked<'a, ObjectType> { ... }
}

/// An iterator over unretained items of a set.
///
/// # Safety
///
/// The set must not be mutated while this is alive.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IntoIter<ObjectType: Message>(iter::IntoIter<NSSet<ObjectType>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, ObjectType: Message> Iterator<Item = Retained<ObjectType>> for IntoIter<ObjectType> { ... }
}

#[cfg(feature = "NSEnumerator")]
__impl_into_iter! {
    impl<ObjectType: Message> IntoIterator for &NSSet<ObjectType> {
        type IntoIter = Iter<'_, ObjectType>;
    }

    impl<ObjectType: Message> IntoIterator for &NSMutableSet<ObjectType> {
        type IntoIter = Iter<'_, ObjectType>;
    }

    impl<ObjectType: Message> IntoIterator for Retained<NSSet<ObjectType>> {
        #[uses(new)]
        type IntoIter = IntoIter<ObjectType>;
    }

    impl<ObjectType: Message> IntoIterator for Retained<NSMutableSet<ObjectType>> {
        #[uses(new_mutable)]
        type IntoIter = IntoIter<ObjectType>;
    }
}

#[cfg(feature = "NSEnumerator")]
impl<ObjectType: fmt::Debug + Message> fmt::Debug for NSSet<ObjectType> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self).finish()
    }
}

#[cfg(feature = "NSEnumerator")]
impl<ObjectType: fmt::Debug + Message> fmt::Debug for NSMutableSet<ObjectType> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(feature = "NSEnumerator")]
impl<ObjectType: fmt::Debug + Message> fmt::Debug for crate::NSCountedSet<ObjectType> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<ObjectType: Message> Extend<Retained<ObjectType>> for &NSMutableSet<ObjectType> {
    fn extend<I: IntoIterator<Item = Retained<ObjectType>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| {
            self.addObject(&item);
        });
    }
}

impl<'a, ObjectType: Message> Extend<&'a ObjectType> for &NSMutableSet<ObjectType> {
    fn extend<I: IntoIterator<Item = &'a ObjectType>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| {
            self.addObject(item);
        });
    }
}

impl<'a, ObjectType: Message + 'a> RetainedFromIterator<&'a ObjectType> for NSSet<ObjectType> {
    fn retained_from_iter<I: IntoIterator<Item = &'a ObjectType>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<ObjectType: Message> RetainedFromIterator<Retained<ObjectType>> for NSSet<ObjectType> {
    fn retained_from_iter<I: IntoIterator<Item = Retained<ObjectType>>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_retained_slice(&vec)
    }
}

impl<'a, ObjectType: Message + 'a> RetainedFromIterator<&'a ObjectType>
    for NSMutableSet<ObjectType>
{
    fn retained_from_iter<I: IntoIterator<Item = &'a ObjectType>>(iter: I) -> Retained<Self> {
        // TODO: Is this, or is using `initWithCapacity` the most optimal?
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<ObjectType: Message> RetainedFromIterator<Retained<ObjectType>> for NSMutableSet<ObjectType> {
    fn retained_from_iter<I: IntoIterator<Item = Retained<ObjectType>>>(iter: I) -> Retained<Self> {
        // TODO: Is this, or is using `initWithCapacity` the most optimal?
        let vec = Vec::from_iter(iter);
        Self::from_retained_slice(&vec)
    }
}
