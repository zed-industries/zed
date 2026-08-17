use core::{
    cmp, fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ptr::{self, NonNull},
};

use crate::{TagNonNull, TagPtr};

/********** impl Clone ****************************************************************************/

impl<T, const N: usize> Clone for TagPtr<T, N> {
    impl_clone!();
}

/********** impl Copy *****************************************************************************/

impl<T, const N: usize> Copy for TagPtr<T, N> {}

/********** impl inherent *************************************************************************/

impl<T, const N: usize> TagPtr<T, N> {
    doc_comment! {
        doc_tag_bits!(),
        pub const TAG_BITS: usize = N;
    }

    doc_comment! {
        doc_tag_mask!(),
        pub const TAG_MASK: usize = crate::mark_mask(Self::TAG_BITS);
    }

    doc_comment! {
        doc_ptr_mask!(),
        pub const POINTER_MASK: usize = !Self::TAG_MASK;
    }

    doc_comment! {
        doc_null!(),
        ///
        /// # Examples
        ///
        /// ```
        /// use core::ptr;
        ///
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let ptr = TagPtr::null();
        /// assert_eq!(ptr.decompose(), (ptr::null_mut(), 0));
        /// ```
        #[inline]
        pub const fn null() -> Self {
            Self::new(ptr::null_mut())
        }
    }

    doc_comment! {
        doc_new!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let reference = &mut 1;
        /// let ptr = TagPtr::new(reference);
        /// assert_eq!(ptr.decompose(), (reference as *mut _, 0));
        /// ```
        #[inline]
        pub const fn new(ptr: *mut T) -> Self {
            Self { inner: ptr, _marker: PhantomData }
        }
    }

    doc_comment! {
        doc_from_usize!(),
        ///
        /// # Examples
        ///
        /// ```
        /// use core::ptr;
        ///
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let ptr = TagPtr::from_usize(0b11);
        /// assert_eq!(ptr.decompose(), (ptr::null_mut(), 0b11));
        /// ```
        #[inline]
        pub const fn from_usize(val: usize) -> Self {
            Self::new(val as _)
        }
    }

    doc_comment! {
        doc_into_raw!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let ptr = TagPtr::from_usize(0b11);
        /// assert_eq!(ptr.into_raw(), 0b11 as *mut _);
        /// ```
        #[inline]
        pub const fn into_raw(self) -> *mut T {
            self.inner
        }
    }

    doc_comment! {
        doc_cast!(),
        pub const fn cast<U>(self) -> TagPtr<U, N> {
            TagPtr { inner: self.inner.cast(), _marker: PhantomData }
        }
    }

    doc_comment! {
        doc_into_usize!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let ptr = TagPtr::from_usize(0b11);
        /// assert_eq!(ptr.into_usize(), 0b11);
        /// ```
        #[inline]
        pub fn into_usize(self) -> usize {
            self.inner as usize
        }
    }

    doc_comment! {
        doc_compose!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let raw = &1 as *const i32 as *mut i32;
        /// let ptr = TagPtr::compose(raw, 0b11);
        /// assert_eq!(ptr.decompose(), (raw, 0b11));
        /// // excess bits are silently truncated
        /// let ptr = TagPtr::compose(raw, 0b101);
        /// assert_eq!(ptr.decompose(), (raw, 0b01));
        /// ```
        #[inline]
        pub fn compose(ptr: *mut T, tag: usize) -> Self {
            Self::new(crate::compose::<T, N>(ptr, tag))
        }
    }

    /// Returns `true` if the marked pointer is `null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ptr;
    ///
    /// type TagPtr = tagptr::TagPtr<i32, 2>;
    ///
    /// let ptr = TagPtr::compose(ptr::null_mut(), 0b11);
    /// assert!(ptr.is_null());
    /// ```
    #[inline]
    pub fn is_null(self) -> bool {
        self.decompose_ptr().is_null()
    }

    doc_comment! {
        doc_clear_tag!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let reference = &mut 1;
        /// let ptr = TagPtr::compose(reference, 0b11);
        ///
        /// assert_eq!(ptr.clear_tag().decompose(), (reference as *mut _, 0));
        /// ```
        #[inline]
        pub fn clear_tag(self) -> Self {
            Self::new(self.decompose_ptr())
        }
    }

