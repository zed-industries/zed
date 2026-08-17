use std::marker::PhantomData;
use std::panic::UnwindSafe;
use std::sync::atomic::Ordering::Relaxed;
use std::{ptr, ptr::NonNull};

use crate::ref_counted::RefCounted;
use crate::{Shared, Tag};

/// [`Ptr`] points to an instance.
#[derive(Debug)]
pub struct Ptr<'g, T> {
    instance_ptr: *const RefCounted<T>,
    _phantom: PhantomData<&'g T>,
}

impl<'g, T> Ptr<'g, T> {
    /// Creates a null [`Ptr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Ptr;
    ///
    /// let ptr: Ptr<usize> = Ptr::null();
    /// ```
    #[inline]
    #[must_use]
    pub const fn null() -> Self {
        Self {
            instance_ptr: ptr::null(),
            _phantom: PhantomData,
        }
    }

    /// Returns `true` if the [`Ptr`] is null.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Ptr;
    ///
    /// let ptr: Ptr<usize> = Ptr::null();
    /// assert!(ptr.is_null());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_null(&self) -> bool {
        Tag::unset_tag(self.instance_ptr).is_null()
    }

    /// Tries to create a reference to the underlying instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicShared, Guard};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_shared: AtomicShared<usize> = AtomicShared::new(21);
    /// let guard = Guard::new();
    /// let ptr = atomic_shared.load(Relaxed, &guard);
    /// assert_eq!(*ptr.as_ref().unwrap(), 21);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> Option<&'g T> {
        let ptr = Tag::unset_tag(self.instance_ptr);
        if ptr.is_null() {
            return None;
        }
        unsafe { Some(&*ptr) }
    }

    /// Tries to create a reference to the underlying instance without checking tag bits.
    ///
    /// # Safety
    ///
    /// This [`Ptr`] must not have any tag bits set, otherwise dereferencing the pointer may lead to
    /// undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicShared, Guard};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_shared: AtomicShared<usize> = AtomicShared::new(21);
    /// let guard = Guard::new();
    /// let ptr = atomic_shared.load(Relaxed, &guard);
    /// assert_eq!(unsafe { *ptr.as_ref_unchecked().unwrap() }, 21);
    /// ```
    #[inline]
    #[must_use]
    pub const unsafe fn as_ref_unchecked(&self) -> Option<&'g T> {
        unsafe { RefCounted::inst_ptr(self.instance_ptr).as_ref() }
    }

    /// Provides a raw pointer to the instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Guard, Shared};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let shared: Shared<usize> = Shared::new(29);
    /// let guard = Guard::new();
    /// let ptr = shared.get_guarded_ptr(&guard);
    /// drop(shared);
    ///
    /// assert_eq!(unsafe { *ptr.as_ptr() }, 29);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        RefCounted::inst_ptr(Tag::unset_tag(self.instance_ptr))
    }

    /// Tries to create a pointer to the underlying instance without checking tag bits.
    ///
    /// # Safety
    ///
    /// This [`Ptr`] must not have any tag bits set, otherwise dereferencing the pointer may lead to
    /// undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{AtomicShared, Guard};
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let atomic_shared: AtomicShared<usize> = AtomicShared::new(21);
    /// let guard = Guard::new();
    /// let ptr = atomic_shared.load(Relaxed, &guard);
    /// assert_eq!(unsafe { *ptr.as_ptr_unchecked() }, 21);
    /// ```
    #[inline]
    #[must_use]
    pub const unsafe fn as_ptr_unchecked(&self) -> *const T {
        RefCounted::inst_ptr(self.instance_ptr)
    }

    /// Returns its [`Tag`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Ptr, Tag};
    ///
    /// let ptr: Ptr<usize> = Ptr::null();
    /// assert_eq!(ptr.tag(), Tag::None);
    /// ```
    #[inline]
    #[must_use]
    pub fn tag(&self) -> Tag {
        Tag::into_tag(self.instance_ptr)
    }

    /// Sets a [`Tag`], overwriting its existing [`Tag`].
    ///
    /// Returns the previous tag value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Ptr, Tag};
    ///
    /// let mut ptr: Ptr<usize> = Ptr::null();
    /// assert_eq!(ptr.set_tag(Tag::Both), Tag::None);
    /// assert_eq!(ptr.tag(), Tag::Both);
    /// ```
    #[inline]
    pub fn set_tag(&mut self, tag: Tag) -> Tag {
        let old_tag = Tag::into_tag(self.instance_ptr);
        self.instance_ptr = Tag::update_tag(self.instance_ptr, tag);
        old_tag
    }

    /// Clears its [`Tag`].
    ///
    /// Returns the previous tag value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Ptr, Tag};
    ///
    /// let mut ptr: Ptr<usize> = Ptr::null().with_tag(Tag::Both);
    /// assert_eq!(ptr.unset_tag(), Tag::Both);
    /// ```
    #[inline]
    pub fn unset_tag(&mut self) -> Tag {
        let old_tag = Tag::into_tag(self.instance_ptr);
        self.instance_ptr = Tag::unset_tag(self.instance_ptr);
        old_tag
    }

    /// Returns a copy of `self` with a [`Tag`] set.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Ptr, Tag};
    ///
    /// let mut ptr: Ptr<usize> = Ptr::null();
    /// assert_eq!(ptr.tag(), Tag::None);
    ///
    /// let ptr_with_tag = ptr.with_tag(Tag::First);
    /// assert_eq!(ptr_with_tag.tag(), Tag::First);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_tag(self, tag: Tag) -> Self {
        Self::from(Tag::update_tag(self.instance_ptr, tag))
    }

    /// Returns a copy of `self` with its [`Tag`] erased.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Ptr, Tag};
    ///
    /// let mut ptr: Ptr<usize> = Ptr::null();
    /// ptr.set_tag(Tag::Second);
    /// assert_eq!(ptr.tag(), Tag::Second);
    ///
    /// let ptr_without_tag = ptr.without_tag();
    /// assert_eq!(ptr_without_tag.tag(), Tag::None);
    /// ```
    #[inline]
    #[must_use]
    pub fn without_tag(self) -> Self {
        Self::from(Tag::unset_tag(self.instance_ptr))
    }

    /// Tries to convert itself into a [`Shared`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Guard, Shared};
    ///
    /// let shared: Shared<usize> = Shared::new(83);
    /// let guard = Guard::new();
    /// let ptr = shared.get_guarded_ptr(&guard);
    /// let shared_restored = ptr.get_shared().unwrap();
    /// assert_eq!(*shared_restored, 83);
    ///
    /// drop(shared);
    /// drop(shared_restored);
    ///
    /// assert!(ptr.get_shared().is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_shared(self) -> Option<Shared<T>> {
        unsafe {
            if let Some(ptr) = NonNull::new(Tag::unset_tag(self.instance_ptr).cast_mut()) {
                if (*ptr.as_ptr()).try_add_ref(Relaxed) {
                    return Some(Shared::from(ptr));
                }
            }
        }
        None
    }

    /// Creates a new [`Ptr`] from a raw pointer.
    #[inline]
    pub(super) const fn from(ptr: *const RefCounted<T>) -> Self {
        Self {
            instance_ptr: ptr,
            _phantom: PhantomData,
        }
    }

    /// Provides a raw pointer to its [`RefCounted`].
    #[inline]
    pub(super) const fn as_underlying_ptr(self) -> *const RefCounted<T> {
        self.instance_ptr
    }
}

impl<T> Clone for Ptr<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ptr<'_, T> {}

impl<T> Default for Ptr<'_, T> {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl<T> Eq for Ptr<'_, T> {}

impl<T> PartialEq for Ptr<'_, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.instance_ptr == other.instance_ptr
    }
}

impl<T: UnwindSafe> UnwindSafe for Ptr<'_, T> {}
