// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, mem, ptr};

use crate::{ffi, translate::*};

// rustdoc-stripper-ignore-next
/// Minimum size of the `PtrSlice` allocation.
const MIN_SIZE: usize = 16;

// rustdoc-stripper-ignore-next
/// Slice of elements of type `T` allocated by the GLib allocator.
///
/// The underlying memory is always `NULL`-terminated. [`Slice<T>`](crate::collections::slice::Slice)
/// can be used for a non-`NULL`-terminated slice.
///
/// This can be used like a `&[T]`, `&mut [T]` and `Vec<T>`.
pub struct PtrSlice<T: TransparentPtrType> {
    ptr: ptr::NonNull<<T as GlibPtrDefault>::GlibType>,
    // rustdoc-stripper-ignore-next
    /// Length without the `NULL`-terminator.
    len: usize,
    // rustdoc-stripper-ignore-next
    /// Capacity **with** the `NULL`-terminator, i.e. the actual allocation size.
    capacity: usize,
}

impl<T: fmt::Debug + TransparentPtrType> fmt::Debug for PtrSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

unsafe impl<T: Send + TransparentPtrType> Send for PtrSlice<T> {}

unsafe impl<T: Sync + TransparentPtrType> Sync for PtrSlice<T> {}

impl<T: PartialEq + TransparentPtrType> PartialEq for PtrSlice<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq + TransparentPtrType> Eq for PtrSlice<T> {}

impl<T: PartialOrd + TransparentPtrType> PartialOrd for PtrSlice<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord + TransparentPtrType> Ord for PtrSlice<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: std::hash::Hash + TransparentPtrType> std::hash::Hash for PtrSlice<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T: PartialEq + TransparentPtrType> PartialEq<[T]> for PtrSlice<T> {
    #[inline]
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq + TransparentPtrType> PartialEq<PtrSlice<T>> for [T] {
    #[inline]
    fn eq(&self, other: &PtrSlice<T>) -> bool {
        self == other.as_slice()
    }
}

impl<T: TransparentPtrType> Drop for PtrSlice<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                for i in 0..self.len {
                    ptr::drop_in_place::<T>(self.ptr.as_ptr().add(i) as *mut T);
                }
            }

            if self.capacity != 0 {
                ffi::g_free(self.ptr.as_ptr() as ffi::gpointer);
            }
        }
    }
}

impl<T: TransparentPtrType> AsRef<[T]> for PtrSlice<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: TransparentPtrType> AsMut<[T]> for PtrSlice<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: TransparentPtrType> std::borrow::Borrow<[T]> for PtrSlice<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: TransparentPtrType> std::borrow::BorrowMut<[T]> for PtrSlice<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: TransparentPtrType> std::ops::Deref for PtrSlice<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: TransparentPtrType> std::ops::DerefMut for PtrSlice<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: TransparentPtrType> Default for PtrSlice<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: TransparentPtrType> std::iter::Extend<T> for PtrSlice<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        for item in iter {
            self.push(item);
        }
    }
}

impl<'a, T: TransparentPtrType + 'a> std::iter::Extend<&'a T> for PtrSlice<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        for item in iter {
            self.push(item.clone());
        }
    }
}

impl<T: TransparentPtrType> std::iter::FromIterator<T> for PtrSlice<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut s = Self::with_capacity(iter.size_hint().0);
        for item in iter {
            s.push(item);
        }
        s
    }
}

impl<'a, T: TransparentPtrType> std::iter::IntoIterator for &'a PtrSlice<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T: TransparentPtrType> std::iter::IntoIterator for &'a mut PtrSlice<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<T: TransparentPtrType> std::iter::IntoIterator for PtrSlice<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct IntoIter<T: TransparentPtrType> {
    ptr: ptr::NonNull<<T as GlibPtrDefault>::GlibType>,
    idx: ptr::NonNull<<T as GlibPtrDefault>::GlibType>,
    len: usize,
    empty: bool,
}

impl<T: TransparentPtrType> IntoIter<T> {
    #[inline]
    fn new(slice: PtrSlice<T>) -> Self {
        let slice = mem::ManuallyDrop::new(slice);
        IntoIter {
            ptr: slice.ptr,
            idx: slice.ptr,
            len: slice.len,
            empty: slice.capacity == 0,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the remaining items as slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            if self.len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.idx.as_ptr() as *mut T, self.len)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the remaining items as mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            if self.len == 0 {
                &mut []
            } else {
                std::slice::from_raw_parts_mut(self.idx.as_ptr() as *mut T, self.len)
            }
        }
    }
}

