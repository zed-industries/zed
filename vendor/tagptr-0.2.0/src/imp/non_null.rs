use core::{
    cmp,
    convert::TryFrom,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    ptr::NonNull,
};

use crate::{Null, TagNonNull, TagPtr};

/********** impl Clone ****************************************************************************/

impl<T, const N: usize> Clone for TagNonNull<T, N> {
    impl_clone!();
}

/********** impl Copy *****************************************************************************/

impl<T, const N: usize> Copy for TagNonNull<T, N> {}

/********** impl inherent *************************************************************************/

impl<T, const N: usize> TagNonNull<T, N> {
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

    const COMPOSE_ERR_MSG: &'static str =
        "argument `ptr` is mis-aligned for `N` tag bits and could be parsed as marked `null` \
        pointer.";

    /// Creates a new marked non-null pointer from `marked_ptr` without
    /// checking if it is `null`.
    ///
    /// # Safety
    ///
    /// The caller has to ensure that `marked_ptr` is not `null`.
    /// This includes `null` pointers with non-zero tag values.
    #[inline]
    pub const unsafe fn new_unchecked(marked_ptr: TagPtr<T, N>) -> Self {
        Self { inner: NonNull::new_unchecked(marked_ptr.inner), _marker: PhantomData }
    }

    doc_comment! {
        doc_from_usize!(),
        #[inline]
        pub const unsafe fn from_usize(val: usize) -> Self {
            Self { inner: NonNull::new_unchecked(val as *mut _), _marker: PhantomData }
        }
    }

    doc_comment! {
        doc_into_raw!(),
        #[inline]
        pub const fn into_raw(self) -> NonNull<T> {
            self.inner
        }
    }

    doc_comment! {
        doc_cast!(),
        pub const fn cast<U>(self) -> TagNonNull<U, N> {
            TagNonNull { inner: self.inner.cast(), _marker: PhantomData }
        }
    }

    doc_comment! {
        doc_into_usize!(),
        #[inline]
        pub fn into_usize(self) -> usize {
            self.inner.as_ptr() as _
        }
    }

    /// Converts `self` into a (nullable) marked pointer.
    #[inline]
    pub const fn into_marked_ptr(self) -> TagPtr<T, N> {
        TagPtr::new(self.inner.as_ptr())
    }

    /// Creates a new non-null pointer from `marked_ptr`.
    ///
    /// # Errors
    ///
    /// Fails if `marked_ptr` is `null`, in which case a [`Null`] instance is
    /// returned containing argument pointer's tag value.
    #[inline]
    pub fn new(marked_ptr: TagPtr<T, N>) -> Result<Self, Null> {
        Self::try_from(marked_ptr)
    }

    /// Creates a new pointer that is dangling but well aligned.
    #[inline]
    pub const fn dangling() -> Self {
        let alignment = mem::align_of::<T>();
        let val = if alignment >= Self::TAG_MASK + 1 { alignment } else { Self::TAG_MASK + 1 };
        // SAFETY: a type's alignment is never 0, so val is always non-zero
        unsafe { Self::from_usize(val) }
    }

    doc_comment! {
        doc_compose!(),
        /// # Panics
        ///
        /// Panics if `ptr` is mis-aligned for `N` tag bits and contains only
        /// zero bits in the upper bits, i.e., it would be parsed as a marked
        /// `null` pointer.
        #[inline]
        pub fn compose(ptr: NonNull<T>, tag: usize) -> Self {
            Self::try_compose(ptr, tag).expect(Self::COMPOSE_ERR_MSG)
        }
    }

    /// Attempts to compose a new marked pointer from a raw (non-null) `ptr` and
    /// a `tag` value.
    ///
    /// # Errors
    ///
    /// Fails if `ptr` is mis-aligned for `N` tag bits and contains only
    /// zero bits in the upper bits, i.e., it would be parsed as a marked
    /// `null` pointer.
    /// In this case a [`Null`] instance is returned containing the argument
    /// pointer's tag value.
    #[inline]
    pub fn try_compose(ptr: NonNull<T>, tag: usize) -> Result<Self, Null> {
        Self::try_compose_inner(ptr.as_ptr(), tag)
    }

    /// Composes a new marked pointer from a raw (non-null) `ptr` and a `tag`
    /// value without checking if `ptr` is valid.
    ///
    /// # Safety
    ///
    /// The caller has to ensure that `ptr` is non-null even after considering
    /// its `N` lower bits as tag bits.
    #[inline]
    pub unsafe fn compose_unchecked(ptr: NonNull<T>, tag: usize) -> Self {
        Self::new_unchecked(TagPtr::compose(ptr.as_ptr(), tag))
    }

