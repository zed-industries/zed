pub use tracing::{Level, field};

#[cfg(ztracing)]
pub use tracing::{
    Span, debug_span, error_span, event, info_span, instrument, span, trace_span, warn_span,
};

#[cfg(not(ztracing))]
pub use ztracing_macro::instrument;

#[cfg(ztracing)]
const MAX_CALLSTACK_DEPTH: u16 = 16;

#[cfg(all(ztracing, ztracing_with_memory))]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, MAX_CALLSTACK_DEPTH);

#[cfg(all(ztracing, ztracing_with_memory))]
thread_local! {
    static RECORDER: AllocRecorder = AllocRecorder::default();
}

#[cfg(all(ztracing, ztracing_with_memory))]
#[derive(Debug, PartialEq, Eq, Hash)]
struct Allocation(*const std::ffi::c_void);
#[cfg(all(ztracing, ztracing_with_memory))]
unsafe impl Send for Allocation {}
#[cfg(all(ztracing, ztracing_with_memory))]
unsafe impl Sync for Allocation {}

#[cfg(all(ztracing, ztracing_with_memory))]
#[derive(Default)]
struct AllocRecorder {
    outstanding: std::sync::Mutex<std::collections::HashMap<Allocation, std::alloc::Layout>>,
}

#[cfg(all(ztracing, ztracing_with_memory))]
mod ts_allocs_tracy {
    use super::{Allocation, GLOBAL, RECORDER};
    use std::alloc::{GlobalAlloc, Layout};
    use std::ffi::c_void;

    fn record_alloc(ptr: *mut c_void, layout: Layout) {
        RECORDER.with(|recorder| {
            recorder
                .outstanding
                .lock()
                .unwrap()
                .insert(Allocation(ptr), layout);
        })
    }

    fn record_free(ptr: *mut c_void) -> Layout {
        RECORDER.with(|recorder| {
            recorder
                .outstanding
                .lock()
                .unwrap()
                .remove(&Allocation(ptr))
                .unwrap_or(unsafe { Layout::from_size_align_unchecked(0, 16) })
        })
    }

    pub unsafe extern "C" fn ts_malloc_tracy(size: usize) -> *mut c_void {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, 16);
            let ptr = GLOBAL.alloc(layout);
            record_alloc(ptr.cast(), layout);
            ptr.cast()
        }
    }

    pub unsafe extern "C" fn ts_free_tracy(ptr: *mut c_void) {
        let layout = record_free(ptr);
        unsafe {
            GLOBAL.dealloc(ptr.cast(), layout);
        }
    }

    pub unsafe extern "C" fn ts_realloc_tracy(ptr: *mut c_void, size: usize) -> *mut c_void {
        unsafe {
            let layout = if !ptr.is_null() {
                record_free(ptr)
            } else {
                Layout::from_size_align_unchecked(size, 16)
            };
            let new_ptr = GLOBAL.realloc(ptr.cast(), layout, size);
            let new_layout = Layout::from_size_align_unchecked(size, 16);
            record_alloc(new_ptr.cast(), new_layout);
            new_ptr.cast()
        }
    }

    pub unsafe extern "C" fn ts_calloc_tracy(n: usize, size: usize) -> *mut c_void {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size * n, 16);
            let ptr = GLOBAL.alloc_zeroed(layout);
            record_alloc(ptr.cast(), layout);
            ptr.cast()
        }
    }
}

#[cfg(all(ztracing, ztracing_with_memory))]
use ts_allocs_tracy::{ts_calloc_tracy, ts_free_tracy, ts_malloc_tracy, ts_realloc_tracy};

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

    #[cfg(ztracing_with_memory)]
    unsafe {
        tree_sitter::set_allocator(
            Some(ts_malloc_tracy),
            Some(ts_calloc_tracy),
            Some(ts_realloc_tracy),
            Some(ts_free_tracy),
        )
    }
}

#[cfg(not(ztracing))]
pub fn init() {}