impl<T: TransparentPtrType> Drop for IntoIter<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                for i in 0..self.len {
                    ptr::drop_in_place::<T>(self.idx.as_ptr().add(i) as *mut T);
                }
            }

            if !self.empty {
                ffi::g_free(self.ptr.as_ptr() as ffi::gpointer);
            }
        }
    }
}

impl<T: TransparentPtrType> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let p = self.idx.as_ptr();
            self.len -= 1;
            self.idx = ptr::NonNull::new_unchecked(p.add(1));
            Some(ptr::read(p as *mut T))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len
    }

    #[inline]
    fn last(mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { ptr::read(self.idx.as_ptr().add(self.len) as *mut T) })
        }
    }
}

impl<T: TransparentPtrType> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { ptr::read(self.idx.as_ptr().add(self.len) as *mut T) })
        }
    }
}

impl<T: TransparentPtrType> ExactSizeIterator for IntoIter<T> {}

impl<T: TransparentPtrType> std::iter::FusedIterator for IntoIter<T> {}

impl<T: TransparentPtrType> From<PtrSlice<T>> for Vec<T> {
    #[inline]
    fn from(mut value: PtrSlice<T>) -> Self {
        unsafe {
            let mut s = Vec::with_capacity(value.len);
            ptr::copy_nonoverlapping(value.ptr.as_ptr() as *const T, s.as_mut_ptr(), value.len);
            s.set_len(value.len);
            value.len = 0;
            s
        }
    }
}

impl<T: TransparentPtrType> From<Vec<T>> for PtrSlice<T> {
    #[inline]
    fn from(mut value: Vec<T>) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            ptr::copy_nonoverlapping(value.as_ptr(), s.ptr.as_ptr() as *mut T, value.len());
            s.len = value.len();
            value.set_len(0);
            ptr::write(
                s.ptr.as_ptr().add(s.len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );
            s
        }
    }
}

impl<T: TransparentPtrType, const N: usize> From<[T; N]> for PtrSlice<T> {
    #[inline]
    fn from(value: [T; N]) -> Self {
        unsafe {
            let value = mem::ManuallyDrop::new(value);
            let len = value.len();
            let mut s = Self::with_capacity(len);
            ptr::copy_nonoverlapping(value.as_ptr(), s.ptr.as_ptr() as *mut T, len);
            s.len = len;
            ptr::write(
                s.ptr.as_ptr().add(len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );
            s
        }
    }
}

impl<'a, T: TransparentPtrType> From<&'a [T]> for PtrSlice<T> {
    #[inline]
    fn from(value: &'a [T]) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            for (i, item) in value.iter().enumerate() {
                ptr::write(s.ptr.as_ptr().add(i) as *mut T, item.clone());
            }
            s.len = value.len();
            ptr::write(
                s.ptr.as_ptr().add(s.len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );
            s
        }
    }
}

impl<'a, T: TransparentPtrType> From<&'a [&'a T]> for PtrSlice<T> {
    #[inline]
    fn from(value: &'a [&'a T]) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            for (i, item) in value.iter().enumerate() {
                ptr::write(s.ptr.as_ptr().add(i) as *mut T, (*item).clone());
            }
            s.len = value.len();
            ptr::write(
                s.ptr.as_ptr().add(s.len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );
            s
        }
    }
}

impl<T: TransparentPtrType> Clone for PtrSlice<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::from(self.as_slice())
    }
}

