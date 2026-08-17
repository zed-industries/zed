/// Extracts the successful type of `Poll<T>`.
///
/// This macro bakes in propagation of `Pending` signals by returning early.
///
/// **Note:** Since Rust 1.64, this macro is soft-deprecated in favor of
/// [`ready!`](core::task::ready) macro in the standard library.
#[macro_export]
macro_rules! ready {
    ($e:expr $(,)?) => {
        match $e {
            $crate::task::Poll::Ready(t) => t,
            $crate::task::Poll::Pending => return $crate::task::Poll::Pending,
        }
    };
}
