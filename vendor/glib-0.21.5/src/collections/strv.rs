// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::c_char, fmt, marker::PhantomData, mem, ptr};

use crate::{ffi, gobject_ffi, prelude::*, translate::*, GStr, GString, GStringPtr};

// rustdoc-stripper-ignore-next
/// Minimum size of the `StrV` allocation.
const MIN_SIZE: usize = 16;

// rustdoc-stripper-ignore-next
/// `NULL`-terminated array of `NULL`-terminated strings.
///
/// The underlying memory is always `NULL`-terminated.
///
/// This can be used like a `&[&str]`, `&mut [&str]` and `Vec<&str>`.
pub struct StrV {
    ptr: ptr::NonNull<*mut c_char>,
    // rustdoc-stripper-ignore-next
    /// Length without the `NULL`-terminator.
    len: usize,
    // rustdoc-stripper-ignore-next
    /// Capacity **with** the `NULL`-terminator, i.e. the actual allocation size.
    capacity: usize,
}

impl fmt::Debug for StrV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

unsafe impl Send for StrV {}

unsafe impl Sync for StrV {}

impl PartialEq for StrV {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for StrV {}

impl PartialOrd for StrV {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StrV {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl std::hash::Hash for StrV {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl PartialEq<[&'_ str]> for StrV {
    fn eq(&self, other: &[&'_ str]) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (a, b) in Iterator::zip(self.iter(), other.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl PartialEq<StrV> for [&'_ str] {
    #[inline]
    fn eq(&self, other: &StrV) -> bool {
        other.eq(self)
    }
}

impl Drop for StrV {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if self.capacity != 0 {
                ffi::g_strfreev(self.ptr.as_ptr());
            }
        }
    }
}

impl Default for StrV {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[GStringPtr]> for StrV {
    #[inline]
    fn as_ref(&self) -> &[GStringPtr] {
        self.as_slice()
    }
}

impl std::borrow::Borrow<[GStringPtr]> for StrV {
    #[inline]
    fn borrow(&self) -> &[GStringPtr] {
        self.as_slice()
    }
}

impl AsRef<StrVRef> for StrV {
    #[inline]
    fn as_ref(&self) -> &StrVRef {
        self.into()
    }
}

impl std::borrow::Borrow<StrVRef> for StrV {
    #[inline]
    fn borrow(&self) -> &StrVRef {
        self.into()
    }
}

impl std::ops::Deref for StrV {
    type Target = StrVRef;

    #[inline]
    fn deref(&self) -> &StrVRef {
        self.into()
    }
}

impl std::iter::Extend<GString> for StrV {
    #[inline]
    fn extend<I: IntoIterator<Item = GString>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        for item in iter {
            self.push(item);
        }
    }
}

impl<'a> std::iter::Extend<&'a str> for StrV {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        for item in iter {
            self.push(GString::from(item));
        }
    }
}

impl std::iter::FromIterator<GString> for StrV {
    #[inline]
    fn from_iter<I: IntoIterator<Item = GString>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut s = Self::with_capacity(iter.size_hint().0);
        for item in iter {
            s.push(item);
        }
        s
    }
}

impl<'a> std::iter::IntoIterator for &'a StrV {
    type Item = &'a GStringPtr;
    type IntoIter = std::slice::Iter<'a, GStringPtr>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl std::iter::IntoIterator for StrV {
    type Item = GString;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct IntoIter {
    ptr: ptr::NonNull<*mut c_char>,
    idx: ptr::NonNull<*mut c_char>,
    len: usize,
    empty: bool,
}

impl IntoIter {
    #[inline]
    fn new(slice: StrV) -> Self {
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
    pub const fn as_slice(&self) -> &[GStringPtr] {
        unsafe {
            if self.len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.idx.as_ptr() as *const GStringPtr, self.len)
            }
        }
    }
}

impl Drop for IntoIter {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ffi::g_free(*self.idx.as_ptr().add(i) as ffi::gpointer);
            }

            if !self.empty {
                ffi::g_free(self.ptr.as_ptr() as ffi::gpointer);
            }
        }
    }
}

impl Iterator for IntoIter {
    type Item = GString;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let p = self.idx.as_ptr();
            self.len -= 1;
            self.idx = ptr::NonNull::new_unchecked(p.add(1));
            Some(GString::from_glib_full(*p))
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
    fn last(mut self) -> Option<GString> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { GString::from_glib_full(*self.idx.as_ptr().add(self.len)) })
        }
    }
}

impl DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<GString> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { GString::from_glib_full(*self.idx.as_ptr().add(self.len)) })
        }
    }
}

impl ExactSizeIterator for IntoIter {}

impl std::iter::FusedIterator for IntoIter {}

impl From<StrV> for Vec<GString> {
    #[inline]
    fn from(value: StrV) -> Self {
        value.into_iter().collect()
    }
}

