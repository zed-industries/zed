//! Utilities for the `NSArray` and `NSMutableArray` classes.
use alloc::vec::Vec;
#[cfg(feature = "NSEnumerator")]
use core::fmt;
#[cfg(feature = "NSRange")]
use core::ops::Range;
use core::ops::{Index, IndexMut};

use objc2::mutability::{IsIdCloneable, IsMutable, IsRetainable};
use objc2::rc::{Retained, RetainedFromIterator};
use objc2::{extern_methods, ClassType, Message};

#[cfg(feature = "NSEnumerator")]
use super::iter;
use super::util;
use crate::Foundation::{NSArray, NSMutableArray};

impl<T: Message> NSArray<T> {
    pub fn from_vec(mut vec: Vec<Retained<T>>) -> Retained<Self> {
        // We intentionally extract the length before we access the
        // pointer as mutable, to not invalidate that mutable pointer.
        let len = vec.len();
        let ptr = util::retained_ptr_cast(vec.as_mut_ptr());
        // SAFETY: We've consumed the `Retained<T>`s, which means that we can
        // now safely take ownership (even if `T` is mutable).
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
        // The drop of `Vec` here would invalidate our mutable pointer,
        // except for the fact that we're using `UnsafeCell` in `AnyObject`.
    }

    pub fn from_id_slice(slice: &[Retained<T>]) -> Retained<Self>
    where
        T: IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Because of the `T: IsIdCloneable` bound, and since we
        // take `&[Retained<T>]` (effectively `&Retained<T>`), we are allowed to give
        // the slice to Objective-C, which will retain it internally.
        //
        // Faster version of:
        //     Self::from_vec(slice.iter().map(|obj| obj.clone()).collect())
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Retained<Self>
    where
        T: IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Because of the `T: IsRetainable` bound, we are allowed
        // to give the slice to Objective-C, which will retain it
        // internally.
        //
        // Faster version of:
        //     Self::from_vec(slice.iter().map(|obj| obj.retain()).collect())
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    #[doc(alias = "getObjects:range:")]
    #[cfg(feature = "NSRange")]
    pub fn to_vec(&self) -> Vec<&T> {
        // SAFETY: The range is know to be in bounds
        unsafe { self.objects_in_range_unchecked(0..self.len()) }
    }

    #[doc(alias = "getObjects:range:")]
    #[cfg(feature = "NSRange")]
    pub fn to_vec_retained(&self) -> Vec<Retained<T>>
    where
        T: IsIdCloneable,
    {
        // SAFETY: The objects are stored in the array
        self.to_vec()
            .into_iter()
            .map(|obj| unsafe { util::collection_retain(obj) })
            .collect()
    }

    // `fn into_vec(Retained<NSArray>) -> Vec<Retained<T>>` would not be safe, since
    // the array itself is unconditionally `IsIdCloneable`, even when
    // containing mutable elements, and hence we would be able to
    // duplicate those.
}

impl<T: Message> NSMutableArray<T> {
    pub fn from_vec(mut vec: Vec<Retained<T>>) -> Retained<Self> {
        let len = vec.len();
        let ptr = util::retained_ptr_cast(vec.as_mut_ptr());
        // SAFETY: Same as `NSArray::from_vec`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_id_slice(slice: &[Retained<T>]) -> Retained<Self>
    where
        T: IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_id_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Retained<Self>
    where
        T: IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    #[cfg(feature = "NSRange")]
    pub fn into_vec(array: Retained<Self>) -> Vec<Retained<T>> {
        // SAFETY: We've consumed the array, so taking ownership of the
        // returned values is safe.
        array
            .to_vec()
            .into_iter()
            .map(|obj| unsafe { util::mutable_collection_retain_removed(obj) })
            .collect()
    }
}

impl<T: Message> NSArray<T> {
    #[doc(alias = "count")]
    pub fn len(&self) -> usize {
        self.count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

extern_methods!(
    unsafe impl<T: Message> NSArray<T> {
        #[method(objectAtIndex:)]
        unsafe fn get_unchecked(&self, index: usize) -> &T;

        #[doc(alias = "objectAtIndex:")]
        pub fn get(&self, index: usize) -> Option<&T> {
            // TODO: Replace this check with catching the thrown NSRangeException
            if index < self.len() {
                // SAFETY: The index is checked to be in bounds.
                Some(unsafe { self.get_unchecked(index) })
            } else {
                None
            }
        }

        #[doc(alias = "objectAtIndex:")]
        pub fn get_retained(&self, index: usize) -> Option<Retained<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.get(index)
                .map(|obj| unsafe { util::collection_retain(obj) })
        }

        #[method(objectAtIndex:)]
        unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T;

        #[doc(alias = "objectAtIndex:")]
        pub fn get_mut(&mut self, index: usize) -> Option<&mut T>
        where
            T: IsMutable,
        {
            // TODO: Replace this check with catching the thrown NSRangeException
            if index < self.len() {
                // SAFETY: The index is checked to be in bounds, and the
                // reference is safe as mutable because of the `T: IsMutable`
                // bound.
                Some(unsafe { self.get_unchecked_mut(index) })
            } else {
                None
            }
        }

        #[doc(alias = "firstObject")]
        #[method(firstObject)]
        pub fn first(&self) -> Option<&T>;

        #[doc(alias = "firstObject")]
        pub fn first_retained(&self) -> Option<Retained<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.first()
                .map(|obj| unsafe { util::collection_retain(obj) })
        }

        #[doc(alias = "firstObject")]
        #[method(firstObject)]
        pub fn first_mut(&mut self) -> Option<&mut T>
        where
            T: IsMutable;

        #[doc(alias = "lastObject")]
        #[method(lastObject)]
        pub fn last(&self) -> Option<&T>;

        #[doc(alias = "lastObject")]
        pub fn last_retained(&self) -> Option<Retained<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.last()
                .map(|obj| unsafe { util::collection_retain(obj) })
        }

