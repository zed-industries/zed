pub use backtrace::Backtrace;

#[macro_export]
macro_rules! verify {
    ( $expression:expr, else $block:expr ) => {
        verify!($expression, (""), else $block)
    };

    ( $expression:expr, ( $($fmt_arg:tt)* ), else $block:expr ) => {{
        let verify_str = stringify!($expression);

        if !$expression {
            if cfg!(debug_assertions) {
                panic!("Claim failed {:?}: {}", verify_str, format_args!($($fmt_arg)*));
            } else {
                let backtrace = $crate::Backtrace::new();
                log::error!("Claim failed {:?}\n{:?}", verify_str, backtrace);
                $block
            }
        }
    }};
}

#[macro_export]
macro_rules! verify_not {
    ( $expression:expr, else $block:expr ) => {
        verify_not!($expression, (""), else $block)
    };

    ( $expression:expr, ( $($fmt_arg:tt)* ), else $block:expr ) => {
        verify!(!$expression, ( $($fmt_arg)* ), else $block)
    };
}
