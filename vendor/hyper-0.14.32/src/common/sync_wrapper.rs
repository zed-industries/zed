/*
 * This is a copy of the sync_wrapper crate.
 */

/// A mutual exclusion primitive that relies on static type information only
///
/// In some cases synchronization can be proven statically: whenever you hold an exclusive `&mut`
/// reference, the Rust type system ensures that no other part of the program can hold another
/// reference to the data. Therefore it is safe to access it even if the current thread obtained
/// this reference via a channel. Whenever this is the case, the overhead of allocating and locking
/// a [`Mutex`] can be avoided by using this static version.
///
/// One example where this is often applicable is [`Future`], which requires an exclusive reference
/// for its [`poll`] method: While a given `Future` implementation may not be safe to access by
/// multiple threads concurrently, the executor can only run the `Future` on one thread at any
/// given time, making it [`Sync`] in practice as long as the implementation is `Send`. You can
/// therefore use the sync wrapper to prove that your data structure is `Sync` even though it
/// contains such a `Future`.
///
/// # Example
///
/// ```ignore
/// use hyper::common::sync_wrapper::SyncWrapper;
/// use std::future::Future;
///
/// struct MyThing {
///     future: SyncWrapper<Box<dyn Future<Output = String> + Send>>,
/// }
///
/// impl MyThing {
///     // all accesses to `self.future` now require an exclusive reference or ownership
/// }
///
/// fn assert_sync<T: Sync>() {}
///
/// assert_sync::<MyThing>();
/// ```
///
/// [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
/// [`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
/// [`poll`]: https://doc.rust-lang.org/std/future/trait.Future.html#method.poll
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
#[repr(transparent)]
pub(crate) struct SyncWrapper<T>(T);

impl<T> SyncWrapper<T> {
    /// Creates a new SyncWrapper containing the given value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use hyper::common::sync_wrapper::SyncWrapper;
    ///
    /// let wrapped = SyncWrapper::new(42);
    /// ```
    pub(crate) fn new(value: T) -> Self {
        Self(value)
    }

    /// Acquires a reference to the protected value.
    ///
    /// This is safe because it requires an exclusive reference to the wrapper. Therefore this method
    /// neither panics nor does it return an error. This is in contrast to [`Mutex::get_mut`] which
    /// returns an error if another thread panicked while holding the lock. It is not recommended
    /// to send an exclusive reference to a potentially damaged value to another thread for further
    /// processing.
    ///
    /// [`Mutex::get_mut`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.get_mut
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use hyper::common::sync_wrapper::SyncWrapper;
    ///
    /// let mut wrapped = SyncWrapper::new(42);
    /// let value = wrapped.get_mut();
    /// *value = 0;
    /// assert_eq!(*wrapped.get_mut(), 0);
    /// ```
    pub(crate) fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Consumes this wrapper, returning the underlying data.
    ///
    /// This is safe because it requires ownership of the wrapper, aherefore this method will neither
    /// panic nor does it return an error. This is in contrast to [`Mutex::into_inner`] which
    /// returns an error if another thread panicked while holding the lock. It is not recommended
    /// to send an exclusive reference to a potentially damaged value to another thread for further
    /// processing.
    ///
    /// [`Mutex::into_inner`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.into_inner
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use hyper::common::sync_wrapper::SyncWrapper;
    ///
    /// let mut wrapped = SyncWrapper::new(42);
    /// assert_eq!(wrapped.into_inner(), 42);
    /// ```
    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> T {
        self.0
    }
}

// this is safe because the only operations permitted on this data structure require exclusive
// access or ownership
unsafe impl<T: Send> Sync for SyncWrapper<T> {}
