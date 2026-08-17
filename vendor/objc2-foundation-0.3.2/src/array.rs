//! Utilities for the `NSArray` and `NSMutableArray` classes.
use alloc::vec::Vec;
#[cfg(feature = "NSEnumerator")]
use core::fmt;
use core::mem;
use core::ptr::NonNull;

use objc2::rc::{Retained, RetainedFromIterator};
use objc2::{msg_send, AnyThread, Message};

#[cfg(feature = "NSEnumerator")]
use crate::iter;
use crate::{util, NSArray, NSMutableArray};

/// Convenience creation methods.
impl<ObjectType: Message> NSArray<ObjectType> {
    /// Create a new array from a slice of objects.
    ///
    /// This is a safe interface to `initWithObjects:count:`.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSArray, ns_string};
    ///
    /// let array = NSArray::from_slice(&[
    ///     ns_string!("abc"),
    ///     ns_string!("def"),
    ///     ns_string!("ghi"),
    /// ]);
    /// ```
    #[doc(alias = "initWithObjects:count:")]
    pub fn from_slice(slice: &[&ObjectType]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY:
        // - All `ObjectType: Message` use interior mutability, and the array
        //   extends the lifetime of them internally by retaining them.
        // - The pointer and length are valid until the method has finished
        //   executing, at which point the array will have created its own
        //   internal storage for holding the pointers.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Create a new array from a slice of retained objects.
    ///
    /// This is a safe interface to `initWithObjects:count:`.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSArray, NSObject};
    ///
    /// let array = NSArray::from_retained_slice(&[
    ///     NSObject::new(),
    ///     NSObject::new(),
    ///     NSObject::new(),
    /// ]);
    /// ```
    #[doc(alias = "initWithObjects:count:")]
    pub fn from_retained_slice(slice: &[Retained<ObjectType>]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `from_slice`, this is just a faster version to
        // avoid creating a new slice if your elements are already retained.
        //
        // Otherwise equivalent to:
        //     Self::from_slice(&slice.iter().map(|obj| &*obj).collect())
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }
}

/// Convenience creation methods.
impl<ObjectType: Message> NSMutableArray<ObjectType> {
    #[doc(alias = "initWithObjects:count:")]
    pub fn from_slice(slice: &[&ObjectType]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    #[doc(alias = "initWithObjects:count:")]
    pub fn from_retained_slice(slice: &[Retained<ObjectType>]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_retained_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }
}

/// Direct, unsafe object accessors.
///
/// Foundation's collection types store their items in such a way that they
/// can give out references to their data without having to autorelease it
/// first, see [the docs][collections-own].
///
/// This means that we can more efficiently access the array's objects, but
/// _only_ if the array isn't mutated via e.g. `NSMutableArray` methods while
/// doing so - otherwise, we might end up accessing a deallocated object.
///
/// [collections-own]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmPractical.html#//apple_ref/doc/uid/TP40004447-SW12
impl<ObjectType: Message> NSArray<ObjectType> {
    /// Get a direct reference to one of the array's objects.
    ///
    /// Throws an error if the object was not found.
    ///
    /// Consider using the [`objectAtIndex`](Self::objectAtIndex) method
    /// instead, unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the reference is live.
    #[doc(alias = "objectAtIndex:")]
    #[inline]
    pub unsafe fn objectAtIndex_unchecked(&self, index: usize) -> &ObjectType {
        // SAFETY: Upheld by caller.
        unsafe { msg_send![self, objectAtIndex: index] }
    }

    /// A direct reference to the array's first object, if any.
    ///
    /// Consider using the [`firstObject`](Self::firstObject) method instead,
    /// unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the reference is live.
    #[doc(alias = "firstObject")]
    #[inline]
    pub unsafe fn firstObject_unchecked(&self) -> Option<&ObjectType> {
        // SAFETY: Upheld by caller.
        unsafe { msg_send![self, firstObject] }
    }

    /// A direct reference to the array's last object, if any.
    ///
    /// Consider using the [`lastObject`](Self::lastObject) method instead,
    /// unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the reference is live.
    #[doc(alias = "lastObject")]
    #[inline]
    pub unsafe fn lastObject_unchecked(&self) -> Option<&ObjectType> {
        // SAFETY: Upheld by caller.
        unsafe { msg_send![self, lastObject] }
    }

