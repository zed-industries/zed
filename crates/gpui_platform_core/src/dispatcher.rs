//! The platform dispatcher contract shared by `gpui` and its platform backends.

use std::any::Any;
use std::time::Duration;

use gpui_util::defer;
use scheduler::Instant;
use scheduler::Priority;

use crate::{ActivityGuard, RunnableVariant, TimerResolutionGuard};

#[cfg(any(test, feature = "test-support"))]
use crate::TestDispatcher;

/// This type is public so that our test macro can generate and use it, but it should not
/// be considered part of our public API.
#[doc(hidden)]
pub trait PlatformDispatcher: Send + Sync + Any {
    fn is_main_thread(&self) -> bool;
    fn dispatch(&self, runnable: RunnableVariant, priority: Priority);
    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority);
    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant);

    fn dispatch_on_main_thread_when_idle(
        &self,
        runnable: RunnableVariant,
        timeout: Option<Duration>,
    ) {
        let _ = timeout;
        self.dispatch_on_main_thread(runnable, Priority::Low);
    }

    fn idle_time_remaining(&self) -> Option<Duration> {
        None
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>);

    fn now(&self) -> Instant {
        Instant::now()
    }

    fn increase_timer_resolution(&self) -> TimerResolutionGuard {
        defer(Box::new(|| {}))
    }

    fn prevent_app_nap(&self, _reason: &str) -> ActivityGuard {
        ActivityGuard::noop()
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_test(&self) -> Option<&TestDispatcher> {
        None
    }
}