    doc_comment! {
        doc_clear_tag!(),
        #[inline]
        pub fn clear_tag(self) -> Self {
            Self { inner: self.decompose_non_null(), _marker: PhantomData }
        }
    }

    doc_comment! {
        doc_split_tag!(),
        #[inline]
        pub fn split_tag(self) -> (Self, usize) {
            let (inner, tag) = self.decompose();
            (Self { inner, _marker: PhantomData }, tag)
        }
    }

    doc_comment! {
        doc_set_tag!(),
        #[inline]
        pub fn set_tag(self, tag: usize) -> Self {
            let ptr = self.decompose_non_null();
            // SAFETY: ptr was decomposed from a valid marked non-nullable pointer
            unsafe { Self::compose_unchecked(ptr, tag) }
        }
    }

    doc_comment! {
        doc_update_tag!(),
        #[inline]
        pub fn update_tag(self, func: impl FnOnce(usize) -> usize) -> Self {
            let (ptr, tag) = self.decompose();
            // SAFETY: ptr was decomposed from a valid marked non-nullable pointer
            unsafe { Self::compose_unchecked(ptr, func(tag)) }
        }
    }

    doc_comment! {
        doc_add_tag!(),
        /// # Safety
        ///
        /// The caller has to ensure that the resulting pointer is not
        /// `null` (neither marked nor unmarked).
        #[inline]
        pub unsafe fn add_tag(self, value: usize) -> Self {
            Self::from_usize(self.into_usize().wrapping_add(value))
        }
    }

    doc_comment! {
        doc_sub_tag!(),
        /// # Safety
        ///
        /// The caller has to ensure that the resulting pointer is not
        /// `null` (neither marked nor unmarked).
        #[inline]
        pub unsafe fn sub_tag(self, value: usize) -> Self {
            Self::from_usize(self.into_usize().wrapping_sub(value))
        }
    }

    doc_comment! {
        doc_decompose!(),
        #[inline]
        pub fn decompose(self) -> (NonNull<T>, usize) {
            (self.decompose_non_null(), self.decompose_tag())
        }
    }

    doc_comment! {
        doc_decompose_ptr!(),
        #[inline]
        pub fn decompose_ptr(self) -> *mut T {
            crate::decompose_ptr(self.inner.as_ptr() as usize, Self::TAG_BITS)
        }
    }

    doc_comment! {
        doc_decompose_non_null!(),
        #[inline]
        pub fn decompose_non_null(self) -> NonNull<T> {
            // SAFETY: every valid TagNonNull is also a valid NonNull
            unsafe { NonNull::new_unchecked(self.decompose_ptr()) }
        }
    }

    doc_comment! {
        doc_decompose_tag!(),
        #[inline]
        pub fn decompose_tag(self) -> usize {
            crate::decompose_tag::<T>(self.inner.as_ptr() as usize, Self::TAG_BITS)
        }
    }

    doc_comment! {
        doc_as_ref!("non-nullable"),
        #[inline]
        pub unsafe fn as_ref(&self) -> &T {
            &*self.decompose_non_null().as_ptr()
        }
    }

    doc_comment! {
        doc_as_mut!("non-nullable", TagNonNull),
        #[inline]
        pub unsafe fn as_mut(&mut self) -> &mut T {
            &mut *self.decompose_non_null().as_ptr()
        }
    }

    /// Decomposes the marked pointer, returning a reference and the separated
    /// tag.
    ///
    /// # Safety
    ///
    /// The same safety caveats as with [`as_ref`][TagNonNull::as_ref] apply.
    #[inline]
    pub unsafe fn decompose_ref(&self) -> (&T, usize) {
        let (ptr, tag) = self.decompose();
        (&*ptr.as_ptr(), tag)
    }

    /// Decomposes the marked pointer, returning a *mutable* reference and the
    /// separated tag.
    ///
    /// # Safety
    ///
    /// The same safety caveats as with [`as_mut`][TagNonNull::as_mut] apply.
    #[inline]
    pub unsafe fn decompose_mut(&mut self) -> (&mut T, usize) {
        let (ptr, tag) = self.decompose();
        (&mut *ptr.as_ptr(), tag)
    }

