use super::{Guard, RefCnt};

mod sealed {
    pub trait Sealed {}
}

use self::sealed::Sealed;

/// A trait describing things that can be turned into a raw pointer.
///
/// This is just an abstraction of things that can be passed to the
/// [`compare_and_swap`](struct.ArcSwapAny.html#method.compare_and_swap).
///
/// # Examples
///
/// ```
/// use std::ptr;
/// use std::sync::Arc;
///
/// use arc_swap::ArcSwapOption;
///
/// let a = Arc::new(42);
/// let shared = ArcSwapOption::from(Some(Arc::clone(&a)));
///
/// shared.compare_and_swap(&a, Some(Arc::clone(&a)));
/// shared.compare_and_swap(&None::<Arc<_>>, Some(Arc::clone(&a)));
/// shared.compare_and_swap(shared.load(), Some(Arc::clone(&a)));
/// shared.compare_and_swap(&shared.load(), Some(Arc::clone(&a)));
/// shared.compare_and_swap(ptr::null(), Some(Arc::clone(&a)));
/// ```
///
/// Due to technical limitation, this is not implemented for owned `Arc`/`Option<Arc<_>>`, they
/// need to be borrowed.
pub trait AsRaw<T>: Sealed {
    /// Converts the value into a raw pointer.
    fn as_raw(&self) -> *mut T;
}

impl<T: RefCnt> Sealed for &T {}
impl<T: RefCnt> AsRaw<T::Base> for &T {
    fn as_raw(&self) -> *mut T::Base {
        T::as_ptr(self)
    }
}

impl<T: RefCnt> Sealed for &Guard<T> {}
impl<T: RefCnt> AsRaw<T::Base> for &Guard<T> {
    fn as_raw(&self) -> *mut T::Base {
        T::as_ptr(self)
    }
}

impl<T: RefCnt> Sealed for Guard<T> {}
impl<T: RefCnt> AsRaw<T::Base> for Guard<T> {
    fn as_raw(&self) -> *mut T::Base {
        T::as_ptr(self)
    }
}

impl<T> Sealed for *mut T {}
impl<T> AsRaw<T> for *mut T {
    fn as_raw(&self) -> *mut T {
        *self
    }
}

impl<T> Sealed for *const T {}
impl<T> AsRaw<T> for *const T {
    fn as_raw(&self) -> *mut T {
        *self as *mut T
    }
}