impl<T: TransparentPtrType> PtrSlice<T> {
    // rustdoc-stripper-ignore-next
    /// Borrows a C array.
    #[inline]
    pub unsafe fn from_glib_borrow<'a>(ptr: *const <T as GlibPtrDefault>::GlibType) -> &'a [T] {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }
        Self::from_glib_borrow_num(ptr, len)
    }

    // rustdoc-stripper-ignore-next
    /// Borrows a C array.
    #[inline]
    pub unsafe fn from_glib_borrow_num<'a>(
        ptr: *const <T as GlibPtrDefault>::GlibType,
        len: usize,
    ) -> &'a [T] {
        debug_assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(ptr as *const T, len)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a C array.
    #[inline]
    pub unsafe fn from_glib_none_num(
        ptr: *const <T as GlibPtrDefault>::GlibType,
        len: usize,
        _null_terminated: bool,
    ) -> Self {
        debug_assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            PtrSlice::default()
        } else {
            // Need to fully copy the array here.
            let s = Self::from_glib_borrow_num(ptr, len);
            Self::from(s)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a C array.
    #[inline]
    pub unsafe fn from_glib_container_num(
        ptr: *mut <T as GlibPtrDefault>::GlibType,
        len: usize,
        null_terminated: bool,
    ) -> Self {
        debug_assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            ffi::g_free(ptr as ffi::gpointer);
            PtrSlice::default()
        } else {
            // Need to clone every item because we don't own it here
            for i in 0..len {
                let p = ptr.add(i) as *mut T;
                let clone: T = (*p).clone();
                ptr::write(p, clone);
            }

            // And now it can be handled exactly the same as `from_glib_full_num()`.
            Self::from_glib_full_num(ptr, len, null_terminated)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a C array.
    #[inline]
    pub unsafe fn from_glib_full_num(
        ptr: *mut <T as GlibPtrDefault>::GlibType,
        len: usize,
        null_terminated: bool,
    ) -> Self {
        debug_assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            ffi::g_free(ptr as ffi::gpointer);
            PtrSlice::default()
        } else {
            if null_terminated {
                return PtrSlice {
                    ptr: ptr::NonNull::new_unchecked(ptr),
                    len,
                    capacity: len + 1,
                };
            }

            // Need to re-allocate here for adding the NULL-terminator
            let capacity = len + 1;
            assert_ne!(capacity, 0);
            let ptr = ffi::g_realloc(
                ptr as *mut _,
                mem::size_of::<T>().checked_mul(capacity).unwrap(),
            ) as *mut <T as GlibPtrDefault>::GlibType;

            ptr::write(
                ptr.add(len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );

            PtrSlice {
                ptr: ptr::NonNull::new_unchecked(ptr),
                len,
                capacity,
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a `NULL`-terminated C array.
    #[inline]
    pub unsafe fn from_glib_none(ptr: *const <T as GlibPtrDefault>::GlibType) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        PtrSlice::from_glib_none_num(ptr, len, true)
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a `NULL`-terminated C array.
    #[inline]
    pub unsafe fn from_glib_container(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        PtrSlice::from_glib_container_num(ptr, len, true)
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a `NULL`-terminated C array.
    #[inline]
    pub unsafe fn from_glib_full(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        PtrSlice::from_glib_full_num(ptr, len, true)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new empty slice.
    #[inline]
    pub fn new() -> Self {
        debug_assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );

        PtrSlice {
            ptr: ptr::NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new empty slice with the given capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut s = Self::new();
        s.reserve(capacity);
        s
    }

    // rustdoc-stripper-ignore-next
    /// Returns the underlying pointer.
    ///
    /// This is guaranteed to be `NULL`-terminated.
    #[inline]
    pub fn as_ptr(&self) -> *const <T as GlibPtrDefault>::GlibType {
        if self.len == 0 {
            static EMPTY: [usize; 1] = [0];

            EMPTY.as_ptr() as *const _
        } else {
            self.ptr.as_ptr()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the underlying pointer.
    ///
    /// This is guaranteed to be `NULL`-terminated.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut <T as GlibPtrDefault>::GlibType {
        if self.len == 0 {
            static EMPTY: [usize; 1] = [0];

            EMPTY.as_ptr() as *mut _
        } else {
            self.ptr.as_ptr()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Consumes the slice and returns the underlying pointer.
    ///
    /// This is guaranteed to be `NULL`-terminated.
    #[inline]
    pub fn into_raw(mut self) -> *mut <T as GlibPtrDefault>::GlibType {
        // Make sure to allocate a valid pointer that points to a
        // NULL-pointer.
        if self.len == 0 {
            self.reserve(0);
            unsafe {
                ptr::write(
                    self.ptr.as_ptr().add(0),
                    Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
                );
            }
        }

        self.len = 0;
        self.capacity = 0;
        self.ptr.as_ptr()
    }

    // rustdoc-stripper-ignore-next
    /// Gets the length of the slice.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if the slice is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // rustdoc-stripper-ignore-next
    /// Returns the capacity of the slice.
    ///
    /// This includes the space that is reserved for the `NULL`-terminator.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // rustdoc-stripper-ignore-next
    /// Sets the length of the slice to `len`.
    ///
    /// # SAFETY
    ///
    /// There must be at least `len` valid items and a `NULL`-terminator after the last item.
    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    // rustdoc-stripper-ignore-next
    /// Reserves at least this much additional capacity.
    #[allow(clippy::int_plus_one)]
    pub fn reserve(&mut self, additional: usize) {
        // Nothing new to reserve as there's still enough space
        if additional < self.capacity - self.len {
            return;
        }

        let new_capacity =
            usize::next_power_of_two(std::cmp::max(self.len + additional, MIN_SIZE) + 1);
        assert_ne!(new_capacity, 0);
        assert!(new_capacity > self.capacity);

        unsafe {
            let ptr = if self.capacity == 0 {
                ptr::null_mut()
            } else {
                self.ptr.as_ptr() as *mut _
            };
            let new_ptr =
                ffi::g_realloc(ptr, mem::size_of::<T>().checked_mul(new_capacity).unwrap())
                    as *mut <T as GlibPtrDefault>::GlibType;
            if self.capacity == 0 {
                ptr::write(
                    new_ptr,
                    Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
                );
            }
            self.ptr = ptr::NonNull::new_unchecked(new_ptr);
            self.capacity = new_capacity;
        }
    }

    // rustdoc-stripper-ignore-next
    /// Borrows this slice as a `&[T]`.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            if self.len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.ptr.as_ptr() as *const T, self.len)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Borrows this slice as a `&mut [T]`.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            if self.len == 0 {
                &mut []
            } else {
                std::slice::from_raw_parts_mut(self.ptr.as_ptr() as *mut T, self.len)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Removes all items from the slice.
    #[inline]
    pub fn clear(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                for i in 0..self.len {
                    ptr::drop_in_place::<T>(self.ptr.as_ptr().add(i) as *mut T);
                }
            }

            self.len = 0;
        }
    }

    // rustdoc-stripper-ignore-next
    /// Clones and appends all elements in `slice` to the slice.
    #[inline]
    pub fn extend_from_slice(&mut self, other: &[T]) {
        // Nothing new to reserve as there's still enough space
        if other.len() >= self.capacity - self.len {
            self.reserve(other.len());
        }

        unsafe {
            for item in other {
                ptr::write(self.ptr.as_ptr().add(self.len) as *mut T, item.clone());
                self.len += 1;

                // Add null terminator on every iteration because `clone`
                // may panic
                ptr::write(
                    self.ptr.as_ptr().add(self.len),
                    Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
                );
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Inserts `item` at position `index` of the slice, shifting all elements after it to the
    /// right.
    #[inline]
    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len);

        // Nothing new to reserve as there's still enough space
        if 1 >= self.capacity - self.len {
            self.reserve(1);
        }

        unsafe {
            if index == self.len {
                ptr::write(self.ptr.as_ptr().add(self.len) as *mut T, item);
            } else {
                let p = self.ptr.as_ptr().add(index);
                ptr::copy(p, p.add(1), self.len - index);
                ptr::write(self.ptr.as_ptr().add(index) as *mut T, item);
            }

            self.len += 1;

            ptr::write(
                self.ptr.as_ptr().add(self.len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );
        }
    }

    // rustdoc-stripper-ignore-next
    /// Pushes `item` to the end of the slice.
    #[inline]
    pub fn push(&mut self, item: T) {
        // Nothing new to reserve as there's still enough space
        if 1 >= self.capacity - self.len {
            self.reserve(1);
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len) as *mut T, item);
            self.len += 1;

            ptr::write(
                self.ptr.as_ptr().add(self.len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );
        }
    }

    // rustdoc-stripper-ignore-next
    /// Removes item from position `index` of the slice, shifting all elements after it to the
    /// left.
    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);

        unsafe {
            let p = self.ptr.as_ptr().add(index);
            let item = ptr::read(p as *mut T);
            ptr::copy(p.add(1), p, self.len - index - 1);

            self.len -= 1;

            ptr::write(
                self.ptr.as_ptr().add(self.len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );

            item
        }
    }

    // rustdoc-stripper-ignore-next
    /// Removes the last item of the slice and returns it.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            self.len -= 1;
            let p = self.ptr.as_ptr().add(self.len);
            let item = ptr::read(p as *mut T);

            ptr::write(
                self.ptr.as_ptr().add(self.len),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );

            Some(item)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Shortens the slice by keeping the last `len` items.
    ///
    /// If there are fewer than `len` items then this has no effect.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        if self.len <= len {
            return;
        }

        unsafe {
            while self.len > len {
                self.len -= 1;
                let p = self.ptr.as_ptr().add(self.len);
                ptr::drop_in_place::<T>(p as *mut T);
                ptr::write(
                    p,
                    Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
                );
            }
        }
    }
}

impl<T: TransparentPtrType>
    FromGlibContainer<<T as GlibPtrDefault>::GlibType, *mut <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    #[inline]
    unsafe fn from_glib_none_num(ptr: *mut <T as GlibPtrDefault>::GlibType, num: usize) -> Self {
        Self::from_glib_none_num(ptr, num, false)
    }

    #[inline]
    unsafe fn from_glib_container_num(
        ptr: *mut <T as GlibPtrDefault>::GlibType,
        num: usize,
    ) -> Self {
        Self::from_glib_container_num(ptr, num, false)
    }

    #[inline]
    unsafe fn from_glib_full_num(ptr: *mut <T as GlibPtrDefault>::GlibType, num: usize) -> Self {
        Self::from_glib_full_num(ptr, num, false)
    }
}

impl<T: TransparentPtrType>
    FromGlibContainer<<T as GlibPtrDefault>::GlibType, *const <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    unsafe fn from_glib_none_num(ptr: *const <T as GlibPtrDefault>::GlibType, num: usize) -> Self {
        Self::from_glib_none_num(ptr, num, false)
    }

    unsafe fn from_glib_container_num(
        _ptr: *const <T as GlibPtrDefault>::GlibType,
        _num: usize,
    ) -> Self {
        unimplemented!();
    }

    unsafe fn from_glib_full_num(
        _ptr: *const <T as GlibPtrDefault>::GlibType,
        _num: usize,
    ) -> Self {
        unimplemented!();
    }
}

impl<T: TransparentPtrType>
    FromGlibPtrContainer<<T as GlibPtrDefault>::GlibType, *mut <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    #[inline]
    unsafe fn from_glib_none(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        Self::from_glib_none(ptr)
    }

    #[inline]
    unsafe fn from_glib_container(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        Self::from_glib_container(ptr)
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        Self::from_glib_full(ptr)
    }
}

impl<T: TransparentPtrType>
    FromGlibPtrContainer<<T as GlibPtrDefault>::GlibType, *const <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    #[inline]
    unsafe fn from_glib_none(ptr: *const <T as GlibPtrDefault>::GlibType) -> Self {
        Self::from_glib_none(ptr)
    }

    unsafe fn from_glib_container(_ptr: *const <T as GlibPtrDefault>::GlibType) -> Self {
        unimplemented!();
    }

    unsafe fn from_glib_full(_ptr: *const <T as GlibPtrDefault>::GlibType) -> Self {
        unimplemented!();
    }
}

impl<'a, T: TransparentPtrType + 'a> ToGlibPtr<'a, *mut <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut <T as GlibPtrDefault>::GlibType, Self> {
        Stash(self.as_ptr() as *mut _, PhantomData)
    }

    #[inline]
    fn to_glib_container(&'a self) -> Stash<'a, *mut <T as GlibPtrDefault>::GlibType, Self> {
        unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<T>().checked_mul(self.len() + 1).unwrap())
                as *mut <T as GlibPtrDefault>::GlibType;
            ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len() + 1);
            Stash(ptr, PhantomData)
        }
    }

    #[inline]
    fn to_glib_full(&self) -> *mut <T as GlibPtrDefault>::GlibType {
        self.clone().into_raw()
    }
}

