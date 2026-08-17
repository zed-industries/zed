#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        let _span = $crate::tracing::span!($crate::tracing::Level::INFO, $name);
        let _span_entered = _span.enter();
    };
    ($name:expr, $data:expr) => {
        let _span = $crate::tracing::span!($crate::tracing::Level::INFO, $name, tag = $data);
        let _span_entered = _span.enter();
    };
}

#[macro_export]
macro_rules! function_scope {
    () => {
        let function_name = {
            struct S;
            let type_name = core::any::type_name::<S>();
            &type_name[..type_name.len() - 3]
        };
        let _span = $crate::tracing::span!(
            $crate::tracing::Level::INFO,
            "function_scope",
            "{}",
            function_name
        );
        let _span_entered = _span.enter();
    };
    ($data:expr) => {
        let function_name = {
            struct S;
            let type_name = core::any::type_name::<S>();
            &type_name[..type_name.len() - 3]
        };
        let _span = $crate::tracing::span!(
            $crate::tracing::Level::INFO,
            "function_scope",
            tag = $data,
            "{}",
            function_name
        );
        let _span_entered = _span.enter();
    };
}

#[macro_export]
macro_rules! register_thread {
    () => {};
    ($name:expr) => {};
}

#[macro_export]
macro_rules! finish_frame {
    () => {
        $crate::tracing::event!($crate::tracing::Level::INFO, tracy.frame_mark = true);
    };
}
