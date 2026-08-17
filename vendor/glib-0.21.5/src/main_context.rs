// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use crate::ffi::{self, gboolean, gpointer};

use crate::{source::Priority, translate::*, MainContext, Source, SourceId};

impl MainContext {
    #[doc(alias = "g_main_context_prepare")]
    pub fn prepare(&self) -> (bool, i32) {
        unsafe {
            let mut priority = mem::MaybeUninit::uninit();

            let res = from_glib(ffi::g_main_context_prepare(
                self.to_glib_none().0,
                priority.as_mut_ptr(),
            ));
            let priority = priority.assume_init();
            (res, priority)
        }
    }

    #[doc(alias = "g_main_context_find_source_by_id")]
    pub fn find_source_by_id(&self, source_id: &SourceId) -> Option<Source> {
        unsafe {
            from_glib_none(ffi::g_main_context_find_source_by_id(
                self.to_glib_none().0,
                source_id.as_raw(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Invokes `func` on the main context.
    ///
    /// If the current thread is the owner of the main context or the main context currently has no
    /// owner then `func` will be called directly from inside this function. If this behaviour is
    /// not desired and `func` should always be called asynchronously then use [`MainContext::spawn`]
    /// [`glib::idle_add`](crate::idle_add) instead.
    #[doc(alias = "g_main_context_invoke")]
    pub fn invoke<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.invoke_with_priority(crate::Priority::DEFAULT_IDLE, func);
    }

    // rustdoc-stripper-ignore-next
    /// Invokes `func` on the main context with the given priority.
    ///
    /// If the current thread is the owner of the main context or the main context currently has no
    /// owner then `func` will be called directly from inside this function. If this behaviour is
    /// not desired and `func` should always be called asynchronously then use [`MainContext::spawn`]
    /// [`glib::idle_add`](crate::idle_add) instead.
    #[doc(alias = "g_main_context_invoke_full")]
    pub fn invoke_with_priority<F>(&self, priority: Priority, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe {
            self.invoke_unsafe(priority, func);
        }
    }

    // rustdoc-stripper-ignore-next
    /// Invokes `func` on the main context.
    ///
    /// Different to `invoke()`, this does not require `func` to be
    /// `Send` but can only be called from the thread that owns the main context.
    ///
    /// This function panics if called from a different thread than the one that
    /// owns the main context.
    ///
    /// Note that this effectively means that `func` is called directly from inside this function
    /// or otherwise panics immediately. If this behaviour is not desired and `func` should always
    /// be called asynchronously then use [`MainContext::spawn_local`]
    /// [`glib::idle_add_local`](crate::idle_add_local) instead.
    pub fn invoke_local<F>(&self, func: F)
    where
        F: FnOnce() + 'static,
    {
        self.invoke_local_with_priority(crate::Priority::DEFAULT_IDLE, func);
    }

    // rustdoc-stripper-ignore-next
    /// Invokes `func` on the main context with the given priority.
    ///
    /// Different to `invoke_with_priority()`, this does not require `func` to be
    /// `Send` but can only be called from the thread that owns the main context.
    ///
    /// This function panics if called from a different thread than the one that
    /// owns the main context.
    ///
    /// Note that this effectively means that `func` is called directly from inside this function
    /// or otherwise panics immediately. If this behaviour is not desired and `func` should always
    /// be called asynchronously then use [`MainContext::spawn_local`]
    /// [`glib::idle_add_local`](crate::idle_add_local) instead.
    #[allow(clippy::if_same_then_else)]
    pub fn invoke_local_with_priority<F>(&self, _priority: Priority, func: F)
    where
        F: FnOnce() + 'static,
    {
        // Checks from `g_main_context_invoke_full()`
        // FIXME: Combine the first two cases somehow
        if self.is_owner() {
            func();
        } else if let Ok(_acquire) = self.acquire() {
            func();
        } else {
            panic!("Must be called from a thread that owns the main context");
        }
    }

    unsafe fn invoke_unsafe<F>(&self, priority: Priority, func: F)
    where
        F: FnOnce() + 'static,
    {
        unsafe extern "C" fn trampoline<F: FnOnce() + 'static>(func: gpointer) -> gboolean {
            let func: &mut Option<F> = &mut *(func as *mut Option<F>);
            let func = func
                .take()
                .expect("MainContext::invoke() closure called multiple times");
            func();
            ffi::G_SOURCE_REMOVE
        }
        unsafe extern "C" fn destroy_closure<F: FnOnce() + 'static>(ptr: gpointer) {
            let _ = Box::<Option<F>>::from_raw(ptr as *mut _);
        }
        let func = Box::into_raw(Box::new(Some(func)));
        ffi::g_main_context_invoke_full(
            self.to_glib_none().0,
            priority.into_glib(),
            Some(trampoline::<F>),
            func as gpointer,
            Some(destroy_closure::<F>),
        )
    }

    // rustdoc-stripper-ignore-next
    /// Call closure with the main context configured as the thread default one.
    ///
    /// The thread default main context is changed in a panic-safe manner before calling `func` and
    /// released again afterwards regardless of whether closure panicked or not.
    ///
    /// This will fail if the main context is owned already by another thread.
    #[doc(alias = "g_main_context_push_thread_default")]
    pub fn with_thread_default<R, F: FnOnce() -> R + Sized>(
        &self,
        func: F,
    ) -> Result<R, crate::BoolError> {
        let _acquire = self.acquire()?;
        let _thread_default = ThreadDefaultContext::new(self);
        Ok(func())
    }

    // rustdoc-stripper-ignore-next
    /// Acquire ownership of the main context.
    ///
    /// Ownership will automatically be released again once the returned acquire guard is dropped.
    ///
    /// This will fail if the main context is owned already by another thread.
    #[doc(alias = "g_main_context_acquire")]
    pub fn acquire(&self) -> Result<MainContextAcquireGuard<'_>, crate::BoolError> {
        unsafe {
            let ret: bool = from_glib(ffi::g_main_context_acquire(self.to_glib_none().0));
            if ret {
                Ok(MainContextAcquireGuard(self))
            } else {
                Err(bool_error!("Failed to acquire ownership of main context, already acquired by another thread"))
            }
        }
    }
}

#[must_use = "if unused the main context will be released immediately"]
pub struct MainContextAcquireGuard<'a>(&'a MainContext);

