pub use tracing::{Level, field};

#[cfg(any(ztracing, all(target_family = "wasm", feature = "web")))]
pub use tracing::{
    Span, debug_span, error_span, event, info_span, instrument, span, trace_span, warn_span,
};

#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub use ztracing_macro::instrument;

#[cfg(all(ztracing, not(target_family = "wasm")))]
const MAX_CALLSTACK_DEPTH: u16 = 16;

#[cfg(all(ztracing, ztracing_with_memory, not(target_family = "wasm")))]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, MAX_CALLSTACK_DEPTH);

#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub use __consume_all_tokens as trace_span;
#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub use __consume_all_tokens as info_span;
#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub use __consume_all_tokens as debug_span;
#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub use __consume_all_tokens as warn_span;
#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub use __consume_all_tokens as error_span;
#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub use __consume_all_tokens as event;
#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub use __consume_all_tokens as span;

#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
#[macro_export]
macro_rules! __consume_all_tokens {
    ($($t:tt)*) => {
        $crate::Span
    };
}

#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
pub struct Span;

#[cfg(not(any(ztracing, all(target_family = "wasm", feature = "web"))))]
impl Span {
    pub fn current() -> Self {
        Self
    }

    pub fn enter(&self) {}

    pub fn record<T, S>(&self, _t: T, _s: S) {}
}

#[cfg(all(ztracing, not(target_family = "wasm")))]
pub fn init() {
    use tracing_subscriber::fmt::format::DefaultFields;
    use tracing_subscriber::prelude::*;

    #[derive(Default)]
    struct TracyLayerConfig {
        fmt: DefaultFields,
    }

    impl tracing_tracy::Config for TracyLayerConfig {
        type Formatter = DefaultFields;

        fn formatter(&self) -> &Self::Formatter {
            &self.fmt
        }

        fn stack_depth(&self, _: &tracing::Metadata) -> u16 {
            MAX_CALLSTACK_DEPTH
        }

        fn format_fields_in_zone_name(&self) -> bool {
            true
        }

        fn on_error(&self, client: &tracy_client::Client, error: &'static str) {
            client.color_message(error, 0xFF000000, 0);
        }
    }

    zlog::info!("Starting tracy subscriber, you can now connect the profiler");
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_tracy::TracyLayer::new(TracyLayerConfig::default())),
    )
    .expect("setup tracy layer");
}

#[cfg(all(target_family = "wasm", feature = "web"))]
mod web;

#[cfg(all(target_family = "wasm", feature = "web"))]
pub use web::{PerformanceLayer, PerformanceReporter, performance_layer};

#[cfg(all(target_family = "wasm", feature = "web"))]
pub fn init(start_reporter: impl FnOnce(PerformanceReporter)) {
    use tracing_subscriber::prelude::*;

    let (performance_layer, reporter) = performance_layer();
    if tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(performance_layer),
    )
    .is_ok()
    {
        start_reporter(reporter);
    } else {
        zlog::error!("failed to set browser performance tracing subscriber");
    }
}

#[cfg(all(not(ztracing), not(all(target_family = "wasm", feature = "web"))))]
pub fn init() {}