    doc_comment! {
        doc_split_tag!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let reference = &mut 1;
        /// let ptr = TagPtr::compose(reference, 0b11);
        ///
        /// assert_eq!(ptr.split_tag(), (TagPtr::new(reference), 0b11));
        /// ```
        #[inline]
        pub fn split_tag(self) -> (Self, usize) {
            let (ptr, tag) = self.decompose();
            (Self::new(ptr), tag)
        }
    }

    doc_comment! {
        doc_set_tag!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let reference = &mut 1;
        /// let ptr = TagPtr::compose(reference, 0b11);
        ///
        /// assert_eq!(ptr.set_tag(0b01).decompose(), (reference as *mut _, 0b01));
        /// ```
        #[inline]
        pub fn set_tag(self, tag: usize) -> Self {
            let ptr = self.decompose_ptr();
            Self::compose(ptr, tag)
        }
    }

    doc_comment! {
        doc_update_tag!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let reference = &mut 1;
        /// let ptr = TagPtr::compose(reference, 0b11);
        ///
        /// assert_eq!(ptr.update_tag(|tag| tag - 1).decompose(), (reference as *mut _, 0b10));
        /// ```
        #[inline]
        pub fn update_tag(self, func: impl FnOnce(usize) -> usize) -> Self {
            let (ptr, tag) = self.decompose();
            Self::compose(ptr, func(tag))
        }
    }

    doc_comment! {
        doc_add_tag!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let reference = &mut 1;
        /// let ptr = TagPtr::compose(reference, 0b10);
        ///
        /// assert_eq!(ptr.add_tag(1).decompose(), (reference as *mut _, 0b11));
        /// ```
        #[inline]
        pub fn add_tag(self, value: usize) -> Self {
            Self::from_usize(self.into_usize().wrapping_add(value))
        }
    }

    doc_comment! {
        doc_sub_tag!(),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let reference = &mut 1;
        /// let ptr = TagPtr::compose(reference, 0b10);
        ///
        /// assert_eq!(ptr.sub_tag(1).decompose(), (reference as *mut _, 0b01));
        /// ```
        #[inline]
        pub fn sub_tag(self, value: usize) -> Self {
            Self::from_usize(self.into_usize().wrapping_sub(value))
        }
    }

    doc_comment! {
        doc_decompose!(),
        #[inline]
        pub fn decompose(self) -> (*mut T, usize) {
            (self.decompose_ptr(), self.decompose_tag())
        }
    }

    doc_comment! {
        doc_decompose_ptr!(),
        #[inline]
        pub fn decompose_ptr(self) -> *mut T {
            crate::decompose_ptr::<T>(self.inner as usize, Self::TAG_BITS)
        }
    }

    doc_comment! {
        doc_decompose_tag!(),
        #[inline]
        pub fn decompose_tag(self) -> usize {
            crate::decompose_tag::<T>(self.inner as usize, Self::TAG_BITS)
        }
    }

    doc_comment! {
        doc_as_ref!("nullable"),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let reference = &1;
        /// let ptr = TagPtr::compose(reference as *const _ as *mut _, 0b11);
        ///
        /// unsafe {
        ///     assert_eq!(ptr.as_ref(), Some(&1));
        /// }
        /// ```
        #[inline]
        pub unsafe fn as_ref<'a>(self) -> Option<&'a T> {
            self.decompose_ptr().as_ref()
        }
    }

    doc_comment! {
        doc_as_mut!("nullable", TagPtr),
        ///
        /// # Examples
        ///
        /// ```
        /// type TagPtr = tagptr::TagPtr<i32, 2>;
        ///
        /// let mut val = 1;
        /// let ptr = TagPtr::compose(&mut val, 0b11);
        ///
        /// unsafe {
        ///     assert_eq!(ptr.as_mut(), Some(&mut 1));
        /// }
        /// ```
        #[inline]
        pub unsafe fn as_mut<'a>(self) -> Option<&'a mut T> {
            self.decompose_ptr().as_mut()
        }
    }