impl<'a, T: TransparentPtrType + 'a> ToGlibPtr<'a, *const <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const <T as GlibPtrDefault>::GlibType, Self> {
        Stash(self.as_ptr(), PhantomData)
    }
}

impl<'a, T: TransparentPtrType + 'a> ToGlibPtrMut<'a, *mut <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    type Storage = PhantomData<&'a mut Self>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut <T as GlibPtrDefault>::GlibType, Self> {
        StashMut(self.as_mut_ptr(), PhantomData)
    }
}

impl<T: TransparentPtrType> IntoGlibPtr<*mut <T as GlibPtrDefault>::GlibType> for PtrSlice<T> {
    #[inline]
    fn into_glib_ptr(self) -> *mut <T as GlibPtrDefault>::GlibType {
        self.into_raw()
    }
}

impl<T: TransparentPtrType> From<super::Slice<T>> for PtrSlice<T> {
    fn from(value: super::Slice<T>) -> Self {
        let len = value.len();
        let capacity = value.capacity();
        unsafe {
            let ptr = value.into_raw();
            let mut s = PtrSlice::<T> {
                ptr: ptr::NonNull::new_unchecked(ptr),
                len,
                capacity,
            };

            // Reserve space for the `NULL`-terminator if needed
            if len == capacity {
                s.reserve(0);
            }

            ptr::write(
                s.ptr.as_ptr().add(s.len()),
                Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
            );

            s
        }
    }
}

