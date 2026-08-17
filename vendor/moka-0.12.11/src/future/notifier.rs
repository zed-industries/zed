use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use futures_util::FutureExt;

use crate::notification::{AsyncEvictionListener, RemovalCause};

pub(crate) struct RemovalNotifier<K, V> {
    listener: AsyncEvictionListener<K, V>,
    is_enabled: AtomicBool,
    #[cfg(feature = "logging")]
    cache_name: Option<String>,
}

impl<K, V> RemovalNotifier<K, V> {
    pub(crate) fn new(listener: AsyncEvictionListener<K, V>, _cache_name: Option<String>) -> Self {
        Self {
            listener,
            is_enabled: AtomicBool::new(true),
            #[cfg(feature = "logging")]
            cache_name: _cache_name,
        }
    }

    pub(crate) async fn notify(&self, key: Arc<K>, value: V, cause: RemovalCause) {
        use std::panic::{catch_unwind, AssertUnwindSafe};

        if !self.is_enabled.load(Ordering::Acquire) {
            return;
        }

        // This macro unwraps the result of the catch_unwind call if it is Ok. And
        // disable the notifier and do early return if the listener panicked.
        macro_rules! try_or_disable {
            ($match_expr:expr) => {
                match $match_expr {
                    Ok(v) => v,
                    Err(_payload) => {
                        self.is_enabled.store(false, Ordering::Release);
                        #[cfg(feature = "logging")]
                        log_panic(&*_payload, self.cache_name.as_deref());
                        return;
                    }
                }
            };
        }

        let listener_clo = || (self.listener)(key, value, cause);

        // Safety: It is safe to assert unwind safety here because we will not
        // call the listener again if it has been panicked.
        let fut = try_or_disable!(catch_unwind(AssertUnwindSafe(listener_clo)));
        try_or_disable!(AssertUnwindSafe(fut).catch_unwind().await);
    }
}

#[cfg(feature = "logging")]
fn log_panic(payload: &(dyn std::any::Any + Send + 'static), cache_name: Option<&str>) {
    // Try to downcast the payload into &str or String.
    //
    // NOTE: Clippy will complain if we use `if let Some(_)` here.
    // https://rust-lang.github.io/rust-clippy/master/index.html#manual_map
    let message: Option<std::borrow::Cow<'_, str>> =
        (payload.downcast_ref::<&str>().map(|s| (*s).into()))
            .or_else(|| payload.downcast_ref::<String>().map(Into::into));

    let cn = cache_name
        .map(|name| format!("[{name}] "))
        .unwrap_or_default();

    if let Some(m) = message {
        log::error!("{cn}Disabled the eviction listener because it panicked at '{m}'");
    } else {
        log::error!("{cn}Disabled the eviction listener because it panicked");
    }
}
