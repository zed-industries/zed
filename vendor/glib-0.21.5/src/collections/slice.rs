// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, mem, ptr};

use crate::{ffi, translate::*};

// rustdoc-stripper-ignore-next
/// Minimum size of the `Slice` allocation in bytes.
const MIN_SIZE: usize = 256;

// rustdoc-stripper-ignore-next
/// Slice of elements of type `T` allocated by the GLib allocator.
///
/// This can be used like a `&[T]`, `&mut [T]` and `Vec<T>`.
pub struct Slice<T: TransparentType> {
    ptr: ptr::NonNull<T::GlibType>,
    len: usize,
    capacity: usize,
}

unsafe impl<T: TransparentType + Send> Send for Slice<T> {}

unsafe impl<T: TransparentType + Sync> Sync for Slice<T> {}

impl<T: fmt::Debug + TransparentType> fmt::Debug for Slice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T: PartialEq + TransparentType> PartialEq for Slice<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq + TransparentType> Eq for Slice<T> {}

impl<T: PartialOrd + TransparentType> PartialOrd for Slice<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord + TransparentType> Ord for Slice<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: std::hash::Hash + TransparentType> std::hash::Hash for Slice<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T: PartialEq + TransparentType> PartialEq<[T]> for Slice<T> {
    #[inline]
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq + TransparentType> PartialEq<Slice<T>> for [T] {
    #[inline]
    fn eq(&self, other: &Slice<T>) -> bool {
        self == other.as_slice()
    }
}

impl<T: TransparentType> Drop for Slice<T> {
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

impl<T: TransparentType> AsRef<[T]> for Slice<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: TransparentType> AsMut<[T]> for Slice<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: TransparentType> std::borrow::Borrow<[T]> for Slice<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: TransparentType> std::borrow::BorrowMut<[T]> for Slice<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: TransparentType> std::ops::Deref for Slice<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: TransparentType> std::ops::DerefMut for Slice<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: TransparentType> Default for Slice<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: TransparentType> std::iter::Extend<T> for Slice<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        for item in iter {
            self.push(item);
        }
    }
}

impl<'a, T: TransparentType + 'a> std::iter::Extend<&'a T> for Slice<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        for item in iter {
            self.push(item.clone());
        }
    }
}

impl<T: TransparentType> std::iter::FromIterator<T> for Slice<T> {
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

impl<'a, T: TransparentType> std::iter::IntoIterator for &'a Slice<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T: TransparentType> std::iter::IntoIterator for &'a mut Slice<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<T: TransparentType> std::iter::IntoIterator for Slice<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct IntoIter<T: TransparentType> {
    ptr: ptr::NonNull<T::GlibType>,
    idx: ptr::NonNull<T::GlibType>,
    len: usize,
    empty: bool,
}

impl<T: TransparentType> IntoIter<T> {
    #[inline]
    fn new(slice: Slice<T>) -> Self {
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

impl<T: TransparentType> Drop for IntoIter<T> {
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

impl<T: TransparentType> Iterator for IntoIter<T> {
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

impl<T: TransparentType> DoubleEndedIterator for IntoIter<T> {
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

impl<T: TransparentType> ExactSizeIterator for IntoIter<T> {}

impl<T: TransparentType> std::iter::FusedIterator for IntoIter<T> {}

impl<T: TransparentType> From<Slice<T>> for Vec<T> {
    #[inline]
    fn from(mut value: Slice<T>) -> Self {
        unsafe {
            let mut s = Vec::with_capacity(value.len);
            ptr::copy_nonoverlapping(value.ptr.as_ptr() as *const T, s.as_mut_ptr(), value.len);
            s.set_len(value.len);
            value.len = 0;
            s
        }
    }
}

impl<T: TransparentType> From<Vec<T>> for Slice<T> {
    #[inline]
    fn from(mut value: Vec<T>) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            ptr::copy_nonoverlapping(value.as_ptr(), s.ptr.as_ptr() as *mut T, value.len());
            s.len = value.len();
            value.set_len(0);
            s
        }
    }
}

impl<T: TransparentType, const N: usize> From<[T; N]> for Slice<T> {
    #[inline]
    fn from(value: [T; N]) -> Self {
        unsafe {
            let value = mem::ManuallyDrop::new(value);
            let len = value.len();
            let mut s = Self::with_capacity(len);
            ptr::copy_nonoverlapping(value.as_ptr(), s.ptr.as_ptr() as *mut T, len);
            s.len = len;
            s
        }
    }
}

impl<'a, T: TransparentType> From<&'a [T]> for Slice<T> {
    #[inline]
    fn from(value: &'a [T]) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            for (i, item) in value.iter().enumerate() {
                ptr::write(s.ptr.as_ptr().add(i) as *mut T, item.clone());
            }
            s.len = value.len();
            s
        }
    }
}

impl<'a, T: TransparentType> From<&'a [&'a T]> for Slice<T> {
    #[inline]
    fn from(value: &'a [&'a T]) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            for (i, item) in value.iter().enumerate() {
                ptr::write(s.ptr.as_ptr().add(i) as *mut T, (*item).clone());
            }
            s.len = value.len();
            s
        }
    }
}