// rustdoc-stripper-ignore-next
/// A trait to accept both `&[T]` or `PtrSlice<T>` as an argument.
pub trait IntoPtrSlice<T: TransparentPtrType> {
    // rustdoc-stripper-ignore-next
    /// Runs the given closure with a `NULL`-terminated array.
    fn run_with_ptr_slice<R, F: FnOnce(&[<T as GlibPtrDefault>::GlibType]) -> R>(self, f: F) -> R;
}

impl<T: TransparentPtrType> IntoPtrSlice<T> for PtrSlice<T> {
    #[inline]
    fn run_with_ptr_slice<R, F: FnOnce(&[<T as GlibPtrDefault>::GlibType]) -> R>(self, f: F) -> R {
        <&Self>::run_with_ptr_slice(&self, f)
    }
}

impl<T: TransparentPtrType> IntoPtrSlice<T> for &'_ PtrSlice<T> {
    #[inline]
    fn run_with_ptr_slice<R, F: FnOnce(&[<T as GlibPtrDefault>::GlibType]) -> R>(self, f: F) -> R {
        f(unsafe { std::slice::from_raw_parts(self.as_ptr() as *mut _, self.len() + 1) })
    }
}

// rustdoc-stripper-ignore-next
/// Maximum number of pointers to stack-allocate before falling back to a heap allocation.
const MAX_STACK_ALLOCATION: usize = 16;

