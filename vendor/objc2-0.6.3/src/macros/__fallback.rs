#[doc(hidden)]
#[macro_export]
macro_rules! __fallback_if_not_set {
    (
        ()
        ($($fallback:tt)*)
    ) => {
        $($fallback)*
    };
    (
        ($($actual:tt)+)
        ($($_fallback:tt)*)
    ) => {
        $($actual)+
    };
}
