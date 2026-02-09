pub use tracing::{Level, field};

#[cfg(ztracing)]
pub use tracing::{
    Span, debug_span, error_span, event, info_span, instrument, span, trace_span, warn_span,
};
#[cfg(not(ztracing))]
pub use ztracing_macro::instrument;

#[cfg(all(ztracing, ztracing_with_memory))]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, 100);

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
        $crate::Span
    };
}

#[cfg(not(ztracing))]
pub struct Span;

#[cfg(not(ztracing))]
impl Span {
    pub fn current() -> Self {
        Self
    }

    pub fn enter(&self) {}

    pub fn record<T, S>(&self, _t: T, _s: S) {}
}

#[cfg(ztracing)]
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
            8
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

#[cfg(not(ztracing))]
pub fn init() {}
