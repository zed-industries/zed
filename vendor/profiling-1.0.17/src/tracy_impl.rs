#[macro_export]
macro_rules! scope {
    // Note: literal patterns provided as an optimization since they can skip an allocation.
    ($name:literal) => {
        // Note: callstack_depth is 0 since this has significant overhead
        let _tracy_span = $crate::tracy_client::span!($name, 0);
    };
    ($name:literal, $data:expr) => {
        // Note: callstack_depth is 0 since this has significant overhead
        let _tracy_span = $crate::tracy_client::span!($name, 0);
        _tracy_span.emit_text($data);
    };
    ($name:expr) => {
        let _function_name = {
            struct S;
            let type_name = core::any::type_name::<S>();
            &type_name[..type_name.len() - 3]
        };
        let _tracy_span = $crate::tracy_client::Client::running()
            .expect("scope! without a running tracy_client::Client")
            // Note: callstack_depth is 0 since this has significant overhead
            .span_alloc(Some($name), _function_name, file!(), line!(), 0);
    };
    ($name:expr, $data:expr) => {
        let _function_name = {
            struct S;
            let type_name = core::any::type_name::<S>();
            &type_name[..type_name.len() - 3]
        };
        let _tracy_span = $crate::tracy_client::Client::running()
            .expect("scope! without a running tracy_client::Client")
            // Note: callstack_depth is 0 since this has significant overhead
            .span_alloc(Some($name), _function_name, file!(), line!(), 0);
        _tracy_span.emit_text($data);
    };
}

#[macro_export]
macro_rules! function_scope {
    () => {
        let _tracy_span = $crate::tracy_client::span!();
    };
    ($data:expr) => {
        let _location = $crate::tracy_client::span_location!();
        let _tracy_span = $crate::tracy_client::Client::running()
            .expect("function_scope! without a running tracy_client::Client")
            .span(_location, 0);
        _tracy_span.emit_text($data);
    };
}

/// Registers a thread with the profiler API(s). This is usually setting a name for the thread.
/// Two variants:
///  - register_thread!() - Tries to get the name of the thread, or an ID if no name is set
///  - register_thread!(name: &str) - Registers the thread using the given name
#[macro_export]
macro_rules! register_thread {
    () => {
        let thread_name = std::thread::current()
            .name()
            .map(|x| x.to_string())
            .unwrap_or_else(|| format!("Thread {:?}", std::thread::current().id()));

        $crate::register_thread!(&thread_name);
    };
    ($name:expr) => {
        $crate::tracy_client::Client::running()
            .expect("register_thread! without a running tracy_client::Client")
            .set_thread_name($name);
    };
}

/// Finishes the frame. This isn't strictly necessary for some kinds of applications but a pretty
/// normal thing to track in games.
#[macro_export]
macro_rules! finish_frame {
    () => {
        $crate::tracy_client::Client::running()
            .expect("finish_frame! without a running tracy_client::Client")
            .frame_mark();
    };
}