        #[doc(alias = "lastObject")]
        #[method(lastObject)]
        pub fn last_mut(&mut self) -> Option<&mut T>
        where
            T: IsMutable;
    }
);

impl<T: Message> NSArray<T> {
    #[cfg(feature = "NSRange")]
    unsafe fn objects_in_range_unchecked(&self, range: Range<usize>) -> Vec<&T> {
        let range = crate::Foundation::NSRange::from(range);
        let mut vec: Vec<core::ptr::NonNull<T>> = Vec::with_capacity(range.length);
        unsafe {
            self.getObjects_range(core::ptr::NonNull::new(vec.as_mut_ptr()).unwrap(), range);
            vec.set_len(range.length);
            core::mem::transmute(vec)
        }
    }

    #[doc(alias = "getObjects:range:")]
    #[cfg(feature = "NSRange")]
    pub fn objects_in_range(&self, range: Range<usize>) -> Option<Vec<&T>> {
        if range.end > self.len() {
            return None;
        }
        // SAFETY: Just checked that the range is in bounds
        Some(unsafe { self.objects_in_range_unchecked(range) })
    }
}

impl<T: Message> NSMutableArray<T> {
    #[doc(alias = "addObject:")]
    pub fn push(&mut self, obj: Retained<T>) {
        // SAFETY: We've consumed ownership of the object.
        unsafe { self.addObject(&obj) }
    }

    #[doc(alias = "insertObject:atIndex:")]
    pub fn insert(&mut self, index: usize, obj: Retained<T>) {
        // TODO: Replace this check with catching the thrown NSRangeException
        let len = self.len();
        if index < len {
            // SAFETY: We've consumed ownership of the object, and the
            // index is checked to be in bounds.
            unsafe { self.insertObject_atIndex(&obj, index) }
        } else {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, len
            );
        }
    }

    #[doc(alias = "replaceObjectAtIndex:withObject:")]
    pub fn replace(&mut self, index: usize, obj: Retained<T>) -> Result<Retained<T>, Retained<T>> {
        if let Some(old_obj) = self.get(index) {
            // SAFETY: We remove the object from the array below.
            let old_obj = unsafe { util::mutable_collection_retain_removed(old_obj) };
            // SAFETY: The index is checked to be in bounds, and we've
            // consumed ownership of the new object.
            unsafe { self.replaceObjectAtIndex_withObject(index, &obj) };
            Ok(old_obj)
        } else {
            Err(obj)
        }
    }

    #[doc(alias = "removeObjectAtIndex:")]
    pub fn remove(&mut self, index: usize) -> Option<Retained<T>> {
        let obj = self.get(index)?;
        // SAFETY: We remove the object from the array below.
        let obj = unsafe { util::mutable_collection_retain_removed(obj) };
        // SAFETY: The index is checked to be in bounds.
        unsafe { self.removeObjectAtIndex(index) };
        Some(obj)
    }

    #[doc(alias = "removeLastObject")]
    pub fn pop(&mut self) -> Option<Retained<T>> {
        let obj = self.last()?;
        // SAFETY: We remove the object from the array below.
        let obj = unsafe { util::mutable_collection_retain_removed(obj) };
        // SAFETY: Just checked that there is an object.
        unsafe { self.removeLastObject() };
        Some(obj)
    }

    #[cfg(feature = "NSObjCRuntime")]
    #[doc(alias = "sortUsingFunction:context:")]
    pub fn sort_by<F: FnMut(&T, &T) -> core::cmp::Ordering>(&mut self, compare: F) {
        // TODO: "C-unwind"
        unsafe extern "C" fn compare_with_closure<T, F: FnMut(&T, &T) -> core::cmp::Ordering>(
            obj1: core::ptr::NonNull<T>,
            obj2: core::ptr::NonNull<T>,
            context: *mut std::os::raw::c_void,
        ) -> isize {
            let context: *mut F = context.cast();
            // Bring back a reference to the closure.
            // Guaranteed to be unique, we gave `sortUsingFunction` unique is
            // ownership, and that method only runs one function at a time.
            let closure: &mut F = unsafe { context.as_mut().unwrap_unchecked() };

            // SAFETY: The objects are guaranteed to be valid
            let (obj1, obj2) = unsafe { (obj1.as_ref(), obj2.as_ref()) };

            crate::Foundation::NSComparisonResult::from((*closure)(obj1, obj2)) as _
        }

        // Create function pointer
        let f: unsafe extern "C" fn(_, _, _) -> _ = compare_with_closure::<T, F>;

        // Grab a type-erased pointer to the closure (a pointer to stack).
        let mut closure = compare;
        let context: *mut F = &mut closure;

        unsafe { self.sortUsingFunction_context(f, context.cast()) };
        // Keep the closure alive until the function has run.
        drop(closure);
    }
}

