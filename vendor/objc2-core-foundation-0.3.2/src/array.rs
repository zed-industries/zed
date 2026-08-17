#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::ffi::c_void;

use crate::{kCFTypeArrayCallBacks, CFArray, CFIndex, CFMutableArray, CFRetained, Type};

#[inline]
fn get_len<T>(objects: &[T]) -> CFIndex {
    // An allocation in Rust cannot be larger than isize::MAX, so this will
    // never fail.
    //
    // Note that `CFArray::new` documents:
    // > If this parameter is negative, [...] the behavior is undefined.
    let len = objects.len();
    debug_assert!(len < CFIndex::MAX as usize);
    len as CFIndex
}

/// Reading the source code:
/// <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFArray.c#L391>
/// <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFRuntime.c#L323>
///
/// It is clear that creating arrays can only realistically fail if allocating
/// failed. So we choose to panic/abort in those cases, to roughly match
/// `Vec`'s behaviour.
#[cold]
fn failed_creating_array(len: CFIndex) -> ! {
    #[cfg(feature = "alloc")]
    {
        use alloc::alloc::{handle_alloc_error, Layout};
        use core::mem::align_of;

        // The layout here is not correct, only best-effort (CFArray adds
        // extra padding when allocating).
        let layout = Layout::array::<*const ()>(len as usize).unwrap_or_else(|_| unsafe {
            Layout::from_size_align_unchecked(0, align_of::<*const ()>())
        });

        handle_alloc_error(layout)
    }
    #[cfg(not(feature = "alloc"))]
    {
        panic!("failed allocating CFArray holding {len} elements")
    }
}

/// Convenience creation methods.
impl<T: ?Sized> CFArray<T> {
    /// Create a new empty `CFArray` capable of holding CoreFoundation
    /// objects.
    #[inline]
    #[doc(alias = "CFArray::new")]
    pub fn empty() -> CFRetained<Self>
    where
        T: Type,
    {
        // It may not strictly be necessary to use correct array callbacks
        // here, though it's good to know that it's correct for use in e.g.
        // `CFMutableArray::newCopy`.
        Self::from_objects(&[])
    }

    /// Create a new `CFArray` with the given CoreFoundation objects.
    #[inline]
    #[doc(alias = "CFArray::new")]
    pub fn from_objects(objects: &[&T]) -> CFRetained<Self>
    where
        T: Type,
    {
        let len = get_len(objects);
        // `&T` has the same layout as `*const c_void`, and are non-NULL.
        let ptr = objects.as_ptr().cast::<*const c_void>().cast_mut();

        // SAFETY: The objects are CFTypes (`T: Type` bound), and the array
        // callbacks are thus correct.
        //
        // The objects are retained internally by the array, so we do not need
        // to keep them alive ourselves after this.
        let array = unsafe { CFArray::new(None, ptr, len, &kCFTypeArrayCallBacks) }
            .unwrap_or_else(|| failed_creating_array(len));

        // SAFETY: The objects came from `T`.
        unsafe { CFRetained::cast_unchecked::<Self>(array) }
    }

    /// Alias for easier transition from the `core-foundation` crate.
    #[inline]
    #[allow(non_snake_case)]
    #[deprecated = "renamed to CFArray::from_objects"]
    pub fn from_CFTypes(objects: &[&T]) -> CFRetained<Self>
    where
        T: Type,
    {
        Self::from_objects(objects)
    }

    /// Create a new `CFArray` with the given retained CoreFoundation objects.
    #[inline]
    #[doc(alias = "CFArray::new")]
    pub fn from_retained_objects(objects: &[CFRetained<T>]) -> CFRetained<Self>
    where
        T: Type,
    {
        let len = get_len(objects);
        // `CFRetained<T>` has the same layout as `*const c_void`.
        let ptr = objects.as_ptr().cast::<*const c_void>().cast_mut();

        // SAFETY: Same as in `from_objects`.
        let array = unsafe { CFArray::new(None, ptr, len, &kCFTypeArrayCallBacks) }
            .unwrap_or_else(|| failed_creating_array(len));

        // SAFETY: The objects came from `T`.
        unsafe { CFRetained::cast_unchecked::<Self>(array) }
    }
}

