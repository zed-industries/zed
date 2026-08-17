use std::mem::forget;
use std::panic::UnwindSafe;
use std::ptr::{NonNull, null, null_mut};
#[cfg(not(feature = "loom"))]
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{self, Relaxed};

#[cfg(feature = "loom")]
use loom::sync::atomic::AtomicPtr;

use crate::ref_counted::RefCounted;
use crate::{Guard, Owned, Ptr, Tag};

/// [`AtomicOwned`] owns the underlying instance, and allows users to perform atomic operations
/// on the pointer to it.
#[derive(Debug)]
pub struct AtomicOwned<T> {
    instance_ptr: AtomicPtr<RefCounted<T>>,
}

/// A pair of [`Owned`] and [`Ptr`] of the same type.
pub type OwnedPtrPair<'g, T> = (Option<Owned<T>>, Ptr<'g, T>);

impl<T: 'static> AtomicOwned<T> {
    /// Creates a new [`AtomicOwned`] from an instance of `T`.
    ///
    /// The type of the instance must be determined at compile-time, must not contain non-static
    /// references, and must not be a non-static reference since the instance can theoretically
    /// live as long as the process. For instance, `struct Disallowed<'l, T>(&'l T)` is not
    /// allowed, because an instance of the type cannot outlive `'l` whereas the garbage collector
    /// does not guarantee that the instance is dropped within `'l`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::AtomicOwned;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::new(10);
    /// ```
    #[inline]
    pub fn new(t: T) -> Self {
        Self {
            instance_ptr: AtomicPtr::new(RefCounted::new_unique(|| t).as_ptr()),
        }
    }
}

impl<T> AtomicOwned<T> {
    /// Creates a new [`AtomicOwned`] from an [`Owned`] of `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Owned};
    ///
    /// let owned: Owned<usize> = Owned::new(10);
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::from(owned);
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    #[must_use]
    pub const fn from(owned: Owned<T>) -> Self {
        let ptr = owned.underlying_ptr();
        forget(owned);
        let instance_ptr: std::sync::atomic::AtomicPtr<RefCounted<T>> =
            AtomicPtr::new(ptr.cast_mut());
        Self { instance_ptr }
    }

    /// Creates a new [`AtomicOwned`] from an [`Owned`] of `T`.
    #[cfg(feature = "loom")]
    #[inline]
    #[must_use]
    pub fn from(owned: Owned<T>) -> Self {
        let ptr = owned.underlying_ptr();
        forget(owned);
        let instance_ptr: loom::sync::atomic::AtomicPtr<RefCounted<T>> =
            AtomicPtr::new(ptr.cast_mut());
        Self { instance_ptr }
    }

    /// Creates a null [`AtomicOwned`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::AtomicOwned;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::null();
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    #[must_use]
    pub const fn null() -> Self {
        let instance_ptr: std::sync::atomic::AtomicPtr<RefCounted<T>> = AtomicPtr::new(null_mut());
        Self { instance_ptr }
    }

    /// Creates a null [`AtomicOwned`].
    #[cfg(feature = "loom")]
    #[inline]
    #[must_use]
    pub fn null() -> Self {
        let instance_ptr: loom::sync::atomic::AtomicPtr<RefCounted<T>> = AtomicPtr::new(null_mut());
        Self { instance_ptr }
    }

    /// Returns `true` if the [`AtomicOwned`] is null.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Tag};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::null();
    /// atomic_owned.update_tag_if(Tag::Both, |p| p.tag() == Tag::None, Relaxed, Relaxed);
    /// assert!(atomic_owned.is_null(Relaxed));
    /// ```
    #[inline]
    #[must_use]
    pub fn is_null(&self, order: Ordering) -> bool {
        Tag::unset_tag(self.instance_ptr.load(order)).is_null()
    }