    /// A vector containing direct references to the array's objects.
    ///
    /// Consider using the [`to_vec`](Self::to_vec) method instead, unless
    /// you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the returned references are alive.
    #[doc(alias = "getObjects:")]
    pub unsafe fn to_vec_unchecked(&self) -> Vec<&ObjectType> {
        let len = self.count();
        let mut vec: Vec<NonNull<ObjectType>> = Vec::with_capacity(len);
        let ptr: NonNull<NonNull<ObjectType>> = NonNull::new(vec.as_mut_ptr()).unwrap();

        // SAFETY: The buffer is at least the size of the array, as guaranteed
        // by `Vec::with_capacity`.
        unsafe {
            #[allow(deprecated)]
            self.getObjects(ptr)
        };

        // SAFETY: The elements were just initialized by `getObjects:`.
        //
        // Note: We set the length _after_ we've copied the elements, so that
        // if `getObjects:` unwinds, we don't end up deallocating
        // uninitialized elements.
        unsafe { vec.set_len(len) };

        // SAFETY: `NonNull<ObjectType>` has the same layout as `&ObjectType`,
        // and the lifetime is bound to the array, and caller upholds that the
        // array isn't mutated.
        unsafe { mem::transmute::<Vec<NonNull<ObjectType>>, Vec<&ObjectType>>(vec) }
    }

    /// Iterate over the array without retaining the elements.
    ///
    /// Consider using the [`iter`](Self::iter) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated for the lifetime of the iterator or for
    /// the lifetime of the elements the iterator returns.
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub unsafe fn iter_unchecked(&self) -> IterUnchecked<'_, ObjectType> {
        IterUnchecked(iter::IterUnchecked::new(self))
    }
}

/// Various accessor methods.
impl<ObjectType: Message> NSArray<ObjectType> {
    /// The amount of elements in the array.
    #[doc(alias = "count")]
    #[inline]
    pub fn len(&self) -> usize {
        self.count()
    }

    /// Whether the array is empty or not.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert the array to a `Vec` of the array's objects.
    #[doc(alias = "getObjects:")]
    pub fn to_vec(&self) -> Vec<Retained<ObjectType>> {
        // SAFETY: We retain the elements below, so we know that the array
        // isn't mutated while the references are alive.
        //
        // Note that this is _technically_ wrong; the user _could_ have
        // implemented a `retain` method that mutates the array. We're going
        // to rule this out though, as that's basically never going to happen,
        // and will make a lot of other things unsound too.
        let vec = unsafe { self.to_vec_unchecked() };
        vec.into_iter().map(ObjectType::retain).collect()
    }

    /// Iterate over the array's elements.
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, ObjectType> {
        Iter(iter::Iter::new(self))
    }

    /// Returns the objects within the given range.
    ///
    /// # Panics
    ///
    /// Panics if the range was out of bounds.
    #[doc(alias = "getObjects:range:")]
    #[cfg(feature = "NSRange")]
    pub fn objects_in_range(&self, range: core::ops::Range<usize>) -> Vec<Retained<ObjectType>> {
        let count = self.count();

        // TODO: Replace this check with catching the thrown NSRangeException
        if range.end > count {
            panic!(
                "range end index {} out of range for array of length {}",
                range.end, count
            );
        }

        let range = crate::NSRange::from(range);
        let mut vec: Vec<NonNull<ObjectType>> = Vec::with_capacity(range.length);
        let ptr: NonNull<NonNull<ObjectType>> = NonNull::new(vec.as_mut_ptr()).unwrap();

        // SAFETY: Mostly the same as in `to_vec_unchecked`.
        unsafe { self.getObjects_range(ptr, range) };
        unsafe { vec.set_len(range.length) };
        let vec = unsafe { mem::transmute::<Vec<NonNull<ObjectType>>, Vec<&ObjectType>>(vec) };

        vec.into_iter().map(ObjectType::retain).collect()
    }
}

/// Convenience mutation methods.
impl<ObjectType: Message> NSMutableArray<ObjectType> {
    /// Insert an object into the array at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[doc(alias = "insertObject:atIndex:")]
    pub fn insert(&self, index: usize, obj: &ObjectType) {
        // TODO: Replace this check with catching the thrown NSRangeException
        let len = self.len();
        if index <= len {
            self.insertObject_atIndex(obj, index)
        } else {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, len
            );
        }
    }

