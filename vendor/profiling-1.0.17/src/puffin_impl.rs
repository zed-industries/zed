#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        $crate::puffin::profile_scope!($name);
    };
    ($name:expr, $data:expr) => {
        $crate::puffin::profile_scope!($name, $data);
    };
}

#[macro_export]
macro_rules! function_scope {
    () => {
        $crate::puffin::profile_function!();
    };
    ($data:expr) => {
        $crate::puffin::profile_function!($data);
    };
}

#[macro_export]
macro_rules! register_thread {
    () => {};
    ($name:expr) => {
        // puffin uses the thread name
    };
}

/// Finishes the frame. This isn't strictly necessary for some kinds of applications but a pretty
/// normal thing to track in games.
#[macro_export]
macro_rules! finish_frame {
    () => {
        $crate::puffin::GlobalProfiler::lock().new_frame();
    };
}
