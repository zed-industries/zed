//! A niched archived `Option<Box<T>>` that uses less space.

use crate::{
    boxed::{ArchivedBox, BoxResolver},
    ser::Serializer,
    ArchivePointee, ArchiveUnsized, SerializeUnsized,
};
use core::{
    cmp::{self, Eq, Ord, PartialEq, PartialOrd},
    fmt, hash,
    hint::unreachable_unchecked,
    ops::Deref,
    pin::Pin,
};

/// A niched archived `Option<Box<T>>`.
///
/// It uses less space by storing the `None` variant as a null pointer.
#[repr(transparent)]
pub struct ArchivedOptionBox<T: ArchivePointee + ?Sized> {
    inner: ArchivedBox<T>,
}

impl<T: ArchivePointee + ?Sized> ArchivedOptionBox<T> {
    /// Returns `true` if the option box is a `None` value.
    #[inline]
    pub fn is_none(&self) -> bool {
        self.as_ref().is_none()
    }

    /// Returns `true` if the option box is a `Some` value.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.as_ref().is_some()
    }

    /// Converts to an `Option<&ArchivedBox<T>>`.
    #[inline]
    pub fn as_ref(&self) -> Option<&ArchivedBox<T>> {
        if self.inner.is_null() {
            None
        } else {
            Some(&self.inner)
        }
    }

    /// Converts to an `Option<&mut ArchivedBox<T>>`.
    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut ArchivedBox<T>> {
        if self.inner.is_null() {
            None
        } else {
            Some(&mut self.inner)
        }
    }

    /// Converts from `Pin<&ArchivedOptionBox<T>>` to `Option<Pin<&ArchivedBox<T>>>`.
    #[inline]
    pub fn as_pin_ref(self: Pin<&Self>) -> Option<Pin<&ArchivedBox<T>>> {
        unsafe { Pin::get_ref(self).as_ref().map(|x| Pin::new_unchecked(x)) }
    }

    /// Converts from `Pin<&mut ArchivedOption<T>>` to `Option<Pin<&mut ArchivedBox<T>>>`.
    #[inline]
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut ArchivedBox<T>>> {
        unsafe {
            Pin::get_unchecked_mut(self)
                .as_mut()
                .map(|x| Pin::new_unchecked(x))
        }
    }

    /// Returns an iterator over the possibly contained value.
    #[inline]
    pub fn iter(&self) -> Iter<'_, ArchivedBox<T>> {
        Iter {
            inner: self.as_ref(),
        }
    }

    /// Returns a mutable iterator over the possibly contained value.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, ArchivedBox<T>> {
        IterMut {
            inner: self.as_mut(),
        }
    }

    /// Converts from `&ArchivedOptionBox<T>` to `Option<&T>`.
    ///
    /// Leaves the original `ArchivedOptionBox` in-place, creating a new one with a reference to the
    /// original one.
    #[inline]
    pub fn as_deref(&self) -> Option<&T> {
        self.as_ref().map(|x| (*x).deref())
    }
}

impl<T: ArchivePointee + ?Sized> ArchivedOptionBox<T>
where
    T::ArchivedMetadata: Default,
{
    /// Resolves an `ArchivedOptionBox<T::Archived>` from an `Option<&T>`.
    ///
    /// # Safety
    ///
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing `field`
    #[inline]
    pub unsafe fn resolve_from_option<U: ArchiveUnsized<Archived = T> + ?Sized>(
        field: Option<&U>,
        pos: usize,
        resolver: OptionBoxResolver<U::MetadataResolver>,
        out: *mut Self,
    ) {
        let (fp, fo) = out_field!(out.inner);
        if let Some(value) = field {
            let resolver = if let OptionBoxResolver::Some(metadata_resolver) = resolver {
                metadata_resolver
            } else {
                unreachable_unchecked();
            };

            ArchivedBox::resolve_from_ref(value, pos + fp, resolver, fo)
        } else {
            ArchivedBox::emplace_null(pos + fp, fo);
        }
    }

    /// Serializes an `ArchivedOptionBox<T::Archived>` from an `Option<&T>`.
    #[inline]
    pub fn serialize_from_option<U, S>(
        field: Option<&U>,
        serializer: &mut S,
    ) -> Result<OptionBoxResolver<U::MetadataResolver>, S::Error>
    where
        U: SerializeUnsized<S, Archived = T> + ?Sized,
        S: Serializer + ?Sized,
    {
        if let Some(value) = field {
            Ok(OptionBoxResolver::Some(ArchivedBox::serialize_from_ref(
                value, serializer,
            )?))
        } else {
            Ok(OptionBoxResolver::None)
        }
    }
}

impl<T: ArchivePointee + ?Sized> fmt::Debug for ArchivedOptionBox<T>
where
    T::ArchivedMetadata: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_ref() {
            Some(inner) => inner.fmt(f),
            None => f.debug_tuple("None").finish(),
        }
    }
}

impl<T: ArchivePointee + Eq + ?Sized> Eq for ArchivedOptionBox<T> {}

impl<T: ArchivePointee + hash::Hash + ?Sized> hash::Hash for ArchivedOptionBox<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<T: ArchivePointee + Ord + ?Sized> Ord for ArchivedOptionBox<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl<T: ArchivePointee + PartialEq + ?Sized> PartialEq for ArchivedOptionBox<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(&other.as_ref())
    }
}

impl<T: ArchivePointee + PartialOrd + ?Sized> PartialOrd for ArchivedOptionBox<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.as_ref().partial_cmp(&other.as_ref())
    }
}

/// An iterator over a reference to the `Some` variant of an `ArchivedOptionBox`.
///
/// This iterator yields one value if the `ArchivedOptionBox` is a `Some`, otherwise none.
///
/// This `struct` is created by the [`ArchivedOptionBox::iter`] function.
pub type Iter<'a, T> = crate::option::Iter<'a, T>;

/// An iterator over a mutable reference to the `Some` variant of an `ArchivedOptionBox`.
///
/// This iterator yields one value if the `ArchivedOptionBox` is a `Some`, otherwise none.
///
/// This `struct` is created by the [`ArchivedOptionBox::iter_mut`] function.
pub type IterMut<'a, T> = crate::option::IterMut<'a, T>;

/// The resolver for [`ArchivedOptionBox`].
pub enum OptionBoxResolver<T> {
    /// The `ArchivedOptionBox` was `None`
    None,
    /// The resolver for the `ArchivedBox`
    Some(BoxResolver<T>),
}