/// Convenience creation methods.
impl<T: ?Sized> CFMutableArray<T> {
    /// Create a new empty mutable array.
    #[inline]
    #[doc(alias = "CFMutableArray::new")]
    pub fn empty() -> CFRetained<Self>
    where
        T: Type,
    {
        Self::with_capacity(0)
    }

    /// Create a new mutable array with the given capacity.
    #[inline]
    #[doc(alias = "CFMutableArray::new")]
    pub fn with_capacity(capacity: usize) -> CFRetained<Self>
    where
        T: Type,
    {
        // User can pass wrong value here, we must check.
        let capacity = capacity.try_into().expect("capacity too high");

        // SAFETY: The objects are CFTypes (`T: Type` bound), and the array
        // callbacks are thus correct.
        let array = unsafe { CFMutableArray::new(None, capacity, &kCFTypeArrayCallBacks) }
            .unwrap_or_else(|| failed_creating_array(capacity));

        // SAFETY: The array contains no objects yet, and thus it's safe to
        // cast them to `T` (as the array callbacks are matching).
        unsafe { CFRetained::cast_unchecked::<Self>(array) }
    }
}

// TODO: Do we want to pass NULL callbacks or `CFArrayEqualCallBack`.
// impl CFArray<()> {
//     /// Create a new `CFArray` with the given retained CoreFoundation objects.
//     pub fn from_usize(objects: &[usize]) -> CFRetained<Self> {
//         let len = get_len(objects);
//         // `CFRetained<T>` has the same layout as `*const c_void`.
//         let ptr: *const c_void = objects.as_ptr().cast();
//
//         // SAFETY: Same as in `from_objects`.
//         let array = unsafe { CFArray::new(None, ptr, len, null) }
//             .unwrap_or(|| failed_creating_array(len));
//
//         // SAFETY: The objects came from `T`.
//         unsafe { CFRetained::cast_unchecked::<Self>(array) }
//     }
// }

/// Direct, unsafe object accessors.
///
/// CFArray stores its values directly, and you can get references to said
/// values data without having to retain it first - but only if the array
/// isn't mutated while doing so - otherwise, we might end up accessing a
/// deallocated object.
impl<T: ?Sized> CFArray<T> {
    /// Get a direct reference to one of the array's objects.
    ///
    /// Consider using the [`get`](Self::get) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// - The index must not be negative, and must be in bounds of the array.
    /// - The array must not be mutated while the returned reference is live.
    #[inline]
    #[doc(alias = "CFArrayGetValueAtIndex")]
    pub unsafe fn get_unchecked(&self, index: CFIndex) -> &T
    where
        T: Type + Sized,
    {
        // SAFETY: Caller ensures that `index` is in bounds.
        let ptr = unsafe { self.as_opaque().value_at_index(index) };
        // SAFETY: The array's values are of type `T`, and the objects are
        // CoreFoundation types (and thus cannot be NULL).
        //
        // Caller ensures that the array isn't mutated for the lifetime of the
        // reference.
        unsafe { &*ptr.cast::<T>() }
    }

    /// A vector containing direct references to the array's objects.
    ///
    /// Consider using the [`to_vec`](Self::to_vec) method instead, unless
    /// you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the returned references are alive.
    #[cfg(feature = "alloc")]
    #[doc(alias = "CFArrayGetValues")]
    pub unsafe fn to_vec_unchecked(&self) -> Vec<&T>
    where
        T: Type,
    {
        let len = self.len();
        let range = crate::CFRange {
            location: 0,
            // Fine to cast, it came from CFIndex
            length: len as CFIndex,
        };
        let mut vec = Vec::<&T>::with_capacity(len);

        // `&T` has the same layout as `*const c_void`.
        let ptr = vec.as_mut_ptr().cast::<*const c_void>();
        // SAFETY: The range is in bounds
        unsafe { self.as_opaque().values(range, ptr) };
        // SAFETY: Just initialized the Vec above.
        unsafe { vec.set_len(len) };

        vec
    }

