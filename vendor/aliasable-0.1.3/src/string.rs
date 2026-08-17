//! Aliasable `String`.

use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::{fmt, str};

use crate::vec::AliasableVec;

pub use alloc::string::String as UniqueString;

/// Basic aliasable (non `core::ptr::Unique`) alternative to
/// [`alloc::string::String`].
pub struct AliasableString(AliasableVec<u8>);

impl AliasableString {
    /// Consumes `self` into an [`AliasableVec`] of UTF-8 bytes.
    pub fn into_bytes(self) -> AliasableVec<u8> {
        self.0
    }

    /// Construct an `AliasableString` from a [`UniqueString`].
    pub fn from_unique(s: UniqueString) -> Self {
        Self(s.into_bytes().into())
    }

    /// Consumes `self` and converts it into a non-aliasable [`UniqueString`].
    #[inline]
    pub fn into_unique(s: AliasableString) -> UniqueString {
        let unique_bytes = s.into_bytes().into();
        unsafe { UniqueString::from_utf8_unchecked(unique_bytes) }
    }

    /// Convert a pinned [`AliasableString`] to a `core::ptr::Unique` backed pinned
    /// [`UniqueString`].
    pub fn into_unique_pin(pin: Pin<AliasableString>) -> Pin<UniqueString> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableString::into_unique(aliasable))
        }
    }

    /// Convert a pinned `core::ptr::Unique` backed [`UniqueString`] to a
    /// pinned [`AliasableString`].
    pub fn from_unique_pin(pin: Pin<UniqueString>) -> Pin<AliasableString> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableString::from(unique))
        }
    }
}

impl From<UniqueString> for AliasableString {
    #[inline]
    fn from(s: UniqueString) -> Self {
        Self::from_unique(s)
    }
}

impl From<AliasableString> for UniqueString {
    #[inline]
    fn from(s: AliasableString) -> Self {
        AliasableString::into_unique(s)
    }
}

impl Deref for AliasableString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        // SAFETY: `AliasableString` will only ever contain UTF-8.
        unsafe { str::from_utf8_unchecked(&*self.0) }
    }
}

impl DerefMut for AliasableString {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        // SAFETY: `AliasableString` will only ever contain UTF-8.
        unsafe { str::from_utf8_unchecked_mut(&mut *self.0) }
    }
}

impl AsRef<str> for AliasableString {
    #[inline]
    fn as_ref(&self) -> &str {
        &*self
    }
}

impl AsMut<str> for AliasableString {
    fn as_mut(&mut self) -> &mut str {
        &mut *self
    }
}

impl fmt::Debug for AliasableString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

#[cfg(feature = "traits")]
unsafe impl crate::StableDeref for AliasableString {}

#[cfg(feature = "traits")]
unsafe impl crate::AliasableDeref for AliasableString {}

#[cfg(test)]
mod tests {
    use super::{AliasableString, AliasableVec, UniqueString};
    use alloc::{format, vec};
    use core::pin::Pin;

    #[test]
    fn test_new() {
        let aliasable = AliasableString::from_unique(UniqueString::from("hello"));
        assert_eq!(&*aliasable, &"hello"[..]);
        let unique = AliasableString::into_unique(aliasable);
        assert_eq!(&*unique, &"hello"[..]);
    }

    #[test]
    fn test_new_pin() {
        let aliasable = AliasableString::from_unique_pin(Pin::new(UniqueString::from("hello")));
        assert_eq!(&*aliasable, &"hello"[..]);
        let unique = AliasableString::into_unique_pin(aliasable);
        assert_eq!(&*unique, &"hello"[..]);
    }

    #[test]
    fn test_refs() {
        let mut aliasable = AliasableString::from_unique(UniqueString::from("hello"));
        let ptr: *const str = &*aliasable;
        let as_mut_ptr: *const str = aliasable.as_mut();
        let as_ref_ptr: *const str = aliasable.as_ref();
        assert_eq!(ptr, as_mut_ptr);
        assert_eq!(ptr, as_ref_ptr);
    }

    #[test]
    fn test_debug() {
        let aliasable = AliasableString::from_unique(UniqueString::from("hello"));
        assert_eq!(format!("{:?}", aliasable), "\"hello\"");
    }

    #[test]
    fn test_into_bytes() {
        let aliasable = AliasableString::from_unique(UniqueString::from("hello"));
        assert_eq!(
            AliasableVec::into_unique(aliasable.into_bytes()),
            vec![b'h', b'e', b'l', b'l', b'o']
        );
    }
}
