//! Aliasable `&mut`.

use core::fmt;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;

/// Basic aliasable alternative to `&mut`.
///
/// Note that this does not circumvent the core aliasing rules of Rust; if you use this to create
/// multiple mutable references to a memory location at the same time, that is still UB. This type
/// just adds a few abilities:
///
/// - You may hold any number of `AliasableMut`s and no references to a location.
/// - You may hold any number of `AliasableMut`s and any number of shared references to a location
/// at once.
/// - You may hold any number of `AliasableMut`s and one mutable reference to a location at once.
pub struct AliasableMut<'a, T: ?Sized> {
    inner: NonNull<T>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a, T: ?Sized> AliasableMut<'a, T> {
    /// Construct an `AliasableMut` from an `&mut`.
    #[inline]
    pub fn from_unique(ptr: &'a mut T) -> Self {
        Self {
            inner: NonNull::from(ptr),
            _lifetime: PhantomData,
        }
    }

    /// Consumes `self` and converts it into a non-aliasable `&mut`.
    #[inline]
    pub fn into_unique(ptr: Self) -> &'a mut T {
        unsafe { &mut *ptr.inner.as_ptr() }
    }

    /// Convert a pinned `AliasableMut` to a pinned `&mut`.
    pub fn into_unique_pin(pin: Pin<Self>) -> Pin<&'a mut T> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(Self::into_unique(aliasable))
        }
    }

    /// Convert a pinned `&mut` to a pinned `AliasableMut`.
    pub fn from_unique_pin(pin: Pin<&'a mut T>) -> Pin<Self> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(Self::from_unique(unique))
        }
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for AliasableMut<'a, T> {
    fn from(ptr: &'a mut T) -> Self {
        Self::from_unique(ptr)
    }
}

impl<T: ?Sized> Deref for AliasableMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: It is the callers responsibility to make sure that there are no `&mut`
        // references at this point.
        unsafe { self.inner.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for AliasableMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: It is the callers responsibility to make sure that there are no `&mut`
        // references at this point.
        unsafe { self.inner.as_mut() }
    }
}

impl<T: ?Sized> AsRef<T> for AliasableMut<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsMut<T> for AliasableMut<'_, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized> fmt::Debug for AliasableMut<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

unsafe impl<T: ?Sized> Send for AliasableMut<'_, T> where T: Send {}
unsafe impl<T: ?Sized> Sync for AliasableMut<'_, T> where T: Sync {}

#[cfg(feature = "traits")]
unsafe impl<T: ?Sized> crate::StableDeref for AliasableMut<'_, T> {}

#[cfg(feature = "traits")]
unsafe impl<T: ?Sized> crate::AliasableDeref for AliasableMut<'_, T> {}

#[cfg(test)]
mod tests {
    use super::AliasableMut;
    use alloc::boxed::Box;
    use alloc::format;
    use core::pin::Pin;

    #[test]
    fn test_new() {
        let mut data = Box::new(10);
        let aliasable = AliasableMut::from_unique(&mut data);
        assert_eq!(**aliasable, 10);
        let unique = AliasableMut::into_unique(aliasable);
        assert_eq!(**unique, 10);
    }

    #[test]
    fn test_new_pin() {
        let mut data = Box::new(10);
        let data = unsafe { Pin::new_unchecked(&mut data) };
        let aliasable = AliasableMut::from_unique_pin(data);
        assert_eq!(**aliasable, 10);
        let unique = AliasableMut::into_unique_pin(aliasable);
        assert_eq!(**unique, 10);
    }

    #[test]
    fn test_refs() {
        let mut data = Box::new(10);
        let mut aliasable = AliasableMut::from_unique(&mut data);
        let ptr: *const Box<u8> = &mut *aliasable;
        let as_mut_ptr: *const Box<u8> = aliasable.as_mut();
        let as_ref_ptr: *const Box<u8> = aliasable.as_ref();
        assert_eq!(ptr, as_mut_ptr);
        assert_eq!(ptr, as_ref_ptr);
    }

    #[test]
    fn test_debug() {
        let mut data = 10;
        let aliasable = AliasableMut::from_unique(&mut data);
        assert_eq!(format!("{:?}", aliasable), "10");
    }
}