impl From<Vec<String>> for StrV {
    #[inline]
    fn from(value: Vec<String>) -> Self {
        unsafe {
            let len = value.len();
            let mut s = Self::with_capacity(len);
            for (i, item) in value.into_iter().enumerate() {
                *s.ptr.as_ptr().add(i) = GString::from(item).into_glib_ptr();
            }
            s.len = len;
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl From<Vec<&'_ str>> for StrV {
    #[inline]
    fn from(value: Vec<&'_ str>) -> Self {
        value.as_slice().into()
    }
}

impl From<Vec<GString>> for StrV {
    #[inline]
    fn from(value: Vec<GString>) -> Self {
        unsafe {
            let len = value.len();
            let mut s = Self::with_capacity(len);
            for (i, v) in value.into_iter().enumerate() {
                *s.ptr.as_ptr().add(i) = v.into_glib_ptr();
            }
            s.len = len;
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl<const N: usize> From<[GString; N]> for StrV {
    #[inline]
    fn from(value: [GString; N]) -> Self {
        unsafe {
            let len = value.len();
            let mut s = Self::with_capacity(len);
            for (i, v) in value.into_iter().enumerate() {
                *s.ptr.as_ptr().add(i) = v.into_glib_ptr();
            }
            s.len = len;
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl<const N: usize> From<[String; N]> for StrV {
    #[inline]
    fn from(value: [String; N]) -> Self {
        unsafe {
            let len = value.len();
            let mut s = Self::with_capacity(len);
            for (i, v) in value.into_iter().enumerate() {
                *s.ptr.as_ptr().add(i) = GString::from(v).into_glib_ptr();
            }
            s.len = len;
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl<const N: usize> From<[&'_ str; N]> for StrV {
    #[inline]
    fn from(value: [&'_ str; N]) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            for (i, item) in value.iter().enumerate() {
                *s.ptr.as_ptr().add(i) = GString::from(*item).into_glib_ptr();
            }
            s.len = value.len();
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl<const N: usize> From<[&'_ GStr; N]> for StrV {
    #[inline]
    fn from(value: [&'_ GStr; N]) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            for (i, item) in value.iter().enumerate() {
                *s.ptr.as_ptr().add(i) = GString::from(*item).into_glib_ptr();
            }
            s.len = value.len();
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl From<&'_ [&'_ str]> for StrV {
    #[inline]
    fn from(value: &'_ [&'_ str]) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            for (i, item) in value.iter().enumerate() {
                *s.ptr.as_ptr().add(i) = GString::from(*item).into_glib_ptr();
            }
            s.len = value.len();
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl From<&'_ [&'_ GStr]> for StrV {
    #[inline]
    fn from(value: &'_ [&'_ GStr]) -> Self {
        unsafe {
            let mut s = Self::with_capacity(value.len());
            for (i, item) in value.iter().enumerate() {
                *s.ptr.as_ptr().add(i) = GString::from(*item).into_glib_ptr();
            }
            s.len = value.len();
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl Clone for StrV {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut s = Self::with_capacity(self.len());
            for (i, item) in self.iter().enumerate() {
                *s.ptr.as_ptr().add(i) = GString::from(item.as_str()).into_glib_ptr();
            }
            s.len = self.len();
            *s.ptr.as_ptr().add(s.len) = ptr::null_mut();
            s
        }
    }
}

impl StrV {
    // rustdoc-stripper-ignore-next
    /// Borrows a C array.
    #[inline]
    pub unsafe fn from_glib_borrow<'a>(ptr: *const *const c_char) -> &'a [GStringPtr] {
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
        ptr: *const *const c_char,
        len: usize,
    ) -> &'a [GStringPtr] {
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(ptr as *const GStringPtr, len)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `StrV` around a C array.
    #[inline]
    pub unsafe fn from_glib_none_num(
        ptr: *const *const c_char,
        len: usize,
        _null_terminated: bool,
    ) -> Self {
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            StrV::default()
        } else {
            // Allocate space for len + 1 pointers, one pointer for each string and a trailing
            // null pointer.
            let new_ptr =
                ffi::g_malloc(mem::size_of::<*mut c_char>() * (len + 1)) as *mut *mut c_char;

            // Need to clone every item because we don't own it here
            for i in 0..len {
                let p = ptr.add(i) as *mut *const c_char;
                let q = new_ptr.add(i) as *mut *const c_char;
                *q = ffi::g_strdup(*p);
            }

            *new_ptr.add(len) = ptr::null_mut();

            StrV {
                ptr: ptr::NonNull::new_unchecked(new_ptr),
                len,
                capacity: len + 1,
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `StrV` around a C array.
    #[inline]
    pub unsafe fn from_glib_container_num(
        ptr: *mut *const c_char,
        len: usize,
        null_terminated: bool,
    ) -> Self {
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            ffi::g_free(ptr as ffi::gpointer);
            StrV::default()
        } else {
            // Need to clone every item because we don't own it here
            for i in 0..len {
                let p = ptr.add(i);
                *p = ffi::g_strdup(*p);
            }

            // And now it can be handled exactly the same as `from_glib_full_num()`.
            Self::from_glib_full_num(ptr as *mut *mut c_char, len, null_terminated)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `StrV` around a C array.
    #[inline]
    pub unsafe fn from_glib_full_num(
        ptr: *mut *mut c_char,
        len: usize,
        null_terminated: bool,
    ) -> Self {
        debug_assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            ffi::g_free(ptr as ffi::gpointer);
            StrV::default()
        } else {
            if null_terminated {
                return StrV {
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
                mem::size_of::<*mut c_char>().checked_mul(capacity).unwrap(),
            ) as *mut *mut c_char;
            *ptr.add(len) = ptr::null_mut();

            StrV {
                ptr: ptr::NonNull::new_unchecked(ptr),
                len,
                capacity,
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `StrV` around a `NULL`-terminated C array.
    #[inline]
    pub unsafe fn from_glib_none(ptr: *const *const c_char) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        StrV::from_glib_none_num(ptr, len, true)
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `StrV` around a `NULL`-terminated C array.
    #[inline]
    pub unsafe fn from_glib_container(ptr: *mut *const c_char) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        StrV::from_glib_container_num(ptr, len, true)
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `StrV` around a `NULL`-terminated C array.
    #[inline]
    pub unsafe fn from_glib_full(ptr: *mut *mut c_char) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        StrV::from_glib_full_num(ptr, len, true)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new empty slice.
    #[inline]
    pub fn new() -> Self {
        StrV {
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
    pub fn as_ptr(&self) -> *const *mut c_char {
        if self.len == 0 {
            static EMPTY: [usize; 1] = [0];

            EMPTY.as_ptr() as *const _
        } else {
            self.ptr.as_ptr()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Consumes the slice and returns the underlying pointer.
    ///
    /// This is guaranteed to be `NULL`-terminated.
    #[inline]
    pub fn into_raw(mut self) -> *mut *mut c_char {
        // Make sure to allocate a valid pointer that points to a
        // NULL-pointer.
        if self.len == 0 {
            self.reserve(0);
            unsafe {
                *self.ptr.as_ptr().add(0) = ptr::null_mut();
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
            let new_ptr = ffi::g_realloc(
                ptr,
                mem::size_of::<*mut c_char>()
                    .checked_mul(new_capacity)
                    .unwrap(),
            ) as *mut *mut c_char;
            if self.capacity == 0 {
                *new_ptr = ptr::null_mut();
            }
            self.ptr = ptr::NonNull::new_unchecked(new_ptr);
            self.capacity = new_capacity;
        }
    }

    // rustdoc-stripper-ignore-next
    /// Borrows this slice as a `&[GStringPtr]`.
    #[inline]
    pub const fn as_slice(&self) -> &[GStringPtr] {
        unsafe {
            if self.len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.ptr.as_ptr() as *const GStringPtr, self.len)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Removes all items from the slice.
    #[inline]
    pub fn clear(&mut self) {
        unsafe {
            for i in 0..self.len {
                ffi::g_free(*self.ptr.as_ptr().add(i) as ffi::gpointer);
            }

            self.len = 0;
        }
    }

    // rustdoc-stripper-ignore-next
    /// Clones and appends all elements in `slice` to the slice.
    #[inline]
    pub fn extend_from_slice<S: AsRef<str>>(&mut self, other: &[S]) {
        // Nothing new to reserve as there's still enough space
        if other.len() >= self.capacity - self.len {
            self.reserve(other.len());
        }

        unsafe {
            for item in other {
                *self.ptr.as_ptr().add(self.len) = GString::from(item.as_ref()).into_glib_ptr();
                self.len += 1;

                // Add null terminator on every iteration because `as_ref`
                // may panic
                *self.ptr.as_ptr().add(self.len) = ptr::null_mut();
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Inserts `item` at position `index` of the slice, shifting all elements after it to the
    /// right.
    #[inline]
    pub fn insert(&mut self, index: usize, item: GString) {
        assert!(index <= self.len);

        // Nothing new to reserve as there's still enough space
        if 1 >= self.capacity - self.len {
            self.reserve(1);
        }

        unsafe {
            if index == self.len {
                *self.ptr.as_ptr().add(self.len) = item.into_glib_ptr();
            } else {
                let p = self.ptr.as_ptr().add(index);
                ptr::copy(p, p.add(1), self.len - index);
                *self.ptr.as_ptr().add(index) = item.into_glib_ptr();
            }

            self.len += 1;

            *self.ptr.as_ptr().add(self.len) = ptr::null_mut();
        }
    }

    // rustdoc-stripper-ignore-next
    /// Pushes `item` to the end of the slice.
    #[inline]
    pub fn push(&mut self, item: GString) {
        // Nothing new to reserve as there's still enough space
        if 1 >= self.capacity - self.len {
            self.reserve(1);
        }

        unsafe {
            *self.ptr.as_ptr().add(self.len) = item.into_glib_ptr();
            self.len += 1;

            *self.ptr.as_ptr().add(self.len) = ptr::null_mut();
        }
    }

    // rustdoc-stripper-ignore-next
    /// Removes item from position `index` of the slice, shifting all elements after it to the
    /// left.
    #[inline]
    pub fn remove(&mut self, index: usize) -> GString {
        assert!(index < self.len);

        unsafe {
            let p = self.ptr.as_ptr().add(index);
            let item = *p;
            ptr::copy(p.add(1), p, self.len - index - 1);

            self.len -= 1;

            *self.ptr.as_ptr().add(self.len) = ptr::null_mut();

            GString::from_glib_full(item)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Swaps item from position `index` of the slice and returns it.
    #[inline]
    pub fn swap(&mut self, index: usize, new_item: GString) -> GString {
        assert!(index < self.len);

        unsafe {
            let p = self.ptr.as_ptr().add(index);
            let item = *p;
            *p = new_item.into_glib_ptr();

            GString::from_glib_full(item)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Removes the last item of the slice and returns it.
    #[inline]
    pub fn pop(&mut self) -> Option<GString> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            self.len -= 1;
            let p = self.ptr.as_ptr().add(self.len);
            let item = *p;

            *self.ptr.as_ptr().add(self.len) = ptr::null_mut();

            Some(GString::from_glib_full(item))
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
                ffi::g_free(*p as ffi::gpointer);
                *p = ptr::null_mut();
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Joins the strings into a longer string, with an optional separator
    #[inline]
    #[doc(alias = "g_strjoinv")]
    pub fn join(&self, separator: Option<impl IntoGStr>) -> GString {
        separator.run_with_gstr(|separator| unsafe {
            from_glib_full(ffi::g_strjoinv(
                separator.to_glib_none().0,
                self.as_ptr() as *mut _,
            ))
        })
    }

    // rustdoc-stripper-ignore-next
    /// Checks whether the `StrV` contains the specified string
    #[inline]
    #[doc(alias = "g_strv_contains")]
    pub fn contains(&self, s: impl IntoGStr) -> bool {
        s.run_with_gstr(|s| unsafe {
            from_glib(ffi::g_strv_contains(
                self.as_ptr() as *const _,
                s.to_glib_none().0,
            ))
        })
    }
}

impl FromGlibContainer<*mut c_char, *mut *mut c_char> for StrV {
    #[inline]
    unsafe fn from_glib_none_num(ptr: *mut *mut c_char, num: usize) -> Self {
        Self::from_glib_none_num(ptr as *const *const c_char, num, false)
    }

    #[inline]
    unsafe fn from_glib_container_num(ptr: *mut *mut c_char, num: usize) -> Self {
        Self::from_glib_container_num(ptr as *mut *const c_char, num, false)
    }

    #[inline]
    unsafe fn from_glib_full_num(ptr: *mut *mut c_char, num: usize) -> Self {
        Self::from_glib_full_num(ptr, num, false)
    }
}

impl FromGlibContainer<*mut c_char, *const *mut c_char> for StrV {
    unsafe fn from_glib_none_num(ptr: *const *mut c_char, num: usize) -> Self {
        Self::from_glib_none_num(ptr as *const *const c_char, num, false)
    }

    unsafe fn from_glib_container_num(_ptr: *const *mut c_char, _num: usize) -> Self {
        unimplemented!();
    }

    unsafe fn from_glib_full_num(_ptr: *const *mut c_char, _num: usize) -> Self {
        unimplemented!();
    }
}

impl FromGlibPtrContainer<*mut c_char, *mut *mut c_char> for StrV {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut *mut c_char) -> Self {
        Self::from_glib_none(ptr as *const *const c_char)
    }

    #[inline]
    unsafe fn from_glib_container(ptr: *mut *mut c_char) -> Self {
        Self::from_glib_container(ptr as *mut *const c_char)
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut *mut c_char) -> Self {
        Self::from_glib_full(ptr)
    }
}

impl FromGlibPtrContainer<*mut c_char, *const *mut c_char> for StrV {
    #[inline]
    unsafe fn from_glib_none(ptr: *const *mut c_char) -> Self {
        Self::from_glib_none(ptr as *const *const c_char)
    }

    unsafe fn from_glib_container(_ptr: *const *mut c_char) -> Self {
        unimplemented!();
    }

    unsafe fn from_glib_full(_ptr: *const *mut c_char) -> Self {
        unimplemented!();
    }
}

impl<'a> ToGlibPtr<'a, *mut *mut c_char> for StrV {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut *mut c_char, Self> {
        Stash(self.as_ptr() as *mut _, PhantomData)
    }

    #[inline]
    fn to_glib_container(&'a self) -> Stash<'a, *mut *mut c_char, Self> {
        unsafe {
            let ptr =
                ffi::g_malloc(mem::size_of::<*mut c_char>() * (self.len() + 1)) as *mut *mut c_char;
            ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len() + 1);
            Stash(ptr, PhantomData)
        }
    }

    #[inline]
    fn to_glib_full(&self) -> *mut *mut c_char {
        self.clone().into_raw()
    }
}

impl<'a> ToGlibPtr<'a, *const *mut c_char> for StrV {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const *mut c_char, Self> {
        Stash(self.as_ptr(), PhantomData)
    }
}

impl IntoGlibPtr<*mut *mut c_char> for StrV {
    #[inline]
    fn into_glib_ptr(self) -> *mut *mut c_char {
        self.into_raw()
    }
}

impl StaticType for StrV {
    #[inline]
    fn static_type() -> crate::Type {
        <Vec<String>>::static_type()
    }
}

impl StaticType for &'_ [GStringPtr] {
    #[inline]
    fn static_type() -> crate::Type {
        <Vec<String>>::static_type()
    }
}

impl crate::value::ValueType for StrV {
    type Type = Vec<String>;
}

unsafe impl<'a> crate::value::FromValue<'a> for StrV {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a crate::value::Value) -> Self {
        let ptr = gobject_ffi::g_value_dup_boxed(value.to_glib_none().0) as *mut *mut c_char;
        FromGlibPtrContainer::from_glib_full(ptr)
    }
}

unsafe impl<'a> crate::value::FromValue<'a> for &'a [GStringPtr] {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a crate::value::Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
        StrV::from_glib_borrow(ptr)
    }
}

impl crate::value::ToValue for StrV {
    fn to_value(&self) -> crate::value::Value {
        unsafe {
            let mut value = crate::value::Value::for_value_type::<Self>();
            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.as_ptr() as ffi::gpointer,
            );
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        <StrV as StaticType>::static_type()
    }
}

impl From<StrV> for crate::Value {
    #[inline]
    fn from(s: StrV) -> Self {
        unsafe {
            let mut value = crate::value::Value::for_value_type::<StrV>();
            gobject_ffi::g_value_take_boxed(
                value.to_glib_none_mut().0,
                s.into_raw() as ffi::gpointer,
            );
            value
        }
    }
}

// rustdoc-stripper-ignore-next
/// A trait to accept both `&[T]` or `StrV` as an argument.
pub trait IntoStrV {
    // rustdoc-stripper-ignore-next
    /// Runs the given closure with a `NULL`-terminated array.
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R;
}

impl IntoStrV for StrV {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        <&Self>::run_with_strv(&self, f)
    }
}

impl IntoStrV for &'_ StrV {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        f(unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) })
    }
}

// rustdoc-stripper-ignore-next
/// Maximum number of pointers to stack-allocate before falling back to a heap allocation.
///
/// The beginning will be used for the pointers, the remainder for the actual string content.
const MAX_STACK_ALLOCATION: usize = 16;

impl IntoStrV for Vec<GString> {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl IntoStrV for Vec<&'_ GString> {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl IntoStrV for Vec<&'_ GStr> {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl IntoStrV for Vec<&'_ str> {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl IntoStrV for Vec<String> {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl IntoStrV for Vec<&'_ String> {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl IntoStrV for &[GString] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        let required_len = (self.len() + 1) * mem::size_of::<*mut c_char>();

        if required_len < MAX_STACK_ALLOCATION * mem::size_of::<*mut c_char>() {
            unsafe {
                let mut s = mem::MaybeUninit::<[*mut c_char; MAX_STACK_ALLOCATION]>::uninit();
                let ptrs = s.as_mut_ptr() as *mut *mut c_char;

                for (i, item) in self.iter().enumerate() {
                    *ptrs.add(i) = item.as_ptr() as *mut _;
                }
                *ptrs.add(self.len()) = ptr::null_mut();

                f(std::slice::from_raw_parts(ptrs, self.len()))
            }
        } else {
            let mut s = StrV::with_capacity(self.len());
            s.extend_from_slice(self);
            s.run_with_strv(f)
        }
    }
}

impl IntoStrV for &[&GString] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        let required_len = (self.len() + 1) * mem::size_of::<*mut c_char>();

        if required_len < MAX_STACK_ALLOCATION * mem::size_of::<*mut c_char>() {
            unsafe {
                let mut s = mem::MaybeUninit::<[*mut c_char; MAX_STACK_ALLOCATION]>::uninit();
                let ptrs = s.as_mut_ptr() as *mut *mut c_char;

                for (i, item) in self.iter().enumerate() {
                    *ptrs.add(i) = item.as_ptr() as *mut _;
                }
                *ptrs.add(self.len()) = ptr::null_mut();

                f(std::slice::from_raw_parts(ptrs, self.len()))
            }
        } else {
            let mut s = StrV::with_capacity(self.len());
            s.extend_from_slice(self);
            s.run_with_strv(f)
        }
    }
}

impl IntoStrV for &[&GStr] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        let required_len = (self.len() + 1) * mem::size_of::<*mut c_char>();

        if required_len < MAX_STACK_ALLOCATION * mem::size_of::<*mut c_char>() {
            unsafe {
                let mut s = mem::MaybeUninit::<[*mut c_char; MAX_STACK_ALLOCATION]>::uninit();
                let ptrs = s.as_mut_ptr() as *mut *mut c_char;

                for (i, item) in self.iter().enumerate() {
                    *ptrs.add(i) = item.as_ptr() as *mut _;
                }
                *ptrs.add(self.len()) = ptr::null_mut();

                f(std::slice::from_raw_parts(ptrs, self.len()))
            }
        } else {
            let mut s = StrV::with_capacity(self.len());
            s.extend_from_slice(self);
            s.run_with_strv(f)
        }
    }
}

impl IntoStrV for &[&str] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        let required_len = (self.len() + 1) * mem::size_of::<*mut c_char>()
            + self.iter().map(|s| s.len() + 1).sum::<usize>();

        if required_len < MAX_STACK_ALLOCATION * mem::size_of::<*mut c_char>() {
            unsafe {
                let mut s = mem::MaybeUninit::<[*mut c_char; MAX_STACK_ALLOCATION]>::uninit();
                let ptrs = s.as_mut_ptr() as *mut *mut c_char;
                let mut strs = ptrs.add(self.len() + 1) as *mut c_char;

                for (i, item) in self.iter().enumerate() {
                    ptr::copy_nonoverlapping(item.as_ptr() as *const _, strs, item.len());
                    *strs.add(item.len()) = 0;
                    *ptrs.add(i) = strs;
                    strs = strs.add(item.len() + 1);
                }
                *ptrs.add(self.len()) = ptr::null_mut();

                f(std::slice::from_raw_parts(ptrs, self.len()))
            }
        } else {
            let mut s = StrV::with_capacity(self.len());
            s.extend_from_slice(self);
            s.run_with_strv(f)
        }
    }
}

impl IntoStrV for &[String] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        let required_len = (self.len() + 1) * mem::size_of::<*mut c_char>()
            + self.iter().map(|s| s.len() + 1).sum::<usize>();

        if required_len < MAX_STACK_ALLOCATION * mem::size_of::<*mut c_char>() {
            unsafe {
                let mut s = mem::MaybeUninit::<[*mut c_char; MAX_STACK_ALLOCATION]>::uninit();
                let ptrs = s.as_mut_ptr() as *mut *mut c_char;
                let mut strs = ptrs.add(self.len() + 1) as *mut c_char;

                for (i, item) in self.iter().enumerate() {
                    ptr::copy_nonoverlapping(item.as_ptr() as *const _, strs, item.len());
                    *strs.add(item.len()) = 0;
                    *ptrs.add(i) = strs;
                    strs = strs.add(item.len() + 1);
                }
                *ptrs.add(self.len()) = ptr::null_mut();

                f(std::slice::from_raw_parts(ptrs, self.len()))
            }
        } else {
            let mut s = StrV::with_capacity(self.len());
            s.extend_from_slice(self);
            s.run_with_strv(f)
        }
    }
}

impl IntoStrV for &[&String] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        let required_len = (self.len() + 1) * mem::size_of::<*mut c_char>()
            + self.iter().map(|s| s.len() + 1).sum::<usize>();

        if required_len < MAX_STACK_ALLOCATION * mem::size_of::<*mut c_char>() {
            unsafe {
                let mut s = mem::MaybeUninit::<[*mut c_char; MAX_STACK_ALLOCATION]>::uninit();
                let ptrs = s.as_mut_ptr() as *mut *mut c_char;
                let mut strs = ptrs.add(self.len() + 1) as *mut c_char;

                for (i, item) in self.iter().enumerate() {
                    ptr::copy_nonoverlapping(item.as_ptr() as *const _, strs, item.len());
                    *strs.add(item.len()) = 0;
                    *ptrs.add(i) = strs;
                    strs = strs.add(item.len() + 1);
                }
                *ptrs.add(self.len()) = ptr::null_mut();

                f(std::slice::from_raw_parts(ptrs, self.len()))
            }
        } else {
            let mut s = StrV::with_capacity(self.len());
            s.extend_from_slice(self);
            s.run_with_strv(f)
        }
    }
}

impl<const N: usize> IntoStrV for [GString; N] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl<const N: usize> IntoStrV for [&'_ GString; N] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl<const N: usize> IntoStrV for [&'_ GStr; N] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl<const N: usize> IntoStrV for [&'_ str; N] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl<const N: usize> IntoStrV for [String; N] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

impl<const N: usize> IntoStrV for [&'_ String; N] {
    #[inline]
    fn run_with_strv<R, F: FnOnce(&[*mut c_char]) -> R>(self, f: F) -> R {
        self.as_slice().run_with_strv(f)
    }
}

// rustdoc-stripper-ignore-next
/// Representation of a borrowed `NULL`-terminated C array of `NULL`-terminated UTF-8 strings.
///
/// It can be constructed safely from a `&StrV` and unsafely from a pointer to a C array.
/// This type is very similar to `[GStringPtr]`, but with one added constraint: the underlying C array must be `NULL`-terminated.
#[repr(transparent)]
pub struct StrVRef {
    inner: [GStringPtr],
}

impl StrVRef {
    // rustdoc-stripper-ignore-next
    /// Borrows a C array.
    /// # Safety
    ///
    /// The provided pointer **must** be `NULL`-terminated. It is undefined behavior to
    /// pass a pointer that does not uphold this condition.
    #[inline]
    pub unsafe fn from_glib_borrow<'a>(ptr: *const *const c_char) -> &'a StrVRef {
        let slice = StrV::from_glib_borrow(ptr);
        &*(slice as *const [GStringPtr] as *const StrVRef)
    }

    // rustdoc-stripper-ignore-next
    /// Borrows a C array.
    /// # Safety
    ///
    /// The provided pointer **must** be `NULL`-terminated. It is undefined behavior to
    /// pass a pointer that does not uphold this condition.
    #[inline]
    pub unsafe fn from_glib_borrow_num<'a>(ptr: *const *const c_char, len: usize) -> &'a StrVRef {
        let slice = StrV::from_glib_borrow_num(ptr, len);
        &*(slice as *const [GStringPtr] as *const StrVRef)
    }

    // rustdoc-stripper-ignore-next
    /// Returns the underlying pointer.
    ///
    /// This is guaranteed to be nul-terminated.
    #[inline]
    pub const fn as_ptr(&self) -> *const *const c_char {
        self.inner.as_ptr() as *const *const _
    }
}

impl fmt::Debug for StrVRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

unsafe impl Send for StrVRef {}

unsafe impl Sync for StrVRef {}

impl PartialEq for StrVRef {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for StrVRef {}

impl PartialOrd for StrVRef {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StrVRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl std::hash::Hash for StrVRef {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl PartialEq<[&'_ str]> for StrVRef {
    fn eq(&self, other: &[&'_ str]) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (a, b) in Iterator::zip(self.iter(), other.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl PartialEq<StrVRef> for [&'_ str] {
    #[inline]
    fn eq(&self, other: &StrVRef) -> bool {
        other.eq(self)
    }
}

impl Default for &StrVRef {
    #[inline]
    fn default() -> Self {
        const SLICE: &[*const c_char] = &[ptr::null()];
        // SAFETY: `SLICE` is indeed a valid nul-terminated array.
        unsafe { StrVRef::from_glib_borrow(SLICE.as_ptr()) }
    }
}

impl std::ops::Deref for StrVRef {
    type Target = [GStringPtr];

    #[inline]
    fn deref(&self) -> &[GStringPtr] {
        &self.inner
    }
}

impl<'a> std::iter::IntoIterator for &'a StrVRef {
    type Item = &'a GStringPtr;
    type IntoIter = std::slice::Iter<'a, GStringPtr>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> From<&'a StrV> for &'a StrVRef {
    fn from(value: &'a StrV) -> Self {
        let slice = value.as_slice();
        // Safety: `&StrV` is a null-terminated C array of nul-terminated UTF-8 strings,
        // therefore `&StrV::as_slice()` return a a null-terminated slice of nul-terminated UTF-8 strings,
        // thus it is safe to convert it to `&CStr`.
        unsafe { &*(slice as *const [GStringPtr] as *const StrVRef) }
    }
}

impl FromGlibContainer<*mut c_char, *const *const c_char> for &StrVRef {
    unsafe fn from_glib_none_num(ptr: *const *const c_char, num: usize) -> Self {
        StrVRef::from_glib_borrow_num(ptr, num)
    }

    unsafe fn from_glib_container_num(_ptr: *const *const c_char, _num: usize) -> Self {
        unimplemented!();
    }

    unsafe fn from_glib_full_num(_ptr: *const *const c_char, _num: usize) -> Self {
        unimplemented!();
    }
}

impl FromGlibPtrContainer<*mut c_char, *const *const c_char> for &StrVRef {
    #[inline]
    unsafe fn from_glib_none(ptr: *const *const c_char) -> Self {
        StrVRef::from_glib_borrow(ptr)
    }

    unsafe fn from_glib_container(_ptr: *const *const c_char) -> Self {
        unimplemented!();
    }

    unsafe fn from_glib_full(_ptr: *const *const c_char) -> Self {
        unimplemented!();
    }
}

impl<'a> ToGlibPtr<'a, *const *const c_char> for StrVRef {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const *const c_char, Self> {
        Stash(self.as_ptr(), PhantomData)
    }
}

impl IntoGlibPtr<*const *const c_char> for &StrVRef {
    #[inline]
    fn into_glib_ptr(self) -> *const *const c_char {
        self.as_ptr()
    }
}

impl StaticType for StrVRef {
    #[inline]
    fn static_type() -> crate::Type {
        <Vec<String>>::static_type()
    }
}

unsafe impl<'a> crate::value::FromValue<'a> for &'a StrVRef {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a crate::value::Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
        StrVRef::from_glib_borrow(ptr)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_glib_full() {
        let items = ["str1", "str2", "str3", "str4"];

        let slice = unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<*mut c_char>() * 4) as *mut *mut c_char;
            *ptr.add(0) = items[0].to_glib_full();
            *ptr.add(1) = items[1].to_glib_full();
            *ptr.add(2) = items[2].to_glib_full();
            *ptr.add(3) = items[3].to_glib_full();

            StrV::from_glib_full_num(ptr, 4, false)
        };

        assert_eq!(items.len(), slice.len());
        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_from_glib_container() {
        let items = [
            crate::gstr!("str1"),
            crate::gstr!("str2"),
            crate::gstr!("str3"),
            crate::gstr!("str4"),
        ];

        let slice = unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<*mut c_char>() * 4) as *mut *const c_char;
            *ptr.add(0) = items[0].as_ptr();
            *ptr.add(1) = items[1].as_ptr();
            *ptr.add(2) = items[2].as_ptr();
            *ptr.add(3) = items[3].as_ptr();

            StrV::from_glib_container_num(ptr, 4, false)
        };

        assert_eq!(items.len(), slice.len());
        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_from_glib_none() {
        let items = [
            crate::gstr!("str1"),
            crate::gstr!("str2"),
            crate::gstr!("str3"),
            crate::gstr!("str4"),
        ];

        let slice = unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<*mut c_char>() * 4) as *mut *const c_char;
            *ptr.add(0) = items[0].as_ptr();
            *ptr.add(1) = items[1].as_ptr();
            *ptr.add(2) = items[2].as_ptr();
            *ptr.add(3) = items[3].as_ptr();

            let res = StrV::from_glib_none_num(ptr, 4, false);
            ffi::g_free(ptr as ffi::gpointer);
            res
        };

        assert_eq!(items.len(), slice.len());
        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_from_slice() {
        let items = [
            crate::gstr!("str1"),
            crate::gstr!("str2"),
            crate::gstr!("str3"),
        ];

        let slice1 = StrV::from(&items[..]);
        let slice2 = StrV::from(items);
        assert_eq!(slice1.len(), 3);
        assert_eq!(slice1, slice2);
    }

    #[test]
    fn test_safe_api() {
        let items = [
            crate::gstr!("str1"),
            crate::gstr!("str2"),
            crate::gstr!("str3"),
        ];

        let mut slice = StrV::from(&items[..]);
        assert_eq!(slice.len(), 3);
        slice.push(GString::from("str4"));
        assert_eq!(slice.len(), 4);

        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a, b);
        }
        assert_eq!(slice[3], "str4");

        let vec = Vec::from(slice);
        assert_eq!(vec.len(), 4);
        for (a, b) in Iterator::zip(items.iter(), vec.iter()) {
            assert_eq!(a, b);
        }
        assert_eq!(vec[3], "str4");

        let mut slice = StrV::from(vec);
        assert_eq!(slice.len(), 4);
        let e = slice.pop().unwrap();
        assert_eq!(e, "str4");
        assert_eq!(slice.len(), 3);
        slice.insert(2, e);
        assert_eq!(slice.len(), 4);
        assert_eq!(slice[0], "str1");
        assert_eq!(slice[1], "str2");
        assert_eq!(slice[2], "str4");
        assert_eq!(slice[3], "str3");
        let e = slice.remove(2);
        assert_eq!(e, "str4");
        assert_eq!(slice.len(), 3);
        slice.push(e);
        assert_eq!(slice.len(), 4);

        for (a, b) in Iterator::zip(items.iter(), slice.into_iter()) {
            assert_eq!(*a, b);
        }
    }

    #[test]
    fn test_into_strv() {
        let items = ["str1", "str2", "str3", "str4"];

        items[..].run_with_strv(|s| unsafe {
            assert!((*s.as_ptr().add(4)).is_null());
            assert_eq!(s.len(), items.len());
            let s = StrV::from_glib_borrow(s.as_ptr() as *const *const c_char);
            assert_eq!(s, items);
        });

        Vec::from(&items[..]).run_with_strv(|s| unsafe {
            assert!((*s.as_ptr().add(4)).is_null());
            assert_eq!(s.len(), items.len());
            let s = StrV::from_glib_borrow(s.as_ptr() as *const *const c_char);
            assert_eq!(s, items);
        });

        StrV::from(&items[..]).run_with_strv(|s| unsafe {
            assert!((*s.as_ptr().add(4)).is_null());
            assert_eq!(s.len(), items.len());
            let s = StrV::from_glib_borrow(s.as_ptr() as *const *const c_char);
            assert_eq!(s, items);
        });

        let v = items.iter().copied().map(String::from).collect::<Vec<_>>();
        items.run_with_strv(|s| unsafe {
            assert!((*s.as_ptr().add(4)).is_null());
            assert_eq!(s.len(), v.len());
            let s = StrV::from_glib_borrow(s.as_ptr() as *const *const c_char);
            assert_eq!(s, items);
        });

        let v = items.iter().copied().map(GString::from).collect::<Vec<_>>();
        items.run_with_strv(|s| unsafe {
            assert!((*s.as_ptr().add(4)).is_null());
            assert_eq!(s.len(), v.len());
            let s = StrV::from_glib_borrow(s.as_ptr() as *const *const c_char);
            assert_eq!(s, items);
        });
    }

    #[test]
    fn test_join() {
        let items = [
            crate::gstr!("str1"),
            crate::gstr!("str2"),
            crate::gstr!("str3"),
        ];

        let strv = StrV::from(&items[..]);
        assert_eq!(strv.join(None::<&str>), "str1str2str3");
        assert_eq!(strv.join(Some(",")), "str1,str2,str3");
    }

    #[test]
    fn test_contains() {
        let items = [
            crate::gstr!("str1"),
            crate::gstr!("str2"),
            crate::gstr!("str3"),
        ];

        let strv = StrV::from(&items[..]);
        assert!(strv.contains("str2"));
        assert!(!strv.contains("str4"));
    }

    #[test]
    #[should_panic]
    fn test_reserve_overflow() {
        let mut strv = StrV::from(&[crate::gstr!("foo"); 3][..]);

        // An old implementation of `reserve` used the condition `self.len +
        // additional + 1 <= self.capacity`, which was prone to overflow
        strv.reserve(usize::MAX - 3);
    }

    #[test]
    #[should_panic]
    fn test_extend_from_slice_overflow() {
        // We need a zero-sized type because only a slice of ZST can legally
        // contain up to `usize::MAX` elements.
        #[derive(Clone, Copy)]
        struct ImplicitStr;

        impl AsRef<str> for ImplicitStr {
            fn as_ref(&self) -> &str {
                ""
            }
        }

        let mut strv = StrV::from(&[crate::gstr!(""); 3][..]);

        // An old implementation of `extend_from_slice` used the condition
        // `self.len + other.len() + 1 <= self.capacity`, which was prone to
        // overflow
        strv.extend_from_slice(&[ImplicitStr; usize::MAX - 3]);
    }

    #[test]
    fn test_extend_from_slice_panic_safe() {
        struct MayPanic(bool);

        impl AsRef<str> for MayPanic {
            fn as_ref(&self) -> &str {
                if self.0 {
                    panic!("panicking as per request");
                } else {
                    ""
                }
            }
        }

        let mut strv = StrV::from(&[crate::gstr!(""); 3][..]);
        strv.clear();

        // Write one element and panic while getting the second element
        _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            strv.extend_from_slice(&[MayPanic(false), MayPanic(true)]);
        }));

        // Check that it contains up to one element is null-terminated
        assert!(strv.len() <= 1);
        unsafe {
            for i in 0..strv.len() {
                assert!(!(*strv.as_ptr().add(i)).is_null());
            }
            assert!((*strv.as_ptr().add(strv.len())).is_null());
        }
    }

    #[test]
    fn test_strv_ref_eq_str_slice() {
        let strv = StrV::from(&[crate::gstr!("a")][..]);
        let strv_ref: &StrVRef = strv.as_ref();

        // Test `impl PartialEq<[&'_ str]> for StrVRef`
        assert_eq!(strv_ref, &["a"][..]);
        assert_ne!(strv_ref, &[][..]);
        assert_ne!(strv_ref, &["a", "b"][..]);
        assert_ne!(strv_ref, &["b"][..]);
    }
}