    /// Iterate over the array without touching the elements.
    ///
    /// Consider using the [`iter`](Self::iter) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated for the lifetime of the iterator or for
    /// the lifetime of the elements the iterator returns.
    #[inline]
    pub unsafe fn iter_unchecked(&self) -> CFArrayIterUnchecked<'_, T>
    where
        T: Type,
    {
        CFArrayIterUnchecked {
            array: self,
            index: 0,
            len: self.len() as CFIndex,
        }
    }
}

/// Various accessor methods.
impl<T: ?Sized> CFArray<T> {
    /// The amount of elements in the array.
    #[inline]
    #[doc(alias = "CFArrayGetCount")]
    pub fn len(&self) -> usize {
        // Fine to cast here, the count is never negative.
        self.as_opaque().count() as _
    }

    /// Whether the array is empty or not.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retrieve the object at the given index.
    ///
    /// Returns `None` if the index was out of bounds.
    #[doc(alias = "CFArrayGetValueAtIndex")]
    pub fn get(&self, index: usize) -> Option<CFRetained<T>>
    where
        T: Type + Sized,
    {
        if index < self.len() {
            // Index is `usize` and just compared below the length (which is
            // at max `CFIndex::MAX`), so a cast is safe here.
            let index = index as CFIndex;
            // SAFETY:
            // - Just checked that the index is in bounds.
            // - We retain the value right away, so that the reference is not
            //   used while the array is mutated.
            //
            // Note that this is _technically_ wrong; the user _could_ have
            // implemented a `retain` method that mutates the array. We're
            // going to rule this out though, as that's basically never going
            // to happen, and will make a lot of other things unsound too.
            Some(unsafe { self.get_unchecked(index) }.retain())
        } else {
            None
        }
    }

    /// Convert the array to a `Vec` of the array's objects.
    #[cfg(feature = "alloc")]
    #[doc(alias = "CFArrayGetValues")]
    pub fn to_vec(&self) -> Vec<CFRetained<T>>
    where
        T: Type + Sized,
    {
        // SAFETY: We retain the elements below, so we know that the array
        // isn't mutated while the references are alive.
        let vec = unsafe { self.to_vec_unchecked() };
        vec.into_iter().map(T::retain).collect()
    }

    /// Iterate over the array's elements.
    #[inline]
    pub fn iter(&self) -> CFArrayIter<'_, T> {
        CFArrayIter {
            array: self,
            index: 0,
        }
    }
}

/// Convenience mutation methods.
impl<T> CFMutableArray<T> {
    /// Push an object to the end of the array.
    #[inline]
    #[doc(alias = "CFArrayAppendValue")]
    pub fn append(&self, obj: &T) {
        let ptr: *const T = obj;
        let ptr: *const c_void = ptr.cast();
        // SAFETY: The pointer is valid.
        unsafe { CFMutableArray::append_value(Some(self.as_opaque()), ptr) }
    }

    /// Insert an object into the array at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[doc(alias = "CFArrayInsertValueAtIndex")]
    pub fn insert(&self, index: usize, obj: &T) {
        // TODO: Replace this check with catching the thrown NSRangeException
        let len = self.len();
        if index <= len {
            let ptr: *const T = obj;
            let ptr: *const c_void = ptr.cast();
            // SAFETY: The pointer is valid, and just checked that the index
            // is in bounds.
            unsafe {
                CFMutableArray::insert_value_at_index(Some(self.as_opaque()), index as CFIndex, ptr)
            }
        } else {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, len
            );
        }
    }
}

/// An iterator over retained objects of an array.
#[derive(Debug)]
pub struct CFArrayIter<'a, T: ?Sized + 'a> {
    array: &'a CFArray<T>,
    index: usize,
}

impl<T: Type> Iterator for CFArrayIter<'_, T> {
    type Item = CFRetained<T>;

    fn next(&mut self) -> Option<CFRetained<T>> {
        // We _must_ re-check the length on every loop iteration, since the
        // array could have come from `CFMutableArray` and have been mutated
        // while we're iterating.
        let value = self.array.get(self.index)?;
        self.index += 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.len().saturating_sub(self.index);
        (len, Some(len))
    }
}