    #[inline]
    fn try_compose_inner(ptr: *mut T, tag: usize) -> Result<Self, Null> {
        match ptr as usize & Self::POINTER_MASK {
            0 => Err(Null(ptr as usize)),
            // SAFETY: the pointer's upper bits are non-zero,
            _ => Ok(unsafe { Self::new_unchecked(TagPtr::compose(ptr, tag)) }),
        }
    }
}

/********** impl Debug ****************************************************************************/

impl<T, const N: usize> fmt::Debug for TagNonNull<T, N> {
    impl_debug!("TagNonNull");
}

/********** impl Pointer **************************************************************************/

impl<T, const N: usize> fmt::Pointer for TagNonNull<T, N> {
    impl_pointer!();
}

/********** impl From (&T) ************************************************************************/

impl<T, const N: usize> From<&T> for TagNonNull<T, N> {
    #[inline]
    fn from(reference: &T) -> Self {
        Self { inner: NonNull::from(reference), _marker: PhantomData }
    }
}

/********** impl From (&mut T) ********************************************************************/

impl<T, const N: usize> From<&mut T> for TagNonNull<T, N> {
    #[inline]
    fn from(reference: &mut T) -> Self {
        Self { inner: NonNull::from(reference), _marker: PhantomData }
    }
}

/********** impl PartialEq ************************************************************************/

impl<T, const N: usize> PartialEq for TagNonNull<T, N> {
    impl_partial_eq!();
}

/********** impl PartialOrd ***********************************************************************/

impl<T, const N: usize> PartialOrd for TagNonNull<T, N> {
    impl_partial_ord!();
}

/********** impl Eq *******************************************************************************/

impl<T, const N: usize> Eq for TagNonNull<T, N> {}

/********** impl Ord ******************************************************************************/

impl<T, const N: usize> Ord for TagNonNull<T, N> {
    impl_ord!();
}

/********** impl Hash *****************************************************************************/

impl<T, const N: usize> Hash for TagNonNull<T, N> {
    impl_hash!();
}

/********** impl TryFrom (*mut T) *****************************************************************/

impl<T, const N: usize> TryFrom<*mut T> for TagNonNull<T, N> {
    type Error = Null;

    #[inline]
    fn try_from(ptr: *mut T) -> Result<Self, Self::Error> {
        Self::try_compose_inner(ptr, 0)
    }
}

/********** impl TryFrom (*const T) ***************************************************************/

impl<T, const N: usize> TryFrom<*const T> for TagNonNull<T, N> {
    type Error = Null;

    #[inline]
    fn try_from(ptr: *const T) -> Result<Self, Self::Error> {
        Self::try_from(ptr as *mut _)
    }
}

/********** impl TryFrom (TagPtr) **************************************************************/

impl<T, const N: usize> TryFrom<TagPtr<T, N>> for TagNonNull<T, N> {
    type Error = Null;

    #[inline]
    fn try_from(ptr: TagPtr<T, N>) -> Result<Self, Self::Error> {
        Self::try_from(ptr.into_raw())
    }
}

/********** impl TryFrom (NonNull) ****************************************************************/

impl<T, const N: usize> TryFrom<NonNull<T>> for TagNonNull<T, N> {
    type Error = Null;

    #[inline]
    fn try_from(ptr: NonNull<T>) -> Result<Self, Self::Error> {
        Self::try_from(ptr.as_ptr())
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use crate::Null;

    type TagNonNull = crate::TagNonNull<i32, 2>;

    #[test]
    fn test_dangling() {
        assert_eq!(TagNonNull::dangling().into_raw(), NonNull::dangling());

        #[repr(align(64))]
        struct Alignment64;
        assert_eq!(crate::TagNonNull::<Alignment64, 0>::dangling().into_usize(), 64);
    }

    #[test]
    fn test_try_compose() {
        let reference = &1;
        let ptr = NonNull::from(reference);
        let res = TagNonNull::try_compose(ptr, 0b11).map(|ptr| ptr.decompose());
        assert_eq!(res, Ok((ptr, 0b11)));

        let dangling = NonNull::dangling();
        let res = TagNonNull::try_compose(dangling, 0).map(|ptr| ptr.decompose());
        assert_eq!(res, Ok((dangling, 0)));

        let ptr = NonNull::new(0b11 as *mut i32).unwrap();
        let res = TagNonNull::try_compose(ptr, 0b11);
        assert_eq!(res, Err(Null(0b11)));
    }
}