impl Drop for MainContextAcquireGuard<'_> {
    #[doc(alias = "g_main_context_release")]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::g_main_context_release(self.0.to_glib_none().0);
        }
    }
}

struct ThreadDefaultContext<'a>(&'a MainContext);

impl ThreadDefaultContext<'_> {
    fn new(ctx: &MainContext) -> ThreadDefaultContext<'_> {
        unsafe {
            ffi::g_main_context_push_thread_default(ctx.to_glib_none().0);
        }
        ThreadDefaultContext(ctx)
    }
}

impl Drop for ThreadDefaultContext<'_> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::g_main_context_pop_thread_default(self.0.to_glib_none().0);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{panic, ptr, thread};

    use super::*;

    #[test]
    fn test_invoke() {
        let c = MainContext::new();
        let l = crate::MainLoop::new(Some(&c), false);

        let l_clone = l.clone();
        let join_handle = thread::spawn(move || {
            c.invoke(move || l_clone.quit());
        });

        l.run();

        join_handle.join().unwrap();
    }

    fn is_same_context(a: &MainContext, b: &MainContext) -> bool {
        ptr::eq(a.to_glib_none().0, b.to_glib_none().0)
    }

    #[test]
    fn test_with_thread_default() {
        let a = MainContext::new();
        let b = MainContext::new();

        assert!(!is_same_context(&a, &b));

        a.with_thread_default(|| {
            let t = MainContext::thread_default().unwrap();
            assert!(is_same_context(&a, &t));

            b.with_thread_default(|| {
                let t = MainContext::thread_default().unwrap();
                assert!(is_same_context(&b, &t));
            })
            .unwrap();

            let t = MainContext::thread_default().unwrap();
            assert!(is_same_context(&a, &t));
        })
        .unwrap();
    }

    #[test]
    fn test_with_thread_default_is_panic_safe() {
        let a = MainContext::new();
        let b = MainContext::new();

        assert!(!is_same_context(&a, &b));

        a.with_thread_default(|| {
            let t = MainContext::thread_default().unwrap();
            assert!(is_same_context(&a, &t));

            let result = panic::catch_unwind(|| {
                b.with_thread_default(|| {
                    panic!();
                })
                .unwrap();
            });
            assert!(result.is_err());

            let t = MainContext::thread_default().unwrap();
            assert!(is_same_context(&a, &t));
        })
        .unwrap();
    }
}
