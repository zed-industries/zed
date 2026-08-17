#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        $crate::optick::event!($name);
    };
    // NOTE: I've not been able to get attached data to work with optick
    ($name:expr, $data:expr) => {
        $crate::optick::event!($name);
        $crate::optick::tag!("tag", $data);
    };
}

#[macro_export]
macro_rules! function_scope {
    () => {
        $crate::optick::event!();
    };
    ($data:expr) => {
        $crate::optick::event!();
        $crate::optick::tag!("tag", $data);
    };
}

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
        $crate::optick::register_thread($name);
    };
}

/// Finishes the frame. This isn't strictly necessary for some kinds of applications but a pretty
/// normal thing to track in games.
#[macro_export]
macro_rules! finish_frame {
    () => {
        $crate::optick::next_frame();
    };
}
