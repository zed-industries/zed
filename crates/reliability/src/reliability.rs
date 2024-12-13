use futures::FutureExt as _;
use std::{
    cell::RefCell,
    future::Future,
    panic::{AssertUnwindSafe, PanicHookInfo},
};

thread_local! {
    static GLOBAL_HOOK: RefCell<Option<Box<dyn ReliabilityHook + Send + Sync>>> = RefCell::new(None);
}

pub trait ReliabilityHook {
    fn contextualize_panic(&self, info: &PanicHookInfo<'_>) -> String;
}

impl<F> ReliabilityHook for F
where
    F: Fn(&PanicHookInfo<'_>) -> String,
{
    fn contextualize_panic(&self, info: &PanicHookInfo<'_>) -> String {
        self(info)
    }
}

pub struct Guard(Option<Box<dyn ReliabilityHook + Send + Sync>>);

pub fn hook_fn(f: impl 'static + Send + Sync + Fn(&PanicHookInfo<'_>) -> String) -> Guard {
    GLOBAL_HOOK.with_borrow_mut(|global| {
        let old = std::mem::replace(global, Some(Box::new(f)));
        Guard(old)
    })
}

pub fn with_hook<R>(
    f: impl FnOnce(&(dyn ReliabilityHook + Send + Sync + 'static)) -> R,
) -> Option<R> {
    let result = GLOBAL_HOOK.with_borrow(|global| {
        let global = &**global.as_ref()?;
        Some(f(global))
    })?;
    Some(result)
}

impl Guard {
    pub fn catch_unwind<R>(&self, f: impl FnOnce() -> R) -> std::thread::Result<R> {
        std::panic::catch_unwind(AssertUnwindSafe(f))
    }

    pub async fn catch_unwind_future<R>(
        &self,
        f: impl Future<Output = R>,
    ) -> std::thread::Result<R> {
        AssertUnwindSafe(async {
            let _self = self;
            f.await
        })
        .catch_unwind()
        .await
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        GLOBAL_HOOK.with_borrow_mut(|global| {
            std::mem::swap(&mut self.0, global);
        });
    }
}
