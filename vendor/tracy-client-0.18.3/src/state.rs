use crate::Client;

/// Client initialization and lifetime management.
impl Client {
    /// Start the client.
    ///
    /// The client must be started with this function before any instrumentation is invoked
    /// anywhere in the process. This function can be called multiple times to obtain multiple
    /// `Client` values.
    ///
    /// The underlying client implementation will be started up only if it wasn't already running
    /// yet.
    ///
    /// Note that when the `manual-lifetime` feature is used, it is a responsibility of the user
    /// to stop `tracy` using the [`sys::___tracy_shutdown_profiler`] function. Keep in mind that
    /// at the time this function is called there can be no other invocations to the tracy
    /// profiler, even from other threads (or you may get a crash!)
    ///
    /// # Example
    ///
    /// ```rust
    /// // fn main() {
    ///     let _client = tracy_client::Client::start();
    ///     // ...
    /// // }
    /// ```
    pub fn start() -> Self {
        #[cfg(not(feature = "enable"))]
        return Self(());
        #[cfg(all(feature = "enable", feature = "manual-lifetime"))]
        return manual_lifetime::start();
        #[cfg(all(feature = "enable", not(feature = "manual-lifetime")))]
        return Self(());
    }

    /// Obtain a client handle, but only if the client is already running.
    #[must_use]
    pub fn running() -> Option<Self> {
        if Self::is_running() {
            Some(Self(()))
        } else {
            None
        }
    }

    /// Is the client already running?
    pub fn is_running() -> bool {
        #[cfg(not(feature = "enable"))]
        return true; // If the client is disabled, produce a "no-op" one so that users don’t need
                     // to wory about conditional use of the instrumentation in their own code.
        #[cfg(all(feature = "enable", feature = "manual-lifetime"))]
        return manual_lifetime::is_running();
        #[cfg(all(feature = "enable", not(feature = "manual-lifetime")))]
        return true; // The client is started in life-before-main (or upon first use in case of
                     // `delayed-init`
    }
}

impl Clone for Client {
    /// A cheaper alternative to [`Client::start`] or [`Client::running`]  when there is already a
    /// handle handy.
    fn clone(&self) -> Self {
        // We already know that the state is `ENABLED`, no need to check.
        Self(())
    }
}

#[cfg(all(feature = "enable", feature = "manual-lifetime"))]
mod manual_lifetime {
    use std::sync::atomic::Ordering;
    /// Enabling `Tracy` when it is already enabled, or Disabling when it is already disabled will
    /// cause applications to crash. I personally think it would be better if this was a sort-of
    /// reference counted kind-of thing so you could enable as many times as you wish and disable
    /// just as many times without any reprecursions. At the very least this could significantly
    /// help tests.
    ///
    /// We can also try to implement something like this ourselves. To do this we'd want to track 4
    /// states that construct a following finite state machine:
    ///
    /// ```text
    ///     0 = disabled  -> 1 = enabling
    ///         ^                v
    ///     3 = disabling <- 2 = enabled
    /// ```
    ///
    /// And also include a reference count somewhere in there. Something we can throw in a static
    /// would be ideal.
    ///
    /// Alas, Tracy's extensive use of thread-local storage presents us with another problem – we must
    /// start up and shut down the client within the same thread. A most straightforward soution for
    /// that would be to run a separate thread that would be dedicated entirely to just starting up and
    /// shutting down the profiler.
    ///
    /// All that seems like a major pain to implement, and so we’ll punt on disabling entirely until
    /// somebody comes with a good use-case warranting that sort of complexity.
    #[cfg(not(loom))]
    static CLIENT_STATE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    #[cfg(loom)]
    loom::lazy_static! {
        static ref CLIENT_STATE: loom::sync::atomic::AtomicUsize =
            loom::sync::atomic::AtomicUsize::new(0);
    }

    const STATE_STEP: usize = 1; // Move forward by 1 step in the FSM
    const STATE_DISABLED: usize = 0;
    const STATE_ENABLING: usize = STATE_DISABLED + STATE_STEP;
    const STATE_ENABLED: usize = STATE_ENABLING + STATE_STEP;

    #[inline(always)]
    fn spin_loop() {
        #[cfg(loom)]
        loom::thread::yield_now();
        #[cfg(not(loom))]
        std::hint::spin_loop();
    }

    pub(super) fn start() -> super::Client {
        let mut old_state = CLIENT_STATE.load(Ordering::Relaxed);
        loop {
            match old_state {
                STATE_ENABLED => return super::Client(()),
                STATE_ENABLING => {
                    while !is_running() {
                        spin_loop();
                    }
                    return super::Client(());
                }
                STATE_DISABLED => {
                    let result = CLIENT_STATE.compare_exchange_weak(
                        old_state,
                        STATE_ENABLING,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    );
                    if let Err(next_old_state) = result {
                        old_state = next_old_state;
                        continue;
                    } else {
                        unsafe {
                            // SAFE: This function must not be called if the profiler has
                            // already been enabled. While in practice calling this function
                            // multiple times will only serve to trigger an assertion, we
                            // cannot exactly rely on this, since it is an undocumented
                            // behaviour and the upstream might very well just decide to invoke
                            // UB instead. In the case there are multiple copies of
                            // `tracy-client` this invariant is not actually maintained, but
                            // otherwise this is sound due to the `ENABLE_STATE` that we
                            // manage.
                            //
                            // TODO: we _could_ define `ENABLE_STATE` in the `-sys` crate...
                            let () = sys::___tracy_startup_profiler();
                            CLIENT_STATE.store(STATE_ENABLED, Ordering::Release);
                            return super::Client(());
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    pub(super) fn is_running() -> bool {
        return CLIENT_STATE.load(Ordering::Relaxed) == STATE_ENABLED;
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn state_transitions() {
            assert_eq!(0, STATE_DISABLED);
            assert_eq!(STATE_DISABLED.wrapping_add(STATE_STEP), STATE_ENABLING);
            assert_eq!(STATE_ENABLING.wrapping_add(STATE_STEP), STATE_ENABLED);
        }
    }
}
