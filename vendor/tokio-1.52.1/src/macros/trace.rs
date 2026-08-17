cfg_trace! {
    macro_rules! trace_op {
        ($name:expr, $readiness:literal) => {
            tracing::trace!(
                target: "runtime::resource::poll_op",
                op_name = $name,
                is_ready = $readiness
            );
        }
    }

    macro_rules! trace_poll_op {
        ($name:expr, $poll:expr $(,)*) => {
            match $poll {
                $crate::macros::support::Poll::Ready(t) => {
                    trace_op!($name, true);
                    $crate::macros::support::Poll::Ready(t)
                }
                $crate::macros::support::Poll::Pending => {
                    trace_op!($name, false);
                    return $crate::macros::support::Poll::Pending;
                }
            }
        };
    }
}