impl<T: TransparentPtrType> IntoPtrSlice<T> for Vec<T> {
    #[inline]
    fn run_with_ptr_slice<R, F: FnOnce(&[<T as GlibPtrDefault>::GlibType]) -> R>(self, f: F) -> R {
        if self.len() < MAX_STACK_ALLOCATION {
            unsafe {
                let mut s = mem::MaybeUninit::<
                    [<T as GlibPtrDefault>::GlibType; MAX_STACK_ALLOCATION],
                >::uninit();
                let ptr = s.as_mut_ptr() as *mut <T as GlibPtrDefault>::GlibType;
                ptr::copy_nonoverlapping(
                    self.as_ptr() as *mut <T as GlibPtrDefault>::GlibType,
                    ptr,
                    self.len(),
                );
                ptr::write(
                    ptr.add(self.len()),
                    Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
                );
                f(std::slice::from_raw_parts(ptr, self.len() + 1))
            }
        } else {
            PtrSlice::<T>::from(self).run_with_ptr_slice(f)
        }
    }
}

impl<T: TransparentPtrType, const N: usize> IntoPtrSlice<T> for [T; N] {
    #[inline]
    fn run_with_ptr_slice<R, F: FnOnce(&[<T as GlibPtrDefault>::GlibType]) -> R>(self, f: F) -> R {
        if self.len() < MAX_STACK_ALLOCATION {
            unsafe {
                let mut s = mem::MaybeUninit::<
                    [<T as GlibPtrDefault>::GlibType; MAX_STACK_ALLOCATION],
                >::uninit();
                let ptr = s.as_mut_ptr() as *mut <T as GlibPtrDefault>::GlibType;
                ptr::copy_nonoverlapping(
                    self.as_ptr() as *mut <T as GlibPtrDefault>::GlibType,
                    ptr,
                    self.len(),
                );
                ptr::write(
                    ptr.add(self.len()),
                    Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
                );
                f(std::slice::from_raw_parts(ptr, self.len() + 1))
            }
        } else {
            PtrSlice::<T>::from(self).run_with_ptr_slice(f)
        }
    }
}