    /// Decomposes the marked pointer, returning an optional reference and the
    /// separated tag.
    ///
    /// # Safety
    ///
    /// The same safety caveats as with [`as_ref`][TagPtr::as_ref] apply.
    ///
    /// # Examples
    ///
    /// ```
    /// type TagPtr = tagptr::TagPtr<i32, 2>;
    ///
    /// let reference = &1;
    /// let ptr = TagPtr::compose(reference as *const _ as *mut _, 0b11);
    ///
    /// unsafe {
    ///     assert_eq!(ptr.decompose_ref(), (Some(&1), 0b11));
    /// }
    /// ```
    #[inline]
    pub unsafe fn decompose_ref<'a>(self) -> (Option<&'a T>, usize) {
        (self.as_ref(), self.decompose_tag())
    }

    /// Decomposes the marked pointer, returning an optional *mutable* reference
    /// and the separated tag.
    ///
    /// # Safety
    ///
    /// The same safety caveats as with [`as_mut`][TagPtr::as_mut] apply.
    ///
    /// # Examples
    ///
    /// ```
    /// type TagPtr = tagptr::TagPtr<i32, 2>;
    ///
    /// let mut val = 1;
    /// let ptr = TagPtr::compose(&mut val, 0b11);
    ///
    /// unsafe {
    ///     assert_eq!(ptr.decompose_mut(), (Some(&mut 1), 0b11));
    /// }
    /// ```
    #[inline]
    pub unsafe fn decompose_mut<'a>(self) -> (Option<&'a mut T>, usize) {
        (self.as_mut(), self.decompose_tag())
    }
}

/********** impl Debug ****************************************************************************/

impl<T, const N: usize> fmt::Debug for TagPtr<T, N> {
    impl_debug!("TagPtr");
}

/********** impl Default **************************************************************************/

impl<T, const N: usize> Default for TagPtr<T, N> {
    impl_default!();
}

/********** impl From (*mut T) ********************************************************************/

impl<T, const N: usize> From<*mut T> for TagPtr<T, N> {
    #[inline]
    fn from(ptr: *mut T) -> Self {
        Self::new(ptr)
    }
}

/********** impl From (*const T) ******************************************************************/

impl<T, const N: usize> From<*const T> for TagPtr<T, N> {
    #[inline]
    fn from(ptr: *const T) -> Self {
        Self::new(ptr as _)
    }
}

/********** impl From (&T) ************************************************************************/

impl<T, const N: usize> From<&T> for TagPtr<T, N> {
    #[inline]
    fn from(reference: &T) -> Self {
        Self::from(reference as *const _)
    }
}

/********** impl From ((&T, usize)) ***************************************************************/

impl<T, const N: usize> From<(&T, usize)> for TagPtr<T, N> {
    #[inline]
    fn from((reference, tag): (&T, usize)) -> Self {
        Self::compose(reference as *const T as *mut T, tag)
    }
}

/********** impl From (&mut T) ********************************************************************/

impl<T, const N: usize> From<&mut T> for TagPtr<T, N> {
    #[inline]
    fn from(reference: &mut T) -> Self {
        Self::from(reference as *const _)
    }
}

/********** impl From ((&mut T, usize)) ***********************************************************/

impl<T, const N: usize> From<(&mut T, usize)> for TagPtr<T, N> {
    #[inline]
    fn from((reference, tag): (&mut T, usize)) -> Self {
        Self::compose(reference, tag)
    }
}

/********** impl From (NonNull) *******************************************************************/

impl<T, const N: usize> From<NonNull<T>> for TagPtr<T, N> {
    #[inline]
    fn from(ptr: NonNull<T>) -> Self {
        Self::new(ptr.as_ptr())
    }
}

/********** impl From (TagNonNull) *************************************************************/

impl<T, const N: usize> From<TagNonNull<T, N>> for TagPtr<T, N> {
    #[inline]
    fn from(ptr: TagNonNull<T, N>) -> Self {
        ptr.into_marked_ptr()
    }
}

/********** impl PartialEq ************************************************************************/

