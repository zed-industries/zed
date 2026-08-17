use std::cell::UnsafeCell;

use atomic::{Atomic, Ordering};

pub struct StateCell<S, T>
where
    S: Copy,
{
    state: Atomic<S>,
    data: UnsafeCell<Option<T>>,
}

impl<S, T> StateCell<S, T>
where
    S: Copy,
{
    pub fn new(state: S) -> Self {
        debug_assert!(Atomic::<S>::is_lock_free());

        Self {
            state: Atomic::new(state),
            data: UnsafeCell::new(None),
        }
    }

    pub unsafe fn compare_store(
        &self,
        current: S,
        locked: S,
        data: T,
        complete: S,
        success: Ordering,
        failure: Ordering,
    ) -> Result<S, (S, T)> {
        match self
            .state
            .compare_exchange(current, locked, success, failure)
        {
            Ok(s) => {
                *self.data.get() = Some(data);
                self.state.store(complete, Ordering::Release);
                Ok(s)
            }
            Err(s) => Err((s, data)),
        }
    }

    pub unsafe fn compare_take(
        &self,
        current: S,
        new: S,
        success: Ordering,
        failure: Ordering,
    ) -> Result<T, S> {
        match self.state.compare_exchange(current, new, success, failure) {
            Ok(_s) => Ok(self.take_internal()),
            Err(s) => Err(s),
        }
    }

    unsafe fn take_internal(&self) -> T {
        let reference = self.data.get().as_mut().unwrap();
        reference.take().unwrap()
    }
}

unsafe impl<S, T> Sync for StateCell<S, T>
where
    S: Copy,
    T: Send,
{
}
