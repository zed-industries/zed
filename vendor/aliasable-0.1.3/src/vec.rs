//! Aliasable `Vec`.

use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;
use core::{fmt, mem, slice};

pub use alloc::vec::Vec as UniqueVec;

/// Basic aliasable (non `core::ptr::Unique`) alternative to
/// [`alloc::vec::Vec`].
pub struct AliasableVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> AliasableVec<T> {
    /// Construct an `AliasableVec` from a [`UniqueVec`].
    pub fn from_unique(mut vec: UniqueVec<T>) -> Self {
        let ptr = vec.as_mut_ptr();
        let len = vec.len();
        let cap = vec.capacity();

        mem::forget(vec);

        let ptr = unsafe { NonNull::new_unchecked(ptr) };

        Self { ptr, len, cap }
    }

    /// Consumes the [`AliasableVec`] and converts it back into a
    /// non-aliasable [`UniqueVec`].
    #[inline]
    pub fn into_unique(mut vec: AliasableVec<T>) -> UniqueVec<T> {
        // SAFETY: As we are consuming the `Vec` structure we can safely assume
        // any aliasing has ended and convert the aliasable `Vec` back to into
        // an unaliasable `UniqueVec`.
        let unique = unsafe { vec.reclaim_as_unique_vec() };
        // Forget the aliasable `Vec` so the allocation behind the `UniqueVec`
        // is not deallocated.
        mem::forget(vec);
        // Return the `UniqueVec`.
        unique
    }

    /// Convert a pinned [`AliasableVec`] to a `core::ptr::Unique` backed pinned
    /// [`UniqueVec`].
    pub fn into_unique_pin(pin: Pin<AliasableVec<T>>) -> Pin<UniqueVec<T>> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableVec::into_unique(aliasable))
        }
    }

    /// Convert a pinned `core::ptr::Unique` backed [`UniqueVec`] to a
    /// pinned [`AliasableVec`].
    pub fn from_unique_pin(pin: Pin<UniqueVec<T>>) -> Pin<AliasableVec<T>> {
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableVec::from(unique))
        }
    }

    #[inline]
    unsafe fn reclaim_as_unique_vec(&mut self) -> UniqueVec<T> {
        UniqueVec::from_raw_parts(self.ptr.as_mut(), self.len, self.cap)
    }
}

impl<T> From<UniqueVec<T>> for AliasableVec<T> {
    #[inline]
    fn from(vec: UniqueVec<T>) -> Self {
        Self::from_unique(vec)
    }
}

impl<T> From<AliasableVec<T>> for UniqueVec<T> {
    #[inline]
    fn from(vec: AliasableVec<T>) -> Self {
        AliasableVec::into_unique(vec)
    }
}

impl<T> Drop for AliasableVec<T> {
    fn drop(&mut self) {
        // As the `Vec` structure is being dropped we can safely assume any
        // aliasing has ended and convert the aliasable `Vec` back to into an
        // unaliasable `UniqueVec` to handle the deallocation.
        let _ = unsafe { self.reclaim_as_unique_vec() };
    }
}

impl<T> Deref for AliasableVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        // SAFETY: We own the data, so we can return a reference to it.
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for AliasableVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: We own the data, so we can return a reference to it.
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> AsRef<[T]> for AliasableVec<T> {
    fn as_ref(&self) -> &[T] {
        &*self
    }
}

impl<T> AsMut<[T]> for AliasableVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut *self
    }
}

impl<T> fmt::Debug for AliasableVec<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

unsafe impl<T> Send for AliasableVec<T> where T: Send {}
unsafe impl<T> Sync for AliasableVec<T> where T: Sync {}

#[cfg(feature = "traits")]
unsafe impl<T> crate::StableDeref for AliasableVec<T> {}

#[cfg(feature = "traits")]
unsafe impl<T> crate::AliasableDeref for AliasableVec<T> {}

#[cfg(test)]
mod tests {
    use super::AliasableVec;
    use alloc::{format, vec};
    use core::pin::Pin;

    #[test]
    fn test_new() {
        let aliasable = AliasableVec::from_unique(vec![10]);
        assert_eq!(&*aliasable, &[10]);
        let unique = AliasableVec::into_unique(aliasable);
        assert_eq!(&*unique, &[10]);
    }

    #[test]
    fn test_new_pin() {
        let aliasable = AliasableVec::from_unique_pin(Pin::new(vec![10]));
        assert_eq!(&*aliasable, &[10]);
        let unique = AliasableVec::into_unique_pin(aliasable);
        assert_eq!(&*unique, &[10]);
    }

    #[test]
    fn test_refs() {
        let mut aliasable = AliasableVec::from_unique(vec![10]);
        let ptr: *const [u8] = &*aliasable;
        let as_mut_ptr: *const [u8] = aliasable.as_mut();
        let as_ref_ptr: *const [u8] = aliasable.as_ref();
        assert_eq!(ptr, as_mut_ptr);
        assert_eq!(ptr, as_ref_ptr);
    }

    #[test]
    fn test_debug() {
        let aliasable = AliasableVec::from_unique(vec![10]);
        assert_eq!(format!("{:?}", aliasable), "[10]");
    }
}
