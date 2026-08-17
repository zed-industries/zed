#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        let _superluminal_guard = $crate::superluminal::SuperluminalGuard::new($name);
    };
    ($name:expr, $data:expr) => {
        let _superluminal_guard =
            $crate::superluminal::SuperluminalGuard::new_with_data($name, $data);
    };
}

#[macro_export]
macro_rules! function_scope {
    () => {
        let _function_name = {
            struct S;
            let type_name = core::any::type_name::<S>();
            &type_name[..type_name.len() - 3]
        };
        $crate::scope!(_function_name);
    };
    ($data:expr) => {
        let _function_name = {
            struct S;
            let type_name = core::any::type_name::<S>();
            &type_name[..type_name.len() - 3]
        };
        $crate::scope!(_function_name, $data);
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
        $crate::superluminal_perf::set_current_thread_name($name);
    };
}

#[macro_export]
macro_rules! finish_frame {
    () => {
        // superluminal does not have a frame end function
    };
}

//
// RAII wrappers to support superluminal. These are public as they need to be callable from macros
// but are not intended for direct use.
//
#[doc(hidden)]
pub mod superluminal {
    pub struct SuperluminalGuard;

    // 0xFFFFFFFF means "use default color"
    const DEFAULT_SUPERLUMINAL_COLOR: u32 = 0xFFFFFFFF;

    impl SuperluminalGuard {
        pub fn new(name: &'static str) -> Self {
            superluminal_perf::begin_event(name);
            SuperluminalGuard
        }

        pub fn new_with_data(
            name: &'static str,
            data: &str,
        ) -> Self {
            superluminal_perf::begin_event_with_data(name, data, DEFAULT_SUPERLUMINAL_COLOR);
            SuperluminalGuard
        }
    }

    impl Drop for SuperluminalGuard {
        fn drop(&mut self) {
            superluminal_perf::end_event();
        }
    }
}
