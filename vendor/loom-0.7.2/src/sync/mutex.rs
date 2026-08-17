use crate::rt;

use std::ops;
use std::sync::{LockResult, TryLockError, TryLockResult};

/// Mock implementation of `std::sync::Mutex`.
#[derive(Debug)]
pub struct Mutex<T: ?Sized> {
    object: rt::Mutex,
    data: std::sync::Mutex<T>,
}

/// Mock implementation of `std::sync::MutexGuard`.
#[derive(Debug)]
pub struct MutexGuard<'a, T: ?Sized> {
    lock: &'a Mutex<T>,
    data: Option<std::sync::MutexGuard<'a, T>>,
}

impl<T> Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    pub fn new(data: T) -> Mutex<T> {
        Mutex {
            data: std::sync::Mutex::new(data),
            object: rt::Mutex::new(true),
        }
    }

    /// Consumes this mutex, returning the underlying data.
    pub fn into_inner(self) -> LockResult<T> {
        Ok(self.data.into_inner().unwrap())
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Acquires a mutex, blocking the current thread until it is able to do so.
    #[track_caller]
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        self.object.acquire_lock(location!());

        Ok(MutexGuard {
            lock: self,
            data: Some(self.data.lock().unwrap()),
        })
    }

    /// Attempts to acquire this lock.
    ///
    /// If the lock could not be acquired at this time, then `Err` is returned.
    /// Otherwise, an RAII guard is returned. The lock will be unlocked when the
    /// guard is dropped.
    ///
    /// This function does not block.
    #[track_caller]
    pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>> {
        if self.object.try_acquire_lock(location!()) {
            Ok(MutexGuard {
                lock: self,
                data: Some(self.data.lock().unwrap()),
            })
        } else {
            Err(TryLockError::WouldBlock)
        }
    }

    /// Returns a mutable reference to the underlying data.
    pub fn get_mut(&mut self) -> LockResult<&mut T> {
        Ok(self.data.get_mut().unwrap())
    }
}

impl<T: ?Sized + Default> Default for Mutex<T> {
    /// Creates a `Mutex<T>`, with the `Default` value for T.
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> From<T> for Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    /// This is equivalent to [`Mutex::new`].
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<'a, T: ?Sized + 'a> MutexGuard<'a, T> {
    pub(super) fn unborrow(&mut self) {
        self.data = None;
    }

    pub(super) fn reborrow(&mut self) {
        self.data = Some(self.lock.data.lock().unwrap());
    }

    pub(super) fn rt(&self) -> &rt::Mutex {
        &self.lock.object
    }
}

impl<'a, T: ?Sized> ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data.as_ref().unwrap().deref()
    }
}

impl<'a, T: ?Sized> ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data.as_mut().unwrap().deref_mut()
    }
}

impl<'a, T: ?Sized + 'a> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.data = None;
        self.lock.object.release_lock();
    }
}