impl<T: Message> NSArray<T> {
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(super::iter::Iter::new(self))
    }

    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T>
    where
        T: IsMutable,
    {
        IterMut(super::iter::IterMut::new(self))
    }

    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter_retained(&self) -> IterRetained<'_, T>
    where
        T: IsIdCloneable,
    {
        IterRetained(super::iter::IterRetained::new(self))
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<T: Message> iter::FastEnumerationHelper for NSArray<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<T: Message> iter::FastEnumerationHelper for NSMutableArray<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An iterator over the items of a `NSArray`.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct Iter<'a, T: Message>(iter::Iter<'a, NSArray<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message> Iterator<Item = &'a T> for Iter<'a, T> { ... }
}

/// A mutable iterator over the items of a `NSArray`.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IterMut<'a, T: Message>(iter::IterMut<'a, NSArray<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message + IsMutable> Iterator<Item = &'a mut T> for IterMut<'a, T> { ... }
}

/// An iterator that retains the items of a `NSArray`.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IterRetained<'a, T: Message>(iter::IterRetained<'a, NSArray<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message + IsIdCloneable> Iterator<Item = Retained<T>> for IterRetained<'a, T> { ... }
}

/// A consuming iterator over the items of a `NSArray`.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IntoIter<T: Message>(iter::IntoIter<NSArray<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message> Iterator<Item = Retained<T>> for IntoIter<T> { ... }
}

#[cfg(feature = "NSEnumerator")]
__impl_into_iter! {
    impl<T: Message> IntoIterator for &NSArray<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: Message> IntoIterator for &NSMutableArray<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: Message + IsMutable> IntoIterator for &mut NSArray<T> {
        type IntoIter = IterMut<'_, T>;
    }

    impl<T: Message + IsMutable> IntoIterator for &mut NSMutableArray<T> {
        type IntoIter = IterMut<'_, T>;
    }

    impl<T: Message + IsIdCloneable> IntoIterator for Retained<NSArray<T>> {
        type IntoIter = IntoIter<T>;
    }

    impl<T: Message> IntoIterator for Retained<NSMutableArray<T>> {
        type IntoIter = IntoIter<T>;
    }
}

impl<T: Message> Index<usize> for NSArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: Message> Index<usize> for NSMutableArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: Message + IsMutable> IndexMut<usize> for NSArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: Message + IsMutable> IndexMut<usize> for NSMutableArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

#[cfg(feature = "NSEnumerator")]
impl<T: fmt::Debug + Message> fmt::Debug for NSArray<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[cfg(feature = "NSEnumerator")]
impl<T: fmt::Debug + Message> fmt::Debug for NSMutableArray<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: Message> Extend<Retained<T>> for NSMutableArray<T> {
    fn extend<I: IntoIterator<Item = Retained<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| self.push(item));
    }
}

impl<'a, T: Message + IsRetainable> Extend<&'a T> for NSMutableArray<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        // SAFETY: Because of the `T: IsRetainable` bound, it is safe for the
        // array to retain the object here.
        iter.into_iter()
            .for_each(move |item| unsafe { self.addObject(item) });
    }
}

impl<'a, T: Message + IsRetainable + 'a> RetainedFromIterator<&'a T> for NSArray<T> {
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<T: Message> RetainedFromIterator<Retained<T>> for NSArray<T> {
    fn id_from_iter<I: IntoIterator<Item = Retained<T>>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}

impl<'a, T: Message + IsRetainable + 'a> RetainedFromIterator<&'a T> for NSMutableArray<T> {
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<T: Message> RetainedFromIterator<Retained<T>> for NSMutableArray<T> {
    fn id_from_iter<I: IntoIterator<Item = Retained<T>>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}
