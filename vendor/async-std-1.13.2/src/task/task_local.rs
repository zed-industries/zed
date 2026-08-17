use std::cell::UnsafeCell;
use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::task::TaskLocalsWrapper;

/// The key for accessing a task-local value.
///
/// Every task-local value is lazily initialized on first access and destroyed when the task
/// completes.
#[derive(Debug)]
pub struct LocalKey<T: Send + 'static> {
    #[doc(hidden)]
    pub __init: fn() -> T,

    #[doc(hidden)]
    pub __key: AtomicU32,
}

impl<T: Send + 'static> LocalKey<T> {
    /// Gets a reference to the task-local value with this key.
    ///
    /// The passed closure receives a reference to the task-local value.
    ///
    /// The task-local value will be lazily initialized if this task has not accessed it before.
    ///
    /// # Panics
    ///
    /// This function will panic if not called within the context of a task created by
    /// [`block_on`], [`spawn`], or [`Builder::spawn`].
    ///
    /// [`block_on`]: fn.block_on.html
    /// [`spawn`]: fn.spawn.html
    /// [`Builder::spawn`]: struct.Builder.html#method.spawn
    ///
    /// # Examples
    ///
    /// ```
    /// #
    /// use std::cell::Cell;
    ///
    /// use async_std::task;
    /// use async_std::prelude::*;
    ///
    /// task_local! {
    ///     static NUMBER: Cell<u32> = Cell::new(5);
    /// }
    ///
    /// task::block_on(async {
    ///     let v = NUMBER.with(|c| c.get());
    ///     assert_eq!(v, 5);
    /// });
    /// ```
    pub fn with<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        self.try_with(f)
            .expect("`LocalKey::with` called outside the context of a task")
    }

    /// Attempts to get a reference to the task-local value with this key.
    ///
    /// The passed closure receives a reference to the task-local value.
    ///
    /// The task-local value will be lazily initialized if this task has not accessed it before.
    ///
    /// This function returns an error if not called within the context of a task created by
    /// [`block_on`], [`spawn`], or [`Builder::spawn`].
    ///
    /// [`block_on`]: fn.block_on.html
    /// [`spawn`]: fn.spawn.html
    /// [`Builder::spawn`]: struct.Builder.html#method.spawn
    ///
    /// # Examples
    ///
    /// ```
    /// #
    /// use std::cell::Cell;
    ///
    /// use async_std::task;
    /// use async_std::prelude::*;
    ///
    /// task_local! {
    ///     static VAL: Cell<u32> = Cell::new(5);
    /// }
    ///
    /// task::block_on(async {
    ///     let v = VAL.try_with(|c| c.get());
    ///     assert_eq!(v, Ok(5));
    /// });
    ///
    /// // Returns an error because not called within the context of a task.
    /// assert!(VAL.try_with(|c| c.get()).is_err());
    /// ```
    pub fn try_with<F, R>(&'static self, f: F) -> Result<R, AccessError>
    where
        F: FnOnce(&T) -> R,
    {
        TaskLocalsWrapper::get_current(|task| unsafe {
            // Prepare the numeric key, initialization function, and the map of task-locals.
            let key = self.key();
            let init = || Box::new((self.__init)()) as Box<dyn Send>;

            // Get the value in the map of task-locals, or initialize and insert one.
            let value: *const dyn Send = task.locals().get_or_insert(key, init);

            // Call the closure with the value passed as an argument.
            f(&*(value as *const T))
        })
        .ok_or(AccessError { _private: () })
    }

    /// Returns the numeric key associated with this task-local.
    #[inline]
    fn key(&self) -> u32 {
        #[cold]
        fn init(key: &AtomicU32) -> u32 {
            static COUNTER: AtomicU32 = AtomicU32::new(1);

            let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
            if counter > u32::max_value() / 2 {
                std::process::abort();
            }

            match key.compare_exchange(0, counter, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => counter,
                Err(k) => k,
            }
        }

        match self.__key.load(Ordering::Acquire) {
            0 => init(&self.__key),
            k => k,
        }
    }
}

/// An error returned by [`LocalKey::try_with`].
///
/// [`LocalKey::try_with`]: struct.LocalKey.html#method.try_with
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct AccessError {
    _private: (),
}

impl fmt::Debug for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccessError").finish()
    }
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "already destroyed or called outside the context of a task".fmt(f)
    }
}

impl Error for AccessError {}

/// A key-value entry in a map of task-locals.
struct Entry {
    /// Key identifying the task-local variable.
    key: u32,

    /// Value stored in this entry.
    value: Box<dyn Send>,
}

/// A map that holds task-locals.
pub(crate) struct LocalsMap {
    /// A list of key-value entries sorted by the key.
    entries: UnsafeCell<Option<Vec<Entry>>>,
}

impl LocalsMap {
    /// Creates an empty map of task-locals.
    pub fn new() -> LocalsMap {
        LocalsMap {
            entries: UnsafeCell::new(Some(Vec::new())),
        }
    }

    /// Returns a task-local value associated with `key` or inserts one constructed by `init`.
    #[inline]
    pub fn get_or_insert(&self, key: u32, init: impl FnOnce() -> Box<dyn Send>) -> &dyn Send {
        match unsafe { (*self.entries.get()).as_mut() } {
            None => panic!("can't access task-locals while the task is being dropped"),
            Some(entries) => {
                let index = match entries.binary_search_by_key(&key, |e| e.key) {
                    Ok(i) => i,
                    Err(i) => {
                        let value = init();
                        entries.insert(i, Entry { key, value });
                        i
                    }
                };
                &*entries[index].value
            }
        }
    }

    /// Clears the map and drops all task-locals.
    ///
    /// This method is only safe to call at the end of the task.
    pub unsafe fn clear(&self) {
        // Since destructors may attempt to access task-locals, we musnt't hold a mutable reference
        // to the `Vec` while dropping them. Instead, we first take the `Vec` out and then drop it.
        let entries = (*self.entries.get()).take();
        drop(entries);
    }
}
