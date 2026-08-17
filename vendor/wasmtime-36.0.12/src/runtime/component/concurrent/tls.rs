//! A small self-contained module to manage passing a `&mut dyn VMStore` across
//! function boundaries without it actually being a function parameter.
//!
//! Much of concurrent.rs and futures_and_streams.rs work with `Future` which
//! does not allow customizing state being passed to each poll of a future. In
//! Wasmtime, however, the mutable store is available during a calls to
//! `Future::poll`, but not across calls of `Future::poll`. That means that
//! effectively what we would ideally want is to thread `&mut dyn VMStore` as a
//! parameter to futures, but that's not possible with Rust's future trait.
//!
//! This module is the workaround to otherwise enable this which is to use
//! thread-local-storage instead to pass around this pointer. The goal of this
//! module is to enable the `set` API to pretend like it's passing a pointer as
//! a parameter to a closure and then `get` can be called to acquire this
//! parameter. This module is intentionally small and isolated to keep the
//! internal implementation details private and reduce the surface area that
//! must be audited for the `unsafe` blocks contained within.

use crate::runtime::vm::VMStore;
use core::cell::Cell;
use core::mem;
use core::ptr::NonNull;

std::thread_local! {
    // Note that care is currently taken to minimize the size of this TLS
    // variable as it's expected we'll refactor this in the future and have to
    // plumb it to the platform abstraction layer of Wasmtime eventually where
    // we want as minimal an impact as possible. Thus this TLS variable is
    // a single pointer.
    static STORAGE: Cell<Option<NonNull<SetStorage>>> = const { Cell::new(None) };
}

enum SetStorage {
    Present(NonNull<dyn VMStore>),
    Taken,
}

/// Configures `store` to be available for the duration of `f` through calls to
/// the [`get`] function below.
///
/// This function will replace any prior state that was configured and overwrite
/// it. Upon `f` returning the previous state will be restored. This function
/// intentionally borrows `store` for the entire duration of `f` meaning that
/// `f` is not allowed to access `store` via Rust's borrow checker.
pub fn set<R>(store: &mut dyn VMStore, f: impl FnOnce() -> R) -> R {
    let mut storage = SetStorage::Present(NonNull::from(store));
    let _reset = ResetTls(STORAGE.with(|s| s.replace(Some(NonNull::from(&mut storage)))));
    return f();

    struct ResetTls(Option<NonNull<SetStorage>>);

    impl Drop for ResetTls {
        fn drop(&mut self) {
            STORAGE.with(|s| s.set(self.0));
        }
    }
}

/// Acquires a reference to the previous store configured via [`set`] above,
/// yielding this reference to the closure `f provided here.
///
/// This function will "take" the store from thread-local-storage for the
/// duration of the `get` function here. This "take" operation means that
/// recursive calls to `get` here will fail as the second one won't be able to
/// re-acquire the same pointer the first one has (due to it having `&mut`
/// exclusive access.
///
/// # Panics
///
/// This function will panic if [`set`] has not been previously called or if the
/// current pointer is taken by a previous call to [`get`] on the stack.
pub fn get<R>(f: impl FnOnce(&mut dyn VMStore) -> R) -> R {
    try_get(|val| match val {
        TryGet::Some(store) => f(store),
        TryGet::None | TryGet::Taken => get_failed(),
    })
}

#[cold]
fn get_failed() -> ! {
    panic!(
        "attempted to recursively call `tls::get` when the pointer was not \
         present or already taken by a previous call to `tls::get`"
    );
}

/// Values yielded to the [`try_get`] closure as an argument.
pub enum TryGet<'a> {
    /// The [`set`] API was not previously called, so there is no store
    /// available at all.
    None,
    /// The [`set`] API was previously called but it was then subsequently taken
    /// via a call to [`get`] meaning it's not available.
    Taken,
    /// The [`set`] API was previously called and this is the store that it was
    /// called with.
    Some(&'a mut dyn VMStore),
}

/// Same as [`get`] except that this does not panic if `set` has not been
/// called.
pub fn try_get<R>(f: impl FnOnce(TryGet<'_>) -> R) -> R {
    // SAFETY: This is The Unsafe Block of this module on which everything
    // hinges. The overall idea is that the pointer previously provided to
    // `set` is passed to the closure here but only at most once because it's
    // passed mutably. Thus there's a number of things that this takes care of:
    //
    // * The lifetime in `TryGet` that's handed out is anonymous via the
    //   type signature of `f`, meaning that it cannot be safely persisted
    //   outside that closure. That means that once `f` is returned this
    //   function has exclusive access to the store again.
    //
    // * If `STORAGE` is not set then that means `set` has not been configured,
    //   thus `TryGet::None` is yielded.
    //
    // * If `STORAGE` is set then we're guaranteed it's set for the entire
    //   lifetime of this function call, and we're also guaranteed that the
    //   pointer stored in there is the same pointer we'll be modifying for
    //   this whole function call.
    //
    // * The `STORAGE` pointer is read/written only in a scoped manner here and
    //   borrows of this value are not persisted for very long.
    //
    // With all of that put together it should make it such that this is a safe
    // reborrow of the store provided to `set` to pass to the closure `f` here.
    unsafe {
        let storage = STORAGE.with(|s| s.get());
        let _reset;
        let val = match storage {
            Some(mut storage) => match mem::replace(storage.as_mut(), SetStorage::Taken) {
                SetStorage::Taken => TryGet::Taken,
                SetStorage::Present(mut ptr) => {
                    _reset = ResetStorage(storage, ptr);
                    TryGet::Some(ptr.as_mut())
                }
            },
            None => TryGet::None,
        };
        return f(val);
    }

    struct ResetStorage(NonNull<SetStorage>, NonNull<dyn VMStore>);

    impl Drop for ResetStorage {
        fn drop(&mut self) {
            unsafe {
                *self.0.as_mut() = SetStorage::Present(self.1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TryGet, get, set, try_get};
    use crate::{AsContextMut, Engine, Store};

    #[test]
    fn test_simple() {
        let engine = Engine::default();
        let mut store = Store::new(&engine, ());

        set(store.as_context_mut().0, || {
            get(|_| {});
            try_get(|t| {
                assert!(matches!(t, TryGet::Some(_)));
            });
        });
    }

    #[test]
    fn test_try_get() {
        let engine = Engine::default();
        let mut store = Store::new(&engine, ());

        try_get(|t| {
            assert!(matches!(t, TryGet::None));
            try_get(|t| {
                assert!(matches!(t, TryGet::None));
            });
        });
        set(store.as_context_mut().0, || {
            get(|_| {
                try_get(|t| {
                    assert!(matches!(t, TryGet::Taken));
                    try_get(|t| {
                        assert!(matches!(t, TryGet::Taken));
                    });
                });
            });
            try_get(|t| {
                assert!(matches!(t, TryGet::Some(_)));
                try_get(|t| {
                    assert!(matches!(t, TryGet::Taken));
                    try_get(|t| {
                        assert!(matches!(t, TryGet::Taken));
                    });
                });
            });
            try_get(|t| {
                assert!(matches!(t, TryGet::Some(_)));
                try_get(|t| {
                    assert!(matches!(t, TryGet::Taken));
                });
            });
        });
        try_get(|t| {
            assert!(matches!(t, TryGet::None));
        });
    }

    #[test]
    #[should_panic(expected = "attempted to recursively call")]
    fn test_get_panic() {
        let engine = Engine::default();
        let mut store = Store::new(&engine, ());

        set(store.as_context_mut().0, || {
            get(|_| {
                get(|_| {
                    panic!("should not get here");
                });
            });
        });
    }
}
