#![cfg_attr(feature = "use_std", allow(dead_code))]

use core::marker;
use core::mem;
use core::sync::atomic::AtomicUsize;
#[allow(deprecated)]
use core::sync::atomic::ATOMIC_USIZE_INIT;
use core::sync::atomic::Ordering::{SeqCst, Relaxed};

use super::{BorrowedTask, NotifyHandle};

pub struct LocalKey;
pub struct LocalMap;
pub fn local_map() -> LocalMap { LocalMap }

#[derive(Copy, Clone)]
pub struct BorrowedEvents<'a>(marker::PhantomData<&'a ()>);

#[derive(Copy, Clone)]
pub struct BorrowedUnpark<'a> {
    f: &'a Fn() -> NotifyHandle,
    id: usize,
}

pub struct TaskUnpark {
    handle: NotifyHandle,
    id: usize,
}

#[derive(Clone)]
pub struct UnparkEvents;

impl<'a> BorrowedEvents<'a> {
    pub fn new() -> BorrowedEvents<'a> {
        BorrowedEvents(marker::PhantomData)
    }

    pub fn to_owned(&self) -> UnparkEvents {
        UnparkEvents
    }
}

impl<'a> BorrowedUnpark<'a> {
    #[inline]
    pub fn new(f: &'a Fn() -> NotifyHandle, id: usize) -> BorrowedUnpark<'a> {
        BorrowedUnpark { f: f, id: id }
    }

    #[inline]
    pub fn to_owned(&self) -> TaskUnpark {
        let handle = (self.f)();
        let id = handle.clone_id(self.id);
        TaskUnpark { handle: handle, id: id }
    }
}

impl UnparkEvents {
    pub fn notify(&self) {}

    pub fn will_notify(&self, _other: &BorrowedEvents) -> bool {
        true
    }
}

impl TaskUnpark {
    pub fn notify(&self) {
        self.handle.notify(self.id);
    }

    pub fn will_notify(&self, other: &BorrowedUnpark) -> bool {
        self.id == other.id && self.handle.inner == (other.f)().inner
    }
}

impl Clone for TaskUnpark {
    fn clone(&self) -> TaskUnpark {
        let handle = self.handle.clone();
        let id = handle.clone_id(self.id);
        TaskUnpark { handle: handle, id: id }
    }
}

impl Drop for TaskUnpark {
    fn drop(&mut self) {
        self.handle.drop_id(self.id);
    }
}

#[allow(deprecated)]
static GET: AtomicUsize = ATOMIC_USIZE_INIT;
#[allow(deprecated)]
static SET: AtomicUsize = ATOMIC_USIZE_INIT;

/// Initialize the `futures` task system.
///
/// This function is an unsafe low-level implementation detail typically only
/// used by crates using `futures` in `no_std` context. Users of this crate
/// who also use the standard library never need to invoke this function.
///
/// The task system in the `futures` crate relies on some notion of "local
/// storage" for the running thread and/or context. The `task::current` function
/// can get invoked in any context, for example, and needs to be able to return
/// a `Task`. Typically with the standard library this is supported with
/// thread-local-storage, but this is not available in `no_std` contexts!
///
/// This function is provided to allow `no_std` contexts to continue to be able
/// to use the standard task system in this crate. The functions provided here
/// will be used as-if they were thread-local-storage getters/setters. The `get`
/// function provided is used to retrieve the current thread-local value of the
/// task system's pointer, returning null if not initialized. The `set` function
/// updates the value of the pointer.
///
/// # Return value
///
/// This function will return whether initialization succeeded or not. This
/// function can be called concurrently and only the first invocation will
/// succeed. If `false` is returned then the `get` and `set` pointers provided
/// were *not* registered for use with the task system, but if `true` was
/// provided then they will be called when the task system is used.
///
/// Note that while safe to call concurrently it's recommended to still perform
/// external synchronization when calling this function. This task system is
/// not guaranteed to be ready to go until a call to this function returns
/// `true`. In other words, if you call this function and see `false`, the
/// task system may not be ready to go as another thread may still be calling
/// `init`.
///
/// # Unsafety
///
/// This function is unsafe due to the requirements on the behavior of the
/// `get` and `set` functions. The pointers returned from these functions must
/// reflect the semantics specified above and must also be thread-local,
/// depending on the definition of a "thread" in the calling context.
pub unsafe fn init(get: fn() -> *mut u8, set: fn(*mut u8)) -> bool {
    if GET.compare_exchange(0, get as usize, SeqCst, SeqCst).is_ok() {
        SET.store(set as usize, SeqCst);
        true
    } else {
        false
    }
}

/// Return whether the caller is running in a task (and so can use task_local!).
pub fn is_in_task() -> bool {
    if let Some(ptr) = get_ptr() {
        !ptr.is_null()
    } else {
        false
    }
}

#[inline]
pub fn get_ptr() -> Option<*mut u8> {
    match GET.load(Relaxed) {
        0 => None,
        n => Some(unsafe { mem::transmute::<usize, fn() -> *mut u8>(n)() }),
    }
}

#[cfg(feature = "use_std")]
#[inline]
pub fn is_get_ptr(f: usize) -> bool {
    GET.load(Relaxed) == f
}

pub fn set<'a, F, R>(task: &BorrowedTask<'a>, f: F) -> R
    where F: FnOnce() -> R
{
    let set = match SET.load(Relaxed) {
        0 => panic!("not initialized"),
        n => unsafe { mem::transmute::<usize, fn(*mut u8)>(n) },
    };

    struct Reset(fn(*mut u8), *mut u8);

    impl Drop for Reset {
        #[inline]
        fn drop(&mut self) {
            (self.0)(self.1);
        }
    }

    let _reset = Reset(set, get_ptr().unwrap());
    set(task as *const _ as *mut u8);
    f()
}
