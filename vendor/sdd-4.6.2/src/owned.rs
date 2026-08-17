use std::mem::forget;
use std::ops::Deref;
use std::panic::UnwindSafe;
use std::ptr::NonNull;

use super::ref_counted::RefCounted;
use super::{Guard, Ptr};

/// [`Owned`] uniquely owns an instance.
///
/// The instance is passed to the `EBR` garbage collector when the [`Owned`] is dropped.
#[derive(Debug)]
pub struct Owned<T> {
    instance_ptr: NonNull<RefCounted<T>>,
}

impl<T: 'static> Owned<T> {
    /// Creates a new instance of [`Owned`].
    ///
    /// The type of the instance must be determined at compile-time, must not contain non-static
    /// references, and must not be a non-static reference since the instance can theoretically
    /// survive the process. For instance, `struct Disallowed<'l, T>(&'l T)` is not allowed,
    /// because an instance of the type cannot outlive `'l` whereas the garbage collector does not
    /// guarantee that the instance is dropped within `'l`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Owned;
    ///
    /// let owned: Owned<usize> = Owned::new(31);
    /// ```
    #[inline]
    pub fn new(t: T) -> Self {
        Self::new_with(|| t)
    }

    /// Creates a new [`Owned`] with the provided function.
    ///
    /// This function is identical to [`Owned::new`] except that the value is constructed after
    /// memory allocation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Owned;
    ///
    /// let owned: Owned<String> = Owned::new_with(|| String::from("hello"));
    /// ```
    #[inline]
    pub fn new_with<F: FnOnce() -> T>(f: F) -> Owned<T> {
        Owned {
            instance_ptr: RefCounted::new_unique(f),
        }
    }
}

impl<T> Owned<T> {
    /// Creates a new [`Owned`] without checking the lifetime of `T`.
    ///
    /// # Safety
    ///
    /// `T::drop` can be run after the [`Owned`] is dropped, therefore it is safe only if `T::drop`
    /// does not access short-lived data or [`std::mem::needs_drop`] is `false` for `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Owned;
    ///
    /// let hello = String::from("hello");
    /// let owned: Owned<&str> = unsafe { Owned::new_unchecked(hello.as_str()) };
    /// ```
    #[inline]
    pub unsafe fn new_unchecked(t: T) -> Self {
        unsafe { Self::new_with_unchecked(|| t) }
    }

    /// Creates a new [`Owned`] with the provided function without checking the lifetime of `T`.
    ///
    /// This function is identical to [`Owned::new_unchecked`] except that the value is constructed
    /// after memory allocation.
    ///
    /// # Safety
    ///
    /// See [`Owned::new_unchecked`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Owned;
    ///
    /// let hello = String::from("hello");
    /// let owned: Owned<&str> = unsafe { Owned::new_with_unchecked(|| hello.as_str()) };
    /// ```
    #[inline]
    pub unsafe fn new_with_unchecked<F: FnOnce() -> T>(f: F) -> Owned<T> {
        Owned {
            instance_ptr: RefCounted::new_unique(f),
        }
    }

    /// Returns a [`Ptr`] to the instance that may live as long as the supplied [`Guard`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Guard, Owned};
    ///
    /// let owned: Owned<usize> = Owned::new(37);
    /// let guard = Guard::new();
    /// let ptr = owned.get_guarded_ptr(&guard);
    /// drop(owned);
    ///
    /// assert_eq!(*ptr.as_ref().unwrap(), 37);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_guarded_ptr<'g>(&self, _guard: &'g Guard) -> Ptr<'g, T> {
        Ptr::from(self.instance_ptr.as_ptr())
    }

    /// Returns a reference to the instance that may live as long as the supplied [`Guard`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::{Guard, Owned};
    ///
    /// let owned: Owned<usize> = Owned::new(37);
    /// let guard = Guard::new();
    /// let ref_b = owned.get_guarded_ref(&guard);
    /// drop(owned);
    ///
    /// assert_eq!(*ref_b, 37);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_guarded_ref<'g>(&self, _guard: &'g Guard) -> &'g T {
        unsafe { RefCounted::inst_non_null_ptr(self.instance_ptr).as_ref() }
    }

    /// Returns a mutable reference to the instance.
    ///
    /// # Safety
    ///
    /// The method is `unsafe` since there can be a [`Ptr`] to the instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Owned;
    ///
    /// let mut owned: Owned<usize> = Owned::new(38);
    /// unsafe {
    ///     *owned.get_mut() += 1;
    /// }
    /// assert_eq!(*owned, 39);
    /// ```
    #[inline]
    pub const unsafe fn get_mut(&mut self) -> &mut T {
        unsafe { (*self.instance_ptr.as_ptr()).get_mut_unique() }
    }

    /// Provides a raw pointer to the instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Owned;
    ///
    /// let owned: Owned<usize> = Owned::new(10);
    ///
    /// assert_eq!(unsafe { *owned.as_ptr() }, 10);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const T {
        RefCounted::inst_non_null_ptr(self.instance_ptr).as_ptr()
    }

    /// Provides a raw non-null pointer to the instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Owned;
    ///
    /// let owned: Owned<usize> = Owned::new(10);
    ///
    /// assert_eq!(unsafe { *owned.as_non_null_ptr().as_ref() }, 10);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_non_null_ptr(&self) -> NonNull<T> {
        RefCounted::inst_non_null_ptr(self.instance_ptr)
    }

    /// Drops the instance immediately.
    ///
    /// # Safety
    ///
    /// The caller must ensure that there is no [`Ptr`] pointing to the instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Owned;
    /// use std::sync::atomic::AtomicBool;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// static DROPPED: AtomicBool = AtomicBool::new(false);
    /// struct T(&'static AtomicBool);
    /// impl Drop for T {
    ///     fn drop(&mut self) {
    ///         self.0.store(true, Relaxed);
    ///     }
    /// }
    ///
    /// let owned: Owned<T> = Owned::new(T(&DROPPED));
    /// assert!(!DROPPED.load(Relaxed));
    ///
    /// unsafe {
    ///     owned.drop_in_place();
    /// }
    ///
    /// assert!(DROPPED.load(Relaxed));
    /// ```
    #[inline]
    pub unsafe fn drop_in_place(self) {
        unsafe {
            drop(Box::from_raw(self.instance_ptr.as_ptr()));
        }
        forget(self);
    }

    /// Creates a new [`Owned`] from the given pointer.
    #[inline]
    pub(super) const fn from(ptr: NonNull<RefCounted<T>>) -> Self {
        Self { instance_ptr: ptr }
    }

    /// Returns a pointer to the [`RefCounted`].
    #[inline]
    pub(super) const fn underlying_ptr(&self) -> *const RefCounted<T> {
        self.instance_ptr.as_ptr()
    }
}

impl<T> AsRef<T> for Owned<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        unsafe { &*self.instance_ptr.as_ptr() }
    }
}

impl<T> Deref for Owned<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> Drop for Owned<T> {
    #[inline]
    fn drop(&mut self) {
        RefCounted::pass_to_collector(self.instance_ptr.as_ptr());
    }
}

unsafe impl<T: Send> Send for Owned<T> {}

// `T` does not need to be `Send` since sending `T` is not possible only with `&Owned<T>`.
unsafe impl<T: Sync> Sync for Owned<T> {}

impl<T: UnwindSafe> UnwindSafe for Owned<T> {}
