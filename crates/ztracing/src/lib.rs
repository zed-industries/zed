pub use tracing::{Level, field};

#[cfg(feature = "tracy")]
pub use tracing::{
    Span, debug_span, error_span, event, info_span, instrument, span, trace_span, warn_span,
};
#[cfg(not(feature = "tracy"))]
pub use ztracing_macro::instrument;

#[cfg(not(feature = "tracy"))]
pub use __consume_all_tokens as trace_span;
#[cfg(not(feature = "tracy"))]
pub use __consume_all_tokens as info_span;
#[cfg(not(feature = "tracy"))]
pub use __consume_all_tokens as debug_span;
#[cfg(not(feature = "tracy"))]
pub use __consume_all_tokens as warn_span;
#[cfg(not(feature = "tracy"))]
pub use __consume_all_tokens as error_span;
#[cfg(not(feature = "tracy"))]
pub use __consume_all_tokens as event;
#[cfg(not(feature = "tracy"))]
pub use __consume_all_tokens as span;

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! __consume_all_tokens {
    ($($t:tt)*) => {
        $crate::Span
    };
}

#[cfg(not(feature = "tracy"))]
pub struct Span;

#[cfg(not(feature = "tracy"))]
impl Span {
    pub fn current() -> Self {
        Self
    }

    pub fn enter(&self) {}

    pub fn record<T, S>(&self, _t: T, _s: S) {}
}

#[cfg(feature = "tracy")]
pub fn init() {
    zlog::info!("Starting tracy subscriber, you can now connect the profiler");
    use tracing_subscriber::prelude::*;
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
    )
    .expect("setup tracy layer");
}

#[cfg(not(feature = "tracy"))]
pub fn init() {}

/// Returns true if this build was compiled with Tracy profiling support.
///
/// When true, `init()` will set up the Tracy subscriber and the application
/// can be profiled by connecting Tracy profiler to it.
#[cfg(feature = "tracy")]
pub const fn is_enabled() -> bool {
    true
}

/// Returns true if this build was compiled with Tracy profiling support.
///
/// When true, `init()` will set up the Tracy subscriber and the application
/// can be profiled by connecting Tracy profiler to it.
#[cfg(not(feature = "tracy"))]
pub const fn is_enabled() -> bool {
    false
}