impl<T, const N: usize> PartialEq for TagPtr<T, N> {
    impl_partial_eq!();
}

/********** impl PartialOrd ***********************************************************************/

impl<T, const N: usize> PartialOrd for TagPtr<T, N> {
    impl_partial_ord!();
}

/********** impl Pointer **************************************************************************/

impl<T, const N: usize> fmt::Pointer for TagPtr<T, N> {
    impl_pointer!();
}

/********** impl Eq *******************************************************************************/

impl<T, const N: usize> Eq for TagPtr<T, N> {}

/********** impl Ord ******************************************************************************/

impl<T, const N: usize> Ord for TagPtr<T, N> {
    impl_ord!();
}

/********** impl Hash *****************************************************************************/

impl<T, const N: usize> Hash for TagPtr<T, N> {
    impl_hash!();
}

#[cfg(test)]
mod tests {
    type TagPtr = crate::TagPtr<i32, 2>;

    #[test]
    fn test_debug() {
        let reference = &mut 1;
        let ptr = TagPtr::compose(reference, 0b11);
        assert_eq!(
            std::format!("{:?}", ptr),
            std::format!("TagPtr {{ ptr: {:0p}, tag: {} }}", reference as *mut _, 0b11)
        );
    }

    #[test]
    fn test_cast() {
        type ErasedPtr = crate::TagPtr<(), 2>;

        let reference = &mut 1;
        let ptr = TagPtr::compose(reference, 0b11);
        let cast: ErasedPtr = ptr.cast().set_tag(0b10);

        assert_eq!(cast.into_usize(), reference as *mut _ as usize | 0b10);
        assert_eq!(cast.cast(), TagPtr::compose(reference, 0b10));
    }

    #[test]
    fn test_from_usize() {
        let reference = &1;
        let ptr = TagPtr::from_usize(reference as *const i32 as usize | 0b1);
        assert_eq!(ptr.decompose(), (reference as *const _ as *mut _, 0b1));
    }

    #[test]
    fn test_compose() {
        let reference = &mut 1;
        let ptr1 = TagPtr::compose(reference, 0b11);
        let ptr2 = TagPtr::compose(reference, 0b111);
        // compose silently truncates excess bits, so ptr1 and ptr2 are identical
        assert_eq!(ptr1, ptr2);
        assert_eq!(ptr2.decompose(), (reference as *mut _, 0b11));
    }

    #[test]
    fn test_set_tag() {
        let reference = &mut 1;
        let ptr = TagPtr::compose(reference, 0b11);
        // set_tag must silently truncate excess tag bits
        assert_eq!(ptr, ptr.set_tag(0b111));
    }

    #[test]
    fn test_overflow_tag() {
        let reference = &mut 1;
        let ptr = TagPtr::compose(reference, 0b11);

        // add must cause overflow (corrupt the pointer)
        assert_eq!(ptr.add_tag(1).into_usize(), reference as *mut _ as usize + 0b11 + 1);
        // update must only overflow the tag bits
        assert_eq!(ptr.update_tag(|tag| tag + 1).decompose(), (reference as *mut _, 0));
    }

    #[test]
    fn test_underflow_tag() {
        let reference = &mut 1;
        let ptr = TagPtr::new(reference);

        // sub_tag must underflow the entire pointer
        assert_eq!(ptr.sub_tag(1).into_usize(), reference as *mut _ as usize - 1);
        // update_tag must only underflow the tag value
        assert_eq!(
            ptr.update_tag(|tag| tag.wrapping_sub(1)).decompose(),
            (reference as *mut _, 0b11)
        );
    }

    #[test]
    fn test_erase() {
        #[repr(align(64))]
        struct Aligned64(i32);

        let reference = &Aligned64(1);
        let ptr = crate::TagPtr::<Aligned64, 6>::from((reference, 55));
        let mut erased: crate::TagPtr<(), 6> = ptr.cast();
        erased = erased.update_tag(|tag| tag + 3);
        let ptr: crate::TagPtr<Aligned64, 6> = erased.cast();

        assert_eq!(ptr.decompose(), (reference as *const _ as *mut _, 58));
    }
}
