//! Aliasable `Box`.

use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;
use core::{fmt, mem};

pub use alloc::boxed::Box as UniqueBox;

/// Basic aliasable (non `core::ptr::Unique`) alternative to
/// [`alloc::boxed::Box`].
pub struct AliasableBox<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> AliasableBox<T> {
    /// Construct an `AliasableBox` from a [`UniqueBox`].
    pub fn from_unique(ptr: UniqueBox<T>) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(UniqueBox::into_raw(ptr)) };
        Self(ptr)
    }

    /// Consumes `self` and converts it into a non-aliasable [`UniqueBox`].
    #[inline]
    pub fn into_unique(mut ptr: AliasableBox<T>) -> UniqueBox<T> {
        // As we are consuming the `Box` structure we can safely assume any
        // aliasing has ended and convert the aliasable `Box` back to into an
        // unaliasable `UniqueBox`.
        let unique = unsafe { ptr.reclaim_as_unique_box() };
        // Forget the aliasable `Box` so the allocation behind the `UniqueBox`
        // is not deallocated.
        mem::forget(ptr);
        // Return the `UniqueBox`.
        unique
    }

    /// Convert a pinned [`AliasableBox`] to a `core::ptr::Unique` backed pinned
    /// [`UniqueBox`].
    pub fn into_unique_pin(pin: Pin<AliasableBox<T>>) -> Pin<UniqueBox<T>> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableBox::into_unique(aliasable))
        }
    }

    /// Convert a pinned `core::ptr::Unique` backed [`UniqueBox`] to a
    /// pinned [`AliasableBox`].
    pub fn from_unique_pin(pin: Pin<UniqueBox<T>>) -> Pin<AliasableBox<T>> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableBox::from(unique))
        }
    }

    #[inline]
    unsafe fn reclaim_as_unique_box(&mut self) -> UniqueBox<T> {
        UniqueBox::from_raw(self.0.as_ptr())
    }
}

impl<T: ?Sized> From<UniqueBox<T>> for AliasableBox<T> {
    fn from(ptr: UniqueBox<T>) -> Self {
        Self::from_unique(ptr)
    }
}

impl<T: ?Sized> Drop for AliasableBox<T> {
    fn drop(&mut self) {
        // SAFETY: As `self` is being dropped we can safely assume any aliasing
        // has ended and convert the `AliasableBox` back to into an unaliasable
        // `UniqueBox` to handle the deallocation.
        let _ = unsafe { self.reclaim_as_unique_box() };
    }
}

impl<T: ?Sized> Deref for AliasableBox<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: We own the data, so we can return a reference to it.
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for AliasableBox<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: We own the data, so we can return a reference to it.
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized> AsRef<T> for AliasableBox<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &*self
    }
}

impl<T: ?Sized> AsMut<T> for AliasableBox<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut *self
    }
}

impl<T: ?Sized> fmt::Debug for AliasableBox<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

unsafe impl<T: ?Sized> Send for AliasableBox<T> where T: Send {}
unsafe impl<T: ?Sized> Sync for AliasableBox<T> where T: Sync {}

#[cfg(feature = "traits")]
unsafe impl<T: ?Sized> crate::StableDeref for AliasableBox<T> {}

#[cfg(feature = "traits")]
unsafe impl<T: ?Sized> crate::AliasableDeref for AliasableBox<T> {}

#[cfg(test)]
mod tests {
    use super::{AliasableBox, UniqueBox};
    use alloc::format;

    #[test]
    fn test_new() {
        let aliasable = AliasableBox::from_unique(UniqueBox::new(10));
        assert_eq!(*aliasable, 10);
        let unique = AliasableBox::into_unique(aliasable);
        assert_eq!(*unique, 10);
    }

    #[test]
    fn test_new_pin() {
        let aliasable = AliasableBox::from_unique_pin(UniqueBox::pin(10));
        assert_eq!(*aliasable, 10);
        let unique = AliasableBox::into_unique_pin(aliasable);
        assert_eq!(*unique, 10);
    }

    #[test]
    fn test_refs() {
        let mut aliasable = AliasableBox::from_unique(UniqueBox::new(10));
        let ptr: *const u8 = &*aliasable;
        let as_mut_ptr: *const u8 = aliasable.as_mut();
        let as_ref_ptr: *const u8 = aliasable.as_ref();
        assert_eq!(ptr, as_mut_ptr);
        assert_eq!(ptr, as_ref_ptr);
    }

    #[test]
    fn test_debug() {
        let aliasable = AliasableBox::from_unique(UniqueBox::new(10));
        assert_eq!(format!("{:?}", aliasable), "10");
    }
}
