use crate::rt;

use std::ops;
use std::sync::{LockResult, TryLockError, TryLockResult};

/// Mock implementation of `std::sync::RwLock`
#[derive(Debug)]
pub struct RwLock<T> {
    object: rt::RwLock,
    data: std::sync::RwLock<T>,
}

/// Mock implementation of `std::sync::RwLockReadGuard`
#[derive(Debug)]
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
    data: Option<std::sync::RwLockReadGuard<'a, T>>,
}

/// Mock implementation of `std::sync::rwLockWriteGuard`
#[derive(Debug)]
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
    /// `data` is an Option so that the Drop impl can drop the std guard and release the std lock
    /// before releasing the loom mock lock, as that might cause another thread to acquire the lock
    data: Option<std::sync::RwLockWriteGuard<'a, T>>,
}

impl<T> RwLock<T> {
    /// Creates a new rwlock in an unlocked state ready for use.
    pub fn new(data: T) -> RwLock<T> {
        RwLock {
            data: std::sync::RwLock::new(data),
            object: rt::RwLock::new(),
        }
    }

    /// Locks this rwlock with shared read access, blocking the current
    /// thread until it can be acquired.
    ///
    /// The calling thread will be blocked until there are no more writers
    /// which hold the lock. There may be other readers currently inside the
    /// lock when this method returns. This method does not provide any
    /// guarantees with respect to the ordering of whether contentious readers
    /// or writers will acquire the lock first.
    #[track_caller]
    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        self.object.acquire_read_lock(location!());

        Ok(RwLockReadGuard {
            lock: self,
            data: Some(self.data.try_read().expect("loom::RwLock state corrupt")),
        })
    }

    /// Attempts to acquire this rwlock with shared read access.
    ///
    /// If the access could not be granted at this time, then Err is returned.
    /// Otherwise, an RAII guard is returned which will release the shared
    /// access when it is dropped.
    ///
    /// This function does not block.
    #[track_caller]
    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, T>> {
        if self.object.try_acquire_read_lock(location!()) {
            Ok(RwLockReadGuard {
                lock: self,
                data: Some(self.data.try_read().expect("loom::RwLock state corrupt")),
            })
        } else {
            Err(TryLockError::WouldBlock)
        }
    }

    /// Locks this rwlock with exclusive write access, blocking the current
    /// thread until it can be acquired.
    ///
    /// This function will not return while other writers or other readers
    /// currently have access to the lock.
    #[track_caller]
    pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>> {
        self.object.acquire_write_lock(location!());

        Ok(RwLockWriteGuard {
            lock: self,
            data: Some(self.data.try_write().expect("loom::RwLock state corrupt")),
        })
    }

    /// Attempts to lock this rwlock with exclusive write access.
    ///
    /// If the lock could not be acquired at this time, then Err is returned.
    /// Otherwise, an RAII guard is returned which will release the lock when
    /// it is dropped.
    ///
    /// This function does not block.
    #[track_caller]
    pub fn try_write(&self) -> TryLockResult<RwLockWriteGuard<'_, T>> {
        if self.object.try_acquire_write_lock(location!()) {
            Ok(RwLockWriteGuard {
                lock: self,
                data: Some(self.data.try_write().expect("loom::RwLock state corrupt")),
            })
        } else {
            Err(TryLockError::WouldBlock)
        }
    }

    /// Returns a mutable reference to the underlying data.
    pub fn get_mut(&mut self) -> LockResult<&mut T> {
        Ok(self.data.get_mut().expect("loom::RwLock state corrupt"))
    }

    /// Consumes this `RwLock`, returning the underlying data.
    pub fn into_inner(self) -> LockResult<T> {
        Ok(self.data.into_inner().expect("loom::RwLock state corrupt"))
    }
}

impl<T: Default> Default for RwLock<T> {
    /// Creates a `RwLock<T>`, with the `Default` value for T.
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> From<T> for RwLock<T> {
    /// Creates a new rwlock in an unlocked state ready for use.
    /// This is equivalent to [`RwLock::new`].
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<'a, T> ops::Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data.as_ref().unwrap().deref()
    }
}

impl<'a, T: 'a> Drop for RwLockReadGuard<'a, T> {
    fn drop(&mut self) {
        self.data = None;
        self.lock.object.release_read_lock()
    }
}

impl<'a, T> ops::Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data.as_ref().unwrap().deref()
    }
}

impl<'a, T> ops::DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data.as_mut().unwrap().deref_mut()
    }
}

impl<'a, T: 'a> Drop for RwLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.data = None;
        self.lock.object.release_write_lock()
    }
}