    /// Sort the array by the given comparison closure.
    #[cfg(feature = "NSObjCRuntime")]
    #[doc(alias = "sortUsingFunction:context:")]
    pub fn sort_by<F: FnMut(&ObjectType, &ObjectType) -> core::cmp::Ordering>(&self, compare: F) {
        unsafe extern "C-unwind" fn compare_with_closure<
            ObjectType,
            F: FnMut(&ObjectType, &ObjectType) -> core::cmp::Ordering,
        >(
            obj1: core::ptr::NonNull<ObjectType>,
            obj2: core::ptr::NonNull<ObjectType>,
            context: *mut core::ffi::c_void,
        ) -> isize {
            let context: *mut F = context.cast();
            // Bring back a reference to the closure.
            // Guaranteed to be unique, we gave `sortUsingFunction` unique
            // ownership, and that method only runs one function at a time.
            let closure: &mut F = unsafe { context.as_mut().unwrap_unchecked() };

            // SAFETY: The objects are guaranteed to be valid
            let (obj1, obj2) = unsafe { (obj1.as_ref(), obj2.as_ref()) };

            crate::NSComparisonResult::from((*closure)(obj1, obj2)) as _
        }

        // Create function pointer
        let f: unsafe extern "C-unwind" fn(_, _, _) -> _ = compare_with_closure::<ObjectType, F>;

        // Grab a type-erased pointer to the closure (a pointer to stack).
        let mut closure = compare;
        let context: *mut F = &mut closure;

        unsafe { self.sortUsingFunction_context(f, context.cast()) };
        // Keep the closure alive until the function has run.
        drop(closure);
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: Message> iter::FastEnumerationHelper for NSArray<ObjectType> {
    type Item = ObjectType;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: Message> iter::FastEnumerationHelper for NSMutableArray<ObjectType> {
    type Item = ObjectType;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An iterator over the items of an array.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct Iter<'a, ObjectType: Message>(iter::Iter<'a, NSArray<ObjectType>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, ObjectType: Message> Iterator<Item = Retained<ObjectType>> for Iter<'a, ObjectType> { ... }
}

/// An iterator over unretained items of an array.
///
/// # Safety
///
/// The array must not be mutated while this is alive.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IterUnchecked<'a, ObjectType: Message>(iter::IterUnchecked<'a, NSArray<ObjectType>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, ObjectType: Message> Iterator<Item = &'a ObjectType> for IterUnchecked<'a, ObjectType> { ... }
}

/// A retained iterator over the items of an array.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IntoIter<ObjectType: Message>(iter::IntoIter<NSArray<ObjectType>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<ObjectType: Message> Iterator<Item = Retained<ObjectType>> for IntoIter<ObjectType> { ... }
}

#[cfg(feature = "NSEnumerator")]
__impl_into_iter! {
    impl<ObjectType: Message> IntoIterator for &NSArray<ObjectType> {
        type IntoIter = Iter<'_, ObjectType>;
    }

    impl<ObjectType: Message> IntoIterator for &NSMutableArray<ObjectType> {
        type IntoIter = Iter<'_, ObjectType>;
    }

    impl<ObjectType: Message> IntoIterator for Retained<NSArray<ObjectType>> {
        #[uses(new)]
        type IntoIter = IntoIter<ObjectType>;
    }

    impl<ObjectType: Message> IntoIterator for Retained<NSMutableArray<ObjectType>> {
        #[uses(new_mutable)]
        type IntoIter = IntoIter<ObjectType>;
    }
}

#[cfg(feature = "NSEnumerator")]
impl<ObjectType: fmt::Debug + Message> fmt::Debug for NSArray<ObjectType> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[cfg(feature = "NSEnumerator")]
impl<ObjectType: fmt::Debug + Message> fmt::Debug for NSMutableArray<ObjectType> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<ObjectType: Message> Extend<Retained<ObjectType>> for &NSMutableArray<ObjectType> {
    fn extend<I: IntoIterator<Item = Retained<ObjectType>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| self.addObject(&item));
    }
}

impl<'a, ObjectType: Message> Extend<&'a ObjectType> for &NSMutableArray<ObjectType> {
    fn extend<I: IntoIterator<Item = &'a ObjectType>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| self.addObject(item));
    }
}

impl<'a, ObjectType: Message + 'a> RetainedFromIterator<&'a ObjectType> for NSArray<ObjectType> {
    fn retained_from_iter<I: IntoIterator<Item = &'a ObjectType>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<ObjectType: Message> RetainedFromIterator<Retained<ObjectType>> for NSArray<ObjectType> {
    fn retained_from_iter<I: IntoIterator<Item = Retained<ObjectType>>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_retained_slice(&vec)
    }
}

impl<'a, ObjectType: Message + 'a> RetainedFromIterator<&'a ObjectType>
    for NSMutableArray<ObjectType>
{
    fn retained_from_iter<I: IntoIterator<Item = &'a ObjectType>>(iter: I) -> Retained<Self> {
        // TODO: Is this, or is using `initWithCapacity` the most optimal?
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<ObjectType: Message> RetainedFromIterator<Retained<ObjectType>>
    for NSMutableArray<ObjectType>
{
    fn retained_from_iter<I: IntoIterator<Item = Retained<ObjectType>>>(iter: I) -> Retained<Self> {
        // TODO: Is this, or is using `initWithCapacity` the most optimal?
        let vec = Vec::from_iter(iter);
        Self::from_retained_slice(&vec)
    }
}