impl<T: TransparentType> Clone for Slice<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::from(self.as_slice())
    }
}

impl<T: TransparentType> Slice<T> {
    // rustdoc-stripper-ignore-next
    /// Borrows a C array.
    #[inline]
    pub unsafe fn from_glib_borrow_num<'a>(ptr: *const T::GlibType, len: usize) -> &'a [T] {
        debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<T::GlibType>());
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(ptr as *const T, len)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Borrows a mutable C array.
    #[inline]
    pub unsafe fn from_glib_borrow_num_mut<'a>(ptr: *mut T::GlibType, len: usize) -> &'a mut [T] {
        debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<T::GlibType>());
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &mut []
        } else {
            std::slice::from_raw_parts_mut(ptr as *mut T, len)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Borrows a C array of references.
    #[inline]
    pub unsafe fn from_glib_ptr_borrow_num<'a>(
        ptr: *const *const T::GlibType,
        len: usize,
    ) -> &'a [&'a T] {
        debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<T::GlibType>());
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(ptr as *const &T, len)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Borrows a mutable C array.
    #[inline]
    pub unsafe fn from_glib_ptr_borrow_num_mut<'a>(
        ptr: *mut *mut T::GlibType,
        len: usize,
    ) -> &'a mut [&'a mut T] {
        debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<T::GlibType>());
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &mut []
        } else {
            std::slice::from_raw_parts_mut(ptr as *mut &mut T, len)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `Slice` around a C array.
    #[inline]
    pub unsafe fn from_glib_none_num(ptr: *const T::GlibType, len: usize) -> Self {
        debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<T::GlibType>());
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            Slice::default()
        } else {
            // Need to fully copy the array here.
            let s = Self::from_glib_borrow_num(ptr, len);
            Self::from(s)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `Slice` around a C array.
    #[inline]
    pub unsafe fn from_glib_container_num(ptr: *mut T::GlibType, len: usize) -> Self {
        debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<T::GlibType>());
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            ffi::g_free(ptr as ffi::gpointer);
            Slice::default()
        } else {
            // Need to clone every item because we don't own it here but only
            // if this type requires explicit drop.
            if mem::needs_drop::<T>() {
                for i in 0..len {
                    let p = ptr.add(i) as *mut T;
                    let clone: T = (*p).clone();
                    ptr::write(p, clone);
                }
            }

            // And now it can be handled exactly the same as `from_glib_full_num()`.
            Self::from_glib_full_num(ptr, len)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `Slice` around a C array.
    #[inline]
    pub unsafe fn from_glib_full_num(ptr: *mut T::GlibType, len: usize) -> Self {
        debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<T::GlibType>());
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            ffi::g_free(ptr as ffi::gpointer);
            Slice::default()
        } else {
            Slice {
                ptr: ptr::NonNull::new_unchecked(ptr),
                len,
                capacity: len,
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new empty slice.
    #[inline]
    pub fn new() -> Self {
        debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<T::GlibType>());

        Slice {
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
    #[inline]
    pub fn as_ptr(&self) -> *const T::GlibType {
        if self.len == 0 {
            ptr::null()
        } else {
            self.ptr.as_ptr()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the underlying pointer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T::GlibType {
        if self.len == 0 {
            ptr::null_mut()
        } else {
            self.ptr.as_ptr()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Consumes the slice and returns the underlying pointer.
    #[inline]
    pub fn into_raw(mut self) -> *mut T::GlibType {
        if self.len == 0 {
            ptr::null_mut()
        } else {
            self.len = 0;
            self.capacity = 0;
            self.ptr.as_ptr()
        }
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
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // rustdoc-stripper-ignore-next
    /// Sets the length of the slice to `len`.
    ///
    /// # SAFETY
    ///
    /// There must be at least `len` valid items.
    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    // rustdoc-stripper-ignore-next
    /// Reserves at least this much additional capacity.
    pub fn reserve(&mut self, additional: usize) {
        // Nothing new to reserve as there's still enough space
        if additional <= self.capacity - self.len {
            return;
        }

        let new_capacity = usize::next_power_of_two(std::cmp::max(
            self.len + additional,
            MIN_SIZE / mem::size_of::<T>(),
        ));
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
                    as *mut T::GlibType;
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
        if other.len() > self.capacity - self.len {
            self.reserve(other.len());
        }

        unsafe {
            for item in other {
                ptr::write(self.ptr.as_ptr().add(self.len) as *mut T, item.clone());
                self.len += 1;
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Inserts `item` at position `index` of the slice, shifting all elements after it to the
    /// right.
    #[inline]
    #[allow(clippy::int_plus_one)]
    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len);

        // Nothing new to reserve as there's still enough space
        if 1 > self.capacity - self.len {
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
        }
    }

    // rustdoc-stripper-ignore-next
    /// Pushes `item` to the end of the slice.
    #[inline]
    #[allow(clippy::int_plus_one)]
    pub fn push(&mut self, item: T) {
        // Nothing new to reserve as there's still enough space
        if 1 > self.capacity - self.len {
            self.reserve(1);
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len) as *mut T, item);
            self.len += 1;
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
            }
        }
    }
}

impl<T: TransparentType + 'static> FromGlibContainer<T::GlibType, *mut T::GlibType> for Slice<T> {
    unsafe fn from_glib_none_num(ptr: *mut T::GlibType, num: usize) -> Self {
        Self::from_glib_none_num(ptr, num)
    }

    #[inline]
    unsafe fn from_glib_container_num(ptr: *mut T::GlibType, num: usize) -> Self {
        Self::from_glib_container_num(ptr, num)
    }

    #[inline]
    unsafe fn from_glib_full_num(ptr: *mut T::GlibType, num: usize) -> Self {
        Self::from_glib_full_num(ptr, num)
    }
}

impl<T: TransparentType + 'static> FromGlibContainer<T::GlibType, *const T::GlibType> for Slice<T> {
    unsafe fn from_glib_none_num(ptr: *const T::GlibType, num: usize) -> Self {
        Self::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_container_num(_ptr: *const T::GlibType, _num: usize) -> Self {
        unimplemented!();
    }

    unsafe fn from_glib_full_num(_ptr: *const T::GlibType, _num: usize) -> Self {
        unimplemented!();
    }
}

impl<'a, T: TransparentType + 'a> ToGlibPtr<'a, *mut T::GlibType> for Slice<T> {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut T::GlibType, Self> {
        Stash(self.as_ptr() as *mut _, PhantomData)
    }

    #[inline]
    fn to_glib_container(&'a self) -> Stash<'a, *mut T::GlibType, Self> {
        unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<T>().checked_mul(self.len()).unwrap())
                as *mut T::GlibType;
            ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            Stash(ptr, PhantomData)
        }
    }

    #[inline]
    fn to_glib_full(&self) -> *mut T::GlibType {
        self.clone().into_raw()
    }
}

impl<'a, T: TransparentType + 'a> ToGlibPtr<'a, *const T::GlibType> for Slice<T> {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const T::GlibType, Self> {
        Stash(self.as_ptr(), PhantomData)
    }
}

