//! Mock implementation of the `lazy_static` crate.

use crate::rt;
pub use crate::rt::thread::AccessError;
pub use crate::rt::yield_now;
use crate::sync::atomic::Ordering;

#[doc(no_inline)]
pub use std::thread::panicking;

use std::fmt;
use std::marker::PhantomData;

/// Mock implementation of `lazy_static::Lazy`.
pub struct Lazy<T> {
    // Sadly, these fields have to be public, since function pointers in const
    // fns are unstable. When fn pointer arguments to const fns stabilize, these
    // should be made private and replaced with a `const fn new`.
    //
    // User code should not rely on the existence of these fields.
    #[doc(hidden)]
    pub init: fn() -> T,
    #[doc(hidden)]
    pub _p: PhantomData<fn(T)>,
}

impl<T: 'static> Lazy<T> {
    /// Mock implementation of `lazy_static::Lazy::get`.
    pub fn get(&'static self) -> &'static T {
        // This is not great. Specifically, we're returning a 'static reference to a value that
        // only lives for the duration of the execution. Unfortunately, the semantics of lazy
        // static is that, well, you get a value that is in fact 'static. If we did not provide
        // that, then this replacement wouldn't actually work.
        //
        // The "upside" here is that _if_ the code compiled with `lazy_static::lazy_static!`,
        // _then_ this is safe. That's not super satisfying, but I'm not sure how we do better
        // without changing the API pretty drastically. We could perhaps here provide a
        // `with(closure)` like we do for `UnsafeCell`, and require people to wrap the "real"
        // `lazy_static` the same way, but that seems like its own kind of unfortunate as I'm sure
        // users sometimes _rely_ on the returned reference being 'static. If we provided something
        // that used a closure to give the user a non-`'static` reference, we wouldn't be all that
        // much further along.
        match unsafe { self.try_get() } {
            Some(v) => v,
            None => {
                // Init the value out of the `rt::execution`
                let sv = crate::rt::lazy_static::StaticValue::new((self.init)());

                // While calling init, we may have yielded to the scheduler, in which case some
                // _other_ thread may have initialized the static. The real lazy_static does not
                // have this issue, since it takes a lock before initializing the new value, and
                // readers wait on that lock if they encounter it. We could implement that here
                // too, but for simplicity's sake, we just do another try_get here for now.
                if let Some(v) = unsafe { self.try_get() } {
                    return v;
                }

                rt::execution(|execution| {
                    let sv = execution.lazy_statics.init_static(self, sv);

                    // lazy_static uses std::sync::Once, which does a swap(AcqRel) to set
                    sv.sync.sync_store(&mut execution.threads, Ordering::AcqRel);
                });

                unsafe { self.try_get() }.expect("bug")
            }
        }
    }

    unsafe fn try_get(&'static self) -> Option<&'static T> {
        unsafe fn transmute_lt<'a, 'b, T>(t: &'a T) -> &'b T {
            std::mem::transmute::<&'a T, &'b T>(t)
        }

        let sv = rt::execution(|execution| {
            let sv = execution.lazy_statics.get_static(self)?;

            // lazy_static uses std::sync::Once, which does a load(Acquire) to get
            sv.sync.sync_load(&mut execution.threads, Ordering::Acquire);

            Some(transmute_lt(sv))
        })?;

        Some(sv.get::<T>())
    }
}

impl<T: 'static> fmt::Debug for Lazy<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Lazy { .. }")
    }
}