impl<T: Type> ExactSizeIterator for CFArrayIter<'_, T> {}

// TODO:
// impl<T: Type> DoubleEndedIterator for CFArrayIter<'_, T> { ... }

// Fused unless someone mutates the array, so we won't guarantee that (for now).
// impl<T: Type> FusedIterator for CFArrayIter<'_, T> {}

/// A retained iterator over the items of an array.
#[derive(Debug)]
pub struct CFArrayIntoIter<T: ?Sized> {
    array: CFRetained<CFArray<T>>,
    index: usize,
}

impl<T: Type> Iterator for CFArrayIntoIter<T> {
    type Item = CFRetained<T>;

    fn next(&mut self) -> Option<CFRetained<T>> {
        // Same as `CFArrayIter::next`.
        let value = self.array.get(self.index)?;
        self.index += 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.len().saturating_sub(self.index);
        (len, Some(len))
    }
}

impl<T: Type> ExactSizeIterator for CFArrayIntoIter<T> {}

impl<'a, T: Type> IntoIterator for &'a CFArray<T> {
    type Item = CFRetained<T>;
    type IntoIter = CFArrayIter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Type> IntoIterator for &'a CFMutableArray<T> {
    type Item = CFRetained<T>;
    type IntoIter = CFArrayIter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Type> IntoIterator for CFRetained<CFArray<T>> {
    type Item = CFRetained<T>;
    type IntoIter = CFArrayIntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        CFArrayIntoIter {
            array: self,
            index: 0,
        }
    }
}

impl<T: Type> IntoIterator for CFRetained<CFMutableArray<T>> {
    type Item = CFRetained<T>;
    type IntoIter = CFArrayIntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: Upcasting `CFMutableArray<T>` to `CFArray<T>`.
        let array = unsafe { CFRetained::cast_unchecked::<CFArray<T>>(self) };
        CFArrayIntoIter { array, index: 0 }
    }
}

/// An iterator over raw items of an array.
///
/// # Safety
///
/// The array must not be mutated while this is alive.
#[derive(Debug)]
pub struct CFArrayIterUnchecked<'a, T: ?Sized + 'a> {
    array: &'a CFArray<T>,
    index: CFIndex,
    len: CFIndex,
}

impl<'a, T: Type> Iterator for CFArrayIterUnchecked<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        debug_assert_eq!(
            self.array.len(),
            self.len as usize,
            "array was mutated while iterating"
        );
        if self.index < self.len {
            // SAFETY:
            // - That the array isn't mutated while iterating is upheld by the
            //   caller of `CFArray::iter_unchecked`.
            // - Index in bounds is ensured by the check above (which uses a
            //   pre-computed length, and thus also assumes that the array
            //   isn't mutated while iterating).
            let value = unsafe { self.array.get_unchecked(self.index) };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.len - self.index) as usize;
        (len, Some(len))
    }
}

impl<T: Type> ExactSizeIterator for CFArrayIterUnchecked<'_, T> {}

// Allow easy conversion from `&CFArray<T>` to `&CFArray`.
// Requires `T: Type` because of reflexive impl in `cf_type!`.
impl<T: ?Sized + Type> AsRef<CFArray> for CFArray<T> {
    fn as_ref(&self) -> &CFArray {
        self.as_opaque()
    }
}
impl<T: ?Sized + Type> AsRef<CFMutableArray> for CFMutableArray<T> {
    fn as_ref(&self) -> &CFMutableArray {
        self.as_opaque()
    }
}

