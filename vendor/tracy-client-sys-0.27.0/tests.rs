#[cfg(all(feature = "enable", test))]
mod tests {
    use tracy_client_sys::*;

    fn test_emit_zone() {
        unsafe {
            let srcloc = ___tracy_source_location_data {
                name: b"name\0".as_ptr().cast(),
                function: b"function\0".as_ptr().cast(),
                file: b"file\0".as_ptr().cast(),
                line: 42,
                color: 0,
            };
            let zone_ctx = ___tracy_emit_zone_begin(&srcloc, 1);
            ___tracy_emit_zone_end(zone_ctx);
        }
    }

    fn test_emit_message_no_null() {
        unsafe {
            ___tracy_emit_message(b"hello world".as_ptr().cast(), 11, 1);
        }
    }

    /// Cannot use a libtest harness here because we need manual control over
    /// the profiler startup and shutdown.
    pub(crate) fn main() {
        unsafe {
            ___tracy_startup_profiler();
        }
        test_emit_zone();
        test_emit_message_no_null();
        unsafe {
            ___tracy_fiber_enter(b"hello".as_ptr().cast());
            ___tracy_shutdown_profiler();
        }
    }
}

fn main() {
    #[cfg(all(feature = "enable", test))]
    tests::main();
}
