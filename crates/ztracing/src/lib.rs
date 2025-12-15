pub use tracing::Level;

#[cfg(ztracing)]
pub use tracing::{
    debug_span, error_span, event, info_span, instrument, span, trace_span, warn_span,
};
#[cfg(not(ztracing))]
pub use ztracing_macro::instrument;

#[cfg(not(ztracing))]
pub use __consume_all_tokens as trace_span;
#[cfg(not(ztracing))]
pub use __consume_all_tokens as info_span;
#[cfg(not(ztracing))]
pub use __consume_all_tokens as debug_span;
#[cfg(not(ztracing))]
pub use __consume_all_tokens as warn_span;
#[cfg(not(ztracing))]
pub use __consume_all_tokens as error_span;
#[cfg(not(ztracing))]
pub use __consume_all_tokens as event;
#[cfg(not(ztracing))]
pub use __consume_all_tokens as span;

#[cfg(not(ztracing))]
#[macro_export]
macro_rules! __consume_all_tokens {
    ($($t:tt)*) => {
        $crate::FakeSpan
    };
}

pub struct FakeSpan;
impl FakeSpan {
    pub fn enter(&self) {}
}

// #[cfg(not(ztracing))]
// pub use span;

#[cfg(ztracing)]
pub fn init() {
    zlog::info!("Starting tracy subscriber, you can now connect the profiler");
    use tracing_subscriber::prelude::*;
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
    )
    .expect("setup tracy layer");
}

#[cfg(not(ztracing))]
pub fn init() {}