    /// Loads a pointer value from the [`AtomicOwned`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Guard};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::new(11);
    /// let guard = Guard::new();
    /// let ptr = atomic_owned.load(Relaxed, &guard);
    /// assert_eq!(*ptr.as_ref().unwrap(), 11);
    /// ```
    #[inline]
    #[must_use]
    pub fn load<'g>(&self, order: Ordering, _guard: &'g Guard) -> Ptr<'g, T> {
        Ptr::from(self.instance_ptr.load(order))
    }

    /// Stores the given value into the [`AtomicOwned`] and returns the original value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Guard, Owned, Tag};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::new(14);
    /// let guard = Guard::new();
    /// let (old, tag) = atomic_owned.swap((Some(Owned::new(15)), Tag::Second), Relaxed);
    /// assert_eq!(tag, Tag::None);
    /// assert_eq!(*old.unwrap(), 14);
    /// let (old, tag) = atomic_owned.swap((None, Tag::First), Relaxed);
    /// assert_eq!(tag, Tag::Second);
    /// assert_eq!(*old.unwrap(), 15);
    /// let (old, tag) = atomic_owned.swap((None, Tag::None), Relaxed);
    /// assert_eq!(tag, Tag::First);
    /// assert!(old.is_none());
    /// ```
    #[inline]
    pub fn swap(&self, new: (Option<Owned<T>>, Tag), order: Ordering) -> (Option<Owned<T>>, Tag) {
        let desired = Tag::update_tag(
            new.0.as_ref().map_or_else(null, Owned::underlying_ptr),
            new.1,
        )
        .cast_mut();
        let prev = self.instance_ptr.swap(desired, order);
        let tag = Tag::into_tag(prev);
        let prev_ptr = Tag::unset_tag(prev).cast_mut();
        forget(new);
        (NonNull::new(prev_ptr).map(Owned::from), tag)
    }

    /// Returns its [`Tag`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Tag};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::null();
    /// assert_eq!(atomic_owned.tag(Relaxed), Tag::None);
    /// ```
    #[inline]
    #[must_use]
    pub fn tag(&self, order: Ordering) -> Tag {
        Tag::into_tag(self.instance_ptr.load(order))
    }

    /// Sets a new [`Tag`] if the given condition is met.
    ///
    /// Returns `true` if the new [`Tag`] has been successfully set.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Tag};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::null();
    /// assert!(atomic_owned.update_tag_if(Tag::Both, |p| p.tag() == Tag::None, Relaxed, Relaxed));
    /// assert_eq!(atomic_owned.tag(Relaxed), Tag::Both);
    /// ```
    #[inline]
    pub fn update_tag_if<F: FnMut(Ptr<T>) -> bool>(
        &self,
        tag: Tag,
        mut condition: F,
        set_order: Ordering,
        fetch_order: Ordering,
    ) -> bool {
        self.instance_ptr
            .fetch_update(set_order, fetch_order, |ptr| {
                if condition(Ptr::from(ptr)) {
                    Some(Tag::update_tag(ptr, tag).cast_mut())
                } else {
                    None
                }
            })
            .is_ok()
    }

    /// Stores `new` into the [`AtomicOwned`] if the current value is the same as `current`.
    ///
    /// Returns the previously held value and the updated [`Ptr`].
    ///
    /// # Errors
    ///
    /// Returns `Err` with the supplied [`Owned`] and the current [`Ptr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Guard, Owned, Tag};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::new(17);
    /// let guard = Guard::new();
    ///
    /// let mut ptr = atomic_owned.load(Relaxed, &guard);
    /// assert_eq!(*ptr.as_ref().unwrap(), 17);
    ///
    /// atomic_owned.update_tag_if(Tag::Both, |_| true, Relaxed, Relaxed);
    /// assert!(atomic_owned.compare_exchange(
    ///     ptr, (Some(Owned::new(18)), Tag::First), Relaxed, Relaxed, &guard).is_err());
    ///
    /// ptr.set_tag(Tag::Both);
    /// let old: Owned<usize> = atomic_owned.compare_exchange(
    ///     ptr, (Some(Owned::new(18)), Tag::First), Relaxed, Relaxed, &guard).unwrap().0.unwrap();
    /// assert_eq!(*old, 17);
    /// drop(old);
    ///
    /// assert!(atomic_owned.compare_exchange(
    ///     ptr, (Some(Owned::new(19)), Tag::None), Relaxed, Relaxed, &guard).is_err());
    /// assert_eq!(*ptr.as_ref().unwrap(), 17);
    /// ```
    #[inline]
    pub fn compare_exchange<'g>(
        &self,
        current: Ptr<'g, T>,
        new: (Option<Owned<T>>, Tag),
        success: Ordering,
        failure: Ordering,
        _guard: &'g Guard,
    ) -> Result<OwnedPtrPair<'g, T>, OwnedPtrPair<'g, T>> {
        let desired = Tag::update_tag(
            new.0.as_ref().map_or_else(null, Owned::underlying_ptr),
            new.1,
        )
        .cast_mut();
        match self.instance_ptr.compare_exchange(
            current.as_underlying_ptr().cast_mut(),
            desired,
            success,
            failure,
        ) {
            Ok(prev) => {
                let prev_owned = NonNull::new(Tag::unset_tag(prev).cast_mut()).map(Owned::from);
                forget(new);
                Ok((prev_owned, Ptr::from(desired)))
            }
            Err(actual) => Err((new.0, Ptr::from(actual))),
        }
    }

    /// Stores `new` into the [`AtomicOwned`] if the current value is the same as `current`.
    ///
    /// This method is allowed to spuriously fail even when the comparison succeeds.
    ///
    /// Returns the previously held value and the updated [`Ptr`].
    ///
    /// # Errors
    ///
    /// Returns `Err` with the supplied [`Owned`] and the current [`Ptr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Owned, Guard, Tag};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::new(17);
    /// let guard = Guard::new();
    ///
    /// let mut ptr = atomic_owned.load(Relaxed, &guard);
    /// assert_eq!(*ptr.as_ref().unwrap(), 17);
    ///
    /// while let Err((_, actual)) = atomic_owned.compare_exchange_weak(
    ///     ptr,
    ///     (Some(Owned::new(18)), Tag::First),
    ///     Relaxed,
    ///     Relaxed,
    ///     &guard) {
    ///     ptr = actual;
    /// }
    ///
    /// let mut ptr = atomic_owned.load(Relaxed, &guard);
    /// assert_eq!(*ptr.as_ref().unwrap(), 18);
    /// ```
    #[inline]
    pub fn compare_exchange_weak<'g>(
        &self,
        current: Ptr<'g, T>,
        new: (Option<Owned<T>>, Tag),
        success: Ordering,
        failure: Ordering,
        _guard: &'g Guard,
    ) -> Result<OwnedPtrPair<'g, T>, OwnedPtrPair<'g, T>> {
        let desired = Tag::update_tag(
            new.0.as_ref().map_or_else(null, Owned::underlying_ptr),
            new.1,
        )
        .cast_mut();
        match self.instance_ptr.compare_exchange_weak(
            current.as_underlying_ptr().cast_mut(),
            desired,
            success,
            failure,
        ) {
            Ok(prev) => {
                let prev_owned = NonNull::new(Tag::unset_tag(prev).cast_mut()).map(Owned::from);
                forget(new);
                Ok((prev_owned, Ptr::from(desired)))
            }
            Err(actual) => Err((new.0, Ptr::from(actual))),
        }
    }

    /// Converts `self` into an [`Owned`].
    ///
    /// Returns `None` if `self` did not own an instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicOwned, Owned};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_owned: AtomicOwned<usize> = AtomicOwned::new(55);
    /// let owned: Owned<usize> = atomic_owned.into_owned(Relaxed).unwrap();
    /// assert_eq!(*owned, 55);
    /// ```
    #[inline]
    #[must_use]
    pub fn into_owned(self, order: Ordering) -> Option<Owned<T>> {
        let ptr = self.instance_ptr.swap(null_mut(), order);
        if let Some(underlying_ptr) = NonNull::new(Tag::unset_tag(ptr).cast_mut()) {
            return Some(Owned::from(underlying_ptr));
        }
        None
    }
}

impl<T> Default for AtomicOwned<T> {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl<T> Drop for AtomicOwned<T> {
    #[inline]
    fn drop(&mut self) {
        if let Some(ptr) = NonNull::new(Tag::unset_tag(self.instance_ptr.load(Relaxed)).cast_mut())
        {
            drop(Owned::from(ptr));
        }
    }
}

unsafe impl<T: Send> Send for AtomicOwned<T> {}

unsafe impl<T: Send + Sync> Sync for AtomicOwned<T> {}

impl<T: UnwindSafe> UnwindSafe for AtomicOwned<T> {}