impl<T: TransparentPtrType> IntoPtrSlice<T> for &'_ [T] {
    #[inline]
    fn run_with_ptr_slice<R, F: FnOnce(&[<T as GlibPtrDefault>::GlibType]) -> R>(self, f: F) -> R {
        if self.len() < MAX_STACK_ALLOCATION {
            unsafe {
                let mut s = mem::MaybeUninit::<
                    [<T as GlibPtrDefault>::GlibType; MAX_STACK_ALLOCATION],
                >::uninit();
                let ptr = s.as_mut_ptr() as *mut <T as GlibPtrDefault>::GlibType;
                ptr::copy_nonoverlapping(
                    self.as_ptr() as *mut <T as GlibPtrDefault>::GlibType,
                    ptr,
                    self.len(),
                );
                ptr::write(
                    ptr.add(self.len()),
                    Ptr::from(ptr::null_mut::<<T as GlibPtrDefault>::GlibType>()),
                );
                f(std::slice::from_raw_parts(ptr, self.len() + 1))
            }
        } else {
            PtrSlice::<T>::from(self).run_with_ptr_slice(f)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_glib_full() {
        let items = [
            crate::Error::new(crate::FileError::Failed, "Failed 1"),
            crate::Error::new(crate::FileError::Noent, "Failed 2"),
            crate::Error::new(crate::FileError::Io, "Failed 3"),
            crate::Error::new(crate::FileError::Perm, "Failed 4"),
        ];

        let slice = unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<ffi::GDate>() * 4) as *mut *mut ffi::GError;
            ptr::write(ptr.add(0), items[0].to_glib_full());
            ptr::write(ptr.add(1), items[1].to_glib_full());
            ptr::write(ptr.add(2), items[2].to_glib_full());
            ptr::write(ptr.add(3), items[3].to_glib_full());

            PtrSlice::<crate::Error>::from_glib_full_num(ptr, 4, false)
        };

        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a.message(), b.message());
            assert_eq!(
                a.kind::<crate::FileError>().unwrap(),
                b.kind::<crate::FileError>().unwrap()
            );
        }
    }

    #[test]
    fn test_from_glib_none() {
        let items = [
            crate::Error::new(crate::FileError::Failed, "Failed 1"),
            crate::Error::new(crate::FileError::Noent, "Failed 2"),
            crate::Error::new(crate::FileError::Io, "Failed 3"),
            crate::Error::new(crate::FileError::Perm, "Failed 4"),
        ];

        let slice = unsafe {
            PtrSlice::<crate::Error>::from_glib_none_num(
                items.as_ptr() as *const *mut ffi::GError,
                4,
                false,
            )
        };

        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a.message(), b.message());
            assert_eq!(
                a.kind::<crate::FileError>().unwrap(),
                b.kind::<crate::FileError>().unwrap()
            );
        }
    }

    #[test]
    fn test_safe_api() {
        let items = [
            crate::Error::new(crate::FileError::Failed, "Failed 1"),
            crate::Error::new(crate::FileError::Noent, "Failed 2"),
            crate::Error::new(crate::FileError::Io, "Failed 3"),
        ];

        let mut slice = PtrSlice::from(&items[..]);
        assert_eq!(slice.len(), 3);
        slice.push(crate::Error::new(crate::FileError::Perm, "Failed 4"));
        assert_eq!(slice.len(), 4);

        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a.message(), b.message());
            assert_eq!(
                a.kind::<crate::FileError>().unwrap(),
                b.kind::<crate::FileError>().unwrap()
            );
        }
        assert_eq!(slice[3].message(), "Failed 4");

        let vec = Vec::from(slice);
        assert_eq!(vec.len(), 4);
        for (a, b) in Iterator::zip(items.iter(), vec.iter()) {
            assert_eq!(a.message(), b.message());
            assert_eq!(
                a.kind::<crate::FileError>().unwrap(),
                b.kind::<crate::FileError>().unwrap()
            );
        }
        assert_eq!(vec[3].message(), "Failed 4");

        let mut slice = PtrSlice::from(vec);
        assert_eq!(slice.len(), 4);
        let e = slice.pop().unwrap();
        assert_eq!(e.message(), "Failed 4");
        assert_eq!(slice.len(), 3);
        slice.insert(2, e);
        assert_eq!(slice.len(), 4);
        assert_eq!(slice[0].message(), "Failed 1");
        assert_eq!(slice[1].message(), "Failed 2");
        assert_eq!(slice[2].message(), "Failed 4");
        assert_eq!(slice[3].message(), "Failed 3");
        let e = slice.remove(2);
        assert_eq!(e.message(), "Failed 4");
        assert_eq!(slice.len(), 3);
        slice.push(e);
        assert_eq!(slice.len(), 4);

        let slice2 = crate::Slice::from(slice.clone());

        for (a, b) in Iterator::zip(items.iter(), slice.into_iter()) {
            assert_eq!(a.message(), b.message());
            assert_eq!(
                a.kind::<crate::FileError>().unwrap(),
                b.kind::<crate::FileError>().unwrap()
            );
        }

        let slice3 = crate::PtrSlice::from(slice2.clone());

        for (a, b) in Iterator::zip(items.iter(), slice2.into_iter()) {
            assert_eq!(a.message(), b.message());
            assert_eq!(
                a.kind::<crate::FileError>().unwrap(),
                b.kind::<crate::FileError>().unwrap()
            );
        }

        for (a, b) in Iterator::zip(items.iter(), slice3.into_iter()) {
            assert_eq!(a.message(), b.message());
            assert_eq!(
                a.kind::<crate::FileError>().unwrap(),
                b.kind::<crate::FileError>().unwrap()
            );
        }
    }

    #[test]
    fn test_into_ptrslice() {
        let items = [
            crate::Error::new(crate::FileError::Failed, "Failed 1"),
            crate::Error::new(crate::FileError::Noent, "Failed 2"),
            crate::Error::new(crate::FileError::Io, "Failed 3"),
            crate::Error::new(crate::FileError::Perm, "Failed 4"),
        ];

        items[..].run_with_ptr_slice(|s| unsafe {
            assert!(s[4].is_null());
            assert_eq!(s.len(), items.len() + 1);
            let s = std::slice::from_raw_parts(s.as_ptr() as *const crate::Error, items.len());
            assert_eq!(s, items);
        });

        Vec::from(&items[..]).run_with_ptr_slice(|s| unsafe {
            assert!(s[4].is_null());
            assert_eq!(s.len(), items.len() + 1);
            let s = std::slice::from_raw_parts(s.as_ptr() as *const crate::Error, items.len());
            for (a, b) in Iterator::zip(items.iter(), s.iter()) {
                assert_eq!(a.message(), b.message());
                assert_eq!(
                    a.kind::<crate::FileError>().unwrap(),
                    b.kind::<crate::FileError>().unwrap()
                );
            }
        });

        PtrSlice::<crate::Error>::from(&items[..]).run_with_ptr_slice(|s| unsafe {
            assert!(s[4].is_null());
            assert_eq!(s.len(), items.len() + 1);
            let s = std::slice::from_raw_parts(s.as_ptr() as *const crate::Error, items.len());
            for (a, b) in Iterator::zip(items.iter(), s.iter()) {
                assert_eq!(a.message(), b.message());
                assert_eq!(
                    a.kind::<crate::FileError>().unwrap(),
                    b.kind::<crate::FileError>().unwrap()
                );
            }
        });

        let v = Vec::from(&items[..]);
        items.run_with_ptr_slice(|s| unsafe {
            assert!(s[4].is_null());
            assert_eq!(s.len(), v.len() + 1);
            let s = std::slice::from_raw_parts(s.as_ptr() as *const crate::Error, v.len());
            for (a, b) in Iterator::zip(v.iter(), s.iter()) {
                assert_eq!(a.message(), b.message());
                assert_eq!(
                    a.kind::<crate::FileError>().unwrap(),
                    b.kind::<crate::FileError>().unwrap()
                );
            }
        });
    }
}