impl<'a, T: TransparentType + 'a> ToGlibPtrMut<'a, *mut T::GlibType> for Slice<T> {
    type Storage = PhantomData<&'a mut Self>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut T::GlibType, Self> {
        StashMut(self.as_mut_ptr(), PhantomData)
    }
}

impl<T: TransparentType + 'static> IntoGlibPtr<*mut T::GlibType> for Slice<T> {
    #[inline]
    fn into_glib_ptr(self) -> *mut T::GlibType {
        self.into_raw()
    }
}

impl<T: TransparentPtrType> From<super::PtrSlice<T>> for Slice<T> {
    fn from(value: super::PtrSlice<T>) -> Self {
        let len = value.len();
        let capacity = value.capacity();
        unsafe {
            let ptr = value.into_raw();
            Slice::<T> {
                ptr: ptr::NonNull::new_unchecked(ptr),
                len,
                capacity,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_glib_full() {
        let items = [
            crate::Date::from_dmy(20, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(21, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(22, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(23, crate::DateMonth::November, 2021).unwrap(),
        ];

        let slice = unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<ffi::GDate>() * 4) as *mut ffi::GDate;
            ptr::write(ptr.add(0), *items[0].to_glib_none().0);
            ptr::write(ptr.add(1), *items[1].to_glib_none().0);
            ptr::write(ptr.add(2), *items[2].to_glib_none().0);
            ptr::write(ptr.add(3), *items[3].to_glib_none().0);

            Slice::<crate::Date>::from_glib_full_num(ptr, 4)
        };

        assert_eq!(&items[..], &*slice);
    }

    #[test]
    fn test_from_glib_none() {
        let items = [
            crate::Date::from_dmy(20, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(21, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(22, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(23, crate::DateMonth::November, 2021).unwrap(),
        ];

        let slice = unsafe {
            Slice::<crate::Date>::from_glib_none_num(items.as_ptr() as *const ffi::GDate, 4)
        };

        assert_eq!(&items[..], &*slice);
    }

    #[test]
    fn test_safe_api() {
        let items = [
            crate::Date::from_dmy(20, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(21, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(22, crate::DateMonth::November, 2021).unwrap(),
        ];

        let mut slice = Slice::from(&items[..]);
        assert_eq!(slice.len(), 3);
        slice.push(crate::Date::from_dmy(23, crate::DateMonth::November, 2021).unwrap());
        assert_eq!(slice.len(), 4);

        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a, b);
        }
        assert_eq!(
            (slice[3].day(), slice[3].month(), slice[3].year()),
            (23, crate::DateMonth::November, 2021)
        );

        let vec = Vec::from(slice);
        assert_eq!(vec.len(), 4);
        for (a, b) in Iterator::zip(items.iter(), vec.iter()) {
            assert_eq!(a, b);
        }
        assert_eq!(
            (vec[3].day(), vec[3].month(), vec[3].year()),
            (23, crate::DateMonth::November, 2021)
        );

        let mut slice = Slice::from(vec);
        assert_eq!(slice.len(), 4);
        let e = slice.pop().unwrap();
        assert_eq!(
            (e.day(), e.month(), e.year()),
            (23, crate::DateMonth::November, 2021)
        );
        assert_eq!(slice.len(), 3);
        slice.insert(2, e);
        assert_eq!(slice.len(), 4);
        assert_eq!(slice[0].day(), 20);
        assert_eq!(slice[1].day(), 21);
        assert_eq!(slice[2].day(), 23);
        assert_eq!(slice[3].day(), 22);
        let e = slice.remove(2);
        assert_eq!(
            (e.day(), e.month(), e.year()),
            (23, crate::DateMonth::November, 2021)
        );
        assert_eq!(slice.len(), 3);
        slice.push(e);
        assert_eq!(slice.len(), 4);

        for (a, b) in Iterator::zip(items.iter(), slice.into_iter()) {
            assert_eq!(a, &b);
        }
    }
}