// `Eq`, `Ord` and `Hash` have the same semantics.
impl<T: ?Sized + Type> Borrow<CFArray> for CFArray<T> {
    fn borrow(&self) -> &CFArray {
        self.as_opaque()
    }
}
impl<T: ?Sized + Type> Borrow<CFMutableArray> for CFMutableArray<T> {
    fn borrow(&self) -> &CFMutableArray {
        self.as_opaque()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "CFString")]
    use crate::CFString;
    use core::ptr::null;

    #[test]
    fn array_with_invalid_pointers() {
        // without_provenance
        let ptr = [0 as _, 1 as _, 2 as _, 3 as _, usize::MAX as _].as_mut_ptr();
        let array = unsafe { CFArray::new(None, ptr, 1, null()) }.unwrap();
        let value = unsafe { array.value_at_index(0) };
        assert!(value.is_null());
    }

    #[test]
    #[should_panic]
    #[ignore = "aborts (as expected)"]
    fn object_array_cannot_contain_null() {
        let ptr = [null()].as_mut_ptr();
        let _array = unsafe { CFArray::new(None, ptr, 1, &kCFTypeArrayCallBacks) };
    }

    #[test]
    #[cfg(feature = "CFString")]
    fn correct_retain_count() {
        let objects = [
            CFString::from_str("some long string that doesn't get small-string optimized"),
            CFString::from_str("another long string that doesn't get small-string optimized"),
        ];
        let array = CFArray::from_retained_objects(&objects);

        // Creating array retains elements.
        assert_eq!(array.retain_count(), 1);
        assert_eq!(unsafe { array.get_unchecked(0) }.retain_count(), 2);
        assert_eq!(unsafe { array.get_unchecked(1) }.retain_count(), 2);

        drop(objects);
        assert_eq!(unsafe { array.get_unchecked(0) }.retain_count(), 1);
        assert_eq!(unsafe { array.get_unchecked(1) }.retain_count(), 1);

        // Retaining array doesn't affect retain count of elements.
        let _array2 = array.retain();
        assert_eq!(unsafe { array.get_unchecked(0) }.retain_count(), 1);
        assert_eq!(unsafe { array.get_unchecked(1) }.retain_count(), 1);

        // Using retaining API changes retain count.
        assert_eq!(array.get(0).unwrap().retain_count(), 2);
    }

    #[test]
    #[cfg(feature = "CFString")]
    fn iter() {
        use alloc::vec::Vec;

        let s1 = CFString::from_str("a");
        let s2 = CFString::from_str("b");
        let array = CFArray::from_objects(&[&*s1, &*s2, &*s1]);

        assert_eq!(
            array.iter().collect::<Vec<_>>(),
            [s1.clone(), s2.clone(), s1.clone()]
        );

        assert_eq!(
            unsafe { array.iter_unchecked() }.collect::<Vec<_>>(),
            [&*s1, &*s2, &*s1]
        );
    }

    #[test]
    #[cfg(feature = "CFString")]
    fn iter_fused() {
        let s1 = CFString::from_str("a");
        let s2 = CFString::from_str("b");
        let array = CFArray::from_objects(&[&*s1, &*s2]);

        let mut iter = array.iter();
        assert_eq!(iter.next(), Some(s1.clone()));
        assert_eq!(iter.next(), Some(s2.clone()));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = unsafe { array.iter_unchecked() };
        assert_eq!(iter.next(), Some(&*s1));
        assert_eq!(iter.next(), Some(&*s2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[cfg(feature = "CFString")]
    fn mutate() {
        let array = CFMutableArray::<CFString>::with_capacity(10);
        array.insert(0, &CFString::from_str("a"));
        array.append(&CFString::from_str("c"));
        array.insert(1, &CFString::from_str("b"));
        assert_eq!(
            array.to_vec(),
            [
                CFString::from_str("a"),
                CFString::from_str("b"),
                CFString::from_str("c"),
            ]
        );
    }

    #[test]
    #[cfg(feature = "CFString")]
    #[cfg_attr(
        not(debug_assertions),
        ignore = "not detected when debug assertions are off"
    )]
    #[should_panic = "array was mutated while iterating"]
    fn mutate_while_iter_unchecked() {
        let array = CFMutableArray::<CFString>::with_capacity(10);
        assert_eq!(array.len(), 0);

        let mut iter = unsafe { array.iter_unchecked() };
        array.append(&CFString::from_str("a"));
        // Should panic
        let _ = iter.next();
    }
}
